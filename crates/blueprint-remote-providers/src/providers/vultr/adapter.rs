//! Vultr CloudProviderAdapter implementation

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::vultr::provisioner::VultrProvisioner;
use crate::providers::common::ProvisioningConfig;
use crate::deployment::ssh::{ContainerRuntime, DeploymentConfig, SshConnection, SshDeploymentClient};
use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{info, warn};

/// Vultr adapter for Blueprint deployment
pub struct VultrAdapter {
    provisioner: VultrProvisioner,
    #[allow(dead_code)]
    api_key: String,
}

impl VultrAdapter {
    /// Create new Vultr adapter
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("VULTR_API_KEY")
            .map_err(|_| Error::Other("VULTR_API_KEY environment variable not set".into()))?;

        let provisioner = VultrProvisioner::new(api_key.clone()).await?;
        Ok(Self { api_key, provisioner })
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

        let instance_name = format!("blueprint-{}", uuid::Uuid::new_v4());

        let config = ProvisioningConfig {
            name: instance_name.clone(),
            region: region.to_string(),
            ssh_key_name: std::env::var("VULTR_SSH_KEY_ID").ok(),
            ami_id: None,
            machine_image: None,
            custom_config: HashMap::new(),
        };

        let infra = self.provisioner.provision_instance(&spec, &config).await?;

        info!(
            "Provisioned Vultr instance {} in region {}",
            infra.instance_id, region
        );

        Ok(ProvisionedInstance {
            id: infra.instance_id,
            public_ip: infra.public_ip,
            private_ip: infra.private_ip,
            status: InstanceStatus::Running,
            provider: crate::core::remote::CloudProvider::Vultr,
            region: infra.region,
            instance_type: infra.instance_type,
        })
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        self.provisioner.terminate_instance(instance_id).await
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        self.provisioner.get_instance_status(instance_id).await
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
                #[cfg(feature = "kubernetes")]
                {
                    self.deploy_to_vke(cluster_id, namespace, blueprint_image, resource_spec, env_vars).await
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
                Err(Error::Other("Vultr serverless deployment not implemented".into()))
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
                        info!("Vultr blueprint {} health check passed", deployment.blueprint_id);
                    }
                    Ok(healthy)
                }
                Err(e) => {
                    warn!("Vultr health check failed: {}", e);
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }

    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()> {
        info!("Cleaning up Vultr blueprint deployment: {}", deployment.blueprint_id);
        self.terminate_instance(&deployment.instance.id).await
    }
}

// Private helper methods
impl VultrAdapter {
    /// Deploy to Vultr instance via SSH
    async fn deploy_to_instance(
        &self,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        let instance = self.provision_instance("vc2-2c-4gb", "ewr").await?;
        let public_ip = instance.public_ip.as_ref()
            .ok_or_else(|| Error::Other("Instance has no public IP".into()))?;

        // SSH connection configuration
        let connection = SshConnection {
            host: public_ip.clone(),
            user: self.get_ssh_username().to_string(),
            key_path: std::env::var("VULTR_SSH_KEY_PATH").ok().map(|p| p.into()),
            port: 22,
            password: None,
            jump_host: None,
        };

        let deployment_config = DeploymentConfig {
            name: format!("blueprint-{}", uuid::Uuid::new_v4()),
            namespace: "blueprint-vultr".to_string(),
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
        metadata.insert("provider".to_string(), "vultr-instance".to_string());
        metadata.insert("container_id".to_string(), deployment.container_id.clone());
        metadata.insert("ssh_host".to_string(), deployment.host.clone());

        info!(
            "Successfully deployed blueprint {} to Vultr instance {}",
            deployment.container_id, instance.id
        );

        Ok(BlueprintDeploymentResult {
            instance: instance.clone(),
            blueprint_id: deployment.container_id,
            port_mappings,
            metadata,
        })
    }

    /// Deploy to VKE cluster
    #[cfg(feature = "kubernetes")]
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
    #[cfg(feature = "kubernetes")]
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
}