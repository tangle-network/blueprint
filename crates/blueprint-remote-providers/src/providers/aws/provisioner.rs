//! AWS EC2 instance provisioning

use super::instance_mapper::AwsInstanceMapper;
use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::providers::common::{ProvisionedInfrastructure, ProvisioningConfig};
#[cfg(feature = "aws")]
use aws_sdk_ec2::types::{InstanceType, ResourceType, Tag, TagSpecification};
use blueprint_core::{info, warn};

/// AWS EC2 provisioner
pub struct AwsProvisioner {
    pub(crate) ec2_client: aws_sdk_ec2::Client,
    #[cfg(feature = "aws-eks")]
    #[allow(
        dead_code,
        reason = "constructed for EKS provisioning paths that are not yet wired up"
    )]
    pub(crate) eks_client: Option<aws_sdk_eks::Client>,
}

impl AwsProvisioner {
    /// Create a new AWS provisioner
    pub async fn new() -> Result<Self> {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        Ok(Self::with_config(&config))
    }

    pub fn with_config(config: &aws_config::SdkConfig) -> Self {
        Self {
            ec2_client: aws_sdk_ec2::Client::new(config),
            #[cfg(feature = "aws-eks")]
            eks_client: Some(aws_sdk_eks::Client::new(config)),
        }
    }

    /// Provision an EC2 instance
    pub async fn provision_instance(
        &self,
        spec: &ResourceSpec,
        config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        // Map requirements to instance type
        let mut instance_selection = AwsInstanceMapper::map(spec);
        if let Some(override_type) = config.custom_config.get("instance_type") {
            instance_selection.instance_type = override_type.clone();
        }
        if spec.gpu_count.is_some()
            && !crate::providers::common::gpu_adapter::catalog_skus(spec, "aws")?
                .iter()
                .any(|native| native == &instance_selection.instance_type)
        {
            return Err(Error::ConfigurationError(
                "GPU request does not match the provider SKU".into(),
            ));
        }
        let require_tee = config
            .custom_config
            .get("require_tee")
            .is_some_and(|value| value.eq_ignore_ascii_case("true"));

        spec.validate().map_err(Error::ConfigurationError)?;
        if require_tee {
            return Err(Error::ConfigurationError("AWS Nitro evidence must originate inside the enclave; remote lease attestation is unavailable".into()));
        }
        if self.ec2_client.config().region().map(|r| r.as_ref()) != Some(config.region.as_str()) {
            return Err(Error::ConfigurationError(
                "AWS client region differs from allocation region".into(),
            ));
        }
        // Run EC2 instance
        let mut request = self
            .ec2_client
            .run_instances()
            .image_id(config.ami_id.as_deref().unwrap_or("ami-0c55b159cbfafe1f0")) // Amazon Linux 2
            .instance_type(InstanceType::from(
                instance_selection.instance_type.as_str(),
            ))
            .min_count(1)
            .max_count(1)
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
            );
        if let Some(key) = &spec.ssh_public_key {
            use base64::Engine;
            let cloud_init = format!(
                "#cloud-config\n{}",
                serde_json::json!({"users":[{"name":"blueprint","shell":"/bin/bash",
                "sudo":"ALL=(ALL) NOPASSWD:ALL","ssh_authorized_keys":[key]}],"ssh_pwauth":false})
            );
            request =
                request.user_data(base64::engine::general_purpose::STANDARD.encode(cloud_init));
        } else if let Some(key) = &config.ssh_key_name {
            request = request.key_name(key);
        }
        if let Some(groups) = config.custom_config.get("security_group_ids") {
            request = request
                .set_security_group_ids(Some(groups.split(',').map(str::to_owned).collect()));
        }
        let result = request.send().await?;

        let instance = result
            .instances()
            .first()
            .ok_or_else(|| Error::ConfigurationError("No instance created".into()))?;

        let instance_id = instance
            .instance_id()
            .ok_or_else(|| Error::ConfigurationError("No instance ID".into()))?;

        info!("Created AWS EC2 instance: {}", instance_id);

