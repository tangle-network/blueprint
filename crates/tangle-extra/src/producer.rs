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
            // Try both JobSubmitted and JobSubmittedFromQuote events
            let job_event = match decode_job_submitted(log) {
                Ok(event) => event,
                Err(_) => match decode_job_submitted_from_quote(log) {
                    Ok(event) => event,
                    Err(err) => {
                        blueprint_core::trace!(
                            target: "tangle-producer",
                            "Failed to decode log {:?}: {err}",
                            log
                        );
                        continue;
                    }
                },
            };
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
            let job_call = job_submitted_to_call(job_event, log_block, block_hash.0, timestamp);
            jobs.push(job_call);
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
#[derive(Clone, Debug)]
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
const JOB_SUBMITTED_FROM_QUOTE_SIG: [u8; 32] =
    hex!("b707259a8a1604adca251fecf84eb283329cd45175690dcb8ff1cf52a6252422");
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

/// Decode a JobSubmittedFromQuote event from a log
///
/// ABI layout for non-indexed data:
///   [0..32]   uint8   jobIndex
///   [32..64]  address caller
///   [64..96]  offset  → quotedOperators (dynamic, skipped)
///   [96..128] uint256 totalPrice (skipped — indexer tracks it)
///   [128..160] offset → inputs (dynamic bytes)
fn decode_job_submitted_from_quote(log: &Log) -> Result<JobSubmittedEvent, ProducerError> {
    let topics = log.topics();
    if topics.is_empty() || topics[0].0 != JOB_SUBMITTED_FROM_QUOTE_SIG {
        return Err(ProducerError::Decoding(
            "not a JobSubmittedFromQuote log".into(),
        ));
    }
    if topics.len() < 3 {
        return Err(ProducerError::Decoding("topic list length mismatch".into()));
    }
    let service_id = read_u64_topic(&topics[1]);
    let call_id = read_u64_topic(&topics[2]);

    let data = log.data().data.as_ref();
    // Minimum: 5 head slots × 32 = 160 bytes
    const MIN_HEAD_SIZE: usize = 160;
    if data.len() < MIN_HEAD_SIZE {
        return Err(ProducerError::Decoding(
            "JobSubmittedFromQuote data too short".into(),
        ));
    }

    let mut buf = [0u8; 32];

    // jobIndex at [0..32]
    buf.copy_from_slice(&data[0..32]);
    let job_index = U256::from_be_bytes(buf)
        .to::<u64>()
        .try_into()
        .map_err(|_| ProducerError::Decoding("job index out of range".into()))?;

    // caller at [32..64]
    buf.copy_from_slice(&data[32..64]);
    let mut caller_bytes = [0u8; 20];
    caller_bytes.copy_from_slice(&buf[12..32]);
    let caller = Address::from_slice(&caller_bytes);

    // inputs offset at [128..160]
    buf.copy_from_slice(&data[128..160]);
    let inputs_offset = U256::from_be_bytes(buf).to::<u64>() as usize;
    if data.len() < inputs_offset + 32 {
        return Err(ProducerError::Decoding(
            "JobSubmittedFromQuote inputs offset out of range".into(),
        ));
    }

    buf.copy_from_slice(&data[inputs_offset..inputs_offset + 32]);
    let inputs_len = U256::from_be_bytes(buf).to::<u64>() as usize;
    let start = inputs_offset + 32;
    let end = start.checked_add(inputs_len).ok_or_else(|| {
        ProducerError::Decoding("JobSubmittedFromQuote inputs length overflow".into())
    })?;
    if end > data.len() {
        return Err(ProducerError::Decoding(
            "JobSubmittedFromQuote inputs length exceeds log data".into(),
        ));
    }

    let block_number = log
        .block_number
        .ok_or_else(|| ProducerError::Decoding("log missing block number".to_string()))?;
    let block_hash = log
        .block_hash
        .ok_or_else(|| ProducerError::Decoding("log missing block hash".to_string()))?;

    blueprint_core::trace!(
        target: "tangle-producer",
        job_index,
        inputs_len,
        data_len = data.len(),
        "Decoded JobSubmittedFromQuote payload"
    );

    Ok(JobSubmittedEvent {
        service_id,
        call_id,
        job_index,
        caller,
        inputs: data[start..end].to_vec(),
        block_number,
        block_hash,
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{B256, Bytes, LogData};

    /// Build a mock `Log` from topics and data, with block metadata populated.
    fn make_log(topics: Vec<B256>, data: Vec<u8>) -> Log {
        let inner = alloy_primitives::Log {
            address: Address::ZERO,
            data: LogData::new(topics, Bytes::from(data)).unwrap(),
        };
        Log {
            inner,
            block_hash: Some(B256::ZERO),
            block_number: Some(100),
            block_timestamp: Some(1_700_000_000),
            transaction_hash: None,
            transaction_index: None,
            log_index: Some(0),
            removed: false,
        }
    }

    /// ABI-encode the non-indexed data for a `JobSubmittedFromQuote` event.
    ///
    /// Layout (head-tail ABI encoding):
    ///   [0..32]    uint8   jobIndex
    ///   [32..64]   address caller
    ///   [64..96]   offset  → quotedOperators (dynamic)
    ///   [96..128]  uint256 totalPrice
    ///   [128..160] offset  → inputs (dynamic)
    ///   [160..]    quotedOperators length + elements, then inputs length + data
    fn encode_rfq_data(
        job_index: u8,
        caller: Address,
        quoted_operators: &[Address],
        total_price: U256,
        inputs: &[u8],
    ) -> Vec<u8> {
        let mut data = Vec::new();

        // Head section (5 × 32 bytes)
        // [0..32] jobIndex
        data.extend_from_slice(&U256::from(job_index).to_be_bytes::<32>());
        // [32..64] caller (left-padded to 32 bytes)
        let mut caller_slot = [0u8; 32];
        caller_slot[12..32].copy_from_slice(caller.as_slice());
        data.extend_from_slice(&caller_slot);
        // [64..96] offset to quotedOperators — starts at byte 160 (5 * 32)
        data.extend_from_slice(&U256::from(160u64).to_be_bytes::<32>());
        // [96..128] totalPrice
        data.extend_from_slice(&total_price.to_be_bytes::<32>());
        // [128..160] offset to inputs — computed below
        let operators_tail_len = 32 + quoted_operators.len() * 32; // length slot + elements
        let inputs_offset = 160 + operators_tail_len;
        data.extend_from_slice(&U256::from(inputs_offset as u64).to_be_bytes::<32>());

        // Tail: quotedOperators
        data.extend_from_slice(&U256::from(quoted_operators.len() as u64).to_be_bytes::<32>());
        for op in quoted_operators {
            let mut slot = [0u8; 32];
            slot[12..32].copy_from_slice(op.as_slice());
            data.extend_from_slice(&slot);
        }

        // Tail: inputs
        data.extend_from_slice(&U256::from(inputs.len() as u64).to_be_bytes::<32>());
        data.extend_from_slice(inputs);
        // Pad to 32-byte boundary
        let padding = (32 - (inputs.len() % 32)) % 32;
        data.extend(core::iter::repeat(0u8).take(padding));

        data
    }

    fn rfq_topics(service_id: u64, call_id: u64) -> Vec<B256> {
        vec![
            B256::from(JOB_SUBMITTED_FROM_QUOTE_SIG),
            B256::from(U256::from(service_id).to_be_bytes::<32>()),
            B256::from(U256::from(call_id).to_be_bytes::<32>()),
        ]
    }

    // ── Happy path ──────────────────────────────────────────────────────

    #[test]
    fn test_decode_rfq_event_basic() {
        let caller = Address::repeat_byte(0xAB);
        let operator = Address::repeat_byte(0xCD);
        let inputs = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let data = encode_rfq_data(3, caller, &[operator], U256::from(1_000_000u64), &inputs);
        let log = make_log(rfq_topics(42, 7), data);

        let event = decode_job_submitted_from_quote(&log).unwrap();
        assert_eq!(event.service_id, 42);
        assert_eq!(event.call_id, 7);
        assert_eq!(event.job_index, 3);
        assert_eq!(event.caller, caller);
        assert_eq!(event.inputs, inputs);
        assert_eq!(event.block_number, 100);
    }

    #[test]
    fn test_decode_rfq_event_empty_inputs() {
        let caller = Address::repeat_byte(0x01);
        let data = encode_rfq_data(0, caller, &[Address::ZERO], U256::ZERO, &[]);
        let log = make_log(rfq_topics(1, 1), data);

        let event = decode_job_submitted_from_quote(&log).unwrap();
        assert_eq!(event.job_index, 0);
        assert!(event.inputs.is_empty());
    }

    #[test]
    fn test_decode_rfq_event_multiple_operators() {
        let caller = Address::repeat_byte(0x11);
        let ops = vec![
            Address::repeat_byte(0xAA),
            Address::repeat_byte(0xBB),
            Address::repeat_byte(0xCC),
        ];
        let inputs = vec![1, 2, 3];
        let data = encode_rfq_data(5, caller, &ops, U256::from(999u64), &inputs);
        let log = make_log(rfq_topics(100, 200), data);

        let event = decode_job_submitted_from_quote(&log).unwrap();
        assert_eq!(event.service_id, 100);
        assert_eq!(event.call_id, 200);
        assert_eq!(event.job_index, 5);
        assert_eq!(event.inputs, vec![1, 2, 3]);
    }

    #[test]
    fn test_decode_rfq_event_large_inputs() {
        let caller = Address::repeat_byte(0x22);
        let inputs = vec![0xFFu8; 1024]; // 1 KB of input data
        let data = encode_rfq_data(7, caller, &[Address::ZERO], U256::from(1u64), &inputs);
        let log = make_log(rfq_topics(10, 20), data);

        let event = decode_job_submitted_from_quote(&log).unwrap();
        assert_eq!(event.inputs.len(), 1024);
        assert!(event.inputs.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn test_decode_rfq_event_max_job_index() {
        let caller = Address::repeat_byte(0x33);
        let data = encode_rfq_data(255, caller, &[Address::ZERO], U256::ZERO, &[0x01]);
        let log = make_log(rfq_topics(1, 1), data);

        let event = decode_job_submitted_from_quote(&log).unwrap();
        assert_eq!(event.job_index, 255);
    }

    // ── Error: wrong event signature ────────────────────────────────────

    #[test]
    fn test_decode_rfq_rejects_wrong_signature() {
        let data = encode_rfq_data(0, Address::ZERO, &[Address::ZERO], U256::ZERO, &[]);
        // Use the JobSubmitted signature instead
        let topics = vec![
            B256::from(JOB_SUBMITTED_SIG),
            B256::from(U256::from(1u64).to_be_bytes::<32>()),
            B256::from(U256::from(1u64).to_be_bytes::<32>()),
        ];
        let log = make_log(topics, data);

        let err = decode_job_submitted_from_quote(&log).unwrap_err();
        assert!(
            matches!(err, ProducerError::Decoding(msg) if msg.contains("not a JobSubmittedFromQuote")),
        );
    }

    #[test]
    fn test_decode_rfq_rejects_empty_topics() {
        let log = make_log(vec![], vec![]);
        let err = decode_job_submitted_from_quote(&log).unwrap_err();
        assert!(matches!(err, ProducerError::Decoding(_)));
    }

    #[test]
    fn test_decode_rfq_rejects_too_few_topics() {
        // Only signature + serviceId, missing callId
        let topics = vec![
            B256::from(JOB_SUBMITTED_FROM_QUOTE_SIG),
            B256::from(U256::from(1u64).to_be_bytes::<32>()),
        ];
        let log = make_log(topics, vec![]);
        let err = decode_job_submitted_from_quote(&log).unwrap_err();
        assert!(matches!(err, ProducerError::Decoding(msg) if msg.contains("topic list length")),);
    }

    // ── Error: malformed data ───────────────────────────────────────────

    #[test]
    fn test_decode_rfq_rejects_short_data() {
        // Data less than 160 bytes (5 head slots)
        let log = make_log(rfq_topics(1, 1), vec![0u8; 100]);
        let err = decode_job_submitted_from_quote(&log).unwrap_err();
        assert!(matches!(err, ProducerError::Decoding(msg) if msg.contains("too short")),);
    }

    #[test]
    fn test_decode_rfq_rejects_inputs_offset_out_of_range() {
        // Craft data where inputs offset points beyond data length
        let mut data = vec![0u8; 160];
        // Set inputs offset (at [128..160]) to some huge value
        let offset_bytes = U256::from(99999u64).to_be_bytes::<32>();
        data[128..160].copy_from_slice(&offset_bytes);
        let log = make_log(rfq_topics(1, 1), data);

        let err = decode_job_submitted_from_quote(&log).unwrap_err();
        assert!(matches!(err, ProducerError::Decoding(msg) if msg.contains("offset out of range")),);
    }

    #[test]
    fn test_decode_rfq_rejects_inputs_length_exceeds_data() {
        // Valid offset but inputs_len claims more data than exists
        let mut data = vec![0u8; 224]; // head + enough for offset pointing
        // inputs offset at [128..160] = 160
        let offset = U256::from(160u64).to_be_bytes::<32>();
        data[128..160].copy_from_slice(&offset);
        // At byte 160, set inputs_len to 9999 (way more than remaining data)
        let len_bytes = U256::from(9999u64).to_be_bytes::<32>();
        data[160..192].copy_from_slice(&len_bytes);
        let log = make_log(rfq_topics(1, 1), data);

        let err = decode_job_submitted_from_quote(&log).unwrap_err();
        assert!(matches!(err, ProducerError::Decoding(msg) if msg.contains("exceeds log data")),);
    }

    // ── Error: missing block metadata ───────────────────────────────────

    #[test]
    fn test_decode_rfq_rejects_missing_block_number() {
        let data = encode_rfq_data(0, Address::ZERO, &[Address::ZERO], U256::ZERO, &[0x01]);
        let mut log = make_log(rfq_topics(1, 1), data);
        log.block_number = None;

        let err = decode_job_submitted_from_quote(&log).unwrap_err();
        assert!(matches!(err, ProducerError::Decoding(msg) if msg.contains("block number")),);
    }

    #[test]
    fn test_decode_rfq_rejects_missing_block_hash() {
        let data = encode_rfq_data(0, Address::ZERO, &[Address::ZERO], U256::ZERO, &[0x01]);
        let mut log = make_log(rfq_topics(1, 1), data);
        log.block_hash = None;

        let err = decode_job_submitted_from_quote(&log).unwrap_err();
        assert!(matches!(err, ProducerError::Decoding(msg) if msg.contains("block hash")),);
    }

    // ── Cross-decoder isolation ─────────────────────────────────────────

    #[test]
    fn test_job_submitted_rejects_rfq_event() {
        let data = encode_rfq_data(0, Address::ZERO, &[Address::ZERO], U256::ZERO, &[0x01]);
        let log = make_log(rfq_topics(1, 1), data);
        // decode_job_submitted should reject a JobSubmittedFromQuote log
        let err = decode_job_submitted(&log).unwrap_err();
        assert!(matches!(err, ProducerError::Decoding(msg) if msg.contains("not a JobSubmitted")));
    }

    // ── JobSubmittedEvent → JobCall conversion ──────────────────────────

    #[test]
    fn test_job_submitted_to_call_preserves_metadata() {
        let caller = Address::repeat_byte(0xAB);
        let event = JobSubmittedEvent {
            service_id: 42,
            call_id: 7,
            job_index: 3,
            caller,
            inputs: vec![0xDE, 0xAD],
            block_number: 100,
            block_hash: B256::repeat_byte(0xFF),
        };
        let job = job_submitted_to_call(event, 100, [0xFF; 32], 1_700_000_000);

        assert_eq!(job.job_id(), blueprint_core::JobId::from(3u8));
        assert_eq!(job.body().as_ref(), &[0xDE, 0xAD]);

        let meta = job.metadata();
        let service_id: u64 = meta
            .get(extract::ServiceId::METADATA_KEY)
            .and_then(|v| u64::try_from(v).ok())
            .unwrap();
        assert_eq!(service_id, 42);

        let call_id: u64 = meta
            .get(extract::CallId::METADATA_KEY)
            .and_then(|v| u64::try_from(v).ok())
            .unwrap();
        assert_eq!(call_id, 7);
    }

    // ── Existing decoder (decode_job_submitted) edge cases ──────────────

    #[test]
    fn test_decode_job_submitted_basic() {
        // JobSubmitted(uint64 indexed serviceId, uint64 indexed callId, uint8 jobIndex, address caller, bytes inputs)
        // Non-indexed: (uint8, address, bytes)
        let mut data = Vec::new();
        // jobIndex = 2
        data.extend_from_slice(&U256::from(2u64).to_be_bytes::<32>());
        // caller
        let mut caller_slot = [0u8; 32];
        caller_slot[12..32].copy_from_slice(Address::repeat_byte(0x99).as_slice());
        data.extend_from_slice(&caller_slot);
        // offset to inputs = 96
        data.extend_from_slice(&U256::from(96u64).to_be_bytes::<32>());
        // inputs: [0xCA, 0xFE]
        data.extend_from_slice(&U256::from(2u64).to_be_bytes::<32>());
        data.extend_from_slice(&[0xCA, 0xFE]);
        data.extend(core::iter::repeat(0u8).take(30)); // pad

        let topics = vec![
            B256::from(JOB_SUBMITTED_SIG),
            B256::from(U256::from(10u64).to_be_bytes::<32>()),
            B256::from(U256::from(20u64).to_be_bytes::<32>()),
        ];
        let log = make_log(topics, data);

        let event = decode_job_submitted(&log).unwrap();
        assert_eq!(event.service_id, 10);
        assert_eq!(event.call_id, 20);
        assert_eq!(event.job_index, 2);
        assert_eq!(event.caller, Address::repeat_byte(0x99));
        assert_eq!(event.inputs, vec![0xCA, 0xFE]);
    }

    #[test]
    fn test_decode_job_submitted_rejects_short_data() {
        let topics = vec![
            B256::from(JOB_SUBMITTED_SIG),
            B256::from(U256::from(1u64).to_be_bytes::<32>()),
            B256::from(U256::from(1u64).to_be_bytes::<32>()),
        ];
        let log = make_log(topics, vec![0u8; 32]); // too short (< 96)
        assert!(decode_job_submitted(&log).is_err());
    }
}
