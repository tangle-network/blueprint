use blueprint_core::{error, info, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

use crate::error::Result;
use crate::metrics::EnhancedMetricsProvider;
use crate::metrics::prometheus::server::PrometheusServer as PrometheusMetricsServer;
use crate::servers::ServerManager;
use crate::servers::common::DockerManager;

/// Configuration for the Prometheus server
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
            use_docker: false,
            docker_image: "prom/prometheus:latest".to_string(),
            docker_container_name: "blueprint-prometheus".to_string(),
            config_path: None,
            data_path: None,
        }
    }
}

/// Prometheus server manager
#[derive(Clone)]
pub struct PrometheusServer {
    /// The configuration for the Prometheus server
    config: PrometheusServerConfig,

    /// The Docker manager for the Prometheus server (if using Docker)
    docker_manager: DockerManager,

    /// The container ID for the Prometheus server (if using Docker)
    container_id: Arc<Mutex<Option<String>>>,

    /// The embedded Prometheus server (if not using Docker)
    embedded_server: Arc<Mutex<Option<PrometheusMetricsServer>>>,

    /// The metrics registry provided by EnhancedMetricsProvider (if not using Docker)
    metrics_registry: Option<Arc<prometheus::Registry>>,

    /// The enhanced metrics provider, used to force flush OTEL metrics on scrape
    enhanced_metrics_provider: Arc<EnhancedMetricsProvider>,
}

impl PrometheusServer {
    /// Create a new Prometheus server manager
    #[must_use]
    pub fn new(
        config: PrometheusServerConfig,
        metrics_registry: Option<Arc<prometheus::Registry>>,
        enhanced_metrics_provider: Arc<EnhancedMetricsProvider>,
    ) -> Self {
        Self {
            config,
            docker_manager: DockerManager::new(),
            container_id: Arc::new(Mutex::new(None)),
            embedded_server: Arc::new(Mutex::new(None)),
            metrics_registry,
            enhanced_metrics_provider,
        }
    }

    /// Create a new Docker container for the Prometheus server
    ///
    /// # Errors
    /// Returns an error if the container creation fails
    ///
    /// # Panics
    /// Panics if mutex locks cannot be acquired
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
        let new_container_id_result = self
            .docker_manager
            .run_container(
                &self.config.docker_image,
                &self.config.docker_container_name,
                env_vars,
                ports,
                volumes,
                None, // extra_hosts
                None, // health_check_cmd
            )
            .await;

        match new_container_id_result {
            Ok(id) => {
                info!(
                    "PrometheusServer::start: docker_manager.run_container succeeded. Raw new_container_id: '{}'",
                    id
                );
                if id.trim().is_empty() {
                    error!(
                        "PrometheusServer::start: docker_manager.run_container returned an EMPTY string for container ID."
                    );
                    return Err(crate::error::Error::Other(
                        "Docker manager returned empty container ID for Prometheus".to_string(),
                    ));
                }
                let mut id_guard = self.container_id.lock().unwrap();
                *id_guard = Some(id.clone());
                info!(
                    "PrometheusServer::start: Stored new container_id into self.container_id. Current self.container_id: {:?}",
                    *id_guard
                );
            }
            Err(e) => {
                error!(
                    "PrometheusServer::start: docker_manager.run_container FAILED: {}",
                    e
                );
                return Err(e); // Propagate the error from run_container
            }
        }

        Ok(())
    }

    /// Get the metrics registry used by the embedded Prometheus server
    pub fn registry(&self) -> Option<Arc<prometheus::Registry>> {
        self.metrics_registry.clone()
    }

    pub fn is_docker_based(&self) -> bool {
        self.config.use_docker
    }
}

