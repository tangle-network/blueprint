use std::sync::Arc; // Arc can remain std::sync::Arc as tokio::sync::RwLock is Send + Sync
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock; // Changed from std::sync to tokio::sync

use crate::metrics::types::{
    BlueprintMetrics, BlueprintStatus, MetricsConfig, MetricsProvider, SystemMetrics,
};

/// Default metrics provider implementation
pub struct DefaultMetricsProvider {
    system_metrics: Arc<tokio::sync::RwLock<Vec<SystemMetrics>>>,
    blueprint_metrics: Arc<tokio::sync::RwLock<Vec<BlueprintMetrics>>>,
    blueprint_status: Arc<tokio::sync::RwLock<BlueprintStatus>>,
    custom_metrics: Arc<tokio::sync::RwLock<std::collections::HashMap<String, String>>>,
    config: MetricsConfig,
    start_time: Instant,
}

impl DefaultMetricsProvider {
    /// Create a new `DefaultMetricsProvider`
    #[must_use]
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

    /// Test helper: Clones the internal Arc for `system_metrics`.
    #[must_use]
    pub fn system_metrics_arc_clone(&self) -> Arc<tokio::sync::RwLock<Vec<SystemMetrics>>> {
        self.system_metrics.clone()
    }

    /// Test helper: Clones the internal Arc for `blueprint_metrics`.
    #[must_use]
    pub fn blueprint_metrics_arc_clone(&self) -> Arc<tokio::sync::RwLock<Vec<BlueprintMetrics>>> {
        self.blueprint_metrics.clone()
    }

    /// Test helper: Clones the internal Arc for `blueprint_status`.
    #[must_use]
    pub fn blueprint_status_arc_clone(&self) -> Arc<tokio::sync::RwLock<BlueprintStatus>> {
        self.blueprint_status.clone()
    }

    /// Test helper: Clones the internal Arc for `custom_metrics`.
    #[must_use]
    pub fn custom_metrics_arc_clone(
        &self,
    ) -> Arc<tokio::sync::RwLock<std::collections::HashMap<String, String>>> {
        self.custom_metrics.clone()
    }

