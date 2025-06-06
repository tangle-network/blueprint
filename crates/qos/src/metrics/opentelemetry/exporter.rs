use std::sync::Arc;
use std::fmt::Debug;

use blueprint_core::info;
use opentelemetry::KeyValue;
use opentelemetry::metrics::{Meter, MeterProvider as _}; 
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions as semcov;
use serde::{Deserialize, Serialize}; 
use uuid::Uuid;

use crate::error::{Error, Result}; 

use opentelemetry::metrics::{MetricsError, Result as OTelMetricsResult};
use opentelemetry_prometheus::PrometheusExporter;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::{Aggregation, AggregationSelector, InstrumentKind, MetricReader, Pipeline, Temporality, TemporalitySelector};
use prometheus::{core::Collector as PrometheusCollectorTrait, core::Desc as PrometheusDesc, proto::MetricFamily as PrometheusMetricFamily};

// Adapter to use Arc<PrometheusExporter> as a MetricReader
#[derive(Debug, Clone)]
struct ArcPrometheusReader(Arc<PrometheusExporter>);

impl TemporalitySelector for ArcPrometheusReader {
    fn temporality(&self, kind: InstrumentKind) -> Temporality {
        self.0.temporality(kind)
    }
}

impl AggregationSelector for ArcPrometheusReader {
    fn aggregation(&self, kind: InstrumentKind) -> Aggregation {
        self.0.aggregation(kind)
    }
}

impl MetricReader for ArcPrometheusReader {
    fn register_pipeline(&self, pipeline: std::sync::Weak<Pipeline>) {
        self.0.register_pipeline(pipeline)
    }

    fn collect(&self, rm: &mut ResourceMetrics) -> OTelMetricsResult<()> {
        self.0.collect(rm)
    }

    fn force_flush(&self) -> OTelMetricsResult<()> {
        self.0.force_flush(&opentelemetry::Context::current())
    }

    fn shutdown(&self) -> OTelMetricsResult<()> {
        self.0.shutdown()
    }
}

// Adapter to use Arc<PrometheusExporter> as a prometheus::Collector
#[derive(Debug, Clone)]
pub struct ArcPrometheusCollector(Arc<PrometheusExporter>);

impl PrometheusCollectorTrait for ArcPrometheusCollector {
    fn desc(&self) -> Vec<&PrometheusDesc> {
        self.0.desc()
    }

    fn collect(&self) -> Vec<PrometheusMetricFamily> {
        self.0.collect()
    }
}


/// Name for the OpenTelemetry meter used within the Blueprint system.
const OTEL_METER_NAME: &str = "blueprint_metrics";

// --- OpenTelemetryExporter ---

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
#[derive(Debug)]
pub struct OpenTelemetryExporter {
    meter_provider: Arc<SdkMeterProvider>,
    pub meter: opentelemetry::metrics::Meter,
    config: OpenTelemetryConfig,
    shared_prometheus_exporter: Arc<PrometheusExporter>, // Store the Arc'd exporter
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
    pub fn new(otel_config: OpenTelemetryConfig) -> Result<Self> { 
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

        // Create the PrometheusExporter instance. It will use its own default internal registry.
        let actual_prom_exporter = opentelemetry_prometheus::exporter()
            .build()
            .map_err(|e| Error::Other(format!("Failed to build OTel PrometheusExporter: {}", e)))?;
        
        let shared_prom_exporter_arc = Arc::new(actual_prom_exporter);
        let sdk_reader_adapter = ArcPrometheusReader(shared_prom_exporter_arc.clone());

        info!("OTel PrometheusExporter instance created and wrapped in Arc. Adapter created for SdkMeterProvider.");
        
        let meter_provider = SdkMeterProvider::builder()
            .with_reader(sdk_reader_adapter) // Pass the adapter to the SDK
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
            config: otel_config,
            shared_prometheus_exporter: shared_prom_exporter_arc,
        })
    }

    /// Returns a clone of the Arc-wrapped PrometheusExporter, adapted as a prometheus::Collector.
    pub fn get_collector_adapter(&self) -> ArcPrometheusCollector {
        ArcPrometheusCollector(self.shared_prometheus_exporter.clone())
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
