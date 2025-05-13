pub mod error;
pub mod heartbeat;
pub mod logging;
pub mod metrics;
pub mod service;
pub mod unified_service;

// pub mod proto {
//     tonic::include_proto!("qos");
// }

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/qos.rs"));
}

pub use logging::{GrafanaClient, GrafanaConfig, LokiConfig};
pub use unified_service::{QoSService, QoSServiceBuilder};

/// Configuration for the QoS system
#[derive(Clone, Debug)]
pub struct QoSConfig {
    /// Heartbeat configuration
    pub heartbeat: Option<heartbeat::HeartbeatConfig>,

    /// Metrics configuration
    pub metrics: Option<metrics::MetricsConfig>,

    /// Loki logging configuration
    pub loki: Option<logging::LokiConfig>,

    /// Grafana configuration
    pub grafana: Option<logging::GrafanaConfig>,
}

impl Default for QoSConfig {
    fn default() -> Self {
        Self {
            heartbeat: None,
            metrics: None,
            loki: None,
            grafana: None,
        }
    }
}

/// Create a new QoS configuration with default values
pub fn default_qos_config() -> QoSConfig {
    QoSConfig {
        heartbeat: Some(heartbeat::HeartbeatConfig::default()),
        metrics: Some(metrics::MetricsConfig::default()),
        loki: Some(logging::LokiConfig::default()),
        grafana: Some(logging::GrafanaConfig::default()),
    }
}