    /// Collect system metrics
    fn collect_system_metrics() -> SystemMetrics {
        use sysinfo::{Disks, Networks, System};

        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_usage = sys.global_cpu_usage();

        let memory_usage = sys.used_memory();
        let total_memory = sys.total_memory();

        let mut disk_usage: u64 = 0;
        let mut total_disk: u64 = 0;
        for disk in Disks::new_with_refreshed_list().list() {
            total_disk += disk.total_space();
            disk_usage += disk.total_space() - disk.available_space();
        }

        let mut network_rx_bytes: u64 = 0;
        let mut network_tx_bytes: u64 = 0;
        for data in Networks::new_with_refreshed_list().list().values() {
            network_rx_bytes += data.received();
            network_tx_bytes += data.transmitted();
        }

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
    /// Returns the latest collected `SystemMetrics`.
    #[must_use]
    async fn get_system_metrics(&self) -> SystemMetrics {
        if let Ok(metrics_guard) = self.system_metrics.try_read() {
            metrics_guard.last().cloned().unwrap_or_default()
        } else {
            eprintln!(
                "ERROR: System metrics lock poisoned or unavailable in get_system_metrics. Returning default."
            );
            SystemMetrics::default()
        }
    }
    /// Returns the latest collected `BlueprintMetrics`.
    #[must_use]
    async fn get_blueprint_metrics(&self) -> BlueprintMetrics {
        if let Ok(metrics_guard) = self.blueprint_metrics.try_read() {
            metrics_guard.last().cloned().unwrap_or_default()
        } else {
            eprintln!(
                "ERROR: Blueprint metrics lock poisoned or unavailable in get_blueprint_metrics. Returning default."
            );
            BlueprintMetrics::default()
        }
    }
    /// Returns the current `BlueprintStatus`.
    #[must_use]
    async fn get_blueprint_status(&self) -> BlueprintStatus {
        if let Ok(status_guard) = self.blueprint_status.try_read() {
            status_guard.clone()
        } else {
            eprintln!(
                "ERROR: Blueprint status lock poisoned or unavailable in get_blueprint_status. Returning default."
            );
            BlueprintStatus::default()
        }
    }
    /// Returns a history of collected `SystemMetrics`.
    #[must_use]
    async fn get_system_metrics_history(&self) -> Vec<SystemMetrics> {
        if let Ok(metrics_guard) = self.system_metrics.try_read() {
            metrics_guard.clone()
        } else {
            eprintln!(
                "ERROR: System metrics lock poisoned or unavailable in get_system_metrics_history. Returning empty history."
            );
            Vec::new()
        }
    }
    /// Returns a history of collected `BlueprintMetrics`.
    #[must_use]
    async fn get_blueprint_metrics_history(&self) -> Vec<BlueprintMetrics> {
        if let Ok(metrics_guard) = self.blueprint_metrics.try_read() {
            metrics_guard.clone()
        } else {
            eprintln!(
                "ERROR: Blueprint metrics lock poisoned or unavailable in get_blueprint_metrics_history. Returning empty history."
            );
            Vec::new()
        }
    }
    /// Adds a custom key-value metric.
    async fn add_custom_metric(&self, key: String, value: String) {
        if let Ok(mut custom_guard) = self.custom_metrics.try_write() {
            custom_guard.insert(key, value);
        } else {
            eprintln!(
                "ERROR: Custom metrics lock poisoned or unavailable in add_custom_metric. Metric not added."
            );
        }
    }
    /// Sets the current `BlueprintStatus`.
    async fn set_blueprint_status(&self, status_code: u32, status_message: Option<String>) {
        if let Ok(mut status_guard) = self.blueprint_status.try_write() {
            status_guard.status_code = status_code;
            status_guard.status_message = status_message;
        } else {
            eprintln!(
                "ERROR: Blueprint status lock poisoned or unavailable in set_blueprint_status. Status not set."
            );
        }
    }
    /// Updates the last heartbeat timestamp in `BlueprintStatus`.
    async fn update_last_heartbeat(&self, timestamp: u64) {
        if let Ok(mut status_guard) = self.blueprint_status.try_write() {
            status_guard.last_heartbeat = Some(timestamp);
        } else {
            eprintln!(
                "ERROR: Blueprint status lock poisoned or unavailable in update_last_heartbeat. Heartbeat not updated."
            );
        }
    }

    /// Starts the background task for periodic metrics collection.
    ///
    /// # Errors
    ///
    /// Returns an error if the metrics collection background task cannot be started.
    async fn start_collection(&self) -> Result<(), crate::error::Error> {
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
                let sys_metrics_data = DefaultMetricsProvider::collect_system_metrics();
                if let Ok(mut metrics) = system_metrics.try_write() {
                    metrics.push(sys_metrics_data);
                    if metrics.len() > config.max_history {
                        metrics.remove(0);
                    }
                } else {
                    println!(
                        "COLLECTION_LOOP_ERROR: Failed to acquire system_metrics write lock (try_write); skipping system metrics update for this cycle."
                    );
                }

                let mut bp_metrics_data = BlueprintMetrics::default();
                if let Ok(custom) = custom_metrics.try_read() {
                    bp_metrics_data.custom_metrics = custom.clone();
                } else {
                    println!(
                        "COLLECTION_LOOP_ERROR: Failed to acquire custom_metrics read lock (try_read); skipping custom metrics update for this cycle."
                    );
                }

                if let Ok(mut metrics) = blueprint_metrics.try_write() {
                    metrics.push(bp_metrics_data);
                    if metrics.len() > config.max_history {
                        metrics.remove(0);
                    }
                } else {
                    println!(
                        "COLLECTION_LOOP_ERROR: Failed to acquire blueprint_metrics write lock (try_write); skipping blueprint metrics update for this cycle."
                    );
                }

                if let Ok(mut status) = blueprint_status.try_write() {
                    status.uptime = Instant::now().duration_since(start_time).as_secs();
                    status.timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                } else {
                    println!(
                        "COLLECTION_LOOP_ERROR: Failed to acquire blueprint_status write lock (try_write); skipping blueprint status update for this cycle."
                    );
                }
                println!("COLLECTION_LOOP_INFO: Metrics collection cycle finished.");
            }
        });
        Ok(())
    }
}
