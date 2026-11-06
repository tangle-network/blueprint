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
    pricing: crate::pricing::PricingFetcher,
}

impl DigitalOceanAdapter {
    /// Create new DigitalOcean adapter
    pub async fn new() -> Result<Self> {
        let api_token = std::env::var("DIGITALOCEAN_TOKEN")
            .map_err(|_| Error::Other("DIGITALOCEAN_TOKEN environment variable not set".into()))?;

        let default_region = std::env::var("DO_REGION").unwrap_or_else(|_| "nyc3".to_string());

        Self::with_credentials(api_token, default_region)
    }

    pub fn with_credentials(api_token: String, region: String) -> Result<Self> {
        let pricing = crate::pricing::PricingFetcher::new()
            .and_then(|p| p.with_token(crate::CloudProvider::DigitalOcean, api_token.clone()))
            .map_err(|e| Error::ConfigurationError(e.to_string()))?;
        Ok(Self {
            provisioner: DigitalOceanProvisioner::with_credentials(api_token, region)?,
            pricing,
        })
    }

    pub fn with_endpoint(api_token: String, region: String, endpoint: &str) -> Result<Self> {
        let mut adapter = Self::with_credentials(api_token.clone(), region.clone())?;
        adapter.provisioner = DigitalOceanProvisioner::with_endpoint(api_token, region, endpoint)?;
        adapter.pricing = adapter
            .pricing
            .with_endpoint(crate::CloudProvider::DigitalOcean, endpoint)
            .map_err(|e| Error::ConfigurationError(e.to_string()))?;
        Ok(adapter)
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
                "deleted" => InstanceStatus::Terminated,
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
    async fn quote(
        &self,
        spec: &ResourceSpec,
        region: &str,
    ) -> Result<crate::providers::common::InstanceSelection> {
        spec.validate().map_err(Error::ConfigurationError)?;
        let sku = self.provisioner.select_droplet_size(spec);
        let price = self
            .pricing
            .quote_instance(crate::CloudProvider::DigitalOcean, region, &sku)
            .await
            .map_err(|e| Error::HttpError(e.to_string()))?
            .ok_or_else(|| {
                Error::ConfigurationError(
                    "DigitalOcean exact SKU is unpriced or unavailable".into(),
                )
            })?;
        crate::providers::common::gpu_adapter::quoted_selection(sku, price)
    }

    async fn provision_resources(
        &self,
        instance_type: &str,
        region: &str,
        spec: &ResourceSpec,
        require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        spec.validate().map_err(Error::ConfigurationError)?;
        if require_tee {
            return Err(Error::ConfigurationError(
                "DigitalOcean TEE leases are unsupported".into(),
            ));
        }
        if spec.ssh_public_key.is_none() {
            return Err(Error::ConfigurationError(
                "DigitalOcean lease SSH key required".into(),
            ));
        }
        let instance = self
            .provisioner
            .create_droplet_with_size(
                &format!("blueprint-{}", uuid::Uuid::new_v4()),
                spec,
                vec![],
                instance_type,
                region,
            )
            .await?;
        Ok(Self::droplet_to_instance(instance))
    }

    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        let id = id
            .parse::<u64>()
            .map_err(|_| Error::ConfigurationError("Invalid Droplet id".into()))?;
        Ok(Self::droplet_to_instance(
            self.provisioner.get_droplet_details(id).await?,
        ))
    }

    async fn provision_instance(
        &self,
        _instance_type: &str,
        region: &str,
        require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        // Fail closed at the trait boundary: this provider has no
        // confidential-compute / TEE attestation path. Refuse rather than
        // silently return a non-TEE instance for a require_tee request.
        if require_tee {
            return Err(Error::ConfigurationError(
                "DigitalOcean does not support confidential-compute / TEE attestation; \
                 refusing require_tee"
                    .into(),
            ));
        }
        let spec = ResourceSpec {
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 80.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
            ..Default::default()
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

        Ok(
            Self::droplet_to_instance(self.provisioner.get_droplet_details(droplet_id).await?)
                .status,
        )
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

    async fn deploy_blueprint(
        &self,
        instance: &ProvisionedInstance,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::shared::{SharedSshDeployment, SshDeploymentConfig};
        SharedSshDeployment::deploy_to_instance(
            instance,
            blueprint_image,
            resource_spec,
            env_vars,
            SshDeploymentConfig::digitalocean(),
        )
        .await
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
        let instance = self
            .provision_instance("s-2vcpu-4gb", "nyc3", false)
            .await?;
        self.deploy_blueprint(&instance, blueprint_image, resource_spec, env_vars)
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
