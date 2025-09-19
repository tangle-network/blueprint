//! Traits for cloud provider adapters

use crate::core::error::Result;
use crate::core::resources::ResourceSpec;
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use blueprint_std::collections::HashMap;
use async_trait::async_trait;

/// Blueprint deployment result containing connection information and exposed ports
#[derive(Debug, Clone)]
pub struct BlueprintDeploymentResult {
    /// Infrastructure instance where Blueprint is deployed
    pub instance: ProvisionedInstance,
    /// Blueprint container/service identifier  
    pub blueprint_id: String,
    /// Port mappings: internal_port -> external_port
    pub port_mappings: HashMap<u16, u16>,
    /// Deployment metadata
    pub metadata: HashMap<String, String>,
}

impl BlueprintDeploymentResult {
    /// Get the external port for QoS metrics (9615)
    pub fn qos_metrics_port(&self) -> Option<u16> {
        self.port_mappings.get(&9615).copied()
    }

    /// Get the external port for RPC endpoint (9944)
    pub fn rpc_port(&self) -> Option<u16> {
        self.port_mappings.get(&9944).copied()
    }

    /// Build QoS gRPC endpoint URL
    pub fn qos_grpc_endpoint(&self) -> Option<String> {
        self.qos_metrics_port()
            .map(|port| format!("http://{}:{}", self.instance.public_ip, port))
    }
}

/// Common adapter trait for all cloud providers
#[async_trait]
pub trait CloudProviderAdapter: Send + Sync {
    /// Provision a new instance of the specified type in the given region
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
    ) -> Result<ProvisionedInstance>;
    
    /// Terminate an existing instance
    async fn terminate_instance(&self, instance_id: &str) -> Result<()>;
    
    /// Get the current status of an instance
    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus>;
    
    /// Deploy a Blueprint service to a provisioned instance with QoS port exposure
    /// 
    /// This method handles the complete deployment flow:
    /// 1. Connect to the provisioned instance via SSH
    /// 2. Install container runtime if needed
    /// 3. Deploy Blueprint container with proper port mapping (8080, 9615, 9944)
    /// 4. Return deployment result with QoS endpoint information
    async fn deploy_blueprint(
        &self,
        instance: &ProvisionedInstance,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult>;
    
    /// Check if a Blueprint deployment is healthy and responsive
    async fn health_check_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<bool>;
    
    /// Cleanup a Blueprint deployment from an instance
    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()>;
}