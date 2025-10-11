//! SSH remote deployment cleanup handler

use super::super::types::{CleanupHandler, DeploymentRecord};
use crate::core::error::Result;
use blueprint_core::info;
use std::path::PathBuf;

/// SSH remote cleanup
pub(crate) struct SshCleanup;

#[async_trait::async_trait]
impl CleanupHandler for SshCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        use crate::deployment::ssh::{
            ContainerRuntime, DeploymentConfig, RestartPolicy, SshConnection, SshDeploymentClient,
        };

        if let (Some(host), Some(user)) = (
            deployment.metadata.get("ssh_host"),
            deployment.metadata.get("ssh_user"),
        ) {
            let connection = SshConnection {
                host: host.clone(),
                port: deployment
                    .metadata
                    .get("ssh_port")
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(22),
                user: user.clone(),
                key_path: deployment.metadata.get("ssh_key_path").map(PathBuf::from),
                password: None,
                jump_host: deployment.metadata.get("jump_host").cloned(),
            };

            let runtime = match deployment.metadata.get("runtime").map(|s| s.as_str()) {
                Some("docker") => ContainerRuntime::Docker,
                Some("podman") => ContainerRuntime::Podman,
                _ => ContainerRuntime::Docker,
            };

            let client = SshDeploymentClient::new(
                connection,
                runtime,
                DeploymentConfig {
                    name: deployment.blueprint_id.clone(),
                    namespace: "default".to_string(),
                    restart_policy: RestartPolicy::Never,
                    health_check: None,
                },
            )
            .await?;

            if let Some(container_id) = deployment.resource_ids.get("container_id") {
                info!("Cleaning up remote container: {} on {}", container_id, host);
                client.cleanup_deployment(container_id).await?;
            }
        }

        Ok(())
    }
}
