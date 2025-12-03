//! Tangle-specific job producer for EVM
//!
//! This module provides a producer that specifically listens for Tangle's `JobSubmitted`
//! events from the Jobs contract and converts them to `JobCall`s with proper metadata
//! (service_id, call_id, job_index, caller, inputs).

use alloc::collections::VecDeque;
use alloy_primitives::Address;
use alloy_provider::Provider;
use alloy_rpc_types::{Filter, Log};
use alloy_sol_types::{sol, SolEvent};
use alloy_transport::TransportError;
use blueprint_core::extensions::Extensions;
use blueprint_core::job::call::Parts;
use blueprint_core::metadata::MetadataMap;
use blueprint_core::JobCall;
use blueprint_std::sync::{Arc, Mutex};
use bytes::Bytes;
use core::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use futures::Stream;
use tokio::time::Sleep;

use crate::extract::{BlockHash, BlockNumber, BlockTimestamp, CallId, Caller, JobIndex, ServiceId};

// Define the JobSubmitted event ABI
sol! {
    /// Event emitted when a job is submitted to a service
    #[derive(Debug)]
    event JobSubmitted(
        uint64 indexed serviceId,
        uint64 indexed callId,
        uint8 jobIndex,
        address caller,
        bytes inputs
    );
}

/// Configuration for the Tangle job producer
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct TangleProducerConfig {
    /// Address of the Tangle Jobs contract
    pub contract_address: Address,
    /// Block to start polling from
    start_block: StartBlockSource,
    /// Interval between polling attempts
    poll_interval: Duration,
    /// Number of confirmations required for finality
    confirmations: u64,
    /// Number of blocks to fetch per polling cycle
    step: u64,
}

#[derive(Debug, Clone, Copy)]
enum StartBlockSource {
    Genesis,
    Current(u64),
    Custom(u64),
}

impl StartBlockSource {
    fn number(&self) -> u64 {
        match self {
            StartBlockSource::Genesis => 0,
            StartBlockSource::Current(block) | StartBlockSource::Custom(block) => *block,
        }
    }
}

impl TangleProducerConfig {
    /// Create a new configuration starting from the current block
    pub fn new(contract_address: Address) -> Self {
        Self {
            contract_address,
            start_block: StartBlockSource::Current(0),
            poll_interval: Duration::from_secs(1),
            confirmations: 12,
            step: 1000,
        }
    }

    /// Start from genesis block
    pub fn from_genesis(contract_address: Address) -> Self {
        Self {
            start_block: StartBlockSource::Genesis,
            ..Self::new(contract_address)
        }
    }

    /// Start from a specific block
    pub fn from_block(contract_address: Address, block: u64) -> Self {
        Self {
            start_block: StartBlockSource::Custom(block),
            ..Self::new(contract_address)
        }
    }

    /// Set the polling interval
    pub fn poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Set the number of confirmations for finality
    pub fn confirmations(mut self, confirmations: u64) -> Self {
        self.confirmations = confirmations;
        self
    }

    /// Set the step size (blocks per polling cycle)
    pub fn step(mut self, step: u64) -> Self {
        self.step = step;
        self
    }
}

/// Producer state for managing the polling lifecycle
enum ProducerState {
    /// Fetching the current best block number
    FetchingBlockNumber(Pin<Box<dyn Future<Output = Result<u64, TransportError>> + Send>>),
    /// Fetching logs for a specific block range
    FetchingLogs(Pin<Box<dyn Future<Output = Result<Vec<Log>, TransportError>> + Send>>),
    /// Waiting for next polling interval
    Idle(Pin<Box<Sleep>>),
}

impl core::fmt::Debug for ProducerState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FetchingBlockNumber(_) => f.debug_tuple("FetchingBlockNumber").finish(),
            Self::FetchingLogs(_) => f.debug_tuple("FetchingLogs").finish(),
            Self::Idle(_) => f.debug_tuple("Idle").finish(),
        }
    }
}

