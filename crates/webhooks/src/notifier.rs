//! Outbound job notifications — push completion events to customers.
//!
//! Blueprints with async jobs (avatar generation, video, training) use this
//! module to notify customers when jobs complete, fail, or make progress.
//!
//! Two delivery mechanisms:
//! - **Webhook callback**: operator POSTs to a customer-provided URL with HMAC signature
//! - **SSE stream**: customer connects to `GET /v1/jobs/:id/events` and receives real-time updates
//!
//! # Usage
//!
//! ```rust,ignore
//! use blueprint_webhooks::notifier::{JobNotifier, JobEvent, JobStatus};
//!
//! let notifier = JobNotifier::new(NotifierConfig {
//!     signing_secret: "operator-hmac-key".into(),
//!     max_retries: 3,
//!     ..Default::default()
//! });
//!
//! // Customer provided a webhook_url at job creation
//! notifier.notify("job-123", JobEvent {
//!     status: JobStatus::Completed,
//!     result: Some(serde_json::json!({ "video_url": "https://..." })),
//!     ..Default::default()
//! }, Some("https://customer.com/hooks/results")).await?;
//! ```

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, broadcast};

type HmacSha256 = Hmac<Sha256>;

/// Job lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    /// Job is queued and waiting for resources.
    Queued,
    /// Job is actively being processed.
    Processing,
    /// Job completed successfully.
    Completed,
    /// Job failed.
    Failed,
    /// Job was cancelled.
    Cancelled,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::Processing => write!(f, "processing"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl JobStatus {
    /// Whether this status is terminal (no more events expected).
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

/// A job lifecycle event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobEvent {
    /// Current status.
    pub status: JobStatus,
    /// Optional progress percentage (0-100) for long-running jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u8>,
    /// Optional result payload (for completed jobs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Optional error message (for failed jobs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Unix timestamp (seconds since epoch).
    pub timestamp: u64,
}

impl Default for JobEvent {
    fn default() -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            status: JobStatus::Queued,
            progress: None,
            result: None,
            error: None,
            timestamp,
        }
    }
}

/// Webhook delivery payload sent to customer callback URLs.
#[derive(Debug, Serialize)]
struct WebhookPayload {
    job_id: String,
    event: JobEvent,
}

/// Configuration for the job notifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifierConfig {
    /// HMAC-SHA256 signing secret for webhook payloads.
    /// Customers verify signatures using this shared secret.
    pub signing_secret: String,
    /// Maximum retry attempts for failed webhook deliveries.
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    /// Base delay for exponential backoff (milliseconds).
    #[serde(default = "default_retry_base_ms")]
    pub retry_base_ms: u64,
    /// Request timeout for webhook delivery (seconds).
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
    /// SSE channel capacity per job.
    #[serde(default = "default_sse_capacity")]
    pub sse_capacity: usize,
}

fn default_max_retries() -> u32 {
    3
}
fn default_retry_base_ms() -> u64 {
    1000
}
fn default_timeout_secs() -> u64 {
    10
}
fn default_sse_capacity() -> usize {
    16
}

impl Default for NotifierConfig {
    fn default() -> Self {
        Self {
            signing_secret: String::new(),
            max_retries: default_max_retries(),
            retry_base_ms: default_retry_base_ms(),
            timeout_secs: default_timeout_secs(),
            sse_capacity: default_sse_capacity(),
        }
    }
}

/// Failed webhook delivery, stored for retry or dead-letter inspection.
#[derive(Debug, Clone, Serialize)]
pub struct FailedDelivery {
    pub job_id: String,
    pub url: String,
    pub attempts: u32,
    pub last_error: String,
    pub payload: String,
}

/// Outbound job notification hub.
///
/// Manages SSE broadcast channels per job and delivers webhook callbacks
/// to customer-provided URLs with HMAC-SHA256 signatures.
pub struct JobNotifier {
    config: NotifierConfig,
    /// Per-job SSE broadcast channels.
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<JobEvent>>>>,
    /// HTTP client for webhook delivery.
    client: reqwest::Client,
    /// Dead-letter queue for failed deliveries.
    dead_letters: Arc<RwLock<Vec<FailedDelivery>>>,
}

