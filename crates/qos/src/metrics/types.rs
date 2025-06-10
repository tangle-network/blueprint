use std::collections::HashMap;
use std::time::SystemTime;
use crate::servers::prometheus::PrometheusServerConfig;

/// Configuration for the metrics service
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MetricsConfig {
    // pub bind_address: String, // This is now part of PrometheusServerConfig
    pub prometheus_server: Option<PrometheusServerConfig>,
    pub collection_interval_secs: u64,
    pub max_history: usize,
    pub service_id: u64,
    pub blueprint_id: u64,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            // bind_address: "0.0.0.0:9090".to_string(),
            prometheus_server: Some(PrometheusServerConfig::default()),
            collection_interval_secs: 60,
            max_history: 100,
            service_id: 0,
            blueprint_id: 0,
        }
    }
}

/// System metrics
#[derive(Clone, Debug)]
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

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            total_memory: 0,
            disk_usage: 0,
            total_disk: 0,
            network_rx_bytes: 0,
            network_tx_bytes: 0,
            timestamp: SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

/// Blueprint-specific metrics
#[derive(Clone, Debug)]
pub struct BlueprintMetrics {
    pub custom_metrics: HashMap<String, String>,
    pub timestamp: u64,
}

impl Default for BlueprintMetrics {
    fn default() -> Self {
        Self {
            custom_metrics: HashMap::new(),
            timestamp: SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

/// Status information for a blueprint
#[derive(Clone, Debug)]
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

impl Default for BlueprintStatus {
    fn default() -> Self {
        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            service_id: 0,
            blueprint_id: 0,
            status_code: 0,
            status_message: None,
            uptime: 0,
            start_time: now,
            last_heartbeat: None,
            timestamp: now,
        }
    }
}

/// Trait for providing metrics
pub trait MetricsProvider: Send + Sync + 'static {
    fn get_system_metrics(&self) -> SystemMetrics;
    fn get_blueprint_metrics(&self) -> BlueprintMetrics;
    fn get_blueprint_status(&self) -> BlueprintStatus;
    fn get_system_metrics_history(&self) -> Vec<SystemMetrics>;
    fn get_blueprint_metrics_history(&self) -> Vec<BlueprintMetrics>;
    fn add_custom_metric(&self, key: String, value: String);
    fn set_blueprint_status(&self, status_code: u32, status_message: Option<String>);
    fn update_last_heartbeat(&self, timestamp: u64);
}