impl ServerManager for PrometheusServer {
    async fn start(&self, network: Option<&str>) -> Result<()> {
        // Temporary variable to hold container ID to avoid holding MutexGuard across awaits
        let mut current_container_id_val: Option<String> = None;
        if self.config.use_docker {
            info!(
                "PrometheusServer::start: Attempting to run Docker container '{}' with name '{}' on network {:?}. Ports: {:?}, Config mount: {:?} -> /etc/prometheus/prometheus.yml",
                self.config.docker_image,
                self.config.docker_container_name,
                network,
                self.config.port,
                self.config.config_path
            );
            let mut perform_health_check = true;

            // Step 1: Check existing container ID status without holding lock over await
            let initial_id_check = self.container_id.lock().unwrap().clone();

            if let Some(existing_id) = initial_id_check {
                info!(
                    "PrometheusServer::start: Found existing container_id: {}. Checking if it's running.",
                    existing_id
                );
                let is_running = self
                    .docker_manager
                    .is_container_running(&existing_id)
                    .await?;
                if !is_running {
                    warn!(
                        "PrometheusServer::start: Container {} was found but is not running. Attempting to remove and recreate.",
                        existing_id
                    );
                    if let Err(e) = self
                        .docker_manager
                        .stop_and_remove_container(&existing_id, &self.config.docker_container_name)
                        .await
                    {
                        warn!(
                            "PrometheusServer::start: Failed to remove non-running container {}: {}. Proceeding to create a new one.",
                            existing_id, e
                        );
                    }
                    // Mark that we need to create a new one by ensuring current_container_id_val remains None
                    current_container_id_val = None;
                    // also clear it from the shared state
                    *self.container_id.lock().unwrap() = None;
                } else {
                    info!(
                        "PrometheusServer::start: Container {} is already running.",
                        existing_id
                    );
                    current_container_id_val = Some(existing_id);
                    perform_health_check = false; // Already running, assume healthy or rely on external checks for now
                }
            } else {
                // No initial ID, so current_container_id_val remains None, signaling creation
            }

            // Step 2: Create container if needed (current_container_id_val is None)
            if current_container_id_val.is_none() {
                info!(
                    "PrometheusServer::start: No existing container_id found or old one was not running. Creating new container."
                );
                let mut ports = std::collections::HashMap::new();
                ports.insert(format!("9090/tcp"), self.config.port.to_string());

                let mut volumes = std::collections::HashMap::new();
                if let Some(config_host_path) = &self.config.config_path {
                    volumes.insert(
                        config_host_path.clone(),
                        "/etc/prometheus/prometheus.yml".to_string(),
                    );
                }
                if let Some(data_host_path) = &self.config.data_path {
                    volumes.insert(data_host_path.clone(), "/prometheus".to_string());
                }

                // Add host.docker.internal mapping for Linux, required for the container to find the host.
                let extra_hosts = vec!["host.docker.internal:host-gateway".to_string()];

                let new_id_result = self
                    .docker_manager
                    .run_container(
                        &self.config.docker_image,
                        &self.config.docker_container_name,
                        std::collections::HashMap::new(), // env_vars
                        ports,                            // ports_map
                        volumes,                          // volumes_map
                        Some(extra_hosts),                // extra_hosts
                        None,                             // health_check_cmd
                    )
                    .await;

                match new_id_result {
                    Ok(id) => {
                        if id.trim().is_empty() {
                            error!(
                                "PrometheusServer::start: Docker manager returned an EMPTY string for container ID."
                            );
                            return Err(crate::error::Error::Other(
                                "Docker manager returned empty container ID for Prometheus"
                                    .to_string(),
                            ));
                        }
                        info!(
                            "PrometheusServer::start: Successfully created container with ID: {}",
                            id
                        );
                        current_container_id_val = Some(id.clone());
                        // Update shared state
                        *self.container_id.lock().unwrap() = Some(id);
                    }
                    Err(e) => {
                        error!(
                            "PrometheusServer::start: Failed to run Prometheus container: {}",
                            e
                        );
                        return Err(e);
                    }
                }
            }

            // Step 3: Network connection and health check using current_container_id_val
            let final_id_for_connection_and_health_check =
                current_container_id_val.clone().ok_or_else(|| {
                    crate::error::Error::Other(
                        "Prometheus container ID unexpectedly None after creation/check logic"
                            .to_string(),
                    )
                })?;

            if let Some(net) = network {
                info!(
                    "Connecting Prometheus container {} ({}) to network {}",
                    &self.config.docker_container_name,
                    final_id_for_connection_and_health_check,
                    net
                );
                self.docker_manager
                    .connect_to_network(&final_id_for_connection_and_health_check, net)
                    .await?;
            }

            if perform_health_check {
                info!(
                    "Performing health check for Prometheus container {} ({})",
                    &self.config.docker_container_name, final_id_for_connection_and_health_check
                );
                if !self
                    .docker_manager
                    .wait_for_container_health(&final_id_for_connection_and_health_check, 120)
                    .await
                    .is_ok()
                {
                    let err_msg = format!(
                        "Prometheus Docker container {} ({}) did not become healthy.",
                        self.config.docker_container_name, final_id_for_connection_and_health_check
                    );
                    error!("{}", err_msg);
                    return Err(crate::error::Error::Other(format!(
                        "Prometheus container ({}) health check failed: {}",
                        final_id_for_connection_and_health_check, err_msg
                    )));
                }
                info!(
                    "Prometheus Docker container {} ({}) is healthy.",
                    &self.config.docker_container_name, final_id_for_connection_and_health_check
                );
            } else {
                info!(
                    "Skipping health check for already running Prometheus container {} ({})",
                    &self.config.docker_container_name, final_id_for_connection_and_health_check
                );
            }
        } else {
            // Logic for non-Docker (embedded) server
            // Check if already started
            {
                let guard = self.embedded_server.lock().unwrap();
                if guard.is_some() {
                    info!(
                        "Embedded Prometheus server on {}:{} already initialized.",
                        self.config.host, self.config.port
                    );
                    return Ok(());
                }
            } // Guard dropped here

            // Prepare for start if not started
            let registry_arc_clone;
            let bind_address_for_new_server;

            if let Some(registry) = &self.metrics_registry {
                registry_arc_clone = registry.clone();
                bind_address_for_new_server = format!("{}:{}", self.config.host, self.config.port);
                info!(
                    "Attempting to start embedded Prometheus server on {} using provided registry",
                    bind_address_for_new_server
                );
            } else {
                return Err(crate::error::Error::Other(
                    "Metrics registry not provided for embedded Prometheus server".to_string(),
                ));
            }

            // Pre-bind check to ensure the port is not already in use
            match std::net::TcpListener::bind(&bind_address_for_new_server) {
                Ok(listener) => {
                    // Port is free, drop the listener immediately so Axum can bind
                    drop(listener);
                    info!(
                        "Port {} is free, proceeding to start embedded Prometheus server.",
                        bind_address_for_new_server
                    );
                }
                Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
                    error!(
                        "Failed to bind embedded Prometheus server to {}: Address already in use.",
                        bind_address_for_new_server
                    );
                    return Err(crate::error::Error::Other(format!(
                        "Address {} already in use for embedded Prometheus server",
                        bind_address_for_new_server
                    )));
                }
                Err(e) => {
                    error!(
                        "Failed to perform pre-bind check for embedded Prometheus server on {}: {}",
                        bind_address_for_new_server, e
                    );
                    return Err(crate::error::Error::Other(format!(
                        "Failed pre-bind check for {}: {}",
                        bind_address_for_new_server, e
                    )));
                }
            }

            let mut server_instance = PrometheusMetricsServer::new(
                registry_arc_clone,
                self.enhanced_metrics_provider.clone(),
                bind_address_for_new_server.clone(),
            );

            server_instance.start().await?; // Async operation, no locks held from self.embedded_server

            // Store the started server
            {
                let mut guard = self.embedded_server.lock().unwrap();
                *guard = Some(server_instance);
            } // Guard dropped here

            info!(
                "Successfully started embedded Prometheus server on {}",
                bind_address_for_new_server
            );
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
                        info!("Prometheus Docker container is not running, nothing to stop.");
                        return Ok(());
                    }
                }
            };

            info!(
                "Stopping Prometheus server in Docker container: {}",
                &self.config.docker_container_name
            );
            self.docker_manager
                .stop_and_remove_container(&container_id, &self.config.docker_container_name)
                .await?;

            let mut id = self.container_id.lock().unwrap();
            *id = None;

            info!("Prometheus Docker container stopped successfully.");
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
        }

        // For embedded server, we don't have a good way to check if it's running
        // So we just return true if it's initialized
        let server = self.embedded_server.lock().unwrap();
        Ok(server.is_some())
    }

    async fn wait_until_ready(&self, timeout_secs: u64) -> Result<()> {
        if self.config.use_docker {
            let container_id = {
                let id = self.container_id.lock().unwrap();
                id.as_ref().map(String::clone).ok_or_else(|| {
                    crate::error::Error::Generic("Prometheus container not running".to_string())
                })?
            };

            // First, wait for the container to be healthy.
            info!("Waiting for Prometheus container to be healthy...");
            if let Err(e) = self
                .docker_manager
                .wait_for_container_health(&container_id, timeout_secs)
                .await
            {
                warn!(
                    "Prometheus container health check failed: {}. Proceeding with API check.",
                    e
                );
            } else {
                info!("Prometheus container health check passed.");
            }

            // Second, wait for the API to be responsive.
            info!("Waiting for Prometheus API to be responsive...");
            let client = reqwest::Client::new();
            let url = format!("{}/-/ready", self.url());
            let start_time = tokio::time::Instant::now();
            let timeout = Duration::from_secs(timeout_secs);

            loop {
                if start_time.elapsed() > timeout {
                    return Err(crate::error::Error::Generic(format!(
                        "Prometheus API did not become responsive within {} seconds.",
                        timeout_secs
                    )));
                }

                match client.get(&url).send().await {
                    Ok(response) if response.status().is_success() => {
                        info!("Prometheus API is responsive.");
                        return Ok(());
                    }
                    _ => {
                        // Still waiting, sleep and retry
                    }
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        } else {
            // Logic for embedded server
            let start_time = std::time::Instant::now();
            let timeout = Duration::from_secs(timeout_secs);

            while start_time.elapsed() < timeout {
                if self.is_running().await? {
                    info!("Embedded Prometheus server is running.");
                    return Ok(());
                }
                sleep(Duration::from_millis(500)).await;
            }

            Err(crate::error::Error::Generic(format!(
                "Embedded Prometheus server did not become ready within {} seconds",
                timeout_secs
            )))
        }
    }
}

impl Drop for PrometheusServer {
    fn drop(&mut self) {
        let mut server_guard = self.embedded_server.lock().unwrap();
        if let Some(server) = server_guard.as_mut() {
            server.stop();
        }

        let final_container_id_check = self.container_id.lock().unwrap();
        info!(
            "PrometheusServer::start: For health check, read self.container_id as: {:?}",
            *final_container_id_check
        );
        let final_container_id = final_container_id_check.clone(); // Get the ID again after potential creation
        if let Some(id) = final_container_id {
            // We can't use async in drop, so we just log a warning
            info!(
                "Note: Docker container {} will not be automatically stopped on drop",
                id
            );
        }
    }
}
