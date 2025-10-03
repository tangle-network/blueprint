//! Azure CloudProviderAdapter implementation

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::azure::provisioner::AzureProvisioner;
use crate::providers::common::ProvisioningConfig;
use crate::deployment::ssh::{ContainerRuntime, DeploymentConfig, SshConnection, SshDeploymentClient};
use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{info, warn};

/// Azure adapter for Blueprint deployment
pub struct AzureAdapter {
    provisioner: std::sync::Arc<tokio::sync::Mutex<AzureProvisioner>>,
}

impl AzureAdapter {
    /// Create new Azure adapter
    pub async fn new() -> Result<Self> {
        let provisioner = AzureProvisioner::new().await?;

        Ok(Self {
            provisioner: std::sync::Arc::new(tokio::sync::Mutex::new(provisioner)),
        })
    }

}

#[async_trait]
impl CloudProviderAdapter for AzureAdapter {
    async fn provision_instance(
        &self,
        _instance_type: &str,
        region: &str,
    ) -> Result<ProvisionedInstance> {
        let spec = ResourceSpec {
            cpu: 2.0,
            memory_gb: 8.0,
            storage_gb: 128.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };

        let instance_name = format!("blueprint-{}", uuid::Uuid::new_v4());

        let config = ProvisioningConfig {
            name: instance_name.clone(),
            region: region.to_string(),
            ssh_key_name: std::env::var("AZURE_SSH_KEY_NAME").ok(),
            ami_id: None,
            machine_image: None,
            custom_config: HashMap::new(),
        };

        let mut provisioner = self.provisioner.lock().await;
        let infra = provisioner.provision_instance(&spec, &config).await?;

        info!(
            "Provisioned Azure instance {} in region {}",
            infra.instance_id, region
        );

        Ok(ProvisionedInstance {
            id: infra.instance_id,
            public_ip: infra.public_ip,
            private_ip: infra.private_ip,
            status: InstanceStatus::Running,
            provider: crate::core::remote::CloudProvider::Azure,
            region: infra.region,
            instance_type: infra.instance_type,
        })
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let mut provisioner = self.provisioner.lock().await;
        provisioner.terminate_instance(instance_id).await
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        let vm_name = instance_id.split('/').next_back().unwrap_or(instance_id);

        let subscription_id = std::env::var("AZURE_SUBSCRIPTION_ID")
            .map_err(|_| Error::ConfigurationError("AZURE_SUBSCRIPTION_ID not set".into()))?;
        let resource_group = std::env::var("AZURE_RESOURCE_GROUP")
            .unwrap_or_else(|_| "blueprint-resources".to_string());

        let url = format!(
            "https://management.azure.com/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Compute/virtualMachines/{vm_name}/instanceView?api-version=2023-09-01"
        );

        let client = reqwest::Client::new();
        let mut provisioner = self.provisioner.lock().await;
        let token = provisioner.get_access_token().await?;

        let response = client
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| Error::Other(format!("Failed to get instance status: {e}")))?;

        if response.status() == 404 {
            return Ok(InstanceStatus::Terminated);
        }

        if !response.status().is_success() {
            return Err(Error::Other(format!("Failed to get instance status: {}", response.status())));
        }

        let json: serde_json::Value = response.json().await
            .map_err(|e| Error::Other(format!("Failed to parse response: {e}")))?;

        if let Some(statuses) = json["statuses"].as_array() {
            for status in statuses {
                if let Some(code) = status["code"].as_str() {
                    if code.starts_with("PowerState/") {
                        return match code {
                            "PowerState/running" => Ok(InstanceStatus::Running),
                            "PowerState/starting" => Ok(InstanceStatus::Starting),
                            "PowerState/stopped" | "PowerState/deallocated" => Ok(InstanceStatus::Stopped),
                            "PowerState/stopping" | "PowerState/deallocating" => Ok(InstanceStatus::Stopping),
                            _ => Ok(InstanceStatus::Unknown),
                        };
                    }
                }
            }
        }

