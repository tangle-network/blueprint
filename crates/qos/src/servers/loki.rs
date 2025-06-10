use blueprint_core::{debug, info, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;


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
}

impl Default for LokiServerConfig {
    fn default() -> Self {
        Self {
            port: 3100,
            data_dir: "/var/lib/loki".to_string(),
            container_name: "blueprint-loki".to_string(),
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
}

impl LokiServer {
    /// Create a new Loki server manager
    #[must_use]
    pub fn new(config: LokiServerConfig) -> Self {
        Self {
            docker: DockerManager::new(),
            config,
            container_id: Arc::new(Mutex::new(None)),
        }
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
            timeout_secs: 30,
            otel_config: None,
        }
    }
}

impl ServerManager for LokiServer {
    async fn start(&self) -> Result<()> {
        info!("Starting Loki server on port {}", self.config.port);

        let env_vars = HashMap::new();

        let mut ports = HashMap::new();
        ports.insert("3100/tcp".to_string(), self.config.port.to_string());

        // Only use volume mounts if the data_dir starts with a valid path
        // This helps avoid permission issues in environments where volume mounts are problematic
        let mut volumes = HashMap::new();
        if !self.config.data_dir.is_empty() && self.config.data_dir != "/loki" {
            volumes.insert(self.config.data_dir.clone(), "/loki".to_string());
        }

        let container_id = self
            .docker
            .run_container(
                "grafana/loki:latest",
                &self.config.container_name,
                env_vars,
                ports,
                volumes,
                None, // extra_hosts
                None, // health_check_cmd
            )
            .await?;

        {
            let mut id = self.container_id.lock().unwrap();
            *id = Some(container_id.clone());
        }

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

        // First, wait for the container to be considered healthy by Docker.
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

        // Second, wait for the API to be responsive on any of its health endpoints.
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
                    _ => { // Catches both Err and non-success status
                        // Continue to the next URL or next sleep cycle
                    }
                }
            }
            
            debug!("Loki API not yet responsive. Retrying...");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
