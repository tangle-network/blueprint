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

/// Configuration settings for a Prometheus metrics server.
///
/// This struct defines all the parameters needed to set up and run a Prometheus server,
/// either as a Docker container or as an embedded server within the application.
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

// Default values for PrometheusServerConfig
const DEFAULT_PROMETHEUS_PORT: u16 = 9090;
const DEFAULT_PROMETHEUS_HOST: &str = "0.0.0.0";
const DEFAULT_PROMETHEUS_DOCKER_IMAGE: &str = "prom/prometheus:latest";
const DEFAULT_PROMETHEUS_CONTAINER_NAME: &str = "blueprint-prometheus";
const PROMETHEUS_DOCKER_HEALTH_TIMEOUT_SECS: u64 = 120;

impl Default for PrometheusServerConfig {
    fn default() -> Self {
        Self {
            port: DEFAULT_PROMETHEUS_PORT,
            host: DEFAULT_PROMETHEUS_HOST.to_string(),
            use_docker: false,
            docker_image: DEFAULT_PROMETHEUS_DOCKER_IMAGE.to_string(),
            docker_container_name: DEFAULT_PROMETHEUS_CONTAINER_NAME.to_string(),
            config_path: None,
            data_path: None,
        }
    }
}

/// Manager for a Prometheus metrics server instance.
///
/// This struct handles the lifecycle of a Prometheus server, which can either run
/// as a Docker container or as an embedded server within the application process.
/// It provides methods for starting, stopping, and monitoring the server.
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

    /// The metrics registry provided by `EnhancedMetricsProvider` (if not using Docker)
    metrics_registry: Option<Arc<prometheus::Registry>>,

    /// The enhanced metrics provider, used to force flush OTEL metrics on scrape
    enhanced_metrics_provider: Arc<EnhancedMetricsProvider>,
}

impl PrometheusServer {
    /// Creates a new Prometheus server manager with the provided configuration and metrics registry.
    ///
    /// This constructor prepares the Prometheus server infrastructure but does not start the server.
    /// The actual server (Docker container or embedded) is started when `start()` is called.
    ///
    /// # Parameters
    /// * `config` - The configuration settings for the Prometheus server
    /// * `metrics_registry` - Optional Prometheus registry for the embedded server mode
    /// * `enhanced_metrics_provider` - Provider that generates metrics data
    ///
    /// # Errors
    /// Returns an error if the Docker manager connection fails to initialize
    pub fn new(
        config: PrometheusServerConfig,
        metrics_registry: Option<Arc<prometheus::Registry>>,
        enhanced_metrics_provider: Arc<EnhancedMetricsProvider>,
    ) -> Result<Self> {
        Ok(Self {
            config,
            docker_manager: DockerManager::new()
                .map_err(|e| crate::error::Error::DockerConnection(e.to_string()))?,
            container_id: Arc::new(Mutex::new(None)),
            embedded_server: Arc::new(Mutex::new(None)),
            metrics_registry,
            enhanced_metrics_provider,
        })
    }

    /// Creates and configures a new Docker container for the Prometheus server.
    ///
    /// Sets up a new container with appropriate port mappings, volume mounts, and configuration
    /// based on the server settings. This does not start the container, only creates it.
    ///
    /// # Parameters
    /// * `network` - Optional Docker network to attach the container to
    ///
    /// # Errors
    /// Returns an error if the Docker API fails to create the container
    ///
    /// # Panics
    /// Panics if mutex locks cannot be acquired
    pub async fn create_docker_container(&self) -> Result<()> {
        let env_vars = HashMap::new();

        let mut ports = HashMap::new();
        ports.insert(self.config.port.to_string(), self.config.port.to_string());

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

        let new_container_id_result = self
            .docker_manager
            .run_container(
                &self.config.docker_image,
                &self.config.docker_container_name,
                env_vars,
                ports,
                volumes,
                None,
                None,
                None,
                None,
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
                return Err(e);
            }
        }

