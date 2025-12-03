//! Tangle EVM Consumer
//!
//! Consumes [`JobResult`]s and submits them to the Tangle EVM contract.

use crate::extract;
use alloy_primitives::Bytes;
use blueprint_client_tangle_evm::TangleEvmClient;
use blueprint_core::error::BoxError;
use blueprint_core::JobResult;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_util::Sink;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Error type for the consumer
#[derive(Debug, thiserror::Error)]
pub enum ConsumerError {
    /// Client error
    #[error("Client error: {0}")]
    Client(String),
    /// Missing metadata
    #[error("Missing metadata: {0}")]
    MissingMetadata(&'static str),
    /// Invalid metadata
    #[error("Invalid metadata: {0}")]
    InvalidMetadata(&'static str),
    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),
}

/// Derived job result for submission
struct DerivedJobResult {
    service_id: u64,
    call_id: u64,
    output: Bytes,
}

enum State {
    WaitingForResult,
    ProcessingSubmission(
        Pin<Box<dyn core::future::Future<Output = Result<(), ConsumerError>> + Send>>,
    ),
}

impl State {
    fn is_waiting(&self) -> bool {
        matches!(self, State::WaitingForResult)
    }
}

/// A consumer of Tangle EVM [`JobResult`]s
pub struct TangleEvmConsumer {
    client: Arc<TangleEvmClient>,
    buffer: Mutex<VecDeque<DerivedJobResult>>,
    state: Mutex<State>,
}

impl TangleEvmConsumer {
    /// Create a new [`TangleEvmConsumer`]
    pub fn new(client: TangleEvmClient) -> Self {
        Self {
            client: Arc::new(client),
            buffer: Mutex::new(VecDeque::new()),
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
            // We don't care about errors here
            blueprint_core::trace!(target: "tangle-evm-consumer", "Discarding job result with error");
            return Ok(());
        };

        let (Some(call_id_raw), Some(service_id_raw)) = (
            head.metadata.get(extract::CallId::METADATA_KEY),
            head.metadata.get(extract::ServiceId::METADATA_KEY),
        ) else {
            // Not a tangle EVM job result
            blueprint_core::trace!(target: "tangle-evm-consumer", "Discarding job result with missing metadata");
            return Ok(());
        };

        blueprint_core::debug!(target: "tangle-evm-consumer", result = ?item, "Received job result, handling...");

        let call_id: u64 = call_id_raw
            .try_into()
            .map_err(|_| ConsumerError::InvalidMetadata("call_id"))?;
        let service_id: u64 = service_id_raw
            .try_into()
            .map_err(|_| ConsumerError::InvalidMetadata("service_id"))?;

        self.get_mut().buffer.lock().unwrap().push_back(DerivedJobResult {
            service_id,
            call_id,
            output: Bytes::copy_from_slice(body),
        });
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let consumer = self.get_mut();
        let mut state = consumer.state.lock().unwrap();

        {
            let buffer = consumer.buffer.lock().unwrap();
            if buffer.is_empty() && state.is_waiting() {
                return Poll::Ready(Ok(()));
            }
        }

        loop {
            match &mut *state {
                State::WaitingForResult => {
                    let result = {
                        let mut buffer = consumer.buffer.lock().unwrap();
                        buffer.pop_front()
                    };

                    let Some(DerivedJobResult {
                        service_id,
                        call_id,
                        output,
                    }) = result
                    else {
                        return Poll::Ready(Ok(()));
                    };

                    let client = Arc::clone(&consumer.client);
                    let fut = Box::pin(async move {
                        submit_result(client, service_id, call_id, output).await
                    });

                    *state = State::ProcessingSubmission(fut);
                }
                State::ProcessingSubmission(future) => match future.as_mut().poll(cx) {
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
        let buffer = self.buffer.lock().unwrap();
        if buffer.is_empty() {
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }
}

/// Submit a result to the Tangle contract
async fn submit_result(
    client: Arc<TangleEvmClient>,
    service_id: u64,
    call_id: u64,
    output: Bytes,
) -> Result<(), ConsumerError> {
    blueprint_core::debug!(
        target: "tangle-evm-consumer",
        "Submitting result for service {} call {}",
        service_id,
        call_id
    );

    // Get the contract instance
    let contract = client.tangle_contract();

    // Call submitResult
    // Note: This requires a signer. For now we just do a call to check it works.
    // In production, we'd need to sign and send the transaction.
    let _call = contract.submitResult(service_id, call_id, output);

    // TODO: Sign and send the transaction
    // For now, log that we would submit
    blueprint_core::info!(
        target: "tangle-evm-consumer",
        "Would submit result for service {} call {} (signing not implemented yet)",
        service_id,
        call_id
    );

    Ok(())
}
