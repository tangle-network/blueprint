use crate::error::{Error, Result};
use crate::provisioning::InstanceSelection;
use crate::remote::{CloudProvider, RemoteDeploymentConfig};
use crate::resources::ResourceSpec;
#[cfg(feature = "aws")]
use aws_sdk_ec2::client::Waiters;
use blueprint_std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// Infrastructure provisioner for creating cloud resources
///
/// Handles creation of cloud infrastructure:
/// - EC2 instances for AWS
/// - Compute instances for GCP
/// - VMs for Azure
/// - Droplets for DigitalOcean
/// - Instances for Vultr
pub struct InfrastructureProvisioner {
    provider: CloudProvider,
    #[cfg(feature = "aws")]
    aws_client: Option<aws_sdk_ec2::Client>,
    #[cfg(feature = "aws-eks")]
    eks_client: Option<aws_sdk_eks::Client>,
}

impl InfrastructureProvisioner {
    /// Create a new provisioner for a cloud provider
    pub async fn new(provider: CloudProvider) -> Result<Self> {
        match provider {
            #[cfg(feature = "aws")]
            CloudProvider::AWS => {
                let config = aws_config::load_from_env().await;
                let ec2_client = aws_sdk_ec2::Client::new(&config);
                #[cfg(feature = "aws-eks")]
                let eks_client = aws_sdk_eks::Client::new(&config);
                Ok(Self {
                    provider,
                    aws_client: Some(ec2_client),
                    #[cfg(feature = "aws-eks")]
                    eks_client: Some(eks_client),
                })
            }
            _ => {
                // For providers without SDK support yet
                Ok(Self {
                    provider,
                    #[cfg(feature = "aws")]
                    aws_client: None,
                    #[cfg(feature = "aws-eks")]
                    eks_client: None,
                })
            }
        }
    }

    /// Provision infrastructure based on requirements
    pub async fn provision(
        &self,
        spec: &ResourceSpec,
        config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        info!("Provisioning infrastructure for {:?}", self.provider);

        match self.provider {
            #[cfg(feature = "aws")]
            CloudProvider::AWS => self.provision_aws(spec, config).await,
            CloudProvider::DigitalOcean => self.provision_digitalocean(spec, config).await,
            CloudProvider::Vultr => self.provision_vultr(spec, config).await,
            _ => {
                warn!(
                    "Provider {:?} requires manual infrastructure setup",
                    self.provider
                );
                Err(Error::ConfigurationError(format!(
                    "Automatic provisioning not yet supported for {:?}",
                    self.provider
                )))
            }
        }
    }

    #[cfg(feature = "aws")]
    async fn provision_aws(
        &self,
        spec: &ResourceSpec,
        config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        use aws_sdk_ec2::types::{InstanceType, ResourceType, Tag, TagSpecification};

        let ec2 = self
            .aws_client
            .as_ref()
            .ok_or_else(|| Error::ConfigurationError("AWS client not initialized".into()))?;

        // Map requirements to instance type
        let instance_selection = crate::provisioning::InstanceTypeMapper::map_to_instance_type(
            spec,
            &CloudProvider::AWS,
        );

        // Run EC2 instance
        let result = ec2
            .run_instances()
            .image_id(config.ami_id.as_deref().unwrap_or("ami-0c55b159cbfafe1f0")) // Amazon Linux 2
            .instance_type(InstanceType::from(
                instance_selection.instance_type.as_str(),
            ))
            .min_count(1)
            .max_count(1)
            .key_name(config.ssh_key_name.as_deref().unwrap_or("default"))
            .tag_specifications(
                TagSpecification::builder()
                    .resource_type(ResourceType::Instance)
                    .tags(
                        Tag::builder()
                            .key("Name")
                            .value(format!("blueprint-{}", config.name))
                            .build(),
                    )
                    .tags(
                        Tag::builder()
                            .key("ManagedBy")
                            .value("blueprint-remote")
                            .build(),
                    )
                    .build(),
            )
            .send()
            .await?;

        let instance = result
            .instances()
            .first()
            .ok_or_else(|| Error::ConfigurationError("No instance created".into()))?;

        let instance_id = instance
            .instance_id()
            .ok_or_else(|| Error::ConfigurationError("No instance ID".into()))?;

        info!("Created AWS EC2 instance: {}", instance_id);

        // Wait for instance to be running
        // Wait for instance to be running
        // Note: AWS SDK v1 doesn't have waiters method, need to poll manually
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        let describe_result = ec2
            .describe_instances()
            .instance_ids(instance_id.clone())
            .send()
            .await?;

        // Get public IP
        let describe_result = ec2
            .describe_instances()
            .instance_ids(instance_id)
            .send()
            .await?;

        let public_ip = describe_result
            .reservations()
            .first()
            .and_then(|r| r.instances().first())
            .and_then(|i| i.public_ip_address())
            .unwrap_or("pending");

        Ok(ProvisionedInfrastructure {
            provider: CloudProvider::AWS,
            instance_id: instance_id.to_string(),
            public_ip: Some(public_ip.to_string()),
            private_ip: None,
            instance_type: instance_selection.instance_type,
            cost_per_hour: instance_selection.estimated_hourly_cost,
            metadata: HashMap::new(),
        })
    }

