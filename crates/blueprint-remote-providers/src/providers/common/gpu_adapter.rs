//! Shared helpers for GPU-focused cloud provider adapters.
//!
//! Provider HTTP, catalog selection, retirement and SSH deployment helpers.

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

/// SSH-enabled base image with Blackwell-capable CUDA, pinned to the publisher digest.
pub const DEFAULT_GPU_IMAGE: &str = "runpod/base:2.0.0-cuda1281-ubuntu2404@sha256:c0504ae25d6dd10460905476b05c1c7d1cc18ff51371cde12cc8a082e634e2e6";

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

/// Resolve GPU identity and an exact per-node count from the SDK catalog.
pub fn catalog_skus(spec: &ResourceSpec, provider: &str) -> Result<Vec<String>> {
    spec.validate().map_err(Error::ConfigurationError)?;
    let count = spec.gpu_count.unwrap_or(1);
    let mut classes = blueprint_core::gpu::catalog();
    classes.sort_by_key(|gpu| gpu.vram_gb);
    let mut result = Vec::new();
    for gpu in classes.into_iter().filter(|gpu| {
        spec.gpu_class.as_ref().is_none_or(|class| class == &gpu.id)
            && gpu.vram_gb >= spec.gpu_vram_gb.unwrap_or(0)
    }) {
        if let Some(sku) = gpu.sku(provider, count) {
            if !result.iter().any(|existing| existing == sku) {
                result.push(sku.to_string());
            }
        }
    }
    if result.is_empty() {
        return Err(Error::ConfigurationError(format!(
            "No catalog SKU for {provider}, class {:?}, count {count}",
            spec.gpu_class
        )));
    }
    Ok(result)
}

pub fn catalog_sku(spec: &ResourceSpec, provider: &str) -> Result<String> {
    Ok(catalog_skus(spec, provider)?.remove(0))
}

/// Recover the fixed-node GPU count for the legacy instance-type-only boundary.
pub fn sku_gpu_count(provider: &str, sku: &str) -> u32 {
    blueprint_core::gpu::catalog()
        .iter()
        .filter_map(|class| class.provider_skus.get(provider))
        .flat_map(|skus| skus.iter())
        .find(|(_, native)| native.as_str() == sku)
        .and_then(|(count, _)| count.parse().ok())
        .unwrap_or(1)
}

/// Synchronous mapping describes hardware only; a quote must obtain a live price.
pub fn catalog_selection(spec: &ResourceSpec, provider: &str) -> super::InstanceSelection {
    super::InstanceSelection {
        instance_type: catalog_sku(spec, provider).unwrap_or_default(),
        spot_capable: false,
        estimated_hourly_cost: None,
        gpu_class: catalog_sku(spec, provider)
            .ok()
            .and_then(|sku| catalog_class_for_sku(spec, provider, &sku).ok()),
    }
}

pub fn quoted_selection(instance_type: String, price: f64) -> Result<super::InstanceSelection> {
    if !price.is_finite() || price <= 0.0 {
        return Err(Error::ConfigurationError(
            "Provider has no valid USD price".into(),
        ));
    }
    Ok(super::InstanceSelection {
        instance_type,
        spot_capable: false,
        estimated_hourly_cost: Some(price),
        gpu_class: None,
    })
}

/// Resolve an exact native SKU to one canonical class; ambiguous SKUs need an explicit class.
pub fn catalog_class_for_sku(spec: &ResourceSpec, provider: &str, sku: &str) -> Result<String> {
    let classes: Vec<_> = blueprint_core::gpu::catalog()
        .into_iter()
        .filter(|class| {
            spec.gpu_class.as_ref().is_none_or(|id| id == &class.id)
                && class.vram_gb >= spec.gpu_vram_gb.unwrap_or(0)
                && class.sku(provider, spec.gpu_count.unwrap_or(1)) == Some(sku)
        })
        .collect();
    if classes.len() != 1 {
        return Err(Error::ConfigurationError(
            "Native GPU SKU does not resolve to one requested catalog class".into(),
        ));
    }
    Ok(classes.into_iter().next().unwrap().id)
}

