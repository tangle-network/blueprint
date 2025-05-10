use prometheus::{
    Counter, CounterVec, Gauge, GaugeVec, Histogram, HistogramVec, IntCounter, IntCounterVec,
    IntGauge, IntGaugeVec, Registry,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

use crate::metrics::types::{BlueprintMetrics, BlueprintStatus, MetricsConfig, SystemMetrics};

/// Prometheus metrics collector
pub struct PrometheusCollector {
    registry: Registry,

    // System metrics
    cpu_usage: Gauge,
    memory_usage: IntGauge,
    total_memory: IntGauge,
    disk_usage: IntGauge,
    total_disk: IntGauge,
    network_rx_bytes: IntCounter,
    network_tx_bytes: IntCounter,

    // Blueprint metrics
    job_executions: IntCounterVec,
    job_execution_time: HistogramVec,
    job_errors: IntCounterVec,

    // Status metrics
    uptime: IntGauge,
    last_heartbeat: IntGauge,
    status_code: IntGauge,

    // Configuration
    config: MetricsConfig,

    // Custom metrics storage
    custom_metrics: Arc<RwLock<std::collections::HashMap<String, String>>>,
}

impl PrometheusCollector {
    /// Create a new Prometheus metrics collector
    pub fn new(config: MetricsConfig) -> Result<Self, prometheus::Error> {
        let registry = Registry::new();

        // Register process metrics
        prometheus::process_collector::ProcessCollector::for_self()
            .map_err(|e| {
                error!("Failed to create process collector: {}", e);
                e
            })?
            .collect();

        // System metrics
        let cpu_usage = Gauge::new("blueprint_cpu_usage", "CPU usage percentage")?;
        let memory_usage = IntGauge::new("blueprint_memory_usage", "Memory usage in bytes")?;
        let total_memory =
            IntGauge::new("blueprint_total_memory", "Total memory available in bytes")?;
        let disk_usage = IntGauge::new("blueprint_disk_usage", "Disk usage in bytes")?;
        let total_disk = IntGauge::new("blueprint_total_disk", "Total disk space in bytes")?;
        let network_rx_bytes =
            IntCounter::new("blueprint_network_rx_bytes", "Network received bytes")?;
        let network_tx_bytes =
            IntCounter::new("blueprint_network_tx_bytes", "Network transmitted bytes")?;

        // Blueprint metrics
        let job_executions = IntCounterVec::new(
            prometheus::opts!("blueprint_job_executions", "Number of job executions"),
            &["job_id", "service_id", "blueprint_id"],
        )?;
        let job_execution_time = HistogramVec::new(
            prometheus::histogram_opts!(
                "blueprint_job_execution_time",
                "Job execution time in seconds"
            ),
            &["job_id", "service_id", "blueprint_id"],
        )?;
        let job_errors = IntCounterVec::new(
            prometheus::opts!("blueprint_job_errors", "Number of job errors"),
            &["job_id", "service_id", "blueprint_id", "error_type"],
        )?;

        // Status metrics
        let uptime = IntGauge::new("blueprint_uptime", "Uptime in seconds")?;
        let last_heartbeat = IntGauge::new(
            "blueprint_last_heartbeat",
            "Last heartbeat time as Unix timestamp",
        )?;
        let status_code = IntGauge::new("blueprint_status_code", "Status code")?;

        // Register metrics with registry
        registry.register(Box::new(cpu_usage.clone()))?;
        registry.register(Box::new(memory_usage.clone()))?;
        registry.register(Box::new(total_memory.clone()))?;
        registry.register(Box::new(disk_usage.clone()))?;
        registry.register(Box::new(total_disk.clone()))?;
        registry.register(Box::new(network_rx_bytes.clone()))?;
        registry.register(Box::new(network_tx_bytes.clone()))?;
        registry.register(Box::new(job_executions.clone()))?;
        registry.register(Box::new(job_execution_time.clone()))?;
        registry.register(Box::new(job_errors.clone()))?;
        registry.register(Box::new(uptime.clone()))?;
        registry.register(Box::new(last_heartbeat.clone()))?;
        registry.register(Box::new(status_code.clone()))?;

        Ok(Self {
            registry,
            cpu_usage,
            memory_usage,
            total_memory,
            disk_usage,
            total_disk,
            network_rx_bytes,
            network_tx_bytes,
            job_executions,
            job_execution_time,
            job_errors,
            uptime,
            last_heartbeat,
            status_code,
            config,
            custom_metrics: Arc::new(RwLock::new(std::collections::HashMap::new())),
        })
    }

    /// Get the Prometheus registry
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Update system metrics
    pub fn update_system_metrics(&self, metrics: &SystemMetrics) {
        self.cpu_usage.set(metrics.cpu_usage as f64);
        self.memory_usage.set(metrics.memory_usage as i64);
        self.total_memory.set(metrics.total_memory as i64);
        self.disk_usage.set(metrics.disk_usage as i64);
        self.total_disk.set(metrics.total_disk as i64);

        // For counters, we need to increment by the difference
        // This is a simplification - in a real system, you'd track the previous values
        self.network_rx_bytes.inc_by(metrics.network_rx_bytes);
        self.network_tx_bytes.inc_by(metrics.network_tx_bytes);

        debug!("Updated system metrics in Prometheus");
    }

    /// Update blueprint status
    pub fn update_blueprint_status(&self, status: &BlueprintStatus) {
        self.uptime.set(status.uptime as i64);
        if let Some(last_heartbeat) = status.last_heartbeat {
            self.last_heartbeat.set(last_heartbeat as i64);
        }
        self.status_code.set(status.status_code as i64);

        debug!("Updated blueprint status in Prometheus");
    }

    /// Record job execution
    pub fn record_job_execution(
        &self,
        job_id: u64,
        service_id: u64,
        blueprint_id: u64,
        execution_time: f64,
    ) {
        let labels = [
            job_id.to_string(),
            service_id.to_string(),
            blueprint_id.to_string(),
        ];

        self.job_executions.with_label_values(&labels).inc();
        self.job_execution_time
            .with_label_values(&labels)
            .observe(execution_time);

        debug!(
            job_id = job_id,
            service_id = service_id,
            blueprint_id = blueprint_id,
            execution_time = execution_time,
            "Recorded job execution in Prometheus"
        );
    }

    /// Record job error
    pub fn record_job_error(
        &self,
        job_id: u64,
        service_id: u64,
        blueprint_id: u64,
        error_type: &str,
    ) {
        let labels = [
            job_id.to_string(),
            service_id.to_string(),
            blueprint_id.to_string(),
            error_type.to_string(),
        ];

        self.job_errors.with_label_values(&labels).inc();

        debug!(
            job_id = job_id,
            service_id = service_id,
            blueprint_id = blueprint_id,
            error_type = error_type,
            "Recorded job error in Prometheus"
        );
    }

    /// Add custom metric
    pub async fn add_custom_metric(&self, key: String, value: String) {
        if let Ok(mut custom_metrics) = self.custom_metrics.write().await {
            custom_metrics.insert(key.clone(), value.clone());
            debug!(key = key, value = value, "Added custom metric");
        } else {
            error!("Failed to acquire custom_metrics write lock");
        }
    }

    /// Get custom metrics
    pub async fn get_custom_metrics(&self) -> std::collections::HashMap<String, String> {
        if let Ok(custom_metrics) = self.custom_metrics.read().await {
            custom_metrics.clone()
        } else {
            error!("Failed to acquire custom_metrics read lock");
            std::collections::HashMap::new()
        }
    }
}
