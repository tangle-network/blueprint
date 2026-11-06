//! Traits for cloud provider adapters

use crate::core::error::Result;
use crate::core::resources::ResourceSpec;
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use async_trait::async_trait;
use blueprint_std::collections::HashMap;

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
        match (self.qos_metrics_port(), &self.instance.public_ip) {
            (Some(port), Some(ip)) => Some(format!("http://{ip}:{port}")),
            _ => None,
        }
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
        require_tee: bool,
    ) -> Result<ProvisionedInstance>;

    /// Provision with the complete request, including GPU class/count/VRAM and access key.
    async fn provision_resources(
        &self,
        instance_type: &str,
        region: &str,
        spec: &ResourceSpec,
        require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        spec.validate()
            .map_err(crate::core::error::Error::ConfigurationError)?;
        if spec.gpu_count.is_some() || spec.ssh_public_key.is_some() {
            return Err(crate::core::error::Error::ConfigurationError(
                "Provider does not implement GPU allocation with per-lease access".into(),
            ));
        }
        self.provision_instance(instance_type, region, require_tee)
            .await
    }

    /// Live price for an in-stock allocation. Unpriced providers cannot be selected.
    async fn quote(
        &self,
        _spec: &ResourceSpec,
        _region: &str,
    ) -> Result<crate::providers::common::InstanceSelection> {
        Err(crate::core::error::Error::ConfigurationError(
            "Live price unavailable".into(),
        ))
    }

    /// Install access after creation, once the resource is durably tracked.
    async fn prepare_access(
        &self,
        _instance: &ProvisionedInstance,
        _spec: &ResourceSpec,
    ) -> Result<()> {
        Ok(())
    }

    /// SSH URI for the allocated resource; account credentials never form lease access.
    async fn lease_endpoint(&self, instance: &ProvisionedInstance) -> Result<String> {
        instance
            .public_ip
            .as_ref()
            .map(|ip| format!("ssh://root@{ip}:22"))
            .ok_or_else(|| {
                crate::core::error::Error::ConfigurationError("No lease endpoint".into())
            })
    }

    /// Terminate an existing instance
    async fn terminate_instance(&self, instance_id: &str) -> Result<()>;

    /// Get the current status of an instance
    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus>;

    /// Get full instance details including public IP (may not be implemented by all providers)
    async fn get_instance_details(&self, _instance_id: &str) -> Result<ProvisionedInstance> {
        // Default implementation returns error - providers should override if they support this
        Err(crate::core::error::Error::Other(
            "get_instance_details not implemented for this provider".into(),
        ))
    }

    /// Deploy a Blueprint service with QoS port exposure
    ///
    /// Routes to appropriate deployment method based on target:
    /// - VirtualMachine: SSH + Docker deployment
    /// - ManagedKubernetes: Provider's managed K8s (EKS/GKE/AKS)
    /// - GenericKubernetes: kubectl API deployment
    async fn deploy_blueprint_with_target(
        &self,
        target: &crate::core::deployment_target::DeploymentTarget,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult>;

    /// Legacy method - deploys to VM by default
    async fn deploy_blueprint(
        &self,
        _instance: &ProvisionedInstance,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::core::deployment_target::{ContainerRuntime, DeploymentTarget};
        let target = DeploymentTarget::VirtualMachine {
            runtime: ContainerRuntime::Docker,
        };
        self.deploy_blueprint_with_target(&target, blueprint_image, resource_spec, env_vars)
            .await
    }

    /// Check if a Blueprint deployment is healthy and responsive
    async fn health_check_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<bool>;

    /// Cleanup a Blueprint deployment from an instance
    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()>;
}