    #[cfg(feature = "aws-eks")]
    pub async fn provision_eks_cluster(
        &self,
        config: &EksClusterConfig,
    ) -> Result<RemoteDeploymentConfig> {
        use aws_sdk_eks::types::{ClusterStatus, NodegroupStatus};

        let eks = self
            .eks_client
            .as_ref()
            .ok_or_else(|| Error::ConfigurationError("EKS client not initialized".into()))?;

        info!("Creating EKS cluster: {}", config.cluster_name);

        // Create EKS cluster
        let cluster_result = eks
            .create_cluster()
            .name(&config.cluster_name)
            .role_arn(&config.cluster_role_arn)
            .resources_vpc_config(|vpc| vpc.subnet_ids(config.subnet_ids.clone()))
            .send()
            .await?;

        // Wait for cluster to be active
        loop {
            let describe = eks
                .describe_cluster()
                .name(&config.cluster_name)
                .send()
                .await?;

            if let Some(cluster) = describe.cluster() {
                match cluster.status() {
                    Some(ClusterStatus::Active) => break,
                    Some(ClusterStatus::Failed) => {
                        return Err(Error::ConfigurationError("Cluster creation failed".into()));
                    }
                    _ => {
                        info!("Waiting for cluster to be active...");
                        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                    }
                }
            }
        }

        // Create node group
        info!("Creating EKS node group");

        let nodegroup_result = eks
            .create_nodegroup()
            .cluster_name(&config.cluster_name)
            .nodegroup_name(format!("{}-nodes", config.cluster_name))
            .node_role(&config.node_role_arn)
            .subnets(config.subnet_ids.clone())
            .scaling_config(|s| {
                s.min_size(config.min_nodes)
                    .max_size(config.max_nodes)
                    .desired_size(config.desired_nodes)
            })
            .instance_types(config.instance_types.clone())
            .send()
            .await?;

        // Generate kubeconfig
        let cluster_endpoint = describe
            .cluster()
            .and_then(|c| c.endpoint())
            .ok_or_else(|| Error::ConfigurationError("No cluster endpoint".into()))?;

        let cluster_ca = describe
            .cluster()
            .and_then(|c| c.certificate_authority())
            .and_then(|ca| ca.data())
            .ok_or_else(|| Error::ConfigurationError("No cluster CA".into()))?;

        Ok(RemoteDeploymentConfig {
            kubeconfig_path: None, // Will be generated
            context: Some(format!(
                "arn:aws:eks:{}:{}:cluster/{}",
                config.region, config.account_id, config.cluster_name
            )),
            namespace: "blueprint".to_string(),
            provider: CloudProvider::AWS,
            region: Some(config.region.clone()),
        })
    }

