use opentelemetry::global::meter_provider;
use opentelemetry::metrics::{MeterProvider, Unit};
use opentelemetry_prometheus::PrometheusExporter;
use opentelemetry_sdk::metrics::reader::MetricReader;
use opentelemetry_sdk::metrics::{MeterProviderBuilder, PeriodicReader};
use prometheus::Registry;
use std::sync::Arc;
use tracing::{error, info};

use crate::error::{Error, Result};
use crate::metrics::types::MetricsConfig;

/// OpenTelemetry exporter configuration
#[derive(Clone, Debug)]
pub struct OpenTelemetryConfig {
    /// Service name
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// Service instance ID
    pub service_instance_id: String,
    /// Service namespace
    pub service_namespace: String,
}

impl Default for OpenTelemetryConfig {
    fn default() -> Self {
        Self {
            service_name: "blueprint".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            service_instance_id: uuid::Uuid::new_v4().to_string(),
            service_namespace: "tangle".to_string(),
        }
    }
}

/// OpenTelemetry exporter
pub struct OpenTelemetryExporter {
    /// Prometheus exporter
    // exporter: PrometheusExporter,
    /// Meter provider
    meter_provider: Arc<opentelemetry_sdk::metrics::MeterProvider>,
    /// Meter
    meter: opentelemetry::metrics::Meter,
    /// Configuration
    config: OpenTelemetryConfig,
}

impl OpenTelemetryExporter {
    /// Create a new OpenTelemetry exporter
    pub fn new(
        registry: Registry,
        otel_config: OpenTelemetryConfig,
        metrics_config: &MetricsConfig,
    ) -> Result<Self> {
        // Create a Prometheus exporter pipeline
        let exporter = opentelemetry_prometheus::exporter()
            .with_registry(registry.clone())
            .build()
            .map_err(|e| Error::Other(format!("Failed to create OpenTelemetry exporter: {}", e)))?;

        let meter_provider = MeterProviderBuilder::default()
            .with_reader(exporter)
            .build();

        let meter_provider = Arc::new(meter_provider);

        // Create a meter
        let meter = meter_provider.meter(format!(
            "{}_{}",
            otel_config.service_name, otel_config.service_version
        ));

        info!("Created OpenTelemetry exporter");

        Ok(Self {
            // exporter,
            meter_provider,
            meter,
            config: otel_config,
        })
    }

    /// Get the meter
    pub fn meter(&self) -> &opentelemetry::metrics::Meter {
        &self.meter
    }

    /// Get the meter provider
    pub fn meter_provider(&self) -> Arc<opentelemetry_sdk::metrics::MeterProvider> {
        self.meter_provider.clone()
    }

    // /// Get the Prometheus exporter
    // pub fn prometheus_exporter(&self) -> &PrometheusExporter {
    //     &self.exporter
    // }

    /// Create a counter
    pub fn create_counter(
        &self,
        name: String,
        description: String,
    ) -> opentelemetry::metrics::Counter<u64> {
        self.meter
            .u64_counter(name)
            .with_description(description)
            .init()
    }

    /// Create a counter with attributes
    pub fn create_counter_with_attributes(
        &self,
        name: String,
        description: String,
    ) -> opentelemetry::metrics::Counter<u64> {
        self.meter
            .u64_counter(name)
            .with_description(description)
            .init()
    }

    /// Create a histogram
    pub fn create_histogram(
        &self,
        name: String,
        description: String,
    ) -> opentelemetry::metrics::Histogram<f64> {
        self.meter
            .f64_histogram(name)
            .with_description(description)
            .init()
    }

    /// Create a gauge
    pub fn create_gauge(
        &self,
        name: String,
        description: String,
    ) -> opentelemetry::metrics::ObservableGauge<f64> {
        self.meter
            .f64_observable_gauge(name)
            .with_description(description)
            .init()
    }

    /// Create an up-down counter
    pub fn create_up_down_counter(
        &self,
        name: String,
        description: String,
    ) -> opentelemetry::metrics::UpDownCounter<i64> {
        self.meter
            .i64_up_down_counter(name)
            .with_description(description)
            .init()
    }
}
