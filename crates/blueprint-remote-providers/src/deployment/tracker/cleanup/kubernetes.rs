//! Kubernetes cluster cleanup handlers

use super::super::types::{CleanupHandler, DeploymentRecord};
use crate::core::error::Result;
#[cfg(feature = "aws-eks")]
use crate::core::error::Error;
use blueprint_core::{info, warn};

/// EKS cleanup
pub(crate) struct EksCleanup;

#[async_trait::async_trait]
impl CleanupHandler for EksCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        #[cfg(feature = "aws-eks")]
        {
            let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
            let eks = aws_sdk_eks::Client::new(&config);

            if let Some(cluster_name) = deployment.resource_ids.get("cluster_name") {
                info!("Deleting EKS cluster: {}", cluster_name);

                // Delete node groups first
                let nodegroups = eks
                    .list_nodegroups()
                    .cluster_name(cluster_name)
                    .send()
                    .await?;

                if let Some(ngs) = nodegroups.nodegroups {
                    for ng in ngs {
                        let _ = eks
                            .delete_nodegroup()
                            .cluster_name(cluster_name)
                            .nodegroup_name(ng)
                            .send()
                            .await;
                    }
                }

                // Wait for nodegroups to be deleted
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

                // Delete cluster
                eks.delete_cluster()
                    .name(cluster_name)
                    .send()
                    .await
                    .map_err(|e| {
                        Error::ConfigurationError(format!("Failed to delete EKS: {}", e))
                    })?;
            }
        }

        #[cfg(not(feature = "aws-eks"))]
        let _ = deployment;

        Ok(())
    }
}

/// GKE cleanup
pub(crate) struct GkeCleanup;

#[async_trait::async_trait]
impl CleanupHandler for GkeCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        #[cfg(feature = "gcp")]
        {
            use crate::providers::gcp::GcpProvisioner;

            if let (Some(project), Some(region)) = (
                deployment.metadata.get("project_id"),
                deployment.region.as_ref(),
            ) {
                let provisioner = GcpProvisioner::new(project.clone()).await?;

                if let Some(cluster_name) = deployment.resource_ids.get("cluster_name") {
                    info!("Deleting GKE cluster: {}", cluster_name);
                    // GKE cluster deletion requires gcloud SDK or complex API calls
                    warn!("GKE cluster cleanup not implemented - use gcloud CLI");
                }

                let _ = (provisioner, region);
            }
        }

        #[cfg(not(feature = "gcp"))]
        let _ = deployment;

        Ok(())
    }
}

/// AKS cleanup
pub(crate) struct AksCleanup;

#[async_trait::async_trait]
impl CleanupHandler for AksCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        if let Some(cluster_name) = deployment.resource_ids.get("cluster_name") {
            info!("Deleting AKS cluster: {}", cluster_name);
            // AKS cluster deletion requires Azure CLI or complex API calls
            warn!("AKS cluster cleanup not implemented - use az CLI");
        }
        Ok(())
    }
}
