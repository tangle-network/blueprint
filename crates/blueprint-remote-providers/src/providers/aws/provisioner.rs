//! AWS EC2 instance provisioning

use super::instance_mapper::AwsInstanceMapper;
use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::providers::common::{ProvisionedInfrastructure, ProvisioningConfig};
#[cfg(feature = "aws")]
use aws_sdk_ec2::types::{InstanceType, ResourceType, Tag, TagSpecification};
use tracing::info;

/// AWS EC2 provisioner
pub struct AwsProvisioner {
    pub(crate) ec2_client: aws_sdk_ec2::Client,
    #[cfg(feature = "aws-eks")]
    pub(crate) eks_client: Option<aws_sdk_eks::Client>,
}

impl AwsProvisioner {
    /// Create a new AWS provisioner
    pub async fn new() -> Result<Self> {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let ec2_client = aws_sdk_ec2::Client::new(&config);

        #[cfg(feature = "aws-eks")]
        let eks_client = Some(aws_sdk_eks::Client::new(&config));

        Ok(Self {
            ec2_client,
            #[cfg(feature = "aws-eks")]
            eks_client,
        })
    }


    /// Provision an EC2 instance
    pub async fn provision_instance(
        &self,
        spec: &ResourceSpec,
        config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        // Map requirements to instance type
        let instance_selection = AwsInstanceMapper::map(spec);

        // Run EC2 instance
        let result = self
            .ec2_client
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
                    .tags(Tag::builder().key("Name").value(&config.name).build())
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
        let describe_result = self
            .ec2_client
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
            provider: crate::core::remote::CloudProvider::AWS,
            instance_id: instance_id.to_string(),
            public_ip,
            private_ip,
            region: config.region.clone(),
            instance_type: instance_selection.instance_type,
            metadata: Default::default(),
        })
    }


    /// Terminate an EC2 instance
    pub async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        self.ec2_client
            .terminate_instances()
            .instance_ids(instance_id)
            .send()
            .await?;

        info!("Terminated AWS EC2 instance: {}", instance_id);
        Ok(())
    }


    /// Get instance status
    pub async fn get_instance_status(&self, instance_id: &str) -> Result<crate::infra::types::InstanceStatus> {
        let describe_result = self.ec2_client
            .describe_instances()
            .instance_ids(instance_id)
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to describe instance: {}", e)))?;

        let instance = describe_result
            .reservations()
            .first()
            .and_then(|r| r.instances().first())
            .ok_or_else(|| Error::ConfigurationError("Instance not found".into()))?;

        let state_name = instance.state()
            .and_then(|s| s.name())
            .map(|n| format!("{:?}", n))
            .unwrap_or_else(|| "unknown".to_string());

        match state_name.to_lowercase().as_str() {
            "running" => Ok(crate::infra::types::InstanceStatus::Running),
            "pending" => Ok(crate::infra::types::InstanceStatus::Starting),
            "stopping" | "stopped" | "terminated" => Ok(crate::infra::types::InstanceStatus::Terminated),
            _ => Ok(crate::infra::types::InstanceStatus::Unknown),
        }
    }


    /// Create security group
    pub async fn create_security_group(&self, sg_name: &str) -> Result<String> {
        use aws_sdk_ec2::types::{IpPermission, IpRange};

        let create_result = self.ec2_client
            .create_security_group()
            .group_name(sg_name)
            .description("Blueprint remote providers security group - SSH and QoS ports")
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to create security group: {}", e)))?;

        let sg_id = create_result.group_id().unwrap_or("").to_string();

        // Add inbound rules: SSH (22), QoS (8080, 9615, 9944)
        let ssh_rule = IpPermission::builder()
            .ip_protocol("tcp")
            .from_port(22)
            .to_port(22)
            .ip_ranges(IpRange::builder().cidr_ip("0.0.0.0/0").build())
            .build();

        let qos_rule = IpPermission::builder()
            .ip_protocol("tcp")
            .from_port(8080)
            .to_port(9944)
            .ip_ranges(IpRange::builder().cidr_ip("0.0.0.0/0").build())
            .build();

        let _ = self.ec2_client
            .authorize_security_group_ingress()
            .group_id(&sg_id)
            .ip_permissions(ssh_rule)
            .ip_permissions(qos_rule)
            .send()
            .await;

        Ok(sg_id)
    }

}