        Ok(())
    }

    /// Returns a reference to the metrics registry used by the embedded Prometheus server.
    ///
    /// This registry contains all the metrics that are exposed by the embedded Prometheus server.
    /// Returns None if no registry was provided during initialization.
    #[must_use]
    pub fn registry(&self) -> Option<Arc<prometheus::Registry>> {
        self.metrics_registry.clone()
    }

    /// Returns the Docker container ID if the server is running in Docker mode.
    ///
    /// This identifier can be used to reference the specific Prometheus container for
    /// management operations via Docker API. The container ID is only available when the
    /// server is configured to run in Docker mode and after the container has been created.
    ///
    /// # Returns
    /// * `Some(String)` - Container ID if the server is using Docker and a container has been created
    /// * `None` - If the server is not using Docker, the container hasn't been created yet,
    ///   or if there was an error acquiring the lock on the container ID
    #[must_use]
    pub fn container_id(&self) -> Option<String> {
        self.container_id.lock().map(|id| id.clone()).ok()?
    }

    /// Checks if this server is configured to use Docker.
    ///
    /// The Prometheus server can operate in two modes:
    /// 1. Docker mode: Runs Prometheus in a separate Docker container, providing isolation
    ///    and easier management but requiring Docker to be available
    /// 2. Embedded mode: Runs a Prometheus-compatible metrics HTTP server directly within
    ///    the application process, with no external dependencies
    ///
    /// # Returns
    /// * `true` - If the server is configured to use Docker
    /// * `false` - If the server is using the embedded mode
    #[must_use]
    pub fn is_docker_based(&self) -> bool {
        self.config.use_docker
    }
}

impl ServerManager for PrometheusServer {
    /// Starts the Prometheus server in either Docker or embedded mode.
    ///
    /// * **`Docker`** – Creates (or reuses) a container, mounts configuration/data volumes,
    ///   optionally connects it to a network, and waits for the container health-check.
    /// * **`Embedded`** – Binds an in-process HTTP server to `host:port` and serves the
    ///   `/metrics` endpoint backed by the supplied registry.
    ///
    /// The call blocks until the server is ready.
    ///
    /// # Parameters
    /// * `network` – Optional Docker network name (`Docker` mode only).
    ///
    /// # Errors
    /// Returns an error if the container or server fails to start, health-check fails, the
    /// port is already in use, or required configuration is missing.
    ///
    ///
    /// * `network` - Optional Docker network name to connect the container to. Only used in `Docker` mode.
    ///   When provided, the container will be connected to this network to allow service discovery
    ///   between services (e.g., Prometheus to scrape metrics from other containers).
    ///
    /// # Errors
    /// Returns an error if:
    /// * Docker container creation or startup fails (permission issues, Docker daemon not running, image not found)
    /// * Port binding fails for the embedded server (port already in use or insufficient permissions)
    /// * Server startup times out or fails for any other reason
    /// * Configuration file generation fails or specified paths are invalid (Docker mode only)
    /// * Health check fails for Docker container
    /// * No metrics registry is provided for embedded server mode
    async fn start(&self, network: Option<&str>, bind_ip: Option<String>) -> Result<()> {
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
                if is_running {
                    info!(
                        "PrometheusServer::start: Container {} is already running.",
                        existing_id
                    );
                    current_container_id_val = Some(existing_id);
                    perform_health_check = false;
                } else {
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
                    // Reset container ID to indicate we need to create a new one
                    current_container_id_val = None;
                    *self.container_id.lock().unwrap() = None;
                }
            }

