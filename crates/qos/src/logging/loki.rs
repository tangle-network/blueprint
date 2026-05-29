//! Logging and trace export for blueprint operators.
//!
//! Two independent sinks live here:
//!   * **Loki** — log aggregation for the Grafana stack (existing behaviour).
//!   * **OTLP/HTTP-JSON trace export** — ships the operator's `tracing` spans to a
//!     remote OpenTelemetry collector, by default the Tangle Intelligence platform
//!     (`https://intelligence.tangle.tools/v1/otlp`). The Intelligence adapter
//!     parses `application/json` only (protobuf 400s), so the exporter uses
//!     `Protocol::HttpJson` with `Authorization: Bearer <key>`.
//!
//! Both are composed into a single global tracing subscriber by
//! [`init_telemetry`], which returns a [`TelemetryGuard`] that flushes and shuts
//! the trace exporter down on drop so in-flight spans aren't lost on a clean
//! exit. Hold the guard for the process lifetime (the QoS service does this).
//!
//! ## Enabling trace export
//! Export turns on when a base endpoint or API key is resolved from, in order:
//!   1. [`OtelConfig::endpoint`] / [`OtelConfig::bearer_token`] / [`OtelConfig::headers`]
//!   2. `TANGLE_API_KEY`            → `Authorization: Bearer` + the default endpoint
//!   3. `OTEL_EXPORTER_OTLP_ENDPOINT` → an arbitrary collector base
//!   4. `OTEL_EXPORTER_OTLP_HEADERS` (`k=v,k=v`) → extra headers, passthrough
//!
//! With none of these set the operator keeps its stdout/Loki logs and exports
//! nothing (zero behaviour change). Init never aborts startup: a telemetry
//! misconfig logs a warning and falls back to logs-only — an observability
//! problem must not take down an operator.

use blueprint_core::error;
use std::collections::HashMap;
use std::time::Duration;
use tracing_loki::url::Url;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

use opentelemetry_031::KeyValue;
use opentelemetry_031::trace::TracerProvider as _;
use opentelemetry_otlp::{Protocol, SpanExporter, WithExportConfig, WithHttpConfig};
use opentelemetry_sdk_031::Resource;
use opentelemetry_sdk_031::trace::SdkTracerProvider;

// Default values for LokiConfig
const DEFAULT_LOKI_LABEL_SERVICE_KEY: &str = "service";
const DEFAULT_LOKI_LABEL_SERVICE_VALUE: &str = "blueprint";
const DEFAULT_LOKI_LABEL_ENVIRONMENT_KEY: &str = "environment";
const DEFAULT_LOKI_LABEL_ENVIRONMENT_VALUE: &str = "development";
const DEFAULT_LOKI_URL: &str = "http://localhost:3100";
const DEFAULT_LOKI_BATCH_SIZE: usize = 100;
const DEFAULT_LOKI_TIMEOUT_SECS: u64 = 5;

/// Default Tangle Intelligence OTLP base. The exporter posts to `<base>/v1/traces`.
const DEFAULT_OTLP_BASE: &str = "https://intelligence.tangle.tools/v1/otlp";
const TRACES_PATH: &str = "/v1/traces";
const EXPORT_TIMEOUT_SECS: u64 = 10;

use crate::error::Result;

/// Configuration for Loki log aggregation integration.
///
/// This structure defines settings for connecting to and sending logs to a Loki server,
/// which is part of the Grafana observability stack. Loki is designed for storing and
/// querying log data, providing an efficient way to centralize logs from Blueprint services.
/// The configuration includes connection details, authentication, log batching parameters,
/// and custom labels that will be attached to all logs sent to Loki.
#[derive(Clone, Debug)]
pub struct LokiConfig {
    /// Loki server URL
    pub url: String,

    /// Basic auth username (optional)
    pub username: Option<String>,

    /// Basic auth password (optional)
    pub password: Option<String>,

    /// Labels to attach to all logs
    pub labels: HashMap<String, String>,

    /// Batch size for sending logs
    pub batch_size: usize,

    /// Timeout for sending logs
    pub timeout_secs: u64,

    /// OpenTelemetry trace-export configuration
    pub otel_config: Option<OtelConfig>,
}

