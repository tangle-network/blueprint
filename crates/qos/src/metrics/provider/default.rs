use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tracing::info;

use crate::metrics::types::{
    BlueprintMetrics, BlueprintStatus, MetricsConfig, MetricsProvider, SystemMetrics,
};

/// Default metrics provider implementation
pub struct DefaultMetricsProvider {
    system_metrics: Arc<RwLock<Vec<SystemMetrics>>>,
    blueprint_metrics: Arc<RwLock<Vec<BlueprintMetrics>>>,
    blueprint_status: Arc<RwLock<BlueprintStatus>>,
    custom_metrics: Arc<RwLock<std::collections::HashMap<String, String>>>,
    config: MetricsConfig,
    start_time: Instant,
}

impl DefaultMetricsProvider {
    /// Create a new default metrics provider
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            system_metrics: Arc::new(RwLock::new(Vec::new())),
            blueprint_metrics: Arc::new(RwLock::new(Vec::new())),
            blueprint_status: Arc::new(RwLock::new(BlueprintStatus::default())),
            custom_metrics: Arc::new(RwLock::new(std::collections::HashMap::new())),
            config,
            start_time: Instant::now(),
        }
    }

    /// Start collecting metrics
    pub fn start_collection(&self) -> crate::error::Result<()> {
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
                    let mut metrics = match system_metrics.write() {
                        Ok(lock) => lock,
                        Err(e) => {
                            tracing::error!("Failed to acquire system_metrics write lock: {}", e);
                            continue;
                        }
                    };
                    metrics.push(sys_metrics);
                    if metrics.len() > config.max_history {
                        metrics.remove(0);
                    }
                }
                let mut bp_metrics = BlueprintMetrics::default();
                {
                    let custom = match custom_metrics.read() {
                        Ok(lock) => lock,
                        Err(e) => {
                            tracing::error!("Failed to acquire custom_metrics read lock: {}", e);
                            continue;
                        }
                    };
                    bp_metrics.custom_metrics = custom.clone();
                }
                {
                    let mut metrics = match blueprint_metrics.write() {
                        Ok(lock) => lock,
                        Err(e) => {
                            tracing::error!(
                                "Failed to acquire blueprint_metrics write lock: {}",
                                e
                            );
                            continue;
                        }
                    };
                    metrics.push(bp_metrics);
                    if metrics.len() > config.max_history {
                        metrics.remove(0);
                    }
                }
                {
                    let mut status = match blueprint_status.write() {
                        Ok(lock) => lock,
                        Err(e) => {
                            tracing::error!("Failed to acquire blueprint_status write lock: {}", e);
                            continue;
                        }
                    };
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
        let memory_usage = sys.used_memory();
        let total_memory = sys.total_memory();
        let cpu_usage = sys.global_cpu_usage();
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
        match self.system_metrics.read() {
            Ok(guard) => guard.last().cloned().unwrap_or_default(),
            Err(e) => {
                tracing::error!("Failed to acquire system_metrics read lock: {}", e);
                SystemMetrics::default()
            }
        }
    }
    fn get_blueprint_metrics(&self) -> BlueprintMetrics {
        match self.blueprint_metrics.read() {
            Ok(guard) => guard.last().cloned().unwrap_or_default(),
            Err(e) => {
                tracing::error!("Failed to acquire blueprint_metrics read lock: {}", e);
                BlueprintMetrics::default()
            }
        }
    }
    fn get_blueprint_status(&self) -> BlueprintStatus {
        match self.blueprint_status.read() {
            Ok(guard) => guard.clone(),
            Err(e) => {
                tracing::error!("Failed to acquire blueprint_status read lock: {}", e);
                BlueprintStatus::default()
            }
        }
    }
    fn get_system_metrics_history(&self) -> Vec<SystemMetrics> {
        match self.system_metrics.read() {
            Ok(guard) => guard.clone(),
            Err(e) => {
                tracing::error!("Failed to acquire system_metrics read lock: {}", e);
                Vec::new()
            }
        }
    }
    fn get_blueprint_metrics_history(&self) -> Vec<BlueprintMetrics> {
        match self.blueprint_metrics.read() {
            Ok(guard) => guard.clone(),
            Err(e) => {
                tracing::error!("Failed to acquire blueprint_metrics read lock: {}", e);
                Vec::new()
            }
        }
    }
    fn add_custom_metric(&self, key: String, value: String) {
        if let Ok(mut custom) = self.custom_metrics.write() {
            custom.insert(key, value);
        } else {
            tracing::error!("Failed to acquire custom_metrics write lock");
        }
    }
    fn set_blueprint_status(&self, status_code: u32, status_message: Option<String>) {
        if let Ok(mut status) = self.blueprint_status.write() {
            status.status_code = status_code;
            status.status_message = status_message;
        } else {
            tracing::error!("Failed to acquire blueprint_status write lock");
        }
    }
    fn update_last_heartbeat(&self, timestamp: u64) {
        if let Ok(mut status) = self.blueprint_status.write() {
            status.last_heartbeat = Some(timestamp);
        } else {
            tracing::error!("Failed to acquire blueprint_status write lock");
        }
    }
}
