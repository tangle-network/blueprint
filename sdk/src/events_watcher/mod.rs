//! ## Overview
//!
//! Event watcher traits handle the syncing and listening of events for a given network.
//! The event watcher calls into a storage for handling of important state. The run implementation
//! of an event watcher polls for blocks. Implementations of the event watcher trait define an
//! action to take when the specified event is found in a block at the `handle_event` api.

use core::time::Duration;

use futures::TryFutureExt;
use subxt::OnlineClient;

/// Tangle Event Watcher
pub mod tangle;

/// An error type for the event watcher.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// An error occurred in Subxt.
    #[error(transparent)]
    Subxt(#[from] subxt::Error),
    /// An error occurred in the backoff mechanism.
    #[error(transparent)]
    Backoff(#[from] backoff::Error<subxt::Error>),
    /// An error occurred in the event watcher and we need to restart it.
    #[error("An error occurred in the event watcher and we need to restart it.")]
    ForceRestart,
    /// An error occurred in the event handler.
    #[error(transparent)]
    Handler(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// A type alias to extract the event handler type from the event watcher.
pub type EventHandlerFor<RuntimeConfig> =
    Box<dyn EventHandler<RuntimeConfig> + Send + Sync + 'static>;

/// A trait that defines a handler for a specific set of event types.
///
/// The handlers are implemented separately from the watchers, so that we can have
/// one event watcher and many event handlers that will run in parallel.
#[async_trait::async_trait]
pub trait EventHandler<RuntimeConfig>
where
    RuntimeConfig: subxt::Config + Send + Sync + 'static,
{
    /// A method to be called with a list of events information,
    /// it is up to the handler to decide what to do with the event.
    ///
    /// If this method returned an error, the handler will be considered as failed and will
    /// be discarded. To have a retry mechanism, use the [`EventHandlerWithRetry::handle_events_with_retry`] method
    /// which does exactly what it says.
    async fn handle_events(
        &self,
        client: OnlineClient<RuntimeConfig>,
        (events, block_number): (subxt::events::Events<RuntimeConfig>, u64),
    ) -> Result<(), Error>;

    /// Whether any of the events could be handled by the handler
    async fn can_handle_events(
        &self,
        events: subxt::events::Events<RuntimeConfig>,
    ) -> Result<bool, Error>;
}

/// An Auxiliary trait to handle events with retry logic.
///
/// **Note**: This trait is automatically implemented for all the event handlers.
#[async_trait::async_trait]
pub trait EventHandlerWithRetry<RuntimeConfig>: EventHandler<RuntimeConfig>
where
    RuntimeConfig: subxt::Config + Send + Sync + 'static,
{
    /// A method to be called with the list of events information,
    /// it is up to the handler to decide what to do with these events.
    ///
    /// If this method returned an error, the handler will be considered as failed and will
    /// be retried again, depends on the retry strategy. if you do not care about the retry
    /// strategy, use the [`EventHandler::handle_events`] method instead.
    ///
    /// If this method returns Ok(true), these events will be marked as handled.
    ///
    /// **Note**: This method is automatically implemented for all the event handlers.
    async fn handle_events_with_retry(
        &self,
        client: OnlineClient<RuntimeConfig>,
        (events, block_number): (subxt::events::Events<RuntimeConfig>, u64),
        backoff: impl backoff::backoff::Backoff + Send + Sync + 'static,
    ) -> Result<(), Error> {
        if !self.can_handle_events(events.clone()).await? {
            return Ok(());
        };
        let wrapped_task = || {
            self.handle_events(client.clone(), (events.clone(), block_number))
                .map_err(backoff::Error::transient)
        };
        backoff::future::retry(backoff, wrapped_task).await?;
        Ok(())
    }
}

impl<T, C> EventHandlerWithRetry<C> for T
where
    C: subxt::Config + Send + Sync + 'static,
    T: EventHandler<C> + ?Sized,
{
}

/// Represents a Substrate event watcher.
#[async_trait::async_trait]
pub trait SubstrateEventWatcher<RuntimeConfig>
where
    RuntimeConfig: subxt::Config + Send + Sync + 'static,
{
    /// A helper unique tag to help identify the event watcher in the tracing logs.
    const TAG: &'static str;

    /// The name of the pallet that this event watcher is watching.
    const PALLET_NAME: &'static str;

    /// Returns a task that should be running in the background
    /// that will watch events
    #[tracing::instrument(
        skip_all,
        fields(tag = %Self::TAG, pallet = %Self::PALLET_NAME)
    )]
    async fn run(
        &self,
        client: OnlineClient<RuntimeConfig>,
        handlers: Vec<EventHandlerFor<RuntimeConfig>>,
    ) -> Result<(), Error> {
        const MAX_RETRY_COUNT: usize = 5;

        let backoff = backoff::ExponentialBackoff {
            max_elapsed_time: None,
            ..Default::default()
        };
        let task = || async {
            let blocks = client.blocks();
            let mut best_block: Option<u64> = None;
            loop {
                let latest_block = blocks
                    .at_latest()
                    .map_err(Into::into)
                    .map_err(backoff::Error::transient)
                    .await?;

                let latest_block_number: u64 = latest_block.number().into();

                let new_block = best_block.map(|b| b < latest_block_number);
                match new_block {
                    Some(false) => {
                        // same block, sleep for a while and try again.
                        tokio::time::sleep(Duration::from_secs(6)).await;
                        continue;
                    }
                    Some(true) | None => {
                        // first block or a new block, handle it.
                    }
                }
                let events = latest_block
                    .events()
                    .map_err(Into::into)
                    .map_err(backoff::Error::transient)
                    .await?;
                tracing::trace!("Found #{} events", events.len());
                // wraps each handler future in a retry logic, that will retry the handler
                // if it fails, up to `MAX_RETRY_COUNT`, after this it will ignore that event for
                // that specific handler.
                let tasks = handlers.iter().map(|handler| {
                    // a constant backoff with maximum retry count is used here.
                    let backoff =
                        ConstantWithMaxRetryCount::new(Duration::from_millis(100), MAX_RETRY_COUNT);
                    handler.handle_events_with_retry(
                        client.clone(),
                        (events.clone(), latest_block_number),
                        backoff,
                    )
                });
                let result = futures::future::join_all(tasks).await;
                // this event will be marked as handled if at least one handler succeeded.
                // this because, for the failed events, we arleady tried to handle them
                // many times (at this point), and there is no point in trying again.
                let mark_as_handled = result.iter().any(Result::is_ok);
                // also, for all the failed event handlers, we should print what went
                // wrong.
                for r in &result {
                    if let Err(e) = r {
                        tracing::error!("{}", e);
                    }
                }

                if mark_as_handled {
                    tracing::trace!(
                        "event handled successfully at block #{}",
                        latest_block_number
                    );
                    best_block = Some(latest_block_number);
                } else {
                    tracing::error!("Error while handling event, all handlers failed.");
                    tracing::warn!("Restarting event watcher ...");
                    // this a transient error, so we will retry again.
                    return Err(backoff::Error::transient(Error::ForceRestart));
                }
            }
        };
        // Bridge watcher backoff metric
        backoff::future::retry(backoff, task).await?;
        Ok(())
    }
}

/// Constant with Max Retry Count is a backoff policy which always returns
/// a constant duration, until it exceeds the maximum retry count.
#[derive(Debug, Clone, Copy)]
pub struct ConstantWithMaxRetryCount {
    interval: Duration,
    max_retry_count: usize,
    count: usize,
}

impl ConstantWithMaxRetryCount {
    /// Creates a new Constant backoff with `interval` and `max_retry_count`.
    /// `interval` is the duration to wait between retries, and `max_retry_count` is the maximum
    /// number of retries, after which we return `None` to indicate that we should stop retrying.
    #[must_use]
    pub fn new(interval: Duration, max_retry_count: usize) -> Self {
        Self {
            interval,
            max_retry_count,
            count: 0,
        }
    }
}

impl backoff::backoff::Backoff for ConstantWithMaxRetryCount {
    fn next_backoff(&mut self) -> Option<Duration> {
        (self.count < self.max_retry_count).then(|| {
            self.count += 1;
            self.interval
        })
    }

    fn reset(&mut self) {
        self.count = 0;
    }
}
