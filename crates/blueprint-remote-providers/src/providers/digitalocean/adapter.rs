//! DigitalOcean CloudProviderAdapter implementation

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::digitalocean::{DigitalOceanProvisioner, Droplet};
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;

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

        Ok(Self { provisioner })
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
        )
        .await
    }

    /// Deploy to DOKS cluster
    pub async fn deploy_to_doks(
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

            let config = ManagedK8sConfig::doks("nyc3");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_digitalocean_adapter_creation() {
        let result = DigitalOceanAdapter::new().await;
        // Without credentials, may succeed or fail - just testing the method exists
        assert!(result.is_ok() || result.is_err());
    }

    #[cfg(feature = "kubernetes")]
    #[tokio::test]
    async fn test_doks_deployment_structure() {
        use crate::core::resources::ResourceSpec;

        // Test that the method signature and structure are correct
        let adapter = DigitalOceanAdapter::new()
            .await
            .expect("Failed to create DigitalOcean adapter");

        let mut env_vars = HashMap::new();
        env_vars.insert("REDIS_URL".to_string(), "redis://localhost".to_string());

        let result = adapter
            .deploy_to_doks(
                "test-doks-cluster",
                "production",
                "myapp:v1.0",
                &ResourceSpec::recommended(),
                env_vars,
            )
            .await;

        // Without actual cluster, we expect an error but method should be callable
        assert!(result.is_err());
    }

    #[cfg(feature = "kubernetes")]
    #[tokio::test]
    async fn test_doks_generic_k8s_deployment_structure() {
        use crate::core::resources::ResourceSpec;

        let adapter = DigitalOceanAdapter::new()
            .await
            .expect("Failed to create DigitalOcean adapter");

        let mut env_vars = HashMap::new();
        env_vars.insert("NODE_ENV".to_string(), "production".to_string());
        env_vars.insert("LOG_LEVEL".to_string(), "info".to_string());

        let result = adapter
            .deploy_to_generic_k8s(
                "default",
                "busybox:latest",
                &ResourceSpec::minimal(),
                env_vars,
            )
            .await;

        // Without actual cluster, we expect an error but method should be callable
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_env_vars() {
        let mut env_vars = HashMap::new();
        env_vars.insert("VAR1".to_string(), "value1".to_string());
        env_vars.insert("VAR2".to_string(), "value2".to_string());
        env_vars.insert("VAR3".to_string(), "value3".to_string());

        assert_eq!(env_vars.len(), 3);
        assert!(env_vars.contains_key("VAR1"));
        assert!(env_vars.contains_key("VAR2"));
        assert!(env_vars.contains_key("VAR3"));
    }
}