impl Default for LokiConfig {
    fn default() -> Self {
        let mut labels = HashMap::new();
        labels.insert(
            DEFAULT_LOKI_LABEL_SERVICE_KEY.to_string(),
            DEFAULT_LOKI_LABEL_SERVICE_VALUE.to_string(),
        );
        labels.insert(
            DEFAULT_LOKI_LABEL_ENVIRONMENT_KEY.to_string(),
            DEFAULT_LOKI_LABEL_ENVIRONMENT_VALUE.to_string(),
        );

        Self {
            url: DEFAULT_LOKI_URL.to_string(),
            username: None,
            password: None,
            labels,
            batch_size: DEFAULT_LOKI_BATCH_SIZE,
            timeout_secs: DEFAULT_LOKI_TIMEOUT_SECS,
            otel_config: None,
        }
    }
}

/// OTLP/HTTP-JSON trace-export configuration.
///
/// When present (and an endpoint or key resolves — see the module docs) blueprint
/// `tracing` spans are bridged to a remote OpenTelemetry collector. All fields are
/// optional: with an empty `OtelConfig::default()` the export is driven purely by
/// the `TANGLE_API_KEY` / `OTEL_EXPORTER_OTLP_ENDPOINT` / `OTEL_EXPORTER_OTLP_HEADERS`
/// environment variables, so enabling export is config- *or* env-only.
#[derive(Clone, Debug, Default)]
pub struct OtelConfig {
    /// Collector base URL, e.g. `https://intelligence.tangle.tools/v1/otlp`. The
    /// exporter posts to `<endpoint>/v1/traces`. `None` falls back to
    /// `OTEL_EXPORTER_OTLP_ENDPOINT`, then the Tangle Intelligence default.
    pub endpoint: Option<String>,

    /// Bearer token sent as `Authorization: Bearer <token>`. `None` falls back to
    /// `TANGLE_API_KEY`.
    pub bearer_token: Option<String>,

    /// Additional export headers, merged on top of the bearer token and the
    /// `OTEL_EXPORTER_OTLP_HEADERS` passthrough (these take precedence).
    pub headers: HashMap<String, String>,

    /// Maximum attributes per span (`None` keeps the SDK default).
    pub max_attributes_per_span: Option<u32>,
}

/// Held for the process lifetime; flushes queued spans and shuts the OTLP
/// exporter down on drop so in-flight traces aren't lost on a clean exit.
///
/// A guard with no provider (export disabled) is inert.
#[derive(Default)]
pub struct TelemetryGuard {
    provider: Option<SdkTracerProvider>,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Some(provider) = self.provider.take() {
            let _ = provider.force_flush();
            let _ = provider.shutdown();
        }
    }
}

/// Initializes the global tracing subscriber with all configured sinks.
///
/// Always installs a `fmt` + `EnvFilter` layer (preserving today's stdout logs
/// driven by `RUST_LOG`). Additionally:
///   * a Loki layer when `loki_config` carries a real (parseable, reachable) URL,
///   * an OTLP/HTTP-JSON trace-export layer when export is enabled (see module docs).
///
/// Returns a [`TelemetryGuard`] owning the trace provider; drop it on shutdown to
/// flush. Installing the global subscriber more than once is a no-op (a warning is
/// logged) — this is the single entry point the QoS service calls.
///
/// # Parameters
/// * `loki_config` - Loki connection + embedded [`OtelConfig`] for trace export
/// * `service_name` - the OTel `service.name` resource attribute the dashboard groups by
#[must_use]
pub fn init_telemetry(loki_config: &LokiConfig, service_name: &str) -> TelemetryGuard {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = tracing_subscriber::fmt::layer();

    // Build the OTLP provider/layer first so we can report whether export is on.
    let (provider, otel_layer) =
        match build_otlp_provider(loki_config.otel_config.as_ref(), service_name) {
            Some((provider, endpoint)) => {
                let tracer = provider.tracer(service_name.to_string());
                blueprint_core::info!(
                    target: "telemetry",
                    service = service_name,
                    endpoint = %endpoint,
                    "OTLP trace export enabled"
                );
                (
                    Some(provider),
                    Some(tracing_opentelemetry::layer().with_tracer(tracer)),
                )
            }
            None => (None, None),
        };

    // Build the Loki layer + background task only for a real URL. The default
    // localhost URL is treated as "no Loki" so a misconfigured/absent Loki never
    // breaks startup.
    let loki_layer = build_loki_layer(loki_config);

    let registry = Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .with(otel_layer)
        .with(loki_layer);

    if registry.try_init().is_err() {
        // A subscriber is already installed (e.g. another init, or a test). The
        // exporter provider is still returned so the caller can flush it.
        blueprint_core::warn!(
            target: "telemetry",
            "global tracing subscriber already set; telemetry layers from this init were not installed"
        );
    }

    TelemetryGuard { provider }
}

