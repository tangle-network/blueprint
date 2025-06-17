use blueprint_core::error;
use std::collections::HashMap;
use tracing_loki::url::Url;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{Registry, layer::SubscriberExt};

// Default values for LokiConfig
const DEFAULT_LOKI_LABEL_SERVICE_KEY: &str = "service";
const DEFAULT_LOKI_LABEL_SERVICE_VALUE: &str = "blueprint";
const DEFAULT_LOKI_LABEL_ENVIRONMENT_KEY: &str = "environment";
const DEFAULT_LOKI_LABEL_ENVIRONMENT_VALUE: &str = "development";
const DEFAULT_LOKI_URL: &str = "http://localhost:3100";
const DEFAULT_LOKI_BATCH_SIZE: usize = 100;
const DEFAULT_LOKI_TIMEOUT_SECS: u64 = 5;

use crate::error::{Error, Result};

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

    /// OpenTelemetry configuration
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

/// OpenTelemetry configuration
#[derive(Clone, Debug)]
pub struct OtelConfig {
    /// Maximum attributes per span
    pub max_attributes_per_span: Option<u32>,
}

/// Initializes Loki logging with the provided configuration.
///
/// This function sets up the tracing infrastructure to send logs to a Loki server,
/// creating a background task that collects logs and sends them in batches. It configures
/// labels, authentication, and connection details according to the provided configuration.
/// The function integrates with Rust's tracing ecosystem for seamless log forwarding.
///
/// # Parameters
/// * `config` - Configuration settings for the Loki connection and log processing
///
/// # Errors
/// Returns an error if the Loki layer initialization fails, including URL parsing errors,
/// label configuration errors, or connection setup failures
pub fn init_loki_logging(config: LokiConfig) -> Result<()> {
    // Parse the Loki URL
    let url = Url::parse(&config.url)
        .map_err(|e| Error::Other(format!("Failed to parse Loki URL: {}", e)))?;

    // Create a builder for the Loki layer
    let mut builder = tracing_loki::builder();

    // Add labels
    for (key, value) in config.labels {
        builder = match builder.label(key.clone(), value.clone()) {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to add label to Loki layer: {}", e);
                return Err(Error::Other(format!(
                    "Failed to add label to Loki layer: {}",
                    e
                )));
            }
        };
    }

    // Build the Loki layer with URL
    let (loki_layer, task) = match builder.build_url(url) {
        Ok((layer, task)) => (layer, task),
        Err(e) => {
            error!("Failed to build Loki layer: {}", e);
            return Err(Error::Other(format!("Failed to build Loki layer: {}", e)));
        }
    };

    // Spawn the background task
    tokio::spawn(task);

    // Create a subscriber with the Loki layer
    let _subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(loki_layer);

    // TODO: Fix loki logging
    // // Set the subscriber as the global default
    // match blueprint_core::subscriber::set_global_default(subscriber) {
    //     Ok(()) => {
    //         info!("Initialized Loki logging");
    //         Ok(())
    //     }
    //     Err(e) => {
    //         error!("Failed to set global subscriber: {}", e);
    //         Err(Error::Other(format!(
    //             "Failed to set global subscriber: {}",
    //             e
    //         )))
    //     }
    // }
    Ok(())
}

/// Initializes Loki logging with OpenTelemetry integration for distributed tracing.
///
/// This function sets up both Loki logging and OpenTelemetry tracing, creating an
/// integrated observability pipeline. OpenTelemetry traces can be correlated with
/// logs, providing a unified view of request flows across distributed systems. The
/// integration enriches logs with trace context and enables more powerful debugging
/// and monitoring capabilities.
///
/// # Parameters
/// * `loki_config` - Configuration for the Loki connection and log processing
/// * `service_name` - Name of the service, used for identifying the source in traces and logs
///
/// # Errors
/// Returns an error if the Loki layer or OpenTelemetry initialization fails, including
/// configuration errors, connection issues, or pipeline setup problems
pub fn init_loki_with_opentelemetry(loki_config: &LokiConfig, service_name: &str) -> Result<()> {
    // Parse the Loki URL
    let url = Url::parse(&loki_config.url)
        .map_err(|e| Error::Other(format!("Failed to parse Loki URL: {}", e)))?;

    // Create a builder for the Loki layer
    let mut builder = tracing_loki::builder();

    // Add labels
    for (key, value) in &loki_config.labels {
        builder = match builder.label(key.clone(), value.clone()) {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to add label to Loki layer: {}", e);
                return Err(Error::Other(format!(
                    "Failed to add label to Loki layer: {}",
                    e
                )));
            }
        };
    }

    // Build the Loki layer with URL
    let (loki_layer, task) = match builder.build_url(url) {
        Ok((layer, task)) => (layer, task),
        Err(e) => {
            error!("Failed to build Loki layer: {}", e);
            return Err(Error::Other(format!("Failed to build Loki layer: {}", e)));
        }
    };

    // Spawn the background task
    tokio::spawn(task);

    init_otel_tracer(loki_config, service_name)?;

    let opentelemetry_layer = tracing_opentelemetry::layer();
    let _subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(loki_layer)
        .with(opentelemetry_layer);

    // TODO: Fix loki logging
    // // Set the subscriber as the global default
    // match tracing::subscriber::set_global_default(subscriber) {
    //     Ok(()) => {
    //         info!("Initialized Loki logging with OpenTelemetry");
    //         Ok(())
    //     }
    //     Err(e) => {
    //         error!("Failed to set global subscriber: {}", e);
    //         Err(Error::Other(format!(
    //             "Failed to set global subscriber: {}",
    //             e
    //         )))
    //     }
    // }
    Ok(())
}

/// Initializes an OpenTelemetry tracer with Loki integration for trace export.
///
/// This function configures and starts an OpenTelemetry tracer that sends trace
/// data to Loki. It sets up the trace context propagation, sampling strategies,
/// and export pipelines according to the provided configuration. The tracer captures
/// distributed tracing information that can be viewed alongside logs in Grafana.
///
/// # Parameters
/// * `loki_config` - Configuration for the Loki connection including OpenTelemetry settings
/// * `service_name` - Name of the service to identify the source of traces
///
/// # Errors
/// Returns an error if the OpenTelemetry tracer initialization fails, such as
/// invalid configuration, resource allocation issues, or export pipeline setup failures
pub fn init_otel_tracer(loki_config: &LokiConfig, service_name: &str) -> Result<()> {
    use opentelemetry::KeyValue;

    let service_name_owned = service_name.to_string();

    let resource = opentelemetry_sdk::Resource::builder()
        .with_attributes(vec![
            KeyValue::new("service.name", service_name_owned.clone()),
            KeyValue::new("service.version", env!("CARGO_PKG_VERSION").to_string()),
        ])
        .build();

    // Apply settings from config if present
    if let Some(_otel_cfg) = &loki_config.otel_config {}

    let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_resource(resource)
        .build();

    // Set as global provider
    opentelemetry::global::set_tracer_provider(provider);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the `LokiConfig` default implementation returns a valid configuration.
    ///
    /// ```
    /// LokiConfig::default() -> Valid config
    /// ```
    ///
    /// Expected outcome: Default config has reasonable values
    #[test]
    fn test_loki_config_default() {
        let config = LokiConfig::default();
        assert_eq!(config.url, "http://localhost:3100");
        assert_eq!(config.batch_size, 100);
    }
}
