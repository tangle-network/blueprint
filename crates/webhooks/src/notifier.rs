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
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, broadcast};

type HmacSha256 = Hmac<Sha256>;

/// Maximum number of dead-letter entries retained.
const MAX_DEAD_LETTERS: usize = 1000;

/// Maximum number of concurrent SSE channels (job subscriptions).
const MAX_CHANNELS: usize = 10_000;

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
///
/// Webhook signatures include a `X-Webhook-Timestamp` header for replay
/// protection. The HMAC is computed over `"{timestamp}.{body}"` (Stripe's
/// format). Receivers should reject timestamps older than 5 minutes.
pub struct JobNotifier {
    config: NotifierConfig,
    /// Per-job SSE broadcast channels.
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<JobEvent>>>>,
    /// HTTP client for webhook delivery.
    client: reqwest::Client,
    /// Dead-letter queue for failed deliveries (capped at `MAX_DEAD_LETTERS`).
    dead_letters: Arc<RwLock<VecDeque<FailedDelivery>>>,
    /// Per-job bearer tokens for SSE authentication.
    /// Maps job_id → auth token. Populated by `register_job()`.
    job_tokens: Arc<RwLock<HashMap<String, String>>>,
}

impl JobNotifier {
    /// Create a new notifier.
    ///
    /// If `config.signing_secret` is empty, a warning is logged. Webhook
    /// delivery will be rejected at call time — SSE-only usage is fine.
    pub fn new(config: NotifierConfig) -> Self {
        if config.signing_secret.is_empty() {
            tracing::warn!(
                "notifier created with empty signing_secret — webhook delivery will be rejected"
            );
        }

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .unwrap_or_default();

        Self {
            config,
            channels: Arc::new(RwLock::new(HashMap::new())),
            client,
            dead_letters: Arc::new(RwLock::new(VecDeque::new())),
            job_tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a job and generate a crypto-random bearer token for SSE auth.
    ///
    /// Returns the hex-encoded token. The caller must pass this token to
    /// the customer so they can connect to the SSE endpoint.
    pub async fn register_job(&self, job_id: &str) -> String {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        let token: String = hex::encode(bytes);
        self.job_tokens
            .write()
            .await
            .insert(job_id.to_string(), token.clone());
        token
    }

    /// Validate a bearer token for a given job_id.
    /// Returns `true` if the token matches.
    pub async fn validate_job_token(&self, job_id: &str, token: &str) -> bool {
        let tokens = self.job_tokens.read().await;
        tokens.get(job_id).map_or(false, |expected| {
            use subtle::ConstantTimeEq;
            expected.as_bytes().ct_eq(token.as_bytes()).into()
        })
    }

    /// Subscribe to SSE events for a job. Returns a broadcast receiver.
    ///
    /// If no channel exists yet, one is created. Multiple subscribers can
    /// listen to the same job. Returns `None` if the channel cap
    /// (`MAX_CHANNELS`) is reached and this job doesn't already have one.
    pub async fn subscribe(&self, job_id: &str) -> Option<broadcast::Receiver<JobEvent>> {
        let mut channels = self.channels.write().await;
        if !channels.contains_key(job_id) && channels.len() >= MAX_CHANNELS {
            tracing::error!(
                job_id,
                max = MAX_CHANNELS,
                "SSE channel cap reached, refusing new subscription"
            );
            return None;
        }
        Some(
            channels
                .entry(job_id.to_string())
                .or_insert_with(|| broadcast::channel(self.config.sse_capacity).0)
                .subscribe(),
        )
    }

    /// Emit a job event.
    ///
    /// Broadcasts to all SSE subscribers and, if a webhook URL is provided,
    /// spawns an async task to deliver to the customer's callback endpoint
    /// with HMAC signature. Webhook delivery is fire-and-forget — failures
    /// are logged and dead-lettered but do not fail this call.
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

        // Deliver webhook callback (fire-and-forget)
        if let Some(url) = webhook_url {
            let job_id_owned = job_id.to_string();
            let event_clone = event.clone();
            let url_owned = url.to_string();
            let client = self.client.clone();
            let config = self.config.clone();
            let dead_letters = self.dead_letters.clone();

            tokio::spawn(async move {
                if let Err(e) = deliver_webhook_inner(
                    &client,
                    &config,
                    &job_id_owned,
                    &event_clone,
                    &url_owned,
                    &dead_letters,
                )
                .await
                {
                    tracing::error!(
                        job_id = %job_id_owned,
                        url = %url_owned,
                        error = %e,
                        "webhook delivery failed (fire-and-forget)"
                    );
                }
            });
        }

        // Clean up terminal job channels and tokens
        if is_terminal {
            let mut channels = self.channels.write().await;
            channels.remove(job_id);
            drop(channels);
            self.job_tokens.write().await.remove(job_id);
        }

        Ok(())
    }

    /// Get a snapshot of failed deliveries for monitoring/retry.
    pub async fn dead_letters(&self) -> Vec<FailedDelivery> {
        self.dead_letters.read().await.iter().cloned().collect()
    }

    /// Clear the dead-letter queue (e.g., after manual inspection).
    pub async fn clear_dead_letters(&self) {
        self.dead_letters.write().await.clear();
    }

    /// Number of active SSE channels (jobs with subscribers).
    pub async fn active_channels(&self) -> usize {
        self.channels.read().await.len()
    }

    /// HMAC-SHA256 sign a payload with timestamp, returning `(signature_hex, timestamp)`.
    ///
    /// The signed content is `"{timestamp}.{payload}"` (Stripe's format).
    /// Returns an error if the signing secret is empty.
    fn sign_payload_with_timestamp(
        config: &NotifierConfig,
        payload: &[u8],
    ) -> Result<(String, u64), crate::error::WebhookError> {
        if config.signing_secret.is_empty() {
            return Err(crate::error::WebhookError::DeliveryFailed(
                "cannot sign webhook: signing_secret is empty".into(),
            ));
        }
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let mut mac = HmacSha256::new_from_slice(config.signing_secret.as_bytes())
            .expect("HMAC accepts any key length");
        // Stripe format: "{timestamp}.{body}"
        mac.update(timestamp.to_string().as_bytes());
        mac.update(b".");
        mac.update(payload);
        Ok((hex::encode(mac.finalize().into_bytes()), timestamp))
    }
}

/// Standalone webhook delivery with HMAC-SHA256 signature and retries.
///
/// Runs inside a spawned task — does not block `notify()`.
async fn deliver_webhook_inner(
    client: &reqwest::Client,
    config: &NotifierConfig,
    job_id: &str,
    event: &JobEvent,
    url: &str,
    dead_letters: &Arc<RwLock<VecDeque<FailedDelivery>>>,
) -> Result<(), crate::error::WebhookError> {
    let payload = serde_json::to_string(&WebhookPayload {
        job_id: job_id.to_string(),
        event: event.clone(),
    })
    .map_err(|e| crate::error::WebhookError::Server(format!("serialize: {e}")))?;

    // H2: reject empty signing secret
    let (signature, timestamp) =
        JobNotifier::sign_payload_with_timestamp(config, payload.as_bytes())?;

    let mut last_error = String::new();
    for attempt in 0..=config.max_retries {
        if attempt > 0 {
            let delay = config.retry_base_ms * 2u64.saturating_pow(attempt - 1);
            tokio::time::sleep(Duration::from_millis(delay)).await;
            tracing::info!(job_id, attempt, url, "retrying webhook delivery");
        }

        match client
            .post(url)
            .header("Content-Type", "application/json")
            .header("X-Webhook-Signature", format!("sha256={signature}"))
            .header("X-Webhook-Timestamp", timestamp.to_string())
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

    // All retries exhausted — dead-letter (capped at MAX_DEAD_LETTERS)
    let failed = FailedDelivery {
        job_id: job_id.to_string(),
        url: url.to_string(),
        attempts: config.max_retries + 1,
        last_error: last_error.clone(),
        payload,
    };
    tracing::error!(
        job_id,
        url,
        attempts = failed.attempts,
        "webhook delivery exhausted all retries, dead-lettered"
    );
    {
        let mut dl = dead_letters.write().await;
        if dl.len() >= MAX_DEAD_LETTERS {
            dl.pop_front(); // drop oldest
        }
        dl.push_back(failed);
    }

    Err(crate::error::WebhookError::DeliveryFailed(format!(
        "webhook delivery to {url} failed after {} attempts: {last_error}",
        config.max_retries + 1
    )))
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
    fn hmac_signature_with_timestamp() {
        let config = test_config();
        let (sig1, ts1) = JobNotifier::sign_payload_with_timestamp(&config, b"hello").unwrap();
        let (sig2, ts2) = JobNotifier::sign_payload_with_timestamp(&config, b"hello").unwrap();
        // Timestamps should be the same second (or very close)
        assert!(ts2.abs_diff(ts1) <= 1);
        // Same payload at same timestamp produces same sig (timestamps may differ by 1s
        // across calls, so we just check both are valid hex)
        assert_eq!(sig1.len(), 64); // sha256 hex
        assert_eq!(sig2.len(), 64);
    }

    #[test]
    fn hmac_signature_verifiable_with_timestamp() {
        let config = test_config();
        let payload = b"test payload";
        let (sig_hex, timestamp) =
            JobNotifier::sign_payload_with_timestamp(&config, payload).unwrap();

        // Verify with the same secret and timestamp
        let sig_bytes = hex::decode(&sig_hex).unwrap();
        let mut mac = HmacSha256::new_from_slice(b"test-secret").unwrap();
        mac.update(timestamp.to_string().as_bytes());
        mac.update(b".");
        mac.update(payload);
        let expected = mac.finalize().into_bytes();
        assert_eq!(expected.as_slice(), sig_bytes.as_slice());
    }

    #[test]
    fn empty_secret_rejects_webhook_signing() {
        let config = NotifierConfig {
            signing_secret: String::new(),
            ..Default::default()
        };
        let result = JobNotifier::sign_payload_with_timestamp(&config, b"payload");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("signing_secret is empty"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn timestamp_included_in_hmac() {
        let config = test_config();
        let payload = b"same payload";
        let (sig, ts) = JobNotifier::sign_payload_with_timestamp(&config, payload).unwrap();

        // Manually compute with a different timestamp — should differ
        let fake_ts = ts + 999;
        let mut mac = HmacSha256::new_from_slice(config.signing_secret.as_bytes()).unwrap();
        mac.update(fake_ts.to_string().as_bytes());
        mac.update(b".");
        mac.update(payload);
        let wrong_sig = hex::encode(mac.finalize().into_bytes());
        assert_ne!(sig, wrong_sig, "timestamp must affect the signature");
    }

    #[tokio::test]
    async fn sse_subscribe_and_receive() {
        let notifier = JobNotifier::new(test_config());
        let mut rx = notifier.subscribe("job-1").await.unwrap();

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
        let _rx = notifier.subscribe("job-2").await.unwrap();
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
        let mut rx1 = notifier.subscribe("job-3").await.unwrap();
        let mut rx2 = notifier.subscribe("job-3").await.unwrap();

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

        // notify() no longer returns webhook errors (fire-and-forget)
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
        assert!(result.is_ok());

        // Wait for the spawned delivery task to complete
        tokio::time::sleep(Duration::from_secs(2)).await;

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

    #[tokio::test]
    async fn dead_letter_cap_works() {
        let config = NotifierConfig {
            signing_secret: "secret".into(),
            max_retries: 0,
            timeout_secs: 1,
            ..Default::default()
        };
        let notifier = JobNotifier::new(config.clone());

        // Directly fill the dead-letter queue past the cap
        {
            let mut dl = notifier.dead_letters.write().await;
            for i in 0..MAX_DEAD_LETTERS {
                dl.push_back(FailedDelivery {
                    job_id: format!("old-{i}"),
                    url: "http://example.com".into(),
                    attempts: 1,
                    last_error: "test".into(),
                    payload: "{}".into(),
                });
            }
            assert_eq!(dl.len(), MAX_DEAD_LETTERS);
        }

        // Add one more via the delivery path
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(1))
            .build()
            .unwrap();
        let event = JobEvent {
            status: JobStatus::Failed,
            ..Default::default()
        };
        let _ = deliver_webhook_inner(
            &client,
            &config,
            "overflow-job",
            &event,
            "http://127.0.0.1:1/nope",
            &notifier.dead_letters,
        )
        .await;

        let dl = notifier.dead_letters().await;
        assert_eq!(dl.len(), MAX_DEAD_LETTERS, "cap must not be exceeded");
        // Oldest entry should have been evicted
        assert_eq!(dl[0].job_id, "old-1", "oldest entry should be evicted");
        assert_eq!(
            dl.last().unwrap().job_id,
            "overflow-job",
            "newest entry should be at the end"
        );
    }

    #[tokio::test]
    async fn sse_auth_register_and_validate() {
        let notifier = JobNotifier::new(test_config());

        let token = notifier.register_job("auth-job").await;
        assert_eq!(token.len(), 64, "token should be 32 bytes hex-encoded");

        assert!(notifier.validate_job_token("auth-job", &token).await);
        assert!(!notifier.validate_job_token("auth-job", "wrong-token").await);
        assert!(!notifier.validate_job_token("no-such-job", &token).await);
    }

    // ── Real e2e: webhook delivery + HMAC verification ──────────────────

    #[tokio::test]
    async fn webhook_e2e_delivery_and_signature_verification() {
        use std::sync::Arc;
        use tokio::sync::Mutex;

        let secret = "e2e-test-secret";
        let received: Arc<Mutex<Option<(String, String, Vec<u8>)>>> = Arc::new(Mutex::new(None));
        let received_clone = received.clone();

        // Stand up a real HTTP server
        let app = axum::Router::new().route(
            "/callback",
            axum::routing::post(
                move |headers: axum::http::HeaderMap, body: axum::body::Bytes| {
                    let received = received_clone.clone();
                    async move {
                        let sig = headers
                            .get("x-webhook-signature")
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or("")
                            .to_string();
                        let ts = headers
                            .get("x-webhook-timestamp")
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or("")
                            .to_string();
                        *received.lock().await = Some((sig, ts, body.to_vec()));
                        "ok"
                    }
                },
            ),
        );

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        // Deliver a webhook to the real server
        let notifier = JobNotifier::new(NotifierConfig {
            signing_secret: secret.into(),
            max_retries: 0,
            timeout_secs: 5,
            ..Default::default()
        });

        let event = JobEvent {
            status: JobStatus::Completed,
            result: Some(serde_json::json!({"video_url": "https://cdn.example.com/out.mp4"})),
            ..Default::default()
        };

        let url = format!("http://{addr}/callback");
        notifier.notify("job-e2e", event, Some(&url)).await.unwrap();

        // Wait for spawned delivery
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Verify the server received it
        let (sig, ts, body) = received
            .lock()
            .await
            .take()
            .expect("server should have received the webhook");
        assert!(!body.is_empty(), "body should not be empty");
        assert!(!ts.is_empty(), "timestamp header must be present");

        // Parse the payload
        let payload: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(payload["job_id"], "job-e2e");
        assert_eq!(payload["event"]["status"], "completed");
        assert_eq!(
            payload["event"]["result"]["video_url"],
            "https://cdn.example.com/out.mp4"
        );

        // Verify the HMAC signature with timestamp — same way a customer would
        let sig_hex = sig
            .strip_prefix("sha256=")
            .expect("should have sha256= prefix");
        let sig_bytes = hex::decode(sig_hex).unwrap();
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(ts.as_bytes());
        mac.update(b".");
        mac.update(&body);
        let expected = mac.finalize().into_bytes();
        assert_eq!(
            expected.as_slice(),
            sig_bytes.as_slice(),
            "HMAC signature mismatch — customer verification would fail"
        );

        // Verify timestamp is recent (within 10 seconds)
        let ts_val: u64 = ts.parse().expect("timestamp should be numeric");
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert!(
            now.abs_diff(ts_val) < 10,
            "timestamp should be recent, got {ts_val} vs now {now}"
        );
    }

    #[tokio::test]
    async fn sse_e2e_full_lifecycle() {
        // Simulate: queued → processing (50%) → processing (100%) → completed
        let notifier = JobNotifier::new(test_config());

        let mut rx = notifier.subscribe("lifecycle-job").await.unwrap();

        let statuses = vec![
            (JobStatus::Queued, None, None),
            (JobStatus::Processing, Some(50), None),
            (JobStatus::Processing, Some(100), None),
            (
                JobStatus::Completed,
                None,
                Some(serde_json::json!({"url": "https://result.mp4"})),
            ),
        ];

        for (status, progress, result) in &statuses {
            notifier
                .notify(
                    "lifecycle-job",
                    JobEvent {
                        status: *status,
                        progress: *progress,
                        result: result.clone(),
                        ..Default::default()
                    },
                    None,
                )
                .await
                .unwrap();
        }

        // Receive all 4 events in order
        let e1 = rx.recv().await.unwrap();
        assert_eq!(e1.status, JobStatus::Queued);

        let e2 = rx.recv().await.unwrap();
        assert_eq!(e2.status, JobStatus::Processing);
        assert_eq!(e2.progress, Some(50));

        let e3 = rx.recv().await.unwrap();
        assert_eq!(e3.status, JobStatus::Processing);
        assert_eq!(e3.progress, Some(100));

        let e4 = rx.recv().await.unwrap();
        assert_eq!(e4.status, JobStatus::Completed);
        assert!(e4.result.is_some());

        // Channel should be cleaned up after terminal event
        assert_eq!(notifier.active_channels().await, 0);
    }
}
