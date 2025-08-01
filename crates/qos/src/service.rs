use blueprint_core::info;
use std::sync::Arc;
use tonic::{Request, Response, Status, transport::Server};

use crate::error::{Error, Result};
use crate::metrics::MetricsProvider;
use crate::proto::qos_metrics_server::{QosMetrics, QosMetricsServer};
use crate::proto::{
    BlueprintMetrics as ProtoBlueprintMetrics, GetBlueprintMetricsRequest,
    GetBlueprintMetricsResponse, GetHistoricalMetricsRequest, GetHistoricalMetricsResponse,
    GetResourceUsageRequest, GetResourceUsageResponse, GetStatusRequest, GetStatusResponse,
    SystemMetrics as ProtoSystemMetrics,
};

/// Implementation of the `QoS` gRPC service for exposing metrics via a remote API.
/// This service provides endpoints for retrieving blueprint status, resource usage,
/// custom metrics, and historical data over a standardized protocol.
#[derive(Debug)]
pub struct QosMetricsService<T> {
    provider: Arc<T>,
}

impl<T> QosMetricsService<T>
where
    T: MetricsProvider,
{
    /// Creates a new `QoS` metrics service with the provided metrics provider.
    ///
    /// The provider supplies the actual metric data that will be exposed through the gRPC service.
    pub fn new(provider: Arc<T>) -> Self {
        Self { provider }
    }
}

#[tonic::async_trait]
impl<T> QosMetrics for QosMetricsService<T>
where
    T: MetricsProvider + 'static,
{
    async fn get_status(
        &self,
        request: Request<GetStatusRequest>,
    ) -> std::result::Result<Response<GetStatusResponse>, Status> {
        let req = request.into_inner();
        info!(
            blueprint_id = req.blueprint_id,
            service_id = req.service_id,
            "Received GetStatus request"
        );

        let status = self.provider.get_blueprint_status().await;

        if status.blueprint_id != req.blueprint_id || status.service_id != req.service_id {
            return Err(Status::not_found("Blueprint or service ID not found"));
        }

        let response = GetStatusResponse {
            status_code: status.status_code,
            status_message: status.status_message,
            uptime: status.uptime,
            start_time: status.start_time,
            last_heartbeat: status.last_heartbeat,
            timestamp: status.timestamp,
            service_id: status.service_id,
            blueprint_id: status.blueprint_id,
        };

        Ok(Response::new(response))
    }

    async fn get_resource_usage(
        &self,
        request: Request<GetResourceUsageRequest>,
    ) -> std::result::Result<Response<GetResourceUsageResponse>, Status> {
        let req = request.into_inner();
        info!(
            blueprint_id = req.blueprint_id,
            service_id = req.service_id,
            "Received GetResourceUsage request"
        );

        let status = self.provider.get_blueprint_status().await;

        if status.blueprint_id != req.blueprint_id || status.service_id != req.service_id {
            return Err(Status::not_found("Blueprint or service ID not found"));
        }

        let metrics = self.provider.get_system_metrics().await;

        let response = GetResourceUsageResponse {
            cpu_usage: metrics.cpu_usage,
            memory_usage: metrics.memory_usage,
            total_memory: metrics.total_memory,
            disk_usage: metrics.disk_usage,
            total_disk: metrics.total_disk,
            network_rx_bytes: metrics.network_rx_bytes,
            network_tx_bytes: metrics.network_tx_bytes,
            timestamp: metrics.timestamp,
        };

        Ok(Response::new(response))
    }

    async fn get_blueprint_metrics(
        &self,
        request: Request<GetBlueprintMetricsRequest>,
    ) -> std::result::Result<Response<GetBlueprintMetricsResponse>, Status> {
        let req = request.into_inner();
        info!(
            blueprint_id = req.blueprint_id,
            service_id = req.service_id,
            "Received GetBlueprintMetrics request"
        );

        let status = self.provider.get_blueprint_status().await;

        if status.blueprint_id != req.blueprint_id || status.service_id != req.service_id {
            return Err(Status::not_found("Blueprint or service ID not found"));
        }

        let metrics = self.provider.get_blueprint_metrics().await;

        let response = GetBlueprintMetricsResponse {
            custom_metrics: metrics.custom_metrics,
            timestamp: metrics.timestamp,
        };

        Ok(Response::new(response))
    }

    async fn get_historical_metrics(
        &self,
        request: Request<GetHistoricalMetricsRequest>,
    ) -> std::result::Result<Response<GetHistoricalMetricsResponse>, Status> {
        let req = request.into_inner();
        info!(
            blueprint_id = req.blueprint_id,
            service_id = req.service_id,
            "Received GetHistoricalMetrics request"
        );

        let status = self.provider.get_blueprint_status().await;

        if status.blueprint_id != req.blueprint_id || status.service_id != req.service_id {
            return Err(Status::not_found("Blueprint or service ID not found"));
        }

        let system_metrics_history = if req.metrics_type == 0 {
            self.provider
                .get_system_metrics_history()
                .await
                .into_iter()
                .map(|m| ProtoSystemMetrics {
                    cpu_usage: m.cpu_usage,
                    memory_usage: m.memory_usage,
                    total_memory: m.total_memory,
                    disk_usage: m.disk_usage,
                    total_disk: m.total_disk,
                    network_rx_bytes: m.network_rx_bytes,
                    network_tx_bytes: m.network_tx_bytes,
                    timestamp: m.timestamp,
                })
                .collect()
        } else {
            Vec::new()
        };

        let blueprint_metrics_history = if req.metrics_type == 1 {
            self.provider
                .get_blueprint_metrics_history()
                .await
                .into_iter()
                .map(|m| ProtoBlueprintMetrics {
                    custom_metrics: m.custom_metrics,
                    timestamp: m.timestamp,
                })
                .collect()
        } else {
            Vec::new()
        };

        let response = GetHistoricalMetricsResponse {
            system_metrics: system_metrics_history,
            blueprint_metrics: blueprint_metrics_history,
        };

        Ok(Response::new(response))
    }
}

/// Starts a gRPC server that exposes `QoS` metrics on the specified address.
///
/// This function binds to the provided address and serves the `QosMetrics` gRPC service
/// using the supplied metrics provider as the data source. It runs indefinitely until
/// the server is shut down or encounters an error.
///
/// # Errors
/// Returns an error if the server fails to bind to the address, fails to start,
/// or encounters an error during operation.
pub async fn run_qos_server<T>(bind_address: String, provider: Arc<T>) -> Result<()>
where
    T: MetricsProvider + 'static,
{
    let addr = bind_address
        .parse()
        .map_err(|e| Error::Other(format!("Failed to parse bind address: {}", e)))?;

    info!("QoS metrics server listening on {}", addr);

    let service = QosMetricsService::new(provider);
    let server = QosMetricsServer::new(service);

    Server::builder()
        .add_service(server)
        .serve(addr)
        .await
        .map_err(Error::Grpc)?;

    Ok(())
}
