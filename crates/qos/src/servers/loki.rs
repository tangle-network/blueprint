use blueprint_core::{debug, info, warn};
use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tempfile::{TempDir, TempPath};

use crate::error::{Error, Result};
use crate::logging::LokiConfig;
use crate::servers::ServerManager;
use crate::servers::common::DockerManager;

/// Loki server configuration
#[derive(Clone, Debug)]
pub struct LokiServerConfig {
    /// Port to expose Loki on
    pub port: u16,

    /// Data directory
    pub data_dir: String,

    /// Container name
    pub container_name: String,

    /// Path to the Loki configuration file
    pub config_path: Option<String>,
}

impl Default for LokiServerConfig {
    fn default() -> Self {
        Self {
            port: 3100,
            data_dir: "/var/lib/loki".to_string(),
            container_name: "blueprint-loki".to_string(),
            config_path: None,
        }
    }
}

/// Loki server manager
pub struct LokiServer {
    /// Docker manager
    docker: DockerManager,

    /// Server configuration
    config: LokiServerConfig,

    /// Container ID
    container_id: Arc<Mutex<Option<String>>>,

    /// Temporary config file path
    temp_config_path: Arc<Mutex<Option<TempPath>>>,

    /// Temporary data directory
    temp_data_dir: Arc<Mutex<Option<TempDir>>>,
}

impl LokiServer {
    /// Create a new Loki server manager
    ///
    /// # Errors
    /// Returns an error if the Docker manager fails to create a new container
    pub fn new(config: LokiServerConfig) -> Result<Self> {
        Ok(Self {
            docker: DockerManager::new().map_err(|e| Error::DockerConnection(e.to_string()))?,
            config,
            container_id: Arc::new(Mutex::new(None)),
            temp_config_path: Arc::new(Mutex::new(None)),
            temp_data_dir: Arc::new(Mutex::new(None)),
        })
    }

    /// Get the Loki client configuration
    #[must_use]
    pub fn client_config(&self) -> LokiConfig {
        LokiConfig {
            url: format!("{}/loki/api/v1/push", self.url()),
            username: None,
            password: None,
            batch_size: 1024,
            labels: HashMap::new(),
            // TODO: Update once Loki is fixed
            timeout_secs: 5,
            otel_config: None,
        }
    }
}

impl ServerManager for LokiServer {
    async fn start(&self, network: Option<&str>, bind_ip: Option<String>) -> Result<()> {
        info!("Starting Loki server on port {}", self.config.port);

        let env_vars = HashMap::new();
        let mut args = Vec::new();

        let mut ports = HashMap::new();
        ports.insert("3100/tcp".to_string(), self.config.port.to_string());

        let mut volumes = HashMap::new();
        let temp_data_dir = tempfile::Builder::new().prefix("loki-data-").tempdir()?;
        fs::set_permissions(temp_data_dir.path(), fs::Permissions::from_mode(0o777))?;
        volumes.insert(
            temp_data_dir.path().to_str().unwrap().to_string(),
            "/loki".to_string(),
        );
        self.temp_data_dir.lock().unwrap().replace(temp_data_dir);

        if let Some(config_path) = &self.config.config_path {
            let config_content = std::fs::read_to_string(config_path)?;
            let mut temp_file = tempfile::NamedTempFile::new()?;
            std::io::Write::write_all(&mut temp_file, config_content.as_bytes())?;

            // Set permissions to be world-readable for the Docker container
            fs::set_permissions(temp_file.path(), fs::Permissions::from_mode(0o644))?;

            let temp_path = temp_file.into_temp_path();
            let temp_path_str = temp_path.to_str().unwrap().to_string();
            volumes.insert(
                temp_path_str.to_string(),
                "/etc/loki/config.yaml".to_string(),
            );
            // Keep the temp file alive until the container is started
            self.temp_config_path.lock().unwrap().replace(temp_path);
            args.push("-config.file=/etc/loki/config.yaml".to_string());
        }

        let health_check_cmd = Some(vec![
            "CMD-SHELL".to_string(),
            "wget -q -O /dev/null http://localhost:3100/ready".to_string(),
        ]);

        let container_id = self
            .docker
            .run_container(
                "grafana/loki:latest",
                &self.config.container_name,
                env_vars,
                ports,
                volumes,
                Some(args),
                None,
                health_check_cmd,
                bind_ip,
            )
            .await?;

        if let Some(net) = network {
            info!(
                "Connecting Loki container {} to network {}",
                &self.config.container_name, net
            );
            self.docker.connect_to_network(&container_id, net).await?;
        }

        {
            let mut id = self.container_id.lock().unwrap();
            *id = Some(container_id.clone());
        }

        // TODO: Update once Loki is fixed
        self.wait_until_ready(30).await?;

        info!("Loki server started successfully");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let container_id = {
            let id = self.container_id.lock().unwrap();
            match id.as_ref() {
                Some(id) => id.clone(),
                None => {
                    info!("Loki server is not running, nothing to stop.");
                    return Ok(());
                }
            }
        };

        info!("Stopping Loki server: {}", &self.config.container_name);
        self.docker
            .stop_and_remove_container(&container_id, &self.config.container_name)
            .await?;

        let mut id = self.container_id.lock().unwrap();
        *id = None;

        info!("Loki server stopped successfully.");
        Ok(())
    }

    fn url(&self) -> String {
        format!("http://localhost:{}", self.config.port)
    }

    async fn is_running(&self) -> Result<bool> {
        let container_id = {
            let id = self.container_id.lock().unwrap();
            match id.as_ref() {
                Some(id) => id.clone(),
                None => return Ok(false),
            }
        };

        self.docker.is_container_running(&container_id).await
    }

    async fn wait_until_ready(&self, timeout_secs: u64) -> Result<()> {
        let container_id = {
            let id = self.container_id.lock().unwrap();
            id.as_ref()
                .map(String::clone)
                .ok_or_else(|| Error::Generic("Loki server is not running".to_string()))?
        };

        info!("Waiting for Loki container to be healthy...");
        if let Err(e) = self
            .docker
            .wait_for_container_health(&container_id, timeout_secs)
            .await
        {
            warn!(
                "Loki container health check failed: {}. Proceeding with API check.",
                e
            );
        } else {
            info!("Loki container health check passed.");
        }

        info!("Waiting for Loki API to be responsive...");
        let client = reqwest::Client::new();
        let urls = [
            format!("{}/ready", self.url()),
            format!("{}/metrics", self.url()),
        ];
        let start_time = tokio::time::Instant::now();
        let timeout = Duration::from_secs(timeout_secs);

        loop {
            if start_time.elapsed() > timeout {
                return Err(Error::Generic(format!(
                    "Loki API did not become responsive within {} seconds.",
                    timeout_secs
                )));
            }

            for url in &urls {
                match client.get(url).send().await {
                    Ok(response) if response.status().is_success() => {
                        info!("Loki API is responsive at {}.", url);
                        return Ok(());
                    }
                    _ => {}
                }
            }

            debug!("Loki API not yet responsive. Retrying...");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
