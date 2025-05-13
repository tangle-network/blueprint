use std::collections::HashMap;
use std::time::Duration;
use tracing::{error, info};
use tracing_loki::{LokiLayer, url::Url};
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
    let mut builder = tracing_loki::builder()
        .url(url)
        .batch_size(config.batch_size)
        .timeout(Duration::from_secs(config.timeout_secs));

    // Add basic auth if provided
    if let (Some(username), Some(password)) = (config.username, config.password) {
        builder = builder.basic_auth(username, password);
    }

    // Add labels
    for (key, value) in config.labels {
        builder = builder.label(key, value);
    }

    // Build the Loki layer
    let loki_layer = match builder.build() {
        Ok(layer) => layer,
        Err(e) => {
            error!("Failed to build Loki layer: {}", e);
            return Err(Error::Other(format!("Failed to build Loki layer: {}", e)));
        }
    };

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
    let mut builder = tracing_loki::builder()
        .url(url)
        .batch_size(loki_config.batch_size)
        .timeout(Duration::from_secs(loki_config.timeout_secs));

    // Add basic auth if provided
    if let (Some(username), Some(password)) = (loki_config.username, loki_config.password) {
        builder = builder.basic_auth(username, password);
    }

    // Add labels
    for (key, value) in loki_config.labels {
        builder = builder.label(key, value);
    }

    // Build the Loki layer
    let loki_layer = match builder.build() {
        Ok(layer) => layer,
        Err(e) => {
            error!("Failed to build Loki layer: {}", e);
            return Err(Error::Other(format!("Failed to build Loki layer: {}", e)));
        }
    };

    // Create an OpenTelemetry tracer
    let tracer = opentelemetry::sdk::trace::TracerProvider::builder()
        .with_simple_exporter(opentelemetry_sdk::export::trace::stdout::Exporter::default())
        .build()
        .tracer(service_name);

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