/// Builds the Loki layer and spawns its background task, or `None` when no real
/// Loki URL is configured / the layer fails to build (logs-only fallback).
fn build_loki_layer(config: &LokiConfig) -> Option<tracing_loki::Layer> {
    // Skip the unconfigured default — operators without Loki shouldn't pay for it.
    if config.url == DEFAULT_LOKI_URL {
        return None;
    }

    let url = match Url::parse(&config.url) {
        Ok(url) => url,
        Err(e) => {
            error!("Failed to parse Loki URL '{}': {}", config.url, e);
            return None;
        }
    };

    let mut builder = tracing_loki::builder();
    for (key, value) in &config.labels {
        builder = match builder.label(key.clone(), value.clone()) {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to add label to Loki layer: {}", e);
                return None;
            }
        };
    }

    match builder.build_url(url) {
        Ok((layer, task)) => {
            tokio::spawn(task);
            Some(layer)
        }
        Err(e) => {
            error!("Failed to build Loki layer: {}", e);
            None
        }
    }
}

/// Builds the OTLP tracer provider plus the resolved traces endpoint, or `None`
/// when export is disabled or the exporter fails to construct (logs-only
/// fallback — never abort startup).
fn build_otlp_provider(
    otel: Option<&OtelConfig>,
    service_name: &str,
) -> Option<(SdkTracerProvider, String)> {
    // Resolve the base endpoint: explicit config → env → default (only when a key
    // is present). Export is disabled unless *some* endpoint or key is provided.
    let cfg_endpoint = otel.and_then(|c| non_empty(c.endpoint.as_deref()));
    let env_endpoint = non_empty_env("OTEL_EXPORTER_OTLP_ENDPOINT");
    let cfg_token = otel.and_then(|c| non_empty(c.bearer_token.as_deref()));
    let env_token = non_empty_env("TANGLE_API_KEY");

    let api_key = cfg_token.or(env_token);
    let base = cfg_endpoint.or(env_endpoint);

    if base.is_none() && api_key.is_none() {
        return None;
    }

    let endpoint = traces_endpoint(base.as_deref().unwrap_or(DEFAULT_OTLP_BASE));
    let headers = build_headers(api_key.as_deref(), otel.map(|c| &c.headers));

    let exporter = match SpanExporter::builder()
        .with_http()
        // The Intelligence OTLP adapter is JSON-only; protobuf would 400.
        .with_protocol(Protocol::HttpJson)
        // Programmatic endpoint is used verbatim (no `/v1/traces` auto-append),
        // so `traces_endpoint` builds the full path itself.
        .with_endpoint(endpoint.clone())
        .with_headers(headers)
        .with_timeout(Duration::from_secs(EXPORT_TIMEOUT_SECS))
        .build()
    {
        Ok(exporter) => exporter,
        Err(err) => {
            error!("OTLP exporter init failed ({err}); trace export disabled, logs only");
            return None;
        }
    };

    let resource = Resource::builder()
        .with_attributes([
            KeyValue::new("service.name", service_name.to_string()),
            KeyValue::new("service.version", env!("CARGO_PKG_VERSION").to_string()),
        ])
        .build();

    let provider = SdkTracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter)
        .build();

    Some((provider, endpoint))
}

