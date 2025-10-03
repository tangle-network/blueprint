//! DigitalOcean CloudProviderAdapter implementation

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::digitalocean::{DigitalOceanProvisioner, Droplet};
use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{info, warn};

/// DigitalOcean adapter for Blueprint deployment
#[derive(Debug)]
pub struct DigitalOceanAdapter {
    provisioner: DigitalOceanProvisioner,
}

impl DigitalOceanAdapter {
    /// Create new DigitalOcean adapter
    pub async fn new() -> Result<Self> {
        let api_token = std::env::var("DIGITALOCEAN_TOKEN")
            .map_err(|_| Error::Other("DIGITALOCEAN_TOKEN environment variable not set".into()))?;

        let default_region = std::env::var("DO_REGION").unwrap_or_else(|_| "nyc3".to_string());

        let provisioner = DigitalOceanProvisioner::new(api_token, default_region).await?;

        Ok(Self {
            provisioner,
        })
    }


    /// Convert Droplet to ProvisionedInstance
    fn droplet_to_instance(droplet: Droplet) -> ProvisionedInstance {
        ProvisionedInstance {
            id: droplet.id.to_string(),
            public_ip: droplet.public_ipv4,
            private_ip: droplet.private_ipv4,
            status: match droplet.status.as_str() {
                "active" => InstanceStatus::Running,
                "new" => InstanceStatus::Starting,
                _ => InstanceStatus::Unknown,
            },
            provider: crate::core::remote::CloudProvider::DigitalOcean,
            region: droplet.region,
            instance_type: droplet.size,
        }
    }

    /// Get SSH username for DigitalOcean droplets
    #[allow(dead_code)]
    fn get_ssh_username(&self) -> &'static str {
        "root"
    }
}

#[async_trait]
impl CloudProviderAdapter for DigitalOceanAdapter {
    async fn provision_instance(
        &self,
        _instance_type: &str,
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

        let droplet_name = format!("blueprint-{}", uuid::Uuid::new_v4());
        let ssh_keys = std::env::var("DO_SSH_KEY_IDS")
            .map(|keys| keys.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_else(|_| vec![]);

        let droplet = self
            .provisioner
            .create_droplet(&droplet_name, &spec, ssh_keys)
            .await?;

        info!(
            "Provisioned DigitalOcean droplet {} in region {}",
            droplet.id, region
        );

        Ok(Self::droplet_to_instance(droplet))
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let droplet_id = instance_id
            .parse::<u64>()
            .map_err(|_| Error::Other("Invalid droplet ID".into()))?;

        self.provisioner.delete_droplet(droplet_id).await
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        let droplet_id = instance_id
            .parse::<u64>()
            .map_err(|_| Error::Other("Invalid droplet ID".into()))?;

        match self.provisioner.get_droplet_status(droplet_id).await {
            Ok(status) => {
                let instance_status = match status.as_str() {
                    "active" => InstanceStatus::Running,
                    "new" => InstanceStatus::Starting,
                    "off" => InstanceStatus::Stopped,
                    _ => InstanceStatus::Unknown,
                };
                info!("DigitalOcean droplet {} status: {}", instance_id, status);
                Ok(instance_status)
            }
            Err(e) => {
                warn!("Failed to get DigitalOcean droplet status: {}", e);
                Ok(InstanceStatus::Unknown)
            }
        }
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
                self.deploy_to_droplet(blueprint_image, resource_spec, env_vars)
                    .await
            }
            DeploymentTarget::ManagedKubernetes {
                cluster_id,
                namespace,
            } => {
                self.deploy_to_doks(
                    cluster_id,
                    namespace,
                    blueprint_image,
                    resource_spec,
                    env_vars,
                )
                .await
            }
            DeploymentTarget::GenericKubernetes {
                context: _,
                namespace,
            } => {
                self.deploy_to_generic_k8s(namespace, blueprint_image, resource_spec, env_vars)
                    .await
            }
            DeploymentTarget::Serverless { .. } => Err(Error::Other(
                "DigitalOcean App Platform deployment not implemented".into(),
            )),
        }
    }

    async fn health_check_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<bool> {
        use crate::security::{ApiAuthentication, SecureHttpClient};

        if let Some(endpoint) = deployment.qos_grpc_endpoint() {
            // Use secure HTTP client for health checks
            let client = SecureHttpClient::new()?;
            let auth = ApiAuthentication::None; // Health endpoint typically doesn't require auth

            match client.get(&format!("{endpoint}/health"), &auth).await {
                Ok(response) => Ok(response.status().is_success()),
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()> {
        info!(
            "Cleaning up DigitalOcean Blueprint deployment: {}",
            deployment.blueprint_id
        );
        // Terminate the Droplet
        self.terminate_instance(&deployment.instance.id).await
    }
}

impl DigitalOceanAdapter {
    /// Deploy to DigitalOcean Droplet via SSH
    async fn deploy_to_droplet(
        &self,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::shared::{SharedSshDeployment, SshDeploymentConfig};

        let instance = self.provision_instance("s-2vcpu-4gb", "nyc3").await?;

        // Use shared SSH deployment with DigitalOcean configuration
        SharedSshDeployment::deploy_to_instance(
            &instance,
            blueprint_image,
            resource_spec,
            env_vars,
            SshDeploymentConfig::digitalocean(),
        ).await
    }

    /// Deploy to DOKS cluster
    async fn deploy_to_doks(
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

            let config = ManagedK8sConfig::doks("nyc3");
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

    #[allow(dead_code)]
    async fn health_check_blueprint(
        &self,
        _deployment: &BlueprintDeploymentResult,
    ) -> Result<bool> {
        Ok(false)
    }

    #[allow(dead_code)]
    async fn cleanup_blueprint(&self, _deployment: &BlueprintDeploymentResult) -> Result<()> {
        Ok(())
    }
}
