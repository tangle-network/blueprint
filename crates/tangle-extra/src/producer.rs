//! Tangle Producer
//!
//! Produces [`JobCall`]s from Tangle contract events.

use alloy_primitives::{Address, B256, U256, hex_literal::hex};
use alloy_rpc_types::{BlockNumberOrTag, Filter, Log};
use blueprint_client_tangle::TangleClient;
use blueprint_core::JobCall;
use blueprint_core::extensions::Extensions;
use blueprint_core::job::call::Parts;
use blueprint_core::metadata::MetadataMap;
use blueprint_std::boxed::Box;
use blueprint_std::collections::{BTreeMap, VecDeque};
use blueprint_std::string::{String, ToString};
use blueprint_std::sync::Mutex;
use blueprint_std::vec::Vec;
use core::convert::TryFrom;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;
use futures_core::Stream;
use tokio::time::sleep;

use crate::extract;

/// Error type for the producer
#[derive(Debug, thiserror::Error)]
pub enum ProducerError {
    /// Client error
    #[error("Client error: {0}")]
    Client(String),
    /// Event decoding error
    #[error("Event decoding error: {0}")]
    Decoding(String),
}

/// A producer of Tangle [`JobCall`]s
pub struct TangleProducer {
    client: TangleClient,
    service_id: u64,
    state: Mutex<ProducerState>,
    poll_interval: Duration,
}

struct ProducerState {
    last_block: u64,
    last_log_index: Option<u64>,
    buffer: VecDeque<JobCall>,
    poll_future:
        Option<Pin<Box<dyn Future<Output = Result<ProducerPollResult, ProducerError>> + Send>>>,
}

impl ProducerState {
    fn new(start_block: u64) -> Self {
        Self {
            last_block: start_block,
            last_log_index: None,
            buffer: VecDeque::new(),
            poll_future: None,
        }
    }
}

struct ProducerPollResult {
    jobs: Vec<JobCall>,
    last_block: u64,
    last_log_index: Option<u64>,
}

impl TangleProducer {
    /// Create a new [`TangleProducer`] that yields job calls for a specific service
    ///
    /// # Arguments
    /// * `client` - The Tangle client
    /// * `service_id` - The service ID to filter events for
    pub fn new(client: TangleClient, service_id: u64) -> Self {
        Self {
            client,
            service_id,
            state: Mutex::new(ProducerState::new(0)),
            poll_interval: Duration::from_secs(2),
        }
    }

    /// Create a producer starting from a specific block
    pub fn from_block(client: TangleClient, service_id: u64, start_block: u64) -> Self {
        Self {
            client,
            service_id,
            state: Mutex::new(ProducerState::new(start_block)),
            poll_interval: Duration::from_secs(2),
        }
    }

    /// Set the polling interval
    #[must_use]
    pub fn with_poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Get the service ID
    #[must_use]
    pub fn service_id(&self) -> u64 {
        self.service_id
    }

    /// Get the client
    #[must_use]
    pub fn client(&self) -> &TangleClient {
        &self.client
    }
}

impl Stream for TangleProducer {
    type Item = Result<JobCall, ProducerError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let producer = self.get_mut();
        let mut state = producer.state.lock().unwrap();

        loop {
            // First, check if there are buffered items
            if let Some(job) = state.buffer.pop_front() {
                return Poll::Ready(Some(Ok(job)));
            }

            // Check if there's an ongoing poll
            if let Some(fut) = state.poll_future.as_mut() {
                match fut.as_mut().poll(cx) {
                    Poll::Ready(Ok(result)) => {
                        state.last_block = result.last_block;
                        state.last_log_index = result.last_log_index;
                        state.buffer.extend(result.jobs);
                        state.poll_future = None;

                        if let Some(job) = state.buffer.pop_front() {
                            return Poll::Ready(Some(Ok(job)));
                        }
                        // No jobs found, start a new poll
                    }
                    Poll::Ready(Err(e)) => {
                        state.poll_future = None;
                        return Poll::Ready(Some(Err(e)));
                    }
                    Poll::Pending => return Poll::Pending,
                }
            }

            // Start a new poll for events
            let client = producer.client.clone();
            let service_id = producer.service_id;
            let last_block = state.last_block;
            let last_log_index = state.last_log_index;

            let fut = Box::pin(async move {
                poll_for_jobs(client, service_id, last_block, last_log_index).await
            });
            state.poll_future = Some(fut);
        }
    }
}