pub fn quoted_gpu_selection(
    instance_type: String,
    price: f64,
    class_id: String,
) -> Result<super::InstanceSelection> {
    if blueprint_core::gpu::find(&class_id).is_none() {
        return Err(Error::ConfigurationError("Unknown quote GPU class".into()));
    }
    let mut selection = quoted_selection(instance_type, price)?;
    selection.gpu_class = Some(class_id);
    Ok(selection)
}

/// Read provider errors without including response bodies that may echo credentials.
pub async fn api_json(response: reqwest::Response) -> Result<serde_json::Value> {
    if !response.status().is_success() {
        return Err(Error::HttpError(format!(
            "Provider HTTP {}",
            response.status()
        )));
    }
    let mut response = response;
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| Error::HttpError("Provider response read failed".into()))?
    {
        if body.len().saturating_add(chunk.len()) > 10 * 1024 * 1024 {
            return Err(Error::HttpError(
                "Provider response exceeds size limit".into(),
            ));
        }
        body.extend_from_slice(&chunk);
    }
    serde_json::from_slice(&body)
        .map_err(|_| Error::HttpError("Provider returned invalid JSON".into()))
}

/// A provider's not-found response is authoritative retirement evidence.
pub async fn api_json_or_missing(response: reqwest::Response) -> Result<Option<serde_json::Value>> {
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    api_json(response).await.map(Some)
}

pub fn reject_tee(require_tee: bool) -> Result<()> {
    if require_tee {
        return Err(Error::ConfigurationError(
            "Provider has no TEE attestation path".into(),
        ));
    }
    Ok(())
}

/// Parse provider SSH connection text as data, never through a shell.
pub fn ssh_endpoint(command: &str) -> Result<String> {
    let words: Vec<&str> = command.split_whitespace().collect();
    let target = words
        .iter()
        .find(|word| word.contains('@'))
        .ok_or_else(|| Error::HttpError("Provider SSH connection has no user/host".into()))?;
    let (user, host) = target
        .split_once('@')
        .ok_or_else(|| Error::HttpError("Invalid SSH target".into()))?;
    if user.is_empty()
        || !user
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        || host.is_empty()
        || !host
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | ':' | '[' | ']'))
    {
        return Err(Error::HttpError("Invalid provider SSH target".into()));
    }
    let port = words
        .windows(2)
        .find(|pair| pair[0] == "-p")
        .map(|pair| pair[1])
        .unwrap_or("22")
        .parse::<u16>()
        .map_err(|_| Error::HttpError("Invalid SSH port".into()))?;
    if port == 0 {
        return Err(Error::HttpError("Invalid SSH port".into()));
    }
    Ok(format!("ssh://{target}:{port}"))
}

/// Return the known billable id before any readiness or access request can fail.
pub fn starting_instance(
    provider: crate::core::remote::CloudProvider,
    id: String,
    instance_type: String,
    region: String,
) -> ProvisionedInstance {
    ProvisionedInstance {
        id,
        provider,
        instance_type,
        region,
        public_ip: None,
        private_ip: None,
        status: crate::infra::types::InstanceStatus::Starting,
    }
}

/// Readiness is separate from creation so callers can persist the id first.
pub async fn wait_for_instance(
    adapter: &dyn crate::infra::traits::CloudProviderAdapter,
    id: &str,
) -> Result<ProvisionedInstance> {
    poll_until(
        "GPU instance",
        Duration::from_secs(5),
        Duration::from_secs(900),
        || async {
            let instance = adapter.get_instance_details(id).await?;
            match instance.status {
                crate::infra::types::InstanceStatus::Running if instance.public_ip.is_some() => {
                    Ok(Some(instance))
                }
                crate::infra::types::InstanceStatus::Terminated => Err(Error::HttpError(
                    "GPU instance terminated during startup".into(),
                )),
                _ => Ok(None),
            }
        },
    )
    .await
}

