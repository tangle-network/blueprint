use std::sync::Arc;

use crate::QoSConfig;
use crate::error::Result;
use crate::heartbeat::{HeartbeatConfig, HeartbeatConsumer};
use crate::logging::{GrafanaConfig, LokiConfig};
use crate::metrics::opentelemetry::OpenTelemetryConfig;
use crate::metrics::types::MetricsConfig;
use crate::servers::{
    grafana::GrafanaServerConfig, loki::LokiServerConfig, prometheus::PrometheusServerConfig,
};
use crate::unified_service::QoSService;

/// Fluent builder for assembling a `QoSService`.
///
/// Use this builder to enable and configure optional sub-systems such as heartbeat
/// reporting, metrics (Prometheus/OpenTelemetry), log aggregation (Loki) and
/// Grafana dashboards. All features are off by default and can be enabled with the
/// corresponding `with_*` methods.
pub struct QoSServiceBuilder<C: HeartbeatConsumer + Send + Sync + 'static> {
    config: QoSConfig,
    heartbeat_consumer: Option<Arc<C>>,
    _phantom_c: std::marker::PhantomData<C>,
    otel_config: Option<OpenTelemetryConfig>,
    prometheus_datasource: Option<String>,
    loki_datasource: Option<String>,
    create_dashboard: bool,
    http_rpc_endpoint: Option<String>,
    ws_rpc_endpoint: Option<String>,
    keystore_uri: Option<String>,
}

