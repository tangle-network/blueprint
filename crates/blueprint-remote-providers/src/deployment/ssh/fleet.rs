//! Batch deployment to multiple SSH hosts

use super::client::SshDeploymentClient;
use super::types::{ContainerRuntime, DeploymentConfig, RemoteDeployment, RestartPolicy, SshConnection};
use crate::core::error::Result;
use crate::core::resources::ResourceSpec;
use std::collections::HashMap;
use blueprint_core::{info, warn};

/// Batch deployment to multiple hosts
pub struct BareMetalFleet {
    hosts: Vec<SshConnection>,
    deployments: Vec<RemoteDeployment>,
}

impl BareMetalFleet {
    /// Create a new bare metal fleet
    pub fn new(hosts: Vec<SshConnection>) -> Self {
        Self {
            hosts,
            deployments: Vec::new(),
        }
    }

    /// Deploy to all hosts in parallel
    pub async fn deploy_to_fleet(
        &mut self,
        blueprint_image: &str,
        spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
        runtime: ContainerRuntime,
    ) -> Result<Vec<RemoteDeployment>> {
        use futures::future::join_all;

        let deployment_futures: Vec<_> = self
            .hosts
            .iter()
            .map(|host| {
                let connection = host.clone();
                let image = blueprint_image.to_string();
                let spec = spec.clone();
                let env = env_vars.clone();
                let rt = runtime.clone();

                async move {
                    let client = SshDeploymentClient::new(
                        connection,
                        rt,
                        DeploymentConfig {
                            name: "blueprint".to_string(),
                            namespace: "default".to_string(),
                            restart_policy: RestartPolicy::Always,
                            health_check: None,
                        },
                    )
                    .await?;

                    client.deploy_blueprint(&image, &spec, env).await
                }
            })
            .collect();

        let results = join_all(deployment_futures).await;

        for result in results {
            match result {
                Ok(deployment) => {
                    info!("Successfully deployed to {}", deployment.host);
                    self.deployments.push(deployment);
                }
                Err(e) => {
                    warn!("Failed to deploy to host: {}", e);
                }
            }
        }

        Ok(self.deployments.clone())
    }

    /// Get status of all deployments
    pub fn get_fleet_status(&self) -> HashMap<String, String> {
        self.deployments
            .iter()
            .map(|d| (d.host.clone(), d.status.clone()))
            .collect()
    }
}