/// A producer that polls for Tangle `JobSubmitted` events and converts them to `JobCall`s.
///
/// This producer specifically listens for the `JobSubmitted` event from the Tangle Jobs contract
/// and extracts all job metadata (service_id, call_id, job_index, caller, inputs) into the
/// `JobCall` metadata and extensions.
#[derive(Debug)]
pub struct TangleProducer<P: Provider> {
    provider: Arc<P>,
    filter: Filter,
    config: TangleProducerConfig,
    state: Arc<Mutex<ProducerState>>,
    buffer: VecDeque<JobCall>,
}

impl<P: Provider> TangleProducer<P> {
    /// Creates a new Tangle job producer
    ///
    /// # Arguments
    /// * `provider` - The EVM provider to use for fetching logs
    /// * `config` - Configuration parameters for the producer
    ///
    /// # Errors
    /// Returns a transport error if unable to fetch the current block number
    pub async fn new(provider: Arc<P>, mut config: TangleProducerConfig) -> Result<Self, TransportError> {
        if let StartBlockSource::Current(current_block) = &mut config.start_block {
            *current_block = provider.get_block_number().await?;
        }

        let initial_start_block = config
            .start_block
            .number()
            .saturating_sub(config.confirmations);

        // Create filter for JobSubmitted events from the contract
        let filter = Filter::new()
            .address(config.contract_address)
            .event_signature(JobSubmitted::SIGNATURE_HASH)
            .from_block(initial_start_block)
            .to_block(initial_start_block + config.step);

        blueprint_core::trace!(
            target: "tangle-evm-producer",
            contract = %config.contract_address,
            start_block = initial_start_block,
            step = config.step,
            confirmations = config.confirmations,
            "Initializing Tangle job producer"
        );

        Ok(Self {
            provider,
            config,
            filter,
            state: Arc::new(Mutex::new(ProducerState::Idle(Box::pin(
                tokio::time::sleep(Duration::from_micros(1)),
            )))),
            buffer: VecDeque::with_capacity(100),
        })
    }

    /// Get the contract address being monitored
    pub fn contract_address(&self) -> Address {
        self.config.contract_address
    }
}

impl<P: Provider + 'static> Stream for TangleProducer<P> {
    type Item = Result<JobCall, TransportError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        loop {
            // Serve from buffer if available
            if let Some(job) = this.buffer.pop_front() {
                if !this.buffer.is_empty() {
                    cx.waker().wake_by_ref();
                }
                return Poll::Ready(Some(Ok(job)));
            }

            let mut state = this.state.lock().unwrap();
            match *state {
                ProducerState::Idle(ref mut fut) => match fut.as_mut().poll(cx) {
                    Poll::Ready(()) => {
                        blueprint_core::trace!(
                            target: "tangle-evm-producer",
                            "Polling interval elapsed, fetching current block number"
                        );
                        let provider = this.provider.clone();
                        let fut = async move { provider.get_block_number().await };
                        *state = ProducerState::FetchingBlockNumber(Box::pin(fut));
                    }
                    Poll::Pending => return Poll::Pending,
                },
                ProducerState::FetchingBlockNumber(ref mut fut) => match fut.as_mut().poll(cx) {
                    Poll::Ready(Ok(current_block)) => {
                        let safe_block = current_block.saturating_sub(this.config.confirmations);
                        let last_queried = this
                            .filter
                            .get_to_block()
                            .unwrap_or(this.config.start_block.number());

                        let next_from_block = last_queried.saturating_add(1);
                        let proposed_to_block = last_queried.saturating_add(this.config.step);
                        let next_to_block = proposed_to_block.min(safe_block);

                        blueprint_core::trace!(
                            target: "tangle-evm-producer",
                            current_block,
                            safe_block,
                            next_from_block,
                            next_to_block,
                            "Calculated block range"
                        );

                        if next_from_block > safe_block {
                            blueprint_core::trace!(
                                target: "tangle-evm-producer",
                                "No new blocks to process yet, waiting"
                            );
                            *state = ProducerState::Idle(Box::pin(tokio::time::sleep(
                                this.config.poll_interval,
                            )));
                            continue;
                        }

                        // Update filter for next range
                        this.filter = Filter::new()
                            .address(this.config.contract_address)
                            .event_signature(JobSubmitted::SIGNATURE_HASH)
                            .from_block(next_from_block)
                            .to_block(next_to_block);

                        blueprint_core::trace!(
                            target: "tangle-evm-producer",
                            from_block = next_from_block,
                            to_block = next_to_block,
                            "Fetching JobSubmitted events"
                        );

                        let provider = this.provider.clone();
                        let filter = this.filter.clone();
                        let fut = async move { provider.get_logs(&filter).await };
                        *state = ProducerState::FetchingLogs(Box::pin(fut));
                    }
                    Poll::Ready(Err(e)) => {
                        blueprint_core::error!(
                            target: "tangle-evm-producer",
                            error = ?e,
                            "Failed to fetch current block number"
                        );
                        *state = ProducerState::Idle(Box::pin(tokio::time::sleep(
                            this.config.poll_interval,
                        )));
                        return Poll::Ready(Some(Err(e)));
                    }
                    Poll::Pending => return Poll::Pending,
                },
                ProducerState::FetchingLogs(ref mut fut) => match fut.as_mut().poll(cx) {
                    Poll::Ready(Ok(logs)) => {
                        blueprint_core::trace!(
                            target: "tangle-evm-producer",
                            logs_count = logs.len(),
                            "Successfully fetched JobSubmitted events"
                        );

                        // Convert logs to job calls with metadata
                        let job_calls = logs_to_tangle_job_calls(logs);
                        this.buffer.extend(job_calls);

                        *state = ProducerState::Idle(Box::pin(tokio::time::sleep(
                            this.config.poll_interval,
                        )));
                    }
                    Poll::Ready(Err(e)) => {
                        blueprint_core::error!(
                            target: "tangle-evm-producer",
                            error = ?e,
                            "Failed to fetch logs"
                        );
                        *state = ProducerState::Idle(Box::pin(tokio::time::sleep(
                            this.config.poll_interval,
                        )));
                        return Poll::Ready(Some(Err(e)));
                    }
                    Poll::Pending => return Poll::Pending,
                },
            }
        }
    }
}

