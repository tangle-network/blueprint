use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tracing::info;

use crate::error::Result;

/// Configuration for the metrics service
#[derive(Clone, Debug)]
pub struct MetricsConfig {
    pub bind_address: String,

    pub collection_interval_secs: u64,

    pub max_history: usize,

    pub service_id: u64,

    pub blueprint_id: u64,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:50051".to_string(),
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
    /// CPU usage percentage
    pub cpu_usage: f32,

    /// Memory usage in bytes
    pub memory_usage: u64,

    /// Total memory available in bytes
    pub total_memory: u64,

    /// Disk usage in bytes
    pub disk_usage: u64,

    /// Total disk space in bytes
    pub total_disk: u64,

    /// Network received bytes
    pub network_rx_bytes: u64,

    /// Network transmitted bytes
    pub network_tx_bytes: u64,

    /// Timestamp when metrics were collected
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
                .duration_since(UNIX_EPOCH)
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
                .duration_since(UNIX_EPOCH)
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
            .duration_since(UNIX_EPOCH)
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
    /// Get system metrics
    fn get_system_metrics(&self) -> SystemMetrics;

    /// Get blueprint-specific metrics
    fn get_blueprint_metrics(&self) -> BlueprintMetrics;

    /// Get blueprint status
    fn get_blueprint_status(&self) -> BlueprintStatus;

    /// Get system metrics history
    fn get_system_metrics_history(&self) -> Vec<SystemMetrics>;

    /// Get blueprint metrics history
    fn get_blueprint_metrics_history(&self) -> Vec<BlueprintMetrics>;

    /// Add a custom metric
    fn add_custom_metric(&self, key: String, value: String);

    /// Set the blueprint status
    fn set_blueprint_status(&self, status_code: u32, status_message: Option<String>);

    /// Update the last heartbeat time
    fn update_last_heartbeat(&self, timestamp: u64);
}

/// Default implementation of the metrics provider
pub struct DefaultMetricsProvider {
    system_metrics: Arc<RwLock<Vec<SystemMetrics>>>,

    blueprint_metrics: Arc<RwLock<Vec<BlueprintMetrics>>>,

    blueprint_status: Arc<RwLock<BlueprintStatus>>,

    custom_metrics: Arc<RwLock<HashMap<String, String>>>,

    start_time: Instant,

    config: MetricsConfig,
}

impl DefaultMetricsProvider {
    /// Create a new default metrics provider
    pub fn new(config: MetricsConfig) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut status = BlueprintStatus::default();
        status.service_id = config.service_id;
        status.blueprint_id = config.blueprint_id;
        status.start_time = now;

        Self {
            system_metrics: Arc::new(RwLock::new(Vec::with_capacity(config.max_history))),
            blueprint_metrics: Arc::new(RwLock::new(Vec::with_capacity(config.max_history))),
            blueprint_status: Arc::new(RwLock::new(status)),
            custom_metrics: Arc::new(RwLock::new(HashMap::new())),
            start_time: Instant::now(),
            config,
        }
    }

    /// Start collecting metrics
    pub fn start_collection(&self) -> Result<()> {
        let system_metrics = self.system_metrics.clone();
        let blueprint_metrics = self.blueprint_metrics.clone();
        let blueprint_status = self.blueprint_status.clone();
        let custom_metrics = self.custom_metrics.clone();
        let start_time = self.start_time;
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(Duration::from_secs(config.collection_interval_secs));

            loop {
                interval.tick().await;

                let sys_metrics = DefaultMetricsProvider::collect_system_metrics();

                {
                    let mut metrics = system_metrics.write().unwrap();
                    metrics.push(sys_metrics);
                    if metrics.len() > config.max_history {
                        metrics.remove(0);
                    }
                }

                let mut bp_metrics = BlueprintMetrics::default();
                {
                    let custom = custom_metrics.read().unwrap();
                    bp_metrics.custom_metrics = custom.clone();
                }

                {
                    let mut metrics = blueprint_metrics.write().unwrap();
                    metrics.push(bp_metrics);
                    if metrics.len() > config.max_history {
                        metrics.remove(0);
                    }
                }

                {
                    let mut status = blueprint_status.write().unwrap();
                    status.uptime = start_time.elapsed().as_secs();
                    status.timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                }

                info!("Collected metrics");
            }
        });

        Ok(())
    }

    /// Collect system metrics
    fn collect_system_metrics() -> SystemMetrics {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        // Use only the metrics that are definitely available
        let memory_usage = sys.used_memory();
        let total_memory = sys.total_memory();
        
        // Use global_cpu_usage as suggested in the error message
        let cpu_usage = sys.global_cpu_usage();

        // Set disk and network metrics to 0 for now
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
}

impl MetricsProvider for DefaultMetricsProvider {
    fn get_system_metrics(&self) -> SystemMetrics {
        match self.system_metrics.read().unwrap().last() {
            Some(metrics) => metrics.clone(),
            None => SystemMetrics::default(),
        }
    }

    fn get_blueprint_metrics(&self) -> BlueprintMetrics {
        match self.blueprint_metrics.read().unwrap().last() {
            Some(metrics) => metrics.clone(),
            None => BlueprintMetrics::default(),
        }
    }

    fn get_blueprint_status(&self) -> BlueprintStatus {
        self.blueprint_status.read().unwrap().clone()
    }

    fn get_system_metrics_history(&self) -> Vec<SystemMetrics> {
        self.system_metrics.read().unwrap().clone()
    }

    fn get_blueprint_metrics_history(&self) -> Vec<BlueprintMetrics> {
        self.blueprint_metrics.read().unwrap().clone()
    }

    fn add_custom_metric(&self, key: String, value: String) {
        self.custom_metrics.write().unwrap().insert(key, value);
    }

    fn set_blueprint_status(&self, status_code: u32, status_message: Option<String>) {
        let mut status = self.blueprint_status.write().unwrap();
        status.status_code = status_code;
        status.status_message = status_message;
    }

    fn update_last_heartbeat(&self, timestamp: u64) {
        let mut status = self.blueprint_status.write().unwrap();
        status.last_heartbeat = Some(timestamp);
    }
}