/// Stable ownership scope shared by provider names, native tags and reconciliation.
pub fn managed_resource_prefix(namespace: &str) -> Result<String> {
    if namespace.is_empty()
        || namespace.len() > 128
        || !namespace
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.:".contains(&b))
    {
        return Err(Error::ConfigurationError(
            "Invalid GPU lease namespace".into(),
        ));
    }
    let hash = aws_lc_rs::digest::digest(&aws_lc_rs::digest::SHA256, namespace.as_bytes());
    Ok(format!("tangle-gpu-{}_", &hex::encode(hash.as_ref())[..12]))
}

pub fn managed_resource_name(namespace: &str, lease_id: &str) -> Result<String> {
    if lease_id.is_empty()
        || lease_id.len() > 40
        || !lease_id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
    {
        return Err(Error::ConfigurationError(
            "Invalid GPU lease ownership id".into(),
        ));
    }
    Ok(format!("{}{lease_id}", managed_resource_prefix(namespace)?))
}

pub fn managed_lease_id(name: &str, namespace: &str) -> Option<String> {
    let prefix = managed_resource_prefix(namespace).ok()?;
    let lease_id = name.strip_prefix(&prefix)?;
    (managed_resource_name(namespace, lease_id).ok()?.as_str() == name)
        .then(|| lease_id.to_string())
}

/// Native VM cloud-init launches an image as a GPU container. Secrets are
/// inherited by name from the child environment, never embedded in Docker argv.
pub fn gpu_container_cloud_init(
    image: &str,
    env: &std::collections::BTreeMap<String, String>,
    ports: &[u16],
    command: Option<&[String]>,
) -> Result<String> {
    for key in env.keys() {
        if key.is_empty()
            || !key.bytes().enumerate().all(|(index, b)| {
                b == b'_' || b.is_ascii_alphabetic() || (index > 0 && b.is_ascii_digit())
            })
        {
            return Err(Error::ConfigurationError(
                "Invalid container environment variable name".into(),
            ));
        }
    }
    let configuration =
        serde_json::json!({"image":image,"env":env,"ports":ports,"command":command});
    let launch = r#"import json, os, subprocess
with open('/etc/tangle-gpu/workload.json', encoding='utf-8') as handle:
    workload = json.load(handle)
environment = dict(os.environ, **workload['env'])
arguments = ['docker', 'run', '--detach', '--restart=unless-stopped', '--gpus=all', '--name=tangle-gpu-workload']
for key in workload['env']:
    arguments.extend(['--env', key])
for port in workload['ports']:
    arguments.extend(['--publish', str(port) + ':' + str(port)])
command = workload.get('command')
arguments.append(workload['image'])
if command:
    arguments.extend(command)
subprocess.run(['systemctl', 'enable', '--now', 'docker'], check=True)
subprocess.run(arguments, env=environment, check=True, stdout=subprocess.DEVNULL)
"#;
    let document = serde_json::json!({"write_files":[
        {"path":"/etc/tangle-gpu/workload.json","owner":"root:root","permissions":"0600","content":configuration.to_string()},
        {"path":"/usr/local/lib/tangle-gpu-start.py","owner":"root:root","permissions":"0700","content":launch}
    ],"runcmd":[["python3","/usr/local/lib/tangle-gpu-start.py"]]});
    Ok(format!(
        "#cloud-config\n{}",
        serde_yaml::to_string(&document)
            .map_err(|_| Error::ConfigurationError("Invalid container cloud-init".into()))?
    ))
}

pub fn provider_hourly_price(value: &serde_json::Value) -> Option<f64> {
    value
        .as_f64()
        .or_else(|| value.as_str()?.parse().ok())
        .filter(|v| v.is_finite() && *v > 0.0)
}

