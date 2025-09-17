//! Integration with existing on-chain heartbeat/QoS reporting system

use crate::core::error::{Error, Result};
use blueprint_qos::heartbeat::{HeartbeatConsumer, HeartbeatStatus};
use blueprint_std::sync::Arc;
use parity_scale_codec::{Decode, Encode};
use std::pin::Pin;
use tracing::info;

/// Remote deployment heartbeat consumer that integrates with on-chain reporting
pub struct RemoteDeploymentHeartbeat {
    deployment_id: String,
}

impl RemoteDeploymentHeartbeat {
    pub fn new(deployment_id: String) -> Self {
        Self { deployment_id }
    }
}

impl HeartbeatConsumer for RemoteDeploymentHeartbeat {
    fn send_heartbeat(
        &self,
        status: &HeartbeatStatus,
    ) -> Pin<Box<dyn std::future::Future<Output = blueprint_qos::error::Result<()>> + Send + 'static>>
    {
        let deployment_id = self.deployment_id.clone();
        let status = status.clone();

        Box::pin(async move {
            // Enhance heartbeat with remote deployment metrics
            let mut enhanced_status = status;

            // Add deployment-specific metrics to status message
            enhanced_status.status_message =
                Some(format!("deployment_id={},type=remote", deployment_id));

            info!(
                "Sending remote deployment heartbeat for {} (service_id={}, blueprint_id={})",
                deployment_id, enhanced_status.service_id, enhanced_status.blueprint_id
            );

            // The actual on-chain submission is handled by the HeartbeatService
            // which calls this consumer
            Ok(())
        })
    }
}

/// Extended metrics for remote deployments that get encoded in heartbeat
#[derive(Debug, Clone, Encode, Decode)]
pub struct RemoteDeploymentMetrics {
    pub deployment_id: Vec<u8>,
    pub provider: Vec<u8>, // "aws", "gcp", etc.
    pub region: Vec<u8>,
    pub instance_type: Vec<u8>,
    pub cpu_usage_percent: u8,
    pub memory_usage_percent: u8,
    pub network_mbps: u32,
    pub uptime_seconds: u64,
    pub error_count: u32,
    pub request_count: u64,
}

impl RemoteDeploymentMetrics {
    /// Encode metrics for inclusion in heartbeat
    pub fn encode_for_heartbeat(&self) -> Vec<u8> {
        self.encode()
    }

    /// Create metrics from deployment status
    pub fn from_deployment_status(
        deployment_id: &str,
        provider: &str,
        cpu: f64,
        memory: f64,
        uptime: u64,
    ) -> Self {
        Self {
            deployment_id: deployment_id.as_bytes().to_vec(),
            provider: provider.as_bytes().to_vec(),
            region: b"us-east-1".to_vec(), // TODO: Get actual region
            instance_type: b"t3.medium".to_vec(), // TODO: Get actual type
            cpu_usage_percent: (cpu * 100.0) as u8,
            memory_usage_percent: (memory * 100.0) as u8,
            network_mbps: 0,
            uptime_seconds: uptime,
            error_count: 0,
            request_count: 0,
        }
    }
}

/// Integration point for remote deployments with existing QoS
pub async fn setup_remote_qos(
    deployment_id: String,
    service_id: u64,
    blueprint_id: u64,
    ws_rpc_endpoint: String,
    keystore_uri: String,
) -> Result<blueprint_qos::heartbeat::HeartbeatService<RemoteDeploymentHeartbeat>> {
    let consumer = Arc::new(RemoteDeploymentHeartbeat::new(deployment_id));

    let heartbeat_service = blueprint_qos::heartbeat::HeartbeatService::new(
        blueprint_qos::heartbeat::HeartbeatConfig::default(),
        consumer,
        ws_rpc_endpoint,
        keystore_uri,
        service_id,
        blueprint_id,
    );

    heartbeat_service
        .start_heartbeat()
        .await
        .map_err(|e| crate::core::error::Error::Other(format!("Failed to start heartbeat: {}", e)))?;

    Ok(heartbeat_service)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_encoding() {
        let metrics = RemoteDeploymentMetrics::from_deployment_status(
            "deployment-123",
            "aws",
            0.45,
            0.67,
            3600,
        );

        assert_eq!(metrics.cpu_usage_percent, 45);
        assert_eq!(metrics.memory_usage_percent, 67);
        assert_eq!(metrics.uptime_seconds, 3600);

        let encoded = metrics.encode_for_heartbeat();
        assert!(!encoded.is_empty());

        // Verify we can decode it
        let decoded = RemoteDeploymentMetrics::decode(&mut &encoded[..]).unwrap();
        assert_eq!(decoded.deployment_id, b"deployment-123");
        assert_eq!(decoded.provider, b"aws");
    }
}
