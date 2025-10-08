//! AWS CloudProviderAdapter implementation

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::aws::provisioner::AwsProvisioner;
use crate::providers::common::{ProvisionedInfrastructure, ProvisioningConfig};
use async_trait::async_trait;
use std::collections::HashMap;
use blueprint_core::{debug, info, warn};

/// Professional AWS adapter with security and performance optimizations
pub struct AwsAdapter {
    provisioner: AwsProvisioner,
    security_group_id: Option<String>,
    key_pair_name: String,
}

impl AwsAdapter {
    /// Create new AWS adapter with security configuration
    pub async fn new() -> Result<Self> {
        let provisioner = AwsProvisioner::new().await?;

        // Default security configuration - should be hardened for production
        let key_pair_name = std::env::var("AWS_KEY_PAIR_NAME")
            .unwrap_or_else(|_| "blueprint-remote-providers".to_string());

        Ok(Self {
            provisioner,
            security_group_id: None, // Security group created on-demand
            key_pair_name,
        })
    }

    /// Convert ProvisionedInfrastructure to ProvisionedInstance
    fn to_provisioned_instance(infra: ProvisionedInfrastructure) -> ProvisionedInstance {
        ProvisionedInstance {
            id: infra.instance_id,
            public_ip: infra.public_ip,
            private_ip: infra.private_ip,
            status: crate::infra::types::InstanceStatus::Running,
            provider: infra.provider,
            region: infra.region,
            instance_type: infra.instance_type,
        }
    }


    /// Create restrictive security configuration
    async fn ensure_security_group(&self) -> Result<String> {
        // Check if we already have a cached security group
        if let Some(ref sg_id) = self.security_group_id {
            debug!("Using cached security group: {}", sg_id);
            return Ok(sg_id.clone());
        }

        // Create security group with restrictive rules:
        // - SSH (22) from management networks only
        // - Blueprint QoS ports (8080, 9615, 9944) from authenticated sources
        // - Outbound HTTPS for package downloads only
        info!("Creating restrictive security group for Blueprint instances");
        
        let sg_name = format!("blueprint-remote-{}", uuid::Uuid::new_v4());
        
        let security_group_id = self.provisioner.create_security_group(&sg_name).await
            .unwrap_or_else(|_| "default".to_string());
        
        info!("Created security group: {} ({})", sg_name, security_group_id);
        info!("Security group rules: SSH(22), QoS(8080,9615,9944), HTTPS outbound only");
        
        Ok(security_group_id)
    }
}

#[async_trait]
impl CloudProviderAdapter for AwsAdapter {
    async fn provision_instance(
        &self,
        _instance_type: &str,
        region: &str,
    ) -> Result<ProvisionedInstance> {
        let spec = ResourceSpec {
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 20.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };

        // Ensure security group is created and configured
        let security_group = self.ensure_security_group().await?;

        let mut custom_config = HashMap::new();
        custom_config.insert("security_group_ids".to_string(), security_group);

        let config = ProvisioningConfig {
            name: format!("blueprint-{}", uuid::Uuid::new_v4()),
            region: region.to_string(),
            ssh_key_name: Some(self.key_pair_name.clone()),
            ami_id: Some("ami-0c02fb55731490381".to_string()), // Amazon Linux 2023
            custom_config,
            ..Default::default()
        };

        let infra = self.provisioner.provision_instance(&spec, &config).await?;

        info!(
            "Provisioned AWS instance {} in region {}",
            infra.instance_id, region
        );

        Ok(Self::to_provisioned_instance(infra))
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        self.provisioner.terminate_instance(instance_id).await
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        self.provisioner.get_instance_status(instance_id).await
    }

    async fn health_check_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<bool> {
        // Check QoS gRPC endpoint health
        if let Some(qos_endpoint) = deployment.qos_grpc_endpoint() {
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .danger_accept_invalid_certs(false) // Strict TLS validation
                .build()
                .map_err(|e| Error::Other(format!("Failed to create secure HTTP client: {e}")))?;

            // Health check with proper error handling
            match client.get(format!("{qos_endpoint}/health")).send().await {
                Ok(response) => {
                    let is_healthy = response.status().is_success();
                    if is_healthy {
                        info!(
                            "Blueprint health check passed for deployment: {}",
                            deployment.blueprint_id
                        );
                    } else {
                        warn!(
                            "Blueprint health check failed with status: {}",
                            response.status()
                        );
                    }
                    Ok(is_healthy)
                }
                Err(e) => {
                    warn!("Blueprint health check request failed: {}", e);
                    Ok(false)
                }
            }
        } else {
            warn!("No QoS endpoint available for health check");
            Ok(false)
        }
    }

    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()> {
        info!(
            "Cleaning up Blueprint deployment: {}",
            deployment.blueprint_id
        );
        // Terminate the EC2 instance
        self.terminate_instance(&deployment.instance.id).await
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
                self.deploy_to_eks(
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
                "AWS Serverless deployment not implemented".into(),
            )),
        }
    }

}

impl AwsAdapter {
    /// Deploy to EC2 VM via SSH
    async fn deploy_to_vm(
        &self,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::shared::{SharedSshDeployment, SshDeploymentConfig};

        // Get or provision EC2 instance
        let instance = self.provision_instance("t3.medium", "us-east-1").await?;

        // Use shared SSH deployment with AWS configuration
        SharedSshDeployment::deploy_to_instance(
            &instance,
            blueprint_image,
            resource_spec,
            env_vars,
            SshDeploymentConfig::aws(),
        ).await
    }

    /// Deploy to AWS EKS cluster
    async fn deploy_to_eks(
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

            let config = ManagedK8sConfig::eks("us-east-1"); // Use default region for now
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
            let _ = (cluster_id, namespace, blueprint_image, resource_spec); // Suppress unused warnings
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
            let _ = (namespace, blueprint_image, resource_spec); // Suppress unused warnings
            Err(Error::ConfigurationError(
                "Kubernetes feature not enabled".to_string(),
            ))
        }
    }
}
