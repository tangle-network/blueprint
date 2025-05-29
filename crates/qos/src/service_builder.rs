use blueprint_core::error;
use std::sync::Arc;

use crate::QoSConfig;
use crate::error::Result;
use crate::heartbeat::{HeartbeatConfig, HeartbeatConsumer};
use crate::logging::{GrafanaConfig, LokiConfig};
use crate::metrics::opentelemetry::OpenTelemetryConfig;
use crate::metrics::types::MetricsConfig;
use crate::servers::{
    grafana::GrafanaServerConfig,
    loki::LokiServerConfig,
    prometheus::PrometheusServerConfig,
};
use crate::unified_service::QoSService;

/// Builder for `QoS` service
pub struct QoSServiceBuilder<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    config: QoSConfig,
    heartbeat_consumer: Option<Arc<C>>,
    otel_config: Option<OpenTelemetryConfig>,
    prometheus_datasource: Option<String>,
    loki_datasource: Option<String>,
    create_dashboard: bool,
}

impl<C> QoSServiceBuilder<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    /// Create a new `QoS` service builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: QoSConfig::default(),
            heartbeat_consumer: None,
            otel_config: None,
            prometheus_datasource: None,
            loki_datasource: None,
            create_dashboard: false,
        }
    }

    /// Set the `QoS` configuration
    #[must_use]
    pub fn with_config(mut self, config: QoSConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the heartbeat configuration
    #[must_use]
    pub fn with_heartbeat_config(mut self, config: HeartbeatConfig) -> Self {
        self.config.heartbeat = Some(config);
        self
    }

    /// Set the metrics configuration
    #[must_use]
    pub fn with_metrics_config(mut self, config: MetricsConfig) -> Self {
        self.config.metrics = Some(config);
        self
    }

    /// Set the Loki configuration
    #[must_use]
    pub fn with_loki_config(mut self, config: LokiConfig) -> Self {
        self.config.loki = Some(config);
        self
    }

    /// Set the Grafana configuration
    #[must_use]
    pub fn with_grafana_config(mut self, config: GrafanaConfig) -> Self {
        self.config.grafana = Some(config);
        self
    }

    /// Set the heartbeat consumer
    #[must_use]
    pub fn with_heartbeat_consumer(mut self, consumer: Arc<C>) -> Self {
        self.heartbeat_consumer = Some(consumer);
        self
    }

    /// Set the OpenTelemetry configuration
    #[must_use]
    pub fn with_otel_config(mut self, config: OpenTelemetryConfig) -> Self {
        self.otel_config = Some(config);
        self
    }

    /// Enable dashboard creation with the specified Prometheus datasource UID
    #[must_use]
    pub fn with_prometheus_datasource(mut self, datasource_uid: &str) -> Self {
        self.prometheus_datasource = Some(datasource_uid.to_string());
        self.create_dashboard = true;
        self
    }

    /// Enable dashboard creation with the specified Loki datasource UID
    #[must_use]
    pub fn with_loki_datasource(mut self, datasource_uid: &str) -> Self {
        self.loki_datasource = Some(datasource_uid.to_string());
        self
    }

    /// Set the Grafana server configuration
    #[must_use]
    pub fn with_grafana_server_config(mut self, config: GrafanaServerConfig) -> Self {
        self.config.grafana_server = Some(config);
        self
    }

    /// Set the Loki server configuration
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
    pub async fn build(self) -> Result<QoSService<C>> {
        let heartbeat_consumer = self.heartbeat_consumer.ok_or_else(|| {
            crate::error::Error::Other("Heartbeat consumer is required".to_string())
        })?;

        // Create the QoS service
        let mut service = if let Some(otel_config) = self.otel_config {
            QoSService::with_otel_config(self.config, heartbeat_consumer, otel_config).await?
        } else {
            QoSService::new(self.config, heartbeat_consumer).await?
        };

        // Create a dashboard if requested
        if self.create_dashboard && service.grafana_client().is_some() {
            let prometheus_ds = self
                .prometheus_datasource
                .unwrap_or_else(|| "prometheus".to_string());
            let loki_ds = self.loki_datasource.unwrap_or_else(|| "loki".to_string());

            if let Err(e) = service.create_dashboard(&prometheus_ds, &loki_ds).await {
                error!("Failed to create dashboard: {}", e);
            }
        }

        Ok(service)
    }
}

impl<C> Default for QoSServiceBuilder<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
