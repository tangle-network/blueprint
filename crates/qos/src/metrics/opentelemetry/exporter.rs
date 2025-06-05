use blueprint_core::info;
use opentelemetry::global::set_meter_provider;
use opentelemetry::metrics::MeterProvider as _; // Import the trait
// Note: GlobalMeterProviderGuard does not seem to exist for the current opentelemetry version.
use opentelemetry::KeyValue;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_semantic_conventions::resource::{
    SERVICE_INSTANCE_ID, SERVICE_NAME, SERVICE_NAMESPACE, SERVICE_VERSION,
};
use prometheus::Registry;
use std::sync::Arc;

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
#[derive(Debug)]
pub struct OpenTelemetryExporter {
    /// The OpenTelemetry SdkMeterProvider wrapped in an Arc for sharing.
    /// This provider is configured with the Prometheus exporter and a resource.
    meter_provider: Arc<SdkMeterProvider>,
    /// Meter
    meter: opentelemetry::metrics::Meter,
    /// Configuration
    #[allow(dead_code)]
    config: OpenTelemetryConfig,
}

impl OpenTelemetryExporter {
    /// Create a new OpenTelemetry exporter
    ///
    /// # Errors
    /// Returns an error if the OpenTelemetry meter provider or tracer provider initialization fails
    pub fn new(
        registry: &Registry,
        otel_config: OpenTelemetryConfig,
        _metrics_config: &MetricsConfig,
    ) -> Result<Self> {
        // 1. Create the PrometheusExporter (MetricReader) first.
        let prometheus_exporter = opentelemetry_prometheus::exporter()
            .with_registry(registry.clone())
            .build() // Build the exporter itself
            .map_err(|e| Error::Other(format!("Failed to create OpenTelemetry exporter: {}", e)))?;

        // 2. Create a Resource for the SdkMeterProvider using the builder pattern.
        let resource_attributes = vec![
            KeyValue::new(SERVICE_NAME, otel_config.service_name.clone()),
            KeyValue::new(SERVICE_VERSION, otel_config.service_version.clone()),
            KeyValue::new(SERVICE_INSTANCE_ID, otel_config.service_instance_id.clone()),
            KeyValue::new(SERVICE_NAMESPACE, otel_config.service_namespace.clone()),
        ];
        let resource = opentelemetry_sdk::resource::Resource::builder()
            .with_attributes(resource_attributes)
            .build();

        // 3. Create the SdkMeterProvider, registering the exporter and resource with it.
        let meter_provider = SdkMeterProvider::builder()
            .with_reader(prometheus_exporter) // Pass the exporter directly
            .with_resource(resource) // Add the resource
            .build();
        info!("Built SdkMeterProvider in OpenTelemetryExporter");

        // Set this meter provider as the global meter provider.
        // The SdkMeterProvider itself (which is cloned and set) will keep the provider alive.
        // Storing a guard seems unnecessary with current opentelemetry API (v0.22.x - v0.30.x).
        set_meter_provider(meter_provider.clone());
        info!("Set global meter provider.");

        // 4. Wrap SdkMeterProvider in Arc for storage and sharing.
        let meter_provider_arc = Arc::new(meter_provider);

        // 5. Create a meter from this provider.
        let meter = meter_provider_arc.meter("blueprint_metrics");
        info!("Created meter from SdkMeterProvider in OpenTelemetryExporter");
        info!("Created OpenTelemetry exporter");

        Ok(Self {
            meter_provider: meter_provider_arc,
            meter,
            config: otel_config,
        })
    }

    /// Get the meter
    #[must_use]
    pub fn meter(&self) -> &opentelemetry::metrics::Meter {
        &self.meter
    }

    /// Get the meter provider
    #[must_use]
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
