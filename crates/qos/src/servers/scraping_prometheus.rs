use serde::{Deserialize, Serialize};

/// Configuration for the Scraping Prometheus server.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScrapingPrometheusServerConfig {
    /// The host port to bind to.
    pub host_port: u16,
    /// Full address string for the metrics exposer to be monitored (e.g. "localhost:9091").
    pub scrape_target_address: String,
    /// Docker image name for Prometheus.
    pub image: String,
    /// Default network mode for the Docker container ("host", "bridge", etc).
    #[serde(default = "default_scrape_interval")]
    pub scrape_interval_seconds: u64,

    /// Docker network mode to use ("host", "bridge", etc.)
    /// Using "host" network mode can resolve connectivity issues on Linux
    #[serde(default = "default_network_mode")]
    pub network_mode: String,

    /// Whether to use extra_hosts to map host.docker.internal to host gateway IP
    /// This helps with Docker on Linux to resolve host.docker.internal
    #[serde(default = "default_use_host_gateway_mapping")]
    pub use_host_gateway_mapping: bool,

    /// Custom Docker network name for inter-container communication
    /// When specified, both Prometheus and Grafana containers will be connected to this network
    /// enabling container name-based service discovery
    #[serde(default)]
    pub custom_network: Option<String>,
}

fn default_image() -> String {
    "prom/prometheus:latest".to_string()
}

fn default_port() -> u16 {
    9090
}

fn default_scrape_interval() -> u64 {
    15
}

fn default_network_mode() -> String {
    "bridge".to_string()
}

fn default_use_host_gateway_mapping() -> bool {
    true
}

impl Default for ScrapingPrometheusServerConfig {
    fn default() -> Self {
        Self {
            image: default_image(),
            host_port: default_port(),
            scrape_target_address: "172.17.0.1:9091".to_string(),
            scrape_interval_seconds: default_scrape_interval(),
            network_mode: default_network_mode(),
            use_host_gateway_mapping: default_use_host_gateway_mapping(),
            custom_network: None,
        }
    }
}

use std::fs;
use std::{collections::HashMap, fs::File, io::Write};
use std::{path::PathBuf, sync::Arc, time::Duration};

use bollard::{
    Docker,
    container::{
        Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
        StopContainerOptions,
    },
    image::CreateImageOptions,
    models::{HostConfig, PortBinding},
    network::ConnectNetworkOptions,
};

use blueprint_core::{error, info, warn};
use futures::StreamExt;
use tokio::sync::Mutex;

use crate::error::{Error, Result};

const PROMETHEUS_CONTAINER_NAME: &str = "blueprint-scraping-prometheus";
const CONFIG_DIR: &str = "/tmp/blueprint-prometheus-config";
const CONFIG_FILE: &str = "prometheus.yml";

/// Represents a Prometheus server instance that actively scrapes metrics.
/// This server runs as a Docker container.
#[derive(Debug)]
pub struct ScrapingPrometheusServer {
    config: ScrapingPrometheusServerConfig,
    docker_client: Arc<Docker>,
    // Using a permanent config directory path instead of temporary files
    // This ensures the config file persists as long as needed for the container
    config_path: Arc<Mutex<Option<PathBuf>>>,
}

impl ScrapingPrometheusServer {
    /// Creates a new `ScrapingPrometheusServer`.
    pub fn new(config: ScrapingPrometheusServerConfig) -> Result<Self> {
        let docker_client = Arc::new(
            Docker::connect_with_local_defaults()
                .map_err(|e| Error::DockerConnection(e.to_string()))?,
        );
        Ok(Self {
            config,
            docker_client,
            config_path: Arc::new(Mutex::new(None)),
        })
    }

    fn generate_prometheus_config_file(&self) -> Result<PathBuf> {
        // Create a permanent directory for Prometheus configuration
        let config_dir = PathBuf::from(CONFIG_DIR);

        // Create directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .map_err(|e| Error::Io(format!("Failed to create config directory: {}", e)))?;
        }

