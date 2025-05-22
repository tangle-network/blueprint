use std::sync::Arc;
use tracing::{error, info};

use crate::QoSConfig;
use crate::error::Result;
use crate::heartbeat::{HeartbeatConfig, HeartbeatConsumer, HeartbeatService};
use crate::logging::{GrafanaClient, GrafanaConfig, LokiConfig, init_loki_logging};
use crate::metrics::opentelemetry::OpenTelemetryConfig;
use crate::metrics::provider::EnhancedMetricsProvider;
use crate::metrics::service::MetricsService;
use crate::metrics::types::MetricsConfig;

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
        let grafana_client = config
            .grafana
            .as_ref()
            .map(|grafana_config| Arc::new(GrafanaClient::new(grafana_config.clone())));

        let mut service = Self {
            heartbeat_service,
            metrics_service,
            grafana_client,
            config,
            dashboard_url: None,
        };

        // Start the heartbeat service if configured
        if let Some(heartbeat_service) = &service.heartbeat_service {
            if let Err(e) = heartbeat_service.start_heartbeat().await {
                error!("Failed to start heartbeat service: {}", e);
            } else {
                info!("Started heartbeat service");
            }
        }

        Ok(service)
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
            Some(MetricsService::with_otel_config(
                metrics_config,
                otel_config,
            )?)
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
        let grafana_client = config
            .grafana
            .as_ref()
            .map(|grafana_config| Arc::new(GrafanaClient::new(grafana_config.clone())));

        let mut service = Self {
            heartbeat_service,
            metrics_service,
            grafana_client,
            config,
            dashboard_url: None,
        };

        // Start the heartbeat service if configured
        if let Some(heartbeat_service) = &service.heartbeat_service {
            if let Err(e) = heartbeat_service.start_heartbeat().await {
                error!("Failed to start heartbeat service: {}", e);
            } else {
                info!("Started heartbeat service");
            }
        }

        Ok(service)
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
        // Check if Grafana client is available
        if let Some(client) = &self.grafana_client {
            // Get service and blueprint IDs from metrics config
            let (service_id, blueprint_id) = if let Some(metrics_config) = &self.config.metrics {
                (metrics_config.service_id, metrics_config.blueprint_id)
            } else if let Some(heartbeat_config) = &self.config.heartbeat {
                (heartbeat_config.service_id, heartbeat_config.blueprint_id)
            } else {
                (0, 0)
            };

            // Create dashboard
            match client
                .create_blueprint_dashboard(
                    service_id,
                    blueprint_id,
                    prometheus_datasource,
                    loki_datasource,
                )
                .await
            {
                Ok(url) => {
                    self.dashboard_url = Some(url.clone());
                    info!("Created Grafana dashboard: {}", url);
                    Ok(Some(url))
                }
                Err(e) => {
                    error!("Failed to create Grafana dashboard: {}", e);
                    Err(e)
                }
            }
        } else {
            info!("Grafana client not configured, skipping dashboard creation");
            Ok(None)
        }
    }

    /// Get the dashboard URL if available
    #[must_use]
    pub fn dashboard_url(&self) -> Option<&str> {
        self.dashboard_url.as_deref()
    }

    /// Get the metrics provider if available
    #[must_use]
    pub fn metrics_provider(&self) -> Option<Arc<EnhancedMetricsProvider>> {
        self.metrics_service
            .as_ref()
            .map(super::metrics::service::MetricsService::provider)
    }

    /// Record job execution if metrics service is available
    pub fn record_job_execution(&self, job_id: u64, execution_time: f64) {
        if let Some(metrics_service) = &self.metrics_service {
            metrics_service.record_job_execution(job_id, execution_time);
        }
    }

    /// Record job error if metrics service is available
    pub fn record_job_error(&self, job_id: u64, error_type: &str) {
        if let Some(metrics_service) = &self.metrics_service {
            metrics_service.record_job_error(job_id, error_type);
        }
    }
}

/// Builder for `QoS` service
pub struct QoSServiceBuilder<C> {
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
