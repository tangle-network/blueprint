//! Azure CloudProviderAdapter implementation

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::deployment::ssh::{
    ContainerRuntime, DeploymentConfig, SshConnection, SshDeploymentClient,
};
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::azure::provisioner::AzureProvisioner;
use crate::providers::common::ProvisioningConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::{collections::HashMap, sync::Arc};

/// Azure adapter for Blueprint deployment
pub struct AzureAdapter {
    provisioner: Arc<tokio::sync::Mutex<AzureProvisioner>>,
    pricing: crate::pricing::PricingFetcher,
    network_interface_id: Option<String>,
}

impl AzureAdapter {
    /// Create new Azure adapter
    pub async fn new() -> Result<Self> {
        let provisioner = AzureProvisioner::new().await?;
        Ok(Self {
            provisioner: Arc::new(tokio::sync::Mutex::new(provisioner)),
            pricing: crate::pricing::PricingFetcher::new()
                .map_err(|e| Error::ConfigurationError(e.to_string()))?,
            network_interface_id: std::env::var("AZURE_NETWORK_INTERFACE_ID").ok(),
        })
    }

    pub fn with_credentials(subscription: String, group: String, token: String) -> Result<Self> {
        Ok(Self {
            provisioner: Arc::new(tokio::sync::Mutex::new(AzureProvisioner::with_credentials(
                subscription,
                group,
                token,
            )?)),
            pricing: crate::pricing::PricingFetcher::new()
                .map_err(|e| Error::ConfigurationError(e.to_string()))?,
            network_interface_id: None,
        })
    }

    pub fn with_endpoint(
        subscription: String,
        group: String,
        token: String,
        endpoint: &str,
    ) -> Result<Self> {
        Ok(Self {
            provisioner: Arc::new(tokio::sync::Mutex::new(AzureProvisioner::with_endpoint(
                subscription,
                group,
                token,
                endpoint,
            )?)),
            pricing: crate::pricing::PricingFetcher::new()
                .and_then(|p| p.with_endpoint(crate::CloudProvider::Azure, endpoint))
                .map_err(|e| Error::ConfigurationError(e.to_string()))?,
            network_interface_id: None,
        })
    }

    /// Existing account network, retained when the lease VM is deleted.
    pub fn with_network_interface(mut self, id: String) -> Self {
        self.network_interface_id = Some(id);
        self
    }

    fn env_require_tee() -> bool {
        std::env::var("BLUEPRINT_REMOTE_TEE_REQUIRED")
            .ok()
            .is_some_and(|value| {
                matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes"
                )
            })
    }
}

#[async_trait]
impl CloudProviderAdapter for AzureAdapter {
    async fn quote(
        &self,
        spec: &ResourceSpec,
        region: &str,
    ) -> Result<crate::providers::common::InstanceSelection> {
        spec.validate().map_err(Error::ConfigurationError)?;
        let sku = self.provisioner.lock().await.select_vm_size(spec);
        let price = self
            .pricing
            .quote_instance(crate::CloudProvider::Azure, region, &sku)
            .await
            .map_err(|e| Error::HttpError(e.to_string()))?
            .ok_or_else(|| Error::ConfigurationError("Azure exact VM SKU is unpriced".into()))?;
        crate::providers::common::gpu_adapter::quoted_selection(sku, price)
    }