/// Converts JobSubmitted logs to JobCalls with proper metadata
fn logs_to_tangle_job_calls(logs: Vec<Log>) -> Vec<JobCall> {
    let mut job_calls = Vec::new();

    for log in logs {
        let Some(block_number) = log.block_number else {
            blueprint_core::warn!(?log, "Missing block number");
            continue;
        };

        // Decode the JobSubmitted event
        let event = match JobSubmitted::decode_log(&log.inner) {
            Ok(e) => e,
            Err(e) => {
                blueprint_core::warn!(?e, "Failed to decode JobSubmitted event");
                continue;
            }
        };

        let data = event.data;

        // Build metadata
        let mut metadata = MetadataMap::new();
        metadata.insert(BlockNumber::METADATA_KEY, block_number);
        if let Some(block_hash) = log.block_hash {
            metadata.insert(BlockHash::METADATA_KEY, *block_hash);
        }
        if let Some(block_timestamp) = log.block_timestamp {
            metadata.insert(BlockTimestamp::METADATA_KEY, block_timestamp);
        }

        // Insert job-specific metadata
        metadata.insert(ServiceId::METADATA_KEY, data.serviceId);
        metadata.insert(CallId::METADATA_KEY, data.callId);
        metadata.insert(JobIndex::METADATA_KEY, data.jobIndex as u64);
        metadata.insert(Caller::METADATA_KEY, data.caller.as_slice());

        // Build extensions - store inputs and full log
        let mut extensions = Extensions::new();
        extensions.insert(Bytes::copy_from_slice(&data.inputs));
        extensions.insert(vec![log.clone()]);

        // Create job call with job_index as the ID and inputs as body
        let parts = Parts::new(data.jobIndex as u32)
            .with_metadata(metadata)
            .with_extensions(extensions);

        job_calls.push(JobCall::from_parts(parts, Bytes::copy_from_slice(&data.inputs)));

        blueprint_core::trace!(
            target: "tangle-evm-producer",
            service_id = data.serviceId,
            call_id = data.callId,
            job_index = data.jobIndex,
            caller = %data.caller,
            inputs_len = data.inputs.len(),
            "Created JobCall from JobSubmitted event"
        );
    }

    job_calls
}
