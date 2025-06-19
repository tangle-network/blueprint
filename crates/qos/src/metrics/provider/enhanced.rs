use blueprint_core::error;
use blueprint_core::{debug, info};
use prometheus::Registry;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

use crate::error::Result;
use crate::metrics::opentelemetry::{OpenTelemetryConfig, OpenTelemetryExporter};
use crate::metrics::prometheus::PrometheusCollector;
use crate::metrics::types::{
    BlueprintMetrics, BlueprintStatus, MetricsConfig, MetricsProvider, SystemMetrics,
};
use crate::servers::ServerManager;
use crate::servers::prometheus::PrometheusServer;
use opentelemetry::KeyValue;

/// A comprehensive metrics provider that integrates Prometheus and OpenTelemetry systems.
///
/// This provider acts as the central metrics collection and export hub for the `QoS` system,
/// collecting system metrics, application-specific metrics, and custom metrics. It manages
/// metric collection, storage, and export to monitoring systems through Prometheus and
/// OpenTelemetry protocols. The provider supports historical metrics collection and
/// can manage an embedded Prometheus server for metrics exposure.
#[derive(Clone)]
pub struct EnhancedMetricsProvider {
    /// System metrics
    system_metrics: Arc<RwLock<Vec<SystemMetrics>>>,

    /// Blueprint metrics
    blueprint_metrics: Arc<RwLock<Vec<BlueprintMetrics>>>,

    /// Blueprint status
    blueprint_status: Arc<RwLock<BlueprintStatus>>,

    /// Custom metrics
    custom_metrics: Arc<RwLock<std::collections::HashMap<String, String>>>,

    /// Prometheus collector
    prometheus_collector: Arc<PrometheusCollector>,

    /// OpenTelemetry exporter
    opentelemetry_exporter: Arc<OpenTelemetryExporter>,

    /// Prometheus server
    prometheus_server: Arc<RwLock<Option<PrometheusServer>>>,

    /// Shared Prometheus registry for all metrics
    shared_registry: Arc<Registry>,

    /// OpenTelemetry counter for job executions
    otel_job_executions_counter: opentelemetry::metrics::Counter<u64>,

    /// Configuration
    config: MetricsConfig,

    /// Start time
    start_time: Instant,
}

impl EnhancedMetricsProvider {
    /// Creates a new enhanced metrics provider with Prometheus and OpenTelemetry support.
    ///
    /// Initializes the metrics collection infrastructure including Prometheus collectors,
    /// OpenTelemetry exporters, and shared registries. Sets up metric collection for both
    /// system-level and application-specific metrics, and prepares the provider for metrics
    /// export through multiple protocols.
    ///
    /// # Parameters
    /// * `metrics_config` - Configuration for metrics collection, retention, and reporting
    /// * `otel_config` - OpenTelemetry-specific configuration settings
    ///
    /// # Errors
    /// Returns an error if the `PrometheusCollector` or `OpenTelemetryExporter` initialization fails.
    pub fn new(metrics_config: MetricsConfig, otel_config: &OpenTelemetryConfig) -> Result<Self> {
        let shared_registry = Arc::new(Registry::new());
        let prometheus_collector = Arc::new(
            PrometheusCollector::new(metrics_config.clone(), shared_registry.clone()).map_err(
                |e| {
                    crate::error::Error::Other(format!(
                        "Failed to create Prometheus collector: {}",
                        e
                    ))
                },
            )?,
        );

        let otel_exporter_instance =
            OpenTelemetryExporter::new(otel_config, shared_registry.clone())?;

        info!("OpenTelemetryExporter initialized with shared Prometheus registry.");

        let opentelemetry_exporter = Arc::new(otel_exporter_instance);
        info!(
            "Created and configured OpenTelemetryExporter in EnhancedMetricsProvider: {:?}",
            opentelemetry_exporter
        );

        let otel_job_executions_counter = opentelemetry_exporter
            .meter()
            .u64_counter("otel_job_executions")
            .with_description("Total number of job executions recorded via OTel")
            .build();
        info!("Created otel_job_executions_counter in EnhancedMetricsProvider");

        let blueprint_status = BlueprintStatus {
            service_id: metrics_config.service_id,
            blueprint_id: metrics_config.blueprint_id,
            ..BlueprintStatus::default()
        };

        let provider = Self {
            system_metrics: Arc::new(RwLock::new(Vec::new())),
            blueprint_metrics: Arc::new(RwLock::new(Vec::new())),
            blueprint_status: Arc::new(RwLock::new(blueprint_status)),
            custom_metrics: Arc::new(RwLock::new(std::collections::HashMap::new())),
            prometheus_collector,
            opentelemetry_exporter,
            prometheus_server: Arc::new(RwLock::new(None)),
            shared_registry,
            otel_job_executions_counter,
            config: metrics_config,
            start_time: Instant::now(),
        };

        Ok(provider)
    }

