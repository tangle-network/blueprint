//! Remote instance monitoring for QoS
//!
//! This module extends the QoS system to monitor remote Blueprint instances
//! deployed on cloud providers through the blueprint-remote-providers system.

use crate::error::Error;
use crate::metrics::MetricsProvider;
use crate::metrics::types::{BlueprintMetrics, BlueprintStatus, SystemMetrics};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(feature = "remote")]
use blueprint_remote_providers::observability::MetricsCollector as RemoteMetricsCollector;

/// Remote instance metrics collector
pub struct RemoteMetricsProvider {
    #[cfg(feature = "remote")]
    remote_collector: Arc<RemoteMetricsCollector>,
    /// Remote endpoints for connecting to QoS gRPC services
    remote_endpoints: Arc<RwLock<HashMap<String, String>>>, // instance_id -> grpc_endpoint
    metrics_cache: Arc<RwLock<HashMap<u64, SystemMetrics>>>,
    blueprint_metrics: Arc<RwLock<BlueprintMetrics>>,
    status: Arc<RwLock<BlueprintStatus>>,
    history_system: Arc<RwLock<Vec<SystemMetrics>>>,
    history_blueprint: Arc<RwLock<Vec<BlueprintMetrics>>>,
    max_history: usize,
}

impl RemoteMetricsProvider {
    pub fn new(max_history: usize) -> Self {
        Self {
            #[cfg(feature = "remote")]
            remote_collector: Arc::new(RemoteMetricsCollector::new()),
            remote_endpoints: Arc::new(RwLock::new(HashMap::new())),
            metrics_cache: Arc::new(RwLock::new(HashMap::new())),
            blueprint_metrics: Arc::new(RwLock::new(BlueprintMetrics::default())),
            status: Arc::new(RwLock::new(BlueprintStatus::default())),
            history_system: Arc::new(RwLock::new(Vec::with_capacity(max_history))),
            history_blueprint: Arc::new(RwLock::new(Vec::with_capacity(max_history))),
            max_history,
        }
    }

    /// Register a remote Blueprint instance for metrics collection
    pub async fn register_remote_instance(&self, instance_id: String, host: String, port: u16) {
        let endpoint = format!("http://{}:{}", host, port);
        self.remote_endpoints
            .write()
            .await
            .insert(instance_id, endpoint);
    }

    /// Register a remote Blueprint deployment result for metrics collection
    #[cfg(feature = "remote")]
    pub async fn register_blueprint_deployment(
        &self,
        result: &blueprint_remote_providers::infra::traits::BlueprintDeploymentResult,
    ) {
        if let Some(qos_endpoint) = result.qos_grpc_endpoint() {
            self.remote_endpoints
                .write()
                .await
                .insert(result.blueprint_id.clone(), qos_endpoint);
            tracing::info!(
                "Registered QoS endpoint for Blueprint deployment {}: {}",
                result.blueprint_id,
                qos_endpoint
            );
        } else {
            tracing::warn!(
                "Blueprint deployment {} does not expose QoS metrics port (9615)",
                result.blueprint_id
            );
        }
    }

    /// Unregister a remote Blueprint instance
    pub async fn unregister_remote_instance(&self, instance_id: &str) {
        self.remote_endpoints.write().await.remove(instance_id);
    }

    #[cfg(feature = "remote")]
    async fn collect_remote_metrics(&self) -> Result<(), Error> {
        use crate::proto::qos_metrics_client::QosMetricsClient;
        use crate::proto::{GetBlueprintMetricsRequest, GetResourceUsageRequest};

        let endpoints = self.remote_endpoints.read().await.clone();
        let mut cache = self.metrics_cache.write().await;

        for (instance_id, endpoint) in endpoints {
            // Connect to remote QoS gRPC service
            match QosMetricsClient::connect(endpoint.clone()).await {
                Ok(mut client) => {
                    // Extract service/blueprint IDs from instance_id or use defaults
                    let service_id = 1u64; // TODO: Extract from deployment config
                    let blueprint_id = 1u64; // TODO: Extract from deployment config

                    // Get resource usage metrics
                    if let Ok(response) = client
                        .get_resource_usage(GetResourceUsageRequest {
                            blueprint_id,
                            service_id,
                        })
                        .await
                    {
                        let usage = response.into_inner();
                        let system_metrics = SystemMetrics {
                            cpu_usage: usage.cpu_usage,
                            memory_usage: usage.memory_usage,
                            total_memory: usage.total_memory,
                            disk_usage: usage.disk_usage,
                            total_disk: usage.total_disk,
                            network_rx_bytes: usage.network_rx_bytes,
                            network_tx_bytes: usage.network_tx_bytes,
                            timestamp: usage.timestamp,
                        };

                        cache.insert(service_id, system_metrics);
                    }

                    // Get blueprint-specific metrics
                    if let Ok(response) = client
                        .get_blueprint_metrics(GetBlueprintMetricsRequest {
                            blueprint_id,
                            service_id,
                        })
                        .await
                    {
                        let metrics = response.into_inner();
                        let mut bp_metrics = self.blueprint_metrics.write().await;

                        for (key, value) in metrics.custom_metrics {
                            bp_metrics
                                .custom_metrics
                                .insert(format!("remote_{}_{}", instance_id, key), value);
                        }
                        bp_metrics.timestamp = metrics.timestamp;
                    }
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to connect to remote QoS service at {}: {}",
                        endpoint,
                        e
                    );
                    // TODO: Consider removing dead endpoints after multiple failures
                }
            }
        }

