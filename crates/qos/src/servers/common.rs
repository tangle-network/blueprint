//! Common utilities for server management

use futures::StreamExt;
use shiplift::{ContainerOptions, Docker, ExecContainerOptions, PullOptions};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use tokio_retry::{strategy::ExponentialBackoff, Retry};
use tracing::{debug, error, info};

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
        let images = self.docker.images();
        let all_images = images.list(&Default::default()).await.map_err(|e| {
            Error::Other(format!("Failed to list Docker images: {}", e))
        })?;

        // Check if image already exists
        for img in all_images {
            if let Some(tags) = img.repo_tags {
                if tags.contains(&image.to_string()) {
                    debug!("Image {} already exists", image);
                    return Ok(());
                }
            }
        }

        // Pull the image
        info!("Pulling image: {}", image);
        let options = PullOptions::builder().image(image).build();
        let mut stream = images.pull(&options);

        while let Some(pull_result) = stream.next().await {
            match pull_result {
                Ok(output) => debug!("Pull progress: {:?}", output),
                Err(e) => {
                    return Err(Error::Other(format!("Failed to pull image {}: {}", image, e)));
                }
            }
        }

        info!("Successfully pulled image: {}", image);
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
        // Ensure image exists
        self.ensure_image(image).await?;

        // Check if container already exists
        let containers = self.docker.containers();
        let all_containers = containers.list(&Default::default()).await.map_err(|e| {
            Error::Other(format!("Failed to list Docker containers: {}", e))
        })?;

        for container in all_containers {
            if container.names.iter().any(|n| n == &format!("/{}", name)) {
                info!("Container {} already exists, stopping and removing", name);
                let container = containers.get(container.id);
                container.stop(None).await.map_err(|e| {
                    Error::Other(format!("Failed to stop container {}: {}", name, e))
                })?;
                container.delete().await.map_err(|e| {
                    Error::Other(format!("Failed to delete container {}: {}", name, e))
                })?;
            }
        }

        // Create container options
        let mut options = ContainerOptions::builder(image);
        options.name(name);

        // Add environment variables
        for (key, value) in env_vars {
            options.env(&[format!("{}={}", key, value)]);
        }

        // Add port mappings
        for (container_port, host_port) in ports {
            // Use the expose method with the correct parameters
            // expose(srcport, protocol, hostport)
            options.expose(container_port.parse::<u32>().unwrap_or(3000), "tcp", host_port.parse::<u32>().unwrap_or(3000));
        }

        // Add volume mappings
        for (container_path, host_path) in volumes {
            // Format as "host_path:container_path" for volume mapping
            let volume_mapping = format!("{}:{}", host_path, container_path);
            options.volumes(vec![&volume_mapping]);
        }

        // Create and start the container
        info!("Creating and starting container: {}", name);
        let container_id = containers
            .create(&options.build())
            .await
            .map_err(|e| Error::Other(format!("Failed to create container {}: {}", name, e)))?
            .id;

        let container = containers.get(&container_id);
        container
            .start()
            .await
            .map_err(|e| Error::Other(format!("Failed to start container {}: {}", name, e)))?;

        info!("Successfully started container: {} (ID: {})", name, container_id);
        Ok(container_id)
    }

    /// Stop and remove a container
    ///
    /// # Errors
    /// Returns an error if the container stop or removal fails
    pub async fn stop_container(&self, container_id: &str) -> Result<()> {
        let container = self.docker.containers().get(container_id);
        
        // Stop the container
        info!("Stopping container: {}", container_id);
        container
            .stop(None)
            .await
            .map_err(|e| Error::Other(format!("Failed to stop container {}: {}", container_id, e)))?;

        // Remove the container
        info!("Removing container: {}", container_id);
        container
            .delete()
            .await
            .map_err(|e| Error::Other(format!("Failed to remove container {}: {}", container_id, e)))?;

        info!("Successfully stopped and removed container: {}", container_id);
        Ok(())
    }

    /// Check if a container is running
    ///
    /// # Errors
    /// Returns an error if the container inspection fails
    pub async fn is_container_running(&self, container_id: &str) -> Result<bool> {
        let container = self.docker.containers().get(container_id);
        let details = container
            .inspect()
            .await
            .map_err(|e| Error::Other(format!("Failed to inspect container {}: {}", container_id, e)))?;

        // Check if the container is running by inspecting the state
        // The State struct has a running boolean field
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
        // Create a retry strategy with exponential backoff
        let retry_strategy = ExponentialBackoff::from_millis(100)
            .factor(2)
            .max_delay(Duration::from_secs(1))
            // Use take() to limit the number of retries
            .take(timeout_secs as usize);

        Retry::spawn(retry_strategy, || async {
            let container = self.docker.containers().get(container_id);
            let details = container.inspect().await.map_err(|e| {
                error!("Failed to inspect container {}: {}", container_id, e);
                ()
            })?;

            // Extract health status from container state
            // In shiplift, we need to check the status field
            let health = Some(details.state.status.clone());

            match health {
                Some(status) if status == "healthy" => {
                    info!("Container {} is healthy", container_id);
                    Ok(())
                }
                Some(status) => {
                    debug!("Container {} health status: {}", container_id, status);
                    Err(())
                }
                None => {
                    // If no health check is defined, just check if it's running
                    // Check if the container is running
                    let running = details.state.running;
                    if running {
                        info!("Container {} is running (no health check defined)", container_id);
                        Ok(())
                    } else {
                        debug!("Container {} is not running", container_id);
                        Err(())
                    }
                }
            }
        })
        .await
        .map_err(|_| {
            Error::Other(format!(
                "Container {} did not become healthy within {} seconds",
                container_id, timeout_secs
            ))
        })
    }
}
