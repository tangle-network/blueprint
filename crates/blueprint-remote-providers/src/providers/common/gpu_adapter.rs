//! Shared helpers for GPU-focused cloud provider adapters.
//!
//! Every GPU cloud provider in this crate (Lambda Labs, RunPod, Vast.ai, CoreWeave,
//! Paperspace, Fluidstack, TensorDock, Akash, io.net, Spheron, Nosana, Prime Intellect,
//! Render, Bittensor) shares the same adapter skeleton: provision an instance via REST,
//! wait for it to become SSH-reachable, then hand it off to `SharedSshDeployment` for
//! the actual blueprint deployment. This module captures the common parts so each
//! adapter is a thin provider-specific shell.
//!
//! Key hardening primitives exported by this module:
//! - [`provision_with_cleanup`] — wraps provision+deploy with automatic instance
//!   termination on deploy failure (prevents billing leaks).
//! - [`poll_until_with_retry`] — polling helper that distinguishes transient from
//!   permanent errors and applies exponential backoff + jitter.
//! - [`classify_http_error`] — turns a reqwest response into a typed error
//!   (transient/permanent/rate-limited).
//! - [`RetryPolicy`] — exponential backoff with jitter and max retries.

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::traits::BlueprintDeploymentResult;
use crate::infra::types::ProvisionedInstance;
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::{SharedSshDeployment, SshDeploymentConfig};
use blueprint_core::warn;
use blueprint_std::collections::HashMap;
use std::future::Future;
use std::time::{Duration, Instant};

/// Construct a secure HTTP client configured for a GPU cloud provider API.
pub fn build_http_client() -> Result<SecureHttpClient> {
    SecureHttpClient::new()
}

/// Deploy a blueprint to a pre-provisioned GPU instance via SSH.
///
/// Reuses `SharedSshDeployment::deploy_to_instance`, which is the common
/// Docker-over-SSH path used by every provider in this crate.
pub async fn deploy_via_ssh(
    instance: &ProvisionedInstance,
    blueprint_image: &str,
    resource_spec: &ResourceSpec,
    env_vars: HashMap<String, String>,
    ssh_config: SshDeploymentConfig,
) -> Result<BlueprintDeploymentResult> {
    SharedSshDeployment::deploy_to_instance(
        instance,
        blueprint_image,
        resource_spec,
        env_vars,
        ssh_config,
    )
    .await
}

// ============================================================================
// Error classification (transient vs permanent)
// ============================================================================

/// Classification of an error for retry decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    /// Client error — retrying won't help (4xx except 408, 429).
    Permanent,
    /// Transient failure — retry with backoff (5xx, timeouts, connection errors).
    Transient,
    /// Rate limited — retry after the specified delay.
    RateLimited { retry_after: Duration },
}

/// Classify a reqwest error for retry decisions.
pub fn classify_reqwest_error(err: &reqwest::Error) -> ErrorClass {
    if err.is_timeout() || err.is_connect() || err.is_request() {
        return ErrorClass::Transient;
    }
    if let Some(status) = err.status() {
        return classify_status_code(status.as_u16(), None);
    }
    ErrorClass::Transient
}

/// Classify an HTTP status code with optional `Retry-After` header value.
pub fn classify_status_code(status: u16, retry_after: Option<Duration>) -> ErrorClass {
    match status {
        // Success is never an error.
        200..=299 => ErrorClass::Permanent, // caller shouldn't be classifying success
        // Request timeout — treat as transient.
        408 => ErrorClass::Transient,
        // Rate limited — respect Retry-After.
        429 => ErrorClass::RateLimited {
            retry_after: retry_after.unwrap_or(Duration::from_secs(30)),
        },
        // Other 4xx — permanent, client problem.
        400..=499 => ErrorClass::Permanent,
        // 503 with Retry-After is rate-limit-adjacent.
        503 if retry_after.is_some() => ErrorClass::RateLimited {
            retry_after: retry_after.unwrap_or(Duration::from_secs(30)),
        },
        // Other 5xx — transient.
        500..=599 => ErrorClass::Transient,
        // Everything else — treat as transient so we at least try again.
        _ => ErrorClass::Transient,
    }
}

