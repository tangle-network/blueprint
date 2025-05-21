use opentelemetry::metrics::MeterProvider;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use prometheus::Registry;
use std::sync::Arc;
use tracing::info;

use crate::error::{Error, Result};
use crate::metrics::types::MetricsConfig;

/// OpenTelemetry exporter configuration
#[derive(Clone, Debug)]
pub struct OpenTelemetryConfig {
    pub service_name: String,
    pub service_version: String,
    pub service_instance_id: String,
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
    /// Meter provider
    meter_provider: Arc<SdkMeterProvider>,
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
        _metrics_config: &MetricsConfig,
    ) -> Result<Self> {
        // Create a Prometheus exporter pipeline
        let _exporter = opentelemetry_prometheus::exporter()
            .with_registry(registry.clone())
            .build()
            .map_err(|e| Error::Other(format!("Failed to create OpenTelemetry exporter: {}", e)))?;

        let meter_provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder().build();

        let meter_provider = Arc::new(meter_provider);

        // Create a meter
        let meter = meter_provider.meter("blueprint_metrics");
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
    pub fn meter_provider(&self) -> Arc<SdkMeterProvider> {
        self.meter_provider.clone()
    }

    /// Create a counter
    pub fn create_counter(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> opentelemetry::metrics::Counter<u64> {
        let name = name.into();
        let description = description.into();

        let builder = self.meter.u64_counter(name).with_description(description);
        builder.build()
    }

    /// Create a counter with attributes
    pub fn create_counter_with_attributes(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> opentelemetry::metrics::Counter<u64> {
        let name = name.into();
        let description = description.into();

        let builder = self.meter.u64_counter(name).with_description(description);
        builder.build()
    }

    /// Create a histogram
    pub fn create_histogram(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> opentelemetry::metrics::Histogram<f64> {
        let name = name.into();
        let description = description.into();

        let builder = self.meter.f64_histogram(name).with_description(description);
        builder.build()
    }

    /// Create a gauge
    pub fn create_gauge(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> opentelemetry::metrics::ObservableGauge<f64> {
        let name = name.into();
        let description = description.into();

        self.meter
            .f64_observable_gauge(name)
            .with_description(description)
            .with_callback(|_| {})
            .build()
    }

    /// Create an up-down counter
    pub fn create_up_down_counter(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> opentelemetry::metrics::UpDownCounter<i64> {
        let name = name.into();
        let description = description.into();

        let builder = self
            .meter
            .i64_up_down_counter(name)
            .with_description(description);
        builder.build()
    }
}