        Ok(())
    }
}

impl MetricsProvider for RemoteMetricsProvider {
    fn get_system_metrics(&self) -> impl Future<Output = SystemMetrics> + Send {
        async move {
            // Aggregate all remote instance metrics
            let cache = self.metrics_cache.read().await;
            if cache.is_empty() {
                return SystemMetrics::default();
            }

            // Sum up metrics from all remote instances
            let mut total = SystemMetrics::default();
            for metrics in cache.values() {
                total.network_rx_bytes += metrics.network_rx_bytes;
                total.network_tx_bytes += metrics.network_tx_bytes;
                // Could aggregate other metrics
            }
            total.timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            total
        }
    }

    fn get_blueprint_metrics(&self) -> impl Future<Output = BlueprintMetrics> + Send {
        async move { self.blueprint_metrics.read().await.clone() }
    }

    fn get_blueprint_status(&self) -> impl Future<Output = BlueprintStatus> + Send {
        async move { self.status.read().await.clone() }
    }

    fn get_system_metrics_history(&self) -> impl Future<Output = Vec<SystemMetrics>> + Send {
        async move { self.history_system.read().await.clone() }
    }

    fn get_blueprint_metrics_history(&self) -> impl Future<Output = Vec<BlueprintMetrics>> + Send {
        async move { self.history_blueprint.read().await.clone() }
    }

    fn add_custom_metric(&self, key: String, value: String) -> impl Future<Output = ()> + Send {
        async move {
            let mut metrics = self.blueprint_metrics.write().await;
            metrics.custom_metrics.insert(key, value);
            metrics.timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }

    fn set_blueprint_status(
        &self,
        status_code: u32,
        status_message: Option<String>,
    ) -> impl Future<Output = ()> + Send {
        async move {
            let mut status = self.status.write().await;
            status.status_code = status_code;
            status.status_message = status_message;
            status.timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }

    fn update_last_heartbeat(&self, timestamp: u64) -> impl Future<Output = ()> + Send {
        async move {
            let mut status = self.status.write().await;
            status.last_heartbeat = Some(timestamp);
        }
    }

    fn start_collection(&self) -> impl Future<Output = Result<(), Error>> + Send {
        async move {
            #[cfg(feature = "remote")]
            {
                let provider = self.clone();
                tokio::spawn(async move {
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
                    loop {
                        interval.tick().await;
                        if let Err(e) = provider.collect_remote_metrics().await {
                            tracing::error!("Failed to collect remote metrics: {}", e);
                        }

                        // Store in history
                        let current = provider.get_system_metrics().await;
                        let mut history = provider.history_system.write().await;
                        history.push(current);
                        if history.len() > provider.max_history {
                            history.remove(0);
                        }

                        let current_bp = provider.get_blueprint_metrics().await;
                        let mut history_bp = provider.history_blueprint.write().await;
                        history_bp.push(current_bp);
                        if history_bp.len() > provider.max_history {
                            history_bp.remove(0);
                        }
                    }
                });
            }
            Ok(())
        }
    }
}

impl Clone for RemoteMetricsProvider {
    fn clone(&self) -> Self {
        Self {
            #[cfg(feature = "remote")]
            remote_collector: self.remote_collector.clone(),
            remote_endpoints: self.remote_endpoints.clone(),
            metrics_cache: self.metrics_cache.clone(),
            blueprint_metrics: self.blueprint_metrics.clone(),
            status: self.status.clone(),
            history_system: self.history_system.clone(),
            history_blueprint: self.history_blueprint.clone(),
            max_history: self.max_history,
        }
    }
}