            if current_container_id_val.is_none() {
                info!(
                    "PrometheusServer::start: No existing container_id found or old one was not running. Creating new container."
                );
                let mut ports = std::collections::HashMap::new();
                ports.insert("9090/tcp".to_string(), self.config.port.to_string());

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

                let extra_hosts = vec!["host.docker.internal:host-gateway".to_string()];

                let new_id_result = self
                    .docker_manager
                    .run_container(
                        &self.config.docker_image,
                        &self.config.docker_container_name,
                        std::collections::HashMap::new(), // env_vars
                        ports,
                        volumes,
                        None, // cmd
                        Some(extra_hosts),
                        None, // health_check_cmd
                        bind_ip,
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
                if self
                    .docker_manager
                    .wait_for_container_health(
                        &final_id_for_connection_and_health_check,
                        PROMETHEUS_DOCKER_HEALTH_TIMEOUT_SECS,
                    )
                    .await
                    .is_err()
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
            {
                let guard = self.embedded_server.lock().unwrap();
                if guard.is_some() {
                    info!(
                        "Embedded Prometheus server on {}:{} already initialized.",
                        self.config.host, self.config.port
                    );
                    return Ok(());
                }
            }

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

            match std::net::TcpListener::bind(&bind_address_for_new_server) {
                Ok(listener) => {
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

            server_instance.start().await?;

            {
                let mut guard = self.embedded_server.lock().unwrap();
                *guard = Some(server_instance);
            }

            info!(
                "Successfully started embedded Prometheus server on {}",
                bind_address_for_new_server
            );
        }

        Ok(())
    }

    /// Stops the running Prometheus server and performs necessary cleanup.
    ///
    /// This method safely terminates the Prometheus server instance based on its operational mode:
    ///
    /// ## Docker Mode
    /// When running as a Docker container:
    /// * Retrieves the container ID from internal state
    /// * Stops the Docker container using the Docker API
    /// * Removes the container to free up resources
    /// * Clears the internal container ID reference
    /// * Handles cases where container is already stopped gracefully
    ///
    /// ## Embedded Mode
    /// When running as an embedded server:
    /// * Acquires lock on the server instance
    /// * Triggers a graceful shutdown of the HTTP server
    /// * Waits for in-flight requests to complete
    /// * Releases resources associated with the server
    ///
    /// The method is idempotent and safe to call multiple times, even if the server
    /// is not running.
    ///
    /// # Errors
    /// Returns an error if:
    /// * Docker API encounters errors when stopping or removing the container
    /// * Communication with Docker daemon fails
    /// * Container removal fails due to permission issues
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

    /// Returns the fully qualified URL where the Prometheus server can be accessed.
    ///
    /// Constructs a URL using the configured host and port values in the format:
    /// `http://{host}:{port}`
    ///
    /// This URL can be used to:
    /// * Access the Prometheus web UI for manual query and visualization
    /// * Configure Grafana datasources programmatically
    /// * Set up health checks or monitoring of the Prometheus instance
    /// * Access the Prometheus HTTP API for programmatic queries
    ///
    /// The URL is valid regardless of whether the server is running in Docker mode
    /// or embedded mode, as both modes expose the same interface on the configured
    /// host:port combination.
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

    /// Checks if the Prometheus server is currently running.
    ///
    /// For Docker-based servers, checks the container status.
    /// For embedded servers, checks if the server instance exists.
    ///
    /// # Errors
    /// Returns an error if checking the server status fails or if
    /// the Docker API reports an error.
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

        let server = self.embedded_server.lock().unwrap();
        Ok(server.is_some())
    }

    /// Waits until the Prometheus server is ready to accept connections.
    ///
    /// Periodically checks the server status until it's ready or until the timeout expires.
    /// For Docker-based servers, performs HTTP health checks.
    /// For embedded servers, verifies the server is bound and responding.
    ///
    /// # Parameters
    /// * `timeout_secs` - Maximum time to wait in seconds
    ///
    /// # Errors
    /// Returns an error if the server fails to become ready within the timeout period
    /// or if health checks fail.
    async fn wait_until_ready(&self, timeout_secs: u64) -> Result<()> {
        if self.config.use_docker {
            let container_id = {
                let id = self.container_id.lock().unwrap();
                id.as_ref().map(String::clone).ok_or_else(|| {
                    crate::error::Error::Generic("Prometheus container not running".to_string())
                })?
            };

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
                    _ => {}
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        } else {
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
        let final_container_id = final_container_id_check.clone();
        if let Some(id) = final_container_id {
            info!(
                "Note: Docker container {} will not be automatically stopped on drop",
                id
            );
        }
    }
}
