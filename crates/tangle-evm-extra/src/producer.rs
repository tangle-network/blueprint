//! Tangle EVM Producer
//!
//! Produces [`JobCall`]s from Tangle EVM contract events.

use alloc::collections::VecDeque;
use alloy_primitives::{Address, U256};
use alloy_rpc_types::Log;
use alloy_sol_types::SolEvent;
use blueprint_client_tangle_evm::contracts::ITangle;
use blueprint_client_tangle_evm::TangleEvmClient;
use blueprint_core::extensions::Extensions;
use blueprint_core::job::call::Parts;
use blueprint_core::metadata::{MetadataMap, MetadataValue};
use blueprint_core::JobCall;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_core::Stream;
use std::sync::Mutex;
use std::time::Duration;

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

/// A producer of Tangle EVM [`JobCall`]s
pub struct TangleEvmProducer {
    client: TangleEvmClient,
    service_id: u64,
    state: Mutex<ProducerState>,
    poll_interval: Duration,
}

struct ProducerState {
    last_block: u64,
    buffer: VecDeque<JobCall>,
    poll_future: Option<Pin<Box<dyn Future<Output = Result<Vec<JobCall>, ProducerError>> + Send>>>,
}

impl ProducerState {
    fn new(start_block: u64) -> Self {
        Self {
            last_block: start_block,
            buffer: VecDeque::new(),
            poll_future: None,
        }
    }
}

impl TangleEvmProducer {
    /// Create a new [`TangleEvmProducer`] that yields job calls for a specific service
    ///
    /// # Arguments
    /// * `client` - The Tangle EVM client
    /// * `service_id` - The service ID to filter events for
    pub fn new(client: TangleEvmClient, service_id: u64) -> Self {
        Self {
            client,
            service_id,
            state: Mutex::new(ProducerState::new(0)),
            poll_interval: Duration::from_secs(2),
        }
    }

    /// Create a producer starting from a specific block
    pub fn from_block(client: TangleEvmClient, service_id: u64, start_block: u64) -> Self {
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
    pub fn client(&self) -> &TangleEvmClient {
        &self.client
    }
}

impl Stream for TangleEvmProducer {
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
                    Poll::Ready(Ok(jobs)) => {
                        state.buffer.extend(jobs);
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

            let fut = Box::pin(async move {
                poll_for_jobs(client, service_id, last_block).await
            });
            state.poll_future = Some(fut);
        }
    }
}

/// Poll for new job events
async fn poll_for_jobs(
    client: TangleEvmClient,
    service_id: u64,
    from_block: u64,
) -> Result<Vec<JobCall>, ProducerError> {
    // Get the latest event
    let event = match client.next_event().await {
        Some(e) => e,
        None => return Ok(Vec::new()),
    };

    if event.block_number <= from_block {
        return Ok(Vec::new());
    }

    let mut jobs = Vec::new();

    for log in &event.logs {
        // Try to decode as JobSubmitted event
        if let Ok(job_event) = decode_job_submitted(log) {
            // Filter by service ID
            if job_event.serviceId != service_id {
                continue;
            }

            let job_call = job_submitted_to_call(
                job_event,
                event.block_number,
                event.block_hash.0,
                event.timestamp,
            );
            jobs.push(job_call);
        }
    }

    if !jobs.is_empty() {
        blueprint_core::trace!(
            target: "tangle-evm-producer",
            "Found {} job(s) in block #{}",
            jobs.len(),
            event.block_number
        );
    }

    Ok(jobs)
}

/// Decoded JobSubmitted event
struct JobSubmittedEvent {
    serviceId: u64,
    callId: u64,
    jobIndex: u8,
    caller: Address,
    inputs: alloc::vec::Vec<u8>,
}

/// Decode a JobSubmitted event from a log
fn decode_job_submitted(log: &Log) -> Result<JobSubmittedEvent, ProducerError> {
    // The JobSubmitted event signature
    let event = ITangle::JobSubmitted::decode_log(log, true)
        .map_err(|e| ProducerError::Decoding(e.to_string()))?;

    Ok(JobSubmittedEvent {
        serviceId: event.serviceId,
        callId: event.callId,
        jobIndex: event.jobIndex,
        caller: event.caller,
        inputs: event.inputs.to_vec(),
    })
}

/// Convert a JobSubmitted event to a JobCall
fn job_submitted_to_call(
    event: JobSubmittedEvent,
    block_number: u64,
    block_hash: [u8; 32],
    timestamp: u64,
) -> JobCall {
    let mut metadata = MetadataMap::new();
    metadata.insert(extract::CallId::METADATA_KEY, event.callId);
    metadata.insert(extract::ServiceId::METADATA_KEY, event.serviceId);
    metadata.insert(extract::JobIndex::METADATA_KEY, event.jobIndex);
    metadata.insert(extract::BlockNumber::METADATA_KEY, block_number);
    metadata.insert(extract::BlockHash::METADATA_KEY, block_hash);
    metadata.insert(extract::Timestamp::METADATA_KEY, timestamp);
    metadata.insert(extract::Caller::METADATA_KEY, event.caller.0 .0);

    let extensions = Extensions::new();
    let parts = Parts::new(event.jobIndex)
        .with_metadata(metadata)
        .with_extensions(extensions);

    JobCall::from_parts(parts, event.inputs.into())
}