    async fn provision_resources(
        &self,
        instance_type: &str,
        region: &str,
        spec: &ResourceSpec,
        require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let mut custom_config = HashMap::new();
        custom_config.insert("vm_size".into(), instance_type.into());
        custom_config.insert("require_tee".into(), require_tee.to_string());
        if let Some(id) = &self.network_interface_id {
            custom_config.insert("network_interface_id".into(), id.clone());
        }
        let infra = self
            .provisioner
            .lock()
            .await
            .provision_instance(
                spec,
                &ProvisioningConfig {
                    name: format!("blueprint-{}", uuid::Uuid::new_v4()),
                    region: region.into(),
                    custom_config,
                    ..Default::default()
                },
            )
            .await?;
        Ok(ProvisionedInstance {
            id: infra.instance_id,
            provider: infra.provider,
            instance_type: infra.instance_type,
            region: infra.region,
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Starting,
        })
    }

    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        self.provisioner.lock().await.get_instance_details(id).await
    }

    async fn lease_endpoint(&self, instance: &ProvisionedInstance) -> Result<String> {
        instance
            .public_ip
            .as_ref()
            .map(|ip| format!("ssh://blueprint@{ip}:22"))
            .ok_or_else(|| Error::ConfigurationError("Azure instance address unavailable".into()))
    }

    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let spec = ResourceSpec {
            cpu: 2.0,
            memory_gb: 8.0,
            storage_gb: 128.0,
            gpu_count: None,
            ssh_public_key: std::env::var("AZURE_SSH_PUBLIC_KEY").ok(),
            allow_spot: false,
            qos: Default::default(),
            ..Default::default()
        };

        let instance_name = format!("blueprint-{}", uuid::Uuid::new_v4());

        let config = ProvisioningConfig {
            name: instance_name.clone(),
            region: region.to_string(),
            ssh_key_name: std::env::var("AZURE_SSH_KEY_NAME").ok(),
            ami_id: None,
            machine_image: None,
            custom_config: {
                let mut config = HashMap::new();
                config.insert("vm_size".to_string(), instance_type.to_string());
                if let Some(id) = &self.network_interface_id {
                    config.insert("network_interface_id".into(), id.clone());
                }
                config.insert(
                    "require_tee".to_string(),
                    if require_tee { "true" } else { "false" }.to_string(),
                );
                config
            },
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
        Ok(self.get_instance_details(instance_id).await?.status)
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
                self.deploy_to_vm(blueprint_image, resource_spec, env_vars)
                    .await
            }
            DeploymentTarget::ManagedKubernetes {
                cluster_id,
                namespace,
            } => {
                #[cfg(feature = "kubernetes")]
                {
                    self.deploy_to_aks(
                        cluster_id,
                        namespace,
                        blueprint_image,
                        resource_spec,
                        env_vars,
                    )
                    .await
                }
                #[cfg(not(feature = "kubernetes"))]
                {
                    warn!(
                        "Kubernetes deployment requested for cluster {} namespace {}, but feature not enabled",
                        cluster_id, namespace
                    );
                    Err(Error::Other("Kubernetes support not enabled".into()))
                }
            }
            DeploymentTarget::GenericKubernetes {
                context: _,
                namespace,
            } => {
                #[cfg(feature = "kubernetes")]
                {
                    self.deploy_to_generic_k8s(namespace, blueprint_image, resource_spec, env_vars)
                        .await
                }
                #[cfg(not(feature = "kubernetes"))]
                {
                    warn!(
                        "Kubernetes deployment requested for namespace {}, but feature not enabled",
                        namespace
                    );
                    Err(Error::Other("Kubernetes support not enabled".into()))
                }
            }
            DeploymentTarget::Serverless { .. } => Err(Error::Other(
                "Azure Container Instances deployment not implemented".into(),
            )),
        }
    }

    async fn deploy_blueprint(
        &self,
        instance: &ProvisionedInstance,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        self.deploy_to_existing_vm(instance, blueprint_image, resource_spec, env_vars)
            .await
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
                        info!(
                            "Azure blueprint {} health check passed",
                            deployment.blueprint_id
                        );
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
        info!(
            "Cleaning up Azure blueprint deployment: {}",
            deployment.blueprint_id
        );
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
        let require_tee = Self::env_require_tee();
        let instance = self
            .provision_instance("Standard_B2ms", "eastus", require_tee)
            .await?;
        self.deploy_to_existing_vm(&instance, blueprint_image, resource_spec, env_vars)
            .await
    }

    async fn deploy_to_existing_vm(
        &self,
        instance: &ProvisionedInstance,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        let public_ip = instance
            .public_ip
            .as_ref()
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

        let ssh_client =
            SshDeploymentClient::new(connection, ContainerRuntime::Docker, deployment_config)
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
    pub async fn deploy_to_aks(
        &self,
        cluster_id: &str,
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        #[cfg(feature = "kubernetes")]
        {
            use crate::shared::{ManagedK8sConfig, SharedKubernetesDeployment};

            let config = ManagedK8sConfig::aks("eastus", "blueprint-resources");
            SharedKubernetesDeployment::deploy_to_managed_k8s(
                cluster_id,
                namespace,
                blueprint_image,
                resource_spec,
                env_vars,
                config,
            )
            .await
        }
        #[cfg(not(feature = "kubernetes"))]
        {
            let _ = (
                cluster_id,
                namespace,
                blueprint_image,
                resource_spec,
                env_vars,
            );
            Err(Error::ConfigurationError(
                "Kubernetes feature not enabled".to_string(),
            ))
        }
    }

    /// Deploy to generic Kubernetes cluster
    pub async fn deploy_to_generic_k8s(
        &self,
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        #[cfg(feature = "kubernetes")]
        {
            use crate::shared::SharedKubernetesDeployment;
            SharedKubernetesDeployment::deploy_to_generic_k8s(
                namespace,
                blueprint_image,
                resource_spec,
                env_vars,
            )
            .await
        }
        #[cfg(not(feature = "kubernetes"))]
        {
            let _ = (namespace, blueprint_image, resource_spec, env_vars);
            Err(Error::ConfigurationError(
                "Kubernetes feature not enabled".to_string(),
            ))
        }
    }
}