        Ok(InstanceStatus::Unknown)
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
                #[cfg(feature = "kubernetes")]
                {
                    self.deploy_to_aks(cluster_id, namespace, blueprint_image, resource_spec, env_vars).await
                }
                #[cfg(not(feature = "kubernetes"))]
                {
                    warn!("Kubernetes deployment requested for cluster {} namespace {}, but feature not enabled", cluster_id, namespace);
                    Err(Error::Other("Kubernetes support not enabled".into()))
                }
            }
            DeploymentTarget::GenericKubernetes { context: _, namespace } => {
                #[cfg(feature = "kubernetes")]
                {
                    self.deploy_to_generic_k8s(namespace, blueprint_image, resource_spec, env_vars).await
                }
                #[cfg(not(feature = "kubernetes"))]
                {
                    warn!("Kubernetes deployment requested for namespace {}, but feature not enabled", namespace);
                    Err(Error::Other("Kubernetes support not enabled".into()))
                }
            }
            DeploymentTarget::Serverless { .. } => {
                Err(Error::Other("Azure Container Instances deployment not implemented".into()))
            }
        }
    }







    async fn health_check_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<bool> {
        if let Some(endpoint) = deployment.qos_grpc_endpoint() {
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .map_err(|e| Error::Other(format!("Failed to create HTTP client: {e}")))?;

            match client.get(format!("{endpoint}/health")).send().await {
                Ok(response) => {
                    let healthy = response.status().is_success();
                    if healthy {
                        info!("Azure blueprint {} health check passed", deployment.blueprint_id);
                    }
                    Ok(healthy)
                }
                Err(e) => {
                    warn!("Azure health check failed: {}", e);
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }

    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()> {
        info!("Cleaning up Azure blueprint deployment: {}", deployment.blueprint_id);
        self.terminate_instance(&deployment.instance.id).await
    }
}

// Private helper methods
impl AzureAdapter {
    /// Deploy to Azure VM via SSH
    async fn deploy_to_vm(
        &self,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        let instance = self.provision_instance("Standard_B2ms", "eastus").await?;
        let public_ip = instance.public_ip.as_ref()
            .ok_or_else(|| Error::Other("Instance has no public IP".into()))?;

        // SSH connection configuration
        let connection = SshConnection {
            host: public_ip.clone(),
            user: "azureuser".to_string(),
            key_path: std::env::var("AZURE_SSH_KEY_PATH").ok().map(|p| p.into()),
            port: 22,
            password: None,
            jump_host: None,
        };

        let deployment_config = DeploymentConfig {
            name: format!("blueprint-{}", uuid::Uuid::new_v4()),
            namespace: "blueprint-azure".to_string(),
            restart_policy: crate::deployment::ssh::RestartPolicy::OnFailure,
            health_check: None,
        };

        let ssh_client = SshDeploymentClient::new(connection, ContainerRuntime::Docker, deployment_config)
            .await
            .map_err(|e| Error::Other(format!("Failed to establish SSH connection: {e}")))?;

        let deployment = ssh_client
            .deploy_blueprint(blueprint_image, resource_spec, env_vars)
            .await
            .map_err(|e| Error::Other(format!("Blueprint deployment failed: {e}")))?;

        let mut port_mappings = HashMap::new();
        for (internal_port_str, external_port_str) in &deployment.ports {
            if let (Ok(internal), Ok(external)) = (
                internal_port_str.trim_end_matches("/tcp").parse::<u16>(),
                external_port_str.parse::<u16>(),
            ) {
                port_mappings.insert(internal, external);
            }
        }

        let mut metadata = HashMap::new();
        metadata.insert("provider".to_string(), "azure-vm".to_string());
        metadata.insert("container_id".to_string(), deployment.container_id.clone());
        metadata.insert("ssh_host".to_string(), deployment.host.clone());

        info!(
            "Successfully deployed blueprint {} to Azure VM {}",
            deployment.container_id, instance.id
        );

        Ok(BlueprintDeploymentResult {
            instance: instance.clone(),
            blueprint_id: deployment.container_id,
            port_mappings,
            metadata,
        })
    }

    /// Deploy to AKS cluster
    async fn deploy_to_aks(
        &self,
        cluster_id: &str,
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        _env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        #[cfg(feature = "kubernetes")]
        {
            use crate::shared::{SharedKubernetesDeployment, ManagedK8sConfig};

            let config = ManagedK8sConfig::aks("eastus", "blueprint-resources");
            SharedKubernetesDeployment::deploy_to_managed_k8s(
                cluster_id,
                namespace,
                blueprint_image,
                resource_spec,
                config,
            ).await
        }
        #[cfg(not(feature = "kubernetes"))]
        {
            Err(Error::ConfigurationError(
                "Kubernetes feature not enabled".to_string(),
            ))
        }
    }

    /// Deploy to generic Kubernetes cluster
    async fn deploy_to_generic_k8s(
        &self,
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        _env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        #[cfg(feature = "kubernetes")]
        {
            use crate::shared::SharedKubernetesDeployment;
            SharedKubernetesDeployment::deploy_to_generic_k8s(
                namespace,
                blueprint_image,
                resource_spec,
            ).await
        }
        #[cfg(not(feature = "kubernetes"))]
        {
            Err(Error::ConfigurationError(
                "Kubernetes feature not enabled".to_string(),
            ))
        }
    }
}