/// Poll for new job events
async fn poll_for_jobs(
    client: TangleClient,
    service_id: u64,
    from_block: u64,
    from_log_index: Option<u64>,
) -> Result<ProducerPollResult, ProducerError> {
    let mut block_number_failures = 0u32;
    let mut get_logs_failures = 0u32;
    let mut block_fetch_failures = 0u32;

    'poll_loop: loop {
        let latest_block = match client.block_number().await {
            Ok(number) => {
                if block_number_failures > 0 {
                    blueprint_core::info!(
                        target: "tangle-producer",
                        rpc = "eth_blockNumber",
                        attempts = block_number_failures,
                        "RPC recovered after retries"
                    );
                    block_number_failures = 0;
                }
                number
            }
            Err(err) => {
                block_number_failures += 1;
                let delay = rpc_retry_delay(block_number_failures);
                if block_number_failures >= RPC_ERROR_ESCALATION_ATTEMPTS {
                    blueprint_core::error!(
                        target: "tangle-producer",
                        rpc = "eth_blockNumber",
                        attempts = block_number_failures,
                        delay_ms = delay.as_millis() as u64,
                        "Failed to read block number: {err}"
                    );
                } else {
                    blueprint_core::warn!(
                        target: "tangle-producer",
                        rpc = "eth_blockNumber",
                        attempts = block_number_failures,
                        delay_ms = delay.as_millis() as u64,
                        "Failed to read block number: {err}; retrying"
                    );
                }
                sleep(delay).await;
                continue;
            }
        };

        if latest_block < from_block {
            sleep(Duration::from_millis(250)).await;
            continue;
        }

        let filter = Filter::new()
            .address(client.tangle_address())
            .from_block(from_block)
            .to_block(latest_block);

        let mut logs = match client.get_logs(&filter).await {
            Ok(logs) => {
                if get_logs_failures > 0 {
                    blueprint_core::info!(
                        target: "tangle-producer",
                        rpc = "eth_getLogs",
                        attempts = get_logs_failures,
                        from = from_block,
                        to = latest_block,
                        "RPC recovered after retries"
                    );
                    get_logs_failures = 0;
                }
                logs
            }
            Err(err) => {
                get_logs_failures += 1;
                let delay = rpc_retry_delay(get_logs_failures);
                if get_logs_failures >= RPC_ERROR_ESCALATION_ATTEMPTS {
                    blueprint_core::error!(
                        target: "tangle-producer",
                        rpc = "eth_getLogs",
                        attempts = get_logs_failures,
                        from = from_block,
                        to = latest_block,
                        delay_ms = delay.as_millis() as u64,
                        "Failed to fetch logs: {err}"
                    );
                } else {
                    blueprint_core::warn!(
                        target: "tangle-producer",
                        rpc = "eth_getLogs",
                        attempts = get_logs_failures,
                        from = from_block,
                        to = latest_block,
                        delay_ms = delay.as_millis() as u64,
                        "Failed to fetch logs: {err}; retrying"
                    );
                }
                sleep(delay).await;
                continue;
            }
        };

        logs.sort_by_key(|log| {
            (
                log.block_number.unwrap_or_default(),
                log.log_index.unwrap_or_default(),
            )
        });

        let filtered_logs: Vec<Log> = if let Some(last_index) = from_log_index {
            logs.into_iter()
                .filter(|log| {
                    let block_number = log.block_number.unwrap_or_default();
                    if block_number < from_block {
                        return false;
                    }
                    if block_number > from_block {
                        return true;
                    }
                    let log_index = log.log_index.unwrap_or_default();
                    log_index > last_index
                })
                .collect()
        } else {
            logs
        };

        let (last_block, last_log_index) = if let Some(last) = filtered_logs.last() {
            (last.block_number.unwrap_or(latest_block), last.log_index)
        } else if latest_block == from_block {
            (from_block, from_log_index)
        } else {
            (latest_block, None)
        };

        let mut jobs = Vec::new();
        let mut block_timestamps = BTreeMap::new();

        for log in &filtered_logs {
            match decode_job_submitted(log) {
                Ok(job_event) => {
                    if job_event.service_id != service_id {
                        continue;
                    }

                    let log_block = job_event.block_number;
                    let timestamp = if let Some(ts) = log.block_timestamp {
                        block_timestamps.insert(log_block, ts);
                        ts
                    } else {
                        match block_timestamps.get(&log_block) {
                            Some(ts) => *ts,
                            None => {
                                match client
                                    .get_block(BlockNumberOrTag::Number(log_block.into()))
                                    .await
                                {
                                    Ok(Some(block)) => {
                                        if block_fetch_failures > 0 {
                                            blueprint_core::info!(
                                                target: "tangle-producer",
                                                rpc = "eth_getBlockByNumber",
                                                attempts = block_fetch_failures,
                                                block = log_block,
                                                "RPC recovered after retries"
                                            );
                                        }
                                        block_fetch_failures = 0;
                                        let ts = block.header.timestamp;
                                        block_timestamps.insert(log_block, ts);
                                        ts
                                    }
                                    Ok(None) => {
                                        block_fetch_failures += 1;
                                        let delay = rpc_retry_delay(block_fetch_failures);
                                        if block_fetch_failures >= RPC_ERROR_ESCALATION_ATTEMPTS {
                                            blueprint_core::error!(
                                                target: "tangle-producer",
                                                rpc = "eth_getBlockByNumber",
                                                attempts = block_fetch_failures,
                                                block = log_block,
                                                delay_ms = delay.as_millis() as u64,
                                                "Missing block data while deriving timestamp"
                                            );
                                        } else {
                                            blueprint_core::warn!(
                                                target: "tangle-producer",
                                                rpc = "eth_getBlockByNumber",
                                                attempts = block_fetch_failures,
                                                block = log_block,
                                                delay_ms = delay.as_millis() as u64,
                                                "Missing block data while deriving timestamp; retrying"
                                            );
                                        }
                                        sleep(delay).await;
                                        continue 'poll_loop;
                                    }
                                    Err(err) => {
                                        block_fetch_failures += 1;
                                        let delay = rpc_retry_delay(block_fetch_failures);
                                        if block_fetch_failures >= RPC_ERROR_ESCALATION_ATTEMPTS {
                                            blueprint_core::error!(
                                                target: "tangle-producer",
                                                rpc = "eth_getBlockByNumber",
                                                attempts = block_fetch_failures,
                                                block = log_block,
                                                delay_ms = delay.as_millis() as u64,
                                                "Failed to read block data: {err}"
                                            );
                                        } else {
                                            blueprint_core::warn!(
                                                target: "tangle-producer",
                                                rpc = "eth_getBlockByNumber",
                                                attempts = block_fetch_failures,
                                                block = log_block,
                                                delay_ms = delay.as_millis() as u64,
                                                "Failed to read block data: {err}; retrying"
                                            );
                                        }
                                        sleep(delay).await;
                                        continue 'poll_loop;
                                    }
                                }
                            }
                        }
                    };

                    let block_hash = job_event.block_hash;
                    let job_call =
                        job_submitted_to_call(job_event, log_block, block_hash.0, timestamp);
                    jobs.push(job_call);
                }
                Err(err) => {
                    blueprint_core::trace!(
                        target: "tangle-producer",
                        "Failed to decode log {:?}: {err}",
                        log
                    );
                }
            }
        }

        if jobs.is_empty() {
            blueprint_core::trace!(
                target: "tangle-producer",
                from = from_block,
                to = latest_block,
                "No jobs discovered during this poll"
            );
        } else {
            for job in &jobs {
                let block_number = job
                    .metadata()
                    .get(extract::BlockNumber::METADATA_KEY)
                    .and_then(|value| u64::try_from(value).ok());
                let service_id = job
                    .metadata()
                    .get(extract::ServiceId::METADATA_KEY)
                    .and_then(|value| u64::try_from(value).ok());
                blueprint_core::info!(
                    target: "tangle-producer",
                    job_id = ?job.job_id(),
                    block_number = ?block_number,
                    service_id = ?service_id,
                    "Returning job in producer batch"
                );
            }
        }

        return Ok(ProducerPollResult {
            jobs,
            last_block,
            last_log_index,
        });
    }
}

