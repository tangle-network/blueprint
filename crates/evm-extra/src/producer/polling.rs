//! Polling-based block and event producer for EVM chains
//!
//! This module provides producers that poll an EVM node at regular intervals to fetch new blocks
//! and event logs. The producer implements a state machine pattern to manage the polling lifecycle
//! and maintain a buffer of job calls derived from event logs.
//!
//! # State Machine Flow
//! ```text
//! [Idle] ---(interval elapsed)---> [FetchingBlocks] ---(logs received)---> [Idle]
//!    ^                                    |
//!    |                                    |
//!    +------------(error occurs)----------+
//! ```
//!
//! # Polling Process
//! 1. Starts from configured block number
//! 2. Fetches logs in configured step sizes
//! 3. Respects confirmation depth for finality
//! 4. Converts logs to job calls
//! 5. Maintains a buffer of pending jobs

use alloc::collections::VecDeque;
use alloy_provider::Provider;
use alloy_rpc_types::{Filter, Log};
use alloy_transport::TransportError;
use blueprint_core::JobCall;
use blueprint_std::sync::{Arc, Mutex};
use core::{
    fmt::Debug,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use futures::Stream;
use tokio::time::Sleep;

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

/// Configuration parameters for the polling producer
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct PollingConfig {
    start_block: StartBlockSource,
    poll_interval: Duration,
    confirmations: u64,
    step: u64,
}

impl Default for PollingConfig {
    fn default() -> Self {
        Self {
            start_block: StartBlockSource::Current(0),
            poll_interval: Duration::from_secs(1),
            confirmations: 12,
            step: 1,
        }
    }
}

impl PollingConfig {
    /// Start log collection at block 0
    pub fn from_genesis() -> PollingConfig {
        Self {
            start_block: StartBlockSource::Genesis,
            ..Default::default()
        }
    }

    /// Start log collection at the current block
    ///
    /// NOTE: This is the default
    pub fn from_current() -> PollingConfig {
        Self {
            start_block: StartBlockSource::Current(0),
            ..Default::default()
        }
    }

    /// Start log collection at a specific block
    pub fn from_block(block: u64) -> PollingConfig {
        Self {
            start_block: StartBlockSource::Custom(block),
            ..Default::default()
        }
    }

    /// Interval between polling attempts
    pub fn poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Number of blocks to wait for finality
    pub fn confirmations(mut self, confirmations: u64) -> Self {
        self.confirmations = confirmations;
        self
    }

    /// Number of blocks to fetch in each polling cycle
    pub fn step(mut self, step: u64) -> Self {
        self.step = step;
        self
    }
}

/// A streaming producer that polls an EVM chain for new events and converts them to job calls.
///
/// # State Machine
/// - `Idle`: Waits for configured interval before next polling attempt
/// - `FetchingBlocks`: Actively retrieving logs from the EVM node
///
/// # Buffer Management
/// Maintains an internal buffer of converted job calls to ensure smooth delivery
/// even when the node returns large batches of logs.
#[derive(Debug)]
pub struct PollingProducer<P: Provider> {
    provider: Arc<P>,
    filter: Filter,
    config: PollingConfig,
    state: Arc<Mutex<PollingState>>,
    buffer: VecDeque<JobCall>,
}

/// Producer state for managing the polling lifecycle
enum PollingState {
    /// Fetching the current best block number
    FetchingBlockNumber(Pin<Box<dyn Future<Output = Result<u64, TransportError>> + Send>>),
    /// Fetching logs for a specific block range
    FetchingLogs(Pin<Box<dyn Future<Output = Result<Vec<Log>, TransportError>> + Send>>),
    /// Waiting for next polling interval
    Idle(Pin<Box<Sleep>>),
}

impl Debug for PollingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FetchingBlockNumber(_) => f.debug_tuple("FetchingBlockNumber").finish(),
            Self::FetchingLogs(_) => f.debug_tuple("FetchingLogs").finish(),
            Self::Idle(_) => f.debug_tuple("Idle").finish(),
        }
    }
}

