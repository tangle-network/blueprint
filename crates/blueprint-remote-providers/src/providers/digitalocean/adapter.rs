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
    fn get_ssh_username(&self) -> &'static str {
        "root"
    }
}

#[async_trait]
impl CloudProviderAdapter for DigitalOceanAdapter {
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

        let droplet_name = format!("blueprint-{}", uuid::Uuid::new_v4());
        let ssh_keys = vec![]; // TODO: Add SSH key management

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
        info!("Checking status for DigitalOcean droplet: {}", instance_id);
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
        if let Some(endpoint) = deployment.qos_grpc_endpoint() {
            match reqwest::get(&format!("{}/health", endpoint)).await {
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
        let instance = self.provision_instance("s-2vcpu-4gb", "nyc3").await?;

        // TODO: Implement SSH deployment similar to AWS
        warn!("DigitalOcean Droplet SSH deployment not fully implemented");

        let mut port_mappings = HashMap::new();
        port_mappings.insert(8080, 8080);
        port_mappings.insert(9615, 9615);
        port_mappings.insert(9944, 9944);

        let mut metadata = HashMap::new();
        metadata.insert("provider".to_string(), "digitalocean-droplet".to_string());

        Ok(BlueprintDeploymentResult {
            instance,
            blueprint_id: format!("droplet-{}", uuid::Uuid::new_v4()),
            port_mappings,
            metadata,
        })
    }

    /// Deploy to DOKS cluster
    async fn deploy_to_doks(
        &self,
        cluster_id: &str,
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::deployment::kubernetes::KubernetesDeploymentClient;

        info!("Deploying to DOKS cluster: {}", cluster_id);

        let k8s_client = KubernetesDeploymentClient::new(Some(namespace.to_string())).await?;
        let (deployment_id, exposed_ports) = k8s_client
            .deploy_blueprint("blueprint", blueprint_image, resource_spec, 1)
            .await?;

        let mut port_mappings = HashMap::new();
        for port in exposed_ports {
            port_mappings.insert(port, port);
        }

        let mut metadata = HashMap::new();
        metadata.insert("provider".to_string(), "digitalocean-doks".to_string());
        metadata.insert("cluster_id".to_string(), cluster_id.to_string());
        metadata.insert("namespace".to_string(), namespace.to_string());

        let instance = ProvisionedInstance {
            id: format!("doks-{}", cluster_id),
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Running,
            provider: crate::core::remote::CloudProvider::DigitalOcean,
            region: "nyc3".to_string(),
            instance_type: "doks-cluster".to_string(),
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

    async fn health_check_blueprint(
        &self,
        _deployment: &BlueprintDeploymentResult,
    ) -> Result<bool> {
        Ok(false)
    }

    async fn cleanup_blueprint(&self, _deployment: &BlueprintDeploymentResult) -> Result<()> {
        Ok(())
    }
}