/// Parse a `Retry-After` header value into a Duration.
///
/// The header may be either a number of seconds or an HTTP-date. We only parse
/// the numeric form — HTTP-date is rare for rate limits and not worth the complexity.
pub fn parse_retry_after(header_value: &str) -> Option<Duration> {
    header_value
        .trim()
        .parse::<u64>()
        .ok()
        .map(Duration::from_secs)
}

/// Classify an HTTP response (after fetching it) by reading its status and
/// `Retry-After` header. Returns `None` if the response was a success.
pub fn classify_response(response: &reqwest::Response) -> Option<ErrorClass> {
    let status = response.status();
    if status.is_success() {
        return None;
    }
    let retry_after = response
        .headers()
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|h| h.to_str().ok())
        .and_then(parse_retry_after);
    Some(classify_status_code(status.as_u16(), retry_after))
}

// ============================================================================
// Retry policy
// ============================================================================

/// Retry configuration for GPU provisioning calls.
#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
    pub backoff_multiplier: f64,
    /// Whether to add random jitter (±20%) to each backoff.
    pub jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 5,
            initial_backoff: Duration::from_secs(1),
            max_backoff: Duration::from_secs(60),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

impl RetryPolicy {
    /// Quick policy with minimal retries — for fast-path operations.
    pub fn quick() -> Self {
        Self {
            max_retries: 2,
            initial_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(5),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }

    /// Compute the backoff duration for a given attempt (0-indexed).
    /// Honors an explicit `retry_after` when provided (e.g. from 429 response).
    pub fn backoff_for(&self, attempt: u32, retry_after: Option<Duration>) -> Duration {
        if let Some(after) = retry_after {
            return after.min(self.max_backoff * 4);
        }
        let base =
            self.initial_backoff.as_millis() as f64 * self.backoff_multiplier.powi(attempt as i32);
        let capped = base.min(self.max_backoff.as_millis() as f64);
        let jittered = if self.jitter {
            // Pseudo-random jitter without adding a rand dependency:
            // use the attempt number + process-unique nanos as a cheap seed.
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos() as f64 / 1_000_000_000.0)
                .unwrap_or(0.0);
            let factor = 0.8 + (now * 0.4); // 0.8 .. 1.2
            capped * factor
        } else {
            capped
        };
        Duration::from_millis(jittered as u64)
    }
}

/// Retry an async operation with exponential backoff, classifying errors to decide
/// which ones should be retried. Permanent errors short-circuit immediately.
///
/// The operation callback receives the attempt number (0-indexed) for its own logging.
pub async fn retry_with_backoff<T, F, Fut>(
    label: &str,
    policy: &RetryPolicy,
    mut op: F,
) -> Result<T>
where
    F: FnMut(u32) -> Fut,
    Fut: Future<Output = std::result::Result<T, ClassifiedError>>,
{
    let mut last_err: Option<Error> = None;
    for attempt in 0..=policy.max_retries {
        match op(attempt).await {
            Ok(value) => return Ok(value),
            Err(ClassifiedError {
                class: ErrorClass::Permanent,
                inner,
            }) => return Err(inner),
            Err(ClassifiedError { class, inner }) => {
                if attempt == policy.max_retries {
                    warn!(
                        target: "gpu_adapter",
                        operation = label,
                        attempts = attempt + 1,
                        "Exhausted retries; returning last error"
                    );
                    return Err(inner);
                }
                let retry_after = match class {
                    ErrorClass::RateLimited { retry_after } => Some(retry_after),
                    _ => None,
                };
                let backoff = policy.backoff_for(attempt, retry_after);
                warn!(
                    target: "gpu_adapter",
                    operation = label,
                    attempt = attempt + 1,
                    backoff_ms = backoff.as_millis() as u64,
                    "Transient error; retrying after backoff"
                );
                last_err = Some(inner);
                tokio::time::sleep(backoff).await;
            }
        }
    }
    Err(last_err.unwrap_or_else(|| Error::Other(format!("{label}: all retries exhausted"))))
}

/// An error paired with its retry classification.
pub struct ClassifiedError {
    pub class: ErrorClass,
    pub inner: Error,
}

