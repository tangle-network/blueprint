//! Channel-based producer that feeds verified webhook events into the Blueprint runner.

use blueprint_core::error::BoxError;
use blueprint_core::job::call::Parts;
use blueprint_core::metadata::{MetadataMap, MetadataValue};
use blueprint_core::{JobCall, JobId};
use bytes::Bytes;
use futures_core::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

/// Metadata key marking this job as webhook-originated.
pub const WEBHOOK_ORIGIN_KEY: &str = "X-WEBHOOK-ORIGIN";
/// Metadata key for the webhook endpoint path that triggered this job.
pub const WEBHOOK_PATH_KEY: &str = "X-WEBHOOK-PATH";
/// Metadata key for the webhook endpoint name.
pub const WEBHOOK_NAME_KEY: &str = "X-WEBHOOK-NAME";
/// Metadata key for the service ID.
pub const WEBHOOK_SERVICE_ID_KEY: &str = "X-TANGLE-SERVICE-ID";
/// Metadata key for a synthetic call ID.
pub const WEBHOOK_CALL_ID_KEY: &str = "X-TANGLE-CALL-ID";

/// A verified webhook event ready to be converted into a [`JobCall`].
#[derive(Debug, Clone)]
pub struct WebhookEvent {
    /// Service instance ID.
    pub service_id: u64,
    /// Job ID to trigger.
    pub job_id: u64,
    /// Raw request body from the webhook.
    pub body: Bytes,
    /// The webhook endpoint path that received this event.
    pub path: String,
    /// Human-readable endpoint name (if configured).
    pub name: Option<String>,
    /// Synthetic call ID for tracking.
    pub call_id: u64,
}

impl WebhookEvent {
    /// Convert into a [`JobCall`] with webhook-specific metadata.
    pub fn into_job_call(self) -> JobCall {
        let mut metadata = MetadataMap::new();
        metadata.insert(WEBHOOK_ORIGIN_KEY, MetadataValue::from("webhook"));
        metadata.insert(WEBHOOK_PATH_KEY, MetadataValue::from(self.path.as_str()));
        if let Some(ref name) = self.name {
            metadata.insert(WEBHOOK_NAME_KEY, MetadataValue::from(name.as_str()));
        }
        metadata.insert(WEBHOOK_SERVICE_ID_KEY, MetadataValue::from(self.service_id));
        metadata.insert(WEBHOOK_CALL_ID_KEY, MetadataValue::from(self.call_id));

        let parts = Parts::new(JobId::from(self.job_id)).with_metadata(metadata);

        JobCall::from_parts(parts, self.body)
    }
}

/// A producer stream that yields [`JobCall`]s from webhook events.
///
/// Created via [`WebhookProducer::channel`].
pub struct WebhookProducer {
    rx: mpsc::UnboundedReceiver<WebhookEvent>,
}

impl WebhookProducer {
    /// Create a new producer and its corresponding sender.
    pub fn channel() -> (Self, mpsc::UnboundedSender<WebhookEvent>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { rx }, tx)
    }
}

impl Stream for WebhookProducer {
    type Item = Result<JobCall, BoxError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.rx.poll_recv(cx) {
            Poll::Ready(Some(event)) => {
                tracing::info!(
                    job_id = event.job_id,
                    path = %event.path,
                    name = ?event.name,
                    "webhook event verified, producing JobCall"
                );
                Poll::Ready(Some(Ok(event.into_job_call())))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_producer_receives_events() {
        let (mut producer, tx) = WebhookProducer::channel();

        let event = WebhookEvent {
            service_id: 42,
            job_id: 30,
            body: Bytes::from_static(b"{\"action\":\"buy\"}"),
            path: "/hooks/tradingview".into(),
            name: Some("TradingView Alert".into()),
            call_id: 1,
        };

        tx.send(event).unwrap();
        drop(tx);

        let job_call = producer.next().await.unwrap().unwrap();
        assert_eq!(job_call.job_id(), JobId::from(30u64));

        let origin = job_call.metadata().get(WEBHOOK_ORIGIN_KEY).unwrap();
        assert_eq!(origin.as_bytes(), b"webhook");

        let path = job_call.metadata().get(WEBHOOK_PATH_KEY).unwrap();
        assert_eq!(path.as_bytes(), b"/hooks/tradingview");
    }

    #[test]
    fn test_webhook_event_to_job_call() {
        let event = WebhookEvent {
            service_id: 1,
            job_id: 7,
            body: Bytes::from_static(b"price_alert"),
            path: "/hooks/price".into(),
            name: None,
            call_id: 99,
        };

        let call = event.into_job_call();
        assert_eq!(call.job_id(), JobId::from(7u64));
        assert_eq!(call.body(), &Bytes::from_static(b"price_alert"));
        assert!(call.metadata().get(WEBHOOK_ORIGIN_KEY).is_some());
        assert!(call.metadata().get(WEBHOOK_NAME_KEY).is_none());
    }
}