        // Create the config file path
        let config_file_path = config_dir.join(CONFIG_FILE);

        // Make sure the scrape target address is correctly formatted
        let scrape_target = self.config.scrape_target_address.clone();

        // More robust configuration with additional settings for stability
        let scrape_configs = format!(
            "global:\n  scrape_interval: {}s\n  evaluation_interval: 15s\n  scrape_timeout: 10s\n\nscrape_configs:\n  - job_name: 'qos-service'\n    metrics_path: /metrics\n    scheme: http\n    static_configs:\n      - targets: ['{}']\n",
            self.config.scrape_interval_seconds, scrape_target
        );

        info!(
            "Writing Prometheus config to {}:\n{}",
            config_file_path.display(),
            scrape_configs
        );

        // Write the configuration to file
        let mut file = File::create(&config_file_path)
            .map_err(|e| Error::Io(format!("Failed to create config file: {}", e)))?;

        file.write_all(scrape_configs.as_bytes())
            .map_err(|e| Error::Io(format!("Failed to write config: {}", e)))?;

        file.flush()
            .map_err(|e| Error::Io(format!("Failed to flush config file: {}", e)))?;

        // Set permissions to ensure the file is readable by the container
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&config_file_path)
                .map_err(|e| Error::Io(format!("Failed to get file metadata: {}", e)))?
                .permissions();

            // Set permissions to rwxrwxrwx (777) to ensure Docker container can read it
            perms.set_mode(0o777);
            fs::set_permissions(&config_file_path, perms)
                .map_err(|e| Error::Io(format!("Failed to set file permissions: {}", e)))?;
        }

        info!(
            "Successfully wrote Prometheus config to {}",
            config_file_path.display()
        );

        Ok(config_file_path)
    }

    pub async fn start(&self) -> Result<()> {
        info!(
            "Starting scraping Prometheus server (image: {}, host_port: {}, target: {})...",
            self.config.image, self.config.host_port, self.config.scrape_target_address
        );

        // Ensure old container is removed if it exists
        let remove_options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });
        match self
            .docker_client
            .remove_container(PROMETHEUS_CONTAINER_NAME, remove_options)
            .await
        {
            Ok(_) => info!(
                "Removed existing Prometheus container: {}",
                PROMETHEUS_CONTAINER_NAME
            ),
            Err(bollard::errors::Error::DockerResponseServerError {
                status_code: 404, ..
            }) => {
                // Container does not exist, which is fine
            }
            Err(e) => {
                error!("Error removing existing Prometheus container: {}", e);
                // Continue, as creating a new one might still work or fail with a clearer error
            }
        }

        // Pull the image
        let mut stream = self.docker_client.create_image(
            Some(CreateImageOptions {
                from_image: self.config.image.clone(),
                ..Default::default()
            }),
            None,
            None,
        );
        while let Some(result) = stream.next().await {
            match result {
                Ok(info) => {
                    if let Some(status) = info.status {
                        blueprint_core::debug!("Image pull status: {}", status);
                    }
                    if let Some(error) = info.error {
                        return Err(Error::DockerOperation(format!(
                            "Failed to pull image: {}",
                            error
                        )));
                    }
                }
                Err(e) => {
                    return Err(Error::DockerOperation(format!(
                        "Image pull stream error: {}",
                        e
                    )));
                }
            }
        }
        info!(
            "Successfully pulled Prometheus image: {}",
            self.config.image
        );

        // Generate Prometheus configuration file
        let config_file_path = self.generate_prometheus_config_file()?;
        *self.config_path.lock().await = Some(config_file_path.clone());

        const CONTAINER_CONFIG_DIR: &str = "/etc/prometheus";
        const CONTAINER_CONFIG_PATH: &str = "/etc/prometheus/prometheus.yml";

        // Get the host config path as a string
        let host_config_path = config_file_path
            .to_str()
            .ok_or_else(|| Error::Generic("Failed to convert config path to string".to_string()))?
            .to_string();

        info!("Using config file at path: {}", host_config_path);

        let options = Some(CreateContainerOptions {
            name: PROMETHEUS_CONTAINER_NAME.to_string(),
            ..Default::default()
        });

        // Set up port bindings
        let port_binding = format!("{}/tcp", self.config.host_port);

        let mut port_bindings = HashMap::new();
        port_bindings.insert(
            port_binding.clone(),
            Some(vec![PortBinding {
                host_ip: Some("0.0.0.0".to_string()),
                host_port: Some(port_binding.clone()),
            }]),
        );

        // Create default host config first
        let mut host_config = HostConfig::default();

        // Then set only the specific fields we need
        host_config.port_bindings = Some(port_bindings);
        host_config.network_mode = Some("bridge".to_string());
        host_config.binds = Some(vec![format!(
            "{}:{}",
            host_config_path, CONTAINER_CONFIG_PATH
        )]);
        host_config.privileged = Some(false);
        host_config.publish_all_ports = Some(true);

        // Log the host config details for debugging
        info!(
            "Host config: port_bindings: {:?}, network_mode: {:?}",
            host_config.port_bindings, host_config.network_mode
        );

        // Add host.docker.internal mapping if enabled
        if self.config.use_host_gateway_mapping {
            // Get the Docker host gateway IP - typically 172.17.0.1
            let host_gateway = "host-gateway";
            host_config.extra_hosts = Some(vec![format!("host.docker.internal:{}", host_gateway)]);

            info!(
                "Added host.docker.internal mapping to {} for Prometheus container",
                host_gateway
            );
        }

        info!(
            "Creating Prometheus container with network_mode: {}, scraping target: {}",
            self.config.network_mode, self.config.scrape_target_address
        );

        // Store port bindings for logging before they're moved into the container config
        let port_bindings_for_log = format!("{:?}", host_config.port_bindings);

        // Add restart policy to the host config to ensure container restarts on failure
        host_config.restart_policy = Some(bollard::models::RestartPolicy {
            name: Some(bollard::models::RestartPolicyNameEnum::ON_FAILURE),
            maximum_retry_count: Some(5), // Increase retry count for more resilience
        });

        // Create a minimal container config with only essential options
        let container_config = Config {
            image: Some(self.config.image.clone()),
            exposed_ports: Some(
                [(port_binding, Default::default())]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            host_config: Some(host_config),
            // Use the absolute minimum required options for Prometheus to avoid any conflicts
            cmd: Some(vec![
                format!("--config.file={}", CONTAINER_CONFIG_PATH),
                "--storage.tsdb.path=/prometheus".to_string(),
                format!("--web.listen-address=0.0.0.0:{}", self.config.host_port),
            ]),
            // Set environment variables to improve stability
            env: Some(vec![
                "PROMETHEUS_DISABLE_METRICS_ENDPOINT=false".to_string(),
            ]),
            working_dir: Some("/prometheus".to_string()),
            ..Default::default()
        };

        // Add detailed logging of the container config using stored values
        info!("Creating Prometheus container with configuration:");
        info!("  - Network mode: {}", self.config.network_mode);
        info!("  - Port bindings: {}", port_bindings_for_log);
        info!("  - Exposed port: {}", port_bindings_for_log);
        info!(
            "  - Target address for scraping: {}",
            self.config.scrape_target_address
        );

        self.docker_client
            .create_container(options, container_config)
            .await
            .map_err(|e| Error::DockerOperation(e.to_string()))?;
        info!(
            "Successfully created Prometheus container: {}",
            PROMETHEUS_CONTAINER_NAME
        );

        self.docker_client
            .start_container(
                PROMETHEUS_CONTAINER_NAME,
                None::<StartContainerOptions<String>>,
            )
            .await
            .map_err(|e| Error::DockerOperation(e.to_string()))?;
        info!(
            "Scraping Prometheus server started successfully on port {}. Scraping: {}",
            self.config.host_port, self.config.scrape_target_address
        );

        // If a custom network is specified, connect the container to that network after creation
        // This is a more reliable approach than setting it during container creation
        if let Some(network_name) = &self.config.custom_network {
            info!(
                "Connecting Prometheus container to custom network: {}",
                network_name
            );
            match self
                .docker_client
                .connect_network(
                    network_name,
                    ConnectNetworkOptions::<String> {
                        container: PROMETHEUS_CONTAINER_NAME.to_string(),
                        endpoint_config: bollard::models::EndpointSettings::default(),
                    },
                )
                .await
            {
                Ok(_) => info!(
                    "Successfully connected Prometheus container to network: {}",
                    network_name
                ),
                Err(e) => warn!(
                    "Failed to connect to custom network {}: {}. Container will still work but with possible network limitations.",
                    network_name, e
                ),
            }

            // Add a small delay to allow network connection to stabilize
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        info!("Stopping scraping Prometheus server...");

        // Stop the container with a reasonable timeout
        match self
            .docker_client
            .stop_container(
                PROMETHEUS_CONTAINER_NAME,
                Some(StopContainerOptions { t: 30 }),
            )
            .await
        {
            Ok(_) => info!(
                "Successfully stopped Prometheus container: {}",
                PROMETHEUS_CONTAINER_NAME
            ),
            Err(e) => warn!(
                "Failed to stop Prometheus container (it may not be running): {}",
                e
            ),
        }

        // Remove the container with force option to ensure it's gone
        match self
            .docker_client
            .remove_container(
                PROMETHEUS_CONTAINER_NAME,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await
        {
            Ok(_) => info!(
                "Successfully removed Prometheus container: {}",
                PROMETHEUS_CONTAINER_NAME
            ),
            Err(e) => warn!(
                "Failed to remove Prometheus container (it may not exist): {}",
                e
            ),
        }

        // Clean up config directory if it exists
        if let Some(path) = self.config_path.lock().await.clone() {
            info!("Cleaning up config file: {}", path.display());
            if let Err(e) = fs::remove_file(&path) {
                warn!(
                    "Failed to remove config file (it may have been moved): {}",
                    e
                );
            }

            // Try to remove the parent directory too if it's our config directory
            if path
                .parent()
                .map_or(false, |p| p.to_string_lossy() == CONFIG_DIR)
            {
                if let Err(e) = fs::remove_dir_all(CONFIG_DIR) {
                    warn!("Failed to remove config directory: {}", e);
                } else {
                    info!("Successfully removed config directory: {}", CONFIG_DIR);
                }
            }
        }

        // Reset the config path
        *self.config_path.lock().await = None;

        Ok(())
    }

    pub fn url(&self) -> String {
        // If using a custom network, provide both container name URL and fallback IP address URL
        if self.config.custom_network.is_some() {
            // Container name URL for direct container-to-container communication
            format!(
                "http://{}:{}",
                PROMETHEUS_CONTAINER_NAME, self.config.host_port
            )
        } else {
            // Use explicit IPv4 address to avoid IPv6 issues
            format!("http://127.0.0.1:{}", self.config.host_port)
        }
    }

    pub fn container_name(&self) -> &'static str {
        PROMETHEUS_CONTAINER_NAME
    }

    pub fn container_url(&self) -> Option<String> {
        // Return container URL for Docker communication when using a custom network
        self.config.custom_network.as_ref().map(|_| {
            format!(
                "http://{}:{}",
                PROMETHEUS_CONTAINER_NAME, self.config.host_port
            )
        })
    }

    pub fn name(&self) -> String {
        "Scraping Prometheus Server".to_string()
    }
}
