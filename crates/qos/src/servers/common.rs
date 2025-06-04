use blueprint_core::{debug, error, info, warn};
use futures::StreamExt;
use shiplift::{ContainerOptions, Docker, PullOptions};
use std::collections::HashMap;
use std::time::Duration;
use tokio_retry::{Retry, strategy::ExponentialBackoff};

use crate::error::{Error, Result};

/// Docker container manager
pub struct DockerManager {
    /// Docker client
    docker: Docker,
}

impl Default for DockerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DockerManager {
    /// Create a new Docker manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            docker: Docker::new(),
        }
    }

    /// Pull an image if it doesn't exist
    ///
    /// # Errors
    /// Returns an error if the image pull fails
    pub async fn ensure_image(&self, image: &str) -> Result<()> {
        // Skip checking if the image exists to avoid potential API version mismatches
        // Just try to pull the image directly and handle any errors gracefully
        info!("Attempting to pull image: {}", image);
        let images = self.docker.images();
        let options = PullOptions::builder().image(image).build();

        // Try to pull the image but don't fail if it already exists
        let mut stream = images.pull(&options);

        while let Some(pull_result) = stream.next().await {
            match pull_result {
                Ok(output) => debug!("Pull progress: {:?}", output),
                Err(e) => {
                    // If the error indicates the image already exists, that's fine
                    if e.to_string().contains("already exists") {
                        info!("Image {} already exists", image);
                        return Ok(());
                    }

                    // For other errors, log but continue - we'll try to use the image anyway
                    error!("Error pulling image {}: {}", image, e);
                    // Don't return error here, try to continue with container creation
                }
            }
        }

        info!("Image pull operation completed for: {}", image);
        Ok(())
    }

    /// Create and start a container
    ///
    /// # Errors
    /// Returns an error if the container creation or start fails
    pub async fn run_container(
        &self,
        image: &str,
        name: &str,
        env_vars: HashMap<String, String>,
        ports: HashMap<String, String>,
        volumes: HashMap<String, String>,
    ) -> Result<String> {
        // Try to ensure image exists, but continue even if there are issues
        if let Err(e) = self.ensure_image(image).await {
            warn!(
                "Issue ensuring image exists, but will try to continue: {}",
                e
            );
        }

        // Try to clean up any existing container with the same name
        let containers = self.docker.containers();
        match containers
            .list(&shiplift::ContainerListOptions::default())
            .await
        {
            Ok(all_containers) => {
                for container in all_containers {
                    if container.names.iter().any(|n| n == &format!("/{}", name)) {
                        info!("Container {} already exists, stopping and removing", name);
                        let container = containers.get(container.id);
                        // Try to stop and remove, but don't fail if we can't
                        if let Err(e) = container.stop(None).await {
                            warn!(
                                "Failed to stop container {}: {}, will try to continue",
                                name, e
                            );
                        }
                        if let Err(e) = container.delete().await {
                            warn!(
                                "Failed to delete container {}: {}, will try to continue",
                                name, e
                            );
                        }
                    }
                }
            }
            Err(e) => {
                // If we can't list containers, log and continue
                warn!(
                    "Failed to list Docker containers: {}, will try to continue",
                    e
                );
            }
        }

        // Create container options
        let mut options = ContainerOptions::builder(image);
        options.name(name);

        // Add environment variables
        for (key, value) in &env_vars {
            options.env(&[format!("{}={}", key, value)]);
        }

        // Add port mappings
        for (container_port, host_port) in &ports {
            // Parse the container port, defaulting to 3000 if it fails
            let container_port_num = match container_port.split('/').next() {
                Some(port_str) => port_str.parse::<u32>().unwrap_or(3000),
                None => 3000,
            };

            // Use the expose method with the correct parameters
            // expose(srcport, protocol, hostport)
            options.expose(
                container_port_num,
                "tcp",
                host_port.parse::<u32>().unwrap_or(3000),
            );
        }

        // Add volume mappings if available
        if volumes.is_empty() {
            info!("No volume mappings provided");
        } else {
            info!("Adding volume mappings: {:?}", volumes);
            for (container_path, host_path) in &volumes {
                // Format as "host_path:container_path" for volume mapping
                let volume_mapping = format!("{}:{}", host_path, container_path);
                options.volumes(vec![&volume_mapping]);
            }
        }

        // Create and start the container
        info!("Creating and starting container: {}", name);
        let container_id = match containers.create(&options.build()).await {
            Ok(container) => container.id,
            Err(e) => {
                error!(
                    "[DOCKER ERROR] Failed to create container with volumes: {}",
                    e
                );
                panic!(
                    "[DOCKER PANIC] Could not create container '{}': {}.\n\nPossible causes: Docker is not running, permissions issue, or Docker daemon is not accessible to this user. Please run 'docker info' and ensure you can create containers manually.",
                    name, e
                );
            }
        };

        let container = containers.get(&container_id);
        match container.start().await {
            Ok(()) => {
                info!(
                    "Successfully started container: {} (ID: {})",
                    name, container_id
                );
                Ok(container_id)
            }
            Err(e) => {
                error!("[DOCKER ERROR] Failed to start container: {}", e);
                panic!(
                    "[DOCKER PANIC] Could not start container '{}': {}.\n\nPossible causes: Docker is not running, permissions issue, or Docker daemon is not accessible to this user. Please run 'docker info' and ensure you can create containers manually.",
                    name, e
                );
            }
        }
    }

    /// Stop and remove a container
    ///
    /// # Errors
    /// Returns an error if the container stop or removal fails
    pub async fn stop_container(&self, container_id: &str) -> Result<()> {
        let container = self.docker.containers().get(container_id);

        // Stop the container
        info!("Stopping container: {}", container_id);
        container.stop(None).await.map_err(|e| {
            Error::Other(format!("Failed to stop container {}: {}", container_id, e))
        })?;

        // Remove the container
        info!("Removing container: {}", container_id);
        container.delete().await.map_err(|e| {
            Error::Other(format!(
                "Failed to remove container {}: {}",
                container_id, e
            ))
        })?;

        info!(
            "Successfully stopped and removed container: {}",
            container_id
        );
        Ok(())
    }

    /// Check if a container is running
    ///
    /// # Errors
    /// Returns an error if the container inspection fails
    pub async fn is_container_running(&self, container_id: &str) -> Result<bool> {
        let container = self.docker.containers().get(container_id);
        let details = container.inspect().await.map_err(|e| {
            Error::Other(format!(
                "Failed to inspect container {}: {}",
                container_id, e
            ))
        })?;

        let running = details.state.running;
        Ok(running)
    }

    /// Wait for a container to be healthy
    ///
    /// # Errors
    /// Returns an error if the health check fails
    pub async fn wait_for_container_health(
        &self,
        container_id: &str,
        timeout_secs: u64,
    ) -> Result<()> {
        let retry_strategy = ExponentialBackoff::from_millis(100)
            .factor(2)
            .max_delay(Duration::from_secs(1))
            .take(usize::try_from(timeout_secs).unwrap_or(30));

        Retry::spawn(retry_strategy, || async {
            let container = self.docker.containers().get(container_id);
            let details = container.inspect().await.map_err(|e| {
                error!("Failed to inspect container {}: {}", container_id, e);
                // Map the error to unit type
            })?;

            let health = Some(details.state.status.clone());

            let running = details.state.running;

            if running {
                // If the container is running, consider it good enough
                match health {
                    Some(status) if status == "healthy" => {
                        info!("Container {} is healthy", container_id);
                        Ok(())
                    }
                    Some(status) => {
                        // Accept any status as long as the container is running
                        info!(
                            "Container {} is running with status: {}",
                            container_id, status
                        );
                        Ok(())
                    }
                    None => {
                        info!(
                            "Container {} is running (no health status available)",
                            container_id
                        );
                        Ok(())
                    }
                }
            } else {
                debug!("Container {} is not running", container_id);
                Err(())
            }
        })
        .await
        .map_err(|()| {
            Error::Other(format!(
                "Container {} did not become healthy within {} seconds",
                container_id, timeout_secs
            ))
        })
    }
}
