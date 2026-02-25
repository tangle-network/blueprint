//! Cloud VM cleanup handlers

use super::super::types::{CleanupHandler, DeploymentRecord};
use crate::core::error::{Error, Result};
use blueprint_core::info;

/// AWS cleanup
pub(crate) struct AwsCleanup;

#[async_trait::async_trait]
impl CleanupHandler for AwsCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        #[cfg(feature = "aws")]
        {
            let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
            let ec2 = aws_sdk_ec2::Client::new(&config);

            if let Some(instance_id) = deployment.resource_ids.get("instance_id") {
                info!("Terminating AWS EC2 instance: {}", instance_id);

                ec2.terminate_instances()
                    .instance_ids(instance_id)
                    .send()
                    .await
                    .map_err(|e| {
                        Error::ConfigurationError(format!("Failed to terminate EC2: {e}"))
                    })?;
            }

            // Also cleanup associated resources
            if let Some(eip_allocation) = deployment.resource_ids.get("elastic_ip") {
                let _ = ec2
                    .release_address()
                    .allocation_id(eip_allocation)
                    .send()
                    .await;
            }

            if let Some(volume_id) = deployment.resource_ids.get("ebs_volume") {
                // Wait a bit for instance termination
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

                let _ = ec2.delete_volume().volume_id(volume_id).send().await;
            }
        }

        #[cfg(not(feature = "aws"))]
        let _ = deployment;

        Ok(())
    }
}

/// GCP cleanup
pub(crate) struct GcpCleanup;

#[async_trait::async_trait]
impl CleanupHandler for GcpCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        #[cfg(feature = "gcp")]
        {
            use crate::providers::gcp::GcpProvisioner;

            if let (Some(project), Some(zone)) = (
                deployment.metadata.get("project_id"),
                deployment.region.as_ref(),
            ) {
                let provisioner = GcpProvisioner::new(project.clone()).await?;

                if let Some(instance_name) = deployment.resource_ids.get("instance_name") {
                    info!("Deleting GCP instance: {}", instance_name);
                    provisioner.terminate_instance(instance_name, zone).await?;
                }
            }
        }

        #[cfg(not(feature = "gcp"))]
        let _ = deployment;

        Ok(())
    }
}

/// Azure cleanup
pub(crate) struct AzureCleanup;

#[async_trait::async_trait]
impl CleanupHandler for AzureCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        if let Some(instance_id) = deployment.resource_ids.get("instance_id") {
            use crate::core::remote::CloudProvider;
            use crate::infra::provisioner::CloudProvisioner;

            info!("Deleting Azure VM: {}", instance_id);
            let provisioner = CloudProvisioner::new().await?;
            provisioner
                .terminate(CloudProvider::Azure, instance_id)
                .await?;
        }

        Ok(())
    }
}

/// DigitalOcean cleanup
pub(crate) struct DigitalOceanCleanup;

#[async_trait::async_trait]
impl CleanupHandler for DigitalOceanCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        if let Some(instance_id) = deployment.resource_ids.get("instance_id") {
            use crate::core::remote::CloudProvider;
            use crate::infra::provisioner::CloudProvisioner;

            info!("Deleting DigitalOcean droplet: {}", instance_id);
            let provisioner = CloudProvisioner::new().await?;
            provisioner
                .terminate(CloudProvider::DigitalOcean, instance_id)
                .await?;
        }
        Ok(())
    }
}

/// Vultr cleanup
pub(crate) struct VultrCleanup;

#[async_trait::async_trait]
impl CleanupHandler for VultrCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        if let Some(instance_id) = deployment.resource_ids.get("instance_id") {
            use crate::core::remote::CloudProvider;
            use crate::infra::provisioner::CloudProvisioner;

            info!("Deleting Vultr instance: {}", instance_id);
            let provisioner = CloudProvisioner::new().await?;
            provisioner
                .terminate(CloudProvider::Vultr, instance_id)
                .await?;
        }
        Ok(())
    }
}