    /// Starts the metrics collection and reporting process.
    ///
    /// This method initializes the background metrics collection task that periodically gathers
    /// system and blueprint metrics. It also starts the Prometheus server if configured to
    /// expose metrics via HTTP endpoints. This method should be called once after creating
    /// the provider to begin the metrics pipeline.
    ///
    /// # Errors
    /// Returns an error if the `PrometheusServer` fails to start or if the background
    /// metrics collection task cannot be created.
    pub async fn start_collection(self: Arc<Self>) -> Result<()> {
        let prometheus_server_config = self.config.prometheus_server.clone().unwrap_or_default();

        let server = PrometheusServer::new(
            prometheus_server_config,
            Some(self.shared_registry.clone()),
            self.clone(),
        )?;
        server.start(None, None).await?;

        let mut prometheus_server = self.prometheus_server.write().await;
        *prometheus_server = Some(server);

        let system_metrics = self.system_metrics.clone();
        let blueprint_metrics = self.blueprint_metrics.clone();
        let blueprint_status = self.blueprint_status.clone();
        let custom_metrics = self.custom_metrics.clone();
        let prometheus_collector = self.prometheus_collector.clone();
        let start_time = self.start_time;
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(Duration::from_secs(config.collection_interval_secs));

            loop {
                interval.tick().await;

                let sys_metrics = Self::collect_system_metrics();

                prometheus_collector.update_system_metrics(&sys_metrics);

                let mut metrics = system_metrics.write().await;
                metrics.push(sys_metrics);
                if metrics.len() > config.max_history {
                    metrics.remove(0);
                }

                let mut bp_metrics = BlueprintMetrics::default();
                let custom = custom_metrics.read().await;
                bp_metrics.custom_metrics = custom.clone();

                let mut metrics = blueprint_metrics.write().await;
                metrics.push(bp_metrics);
                if metrics.len() > config.max_history {
                    metrics.remove(0);
                }

                let mut status = blueprint_status.write().await;
                status.uptime = start_time.elapsed().as_secs();
                status.timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                prometheus_collector.update_blueprint_status(&status);

                debug!("Collected metrics");
            }
        });

        info!("Started metrics collection");
        Ok(())
    }

    /// Collects current system metrics including CPU, memory, and network usage.
    ///
    /// This method gathers real-time system metrics using system APIs and formats them
    /// into a structured `SystemMetrics` object. It includes CPU utilization, memory usage,
    /// disk activity, and network statistics from the host system.
    fn collect_system_metrics() -> SystemMetrics {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        let memory_usage = sys.used_memory();
        let total_memory = sys.total_memory();
        let cpu_usage = sys.global_cpu_usage();

        // TODO: Implement disk and network metrics collection
        let disk_usage = 0;
        let total_disk = 0;
        let network_rx_bytes = 0;
        let network_tx_bytes = 0;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        SystemMetrics {
            cpu_usage,
            memory_usage,
            total_memory,
            disk_usage,
            total_disk,
            network_rx_bytes,
            network_tx_bytes,
            timestamp,
        }
    }

    /// Records metrics for a successful job execution.
    ///
    /// Updates both Prometheus and OpenTelemetry metrics with information about a completed job.
    /// This includes recording the execution time, incrementing job counters, and updating histograms
    /// with execution duration data. Job metrics are tagged with service ID, blueprint ID, and job ID
    /// to enable detailed filtering and analysis.
    ///
    /// # Parameters
    /// * `job_id` - Unique identifier for the executed job
    /// * `execution_time` - Duration of the job execution in seconds
    /// * `service_id` - Identifier for the service that executed the job
    /// * `blueprint_id` - Identifier for the blueprint that executed the job
    pub fn record_job_execution(
        &self,
        job_id: u64,
        execution_time: f64,
        service_id: u64,
        blueprint_id: u64,
    ) {
        info!(
            "Recording job execution (job_id: {}). Incrementing otel_job_executions_counter.",
            job_id
        );
        self.otel_job_executions_counter.add(
            1,
            &[
                KeyValue::new("service_id", service_id.to_string()),
                KeyValue::new("blueprint_id", blueprint_id.to_string()),
            ],
        );

        self.prometheus_collector.record_job_execution(
            job_id,
            self.config.service_id,
            self.config.blueprint_id,
            execution_time,
        );
    }

    /// Records metrics for a failed job execution.
    ///
    /// Updates error counters and metrics when a job fails, categorizing the error by type.
    /// This method enables tracking of error rates and common failure modes across jobs.
    ///
    /// # Parameters
    /// * `job_id` - Unique identifier for the failed job
    /// * `error_type` - Classification of the error that occurred
    pub fn record_job_error(&self, job_id: u64, error_type: &str) {
        self.prometheus_collector.record_job_error(
            job_id,
            self.config.service_id,
            self.config.blueprint_id,
            error_type,
        );
    }

    /// Returns a reference to the `OpenTelemetryExporter`.
    ///
    /// Provides access to the underlying `OpenTelemetryExporter` for advanced operations
    /// such as creating custom meters, recorders, or manually pushing metrics to the
    /// OpenTelemetry backend.
    #[must_use]
    pub fn opentelemetry_exporter(&self) -> Arc<OpenTelemetryExporter> {
        self.opentelemetry_exporter.clone()
    }

    /// Returns a reference to the `PrometheusCollector`.
    ///
    /// Provides access to the underlying `PrometheusCollector` for advanced operations
    /// such as registering custom collectors or directly manipulating `Prometheus` metrics.
    #[must_use]
    pub fn prometheus_collector(&self) -> Arc<PrometheusCollector> {
        self.prometheus_collector.clone()
    }

    /// Returns a clone of the `OpenTelemetry` job executions counter.
    ///
    /// This counter tracks the total number of job executions recorded through `OpenTelemetry`.
    /// It can be used to increment execution counts from external components.
    #[must_use]
    pub fn get_otel_job_executions_counter(&self) -> opentelemetry::metrics::Counter<u64> {
        self.otel_job_executions_counter.clone()
    }

    /// Returns the shared `Prometheus` registry used for all metrics.
    ///
    /// This registry consolidates all `Prometheus` metrics from both direct `Prometheus` collectors
    /// and `OpenTelemetry` exporters. It's useful for registering additional custom collectors
    /// or exporting all metrics to external systems.
    #[must_use]
    pub fn shared_registry(&self) -> Arc<Registry> {
        self.shared_registry.clone()
    }

    /// Forces flush of accumulated OpenTelemetry metrics to their destination.
    ///
    /// This method triggers an immediate export of all buffered OpenTelemetry metrics
    /// rather than waiting for the normal export interval. This is useful during graceful
    /// shutdown or when immediate metric visibility is required.
    ///
    /// # Errors
    /// Returns an error if the `OpenTelemetryExporter` fails to force flush metrics.
    pub fn force_flush_otel_metrics(&self) -> crate::error::Result<()> {
        info!("EnhancedMetricsProvider: Attempting to force flush OpenTelemetry metrics...");
        match self.opentelemetry_exporter.force_flush() {
            Ok(()) => {
                info!("EnhancedMetricsProvider: OpenTelemetry metrics force_flush successful.");
                Ok(())
            }
            Err(err) => {
                error!(
                    "EnhancedMetricsProvider: OpenTelemetry metrics force_flush failed: {:?}",
                    err
                );
                Err(crate::error::Error::Metrics(format!(
                    "OpenTelemetry SDK flush error: {}",
                    err
                )))
            }
        }
    }
}

