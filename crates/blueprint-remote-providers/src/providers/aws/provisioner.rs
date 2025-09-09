//! AWS EC2 instance provisioning

use crate::error::{Error, Result};
use crate::providers::common::{ProvisionedInfrastructure, ProvisioningConfig};
use crate::resources::ResourceSpec;
use super::instance_mapper::AwsInstanceMapper;
#[cfg(feature = "aws")]
use aws_sdk_ec2::types::{InstanceType, ResourceType, Tag, TagSpecification};
use tracing::{info, warn};

/// AWS EC2 provisioner
pub struct AwsProvisioner {
    #[cfg(feature = "aws")]
    ec2_client: aws_sdk_ec2::Client,
    #[cfg(feature = "aws-eks")]
    eks_client: Option<aws_sdk_eks::Client>,
}

impl AwsProvisioner {
    /// Create a new AWS provisioner
    #[cfg(feature = "aws")]
    pub async fn new() -> Result<Self> {
        let config = aws_config::load_from_env().await;
        let ec2_client = aws_sdk_ec2::Client::new(&config);
        
        #[cfg(feature = "aws-eks")]
        let eks_client = Some(aws_sdk_eks::Client::new(&config));
        
        Ok(Self {
            ec2_client,
            #[cfg(feature = "aws-eks")]
            eks_client,
        })
    }
    
    #[cfg(not(feature = "aws"))]
    pub async fn new() -> Result<Self> {
        Err(Error::ConfigurationError(
            "AWS support not enabled. Enable the 'aws' feature".into()
        ))
    }

    /// Provision an EC2 instance
    #[cfg(feature = "aws")]
    pub async fn provision_instance(
        &self,
        spec: &ResourceSpec,
        config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        // Map requirements to instance type
        let instance_selection = AwsInstanceMapper::map(spec);

        // Run EC2 instance
        let result = self.ec2_client
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
                            .value(&config.name)
                            .build(),
                    )
                    .tags(
                        Tag::builder()
                            .key("BlueprintDeployment")
                            .value("true")
                            .build(),
                    )
                    .tags(
                        Tag::builder()
                            .key("Provider")
                            .value("blueprint-remote-providers")
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
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        // Get instance details
        let describe_result = self.ec2_client
            .describe_instances()
            .instance_ids(instance_id)
            .send()
            .await?;

        let reservation = describe_result
            .reservations()
            .first()
            .ok_or_else(|| Error::ConfigurationError("No reservation found".into()))?;

        let instance = reservation
            .instances()
            .first()
            .ok_or_else(|| Error::ConfigurationError("No instance found".into()))?;

        let public_ip = instance.public_ip_address().map(|s| s.to_string());
        let private_ip = instance.private_ip_address().map(|s| s.to_string());

        Ok(ProvisionedInfrastructure {
            provider: crate::remote::CloudProvider::AWS,
            instance_id: instance_id.to_string(),
            public_ip,
            private_ip,
            region: config.region.clone(),
            instance_type: instance_selection.instance_type,
            metadata: Default::default(),
        })
    }
    
    #[cfg(not(feature = "aws"))]
    pub async fn provision_instance(
        &self,
        _spec: &ResourceSpec,
        _config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        Err(Error::ConfigurationError(
            "AWS support not enabled. Enable the 'aws' feature".into()
        ))
    }

    /// Terminate an EC2 instance
    #[cfg(feature = "aws")]
    pub async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        self.ec2_client
            .terminate_instances()
            .instance_ids(instance_id)
            .send()
            .await?;
        
        info!("Terminated AWS EC2 instance: {}", instance_id);
        Ok(())
    }
    
    #[cfg(not(feature = "aws"))]
    pub async fn terminate_instance(&self, _instance_id: &str) -> Result<()> {
        Err(Error::ConfigurationError(
            "AWS support not enabled. Enable the 'aws' feature".into()
        ))
    }
}