use std::sync::Arc;
use blueprint_core::{error, info};

use crate::QoSConfig;
use crate::error::Result;
use crate::heartbeat::{HeartbeatConfig, HeartbeatConsumer, HeartbeatService};
use crate::logging::{GrafanaClient, GrafanaConfig, LokiConfig, init_loki_logging};
use crate::metrics::opentelemetry::OpenTelemetryConfig;
use crate::metrics::provider::EnhancedMetricsProvider;
use crate::metrics::service::MetricsService;
use crate::metrics::types::MetricsConfig;
use crate::servers::{
    ServerManager,
    grafana::{GrafanaServer, GrafanaServerConfig},
    loki::{LokiServer, LokiServerConfig},
    prometheus::{PrometheusServer, PrometheusServerConfig},
};

/// Unified `QoS` service that combines heartbeat, metrics, logging, and dashboard functionality
pub struct QoSService<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    /// Heartbeat service
    #[allow(dead_code)]
    heartbeat_service: Option<HeartbeatService<C>>,

    /// Metrics service
    metrics_service: Option<MetricsService>,

    /// Grafana client
    grafana_client: Option<Arc<GrafanaClient>>,

    /// Configuration
    config: QoSConfig,

    /// Dashboard URL
    dashboard_url: Option<String>,

    /// Grafana server manager
    grafana_server: Option<Arc<GrafanaServer>>,

    /// Loki server manager
    loki_server: Option<Arc<LokiServer>>,
    
    /// Prometheus server manager
    prometheus_server: Option<Arc<PrometheusServer>>,
}

