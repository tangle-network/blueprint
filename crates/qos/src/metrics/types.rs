use crate::error::Error;
use crate::servers::prometheus::PrometheusServerConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;

// Default values for MetricsConfig
const DEFAULT_METRICS_COLLECTION_INTERVAL_SECS: u64 = 60;
const DEFAULT_METRICS_MAX_HISTORY: usize = 100;
const DEFAULT_METRICS_SERVICE_ID: u64 = 0;
const DEFAULT_METRICS_BLUEPRINT_ID: u64 = 0;

/// Configuration for the metrics collection, storage, and exposure service.
///
/// This structure defines settings for how metrics should be collected,
/// how long they should be retained, sampling rates, and how they
/// should be exposed to external systems (e.g., through a Prometheus server).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MetricsConfig {
    pub prometheus_server: Option<PrometheusServerConfig>,
    pub collection_interval_secs: u64,
    pub max_history: usize,
    pub service_id: u64,
    pub blueprint_id: u64,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            prometheus_server: Some(PrometheusServerConfig::default()),
            collection_interval_secs: DEFAULT_METRICS_COLLECTION_INTERVAL_SECS,
            max_history: DEFAULT_METRICS_MAX_HISTORY,
            service_id: DEFAULT_METRICS_SERVICE_ID,
            blueprint_id: DEFAULT_METRICS_BLUEPRINT_ID,
        }
    }
}

/// System-level metrics representing hardware and OS resource utilization.
///
/// These metrics include information about the host system's resource usage,
/// such as CPU utilization, memory consumption, disk I/O, and network traffic.
/// They provide a snapshot of the system's state at a point in time.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub total_memory: u64,
    pub disk_usage: u64,
    pub total_disk: u64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub timestamp: u64,
}

/// Blueprint-specific application metrics for monitoring application behavior.
///
/// These metrics track application-specific measurements relevant to blueprint
/// operation such as job execution counts, processing durations, queue depths,
/// and custom metrics defined by the application. They focus on business logic
/// rather than system resources.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BlueprintMetrics {
    pub custom_metrics: HashMap<String, String>,
    pub timestamp: u64,
}

/// Operational status information for a blueprint service instance.
///
/// This structure tracks the operational state of a blueprint service,
/// including its status code, descriptive message, uptime metrics, startup time,
/// and heartbeat activity. It provides a holistic view of service health and
/// availability that can be queried through the `QoS` system.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlueprintStatus {
    pub service_id: u64,
    pub blueprint_id: u64,
    pub status_code: u32,
    pub status_message: Option<String>,
    pub uptime: u64,
    pub start_time: u64,
    pub last_heartbeat: Option<u64>,
    pub timestamp: u64,
}

/// Trait for providing access to system and application metrics.
///
/// This trait defines the core interface for metric collection and retrieval in the `QoS` system.
/// Implementers of this trait are responsible for collecting, storing, and exposing metrics
/// about both the system (CPU, memory, etc.) and the application (blueprint-specific metrics).
/// It supports both current and historical metric access as well as status updates.
pub trait MetricsProvider: Send + Sync {
    /// Get the latest system metrics
    fn get_system_metrics(&self) -> impl Future<Output = SystemMetrics> + Send;
    /// Get the latest blueprint metrics
    fn get_blueprint_metrics(&self) -> impl Future<Output = BlueprintMetrics> + Send;
    /// Get the current blueprint status
    fn get_blueprint_status(&self) -> impl Future<Output = BlueprintStatus> + Send;
    /// Get the historical system metrics
    fn get_system_metrics_history(&self) -> impl Future<Output = Vec<SystemMetrics>> + Send;
    /// Get the historical blueprint metrics
    fn get_blueprint_metrics_history(&self) -> impl Future<Output = Vec<BlueprintMetrics>> + Send;
    /// Add a custom metric
    fn add_custom_metric(&self, key: String, value: String) -> impl Future<Output = ()> + Send;
    /// Set the blueprint status
    fn set_blueprint_status(
        &self,
        status_code: u32,
        status_message: Option<String>,
    ) -> impl Future<Output = ()> + Send;
    /// Update the last heartbeat timestamp
    fn update_last_heartbeat(&self, timestamp: u64) -> impl Future<Output = ()> + Send;
    /// Start the metrics collection background task.
    fn start_collection(&self) -> impl Future<Output = Result<(), Error>> + Send;
}