        let public_ip = instance.public_ip_address().map(str::to_owned);
        let private_ip = instance.private_ip_address().map(str::to_owned);
        let metadata = std::collections::HashMap::new();
        Ok(ProvisionedInfrastructure {
            provider: crate::core::remote::CloudProvider::AWS,
            instance_id: instance_id.to_string(),
            public_ip,
            private_ip,
            region: config.region.clone(),
            instance_type: instance_selection.instance_type,
            metadata,
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
    pub async fn get_instance_status(
        &self,
        instance_id: &str,
    ) -> Result<crate::infra::types::InstanceStatus> {
        let describe_result = self
            .ec2_client
            .describe_instances()
            .instance_ids(instance_id)
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to describe instance: {e}")))?;

        let instance = describe_result
            .reservations()
            .first()
            .and_then(|r| r.instances().first())
            .ok_or_else(|| Error::ConfigurationError("Instance not found".into()))?;

        let state_name = instance
            .state()
            .and_then(|s| s.name())
            .map(|n| format!("{n:?}"))
            .unwrap_or_else(|| "unknown".to_string());

        match state_name.to_lowercase().as_str() {
            "running" => Ok(crate::infra::types::InstanceStatus::Running),
            "pending" => Ok(crate::infra::types::InstanceStatus::Starting),
            "stopping" | "stopped" | "terminated" => {
                Ok(crate::infra::types::InstanceStatus::Terminated)
            }
            _ => Ok(crate::infra::types::InstanceStatus::Unknown),
        }
    }

    pub async fn get_instance_details(
        &self,
        id: &str,
    ) -> Result<crate::infra::types::ProvisionedInstance> {
        let output = self
            .ec2_client
            .describe_instances()
            .instance_ids(id)
            .send()
            .await
            .map_err(|_| Error::HttpError("AWS instance details unavailable".into()))?;
        let instance = output
            .reservations()
            .iter()
            .flat_map(|r| r.instances())
            .find(|i| i.instance_id() == Some(id))
            .ok_or_else(|| Error::HttpError("AWS instance missing".into()))?;
        let status = match instance.state().and_then(|s| s.name()).map(|s| s.as_str()) {
            Some("running") => crate::infra::types::InstanceStatus::Running,
            Some("pending") => crate::infra::types::InstanceStatus::Starting,
            Some("terminated") => crate::infra::types::InstanceStatus::Terminated,
            Some("stopped" | "stopping") => crate::infra::types::InstanceStatus::Stopped,
            _ => crate::infra::types::InstanceStatus::Unknown,
        };
        Ok(crate::infra::types::ProvisionedInstance {
            id: id.into(),
            provider: crate::core::remote::CloudProvider::AWS,
            instance_type: instance
                .instance_type()
                .map(|t| t.as_str())
                .unwrap_or_default()
                .into(),
            region: self
                .ec2_client
                .config()
                .region()
                .map(|r| r.as_ref())
                .unwrap_or_default()
                .into(),
            public_ip: instance.public_ip_address().map(str::to_owned),
            private_ip: instance.private_ip_address().map(str::to_owned),
            status,
        })
    }

    /// Create security group
    pub async fn create_security_group(&self, sg_name: &str) -> Result<String> {
        use aws_sdk_ec2::types::{IpPermission, IpRange};

        let create_result = self
            .ec2_client
            .create_security_group()
            .group_name(sg_name)
            .description("Blueprint remote providers security group - SSH and QoS ports")
            .send()
            .await
            .map_err(|e| {
                Error::ConfigurationError(format!("Failed to create security group: {e}"))
            })?;

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

        match self
            .ec2_client
            .authorize_security_group_ingress()
            .group_id(&sg_id)
            .ip_permissions(ssh_rule)
            .ip_permissions(qos_rule)
            .send()
            .await
        {
            Ok(_) => info!("Security group {} configured with ingress rules", sg_id),
            Err(e) => warn!("Failed to configure security group rules: {}", e),
        }

        Ok(sg_id)
    }
}
