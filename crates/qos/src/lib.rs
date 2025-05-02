pub mod error;
pub mod heartbeat;
pub mod metrics;
pub mod service;

pub mod proto {
    tonic::include_proto!("qos");
}

/// Configuration for the QoS system
#[derive(Clone, Debug)]
pub struct QoSConfig {
    pub heartbeat: Option<heartbeat::HeartbeatConfig>,

    pub metrics: Option<metrics::MetricsConfig>,
}

impl Default for QoSConfig {
    fn default() -> Self {
        Self {
            heartbeat: None,
            metrics: None,
        }
    }
}