/// Decoded JobSubmitted event
#[derive(Clone)]
struct JobSubmittedEvent {
    service_id: u64,
    call_id: u64,
    job_index: u8,
    caller: Address,
    inputs: alloc::vec::Vec<u8>,
    block_number: u64,
    block_hash: B256,
}

const JOB_SUBMITTED_SIG: [u8; 32] =
    hex!("de37cc48d21778e1c9a075c4e41c5aff6918c3ea6151221f0af3ce8121a29db5");
const RPC_RETRY_BASE_DELAY_MS: u64 = 250;
const RPC_RETRY_MAX_DELAY_MS: u64 = 5_000;
const RPC_ERROR_ESCALATION_ATTEMPTS: u32 = 5;

/// Decode a JobSubmitted event from a log
fn decode_job_submitted(log: &Log) -> Result<JobSubmittedEvent, ProducerError> {
    let topics = log.topics();
    if topics.is_empty() || topics[0].0 != JOB_SUBMITTED_SIG {
        return Err(ProducerError::Decoding("not a JobSubmitted log".into()));
    }
    if topics.len() < 3 {
        return Err(ProducerError::Decoding("topic list length mismatch".into()));
    }
    let service_id = read_u64_topic(&topics[1]);
    let call_id = read_u64_topic(&topics[2]);

    let data = log.data().data.as_ref();
    let (job_index, caller, inputs) = decode_job_submitted_data(data)?;
    let job_index = if topics.len() > 3 {
        read_u64_topic(&topics[3]) as u8
    } else {
        job_index
    };

    let block_number = log
        .block_number
        .ok_or_else(|| ProducerError::Decoding("log missing block number".to_string()))?;
    let block_hash = log
        .block_hash
        .ok_or_else(|| ProducerError::Decoding("log missing block hash".to_string()))?;

    Ok(JobSubmittedEvent {
        service_id,
        call_id,
        job_index,
        caller,
        inputs,
        block_number,
        block_hash,
    })
}

