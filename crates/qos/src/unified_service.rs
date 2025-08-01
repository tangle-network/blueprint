use crate::{
    QoSConfig,
    error::{self as qos_error, Result},
    heartbeat::{HeartbeatConsumer, HeartbeatService},
    logging::grafana::{
        CreateDataSourceRequest, Dashboard, GrafanaClient, LokiJsonData, PrometheusJsonData,
    },
    logging::loki::init_loki_logging,
    metrics::{
        opentelemetry::OpenTelemetryConfig, provider::EnhancedMetricsProvider,
        service::MetricsService,
    },
    servers::{
        ServerManager, grafana::GrafanaServer, loki::LokiServer, prometheus::PrometheusServer,
    },
};
use blueprint_core::{error, info};
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

/// Unified Quality of Service (`QoS`) service that integrates heartbeat monitoring, metrics collection,
/// logging, and dashboard visualization into a single cohesive system.
///
/// `QoSService` orchestrates multiple components:
/// - Heartbeat service for liveness monitoring and reporting
/// - Metrics collection and exposure via Prometheus
/// - Log aggregation via Loki
/// - Dashboard visualization via Grafana
/// - Server management for the above components
///
/// It can be configured to automatically manage server instances or connect to externally managed services.
pub struct QoSService<C: HeartbeatConsumer + Send + Sync + 'static> {
    heartbeat_service: Option<Arc<HeartbeatService<C>>>,
    metrics_service: Option<Arc<MetricsService>>,
    grafana_client: Option<Arc<GrafanaClient>>,
    dashboard_url: Option<String>,
    grafana_server: Option<Arc<GrafanaServer>>,
    loki_server: Option<Arc<LokiServer>>,
    prometheus_server: Option<Arc<PrometheusServer>>,
    completion_tx: Arc<Mutex<Option<oneshot::Sender<Result<()>>>>>,
    completion_rx: Mutex<Option<tokio::sync::oneshot::Receiver<Result<()>>>>,
}

