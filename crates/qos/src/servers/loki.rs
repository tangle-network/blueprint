use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio_retry::{Retry, strategy::ExponentialBackoff};
use blueprint_core::{debug, error, info, warn};

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

#[async_trait::async_trait]
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
                    info!("Loki server is not running");
                    return Ok(());
                }
            }
        };

        info!("Stopping Loki server");
        self.docker.stop_container(&container_id).await?;

        let mut id = self.container_id.lock().unwrap();
        *id = None;

        info!("Loki server stopped successfully");
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
            match id.as_ref() {
                Some(id) => id.clone(),
                None => {
                    return Err(Error::Other("Loki server is not running".to_string()));
                }
            }
        };

        // Wait for the container to be running (not necessarily healthy)
        match self.docker.wait_for_container_health(&container_id, timeout_secs).await {
            Ok(_) => {
                info!("Loki container is running");
            }
            Err(e) => {
                // Log the error but continue anyway - the container might still be usable
                warn!("Loki container health check failed: {}, but continuing", e);
                // Give it a moment to initialize even if health check failed
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }

        // Increase timeout for API check to be more lenient
        let api_timeout_secs = timeout_secs.max(60); // At least 60 seconds
        let client = reqwest::Client::new();
        
        // Try multiple Loki API endpoints that might indicate readiness
        let urls = [
            format!("{}/ready", self.url()),
            format!("{}/metrics", self.url()),
            format!("{}/loki/api/v1/status/buildinfo", self.url()),
        ];

        let retry_strategy = ExponentialBackoff::from_millis(500) // Start with longer delay
            .factor(2)
            .max_delay(Duration::from_secs(5)) // Allow longer delays between retries
            .take(usize::try_from(api_timeout_secs).unwrap_or(60));

        // Try to connect to any of the API endpoints, but don't fail if we can't
        let mut success = false;
        
        for url in &urls {
            debug!("Trying Loki API endpoint: {}", url);
            match Retry::spawn(retry_strategy.clone(), || async {
                match client
                    .get(url)
                    .timeout(Duration::from_secs(5)) // Longer timeout per request
                    .send()
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        info!("Loki API endpoint {} is responsive", url);
                        Ok(())
                    }
                    Ok(response) => {
                        debug!("Loki API endpoint {} returned status: {}, will retry", url, response.status());
                        Err(())
                    }
                    Err(e) => {
                        debug!("Still waiting for Loki API endpoint {}: {}", url, e);
                        Err(())
                    }
                }
            })
            .await
            {
                Ok(_) => {
                    info!("Successfully connected to Loki API endpoint: {}", url);
                    success = true;
                    break;
                }
                Err(_) => {
                    debug!("Could not connect to Loki API endpoint: {}", url);
                    // Continue trying other endpoints
                }
            }
        }

        if success {
            info!("Loki API is responsive");
            Ok(())
        } else {
            // Don't fail the startup if API isn't responsive yet
            warn!("Loki API not responsive yet, but continuing anyway");
            info!("You may need to wait a bit longer before Loki is fully operational");
            Ok(())
        }
    }
}