fn decode_job_submitted_data(
    data: &[u8],
) -> Result<(u8, Address, alloc::vec::Vec<u8>), ProducerError> {
    const MIN_FIXED_SIZE: usize = 96;
    if data.len() < MIN_FIXED_SIZE {
        return Err(ProducerError::Decoding(
            "JobSubmitted data too short for fixed fields".into(),
        ));
    }

    let mut buf = [0u8; 32];
    buf.copy_from_slice(&data[0..32]);
    let job_index = U256::from_be_bytes(buf)
        .to::<u64>()
        .try_into()
        .map_err(|_| ProducerError::Decoding("job index out of range".into()))?;

    buf.copy_from_slice(&data[32..64]);
    let mut caller_bytes = [0u8; 20];
    caller_bytes.copy_from_slice(&buf[12..32]);
    let caller = Address::from_slice(&caller_bytes);

    buf.copy_from_slice(&data[64..96]);
    let offset = U256::from_be_bytes(buf).to::<u64>() as usize;
    if data.len() < offset + 32 {
        return Err(ProducerError::Decoding(
            "JobSubmitted inputs offset out of range".into(),
        ));
    }

    buf.copy_from_slice(&data[offset..offset + 32]);
    let inputs_len = U256::from_be_bytes(buf).to::<u64>() as usize;
    let start = offset + 32;
    let end = start
        .checked_add(inputs_len)
        .ok_or_else(|| ProducerError::Decoding("JobSubmitted inputs length overflow".into()))?;
    if end > data.len() {
        return Err(ProducerError::Decoding(
            "JobSubmitted inputs length exceeds log data".into(),
        ));
    }

    blueprint_core::trace!(
        target: "tangle-producer",
        job_index,
        offset,
        inputs_len,
        data_len = data.len(),
        "Decoded JobSubmitted payload"
    );
    Ok((job_index, caller, data[start..end].to_vec()))
}

fn read_u64_topic(topic: &B256) -> u64 {
    let mut buf = [0u8; 32];
    buf.copy_from_slice(topic.as_slice());
    U256::from_be_bytes(buf).to::<u64>()
}

/// Convert a JobSubmitted event to a JobCall
fn job_submitted_to_call(
    event: JobSubmittedEvent,
    block_number: u64,
    block_hash: [u8; 32],
    timestamp: u64,
) -> JobCall {
    let mut metadata = MetadataMap::new();
    metadata.insert(extract::CallId::METADATA_KEY, event.call_id);
    metadata.insert(extract::ServiceId::METADATA_KEY, event.service_id);
    // Convert u8 to [u8; 1] since MetadataValue doesn't impl From<u8>
    metadata.insert(extract::JobIndex::METADATA_KEY, [event.job_index]);
    metadata.insert(extract::BlockNumber::METADATA_KEY, block_number);
    metadata.insert(extract::BlockHash::METADATA_KEY, block_hash);
    metadata.insert(extract::Timestamp::METADATA_KEY, timestamp);
    metadata.insert(extract::Caller::METADATA_KEY, event.caller.0.0);

    let extensions = Extensions::new();
    let parts = Parts::new(event.job_index)
        .with_metadata(metadata)
        .with_extensions(extensions);

    JobCall::from_parts(parts, event.inputs.into())
}

fn rpc_retry_delay(attempt: u32) -> Duration {
    let capped = attempt
        .max(1)
        .min((RPC_RETRY_MAX_DELAY_MS / RPC_RETRY_BASE_DELAY_MS) as u32);
    Duration::from_millis(RPC_RETRY_BASE_DELAY_MS * u64::from(capped))
}