impl<C: HeartbeatConsumer + Send + Sync + 'static> QoSService<C> {
    /// Returns a reference to the heartbeat service if configured.
    ///
    /// The `HeartbeatService` is responsible for sending periodic liveness updates
    /// to the blockchain or other monitoring systems.
    pub fn heartbeat_service(&self) -> Option<&Arc<HeartbeatService<C>>> {
        self.heartbeat_service.as_ref()
    }

    /// Sets the completion sender for this `QoS` service.
    pub async fn set_completion_sender(&self, sender: oneshot::Sender<Result<()>>) {
        let mut guard = self.completion_tx.lock().await;
        if guard.is_some() {
            error!("Completion sender already set for QoSService, overwriting.");
        }
        *guard = Some(sender);
    }

    /// Common initialization logic for `QoSService`.
    async fn initialize(
        config: QoSConfig,
        heartbeat_consumer: Arc<C>,
        ws_rpc_endpoint: String,
        keystore_uri: String,
        otel_config: Option<OpenTelemetryConfig>,
    ) -> Result<Self> {
        let heartbeat_service = config.heartbeat.clone().map(|hc| {
            let ws_rpc = ws_rpc_endpoint.clone();
            Arc::new(HeartbeatService::new(
                hc.clone(),
                heartbeat_consumer.clone(),
                ws_rpc,
                keystore_uri.clone(),
                hc.service_id,
                hc.blueprint_id,
            ))
        });

        let metrics_service = match (config.metrics.clone(), otel_config) {
            (Some(mc), Some(oc)) => Some(Arc::new(MetricsService::with_otel_config(mc, &oc)?)),
            (Some(mc), None) => Some(Arc::new(MetricsService::new(mc)?)),
            (None, _) => None,
        };

        if let Some(ms) = &metrics_service {
            info!("Metrics service is Some, attempting to start collection.");
            ms.provider().clone().start_collection().await?;
        }

        if let Some(loki_config) = &config.loki {
            if let Err(e) = init_loki_logging(loki_config.clone()) {
                error!("Failed to initialize Loki logging: {}", e);
            } else {
                info!("Initialized Loki logging");
            }
        }

        let bind_ip = config.docker_bind_ip.clone();
        let (grafana_server, loki_server, prometheus_server) = if config.manage_servers {
            let grafana = config
                .grafana_server
                .as_ref()
                .map(|c| GrafanaServer::new(c.clone()))
                .transpose()?;

            let loki = config
                .loki_server
                .as_ref()
                .map(|c| LokiServer::new(c.clone()))
                .transpose()?;

            let prometheus = config.prometheus_server.as_ref().map(|c| {
                PrometheusServer::new(
                    c.clone(),
                    Some(
                        metrics_service
                            .as_ref()
                            .ok_or_else(|| qos_error::Error::Generic("Metrics service is required for Prometheus but not configured".to_string()))?
                            .provider()
                            .shared_registry()
                            .clone(),
                    ),
                    metrics_service.as_ref().ok_or_else(|| qos_error::Error::Generic("Metrics service is required for Prometheus but not configured".to_string()))?.provider().clone(),
                )
            }).transpose()?;

            if let Some(s) = &grafana {
                info!("Starting Grafana server...");
                s.start(config.docker_network.as_deref(), bind_ip.clone())
                    .await
                    .map_err(|e| {
                        error!("Failed to start Grafana server: {}", e);
                        e
                    })?;
                info!("Grafana server started successfully: {}", s.url());
            }

            if let Some(s) = &loki {
                info!("Starting Loki server...");
                s.start(config.docker_network.as_deref(), bind_ip.clone())
                    .await
                    .map_err(|e| {
                        error!("Failed to start Loki server: {}", e);
                        e
                    })?;
                info!("Loki server started successfully: {}", s.url());
            }

            if let Some(s) = &prometheus {
                info!("Starting Prometheus server...");
                if let Err(e) = s
                    .start(config.docker_network.as_deref(), bind_ip.clone())
                    .await
                {
                    error!("Failed to start critical Prometheus server: {}", e);
                    return Err(e);
                }
                info!("Prometheus server started successfully: {}", s.url());
            }

            (
                grafana.map(Arc::new),
                loki.map(Arc::new),
                prometheus.map(Arc::new),
            )
        } else {
            (None, None, None)
        };

        let grafana_client = match &grafana_server {
            Some(server) => {
                let mut client_config = server.client_config();
                client_config.loki_config = config.loki.clone();
                Some(Arc::new(GrafanaClient::new(client_config)))
            }
            None => config.grafana.as_ref().map(|user_config| {
                let mut client_config = user_config.clone();
                client_config.loki_config = config.loki.clone();
                Arc::new(GrafanaClient::new(client_config))
            }),
        };

        let (tx, rx): (oneshot::Sender<Result<()>>, oneshot::Receiver<Result<()>>) =
            oneshot::channel();
        Ok(Self {
            heartbeat_service,
            metrics_service,
            grafana_client,
            dashboard_url: None,
            grafana_server,
            loki_server,
            prometheus_server,
            completion_tx: Arc::new(Mutex::new(Some(tx))),
            completion_rx: Mutex::new(Some(rx)),
        })
    }

    /// # Errors
    ///
    /// Returns an error if initialization of any underlying service fails.
    pub async fn new(
        config: QoSConfig,
        heartbeat_consumer: Arc<C>,
        ws_rpc_endpoint: String,
        keystore_uri: String,
    ) -> Result<Self> {
        Self::initialize(
            config,
            heartbeat_consumer,
            ws_rpc_endpoint,
            keystore_uri,
            None,
        )
        .await
    }

    /// # Errors
    ///
    /// Returns an error if initialization of any underlying service fails.
    pub async fn with_otel_config(
        config: QoSConfig,
        heartbeat_consumer: Arc<C>,
        ws_rpc_endpoint: String,
        keystore_uri: String,
        otel_config: OpenTelemetryConfig,
    ) -> Result<Self> {
        Self::initialize(
            config,
            heartbeat_consumer,
            ws_rpc_endpoint,
            keystore_uri,
            Some(otel_config),
        )
        .await
    }

    pub fn debug_server_status(&self) {
        info!("--- QoS Server Status ---");
        if self.grafana_server.is_some() {
            info!("Grafana Server: Configured (instance present)");
        } else {
            info!("Grafana Server: Not configured");
        }
        if self.loki_server.is_some() {
            info!("Loki Server: Configured (instance present)");
        } else {
            info!("Loki Server: Not configured");
        }
        if self.prometheus_server.is_some() {
            info!("Prometheus Server: Configured (instance present)");
        } else {
            info!("Prometheus Server: Not configured");
        }
        if self.grafana_client.is_some() {
            info!("Grafana Client: Configured (instance present)");
        } else {
            info!("Grafana Client: Not configured");
        }
        if self
            .metrics_service
            .as_ref()
            .map(|ms| ms.provider())
            .is_some()
        {
            info!("Metrics Service: Configured (instance present)");
        } else {
            info!("Metrics Service: Not configured");
        }
        info!("-------------------------");
    }

    /// Returns a reference to the Grafana API client if configured.
    ///
    /// The Grafana client can be used to programmatically create or update dashboards,
    /// manage data sources, and configure alerts.
    pub fn grafana_client(&self) -> Option<Arc<GrafanaClient>> {
        self.grafana_client.clone()
    }

    /// Returns the URL of the Grafana server if running.
    ///
    /// This URL can be used to access the Grafana web interface for viewing dashboards
    /// and visualizations.
    pub fn grafana_server_url(&self) -> Option<String> {
        self.grafana_server.as_ref().map(|server| server.url())
    }

    /// Returns the URL of the Loki server if configured and running.
    ///
    /// This URL can be used to configure log shipping or to query logs directly
    /// via the Loki API.
    #[must_use]
    pub fn loki_server_url(&self) -> Option<String> {
        self.loki_server.as_ref().map(|s| s.url())
    }

    /// Returns a reference to the metrics provider if configured.
    ///
    /// The `EnhancedMetricsProvider` collects and aggregates system and application metrics
    /// that can be exposed via Prometheus or queried programmatically.
    pub fn provider(&self) -> Option<Arc<EnhancedMetricsProvider>> {
        self.metrics_service.as_ref().map(|s| s.provider())
    }

    /// Creates a Grafana dashboard for visualizing metrics from the blueprint.
    ///
    /// This method:
    /// 1. Creates or updates required data sources (Prometheus and Loki)
    /// 2. Creates a dashboard with panels for system metrics, job execution statistics,
    ///    and log visualization
    /// 3. Sets up appropriate refresh intervals and time ranges
    ///
    /// # Parameters
    /// * `blueprint_name` - The name of the blueprint to use in dashboard titles
    ///
    /// # Errors
    /// Returns an error if:
    /// - Creating or updating data sources fails
    /// - Dashboard creation fails
    /// - The Grafana client is not configured
    pub async fn create_dashboard(&mut self, blueprint_name: &str) -> Result<()> {
        let client = self.grafana_client.as_ref().ok_or(qos_error::Error::Other(
            "Grafana client not configured".to_string(),
        ))?;

        let loki_ds = CreateDataSourceRequest {
            name: "Loki".to_string(),
            ds_type: "loki".to_string(),
            uid: Some("loki-blueprint".to_string()),
            url: self
                .loki_server
                .as_ref()
                .map_or_else(|| "http://loki:3100".to_string(), |s| s.url()),
            access: "proxy".to_string(),
            is_default: Some(false),
            json_data: Some(serde_json::to_value(LokiJsonData {
                max_lines: Some(1000),
            })?),
        };
        client.create_or_update_datasource(loki_ds).await?;

        let prometheus_url = self
            .grafana_client
            .as_ref()
            .and_then(|gc| gc.prometheus_datasource_url())
            .cloned()
            .or_else(|| self.prometheus_server.as_ref().map(|s| s.url()))
            .ok_or_else(|| {
                qos_error::Error::Other(
                    "Prometheus datasource URL is not configured and no managed server is available."
                        .to_string(),
                )
            })?;

        let prometheus_ds = CreateDataSourceRequest {
            name: "Prometheus".to_string(),
            ds_type: "prometheus".to_string(),
            uid: Some("prometheus_blueprint_default".to_string()),
            url: prometheus_url,
            access: "proxy".to_string(),
            is_default: Some(true),
            json_data: Some(serde_json::to_value(PrometheusJsonData {
                http_method: "GET".to_string(),
                timeout: 30,
            })?),
        };
        let created_prometheus_ds = client.create_or_update_datasource(prometheus_ds).await?;
        info!(
            "Successfully provisioned Prometheus datasource '{}' with UID '{}'. Checking health...",
            created_prometheus_ds.name, created_prometheus_ds.datasource.uid
        );
        match client
            .check_datasource_health(&created_prometheus_ds.datasource.uid)
            .await
        {
            Ok(health) if health.status.to_lowercase() == "ok" => {
                info!(
                    "Prometheus datasource '{}' (UID: {}) is healthy: {}",
                    created_prometheus_ds.name,
                    created_prometheus_ds.datasource.uid,
                    health.message
                );
            }
            Ok(health) => {
                error!(
                    "Prometheus datasource '{}' (UID: {}) is not healthy: Status: {}, Message: {}",
                    created_prometheus_ds.name,
                    created_prometheus_ds.datasource.uid,
                    health.status,
                    health.message
                );
                return Err(qos_error::Error::GrafanaApi(format!(
                    "Datasource {} (UID: {}) reported unhealthy: {}",
                    created_prometheus_ds.name,
                    created_prometheus_ds.datasource.uid,
                    health.message
                )));
            }
            Err(e) => {
                error!(
                    "Failed to check health for Prometheus datasource '{}' (UID: {}): {}",
                    created_prometheus_ds.name, created_prometheus_ds.datasource.uid, e
                );
                return Err(e);
            }
        }

        const DASHBOARD_TEMPLATE: &str = include_str!("../config/grafana_dashboard.json");
        let mut dashboard: Dashboard = serde_json::from_str(DASHBOARD_TEMPLATE)?;
        dashboard.title = format!("{} Dashboard", blueprint_name);

        let dashboard_url = client
            .create_dashboard(dashboard, None, "Provisioning Blueprint Dashboard")
            .await?;
        self.dashboard_url = Some(dashboard_url);

        Ok(())
    }

    /// Records metrics about a job execution for monitoring and visualization.
    ///
    /// This method tracks job execution frequency and performance metrics, which are
    /// exposed via Prometheus and can be visualized in Grafana dashboards.
    ///
    /// # Parameters
    /// * `job_id` - Unique identifier of the executed job
    /// * `execution_time` - Time taken to execute the job in seconds
    /// * `service_id` - ID of the service that executed the job
    /// * `blueprint_id` - ID of the blueprint that contains the job
    pub fn record_job_execution(
        &self,
        job_id: u64,
        execution_time: f64,
        service_id: u64,
        blueprint_id: u64,
    ) {
        if let Some(service) = self.metrics_service.as_ref() {
            service.provider().record_job_execution(
                job_id,
                execution_time,
                service_id,
                blueprint_id,
            );
        }
    }

    /// Records metrics about job execution errors for monitoring and alerting.
    ///
    /// This method tracks job failures by type, which can be used for alerting
    /// and diagnostic purposes in Grafana dashboards.
    ///
    /// # Parameters
    /// * `job_id` - Unique identifier of the job that encountered an error
    /// * `error_type` - Classification or description of the error that occurred
    pub fn record_job_error(&self, job_id: u64, error_type: &str) {
        if let Some(service) = self.metrics_service.as_ref() {
            service.provider().record_job_error(job_id, error_type);
        }
    }

    /// Waits for the `QoS` service to complete its operation.
    ///
    /// This method blocks until a completion signal is received, which typically
    /// happens when the service is being shut down gracefully. It's useful for
    /// coordinating shutdown of the `QoS` service with the rest of the application.
    ///
    /// # Errors
    /// Returns an error if the completion signal receiver is dropped prematurely,
    /// indicating an unexpected termination of the service.
    pub async fn wait_for_completion(&self) -> Result<()> {
        let rx_option = {
            let mut guard = self.completion_rx.lock().await;
            guard.take()
        };

        if let Some(rx) = rx_option {
            match rx.await {
                Ok(inner_result) => inner_result,
                Err(_recv_error) => Err(qos_error::Error::Other(
                    "Completion signal receiver dropped before completion".to_string(),
                )),
            }
        } else {
            Err(qos_error::Error::Other(
                "wait_for_completion can only be called once".to_string(),
            ))
        }
    }

    /// Initiates a graceful shutdown of the `QoS` service and all managed components.
    ///
    /// This method stops all server instances (Grafana, Prometheus, Loki) if they
    /// were started by this service, and signals completion to any waiting tasks.
    ///
    /// # Errors
    /// This function is designed to return errors from shutdown operations,
    /// though the current implementation always returns Ok(()).
    pub fn shutdown(&self) -> Result<()> {
        info!("QoSService shutting down...");
        info!("QoSService shutdown complete.");
        Ok(())
    }
}

impl<C: HeartbeatConsumer + Send + Sync + 'static> Drop for QoSService<C> {
    fn drop(&mut self) {
        let flush_result = self.metrics_service.as_ref().map_or(Ok(()), |ms| {
            ms.provider().force_flush_otel_metrics().map_err(|e| {
                error!("Failed to flush OpenTelemetry metrics on drop: {}", e);
                qos_error::Error::Metrics(format!("OpenTelemetry flush failed on drop: {}", e))
            })
        });

        match self.completion_tx.try_lock() {
            Ok(mut guard) => {
                if let Some(tx) = guard.take() {
                    if tx.send(flush_result).is_err() {
                        info!(
                            "Attempted to send completion signal on drop, but receiver was already gone."
                        );
                    }
                }
            }
            Err(_) => {
                error!(
                    "Failed to acquire lock for completion_tx during drop (lock was contended). Signal not sent."
                );
            }
        }
    }
}
