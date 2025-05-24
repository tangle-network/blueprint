use blueprint_core::{error, info};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

use crate::error::Result;
use crate::metrics::prometheus::server::PrometheusServer as PrometheusMetricsServer;
use crate::servers::ServerManager;
use crate::servers::common::DockerManager;

/// Configuration for the Prometheus server
#[derive(Clone, Debug)]
pub struct PrometheusServerConfig {
    /// The port to bind the Prometheus server to
    pub port: u16,

    /// The host to bind the Prometheus server to
    pub host: String,

    /// Whether to use Docker for the Prometheus server
    pub use_docker: bool,

    /// The Docker image to use for the Prometheus server
    pub docker_image: String,

    /// The Docker container name to use for the Prometheus server
    pub docker_container_name: String,

    /// The path to the Prometheus configuration file
    pub config_path: Option<String>,

    /// The path to mount as the Prometheus data directory
    pub data_path: Option<String>,
}

impl Default for PrometheusServerConfig {
    fn default() -> Self {
        Self {
            port: 9090,
            host: "0.0.0.0".to_string(),
            use_docker: true,
            docker_image: "prom/prometheus:latest".to_string(),
            docker_container_name: "blueprint-prometheus".to_string(),
            config_path: None,
            data_path: None,
        }
    }
}

/// Prometheus server manager
pub struct PrometheusServer {
    /// The configuration for the Prometheus server
    config: PrometheusServerConfig,

    /// The Docker manager for the Prometheus server (if using Docker)
    docker_manager: DockerManager,

    /// The container ID for the Prometheus server (if using Docker)
    container_id: Arc<Mutex<Option<String>>>,

    /// The embedded Prometheus server (if not using Docker)
    embedded_server: Arc<Mutex<Option<PrometheusMetricsServer>>>,

    /// The registry for the embedded Prometheus server
    registry: Arc<Mutex<Option<prometheus::Registry>>>,
}

impl PrometheusServer {
    /// Create a new Prometheus server manager
    #[must_use]
    pub fn new(config: PrometheusServerConfig) -> Self {
        Self {
            config,
            docker_manager: DockerManager::new(),
            container_id: Arc::new(Mutex::new(None)),
            embedded_server: Arc::new(Mutex::new(None)),
            registry: Arc::new(Mutex::new(None)),
        }
    }

    /// Create a new embedded Prometheus server
    async fn create_embedded_server(&self) -> Result<()> {
        let registry = prometheus::Registry::new();
        let bind_address = format!("{}:{}", self.config.host, self.config.port);

        let server = PrometheusMetricsServer::new(registry.clone(), bind_address);

        // Update the embedded server
        {
            let mut embedded_server = self.embedded_server.lock().unwrap();
            *embedded_server = Some(server);
        }

        // Update the registry
        {
            let mut reg = self.registry.lock().unwrap();
            *reg = Some(registry);
        }

        Ok(())
    }

    /// Create a new Docker container for the Prometheus server
    ///
    /// # Errors
    /// Returns an error if the container creation fails
    pub async fn create_docker_container(&self) -> Result<()> {
        // Create environment variables
        let env_vars = HashMap::new();

        // Create port mappings
        let mut ports = HashMap::new();
        ports.insert(self.config.port.to_string(), self.config.port.to_string());

        // Create volume mappings
        let mut volumes = HashMap::new();
        if let Some(config_path) = &self.config.config_path {
            volumes.insert(
                config_path.clone(),
                "/etc/prometheus/prometheus.yml".to_string(),
            );
        }

        if let Some(data_path) = &self.config.data_path {
            volumes.insert(data_path.clone(), "/prometheus".to_string());
        }

        // Run the container
        let container_id = self
            .docker_manager
            .run_container(
                &self.config.docker_image,
                &self.config.docker_container_name,
                env_vars,
                ports,
                volumes,
            )
            .await?;

        let mut id = self.container_id.lock().unwrap();
        *id = Some(container_id);

        Ok(())
    }

    /// Get the registry for the embedded Prometheus server
    #[must_use]
    pub fn registry(&self) -> Option<Arc<prometheus::Registry>> {
        let registry_guard = self.registry.lock().ok()?;
        registry_guard.as_ref().map(|r| Arc::new(r.clone()))
    }
}

