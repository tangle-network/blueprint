use blueprint_core::{error, info};
use std::sync::Arc;

use crate::QoSConfig;
use crate::error::Result;
use crate::heartbeat::{HeartbeatConsumer, HeartbeatService};
use crate::logging::grafana::{CreateDataSourceRequest, LokiJsonData, PrometheusJsonData};
use crate::logging::{GrafanaClient, init_loki_logging};
use crate::metrics::opentelemetry::OpenTelemetryConfig;
use crate::metrics::provider::EnhancedMetricsProvider;
use crate::metrics::service::MetricsService;
use crate::servers::{
    ServerManager, grafana::GrafanaServer, loki::LokiServer, prometheus::PrometheusServer,
};
use tokio::sync::RwLock;

/// Unified `QoS` service that combines heartbeat, metrics, logging, and dashboard functionality
pub struct QoSService<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    /// Heartbeat service
    #[allow(dead_code)]
    heartbeat_service: Option<Arc<HeartbeatService<C>>>, // Changed to Arc

    /// Metrics service
    metrics_service: Option<Arc<MetricsService>>, // Changed to Arc

    /// Grafana client
    grafana_client: Option<Arc<GrafanaClient>>,

    /// Dashboard URL
    dashboard_url: Option<String>,

    /// Grafana server manager
    grafana_server: Option<Arc<GrafanaServer>>,

    /// Loki server manager
    loki_server: Option<Arc<LokiServer>>,

    /// Prometheus server manager
    prometheus_server: Option<Arc<PrometheusServer>>,

    /// Oneshot sender to signal completion when this service is dropped.
    completion_tx: RwLock<Option<tokio::sync::oneshot::Sender<Result<()>>>>,
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
        let _heartbeat_service = config.heartbeat.clone().map(|hc_config| {
            Arc::new(HeartbeatService::new(hc_config, heartbeat_consumer.clone()))
        });

        // Initialize metrics service if configured
        let metrics_service = if let Some(metrics_config) = config.metrics.clone() {
            Some(Arc::new(MetricsService::new(metrics_config)?))
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

        // (around line 88)
        let (grafana_server, loki_server, prometheus_server) = if config.manage_servers {
            // Define instances with new names
            let grafana_server_instance = config
                .grafana_server
                .as_ref()
                .map(|cfg| Arc::new(GrafanaServer::new(cfg.clone())));
            let loki_server_instance = config
                .loki_server
                .as_ref()
                .map(|cfg| Arc::new(LokiServer::new(cfg.clone())));
            let metrics_service_instance = metrics_service.clone();
            let prometheus_registry_for_server: Option<Arc<prometheus::Registry>> =
                metrics_service_instance
                    .as_ref()
                    .map(|ms| ms.provider().prometheus_collector().registry().clone());

            let prometheus_server_instance = config.prometheus_server.as_ref().map(|server_cfg| {
                let registry_to_pass = if !server_cfg.use_docker {
                    prometheus_registry_for_server.clone()
                } else {
                    None
                };
                Arc::new(PrometheusServer::new(server_cfg.clone(), registry_to_pass))
            });
            // Start the servers using the *_instance variables (around line 118 onwards)
            if let Some(server) = &grafana_server_instance {
                // Use grafana_server_instance
                info!("Starting Grafana server...");
                if let Err(e) = server.start().await {
                    error!("Failed to start Grafana server: {}", e); // Non-critical, just log
                } else {
                    info!("Grafana server started successfully: {}", server.url());
                }
            }

            if let Some(server) = &loki_server_instance {
                // Use loki_server_instance
                info!("Starting Loki server...");
                if let Err(e) = server.start().await {
                    error!("Failed to start Loki server: {}", e); // Non-critical, just log
                } else {
                    info!("Loki server started successfully: {}", server.url());
                }
            }

            if let Some(server) = &prometheus_server_instance {
                // Use prometheus_server_instance
                info!("Starting Prometheus server (exposer)...");
                if let Err(e) = server.start().await {
                    error!(
                        "Failed to start critical Prometheus server (exposer): {}",
                        e
                    );
                    return Err(e); // This is critical
                } else {
                    info!(
                        "Prometheus server (exposer) started successfully: {}",
                        server.url()
                    );
                }
            }

            // Return the instances (around line 145)
            (
                grafana_server_instance,
                loki_server_instance,
                prometheus_server_instance,
            )
        } else {
            (None, None, None)
        };
        // Now, the variables from line 88 (grafana_server, loki_server, etc.) are correctly populated
        // and can be used for the struct initialization later.

        // Update Grafana client if we are managing servers
        let grafana_client = if let Some(server) = &grafana_server {
            let mut client_grafana_config = server.client_config();
            client_grafana_config.loki_config = config.loki.clone(); // Populate LokiConfig
            Some(Arc::new(GrafanaClient::new(client_grafana_config)))
        } else {
            config.grafana.as_ref().map(|user_grafana_config| {
                let mut client_grafana_config = user_grafana_config.clone();
                client_grafana_config.loki_config = config.loki.clone(); // Populate LokiConfig
                Arc::new(GrafanaClient::new(client_grafana_config))
            })
        };

        // Initialize MetricsService
        let metrics_service = if let Some(ref mc) = config.metrics {
            Some(Arc::new(MetricsService::new(mc.clone())?))
        } else {
            None
        };

        // Initialize HeartbeatService
        let heartbeat_service = config.heartbeat.clone().map(|hc_config| {
            Arc::new(HeartbeatService::new(hc_config, heartbeat_consumer.clone()))
        });

        Ok(Self {
            heartbeat_service,
            metrics_service,
            grafana_client,
            dashboard_url: None,
            grafana_server,
            loki_server,
            prometheus_server,
            completion_tx: RwLock::new(None),
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
        let _heartbeat_service = config.heartbeat.clone().map(|hc_config| {
            Arc::new(HeartbeatService::new(hc_config, heartbeat_consumer.clone()))
        });

        // Initialize metrics service if configured
        let metrics_service = Some(Arc::new(MetricsService::with_otel_config(
            config.metrics.clone().unwrap_or_default(),
            otel_config,
        )?));

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
            let metrics_service_instance_otel = metrics_service.clone();
            let prometheus_registry_for_server_otel: Option<Arc<prometheus::Registry>> =
                metrics_service_instance_otel
                    .as_ref()
                    .map(|ms| ms.provider().prometheus_collector().registry().clone());

            let (grafana_server, loki_server, prometheus_server) = (
                config
                    .grafana_server
                    .as_ref()
                    .map(|cfg| Arc::new(GrafanaServer::new(cfg.clone()))),
                config
                    .loki_server
                    .as_ref()
                    .map(|cfg| Arc::new(LokiServer::new(cfg.clone()))),
                config.prometheus_server.as_ref().map(|server_cfg| {
                    let registry_to_pass = if !server_cfg.use_docker {
                        prometheus_registry_for_server_otel.clone()
                    } else {
                        None
                    };
                    Arc::new(PrometheusServer::new(server_cfg.clone(), registry_to_pass))
                }),
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
                    error!("Failed to start critical Prometheus server: {}", e);
                    return Err(e);
                } else {
                    info!("Prometheus server started successfully");
                }
            }
            // Return the tuple of server instances
            (grafana_server, loki_server, prometheus_server)
        } else {
            (None, None, None)
        }; // Closes the server management if/else block

        // Initialize Grafana client
        let grafana_client = if let Some(server) = &grafana_server {
            let mut client_grafana_config = server.client_config();
            client_grafana_config.loki_config = config.loki.clone();
            Some(Arc::new(GrafanaClient::new(client_grafana_config)))
        } else {
            config.grafana.as_ref().map(|user_grafana_config| {
                let mut client_grafana_config = user_grafana_config.clone();
                client_grafana_config.loki_config = config.loki.clone();
                Arc::new(GrafanaClient::new(client_grafana_config))
            })
        };

        // METRICS SERVICE IS ALREADY INITIALIZED at the top of this function using the explicit otel_config.
        // The local 'metrics_service' variable (which is an Option<Arc<MetricsService>>) correctly holds this instance.
        // DO NOT RE-INITIALIZE OR OVERWRITE 'metrics_service' here.

        // Initialize HeartbeatService
        let heartbeat_service = config.heartbeat.clone().map(|hc_config| {
            Arc::new(HeartbeatService::new(hc_config, heartbeat_consumer.clone()))
        });

        Ok(Self {
            heartbeat_service,
            metrics_service,
            grafana_client,
            dashboard_url: None,
            grafana_server, // These are Option<Arc<ServerType>> from the manage_servers block
            loki_server,
            prometheus_server,
            completion_tx: RwLock::new(None),
        })
    }

    /// Updates the URL for an existing Prometheus datasource in Grafana
    /// This is particularly useful when switching from direct metrics scraping
    /// to using the scraping Prometheus server
    pub async fn update_prometheus_datasource(
        &self,
        datasource_uid: &str,
        new_url: &str,
    ) -> Result<Option<String>> {
        if let Some(grafana) = &self.grafana_client {
            info!(
                "Updating Prometheus datasource {} to use URL: {}",
                datasource_uid, new_url
            );
            return grafana.update_datasource_url(datasource_uid, new_url).await;
        }
        info!("Grafana not configured, skipping datasource URL update");
        Ok(None)
    }

    /// Create a Grafana dashboard for the service
    ///
    /// # Errors
    /// Returns an error if the dashboard creation fails due to Grafana API issues
    pub async fn create_dashboard(
        &mut self,
        service_id: u64,
        blueprint_id: u64,
        prometheus_datasource_uid: &str, // Renamed for clarity to UID
        loki_datasource_uid: &str,       // Renamed for clarity to UID
    ) -> Result<Option<String>> {
        if let Some(client) = &self.grafana_client {
            let prometheus_ds_name = "Blueprint Prometheus";
            let prometheus_url = self.prometheus_server_url().unwrap_or_else(|| {
                blueprint_core::warn!("Prometheus server URL not available for Grafana data source creation, defaulting to http://localhost:9091");
                "http://localhost:9091".to_string()
            });

            let prometheus_json_data = PrometheusJsonData {
                http_method: "GET".to_string(),
                timeout: Some(30),
                disable_metrics_lookup: false, // Explicitly enable metrics lookup
            };
            let prometheus_request_payload = CreateDataSourceRequest {
                name: prometheus_ds_name.to_string(),
                ds_type: "prometheus".to_string(),
                url: prometheus_url.clone(),
                access: "proxy".to_string(),
                uid: Some(prometheus_datasource_uid.to_string()),
                is_default: Some(true),
                json_data: Some(
                    serde_json::to_value(prometheus_json_data)
                        .map_err(|e| crate::error::Error::Json(e.to_string()))?,
                ),
            };

            match client
                .create_or_update_datasource(prometheus_request_payload)
                .await
            {
                Ok(response) => {
                    info!(
                        "Successfully created/updated Prometheus datasource '{}' in Grafana with UID: {}",
                        response.name, response.datasource.uid
                    );
                }
                Err(e) => {
                    error!(
                        "Failed to create/update Prometheus datasource '{}' in Grafana: {}. Halting dashboard creation.",
                        prometheus_ds_name, e
                    );
                    return Err(e);
                }
            }

            if !loki_datasource_uid.is_empty() {
                let loki_ds_name = "Blueprint Loki";
                let loki_url = self.loki_server_url().or_else(|| {
                    client.config().loki_config.as_ref().map(|lc| lc.url.clone())
                }).unwrap_or_else(|| {
                    blueprint_core::warn!("Loki server URL not available from managed server or Grafana config, defaulting to http://localhost:3100 for Grafana data source creation");
                    "http://localhost:3100".to_string()
                });

                let loki_json_data = LokiJsonData {
                    max_lines: Some(1000),
                };
                let loki_request_payload = CreateDataSourceRequest {
                    name: loki_ds_name.to_string(),
                    ds_type: "loki".to_string(),
                    url: loki_url.clone(),
                    access: "proxy".to_string(),
                    uid: Some(loki_datasource_uid.to_string()),
                    is_default: Some(false),
                    json_data: Some(
                        serde_json::to_value(loki_json_data)
                            .map_err(|e| crate::error::Error::Json(e.to_string()))?,
                    ),
                };

                match client
                    .create_or_update_datasource(loki_request_payload)
                    .await
                {
                    Ok(response) => {
                        info!(
                            "Successfully created/updated Loki datasource '{}' in Grafana with UID: {}",
                            response.name, response.datasource.uid
                        );
                    }
                    Err(e) => {
                        error!(
                            "Failed to create/update Loki datasource in Grafana: {}. Proceeding with dashboard creation attempt.",
                            e
                        );
                    }
                }
            }

            // Now, create the dashboard using the provided datasource UIDs
            match client
                .create_blueprint_dashboard(
                    service_id,   // Now passed as a parameter
                    blueprint_id, // Now passed as a parameter
                    prometheus_datasource_uid,
                    loki_datasource_uid,
                )
                .await
            {
                Ok(url) => {
                    self.dashboard_url = Some(url.clone());
                    info!("Successfully created/updated Grafana dashboard: {}", url);
                    Ok(Some(url))
                }
                Err(e) => {
                    error!("Failed to create Grafana dashboard: {}", e);
                    Err(e)
                }
            }
        } else {
            Ok(None)
        }
    }

    /// Get the metrics provider if available
    #[must_use]
    pub fn metrics_provider(&self) -> Option<Arc<EnhancedMetricsProvider>> {
        self.metrics_service
            .as_ref()
            .map(|arc_service| arc_service.provider())
    }

    /// Get a clone of the OpenTelemetry job executions counter, if the metrics service is available.
    #[must_use]
    pub fn get_otel_job_executions_counter(&self) -> Option<opentelemetry::metrics::Counter<u64>> {
        self.metrics_service
            .as_ref()
            .map(|ms| ms.get_otel_job_executions_counter())
    }

    /// Record job execution if metrics service is available
    pub fn record_job_execution(
        &self,
        job_id: u64,
        execution_time: f64,
        service_id: u64,
        blueprint_id: u64,
    ) {
        if let Some(service) = &self.metrics_service {
            service.record_job_execution(job_id, execution_time, service_id, blueprint_id);
        }
    }

    /// Record job error if metrics service is available
    pub fn record_job_error(&self, job_id: u64, error_type: &str) {
        if let Some(service) = &self.metrics_service {
            service.record_job_error(job_id, error_type);
        }
    }

    /// Get the Grafana server URL if available
    #[must_use]
    pub fn grafana_server_url(&self) -> Option<String> {
        self.grafana_server.as_ref().map(|server| server.url())
    }

    /// Get the Loki server URL if available
    #[must_use]
    pub fn loki_server_url(&self) -> Option<String> {
        self.loki_server.as_ref().map(|server| server.url())
    }

    /// Get the Grafana client if available
    #[must_use]
    pub fn grafana_client(&self) -> Option<&Arc<GrafanaClient>> {
        self.grafana_client.as_ref()
    }

    /// Get the Prometheus server URL if available
    #[must_use]
    pub fn prometheus_server_url(&self) -> Option<String> {
        self.prometheus_server.as_ref().map(|server| server.url())
    }

    /// Get the Prometheus registry if available
    #[must_use]
    pub fn prometheus_registry(&self) -> Option<Arc<prometheus::Registry>> {
        self.metrics_provider().map(|provider| {
            let collector = provider.prometheus_collector();
            collector.registry().clone()
        })
    }

    /// Get the heartbeat service if available
    #[must_use]
    pub fn heartbeat_service(&self) -> Option<&HeartbeatService<C>> {
        self.heartbeat_service.as_ref().map(Arc::as_ref)
    }

    /// Debug method to check if servers are initialized
    pub fn debug_server_status(&self) {
        info!("Server status:");
        info!("Grafana server: {}", self.grafana_server.is_some());
        info!("Loki server: {}", self.loki_server.is_some());
        info!(
            "Prometheus server (exposer): {}",
            self.prometheus_server.is_some()
        );
        if let Some(server) = &self.grafana_server {
            info!("Grafana URL: {}", server.url());
        }
        if let Some(server) = &self.loki_server {
            info!("Loki URL: {}", server.url());
        }
        if let Some(server) = &self.prometheus_server {
            info!("Prometheus (exposer) URL: {}", server.url());
        }
    }

    /// Sets the oneshot sender that will be used to signal completion when this QoSService is dropped.
    /// This is typically called by an adapter (e.g., QoSServiceAdapter) when integrating QoSService
    /// into a runner framework that expects a completion signal.
    pub async fn set_completion_sender(&self, tx: tokio::sync::oneshot::Sender<Result<()>>) {
        let mut guard = self.completion_tx.write().await;
        if guard.is_some() {
            blueprint_core::warn!(
                "[QoSService::set_completion_sender] Completion sender was already set. Overwriting."
            );
        }
        *guard = Some(tx);
    }
} // Closing brace for impl<C> QoSService<C>

