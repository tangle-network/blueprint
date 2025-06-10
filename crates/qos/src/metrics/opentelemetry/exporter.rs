// Std and common crates
use std::fmt::Debug;
use std::sync::{Arc, Weak};


// Serde and Uuid (restored)
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// OpenTelemetry API (for Error)

// OpenTelemetry SDK specific imports
use opentelemetry_sdk::{
    metrics::{
        data::ResourceMetrics,
        // Aggregation, // Unused import
        InstrumentKind,
        MetricError, // For MetricReader trait error types
        Pipeline,
        SdkMeterProvider,
        Temporality,
    },
    metrics::reader::MetricReader, // MetricReader itself is in sdk::metrics::reader
    Resource,
};

// Prometheus related
use opentelemetry_prometheus::PrometheusExporter;
use prometheus::{
    core::Collector as PrometheusCollectorTrait,
    core::Desc as PrometheusDesc,
    proto::MetricFamily as PrometheusMetricFamily,
    Registry,
};

// General OpenTelemetry
use opentelemetry::{metrics::MeterProvider as _, KeyValue};
use opentelemetry_semantic_conventions as semcov;

// Local crate imports
use crate::error::Error as CrateLocalError; // Keep aliased
use blueprint_core::info;

// Adapter to use Arc<PrometheusExporter> as a MetricReader.
// MetricReader trait is implemented by this adapter.
// These are provided by separate impl blocks for ArcPrometheusReader.
#[derive(Clone, Debug)]
struct ArcPrometheusReader(Arc<PrometheusExporter>);

impl MetricReader for ArcPrometheusReader {
    // Methods from MetricReader itself
    fn register_pipeline(&self, pipeline: Weak<Pipeline>) {
        self.0.register_pipeline(pipeline);
    }

    fn temporality(&self, instrument_kind: InstrumentKind) -> Temporality {
        self.0.temporality(instrument_kind)
    }

    fn collect(&self, rm: &mut ResourceMetrics) -> std::result::Result<(), MetricError> {
        self.0.collect(rm).map_err(|e| e.into())
    }

    fn force_flush(&self) -> std::result::Result<(), opentelemetry_sdk::error::OTelSdkError> {
        self.0.force_flush().map_err(|e| e.into())
    }

    fn shutdown(&self) -> std::result::Result<(), opentelemetry_sdk::error::OTelSdkError> {
        self.0.shutdown().map_err(|e| e.into())
    }
}

/// Name for the OpenTelemetry meter used within the Blueprint system.
const OTEL_METER_NAME: &str = "blueprint_metrics";

/// Configuration for the OpenTelemetry exporter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] 
pub struct OpenTelemetryConfig {
    /// Service name to be used in OpenTelemetry resource attributes.
    pub service_name: String,
    /// Service version to be used in OpenTelemetry resource attributes.
    pub service_version: String,
    /// Service instance ID to be used in OpenTelemetry resource attributes.
    pub service_instance_id: String,
    /// Service namespace to be used in OpenTelemetry resource attributes.
    pub service_namespace: String,
}

impl Default for OpenTelemetryConfig {
    fn default() -> Self {
        Self {
            service_name: "blueprint_service".to_string(),
            service_version: "0.1.0".to_string(),
            service_instance_id: Uuid::new_v4().to_string(),
            service_namespace: "blueprint_namespace".to_string(),
        }
    }
}

/// `OpenTelemetryExporter` sets up and manages the OpenTelemetry metrics pipeline,
/// including a Prometheus exporter.
#[derive(Debug, Clone)]
pub struct OpenTelemetryExporter {
    meter_provider: Arc<SdkMeterProvider>,
    pub meter: opentelemetry::metrics::Meter,
    pub prometheus_registry: Registry, // Registry used by the PrometheusExporter
}

impl OpenTelemetryExporter {
    /// Creates a new `OpenTelemetryExporter`.
    ///
    /// This function initializes an OpenTelemetry `SdkMeterProvider` configured with an
    /// adapter for a `PrometheusExporter`. The `PrometheusExporter` uses its own internal
    /// default registry. The created `SdkMeterProvider` is also set as the global
    /// meter provider for the application.
    ///
    /// # Arguments
    /// * `otel_config`: Configuration for OpenTelemetry resource attributes.
    ///
    /// # Errors
    /// Returns an `Error` if the setup of the OpenTelemetry pipeline or Prometheus exporter fails.
    pub fn new(otel_config: OpenTelemetryConfig) -> std::result::Result<Self, CrateLocalError> { 
        info!("Creating OpenTelemetryExporter with config: {:?}", otel_config);

        // Define the OTel Resource attributes using the provided configuration.
        let resource_attributes = vec![
            KeyValue::new(semcov::resource::SERVICE_NAME, otel_config.service_name.clone()),
            KeyValue::new(semcov::resource::SERVICE_VERSION, otel_config.service_version.clone()),
            KeyValue::new(
                semcov::resource::SERVICE_INSTANCE_ID,
                otel_config.service_instance_id.clone(),
            ),
            KeyValue::new(
                semcov::resource::SERVICE_NAMESPACE,
                otel_config.service_namespace.clone(),
            ),
        ];
        let resource = Resource::builder()
            .with_attributes(resource_attributes)
            .build();

        // Create a Prometheus Registry
        let prometheus_registry = Registry::new();

        // Create the PrometheusExporter instance, configured with our registry.
        let actual_prom_exporter = opentelemetry_prometheus::exporter()
            .with_registry(prometheus_registry.clone())
            .build()
            .map_err(|e| CrateLocalError::Other(format!("Failed to build OTel PrometheusExporter: {}", e)))?;
        
        let shared_prom_exporter_arc = Arc::new(actual_prom_exporter);
        info!("OTel PrometheusExporter instance created and wrapped in Arc. Attempting to use it directly as a reader.");
        
        let meter_provider = SdkMeterProvider::builder()
            .with_reader(ArcPrometheusReader(shared_prom_exporter_arc.clone())) // Pass the adapter
            .with_resource(resource)
            .build();

        opentelemetry::global::set_meter_provider(meter_provider.clone());

        let meter = meter_provider.meter(OTEL_METER_NAME); // Use const &'static str for meter name

        info!(
            meter_name = %OTEL_METER_NAME,
            "OpenTelemetryExporter created, global MeterProvider set."
        );

        Ok(OpenTelemetryExporter {
            meter,
            meter_provider: Arc::new(meter_provider),
            prometheus_registry,
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

impl PrometheusCollectorTrait for OpenTelemetryExporter {
    fn desc(&self) -> Vec<&PrometheusDesc> {
        // Descriptors can be complex for a registry; returning an empty vec is often acceptable.
        // Alternatively, self.prometheus_registry.root_descriptors() could be used if lifetimes match.
        vec![]
    }

    fn collect(&self) -> Vec<PrometheusMetricFamily> {
        self.prometheus_registry.gather()
    }
}