/// Assembles export headers: the Tangle bearer token, the standard
/// `OTEL_EXPORTER_OTLP_HEADERS` (`k1=v1,k2=v2`) passthrough, then any explicit
/// config headers (which take precedence).
fn build_headers(
    api_key: Option<&str>,
    extra: Option<&HashMap<String, String>>,
) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    if let Some(key) = api_key {
        headers.insert("Authorization".to_string(), format!("Bearer {key}"));
    }
    if let Some(raw) = non_empty_env("OTEL_EXPORTER_OTLP_HEADERS") {
        for pair in raw.split(',') {
            if let Some((k, v)) = pair.split_once('=') {
                headers.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
    }
    if let Some(extra) = extra {
        for (k, v) in extra {
            headers.insert(k.clone(), v.clone());
        }
    }
    headers
}

fn non_empty(value: Option<&str>) -> Option<String> {
    value
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn non_empty_env(key: &str) -> Option<String> {
    non_empty(std::env::var(key).ok().as_deref())
}

/// Normalizes an OTLP base URL to the full traces path: `<base>` → `<base>/v1/traces`.
/// Idempotent if the caller already included `/v1/traces`; trims trailing slashes.
fn traces_endpoint(base: &str) -> String {
    let base = base.trim_end_matches('/');
    if base.ends_with(TRACES_PATH) {
        base.to_string()
    } else {
        format!("{base}{TRACES_PATH}")
    }
}

/// Initializes Loki logging (and OTLP trace export when configured).
///
/// Backwards-compatible entry point. Composes the global tracing subscriber via
/// [`init_telemetry`] and returns once installed. Prefer [`init_telemetry`]
/// directly when you need the [`TelemetryGuard`] to flush traces on shutdown;
/// this wrapper leaks the guard (process-lifetime telemetry) for callers that
/// don't manage it.
///
/// # Errors
/// Never returns an error — a telemetry misconfig falls back to logs-only.
pub fn init_loki_logging(config: LokiConfig) -> Result<()> {
    let service_name = config
        .labels
        .get(DEFAULT_LOKI_LABEL_SERVICE_KEY)
        .cloned()
        .unwrap_or_else(|| DEFAULT_LOKI_LABEL_SERVICE_VALUE.to_string());
    let guard = init_telemetry(&config, &service_name);
    // Process-lifetime telemetry for the legacy signature: keep the exporter
    // alive without a guard handle. The QoS service uses init_telemetry directly.
    std::mem::forget(guard);
    Ok(())
}

/// Initializes Loki logging with OpenTelemetry trace export.
///
/// Backwards-compatible entry point that takes an explicit `service_name` for the
/// OTel `service.name` resource attribute. See [`init_telemetry`].
///
/// # Errors
/// Never returns an error — a telemetry misconfig falls back to logs-only.
pub fn init_loki_with_opentelemetry(loki_config: &LokiConfig, service_name: &str) -> Result<()> {
    let guard = init_telemetry(loki_config, service_name);
    std::mem::forget(guard);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the `LokiConfig` default implementation returns a valid configuration.
    #[test]
    fn test_loki_config_default() {
        let config = LokiConfig::default();
        assert_eq!(config.url, "http://localhost:3100");
        assert_eq!(config.batch_size, 100);
        assert!(config.otel_config.is_none());
    }

    #[test]
    fn otel_config_default_is_empty() {
        let cfg = OtelConfig::default();
        assert!(cfg.endpoint.is_none());
        assert!(cfg.bearer_token.is_none());
        assert!(cfg.headers.is_empty());
        assert!(cfg.max_attributes_per_span.is_none());
    }

    #[test]
    fn traces_endpoint_appends_once() {
        assert_eq!(
            traces_endpoint("https://intelligence.tangle.tools/v1/otlp"),
            "https://intelligence.tangle.tools/v1/otlp/v1/traces"
        );
        // trailing slash is trimmed before append
        assert_eq!(
            traces_endpoint("https://intelligence.tangle.tools/v1/otlp/"),
            "https://intelligence.tangle.tools/v1/otlp/v1/traces"
        );
        // idempotent when the path is already present
        assert_eq!(
            traces_endpoint("http://localhost:4318/v1/traces"),
            "http://localhost:4318/v1/traces"
        );
    }

    #[test]
    fn headers_carry_bearer_and_passthrough() {
        let h = build_headers(Some("sk-tan-abc"), None);
        assert_eq!(h.get("Authorization").unwrap(), "Bearer sk-tan-abc");
        // no key → no Authorization header (e.g. local unauthenticated collector)
        assert!(!build_headers(None, None).contains_key("Authorization"));
    }

    #[test]
    fn headers_explicit_config_overrides() {
        let mut extra = HashMap::new();
        extra.insert("Authorization".to_string(), "Bearer override".to_string());
        extra.insert("X-Tenant".to_string(), "acme".to_string());
        let h = build_headers(Some("sk-tan-abc"), Some(&extra));
        // explicit config header wins over the derived bearer token
        assert_eq!(h.get("Authorization").unwrap(), "Bearer override");
        assert_eq!(h.get("X-Tenant").unwrap(), "acme");
    }

    #[test]
    fn provider_disabled_without_endpoint_or_key() {
        // No config, and (in a clean env) no TANGLE_API_KEY / endpoint → disabled.
        // Guard against a polluted env by asserting only when both are unset.
        if std::env::var("TANGLE_API_KEY").is_err()
            && std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").is_err()
        {
            assert!(build_otlp_provider(None, "svc").is_none());
            assert!(build_otlp_provider(Some(&OtelConfig::default()), "svc").is_none());
        }
    }

    #[test]
    fn provider_enabled_with_config_endpoint() {
        let cfg = OtelConfig {
            endpoint: Some("http://127.0.0.1:4318".to_string()),
            ..Default::default()
        };
        let built = build_otlp_provider(Some(&cfg), "svc");
        assert!(built.is_some());
        let (_provider, endpoint) = built.unwrap();
        assert_eq!(endpoint, "http://127.0.0.1:4318/v1/traces");
    }
}