impl<P: Provider> PollingProducer<P> {
    /// Creates a new polling producer with the specified configuration
    ///
    /// # Arguments
    /// * `provider` - The EVM provider to use for fetching logs
    /// * `config` - Configuration parameters for the polling behavior
    ///
    /// # Errors
    ///
    /// If using [`PollingConfig::from_current()`], transport errors may occur when fetching the current block number.
    #[allow(clippy::cast_possible_truncation)]
    pub async fn new(provider: Arc<P>, mut config: PollingConfig) -> Result<Self, TransportError> {
        blueprint_core::info!(
            target: "evm-polling-producer",
            "new PollingProducer"
        );
        if let StartBlockSource::Current(current_block) = &mut config.start_block {
            blueprint_core::info!(
                target: "evm-polling-producer",
                "fetching new block ... "
            );
            // Add timeout to prevent hanging during initialization
            let block_number_result = tokio::time::timeout(
                Duration::from_secs(30),
                get_block_number(provider.clone())
            ).await;
            
            match block_number_result {
                Ok(Ok(block)) => {
                    blueprint_core::info!(
                        target: "evm-polling-producer",
                        block_number = block,
                        "Successfully fetched current block number"
                    );
                    *current_block = block;
                },
                Ok(Err(e)) => return Err(e),
                Err(_timeout) => {
                    blueprint_core::error!(
                        target: "evm-polling-producer",
                        "Timeout during PollingProducer initialization"
                    );
                    return Err(TransportError::LocalUsageError("Timeout during initialization".into()));
                }
            }
        }

        // Calculate initial block range accounting for confirmations
        let initial_start_block = config
            .start_block
            .number()
            .saturating_sub(config.confirmations);
        let filter = Filter::new()
            .from_block(initial_start_block)
            .to_block(initial_start_block + config.step);

        blueprint_core::info!(
            target: "evm-polling-producer",
            start_block = initial_start_block,
            step = config.step,
            confirmations = config.confirmations,
            "Initializing polling producer"
        );

        Ok(Self {
            provider,
            config,
            filter,
            state: Arc::new(Mutex::new(PollingState::Idle(Box::pin(
                tokio::time::sleep(Duration::from_micros(1)),
            )))),
            buffer: VecDeque::with_capacity(config.step as usize),
        })
    }
}

impl<P: Provider + 'static> Stream for PollingProducer<P> {
    type Item = Result<JobCall, TransportError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        blueprint_core::info!(
            target: "evm-polling-producer", "Should poll next"
        );
        loop {
            // Serve from buffer if available
            if let Some(job) = this.buffer.pop_front() {
                blueprint_core::info!(
                    target: "evm-polling-producer", "Serving job from buffer"
                );
                if !this.buffer.is_empty() {
                    cx.waker().wake_by_ref();
                }
                return Poll::Ready(Some(Ok(job)));
            }

            let mut state = this.state.lock().unwrap();
            blueprint_core::info!(
                target: "evm-polling-producer", "State: {:?}", *state
            );
            match *state {
                PollingState::Idle(ref mut fut) => match fut.as_mut().poll(cx) {
                    Poll::Ready(()) => {
                        // Transition to fetching block number
                        blueprint_core::info!(
                            target: "evm-polling-producer", "Polling interval elapsed, fetching current block number"
                        );
                        let fut = get_block_number(this.provider.clone());
                        *state = PollingState::FetchingBlockNumber(Box::pin(fut));
                        // Wake up the waker to ensure we get polled again
                        cx.waker().wake_by_ref();
                    }
                    Poll::Pending => return Poll::Pending,
                },
                PollingState::FetchingBlockNumber(ref mut fut) => match fut.as_mut().poll(cx) {
                    Poll::Ready(Ok(current_block)) => {
                        blueprint_core::info!(
                            target: "evm-polling-producer",
                            current_block,
                            "Successfully fetched current block number"
                        );
                        
                        // Calculate the highest block we can safely query considering confirmations
                        let safe_block = current_block.saturating_sub(this.config.confirmations);
                        let last_queried = this
                            .filter
                            .get_to_block()
                            .unwrap_or(this.config.start_block.number());

                        // Calculate next block range
                        let next_from_block = last_queried.saturating_add(1);
                        let proposed_to_block = last_queried.saturating_add(this.config.step);
                        let next_to_block = proposed_to_block.min(safe_block);

                        blueprint_core::info!(
                            target: "evm-polling-producer",
                            current_block,
                            safe_block,
                            next_from_block,
                            next_to_block,
                            "Calculated block range"
                        );

                        // Check if we have new blocks to process
                        if next_from_block > safe_block {
                            blueprint_core::info!(
                                target: "evm-polling-producer", 
                                next_from_block,
                                safe_block,
                                "No new blocks to process yet, waiting for next interval"
                            );
                            *state = PollingState::Idle(Box::pin(tokio::time::sleep(
                                this.config.poll_interval,
                            )));
                            // Wake up the waker to ensure we get polled again after the sleep
                            cx.waker().wake_by_ref();
                            return Poll::Pending;
                        }

                        // Update filter for next range
                        this.filter = Filter::new()
                            .from_block(next_from_block)
                            .to_block(next_to_block);

                        blueprint_core::info!(
                            from_block = next_from_block,
                            to_block = next_to_block,
                            current_block,
                            "Fetching logs for block range"
                        );

                        // Transition to fetching logs
                        let fut = get_logs(this.provider.clone(), this.filter.clone());
                        *state = PollingState::FetchingLogs(Box::pin(fut));
                        // Wake up the waker to ensure we get polled again
                        cx.waker().wake_by_ref();
                    }
                    Poll::Ready(Err(e)) => {
                        blueprint_core::info!(
                            target: "evm-polling-producer",
                            error = ?e,
                            "Failed to fetch current block number, retrying after interval"
                        );
                        // Don't return the error immediately, just go back to idle and retry
                        *state = PollingState::Idle(Box::pin(tokio::time::sleep(
                            this.config.poll_interval,
                        )));
                        continue;
                    }
                    Poll::Pending => {
                        blueprint_core::info!(
                            target: "evm-polling-producer",
                            "Still waiting for block number response"
                        );
                        return Poll::Pending;
                    }
                },
                PollingState::FetchingLogs(ref mut fut) => match fut.as_mut().poll(cx) {
                    Poll::Ready(Ok(logs)) => {
                        blueprint_core::info!(
                            target: "evm-polling-producer",
                            logs_count = logs.len(),
                            from_block = ?this.filter.get_from_block(),
                            to_block = ?this.filter.get_to_block(),
                            "Successfully fetched logs"
                        );

                        // Convert logs to job calls and buffer them
                        let job_calls = super::logs_to_job_calls(logs);
                        this.buffer.extend(job_calls);

                        // Transition back to idle state
                        *state = PollingState::Idle(Box::pin(tokio::time::sleep(
                            this.config.poll_interval,
                        )));
                    }
                    Poll::Ready(Err(e)) => {
                        blueprint_core::info!(
                            target: "evm-polling-producer",
                            error = ?e,
                            from_block = ?this.filter.get_from_block(),
                            to_block = ?this.filter.get_to_block(),
                            "Failed to fetch logs, retrying after interval"
                        );
                        // Don't return the error immediately, just go back to idle and retry
                        *state = PollingState::Idle(Box::pin(tokio::time::sleep(
                            this.config.poll_interval,
                        )));
                        continue;
                    }
                    Poll::Pending => return Poll::Pending,
                },
            }
        }
    }
}