pub fn provider_created_at(value: &serde_json::Value) -> Option<chrono::DateTime<chrono::Utc>> {
    if let Some(text) = value.as_str() {
        return chrono::DateTime::parse_from_rfc3339(text)
            .ok()
            .map(|date| date.with_timezone(&chrono::Utc));
    }
    let seconds = value.as_f64()?;
    if !seconds.is_finite() || seconds <= 0.0 || seconds > i64::MAX as f64 {
        return None;
    }
    chrono::DateTime::from_timestamp(seconds as i64, 0)
}

/// Validate the actual worker TLS key pair before advertising a pinned endpoint.
pub fn worker_tls_enabled(spec: &ResourceSpec) -> Result<bool> {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};
    let cert = spec.env.get("TANGLE_GPU_WORKER_TLS_CERT");
    let key = spec.env.get("TANGLE_GPU_WORKER_TLS_KEY");
    match (cert, key) {
        (None, None) => Ok(false),
        (Some(cert), Some(key)) => {
            let cert = CertificateDer::from_pem_slice(cert.as_bytes()).map_err(|_| {
                Error::ConfigurationError("Invalid worker TLS configuration".into())
            })?;
            let key = PrivateKeyDer::from_pem_slice(key.as_bytes()).map_err(|_| {
                Error::ConfigurationError("Invalid worker TLS configuration".into())
            })?;
            rustls::ServerConfig::builder_with_provider(std::sync::Arc::new(
                rustls::crypto::aws_lc_rs::default_provider(),
            ))
            .with_safe_default_protocol_versions()
            .map_err(|_| Error::ConfigurationError("Invalid worker TLS configuration".into()))?
            .with_no_client_auth()
            .with_single_cert(vec![cert], key)
            .map_err(|_| {
                Error::ConfigurationError("Worker TLS certificate does not match its key".into())
            })?;
            Ok(true)
        }
        _ => Err(Error::ConfigurationError(
            "Worker TLS requires both certificate and private key".into(),
        )),
    }
}

pub fn workload_environment(
    spec: &ResourceSpec,
    tls_mode: Option<&str>,
) -> Result<std::collections::BTreeMap<String, String>> {
    let mut environment = spec.env.clone();
    environment.insert(
        "TANGLE_GPU_REQUEST_FINGERPRINT".into(),
        crate::managed::request_fingerprint(spec)?,
    );
    if let Some(port) = spec.endpoint_port {
        environment.insert("TANGLE_GPU_LEASE_ENDPOINT_PORT".into(), port.to_string());
    }
    if let Some(mode) = tls_mode {
        environment.insert("TANGLE_GPU_LEASE_TLS_MODE".into(), mode.into());
    }
    Ok(environment)
}

pub fn workload_ports(spec: &ResourceSpec, include_ssh: bool) -> Vec<u16> {
    let mut ports = spec.ports.clone();
    if let Some(port) = spec.endpoint_port {
        ports.push(port);
    }
    if include_ssh {
        ports.push(22);
    }
    ports.sort_unstable();
    ports.dedup();
    ports
}

/// Provider GPU memory in MiB must satisfy the catalog's nominal GB capacity.
pub fn gpu_memory_mib(gb: u32) -> u64 {
    (f64::from(gb) * 1_000_000_000.0 / 1_048_576.0).ceil() as u64
}

pub fn catalog_class_for_memory(
    spec: &ResourceSpec,
    provider: &str,
    sku: &str,
    reported_mib: f64,
) -> Result<String> {
    blueprint_core::gpu::catalog()
        .into_iter()
        .filter(|class| {
            spec.gpu_class.as_ref().is_none_or(|id| id == &class.id)
                && class.vram_gb >= spec.gpu_vram_gb.unwrap_or(0)
                && class.sku(provider, spec.gpu_count.unwrap_or(1)) == Some(sku)
                && reported_mib.is_finite()
                && reported_mib >= gpu_memory_mib(class.vram_gb) as f64
        })
        .max_by_key(|class| class.vram_gb)
        .map(|class| class.id)
        .ok_or_else(|| {
            Error::ConfigurationError(
                "Native GPU memory does not prove requested catalog class".into(),
            )
        })
}
