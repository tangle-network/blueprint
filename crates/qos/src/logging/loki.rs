use std::collections::HashMap;
use std::time::Duration;
use tracing::{error, info};
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
        }
    }
}

/// Initialize Loki logging
pub fn init_loki_logging(config: LokiConfig) -> Result<()> {
    // Parse the Loki URL
    let url = Url::parse(&config.url)
        .map_err(|e| Error::Other(format!("Failed to parse Loki URL: {}", e)))?;

    // Create a builder for the Loki layer
    let mut builder = tracing_loki::builder();

    // Add labels
    for (key, value) in config.labels {
        builder = match builder.label(key, value) {
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
    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(loki_layer);

    // Set the subscriber as the global default
    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => {
            info!("Initialized Loki logging");
            Ok(())
        }
        Err(e) => {
            error!("Failed to set global subscriber: {}", e);
            Err(Error::Other(format!(
                "Failed to set global subscriber: {}",
                e
            )))
        }
    }
}

/// Initialize Loki logging with OpenTelemetry integration
pub fn init_loki_with_opentelemetry(loki_config: LokiConfig, service_name: &str) -> Result<()> {
    // Parse the Loki URL
    let url = Url::parse(&loki_config.url)
        .map_err(|e| Error::Other(format!("Failed to parse Loki URL: {}", e)))?;

    // Create a builder for the Loki layer
    let mut builder = tracing_loki::builder();

    // Add labels
    for (key, value) in loki_config.labels {
        builder = match builder.label(key, value) {
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

    // Create an OpenTelemetry tracer
    use opentelemetry::trace::TracerProvider;
    let service_name_owned = service_name.to_owned();
    let tracer = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build()
        .tracer(service_name_owned);

    // Create a subscriber with the Loki layer and OpenTelemetry
    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(loki_layer);

    // Set the subscriber as the global default
    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => {
            info!("Initialized Loki logging with OpenTelemetry");
            Ok(())
        }
        Err(e) => {
            error!("Failed to set global subscriber: {}", e);
            Err(Error::Other(format!(
                "Failed to set global subscriber: {}",
                e
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the LokiConfig default implementation returns a valid configuration.
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
