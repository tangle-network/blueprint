//! Tangle EVM Consumer
//!
//! Consumes [`JobResult`]s and submits them to the Tangle EVM contract.

use crate::extract;
use alloc::collections::VecDeque;
use blueprint_client_tangle_evm::TangleEvmClient;
use blueprint_core::error::BoxError;
use blueprint_core::JobResult;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_util::Sink;
use std::sync::Mutex;

/// Error type for the consumer
#[derive(Debug, thiserror::Error)]
pub enum ConsumerError {
    /// Client error
    #[error("Client error: {0}")]
    Client(String),
    /// Missing metadata
    #[error("Missing required metadata: {0}")]
    MissingMetadata(&'static str),
    /// Invalid metadata value
    #[error("Invalid metadata value for {0}")]
    InvalidMetadata(&'static str),
}

/// A derived job result with extracted metadata
struct DerivedJobResult {
    call_id: u64,
    service_id: u64,
    output: alloc::vec::Vec<u8>,
}

enum State {
    WaitingForResult,
    SubmittingResult(Pin<Box<dyn core::future::Future<Output = Result<(), ConsumerError>> + Send>>),
}

impl State {
    fn is_waiting(&self) -> bool {
        matches!(self, State::WaitingForResult)
    }
}

/// A consumer of Tangle EVM [`JobResult`]s
pub struct TangleEvmConsumer {
    client: TangleEvmClient,
    buffer: VecDeque<DerivedJobResult>,
    state: Mutex<State>,
}

impl TangleEvmConsumer {
    /// Create a new [`TangleEvmConsumer`]
    pub fn new(client: TangleEvmClient) -> Self {
        Self {
            client,
            buffer: VecDeque::new(),
            state: Mutex::new(State::WaitingForResult),
        }
    }

    /// Get the client
    #[must_use]
    pub fn client(&self) -> &TangleEvmClient {
        &self.client
    }
}

impl Sink<JobResult> for TangleEvmConsumer {
    type Error = BoxError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: JobResult) -> Result<(), Self::Error> {
        let JobResult::Ok { head, body } = &item else {
            // Discard error results
            blueprint_core::trace!(
                target: "tangle-evm-consumer",
                "Discarding job result with error"
            );
            return Ok(());
        };

        // Extract required metadata
        let call_id_raw = head
            .metadata
            .get(extract::CallId::METADATA_KEY)
            .ok_or_else(|| ConsumerError::MissingMetadata("call_id"))?;
        let service_id_raw = head
            .metadata
            .get(extract::ServiceId::METADATA_KEY)
            .ok_or_else(|| ConsumerError::MissingMetadata("service_id"))?;

        let call_id: u64 = call_id_raw
            .try_into()
            .map_err(|_| ConsumerError::InvalidMetadata("call_id"))?;
        let service_id: u64 = service_id_raw
            .try_into()
            .map_err(|_| ConsumerError::InvalidMetadata("service_id"))?;

        blueprint_core::debug!(
            target: "tangle-evm-consumer",
            "Received job result for service_id={}, call_id={}",
            service_id,
            call_id
        );

        self.get_mut().buffer.push_back(DerivedJobResult {
            call_id,
            service_id,
            output: body.to_vec(),
        });

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let consumer = self.get_mut();
        let mut state = consumer.state.lock().unwrap();

        if consumer.buffer.is_empty() && state.is_waiting() {
            return Poll::Ready(Ok(()));
        }

        loop {
            match &mut *state {
                State::WaitingForResult => {
                    let Some(DerivedJobResult {
                        call_id,
                        service_id,
                        output,
                    }) = consumer.buffer.pop_front()
                    else {
                        return Poll::Ready(Ok(()));
                    };

                    let client = consumer.client.clone();
                    let fut = Box::pin(async move {
                        submit_result(client, service_id, call_id, output).await
                    });

                    *state = State::SubmittingResult(fut);
                }
                State::SubmittingResult(future) => match future.as_mut().poll(cx) {
                    Poll::Ready(Ok(())) => {
                        *state = State::WaitingForResult;
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(e.into())),
                    Poll::Pending => return Poll::Pending,
                },
            }
        }
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.buffer.is_empty() {
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }
}

/// Submit a job result to the Tangle contract
async fn submit_result(
    _client: TangleEvmClient,
    service_id: u64,
    call_id: u64,
    output: alloc::vec::Vec<u8>,
) -> Result<(), ConsumerError> {
    // TODO: Implement actual transaction submission
    // This requires:
    // 1. Building the submitResult transaction data
    // 2. Signing with the operator's key
    // 3. Broadcasting and waiting for confirmation

    blueprint_core::info!(
        target: "tangle-evm-consumer",
        "Submitting result for service_id={}, call_id={}, output_len={}",
        service_id,
        call_id,
        output.len()
    );

    // For now, log the intent - full implementation requires transaction signing
    // which needs the keystore integration
    blueprint_core::warn!(
        target: "tangle-evm-consumer",
        "Transaction submission not yet implemented - result logged only"
    );

    Ok(())
}