    async fn provision_digitalocean(
        &self,
        spec: &ResourceSpec,
        config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        // TODO: Implement DigitalOcean API client

        let instance_selection = crate::provisioning::InstanceTypeMapper::map_to_instance_type(
            spec,
            &CloudProvider::DigitalOcean,
        );

        warn!("DigitalOcean provisioning requires API token setup");

        // Example API call structure:
        // POST https://api.digitalocean.com/v2/droplets
        // {
        //   "name": "blueprint-instance",
        //   "region": "nyc3",
        //   "size": instance_selection.instance_type,
        //   "image": "ubuntu-20-04-x64"
        // }

        Err(Error::ConfigurationError(
            "DigitalOcean provisioning requires manual API setup".to_string(),
        ))
    }

    async fn provision_vultr(
        &self,
        spec: &ResourceSpec,
        config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        // Vultr API implementation
        let instance_selection = crate::provisioning::InstanceTypeMapper::map_to_instance_type(
            spec,
            &CloudProvider::Vultr,
        );

        warn!("Vultr provisioning requires API key setup");

        Err(Error::ConfigurationError(
            "Vultr provisioning requires manual API setup".to_string(),
        ))
    }
}

/// Configuration for provisioning infrastructure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisioningConfig {
    pub name: String,
    pub region: String,
    pub availability_zone: Option<String>,
    pub vpc_id: Option<String>,
    pub subnet_id: Option<String>,
    pub security_group_ids: Vec<String>,
    pub ssh_key_name: Option<String>,
    pub ami_id: Option<String>,   // For AWS
    pub image_id: Option<String>, // For other providers
    pub user_data: Option<String>,
    pub tags: HashMap<String, String>,
}

impl Default for ProvisioningConfig {
    fn default() -> Self {
        Self {
            name: "blueprint-instance".to_string(),
            region: "us-west-2".to_string(),
            availability_zone: None,
            vpc_id: None,
            subnet_id: None,
            security_group_ids: Vec::new(),
            ssh_key_name: None,
            ami_id: None,
            image_id: None,
            user_data: None,
            tags: HashMap::new(),
        }
    }
}

/// EKS-specific cluster configuration
#[cfg(feature = "aws-eks")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EksClusterConfig {
    pub cluster_name: String,
    pub region: String,
    pub account_id: String,
    pub cluster_role_arn: String,
    pub node_role_arn: String,
    pub subnet_ids: Vec<String>,
    pub instance_types: Vec<String>,
    pub min_nodes: i32,
    pub max_nodes: i32,
    pub desired_nodes: i32,
}

/// Represents provisioned infrastructure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedInfrastructure {
    pub provider: CloudProvider,
    pub instance_id: String,
    pub public_ip: Option<String>,
    pub private_ip: Option<String>,
    pub instance_type: String,
    pub cost_per_hour: f64,
    pub metadata: HashMap<String, String>,
}

impl ProvisionedInfrastructure {
    /// Check if the infrastructure is ready for deployment
    pub async fn is_ready(&self) -> bool {
        // TODO: Add instance health checks
        self.public_ip.is_some() || self.private_ip.is_some()
    }

    /// Get connection endpoint for this infrastructure
    pub fn get_endpoint(&self) -> Option<String> {
        self.public_ip.clone().or_else(|| self.private_ip.clone())
    }
}

/// Cleanup provisioned infrastructure
pub struct InfrastructureCleanup;

impl InfrastructureCleanup {
    #[cfg(feature = "aws")]
    pub async fn cleanup_aws_instance(instance_id: &str) -> Result<()> {
        let config = aws_config::load_from_env().await;
        let ec2 = aws_sdk_ec2::Client::new(&config);

        ec2.terminate_instances()
            .instance_ids(instance_id)
            .send()
            .await?;

        info!("Terminated AWS instance: {}", instance_id);
        Ok(())
    }

    pub async fn cleanup(infra: &ProvisionedInfrastructure) -> Result<()> {
        match infra.provider {
            #[cfg(feature = "aws")]
            CloudProvider::AWS => Self::cleanup_aws_instance(&infra.instance_id).await,
            _ => {
                warn!("Manual cleanup required for {:?}", infra.provider);
                Ok(())
            }
        }
    }
}
