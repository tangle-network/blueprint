//! Observability and metrics collection for remote providers

use blueprint_std::collections::HashMap;
use blueprint_std::sync::Arc;
use tokio::sync::RwLock;

/// Metrics collector for remote provider observability
pub struct MetricsCollector {
    metrics: Arc<RwLock<HashMap<String, f64>>>,
}

impl MetricsCollector {
    /// Create new metrics collector
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Record a metric value
    pub async fn record(&self, name: &str, value: f64) {
        let mut metrics = self.metrics.write().await;
        metrics.insert(name.to_string(), value);
    }

    /// Get all current metrics
    pub async fn get_metrics(&self) -> HashMap<String, f64> {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }
    
    /// Alias for compatibility with QoS module
    pub async fn get_all_metrics(&self) -> HashMap<String, f64> {
        self.get_metrics().await
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}