use std::sync::Arc;
use blueprint_core::{error as core_error, info};
use log::error;
use tokio::sync::{oneshot, RwLock};
use crate::{
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
        grafana::GrafanaServer, loki::LokiServer, prometheus::PrometheusServer, ServerManager,
    },
    QoSConfig,
};

/// Unified `QoS` service that combines heartbeat, metrics, logging, and dashboard functionality.
pub struct QoSService<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    #[allow(dead_code)] // TODO: Used for graceful shutdown
    heartbeat_service: Option<Arc<HeartbeatService<C>>>,
    metrics_service: Option<Arc<MetricsService>>,
    grafana_client: Option<Arc<GrafanaClient>>,
    dashboard_url: Option<String>,
    #[allow(dead_code)] // TODO: Used for graceful shutdown
    grafana_server: Option<Arc<GrafanaServer>>,
    loki_server: Option<Arc<LokiServer>>,
    prometheus_server: Option<Arc<PrometheusServer>>,
    completion_tx: Arc<RwLock<Option<oneshot::Sender<Result<()>>>>>,
    completion_rx: RwLock<Option<tokio::sync::oneshot::Receiver<Result<()>>>>,
}

impl<C> QoSService<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    pub fn heartbeat_service(&self) -> Option<&Arc<HeartbeatService<C>>> {
        self.heartbeat_service.as_ref()
    }

    /// Sets the completion sender for this QoS service.
    /// This sender is used to signal that the service (or the component it's monitoring)
    /// has completed its primary operation, often used in testing or graceful shutdown scenarios.
    pub async fn set_completion_sender(&self, sender: oneshot::Sender<Result<()>>) {
        let mut guard = self.completion_tx.write().await;
        if guard.is_some() {
            core_error!("Completion sender already set for QoSService, overwriting.");
        }
        *guard = Some(sender);
    }

    /// Common initialization logic for `QoSService`.
    async fn initialize(
        config: QoSConfig,
        heartbeat_consumer: Arc<C>,
        otel_config: Option<OpenTelemetryConfig>,
    ) -> Result<Self> {
        let heartbeat_service = config
            .heartbeat
            .clone()
            .map(|hc| Arc::new(HeartbeatService::new(hc, heartbeat_consumer.clone())));

        let metrics_service = match (config.metrics.clone(), otel_config) {
            (Some(mc), Some(oc)) => Some(Arc::new(MetricsService::with_otel_config(mc, oc)?)),
            (Some(mc), None) => Some(Arc::new(MetricsService::new(mc)?)),
            (None, _) => None,
        };

        if let Some(_ms) = &metrics_service {
            info!("Metrics service is Some, attempting to start collection.");
            // ms.provider().clone().start_collection().await?; // Removed: PrometheusServer manager handles this
        }

        if let Some(loki_config) = &config.loki {
            if let Err(e) = init_loki_logging(loki_config.clone()) {
                core_error!("Failed to initialize Loki logging: {}", e);
            } else {
                info!("Initialized Loki logging");
            }
        }

        let (grafana_server, loki_server, prometheus_server) = if config.manage_servers {
            let mut grafana = config
                .grafana_server
                .as_ref()
                .map(|c| GrafanaServer::new(c.clone()));

            let mut loki = config
                .loki_server
                .as_ref()
                .map(|c| LokiServer::new(c.clone()));

            let mut prometheus = config.prometheus_server.as_ref().map(|c| {
                PrometheusServer::new(c.clone(), Some(metrics_service.as_ref().unwrap().provider().shared_registry().clone()), metrics_service.as_ref().unwrap().provider().clone())
            });

            if let Some(s) = &mut grafana {
                info!("Starting Grafana server...");
                if let Err(e) = s.start(config.docker_network.as_deref()).await {
                    error!("Failed to start Grafana server: {}", e);
                } else {
                    info!("Grafana server started successfully: {}", s.url());
                }
            }

            if let Some(s) = &mut loki {
                info!("Starting Loki server...");
                if let Err(e) = s.start(config.docker_network.as_deref()).await {
                    error!("Failed to start Loki server: {}", e);
                } else {
                    info!("Loki server started successfully: {}", s.url());
                }
            }

            if let Some(s) = &mut prometheus {
                info!("Starting Prometheus server...");
                if let Err(e) = s.start(config.docker_network.as_deref()).await {
                    core_error!("Failed to start critical Prometheus server: {}", e);
                    return Err(e); // Critical failure
                } else {
                    info!("Prometheus server started successfully: {}", s.url());
                }
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

        let (tx, rx): (oneshot::Sender<Result<()>>, oneshot::Receiver<Result<()>>) = oneshot::channel();
        Ok(Self {
            heartbeat_service,
            metrics_service,
            grafana_client,
            dashboard_url: None,
            grafana_server,
            loki_server,
            prometheus_server,
            completion_tx: Arc::new(RwLock::new(Some(tx))),
            completion_rx: RwLock::new(Some(rx)),
        })
    }

    pub async fn new(config: QoSConfig, heartbeat_consumer: Arc<C>) -> Result<Self> {
        Self::initialize(config, heartbeat_consumer, None).await
    }

    pub async fn with_otel_config(
        config: QoSConfig,
        heartbeat_consumer: Arc<C>,
        otel_config: OpenTelemetryConfig,
    ) -> Result<Self> {
        Self::initialize(config, heartbeat_consumer, Some(otel_config)).await
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
        if self.metrics_service.as_ref().map(|ms| ms.provider()).is_some() {
            info!("Metrics Service: Configured (instance present)");
        } else {
            info!("Metrics Service: Not configured");
        }
        info!("-------------------------");
    }

    pub fn grafana_client(&self) -> Option<Arc<GrafanaClient>> {
        self.grafana_client.clone()
    }

    pub fn grafana_server_url(&self) -> Option<String> {
        self.grafana_server.as_ref().map(|server| server.url())
    }

    /// Get the Loki server URL, if configured and running.
    #[must_use]
    pub fn loki_server_url(&self) -> Option<String> {
        self.loki_server.as_ref().map(|s| s.url())
    }

    pub fn provider(&self) -> Option<Arc<EnhancedMetricsProvider>> {
        self.metrics_service.as_ref().map(|s| s.provider())
    }

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
            json_data: Some(serde_json::to_value(LokiJsonData { max_lines: Some(1000) })?),
        };
        client.create_or_update_datasource(loki_ds).await?;

        // Determine the correct Prometheus URL for Grafana to use.
        // Priority:
        // 1. Explicit URL from GrafanaConfig (via GrafanaClient).
        // 2. URL from the managed Prometheus server instance.
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
        info!("Successfully provisioned Prometheus datasource '{}' with UID '{}'. Checking health...", created_prometheus_ds.name, created_prometheus_ds.datasource.uid);
        match client.check_datasource_health(&created_prometheus_ds.datasource.uid).await {
            Ok(health) if health.status.to_lowercase() == "ok" => {
                info!("Prometheus datasource '{}' (UID: {}) is healthy: {}", created_prometheus_ds.name, created_prometheus_ds.datasource.uid, health.message);
            }
            Ok(health) => {
                core_error!("Prometheus datasource '{}' (UID: {}) is not healthy: Status: {}, Message: {}", created_prometheus_ds.name, created_prometheus_ds.datasource.uid, health.status, health.message);
                return Err(qos_error::Error::GrafanaApi(format!("Datasource {} (UID: {}) reported unhealthy: {}", created_prometheus_ds.name, created_prometheus_ds.datasource.uid, health.message)));
            }
            Err(e) => {
                core_error!("Failed to check health for Prometheus datasource '{}' (UID: {}): {}", created_prometheus_ds.name, created_prometheus_ds.datasource.uid, e);
                return Err(e.into());
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

    pub fn record_job_execution(
        &self,
        job_id: u64,
        execution_time: f64,
        service_id: u64,
        blueprint_id: u64,
    ) {
        if let Some(service) = self.metrics_service.as_ref() {
            service
                .provider()
                .record_job_execution(job_id, execution_time, service_id, blueprint_id);
        }
    }

    pub fn record_job_error(&self, job_id: u64, error_type: &str) {
        if let Some(service) = self.metrics_service.as_ref() {
            service.provider().record_job_error(job_id, error_type);
        }
    }

    pub async fn wait_for_completion(&self) -> Result<()> {
        let rx_option = {
            let mut guard = self.completion_rx.write().await; // tokio::sync::RwLock uses .await
            guard.take()
        };

        if let Some(rx) = rx_option {
            match rx.await {
                Ok(inner_result) => inner_result, // inner_result is Result<(), crate::error::Error>
                Err(_recv_error) => {
                    // recv_error is tokio::sync::oneshot::error::RecvError
                    Err(qos_error::Error::Other(format!("Completion signal receiver dropped before completion")))
                }
            }
        } else {
            Err(qos_error::Error::Other("wait_for_completion can only be called once".to_string()))
        }
    }

    pub async fn shutdown(&self) -> Result<()> {
        info!("QoSService shutting down...");
        // TODO: Implement graceful shutdown for servers
        info!("QoSService shutdown complete.");
        Ok(())
    }
}

impl<C> Drop for QoSService<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    fn drop(&mut self) {
        if let Some(metrics_service) = self.metrics_service.as_ref() {
            if let Err(e) = metrics_service.provider().force_flush_otel_metrics() {
                core_error!("Failed to flush OpenTelemetry metrics on drop: {}", e);
            }
        }
        match self.completion_tx.try_write() {
            Ok(mut guard) => {
                if let Some(tx) = guard.take() {
                    if tx.send(Ok(())).is_err() {
                        // Receiver was dropped, or channel was closed.
                        // This is not necessarily an error during drop, as the service might have completed normally.
                        info!("Attempted to send completion signal on drop, but receiver was already gone.");
                    }
                }
            }
            Err(_) => {
                core_error!("Failed to acquire lock for completion_tx during drop (lock was contended). Signal not sent.");
            }
        }
    }
}
