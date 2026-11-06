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
    fn managed(&self) -> Option<&dyn crate::managed::ManagedProviderAdapter> {
        None
    }

    /// Lossy legacy calls cannot allocate a managed GPU lease.
    async fn provision_instance(
        &self,
        _sku: &str,
        _region: &str,
        _tee: bool,
    ) -> Result<ProvisionedInstance> {
        Err(crate::Error::Provider(
            crate::managed::ProviderErrorCode::Unsupported,
        ))
    }
    async fn terminate_instance(&self, _id: &str) -> Result<()> {
        Err(crate::Error::Provider(
            crate::managed::ProviderErrorCode::Unsupported,
        ))
    }
    async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        self.get_instance_details(id)
            .await
            .map(|instance| instance.status)
    }
    async fn get_instance_details(&self, _id: &str) -> Result<ProvisionedInstance> {
        Err(crate::Error::Provider(
            crate::managed::ProviderErrorCode::Unsupported,
        ))
    }
    async fn deploy_blueprint_with_target(
        &self,
        target: &crate::core::deployment_target::DeploymentTarget,
        image: &str,
        spec: &ResourceSpec,
        env: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        #[cfg(feature = "kubernetes")]
        if let crate::core::deployment_target::DeploymentTarget::GenericKubernetes {
            namespace,
            ..
        } = target
        {
            return crate::shared::SharedKubernetesDeployment::deploy_to_generic_k8s(
                namespace, image, spec, env,
            )
            .await;
        }
        let _ = (target, image, spec, env);
        Err(crate::Error::Provider(
            crate::managed::ProviderErrorCode::Unsupported,
        ))
    }
    /// Deploying an existing VM requires the SDK's full access-aware lifecycle.
    async fn deploy_blueprint(
        &self,
        _instance: &ProvisionedInstance,
        _image: &str,
        _spec: &ResourceSpec,
        _env: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        Err(crate::Error::Provider(
            crate::managed::ProviderErrorCode::Unsupported,
        ))
    }
    async fn health_check_blueprint(
        &self,
        _deployment: &BlueprintDeploymentResult,
    ) -> Result<bool> {
        Err(crate::Error::Provider(
            crate::managed::ProviderErrorCode::Unsupported,
        ))
    }
    async fn cleanup_blueprint(&self, _deployment: &BlueprintDeploymentResult) -> Result<()> {
        Err(crate::Error::Provider(
            crate::managed::ProviderErrorCode::Unsupported,
        ))
    }
}