impl<C: HeartbeatConsumer + Send + Sync + 'static> Default for QoSServiceBuilder<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: HeartbeatConsumer + Send + Sync + 'static> QoSServiceBuilder<C> {
    /// Creates a new `QoS` service builder with default settings.
    ///
    /// Initializes a builder with an empty configuration. All components (heartbeat, metrics,
    /// logging, etc.) are disabled by default and must be explicitly configured using the
    /// builder methods. This provides a clean starting point for building a custom `QoS`
    /// observability setup.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: QoSConfig::default(),
            heartbeat_consumer: None,
            _phantom_c: std::marker::PhantomData,
            otel_config: None,
            prometheus_datasource: None,
            loki_datasource: None,
            create_dashboard: false,
            http_rpc_endpoint: None,
            ws_rpc_endpoint: None,
            keystore_uri: None,
        }
    }

    /// Sets the complete `QoS` configuration at once.
    ///
    /// This method allows you to provide a pre-configured `QoSConfig` instance,
    /// which can be useful when loading configuration from external sources or
    /// when you want to reuse an existing configuration. This will override any
    /// previous component-specific configurations that were set on the builder.
    ///
    /// # Parameters
    /// * `config` - A complete `QoSConfig` containing settings for all `QoS` components
    #[must_use]
    pub fn with_config(mut self, config: QoSConfig) -> Self {
        self.config = config;
        self
    }

    /// Configures the heartbeat service component.
    ///
    /// The heartbeat service sends periodic signals to the Tangle blockchain to indicate that
    /// the service is alive and functioning properly. This helps prevent slashing of staked
    /// tokens and provides operational visibility.
    ///
    /// # Parameters
    /// * `config` - Configuration for the heartbeat service including service ID, blueprint ID,
    ///   and heartbeat parameters such as interval and jitter
    #[must_use]
    pub fn with_heartbeat_config(mut self, config: HeartbeatConfig) -> Self {
        self.config.heartbeat = Some(config);
        self
    }

    /// Configures the metrics collection component.
    ///
    /// Metrics collection captures system resource usage (CPU, memory, disk, network) and
    /// application-specific metrics (job execution statistics, custom metrics). These metrics
    /// can be exported to Prometheus and visualized in Grafana dashboards.
    ///
    /// # Parameters
    /// * `config` - Configuration for metrics collection including retention settings,
    ///   collection intervals, and export options
    #[must_use]
    pub fn with_metrics_config(mut self, config: MetricsConfig) -> Self {
        self.config.metrics = Some(config);
        self
    }

    /// Configures the Loki logging integration.
    ///
    /// Loki is a log aggregation system that works well with Grafana for visualization.
    /// This integration sends application logs to a Loki server, allowing centralized
    /// log storage, querying, and correlation with metrics.
    ///
    /// # Parameters
    /// * `config` - Configuration for Loki integration including server URL, authentication,
    ///   labels, and batch settings
    #[must_use]
    pub fn with_loki_config(mut self, config: LokiConfig) -> Self {
        self.config.loki = Some(config);
        self
    }

    /// Configures the Grafana integration for dashboard visualization.
    ///
    /// Grafana provides powerful visualization capabilities for metrics and logs. This
    /// configuration allows the `QoS` service to automatically create and update dashboards
    /// that display service health, resource usage, and operational metrics.
    ///
    /// # Parameters
    /// * `config` - Configuration for Grafana integration including server URL, authentication,
    ///   and organization settings
    #[must_use]
    pub fn with_grafana_config(mut self, config: GrafanaConfig) -> Self {
        self.config.grafana = Some(config);
        self
    }

    /// Sets the heartbeat consumer implementation.
    ///
    /// The heartbeat consumer is responsible for processing and submitting heartbeat messages
    /// to the Tangle blockchain. It handles the cryptographic signing of heartbeat messages
    /// and submits them to the appropriate chain endpoint.
    ///
    /// This is required if heartbeat functionality is enabled.
    ///
    /// # Parameters
    /// * `consumer` - Implementation of the `HeartbeatConsumer` trait that will process and
    ///   submit heartbeats to the blockchain
    #[must_use]
    pub fn with_heartbeat_consumer(mut self, consumer: Arc<C>) -> Self {
        self.heartbeat_consumer = Some(consumer);
        self
    }

    /// Configures OpenTelemetry integration for distributed tracing and advanced metrics.
    ///
    /// OpenTelemetry provides a standardized way to collect and export telemetry data
    /// (traces, metrics, logs) across services. This integration enables correlation of
    /// traces with logs and metrics for comprehensive observability.
    ///
    /// # Parameters
    /// * `config` - OpenTelemetry configuration including exporter settings, sampling,
    ///   and resource attribution
    #[must_use]
    pub fn with_otel_config(mut self, config: OpenTelemetryConfig) -> Self {
        self.otel_config = Some(config);
        self
    }

    /// Configures automatic Grafana dashboard creation with a specific Prometheus datasource.
    ///
    /// This method enables the automatic creation or updating of a Grafana dashboard
    /// during `QoS` service initialization. The dashboard will include panels for system
    /// metrics, resource usage, and application-specific metrics sourced from the
    /// specified Prometheus datasource.
    ///
    /// # Parameters
    /// * `datasource_uid` - The Grafana UID of the Prometheus datasource to use for metrics visualization
    #[must_use]
    pub fn with_prometheus_datasource(mut self, datasource_uid: &str) -> Self {
        self.prometheus_datasource = Some(datasource_uid.to_string());
        self.create_dashboard = true;
        self
    }

    /// Configures Grafana dashboard to include logs from a specific Loki datasource.
    ///
    /// When used in combination with `with_prometheus_datasource`, this enables the creation
    /// of comprehensive dashboards that include both metrics and logs. The dashboard will
    /// include panels that allow for correlation between metrics and logs using the same
    /// timestamps.
    ///
    /// # Parameters
    /// * `datasource_uid` - The Grafana UID of the Loki datasource to use for log visualization
    #[must_use]
    pub fn with_loki_datasource(mut self, datasource_uid: &str) -> Self {
        self.loki_datasource = Some(datasource_uid.to_string());
        self
    }

    /// Configures the managed Grafana server instance.
    ///
    /// If server management is enabled, this configuration will be used to start and
    /// manage a Grafana server instance automatically. The server can be run as a
    /// Docker container or embedded server depending on the configuration.
    ///
    /// # Parameters
    /// * `config` - Configuration for the Grafana server including host, port, and authentication settings
    #[must_use]
    pub fn with_grafana_server_config(mut self, config: GrafanaServerConfig) -> Self {
        self.config.grafana_server = Some(config);
        self
    }

    /// Configures the managed Loki log aggregation server instance.
    ///
    /// If server management is enabled, this configuration will be used to start and
    /// manage a Loki server instance automatically. This server will collect and store
    /// logs from the application for later querying and visualization in Grafana.
    ///
    /// # Parameters
    /// * `config` - Configuration for the Loki server including host, port, retention, and storage settings
    #[must_use]
    pub fn with_loki_server_config(mut self, config: LokiServerConfig) -> Self {
        self.config.loki_server = Some(config);
        self
    }

    /// Set the Prometheus server configuration
    #[must_use]
    pub fn with_prometheus_server_config(mut self, config: PrometheusServerConfig) -> Self {
        self.config.prometheus_server = Some(config);
        self
    }

    /// Enable or disable server management
    #[must_use]
    pub fn manage_servers(mut self, manage: bool) -> Self {
        self.config.manage_servers = manage;
        self
    }

    /// Build the `QoS` service
    ///
    /// # Errors
    /// Returns an error if the heartbeat consumer is not provided or if the service initialization fails
    /// Set the HTTP RPC endpoint for `HeartbeatService`
    #[must_use]
    pub fn with_http_rpc_endpoint(mut self, endpoint: String) -> Self {
        self.http_rpc_endpoint = Some(endpoint);
        self
    }

    /// Set the WebSocket RPC endpoint for `HeartbeatService`
    #[must_use]
    pub fn with_ws_rpc_endpoint(mut self, endpoint: String) -> Self {
        self.ws_rpc_endpoint = Some(endpoint);
        self
    }

    /// Set the Keystore URI for `HeartbeatService`
    #[must_use]
    pub fn with_keystore_uri(mut self, uri: String) -> Self {
        self.keystore_uri = Some(uri);
        self
    }

    /// Build the `QoS` service
    ///
    /// # Errors
    /// Returns an error if the heartbeat consumer is not provided or if the service initialization fails
    pub async fn build(self) -> Result<QoSService<C>> {
        let heartbeat_consumer = self.heartbeat_consumer.ok_or_else(|| {
            crate::error::Error::Other("Heartbeat consumer is required".to_string())
        })?;

        let ws_rpc = self.ws_rpc_endpoint.unwrap_or_default();
        let keystore = self.keystore_uri.unwrap_or_default();

        if let Some(otel_config) = self.otel_config {
            QoSService::with_otel_config(
                self.config,
                heartbeat_consumer,
                ws_rpc,
                keystore,
                otel_config,
            )
            .await
        } else {
            QoSService::new(self.config, heartbeat_consumer, ws_rpc, keystore).await
        }
    }
}
