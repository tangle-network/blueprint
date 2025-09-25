//! Azure CloudProviderAdapter implementation

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::azure::AzureProvisioner;
use async_trait::async_trait;
use std::collections::HashMap;

/// Azure adapter for Blueprint deployment
pub struct AzureAdapter {
    provisioner: AzureProvisioner,
}

impl AzureAdapter {
    /// Create new Azure adapter
    pub async fn new() -> Result<Self> {
        let provisioner = AzureProvisioner::new().await?;
        Ok(Self { provisioner })
    }
}

#[async_trait]
impl CloudProviderAdapter for AzureAdapter {
    async fn provision_instance(
        &self,
        _instance_type: &str,
        _region: &str,
    ) -> Result<ProvisionedInstance> {
        Err(Error::Other(
            "Azure adapter not yet implemented - implement in providers/azure/".to_string()
        ))
    }

    async fn terminate_instance(&self, _instance_id: &str) -> Result<()> {
        Err(Error::Other(
            "Azure termination not yet implemented".to_string()
        ))
    }

    async fn get_instance_status(&self, _instance_id: &str) -> Result<InstanceStatus> {
        Err(Error::Other(
            "Azure status checking not yet implemented".to_string()
        ))
    }

    async fn deploy_blueprint_with_target(
        &self,
        target: &crate::core::deployment_target::DeploymentTarget,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::core::deployment_target::DeploymentTarget;
        
        match target {
            DeploymentTarget::VirtualMachine { runtime: _ } => {
                self.deploy_to_vm(blueprint_image, resource_spec, env_vars).await
            }
            DeploymentTarget::ManagedKubernetes { cluster_id, namespace } => {
                self.deploy_to_aks(cluster_id, namespace, blueprint_image, resource_spec, env_vars).await
            }
            DeploymentTarget::GenericKubernetes { context: _, namespace } => {
                self.deploy_to_generic_k8s(namespace, blueprint_image, resource_spec, env_vars).await
            }
            DeploymentTarget::Serverless { .. } => {
                Err(Error::Other("Azure Container Instances deployment not implemented".into()))
            }
        }
    }

    /// Deploy to Azure VM via SSH
    async fn deploy_to_vm(
        &self,
        _blueprint_image: &str,
        _resource_spec: &ResourceSpec,
        _env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        Err(Error::Other("Azure VM deployment not implemented".into()))
    }

    /// Deploy to AKS cluster
    async fn deploy_to_aks(
        &self,
        cluster_id: &str,
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::deployment::kubernetes::KubernetesDeploymentClient;
        
        info!("Deploying to AKS cluster: {}", cluster_id);
        
        let k8s_client = KubernetesDeploymentClient::new(Some(namespace.to_string())).await?;
        let (deployment_id, exposed_ports) = k8s_client
            .deploy_blueprint("blueprint", blueprint_image, resource_spec, 1)
            .await?;

        let mut port_mappings = HashMap::new();
        for port in exposed_ports {
            port_mappings.insert(port, port);
        }

        let mut metadata = HashMap::new();
        metadata.insert("provider".to_string(), "azure-aks".to_string());
        metadata.insert("cluster_id".to_string(), cluster_id.to_string());
        metadata.insert("namespace".to_string(), namespace.to_string());

        let instance = ProvisionedInstance {
            id: format!("aks-{}", cluster_id),
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Running,
            provider: crate::core::remote::CloudProvider::Azure,
            region: "eastus".to_string(),
            instance_type: "aks-cluster".to_string(),
        };

        Ok(BlueprintDeploymentResult {
            instance,
            blueprint_id: deployment_id,
            port_mappings,
            metadata,
        })
    }

    /// Deploy to generic Kubernetes cluster
    async fn deploy_to_generic_k8s(
        &self,
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::deployment::kubernetes::KubernetesDeploymentClient;
        
        info!("Deploying to generic Kubernetes namespace: {}", namespace);
        
        let k8s_client = KubernetesDeploymentClient::new(Some(namespace.to_string())).await?;
        let (deployment_id, exposed_ports) = k8s_client
            .deploy_blueprint("blueprint", blueprint_image, resource_spec, 1)
            .await?;

        let mut port_mappings = HashMap::new();
        for port in exposed_ports {
            port_mappings.insert(port, port);
        }

        let mut metadata = HashMap::new();
        metadata.insert("provider".to_string(), "generic-k8s".to_string());
        metadata.insert("namespace".to_string(), namespace.to_string());

        let instance = ProvisionedInstance {
            id: format!("k8s-{}", namespace),
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Running,
            provider: crate::core::remote::CloudProvider::Generic,
            region: "generic".to_string(),
            instance_type: "kubernetes-cluster".to_string(),
        };

        Ok(BlueprintDeploymentResult {
            instance,
            blueprint_id: deployment_id,
            port_mappings,
            metadata,
        })
    }

    async fn health_check_blueprint(&self, _deployment: &BlueprintDeploymentResult) -> Result<bool> {
        Ok(false)
    }

    async fn cleanup_blueprint(&self, _deployment: &BlueprintDeploymentResult) -> Result<()> {
        Ok(())
    }
}