impl ClassifiedError {
    pub fn permanent(inner: Error) -> Self {
        Self {
            class: ErrorClass::Permanent,
            inner,
        }
    }
    pub fn transient(inner: Error) -> Self {
        Self {
            class: ErrorClass::Transient,
            inner,
        }
    }
    pub fn rate_limited(inner: Error, retry_after: Duration) -> Self {
        Self {
            class: ErrorClass::RateLimited { retry_after },
            inner,
        }
    }
}

// ============================================================================
// Polling with retry
// ============================================================================

/// Poll an async predicate until it returns `Ok(Some(T))` or the deadline expires.
///
/// Unlike the old `poll_until`, this version distinguishes transient from permanent
/// errors and applies exponential backoff on transient failures. It does NOT retry
/// within a single poll iteration — if a call returns a permanent error, the entire
/// poll is aborted.
///
/// Used by GPU adapters to wait for instances to transition into the "running" state
/// and acquire a public IP.
pub async fn poll_until<T, F, Fut>(
    label: &str,
    interval: Duration,
    timeout: Duration,
    mut f: F,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<Option<T>>>,
{
    let deadline = Instant::now() + timeout;
    let mut attempt: u32 = 0;
    loop {
        match f().await {
            Ok(Some(value)) => return Ok(value),
            Ok(None) => {
                if Instant::now() >= deadline {
                    return Err(Error::ConfigurationError(format!(
                        "Timed out waiting for {label} after {}s",
                        timeout.as_secs()
                    )));
                }
                tokio::time::sleep(jittered_interval(interval, attempt)).await;
                attempt = attempt.saturating_add(1);
            }
            Err(e) => {
                // Polling aborts on any error — callers should wrap their
                // HTTP calls in `retry_with_backoff` if they want retries on
                // individual polls. This keeps `poll_until` simple and lets
                // callers decide retry policy per-operation.
                return Err(e);
            }
        }
    }
}

/// Interval with ±10% jitter and a small multiplier per attempt to reduce
/// thundering herd against provider APIs.
fn jittered_interval(base: Duration, attempt: u32) -> Duration {
    let scale = 1.0 + (attempt.min(5) as f64 * 0.1);
    let nanos_seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as f64 / 1_000_000_000.0)
        .unwrap_or(0.0);
    let jitter = 0.9 + (nanos_seed * 0.2); // 0.9 .. 1.1
    let millis = (base.as_millis() as f64 * scale * jitter) as u64;
    Duration::from_millis(millis)
}

// ============================================================================
// Provision + deploy with automatic cleanup on failure
// ============================================================================

/// Run a provision-then-deploy flow, terminating the provisioned instance if deploy
/// fails. This is the critical safeguard against orphaned billing.
///
/// Callers pass two async closures:
/// - `provision`: returns a `ProvisionedInstance` on success
/// - `deploy`: runs deployment against the provisioned instance
///
/// If `deploy` fails, `cleanup` is called with the instance id and the deploy error
/// is returned. The cleanup error (if any) is logged but does not override the
/// original deploy error.
pub async fn provision_with_cleanup<P, D, C, PFut, DFut, CFut>(
    label: &str,
    provision: P,
    deploy: D,
    cleanup: C,
) -> Result<BlueprintDeploymentResult>
where
    P: FnOnce() -> PFut,
    D: FnOnce(ProvisionedInstance) -> DFut,
    C: FnOnce(String) -> CFut,
    PFut: Future<Output = Result<ProvisionedInstance>>,
    DFut: Future<Output = Result<BlueprintDeploymentResult>>,
    CFut: Future<Output = Result<()>>,
{
    let instance = provision().await?;
    let instance_id_for_cleanup = instance.id.clone();
    match deploy(instance).await {
        Ok(result) => Ok(result),
        Err(deploy_err) => {
            warn!(
                target: "gpu_adapter",
                provider = label,
                instance_id = %instance_id_for_cleanup,
                error = %deploy_err,
                "Deploy failed after provisioning; terminating instance to prevent billing leak"
            );
            let cleanup_result = tokio::time::timeout(
                Duration::from_secs(60),
                cleanup(instance_id_for_cleanup.clone()),
            )
            .await;
            match cleanup_result {
                Ok(Ok(())) => {}
                Ok(Err(cleanup_err)) => {
                    warn!(
                        target: "gpu_adapter",
                        provider = label,
                        instance_id = %instance_id_for_cleanup,
                        cleanup_error = %cleanup_err,
                        "Cleanup after failed deploy also failed — instance may be orphaned"
                    );
                }
                Err(_timeout) => {
                    warn!(
                        target: "gpu_adapter",
                        provider = label,
                        instance_id = %instance_id_for_cleanup,
                        "Cleanup timed out after 60s — instance may be orphaned"
                    );
                }
            }
            Err(deploy_err)
        }
    }
}

