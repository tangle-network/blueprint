use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio_retry::{strategy::ExponentialBackoff, Retry};
use tracing::{error, info};

use crate::error::{Error, Result};
use crate::logging::LokiConfig;
use crate::servers::common::DockerManager;
use crate::servers::ServerManager;

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
        
        let mut volumes = HashMap::new();
        volumes.insert(self.config.data_dir.clone(), "/loki".to_string());
        
        let container_id = self.docker.run_container(
            "grafana/loki:latest",
            &self.config.container_name,
            env_vars,
            ports,
            volumes,
        ).await?;
        
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
        
        self.docker.wait_for_container_health(&container_id, timeout_secs).await?;
        
        let client = reqwest::Client::new();
        let url = format!("{}/ready", self.url());
        
        let retry_strategy = ExponentialBackoff::from_millis(100)
            .factor(2)
            .max_delay(Duration::from_secs(1))
            .take(usize::try_from(timeout_secs).unwrap_or(30));
        
        Retry::spawn(retry_strategy, || async {
            match client.get(&url).timeout(Duration::from_secs(1)).send().await {
                Ok(response) if response.status().is_success() => {
                    info!("Loki API is responsive");
                    Ok(())
                }
                Ok(response) => {
                    error!("Loki API returned non-success status: {}", response.status());
                    Err(())
                }
                Err(e) => {
                    error!("Failed to connect to Loki API: {}", e);
                    Err(())
                }
            }
        })
        .await
        .map_err(|()| {
            Error::Other(format!(
                "Loki API did not become responsive within {} seconds",
                timeout_secs
            ))
        })
    }
}