impl<C> QoSService<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    /// Create a new `QoS` service with heartbeat, metrics, and optional Loki/Grafana integration
    ///
    /// # Errors
    /// Returns an error if the metrics service initialization fails
    pub async fn new(config: QoSConfig, heartbeat_consumer: Arc<C>) -> Result<Self> {
        // Initialize heartbeat service if configured
        let heartbeat_service = config
            .heartbeat
            .clone()
            .map(|config| HeartbeatService::new(config, heartbeat_consumer));

        // Initialize metrics service if configured
        let metrics_service = if let Some(metrics_config) = config.metrics.clone() {
            Some(MetricsService::new(metrics_config)?)
        } else {
            None
        };

        // Initialize Loki logging if configured
        if let Some(loki_config) = &config.loki {
            if let Err(e) = init_loki_logging(loki_config.clone()) {
                error!("Failed to initialize Loki logging: {}", e);
            } else {
                info!("Initialized Loki logging");
            }
        }

        // Initialize Grafana client if configured
        let _grafana_client = config
            .grafana
            .as_ref()
            .map(|grafana_config| Arc::new(GrafanaClient::new(grafana_config.clone())));

        // Initialize server managers if configured
        let (grafana_server, loki_server, prometheus_server) = if config.manage_servers {
            let (grafana_server, loki_server, prometheus_server) = (
                config
                    .grafana_server
                    .as_ref()
                    .map(|cfg| Arc::new(GrafanaServer::new(cfg.clone()))),
                config
                    .loki_server
                    .as_ref()
                    .map(|cfg| Arc::new(LokiServer::new(cfg.clone()))),
                config
                    .prometheus_server
                    .as_ref()
                    .map(|cfg| Arc::new(PrometheusServer::new(cfg.clone()))),
            );

            // Start the servers if configured
            if let Some(server) = &grafana_server {
                info!("Starting Grafana server...");
                if let Err(e) = server.start().await {
                    error!("Failed to start Grafana server: {}", e);
                } else {
                    info!("Grafana server started successfully");
                }
            }

            if let Some(server) = &loki_server {
                info!("Starting Loki server...");
                if let Err(e) = server.start().await {
                    error!("Failed to start Loki server: {}", e);
                } else {
                    info!("Loki server started successfully");
                }
            }
            
            if let Some(server) = &prometheus_server {
                info!("Starting Prometheus server...");
                if let Err(e) = server.start().await {
                    error!("Failed to start Prometheus server: {}", e);
                } else {
                    info!("Prometheus server started successfully");
                }
            }

            (grafana_server, loki_server, prometheus_server)
        } else {
            (None, None, None)
        };

        // Update Grafana client if we are managing servers
        let grafana_client = if let Some(server) = &grafana_server {
            Some(Arc::new(GrafanaClient::new(server.client_config())))
        } else {
            // Otherwise use the provided config
            config
                .grafana
                .as_ref()
                .map(|grafana_config| Arc::new(GrafanaClient::new(grafana_config.clone())))
        };

        Ok(Self {
            heartbeat_service,
            metrics_service,
            grafana_client,
            config,
            dashboard_url: None,
            grafana_server,
            loki_server,
            prometheus_server,
        })
    }

    /// Create a new `QoS` service with custom OpenTelemetry configuration
    ///
    /// # Errors
    /// Returns an error if the metrics service initialization fails
    pub async fn with_otel_config(
        config: QoSConfig,
        heartbeat_consumer: Arc<C>,
        otel_config: OpenTelemetryConfig,
    ) -> Result<Self> {
        // Initialize heartbeat service if configured
        let heartbeat_service = config
            .heartbeat
            .clone()
            .map(|config| HeartbeatService::new(config, heartbeat_consumer));

        // Initialize metrics service if configured
        let metrics_service = if let Some(metrics_config) = config.metrics.clone() {
            Some(MetricsService::with_otel_config(metrics_config, otel_config)?)
        } else {
            None
        };

        // Initialize Loki logging if configured
        if let Some(loki_config) = &config.loki {
            if let Err(e) = init_loki_logging(loki_config.clone()) {
                error!("Failed to initialize Loki logging: {}", e);
            } else {
                info!("Initialized Loki logging");
            }
        }

        // Initialize server managers if configured
        let (grafana_server, loki_server, prometheus_server) = if config.manage_servers {
            let (grafana_server, loki_server, prometheus_server) = (
                config
                    .grafana_server
                    .as_ref()
                    .map(|cfg| Arc::new(GrafanaServer::new(cfg.clone()))),
                config
                    .loki_server
                    .as_ref()
                    .map(|cfg| Arc::new(LokiServer::new(cfg.clone()))),
                config
                    .prometheus_server
                    .as_ref()
                    .map(|cfg| Arc::new(PrometheusServer::new(cfg.clone()))),
            );

            // Start the servers if configured
            if let Some(server) = &grafana_server {
                info!("Starting Grafana server...");
                if let Err(e) = server.start().await {
                    error!("Failed to start Grafana server: {}", e);
                } else {
                    info!("Grafana server started successfully");
                }
            }

            if let Some(server) = &loki_server {
                info!("Starting Loki server...");
                if let Err(e) = server.start().await {
                    error!("Failed to start Loki server: {}", e);
                } else {
                    info!("Loki server started successfully");
                }
            }
            
            if let Some(server) = &prometheus_server {
                info!("Starting Prometheus server...");
                if let Err(e) = server.start().await {
                    error!("Failed to start Prometheus server: {}", e);
                } else {
                    info!("Prometheus server started successfully");
                }
            }

            (grafana_server, loki_server, prometheus_server)
        } else {
            (None, None, None)
        };

        // Update Grafana client if we are managing servers
        let grafana_client = if let Some(server) = &grafana_server {
            Some(Arc::new(GrafanaClient::new(server.client_config())))
        } else {
            // Otherwise use the provided config
            config
                .grafana
                .as_ref()
                .map(|grafana_config| Arc::new(GrafanaClient::new(grafana_config.clone())))
        };

        Ok(Self {
            heartbeat_service,
            metrics_service,
            grafana_client,
            config,
            dashboard_url: None,
            grafana_server,
            loki_server,
            prometheus_server,
        })
    }

    /// Create a Grafana dashboard for the service
    ///
    /// # Errors
    /// Returns an error if the dashboard creation fails due to Grafana API issues
    pub async fn create_dashboard(
        &mut self,
        prometheus_datasource: &str,
        loki_datasource: &str,
    ) -> Result<Option<String>> {
        if let Some(client) = &self.grafana_client {
            // Use the service_id and blueprint_id from the metrics config if available
            let service_id = self.config.metrics.as_ref().map_or(0, |m| m.service_id);
            let blueprint_id = self.config.metrics.as_ref().map_or(0, |m| m.blueprint_id);
            
            let dashboard_url = client
                .create_blueprint_dashboard(service_id, blueprint_id, prometheus_datasource, loki_datasource)
                .await?;
            self.dashboard_url = Some(dashboard_url.clone());
            Ok(Some(dashboard_url))
        } else {
            Ok(None)
        }
    }

    /// Get the dashboard URL if available
    pub fn dashboard_url(&self) -> Option<&str> {
        self.dashboard_url.as_deref()
    }

    /// Get the metrics provider if available
    pub fn metrics_provider(&self) -> Option<Arc<EnhancedMetricsProvider>> {
        self.metrics_service.as_ref().map(|service| service.provider())
    }

    /// Record job execution if metrics service is available
    pub fn record_job_execution(&self, job_id: u64, execution_time: f64) {
        if let Some(service) = &self.metrics_service {
            service.record_job_execution(job_id, execution_time);
        }
    }

    /// Record job error if metrics service is available
    pub fn record_job_error(&self, job_id: u64, error_type: &str) {
        if let Some(service) = &self.metrics_service {
            service.record_job_error(job_id, error_type);
        }
    }

    /// Get the Grafana server URL if available
    pub fn grafana_server_url(&self) -> Option<String> {
        self.grafana_server.as_ref().map(|server| server.url())
    }

    /// Get the Loki server URL if available
    pub fn loki_server_url(&self) -> Option<String> {
        self.loki_server.as_ref().map(|server| server.url())
    }

    /// Get the Prometheus server URL if available
    pub fn prometheus_server_url(&self) -> Option<String> {
        self.prometheus_server.as_ref().map(|server| server.url())
    }

    /// Get the Prometheus registry if available
    pub fn prometheus_registry(&self) -> Option<Arc<prometheus::Registry>> {
        self.metrics_provider().map(|provider| {
            let collector = provider.prometheus_collector();
            Arc::new(collector.registry().clone())
        })
    }

    /// Debug method to check if servers are initialized
    pub fn debug_server_status(&self) {
        info!("Server status:");
        info!("Grafana server: {}", self.grafana_server.is_some());
        info!("Loki server: {}", self.loki_server.is_some());
        info!("Prometheus server: {}", self.prometheus_server.is_some());
        
        if let Some(server) = &self.grafana_server {
            info!("Grafana URL: {}", server.url());
        }
        if let Some(server) = &self.loki_server {
            info!("Loki URL: {}", server.url());
        }
        if let Some(server) = &self.prometheus_server {
            info!("Prometheus URL: {}", server.url());
        }
    }
}

