use blueprint_core::error;
use std::collections::HashMap;
use tracing_loki::url::Url;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{Registry, layer::SubscriberExt};

use crate::error::{Error, Result};

/// Configuration for Loki logging
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
        labels.insert("service".to_string(), "blueprint".to_string());
        labels.insert("environment".to_string(), "development".to_string());

        Self {
            url: "http://localhost:3100".to_string(),
            username: None,
            password: None,
            labels,
            batch_size: 100,
            timeout_secs: 5,
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

/// Initialize Loki logging with the given configuration
///
/// # Errors
/// Returns an error if the Loki layer initialization fails
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

/// Initialize Loki logging with OpenTelemetry integration
///
/// # Errors
/// Returns an error if the Loki layer or OpenTelemetry initialization fails
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

/// Initialize OpenTelemetry tracer with Loki integration
///
/// # Errors
/// Returns an error if the OpenTelemetry tracer initialization fails
pub fn init_otel_tracer(loki_config: &LokiConfig, service_name: &str) -> Result<()> {
    use opentelemetry::KeyValue;

    let service_name_owned = service_name.to_string();

    let resource = opentelemetry_sdk::Resource::builder()
        .with_attributes(vec![
            KeyValue::new("service.name", service_name_owned.clone()),
            KeyValue::new("service.version", env!("CARGO_PKG_VERSION").to_string()),
        ])
        .build();

    // Apply settings from loki_config.otel_config if present
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