impl MetricsProvider for EnhancedMetricsProvider {
    /// Returns the latest collected `SystemMetrics`.
    #[must_use]
    async fn get_system_metrics(&self) -> SystemMetrics {
        self.system_metrics
            .read()
            .await
            .last()
            .cloned()
            .unwrap_or_default()
    }

    /// Returns the latest collected `BlueprintMetrics`.
    #[must_use]
    async fn get_blueprint_metrics(&self) -> BlueprintMetrics {
        self.blueprint_metrics
            .read()
            .await
            .last()
            .cloned()
            .unwrap_or_default()
    }

    /// Returns the current `BlueprintStatus`.
    #[must_use]
    async fn get_blueprint_status(&self) -> BlueprintStatus {
        self.blueprint_status.read().await.clone()
    }

    /// Returns the historical `SystemMetrics` (up to `max_history`).
    #[must_use]
    async fn get_system_metrics_history(&self) -> Vec<SystemMetrics> {
        self.system_metrics.read().await.clone()
    }

    /// Returns the historical `BlueprintMetrics` (up to `max_history`).
    #[must_use]
    async fn get_blueprint_metrics_history(&self) -> Vec<BlueprintMetrics> {
        self.blueprint_metrics.read().await.clone()
    }

    /// Adds a custom metric to the collection.
    ///
    /// # Parameters
    /// * `key` - The key for the custom metric.
    /// * `value` - The value of the custom metric.
    async fn add_custom_metric(&self, key: String, value: String) {
        let pc_key = key.clone();
        let pc_value = value.clone();

        let mut custom = self.custom_metrics.write().await;
        custom.insert(key, value);

        self.prometheus_collector
            .add_custom_metric(pc_key, pc_value)
            .await;
    }