/// Fetches the current block number from the provider
async fn get_block_number<P: Provider>(provider: P) -> Result<u64, TransportError> {
    blueprint_core::info!(
        target: "evm-polling-producer",
        "Fetching current block number from provider"
    );
    
    // Add a timeout to prevent hanging
    let result = tokio::time::timeout(
        Duration::from_secs(5), // Reduced timeout to 5 seconds for faster recovery
        provider.get_block_number()
    ).await;
    
    let result = match result {
        Ok(block_result) => {
            blueprint_core::info!(
                target: "evm-polling-producer",
                "Block number request completed"
            );
            block_result
        },
        Err(_timeout) => {
            blueprint_core::info!(
                target: "evm-polling-producer",
                "Timeout while fetching current block number, retrying will happen on next poll"
            );
            return Err(TransportError::LocalUsageError("Timeout fetching block number".into()));
        }
    };
    
    match &result {
        Ok(block_number) => {
            blueprint_core::info!(
                target: "evm-polling-producer",
                block_number = block_number,
                "Successfully fetched current block number"
            );
        }
        Err(e) => {
            blueprint_core::info!(
                target: "evm-polling-producer",
                error = ?e,
                "Failed to fetch current block number, will retry on next poll"
            );
        }
    }
    
    result
}

/// Fetches logs from the provider for the specified filter range
async fn get_logs<P: Provider>(provider: P, filter: Filter) -> Result<Vec<Log>, TransportError> {
    blueprint_core::info!(
        target: "evm-polling-producer",
        from_block = ?filter.get_from_block(),
        to_block = ?filter.get_to_block(),
        "Fetching logs from provider"
    );
    
    // Add timeout to prevent hanging
    let result = tokio::time::timeout(
        Duration::from_secs(10),
        provider.get_logs(&filter)
    ).await;
    
    let logs = match result {
        Ok(logs_result) => logs_result?,
        Err(_timeout) => {
            blueprint_core::info!(
                target: "evm-polling-producer",
                from_block = ?filter.get_from_block(),
                to_block = ?filter.get_to_block(),
                "Timeout while fetching logs, will retry on next poll"
            );
            return Err(TransportError::LocalUsageError("Timeout fetching logs".into()));
        }
    };

    blueprint_core::info!(
        target: "evm-polling-producer",
        from_block = ?filter.get_from_block(),
        to_block = ?filter.get_to_block(),
        logs_count = logs.len(),
        "Successfully fetched logs from provider"
    );
    Ok(logs)
}
