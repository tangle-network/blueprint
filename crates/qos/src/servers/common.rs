use futures::StreamExt;
use shiplift::{ContainerOptions, Docker, PullOptions};
use std::collections::HashMap;
use std::time::Duration;
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
        let all_images = images.list(&shiplift::ImageListOptions::default()).await.map_err(|e| {
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
        let all_containers = containers.list(&shiplift::ContainerListOptions::default()).await.map_err(|e| {
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
        .map_err(|()| {
            Error::Other(format!(
                "Container {} did not become healthy within {} seconds",
                container_id, timeout_secs
            ))
        })
    }
}
