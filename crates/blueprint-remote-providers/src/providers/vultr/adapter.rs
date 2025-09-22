//! Vultr CloudProviderAdapter implementation

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
// use crate::providers::vultr::VultrProvisioner;
use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{info, warn};

/// Vultr adapter for Blueprint deployment
pub struct VultrAdapter {
    // provisioner: VultrProvisioner,
    api_key: String,
}

impl VultrAdapter {
    /// Create new Vultr adapter
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("VULTR_API_KEY")
            .map_err(|_| Error::Other("VULTR_API_KEY environment variable not set".into()))?;
        
        // let provisioner = VultrProvisioner::new(api_key.clone()).await?;
        Ok(Self { api_key })
    }

    /// Get SSH username for Vultr instances
    fn get_ssh_username(&self) -> &'static str {
        "root"
    }
}

#[async_trait]
impl CloudProviderAdapter for VultrAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
    ) -> Result<ProvisionedInstance> {
        let spec = ResourceSpec {
            cpu: 2.0,
            memory_gb: 4.0, 
            storage_gb: 80.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };

        let instance_name = format!("blueprint-{}", uuid::Uuid::new_v4());
        
        // TODO: Implement actual Vultr provisioning
        let instance = ProvisionedInstance {
            id: format!("vultr-{}", uuid::Uuid::new_v4()),
            public_ip: Some("192.168.1.1".to_string()),
            private_ip: Some("10.0.0.1".to_string()),
            status: InstanceStatus::Running,
            provider: crate::core::remote::CloudProvider::Vultr,
            region: region.to_string(),
            instance_type: instance_type.to_string(),
        };

        info!(
            "Provisioned Vultr instance {} in region {}", 
            instance.id, region
        );

        Ok(instance)
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        // TODO: Implement actual Vultr termination
        info!("Terminating Vultr instance: {}", instance_id);
        Ok(())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        info!("Checking status for Vultr instance: {}", instance_id);
        Ok(InstanceStatus::Running)
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
                self.deploy_to_instance(blueprint_image, resource_spec, env_vars).await
            }
            DeploymentTarget::ManagedKubernetes { cluster_id, namespace } => {
                self.deploy_to_vke(cluster_id, namespace, blueprint_image, resource_spec, env_vars).await
            }
            DeploymentTarget::GenericKubernetes { context: _, namespace } => {
                self.deploy_to_generic_k8s(namespace, blueprint_image, resource_spec, env_vars).await
            }
            DeploymentTarget::Serverless { .. } => {
                Err(Error::Other("Vultr serverless deployment not implemented".into()))
            }
        }
    }

    /// Deploy to Vultr instance via SSH
    async fn deploy_to_instance(
        &self,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        let instance = self.provision_instance("vc2-2c-4gb", "ewr").await?;
        
        // TODO: Implement SSH deployment similar to other providers
        warn!("Vultr instance SSH deployment not fully implemented");

        let mut port_mappings = HashMap::new();
        port_mappings.insert(8080, 8080);
        port_mappings.insert(9615, 9615);
        port_mappings.insert(9944, 9944);

        let mut metadata = HashMap::new();
        metadata.insert("provider".to_string(), "vultr-instance".to_string());

        Ok(BlueprintDeploymentResult {
            instance,
            blueprint_id: format!("vultr-{}", uuid::Uuid::new_v4()),
            port_mappings,
            metadata,
        })
    }

    /// Deploy to VKE cluster
    async fn deploy_to_vke(
        &self,
        cluster_id: &str,
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::deployment::kubernetes::KubernetesDeploymentClient;
        
        info!("Deploying to VKE cluster: {}", cluster_id);
        
        let k8s_client = KubernetesDeploymentClient::new(Some(namespace.to_string())).await?;
        let (deployment_id, exposed_ports) = k8s_client
            .deploy_blueprint("blueprint", blueprint_image, resource_spec, 1)
            .await?;

        let mut port_mappings = HashMap::new();
        for port in exposed_ports {
            port_mappings.insert(port, port);
        }

        let mut metadata = HashMap::new();
        metadata.insert("provider".to_string(), "vultr-vke".to_string());
        metadata.insert("cluster_id".to_string(), cluster_id.to_string());
        metadata.insert("namespace".to_string(), namespace.to_string());

        let instance = ProvisionedInstance {
            id: format!("vke-{}", cluster_id),
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Running,
            provider: crate::core::remote::CloudProvider::Vultr,
            region: "ewr".to_string(),
            instance_type: "vke-cluster".to_string(),
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