#[async_trait::async_trait]
impl ServerManager for PrometheusServer {
    async fn start(&self) -> Result<()> {
        if self.config.use_docker {
            let container_id = {
                let id = self.container_id.lock().unwrap();
                match id.as_ref() {
                    Some(id) => id.clone(),
                    None => {
                        return Err(crate::error::Error::Other(
                            "Docker container not initialized".to_string(),
                        ));
                    }
                }
            };

            // Container already created, just check if it's running
            let is_running = self
                .docker_manager
                .is_container_running(&container_id)
                .await?;
            if !is_running {
                // If not running, we need to create it again
                return Err(crate::error::Error::Other(
                    "Docker container not running and cannot be restarted automatically"
                        .to_string(),
                ));
            }
            info!(
                "Prometheus server is already running in Docker container: {}",
                self.config.docker_container_name
            );
        } else {
            // For embedded server, we need to initialize it if it doesn't exist
            if self.embedded_server.lock().unwrap().is_none() {
                // Initialize the embedded server
                self.create_embedded_server().await?;
            }

            info!(
                "Prometheus embedded server initialized on {}:{}",
                self.config.host, self.config.port
            );

            // Check if the embedded server exists
            let server_exists = {
                let guard = self.embedded_server.lock().unwrap();
                guard.is_some()
            };

            if !server_exists {
                return Err(crate::error::Error::Other(
                    "Embedded server not initialized properly".to_string(),
                ));
            }

            // Start the server without holding the mutex guard across the await
            {
                // Get a clone of the server to avoid holding the mutex across an await point
                let mut server = {
                    let server_guard = self.embedded_server.lock().unwrap();
                    if let Some(server) = &*server_guard {
                        // Clone the server's fields to create a new instance
                        let bind_address = format!("{}:{}", self.config.host, self.config.port);
                        let registry = self
                            .registry
                            .lock()
                            .unwrap()
                            .clone()
                            .unwrap_or_else(prometheus::Registry::new);
                        PrometheusMetricsServer::new(registry, bind_address)
                    } else {
                        return Err(crate::error::Error::Other(
                            "Embedded server not initialized properly".to_string(),
                        ));
                    }
                };

                // Start the server
                if let Err(e) = server.start().await {
                    return Err(e);
                }

                // Store the started server back in the mutex
                let mut server_guard = self.embedded_server.lock().unwrap();
                *server_guard = Some(server);

                info!(
                    "Started embedded Prometheus server on {}:{}",
                    self.config.host, self.config.port
                );
            }
        }

        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        if self.config.use_docker {
            let container_id = {
                let id = self.container_id.lock().unwrap();
                match id.as_ref() {
                    Some(id) => id.clone(),
                    None => {
                        info!("Prometheus server is not running");
                        return Ok(());
                    }
                }
            };

            self.docker_manager.stop_container(&container_id).await?;

            // Clear the container ID
            let mut id = self.container_id.lock().unwrap();
            *id = None;

            info!(
                "Stopped Prometheus server in Docker container: {}",
                self.config.docker_container_name
            );
        } else {
            let mut server_guard = self.embedded_server.lock().unwrap();
            if let Some(server) = server_guard.as_mut() {
                server.stop();
                info!("Stopped embedded Prometheus server");
            }
        }

        Ok(())
    }

    fn url(&self) -> String {
        format!(
            "http://{}:{}",
            if self.config.use_docker {
                "localhost"
            } else {
                &self.config.host
            },
            self.config.port
        )
    }

    async fn is_running(&self) -> Result<bool> {
        if self.config.use_docker {
            let container_id = {
                let id = self.container_id.lock().unwrap();
                match id.as_ref() {
                    Some(id) => id.clone(),
                    None => return Ok(false),
                }
            };

            return self
                .docker_manager
                .is_container_running(&container_id)
                .await;
        } else {
            // For embedded server, we don't have a good way to check if it's running
            // So we just return true if it's initialized
            let server = self.embedded_server.lock().unwrap();
            return Ok(server.is_some());
        }
    }

    async fn wait_until_ready(&self, timeout_secs: u64) -> Result<()> {
        let start_time = std::time::Instant::now();
        let timeout = Duration::from_secs(timeout_secs);

        while start_time.elapsed() < timeout {
            if self.is_running().await? {
                return Ok(());
            }
            sleep(Duration::from_millis(500)).await;
        }

        Err(crate::error::Error::Other(format!(
            "Prometheus server did not become ready within {} seconds",
            timeout_secs
        )))
    }
}

impl Drop for PrometheusServer {
    fn drop(&mut self) {
        let mut server_guard = self.embedded_server.lock().unwrap();
        if let Some(server) = server_guard.as_mut() {
            server.stop();
        }

        let container_id = self.container_id.lock().unwrap();
        if let Some(id) = container_id.as_ref() {
            // We can't use async in drop, so we just log a warning
            info!(
                "Note: Docker container {} will not be automatically stopped on drop",
                id
            );
        }
    }
}