impl<C> Drop for QoSService<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    fn drop(&mut self) {
        // No need to explicitly stop the heartbeat service or metrics service
        // as they don't implement the stop method

        // Stop the server managers
        if let Some(server) = &self.grafana_server {
            info!("Stopping Grafana server...");
            let _ = futures::executor::block_on(server.stop());
        }

        if let Some(server) = &self.loki_server {
            info!("Stopping Loki server...");
            let _ = futures::executor::block_on(server.stop());
        }
        
        if let Some(server) = &self.prometheus_server {
            info!("Stopping Prometheus server...");
            let _ = futures::executor::block_on(server.stop());
        }
    }
}

/// Builder for `QoS` service
pub struct QoSServiceBuilder<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    config: QoSConfig,
    heartbeat_consumer: Option<Arc<C>>,
    otel_config: Option<OpenTelemetryConfig>,
    create_dashboard: bool,
    prometheus_datasource: Option<String>,
    loki_datasource: Option<String>,
}

impl<C> QoSServiceBuilder<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    /// Create a new `QoS` service builder
    pub fn new() -> Self {
        Self {
            config: QoSConfig::default(),
            heartbeat_consumer: None,
            otel_config: None,
            create_dashboard: false,
            prometheus_datasource: None,
            loki_datasource: None,
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
    /// Set the Loki datasource UID
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
        if self.create_dashboard && service.grafana_client.is_some() {
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
