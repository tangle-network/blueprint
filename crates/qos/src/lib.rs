pub mod error;
pub mod heartbeat;
pub mod logging;
pub mod metrics;
pub mod servers;
pub mod service;
pub mod service_builder;
pub mod unified_service;

// Allow clippy lints in generated code
#[allow(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_errors_doc,
    clippy::wildcard_imports,
    clippy::doc_markdown,
    clippy::used_underscore_items,
    clippy::default_trait_access
)]
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/qos.rs"));
}

pub use logging::{GrafanaClient, GrafanaConfig, LokiConfig};
pub use servers::{
    grafana::GrafanaServerConfig, loki::LokiServerConfig, prometheus::PrometheusServerConfig,
};
pub use service_builder::QoSServiceBuilder;
pub use unified_service::QoSService;

/// Configuration for the `QoS` system
#[derive(Clone, Debug, Default)]
pub struct QoSConfig {
    /// Heartbeat configuration
    pub heartbeat: Option<heartbeat::HeartbeatConfig>,

    /// Metrics configuration
    pub metrics: Option<metrics::MetricsConfig>,

    /// Loki logging configuration
    pub loki: Option<logging::LokiConfig>,

    /// Grafana configuration
    pub grafana: Option<logging::GrafanaConfig>,

    /// Grafana server configuration (if None, no server will be started)
    pub grafana_server: Option<servers::grafana::GrafanaServerConfig>,

    /// Docker network for managed servers (if None, default network will be used)
    pub docker_network: Option<String>,

    /// Loki server configuration (if None, no server will be started)
    pub loki_server: Option<servers::loki::LokiServerConfig>,

    /// Prometheus server configuration (if None, no server will be started)
    pub prometheus_server: Option<servers::prometheus::PrometheusServerConfig>,

    /// Whether to manage servers automatically
    pub manage_servers: bool,

    /// Optional Service ID for Grafana dashboard context
    pub service_id: Option<u64>,

    /// Optional Blueprint ID for Grafana dashboard context
    pub blueprint_id: Option<u64>,
}

/// Creates a new `QoS` configuration with sensible default values for all components.
///
/// This function initializes `QoSConfig` with defaults for heartbeat, metrics, Loki logging,
/// and Grafana, but disables automatic server management. Prometheus server is not
/// enabled by default. This configuration is suitable as a starting point that can be
/// further customized.
#[must_use]
pub fn default_qos_config() -> QoSConfig {
    QoSConfig {
        heartbeat: Some(heartbeat::HeartbeatConfig::default()),
        metrics: Some(metrics::MetricsConfig::default()),
        loki: Some(logging::LokiConfig::default()),
        grafana: Some(logging::GrafanaConfig::default()),
        grafana_server: Some(servers::grafana::GrafanaServerConfig::default()),
        loki_server: Some(servers::loki::LokiServerConfig::default()),
        prometheus_server: None,
        docker_network: None,
        manage_servers: false,
        service_id: None,
        blueprint_id: None,
    }
}