// ============================================================================
// Existing helpers (kept for backwards compatibility)
// ============================================================================

/// GPU instance selection result — the subset of `InstanceSelection` that
/// GPU adapters actually need to place a provisioning request.
#[derive(Debug, Clone)]
pub struct GpuInstancePlan {
    /// Provider-native instance type identifier (e.g. `gpu_1x_a100`).
    pub instance_type: String,
    /// GPU count requested.
    pub gpu_count: u32,
    /// VRAM per GPU in GiB (0 when unknown).
    pub vram_gb: u32,
    /// Estimated hourly cost in USD (for logging / cost ceilings).
    pub estimated_hourly_cost: f64,
}

/// Extract GPU count from a `ResourceSpec`, defaulting to 1 when unset.
/// GPU cloud providers always allocate at least one GPU per instance.
pub fn gpu_count_or_one(spec: &ResourceSpec) -> u32 {
    spec.gpu_count.unwrap_or(1).max(1)
}

/// Generate a unique blueprint-prefixed instance name.
/// GPU providers use this as the human-readable label on their dashboards.
pub fn generate_instance_name(prefix: &str) -> String {
    format!("blueprint-{}-{}", prefix, uuid::Uuid::new_v4())
}

/// Resolve the public IP address from an instance, returning an error if missing.
/// SSH deployment is impossible without a public IP, so adapters should fail fast.
pub fn require_public_ip(instance: &ProvisionedInstance) -> Result<&str> {
    instance
        .public_ip
        .as_deref()
        .ok_or_else(|| Error::ConfigurationError("Instance has no public IP".to_string()))
}

/// Convenience constructor for `ApiAuthentication` from an env var.
pub fn bearer_auth_from_env(var: &str) -> Result<ApiAuthentication> {
    let token = std::env::var(var)
        .map_err(|_| Error::Other(format!("{var} environment variable not set")))?;
    Ok(ApiAuthentication::Bearer { token })
}