impl JobNotifier {
    /// Create a new notifier.
    pub fn new(config: NotifierConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .unwrap_or_default();

        Self {
            config,
            channels: Arc::new(RwLock::new(HashMap::new())),
            client,
            dead_letters: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Subscribe to SSE events for a job. Returns a broadcast receiver.
    ///
    /// If no channel exists yet, one is created. Multiple subscribers can
    /// listen to the same job.
    pub async fn subscribe(&self, job_id: &str) -> broadcast::Receiver<JobEvent> {
        let mut channels = self.channels.write().await;
        channels
            .entry(job_id.to_string())
            .or_insert_with(|| broadcast::channel(self.config.sse_capacity).0)
            .subscribe()
    }

    /// Emit a job event.
    ///
    /// Broadcasts to all SSE subscribers and, if a webhook URL is provided,
    /// delivers to the customer's callback endpoint with HMAC signature.
    pub async fn notify(
        &self,
        job_id: &str,
        event: JobEvent,
        webhook_url: Option<&str>,
    ) -> Result<(), crate::error::WebhookError> {
        let is_terminal = event.status.is_terminal();

        // Broadcast to SSE subscribers
        {
            let channels = self.channels.read().await;
            if let Some(tx) = channels.get(job_id) {
                // Ignore SendError — means no active subscribers
                let _ = tx.send(event.clone());
            }
        }

        // Deliver webhook callback
        if let Some(url) = webhook_url {
            self.deliver_webhook(job_id, &event, url).await?;
        }

        // Clean up terminal job channels
        if is_terminal {
            let mut channels = self.channels.write().await;
            channels.remove(job_id);
        }

        Ok(())
    }

    /// Deliver a webhook callback with HMAC-SHA256 signature and retries.
    async fn deliver_webhook(
        &self,
        job_id: &str,
        event: &JobEvent,
        url: &str,
    ) -> Result<(), crate::error::WebhookError> {
        let payload = serde_json::to_string(&WebhookPayload {
            job_id: job_id.to_string(),
            event: event.clone(),
        })
        .map_err(|e| crate::error::WebhookError::Server(format!("serialize: {e}")))?;

        let signature = self.sign_payload(payload.as_bytes());

        let mut last_error = String::new();
        for attempt in 0..=self.config.max_retries {
            if attempt > 0 {
                let delay = self.config.retry_base_ms * 2u64.saturating_pow(attempt - 1);
                tokio::time::sleep(Duration::from_millis(delay)).await;
                tracing::info!(job_id, attempt, url, "retrying webhook delivery");
            }

            match self
                .client
                .post(url)
                .header("Content-Type", "application/json")
                .header("X-Webhook-Signature", format!("sha256={signature}"))
                .header("X-Job-Id", job_id)
                .body(payload.clone())
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => {
                    tracing::info!(job_id, url, status = %resp.status(), "webhook delivered");
                    return Ok(());
                }
                Ok(resp) => {
                    last_error = format!("HTTP {}", resp.status());
                    tracing::warn!(
                        job_id, url, status = %resp.status(), attempt,
                        "webhook delivery got non-success status"
                    );
                }
                Err(e) => {
                    last_error = e.to_string();
                    tracing::warn!(
                        job_id, url, error = %e, attempt,
                        "webhook delivery failed"
                    );
                }
            }
        }

        // All retries exhausted — dead-letter
        let failed = FailedDelivery {
            job_id: job_id.to_string(),
            url: url.to_string(),
            attempts: self.config.max_retries + 1,
            last_error: last_error.clone(),
            payload,
        };
        tracing::error!(
            job_id,
            url,
            attempts = failed.attempts,
            "webhook delivery exhausted all retries, dead-lettered"
        );
        self.dead_letters.write().await.push(failed);

        Err(crate::error::WebhookError::Server(format!(
            "webhook delivery to {url} failed after {} attempts: {last_error}",
            self.config.max_retries + 1
        )))
    }

    /// HMAC-SHA256 sign a payload, returning hex-encoded signature.
    fn sign_payload(&self, payload: &[u8]) -> String {
        let mut mac =
            HmacSha256::new_from_slice(self.config.signing_secret.as_bytes()).expect("valid key");
        mac.update(payload);
        hex::encode(mac.finalize().into_bytes())
    }

    /// Get a snapshot of failed deliveries for monitoring/retry.
    pub async fn dead_letters(&self) -> Vec<FailedDelivery> {
        self.dead_letters.read().await.clone()
    }

    /// Clear the dead-letter queue (e.g., after manual inspection).
    pub async fn clear_dead_letters(&self) {
        self.dead_letters.write().await.clear();
    }

    /// Number of active SSE channels (jobs with subscribers).
    pub async fn active_channels(&self) -> usize {
        self.channels.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> NotifierConfig {
        NotifierConfig {
            signing_secret: "test-secret".into(),
            max_retries: 0, // no retries in tests
            ..Default::default()
        }
    }

    #[test]
    fn job_status_terminal() {
        assert!(!JobStatus::Queued.is_terminal());
        assert!(!JobStatus::Processing.is_terminal());
        assert!(JobStatus::Completed.is_terminal());
        assert!(JobStatus::Failed.is_terminal());
        assert!(JobStatus::Cancelled.is_terminal());
    }

    #[test]
    fn hmac_signature_deterministic() {
        let notifier = JobNotifier::new(test_config());
        let sig1 = notifier.sign_payload(b"hello");
        let sig2 = notifier.sign_payload(b"hello");
        assert_eq!(sig1, sig2);
        // Different payload → different sig
        let sig3 = notifier.sign_payload(b"world");
        assert_ne!(sig1, sig3);
    }

    #[test]
    fn hmac_signature_verifiable() {
        let notifier = JobNotifier::new(test_config());
        let payload = b"test payload";
        let sig_hex = notifier.sign_payload(payload);

        // Verify with the same secret
        let sig_bytes = hex::decode(&sig_hex).unwrap();
        let mut mac = HmacSha256::new_from_slice(b"test-secret").unwrap();
        mac.update(payload);
        let expected = mac.finalize().into_bytes();
        assert_eq!(expected.as_slice(), sig_bytes.as_slice());
    }

    #[tokio::test]
    async fn sse_subscribe_and_receive() {
        let notifier = JobNotifier::new(test_config());
        let mut rx = notifier.subscribe("job-1").await;

        notifier
            .notify(
                "job-1",
                JobEvent {
                    status: JobStatus::Processing,
                    progress: Some(50),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        let event = rx.recv().await.unwrap();
        assert_eq!(event.status, JobStatus::Processing);
        assert_eq!(event.progress, Some(50));
    }

    #[tokio::test]
    async fn terminal_event_cleans_up_channel() {
        let notifier = JobNotifier::new(test_config());
        let _rx = notifier.subscribe("job-2").await;
        assert_eq!(notifier.active_channels().await, 1);

        notifier
            .notify(
                "job-2",
                JobEvent {
                    status: JobStatus::Completed,
                    result: Some(serde_json::json!({"url": "https://example.com/result.mp4"})),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        assert_eq!(notifier.active_channels().await, 0);
    }

    #[tokio::test]
    async fn multiple_subscribers_same_job() {
        let notifier = JobNotifier::new(test_config());
        let mut rx1 = notifier.subscribe("job-3").await;
        let mut rx2 = notifier.subscribe("job-3").await;

        notifier
            .notify(
                "job-3",
                JobEvent {
                    status: JobStatus::Queued,
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        assert_eq!(rx1.recv().await.unwrap().status, JobStatus::Queued);
        assert_eq!(rx2.recv().await.unwrap().status, JobStatus::Queued);
    }

    #[tokio::test]
    async fn webhook_to_unreachable_url_dead_letters() {
        let notifier = JobNotifier::new(NotifierConfig {
            signing_secret: "secret".into(),
            max_retries: 0,
            timeout_secs: 1,
            ..Default::default()
        });

        let result = notifier
            .notify(
                "job-4",
                JobEvent {
                    status: JobStatus::Completed,
                    ..Default::default()
                },
                Some("http://127.0.0.1:1/nonexistent"),
            )
            .await;

        assert!(result.is_err());
        let dead = notifier.dead_letters().await;
        assert_eq!(dead.len(), 1);
        assert_eq!(dead[0].job_id, "job-4");
        assert_eq!(dead[0].attempts, 1);
    }

    #[tokio::test]
    async fn notify_without_webhook_succeeds() {
        let notifier = JobNotifier::new(test_config());
        let result = notifier
            .notify(
                "job-5",
                JobEvent {
                    status: JobStatus::Processing,
                    ..Default::default()
                },
                None,
            )
            .await;
        assert!(result.is_ok());
    }

    #[test]
    fn job_event_serialization() {
        let event = JobEvent {
            status: JobStatus::Completed,
            progress: None,
            result: Some(serde_json::json!({"video_url": "https://..."})),
            error: None,
            timestamp: 1_712_534_400,
        };
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["status"], "completed");
        assert!(json.get("progress").is_none()); // skip_serializing_if
        assert!(json.get("error").is_none());
        assert!(json["result"]["video_url"].is_string());
    }

    #[test]
    fn default_config_values() {
        let config = NotifierConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_base_ms, 1000);
        assert_eq!(config.timeout_secs, 10);
        assert_eq!(config.sse_capacity, 16);
    }
}
