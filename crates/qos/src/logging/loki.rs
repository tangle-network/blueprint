use std::collections::HashMap;
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
    use opentelemetry::trace::TracerProvider as _; 
    use opentelemetry_sdk::trace::Config;
    use opentelemetry_sdk::Resource;
    use opentelemetry_sdk::export::trace::SimpleSpanExporter;
    let service_name_owned = service_name.to_owned();
    let tracer = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_simple_exporter(SimpleSpanExporter::new())
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

/// Initialize OpenTelemetry tracer
pub fn init_otel_tracer(loki_config: &LokiConfig, service_name: &str) -> Result<opentelemetry_sdk::trace::Tracer> {
    use opentelemetry::KeyValue;
    use opentelemetry::trace::TracerProvider; 
    use opentelemetry_sdk::export::trace::stdout;
    use opentelemetry_sdk::trace as sdktrace;
    use opentelemetry_sdk::Resource;
    use opentelemetry_semantic_conventions::resource as semconv_resource;

    let service_name_owned = service_name.to_string();

    // 1. Create stdout exporter
    let stdout_exporter = stdout::new_pipeline().build_exporter();

    // 2. Create SDK trace config
    let mut sdk_config = sdktrace::Config::default();

    // Apply settings from loki_config.otel_config if present
    if let Some(otel_cfg) = &loki_config.otel_config {
        if let Some(max_attr) = otel_cfg.max_attributes_per_span {
            sdk_config = sdk_config.with_max_attributes_per_span(max_attr);
        }
        // Map other fields from OtelConfig to sdk_config if they are added in the future
    }

    // Create and add resource information
    let resource = Resource::new(vec![
        KeyValue::new(semconv_resource::SERVICE_NAME.as_str(), service_name_owned.clone()),
    ]);
    sdk_config = sdk_config.with_resource(resource);

    // 3. Build TracerProvider
    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_config(sdk_config)
        .with_simple_exporter(stdout_exporter) 
        .build();

    // 4. Get Tracer
    let tracer = provider.tracer(service_name_owned);

    Ok(tracer)
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