    /// Sets the `BlueprintStatus`.
    ///
    /// # Parameters
    /// * `status_code` - The status code to set.
    /// * `status_message` - An optional status message.
    async fn set_blueprint_status(&self, status_code: u32, status_message: Option<String>) {
        let mut status = self.blueprint_status.write().await;
        status.status_code = status_code;
        status.status_message = status_message;
        self.prometheus_collector.update_blueprint_status(&status);
    }

    /// Updates the last heartbeat timestamp in the `BlueprintStatus`.
    ///
    /// # Parameters
    /// * `timestamp` - The Unix timestamp of the last heartbeat.
    async fn update_last_heartbeat(&self, timestamp: u64) {
        let mut status = self.blueprint_status.write().await;
        status.last_heartbeat = Some(timestamp);
        self.prometheus_collector.update_blueprint_status(&status);
    }

    /// Starts the metrics collection process for this provider.
    ///
    /// This typically involves spawning background tasks for periodic metric gathering
    /// and potentially starting servers for metric exposure (e.g., a Prometheus server).
    ///
    /// # Errors
    /// Returns an error if the collection process or any associated services fail to start.
    async fn start_collection(&self) -> crate::error::Result<()> {
        let prometheus_server_config = self.config.prometheus_server.clone().unwrap_or_default();

        let server = PrometheusServer::new(
            prometheus_server_config,
            Some(self.shared_registry.clone()),
            Arc::new(self.clone()),
        )?;
        server.start(None, None).await?;

        let mut prometheus_server = self.prometheus_server.write().await;
        *prometheus_server = Some(server);

        let system_metrics = self.system_metrics.clone();
        let blueprint_metrics = self.blueprint_metrics.clone();
        let blueprint_status = self.blueprint_status.clone();
        let custom_metrics = self.custom_metrics.clone();
        let prometheus_collector = self.prometheus_collector.clone();
        let start_time = self.start_time;
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(Duration::from_secs(config.collection_interval_secs));

            loop {
                interval.tick().await;

                let sys_metrics = Self::collect_system_metrics();

                prometheus_collector.update_system_metrics(&sys_metrics);

                let mut metrics = system_metrics.write().await;
                metrics.push(sys_metrics);
                if metrics.len() > config.max_history {
                    metrics.remove(0);
                }

                let mut bp_metrics = BlueprintMetrics::default();
                let custom = custom_metrics.read().await;
                bp_metrics.custom_metrics = custom.clone();

                let mut metrics = blueprint_metrics.write().await;
                metrics.push(bp_metrics);
                if metrics.len() > config.max_history {
                    metrics.remove(0);
                }

                let mut status = blueprint_status.write().await;
                status.uptime = start_time.elapsed().as_secs();
                status.timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                prometheus_collector.update_blueprint_status(&status);

                debug!("Collected metrics");
            }
        });

        info!("Started metrics collection");
        Ok(())
    }
}
