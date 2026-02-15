//! Channel-based producer that feeds verified x402 payments into the Blueprint runner.
//!
//! When the x402 gateway verifies a payment, it sends a [`VerifiedPayment`] through
//! an internal channel. The [`X402Producer`] wraps the receiving end as a
//! [`Stream<Item = Result<JobCall, BoxError>>`], which the runner consumes
//! alongside other producers (e.g. `TangleProducer`).

use blueprint_core::error::BoxError;
use blueprint_core::metadata::{MetadataMap, MetadataValue};
use blueprint_core::{JobCall, JobId};
use blueprint_core::job::call::Parts;
use bytes::Bytes;
use futures_core::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

/// Metadata key for the x402 quote digest that triggered this job.
pub const X402_QUOTE_DIGEST_KEY: &str = "X-X402-QUOTE-DIGEST";
/// Metadata key for the x402 payment network (CAIP-2).
pub const X402_PAYMENT_NETWORK_KEY: &str = "X-X402-PAYMENT-NETWORK";
/// Metadata key for the x402 payment token.
pub const X402_PAYMENT_TOKEN_KEY: &str = "X-X402-PAYMENT-TOKEN";
/// Metadata key marking this job as x402-originated.
pub const X402_ORIGIN_KEY: &str = "X-X402-ORIGIN";
/// Metadata key for the service ID.
pub const X402_SERVICE_ID_KEY: &str = "X-TANGLE-SERVICE-ID";
/// Metadata key for a synthetic call ID.
pub const X402_CALL_ID_KEY: &str = "X-TANGLE-CALL-ID";

/// A verified x402 payment ready to be converted into a [`JobCall`].
#[derive(Debug, Clone)]
pub struct VerifiedPayment {
    /// Service instance ID.
    pub service_id: u64,
    /// Job type index.
    pub job_index: u32,
    /// Job input arguments (raw bytes from the HTTP request body).
    pub job_args: Bytes,
    /// The quote digest that was consumed.
    pub quote_digest: [u8; 32],
    /// Which chain the payment was made on.
    pub payment_network: String,
    /// Which token was used.
    pub payment_token: String,
    /// Synthetic call ID for tracking.
    pub call_id: u64,
}

impl VerifiedPayment {
    /// Convert into a [`JobCall`] with x402-specific metadata.
    pub fn into_job_call(self) -> JobCall {
        let mut metadata = MetadataMap::new();
        metadata.insert(X402_ORIGIN_KEY, MetadataValue::from("x402"));
        metadata.insert(X402_QUOTE_DIGEST_KEY, MetadataValue::from(Bytes::copy_from_slice(&self.quote_digest)));
        metadata.insert(X402_PAYMENT_NETWORK_KEY, MetadataValue::from(self.payment_network.as_str()));
        metadata.insert(X402_PAYMENT_TOKEN_KEY, MetadataValue::from(self.payment_token.as_str()));
        metadata.insert(X402_SERVICE_ID_KEY, MetadataValue::from(self.service_id));
        metadata.insert(X402_CALL_ID_KEY, MetadataValue::from(self.call_id));

        let parts = Parts::new(JobId::from(self.job_index as u64))
            .with_metadata(metadata);

        JobCall::from_parts(parts, self.job_args)
    }
}

/// A producer stream that yields [`JobCall`]s from verified x402 payments.
///
/// Created via [`X402Producer::channel`], which returns both the producer
/// (for the runner) and a sender (for the gateway).
pub struct X402Producer {
    rx: mpsc::UnboundedReceiver<VerifiedPayment>,
}

impl X402Producer {
    /// Create a new producer and its corresponding sender.
    ///
    /// The sender is given to the [`X402Gateway`](crate::X402Gateway) to inject
    /// verified payments. The producer is given to the
    /// [`BlueprintRunner`](blueprint_runner::BlueprintRunner) as a job source.
    pub fn channel() -> (Self, mpsc::UnboundedSender<VerifiedPayment>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { rx }, tx)
    }
}

impl Stream for X402Producer {
    type Item = Result<JobCall, BoxError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.rx.poll_recv(cx) {
            Poll::Ready(Some(payment)) => {
                tracing::info!(
                    service_id = payment.service_id,
                    job_index = payment.job_index,
                    network = %payment.payment_network,
                    "x402 payment verified, producing JobCall"
                );
                Poll::Ready(Some(Ok(payment.into_job_call())))
            }
            Poll::Ready(None) => {
                // Channel closed — gateway shut down
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_producer_receives_payments() {
        let (mut producer, tx) = X402Producer::channel();

        let payment = VerifiedPayment {
            service_id: 42,
            job_index: 3,
            job_args: Bytes::from_static(b"hello"),
            quote_digest: [0xAB; 32],
            payment_network: "eip155:8453".into(),
            payment_token: "USDC".into(),
            call_id: 1,
        };

        tx.send(payment).unwrap();
        drop(tx);

        let job_call = producer.next().await.unwrap().unwrap();
        assert_eq!(job_call.job_id(), JobId::from(3u64));

        let origin = job_call.metadata().get(X402_ORIGIN_KEY).unwrap();
        assert_eq!(origin.as_bytes(), b"x402");
    }

    #[test]
    fn test_verified_payment_to_job_call() {
        let payment = VerifiedPayment {
            service_id: 1,
            job_index: 0,
            job_args: Bytes::from_static(b"\x00\x01"),
            quote_digest: [0xFF; 32],
            payment_network: "eip155:1".into(),
            payment_token: "USDC".into(),
            call_id: 99,
        };

        let call = payment.into_job_call();
        assert_eq!(call.job_id(), JobId::from(0u64));
        assert_eq!(call.body(), &Bytes::from_static(b"\x00\x01"));
        assert!(call.metadata().get(X402_ORIGIN_KEY).is_some());
        assert!(call.metadata().get(X402_SERVICE_ID_KEY).is_some());
    }
}