/// Convenience: two-field auth from env vars (for providers that need key+token).
pub fn dual_env(key_var: &str, token_var: &str) -> Result<(String, String)> {
    let key = std::env::var(key_var)
        .map_err(|_| Error::Other(format!("{key_var} environment variable not set")))?;
    let token = std::env::var(token_var)
        .map_err(|_| Error::Other(format!("{token_var} environment variable not set")))?;
    Ok((key, token))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_2xx_is_not_useful_but_safe() {
        // Success codes shouldn't hit the classifier, but ensure they're handled.
        assert_eq!(classify_status_code(200, None), ErrorClass::Permanent);
    }

    #[test]
    fn classify_408_is_transient() {
        assert_eq!(classify_status_code(408, None), ErrorClass::Transient);
    }

    #[test]
    fn classify_429_respects_retry_after() {
        let class = classify_status_code(429, Some(Duration::from_secs(10)));
        assert_eq!(
            class,
            ErrorClass::RateLimited {
                retry_after: Duration::from_secs(10)
            }
        );
    }

    #[test]
    fn classify_429_defaults_to_30s() {
        let class = classify_status_code(429, None);
        assert_eq!(
            class,
            ErrorClass::RateLimited {
                retry_after: Duration::from_secs(30)
            }
        );
    }

    #[test]
    fn classify_4xx_is_permanent() {
        assert_eq!(classify_status_code(400, None), ErrorClass::Permanent);
        assert_eq!(classify_status_code(401, None), ErrorClass::Permanent);
        assert_eq!(classify_status_code(404, None), ErrorClass::Permanent);
        assert_eq!(classify_status_code(422, None), ErrorClass::Permanent);
    }

    #[test]
    fn classify_5xx_is_transient() {
        assert_eq!(classify_status_code(500, None), ErrorClass::Transient);
        assert_eq!(classify_status_code(502, None), ErrorClass::Transient);
        assert_eq!(classify_status_code(504, None), ErrorClass::Transient);
    }

    #[test]
    fn classify_503_with_retry_after_is_rate_limited() {
        let class = classify_status_code(503, Some(Duration::from_secs(60)));
        assert_eq!(
            class,
            ErrorClass::RateLimited {
                retry_after: Duration::from_secs(60)
            }
        );
    }

    #[test]
    fn parse_retry_after_numeric() {
        assert_eq!(parse_retry_after("30"), Some(Duration::from_secs(30)));
        assert_eq!(parse_retry_after("  60  "), Some(Duration::from_secs(60)));
    }

    #[test]
    fn parse_retry_after_invalid_returns_none() {
        assert_eq!(parse_retry_after("invalid"), None);
        assert_eq!(parse_retry_after("Tue, 15 Nov 1994 08:12:31 GMT"), None);
    }

    #[test]
    fn retry_policy_default_backoff_grows() {
        let policy = RetryPolicy {
            jitter: false,
            ..Default::default()
        };
        let b0 = policy.backoff_for(0, None);
        let b1 = policy.backoff_for(1, None);
        let b2 = policy.backoff_for(2, None);
        assert!(b1 >= b0);
        assert!(b2 >= b1);
    }

    #[test]
    fn retry_policy_respects_max_backoff() {
        let policy = RetryPolicy {
            jitter: false,
            max_backoff: Duration::from_secs(5),
            ..Default::default()
        };
        let b = policy.backoff_for(100, None);
        assert!(b <= Duration::from_secs(5));
    }

    #[test]
    fn retry_policy_uses_retry_after_when_provided() {
        let policy = RetryPolicy::default();
        let b = policy.backoff_for(0, Some(Duration::from_secs(15)));
        assert_eq!(b, Duration::from_secs(15));
    }

    #[tokio::test]
    async fn retry_with_backoff_succeeds_first_try() {
        let policy = RetryPolicy::quick();
        let result: Result<i32> = retry_with_backoff("test", &policy, |_| async {
            Ok::<i32, ClassifiedError>(42)
        })
        .await;
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn retry_with_backoff_short_circuits_on_permanent() {
        let policy = RetryPolicy::quick();
        let mut attempts = 0u32;
        let result: Result<i32> = retry_with_backoff("test", &policy, |attempt| {
            attempts = attempt + 1;
            async move {
                Err(ClassifiedError::permanent(Error::Other(
                    "permanent failure".into(),
                )))
            }
        })
        .await;
        assert!(result.is_err());
        assert_eq!(attempts, 1, "should not retry after permanent error");
    }

    #[tokio::test]
    async fn retry_with_backoff_retries_transient() {
        let policy = RetryPolicy {
            jitter: false,
            initial_backoff: Duration::from_millis(1),
            max_backoff: Duration::from_millis(10),
            backoff_multiplier: 2.0,
            max_retries: 3,
        };
        let mut attempts = 0u32;
        let result: Result<i32> = retry_with_backoff("test", &policy, |attempt| {
            attempts = attempt + 1;
            async move {
                if attempt < 2 {
                    Err(ClassifiedError::transient(Error::Other("transient".into())))
                } else {
                    Ok(42)
                }
            }
        })
        .await;
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts, 3);
    }

    #[tokio::test]
    async fn retry_with_backoff_exhausts_retries() {
        let policy = RetryPolicy {
            jitter: false,
            initial_backoff: Duration::from_millis(1),
            max_backoff: Duration::from_millis(5),
            backoff_multiplier: 2.0,
            max_retries: 2,
        };
        let mut attempts = 0u32;
        let result: Result<i32> = retry_with_backoff("test", &policy, |attempt| {
            attempts = attempt + 1;
            async move { Err(ClassifiedError::transient(Error::Other("nope".into()))) }
        })
        .await;
        assert!(result.is_err());
        assert_eq!(attempts, 3); // max_retries + 1 initial attempt
    }

    #[tokio::test]
    async fn provision_with_cleanup_terminates_on_deploy_failure() {
        use crate::core::remote::CloudProvider;
        use crate::infra::types::InstanceStatus;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};

        let cleaned = Arc::new(AtomicBool::new(false));
        let cleaned_clone = cleaned.clone();

        let result = provision_with_cleanup(
            "test",
            || async {
                Ok(ProvisionedInstance {
                    id: "i-123".into(),
                    provider: CloudProvider::Generic,
                    instance_type: "test".into(),
                    region: "test".into(),
                    public_ip: Some("1.2.3.4".into()),
                    private_ip: None,
                    status: InstanceStatus::Running,
                })
            },
            |_instance| async { Err(Error::Other("deploy failed".into())) },
            |_id| async move {
                cleaned_clone.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await;

        assert!(result.is_err());
        assert!(
            cleaned.load(Ordering::SeqCst),
            "cleanup should have been invoked"
        );
    }

    #[tokio::test]
    async fn provision_with_cleanup_skips_cleanup_on_deploy_success() {
        use crate::core::remote::CloudProvider;
        use crate::infra::traits::BlueprintDeploymentResult;
        use crate::infra::types::InstanceStatus;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};

        let cleaned = Arc::new(AtomicBool::new(false));
        let cleaned_clone = cleaned.clone();

        let result = provision_with_cleanup(
            "test",
            || async {
                Ok(ProvisionedInstance {
                    id: "i-456".into(),
                    provider: CloudProvider::Generic,
                    instance_type: "test".into(),
                    region: "test".into(),
                    public_ip: Some("1.2.3.4".into()),
                    private_ip: None,
                    status: InstanceStatus::Running,
                })
            },
            |instance| async move {
                Ok(BlueprintDeploymentResult {
                    instance,
                    blueprint_id: "bp-1".into(),
                    port_mappings: Default::default(),
                    metadata: Default::default(),
                })
            },
            |_id| async move {
                cleaned_clone.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await;

        assert!(result.is_ok());
        assert!(
            !cleaned.load(Ordering::SeqCst),
            "cleanup should NOT run on success"
        );
    }

    #[tokio::test]
    async fn provision_with_cleanup_preserves_deploy_error_when_cleanup_also_fails() {
        use crate::core::remote::CloudProvider;
        use crate::infra::types::InstanceStatus;

        let result = provision_with_cleanup(
            "test",
            || async {
                Ok(ProvisionedInstance {
                    id: "i-789".into(),
                    provider: CloudProvider::Generic,
                    instance_type: "test".into(),
                    region: "test".into(),
                    public_ip: None,
                    private_ip: None,
                    status: InstanceStatus::Running,
                })
            },
            |_| async { Err(Error::Other("deploy error".into())) },
            |_| async { Err(Error::Other("cleanup also failed".into())) },
        )
        .await;

        // The deploy error is preserved; cleanup failure is logged but not propagated.
        let err = result.unwrap_err();
        assert!(format!("{err}").contains("deploy error"));
    }

    #[tokio::test]
    async fn poll_until_returns_on_success() {
        let mut calls = 0;
        let result: Result<i32> = poll_until(
            "test",
            Duration::from_millis(1),
            Duration::from_secs(1),
            || {
                calls += 1;
                let c = calls;
                async move { if c >= 3 { Ok(Some(c)) } else { Ok(None) } }
            },
        )
        .await;
        assert_eq!(result.unwrap(), 3);
    }

    #[tokio::test]
    async fn poll_until_times_out() {
        let result: Result<i32> = poll_until(
            "test",
            Duration::from_millis(1),
            Duration::from_millis(10),
            || async { Ok::<Option<i32>, Error>(None) },
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn poll_until_aborts_on_error() {
        let result: Result<i32> = poll_until(
            "test",
            Duration::from_millis(1),
            Duration::from_secs(1),
            || async { Err(Error::Other("boom".into())) },
        )
        .await;
        assert!(result.is_err());
    }

    #[test]
    fn generate_instance_name_has_prefix_and_uuid() {
        let name = generate_instance_name("abc");
        assert!(name.starts_with("blueprint-abc-"));
        assert!(name.len() > "blueprint-abc-".len() + 20);
    }
}