impl<C> Drop for QoSService<C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    fn drop(&mut self) {
        // use std::backtrace::Backtrace;

        // Stop the server managers
        if let Some(server) = &self.grafana_server {
            info!("[Drop] About to stop Grafana server...");
            let _ = futures::executor::block_on(server.stop());
            info!("[Drop] Grafana server stop returned.");
        }

        if let Some(server) = &self.loki_server {
            info!("[Drop] About to stop Loki server...");
            let _ = futures::executor::block_on(server.stop());
            info!("[Drop] Loki server stop returned.");
        }

        if let Some(server) = &self.prometheus_server {
            info!("[Drop] About to stop Prometheus server (exposer)...");
            let _ = futures::executor::block_on(server.stop());
            info!("[Drop] Prometheus server (exposer) stop returned.");
        }

        // Signal completion if a sender was provided
        let mut sender_option_guard = self.completion_tx.blocking_write();

        if let Some(tx) = sender_option_guard.take() {
            if tx.send(Ok(())).is_err() {
                info!(
                    "[QoSService::Drop] Completion signal receiver was already dropped while sending Ok."
                );
            } else {
                info!("[QoSService::Drop] Sent Ok(()) completion signal.");
            }
        } else {
            info!("[QoSService::Drop] No completion sender was set, so no explicit signal sent.");
        }
        info!("[QoSService::Drop] Finished dropping QoSService<C>.");
    }
}
