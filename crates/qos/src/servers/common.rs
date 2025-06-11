use blueprint_core::{debug, error, info, warn};
use bollard::{
    container::{
        Config, CreateContainerOptions, ListContainersOptions, RemoveContainerOptions,
        StartContainerOptions, StopContainerOptions,
    },
    image::CreateImageOptions,
    models::{HealthConfig, HealthStatusEnum, HostConfig, PortBinding},
    network::{ConnectNetworkOptions, CreateNetworkOptions, InspectNetworkOptions},
    Docker,
};
use futures::StreamExt;
use std::{collections::HashMap, default::Default};
use tokio::time::{sleep, Duration, Instant};

use crate::error::{Error, Result};

/// Docker container manager
#[derive(Clone)]
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
        let docker = Docker::connect_with_local_defaults()
            .expect("Failed to connect to Docker. Is the Docker daemon running?");
        Self { docker }
    }

    /// Pull an image if it doesn't exist
    pub async fn ensure_image(&self, image: &str) -> Result<()> {
        info!("Ensuring image exists: {}", image);
        if self.docker.inspect_image(image).await.is_ok() {
            info!("Image '{}' already exists locally.", image);
            return Ok(());
        }

        info!("Pulling image '{}'...", image);
        let options = Some(CreateImageOptions {
            from_image: image,
            ..Default::default()
        });

        let mut stream = self.docker.create_image(options, None, None);
        while let Some(pull_result) = stream.next().await {
            if let Err(e) = pull_result {
                let err_msg = format!("Failed to pull image '{}': {}", image, e);
                error!("{}", err_msg);
                return Err(Error::DockerOperation(err_msg));
            }
        }
        info!("Image pull complete for: {}", image);
        Ok(())
    }

    /// Create and start a container
    #[allow(clippy::too_many_arguments)]
    pub async fn run_container(
        &self,
        image: &str,
        name: &str,
        env_vars: HashMap<String, String>,
        ports: HashMap<String, String>,
        volumes: HashMap<String, String>,
        extra_hosts: Option<Vec<String>>,
        health_check_cmd: Option<Vec<String>>,
    ) -> Result<String> {
        if let Err(e) = self.ensure_image(image).await {
            warn!("Failed to ensure image exists, but proceeding: {}", e);
        }

        self.cleanup_container_by_name(name).await?;

        let env: Vec<String> = env_vars
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();

        let port_bindings = ports
            .into_iter()
            .map(|(container_port, host_port)| {
                (
                    container_port.to_string(),
                    Some(vec![PortBinding {
                        host_ip: Some("0.0.0.0".to_string()),
                        host_port: Some(host_port),
                    }]),
                )
            })
            .collect();

        let binds = if volumes.is_empty() {
            None
        } else {
            Some(
                volumes
                    .into_iter()
                    .map(|(host_path, container_path)| format!("{}:{}", host_path, container_path))
                    .collect(),
            )
        };

        let host_config = HostConfig {
            port_bindings: Some(port_bindings),
            binds,
            extra_hosts,
            ..Default::default()
        };

        let health_config = health_check_cmd.map(|cmd| HealthConfig {
            test: Some(cmd),
            interval: Some(Duration::from_secs(5).as_nanos() as i64),
            timeout: Some(Duration::from_secs(5).as_nanos() as i64),
            retries: Some(3),
            start_period: Some(Duration::from_secs(5).as_nanos() as i64),
            start_interval: Some(Duration::from_secs(1).as_nanos() as i64),
        });

        let config = Config {
            image: Some(image),
            env: Some(env.iter().map(AsRef::as_ref).collect()),
            host_config: Some(host_config),
            healthcheck: health_config,
            ..Default::default()
        };

        info!("Creating container '{}' from image '{}'", name, image);
        let options = Some(CreateContainerOptions {
            name: name.to_string(),
            platform: None, // Explicitly set platform to None for wider compatibility
        });

        let container_id = match self.docker.create_container(options, config).await {
            Ok(response) => response.id,
            Err(e) => {
                let err_msg = format!("Failed to create container '{}': {}", name, e);
                error!("[DOCKER ERROR] {}", err_msg);
                panic!("[DOCKER PANIC] {}", err_msg);
            }
        };

        info!("Starting container '{}' (ID: {})", name, &container_id);
        if let Err(e) = self
            .docker
            .start_container(&container_id, None::<StartContainerOptions<String>>)
            .await
        {
            let err_msg = format!("Failed to start container '{}': {}", name, e);
            error!("[DOCKER ERROR] {}", err_msg);
            panic!("[DOCKER PANIC] {}", err_msg);
        }

        info!("Successfully started container '{}' (ID: {})", name, &container_id);
        Ok(container_id)
    }

    async fn cleanup_container_by_name(&self, name: &str) -> Result<()> {
        let mut list_options = ListContainersOptions::<String>::default();
        list_options.all = true;

        let containers = self
            .docker
            .list_containers(Some(list_options))
            .await
            .map_err(|e| Error::DockerOperation(e.to_string()))?;

        for container_summary in containers {
            if container_summary
                .names
                .as_ref()
                .map_or(false, |names| names.contains(&format!("/{}", name)))
            {
                info!("Found existing container '{}', stopping and removing.", name);
                if let Some(container_id) = container_summary.id.as_deref() {
                    self.stop_and_remove_container(container_id, name).await?;
                }
                break;
            }
        }
        Ok(())
    }

    pub async fn stop_and_remove_container(
        &self,
        container_id: &str,
        container_name: &str,
    ) -> Result<()> {
        info!("Stopping container '{}' ({})", container_name, container_id);
        if let Err(e) = self.docker.stop_container(container_id, None::<StopContainerOptions>).await {
            warn!(
                "Could not stop container '{}' (may already be stopped): {}. Proceeding with removal.",
                container_name, e
            );
        }

        info!("Removing container '{}' ({})", container_name, container_id);
        self.docker
            .remove_container(
                container_id,
                Some(RemoveContainerOptions { force: true, ..Default::default() }),
            )
            .await
            .map_err(|e| {
                Error::DockerOperation(format!(
                    "Failed to remove container '{}' ({}): {}",
                    container_name, container_id, e
                ))
            })?;
        Ok(())
    }

    pub async fn create_network(&self, network_name: &str) -> Result<()> {
        match self.docker.inspect_network(network_name, None::<InspectNetworkOptions<String>>).await {
            Ok(_) => {
                info!("Network '{}' already exists.", network_name);
                Ok(())
            }
            Err(bollard::errors::Error::DockerResponseServerError { status_code, .. }) if status_code == 404 => {
                info!("Creating Docker network: '{}'", network_name);
                let options = CreateNetworkOptions { name: network_name, ..Default::default() };
                self.docker
                    .create_network(options)
                    .await
                    .map_err(|e| Error::DockerOperation(e.to_string()))?;
                info!("Successfully created Docker network: '{}'", network_name);
                Ok(())
            }
            Err(e) => {
                let err_msg = format!("Failed to inspect Docker network '{}': {}", network_name, e);
                error!("{}", err_msg);
                Err(Error::DockerOperation(err_msg))
            }
        }
    }

    pub async fn connect_to_network(&self, container_name: &str, network_name: &str) -> Result<()> {
        info!(
            "Connecting container '{}' to network '{}'",
            container_name, network_name
        );
        let options = ConnectNetworkOptions { container: container_name, ..Default::default() };
        self.docker
            .connect_network(network_name, options)
            .await
            .map_err(|e| Error::DockerOperation(e.to_string()))
    }

    pub async fn is_container_running(&self, container_id: &str) -> Result<bool> {
        let container = self
            .docker
            .inspect_container(container_id, None)
            .await
            .map_err(|e| Error::DockerOperation(e.to_string()))?;

        Ok(container.state.map_or(false, |s| s.running.unwrap_or(false)))
    }

    pub async fn wait_for_container_health(&self, container_id: &str, timeout_secs: u64) -> Result<()> {
        let timeout = Duration::from_secs(timeout_secs);
        let start = Instant::now();

        while start.elapsed() < timeout {
            let inspect_result = self.docker.inspect_container(container_id, None).await;

            match inspect_result {
                Ok(container) => {
                    if let Some(state) = &container.state {
                        if let Some(health) = &state.health {
                            if let Some(status) = &health.status {
                                match status {
                                    HealthStatusEnum::HEALTHY => {
                                        info!("Container {} is healthy.", container_id);
                                        return Ok(());
                                    }
                                    HealthStatusEnum::UNHEALTHY => {
                                        let err_msg = format!("Container {} reported unhealthy status.", container_id);
                                        error!("{}", err_msg);
                                        return Err(Error::DockerOperation(err_msg));
                                    }
                                    _ => {
                                        debug!("Container {} health status: {:?}", container_id, status);
                                    }
                                }
                            }
                        } else if state.running.unwrap_or(false) {
                            info!("Container {} is running (no health check configured).", container_id);
                            return Ok(());
                        }
                    }
                }
                Err(e) => {
                    warn!("Error inspecting container {}: {}. Retrying...", container_id, e);
                }
            }

            sleep(Duration::from_secs(1)).await;
        }

        Err(Error::DockerOperation(format!(
            "Container {} did not become ready within {} seconds.",
            container_id,
            timeout_secs
        )))
    }
}
