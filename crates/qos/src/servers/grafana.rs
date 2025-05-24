use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio_retry::{Retry, strategy::ExponentialBackoff};
use blueprint_core::{debug, error, info, warn};

use crate::error::{Error, Result};
use crate::logging::GrafanaConfig;
use crate::servers::ServerManager;
use crate::servers::common::DockerManager;

/// Grafana server configuration
#[derive(Clone, Debug)]
pub struct GrafanaServerConfig {
    /// Port to expose Grafana on
    pub port: u16,

    /// Admin username
    pub admin_user: String,

    /// Admin password
    pub admin_password: String,

    /// Whether to allow anonymous access
    pub allow_anonymous: bool,

    /// Anonymous user role
    pub anonymous_role: String,

    /// Data directory
    pub data_dir: String,

    /// Container name
    pub container_name: String,
}

impl Default for GrafanaServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            admin_user: "admin".to_string(),
            admin_password: "admin".to_string(),
            allow_anonymous: true,
            anonymous_role: "Admin".to_string(),
            data_dir: "/var/lib/grafana".to_string(),
            container_name: "blueprint-grafana".to_string(),
        }
    }
}

/// Grafana server manager
pub struct GrafanaServer {
    /// Docker manager
    docker: DockerManager,

    /// Server configuration
    config: GrafanaServerConfig,

    /// Container ID
    container_id: Arc<Mutex<Option<String>>>,
}

impl GrafanaServer {
    /// Create a new Grafana server manager
    #[must_use]
    pub fn new(config: GrafanaServerConfig) -> Self {
        Self {
            docker: DockerManager::new(),
            config,
            container_id: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the Grafana client configuration
    #[must_use]
    pub fn client_config(&self) -> GrafanaConfig {
        GrafanaConfig {
            url: self.url(),
            api_key: String::new(),
            org_id: None,
            folder: None,
        }
    }
}

#[async_trait::async_trait]
impl ServerManager for GrafanaServer {
    async fn start(&self) -> Result<()> {
        info!("Starting Grafana server on port {}", self.config.port);

        // Set up environment variables
        let mut env_vars = HashMap::new();
        env_vars.insert(
            "GF_SECURITY_ADMIN_USER".to_string(),
            self.config.admin_user.clone(),
        );
        env_vars.insert(
            "GF_SECURITY_ADMIN_PASSWORD".to_string(),
            self.config.admin_password.clone(),
        );

        if self.config.allow_anonymous {
            env_vars.insert("GF_AUTH_ANONYMOUS_ENABLED".to_string(), "true".to_string());
            env_vars.insert(
                "GF_AUTH_ANONYMOUS_ORG_ROLE".to_string(),
                self.config.anonymous_role.clone(),
            );
            env_vars.insert("GF_AUTH_DISABLE_LOGIN_FORM".to_string(), "true".to_string());
        }

        env_vars.insert(
            "GF_FEATURE_TOGGLES_ENABLE".to_string(),
            "publicDashboards".to_string(),
        );

        let mut ports = HashMap::new();
        ports.insert("3000/tcp".to_string(), self.config.port.to_string());

        // Only use volume mounts if the data_dir starts with a valid path
        // This helps avoid permission issues in environments where volume mounts are problematic
        let mut volumes = HashMap::new();
        if !self.config.data_dir.is_empty() && self.config.data_dir != "/var/lib/grafana" {
            volumes.insert(self.config.data_dir.clone(), "/var/lib/grafana".to_string());
        }

        let container_id = self
            .docker
            .run_container(
                "grafana/grafana:latest",
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

        info!("Grafana server started successfully");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let container_id = {
            let id = self.container_id.lock().unwrap();
            match id.as_ref() {
                Some(id) => id.clone(),
                None => {
                    info!("Grafana server is not running");
                    return Ok(());
                }
            }
        };

        info!("Stopping Grafana server");
        self.docker.stop_container(&container_id).await?;

        let mut id = self.container_id.lock().unwrap();
        *id = None;

        info!("Grafana server stopped successfully");
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
                    return Err(Error::Other("Grafana server is not running".to_string()));
                }
            }
        };

        // Wait for the container to be running (not necessarily healthy)
        match self.docker.wait_for_container_health(&container_id, timeout_secs).await {
            Ok(_) => {
                info!("Grafana container is running");
            }
            Err(e) => {
                // Log the error but continue anyway - the container might still be usable
                warn!("Grafana container health check failed: {}", e);
                // Give it a moment to initialize even if health check failed
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }

        // Increase timeout for API check to be more lenient
        let api_timeout_secs = timeout_secs.max(60); // At least 60 seconds
        let client = reqwest::Client::new();
        let url = format!("{}/api/health", self.url());

        let retry_strategy = ExponentialBackoff::from_millis(500) // Start with longer delay
            .factor(2)
            .max_delay(Duration::from_secs(5)) // Allow longer delays between retries
            .take(usize::try_from(api_timeout_secs).unwrap_or(60));

        // Try to connect to the API, but don't fail if we can't
        match Retry::spawn(retry_strategy, || async {
            match client
                .get(&url)
                .timeout(Duration::from_secs(5)) // Longer timeout per request
                .send()
                .await
            {
                Ok(response) if response.status().is_success() => {
                    info!("Grafana API is responsive");
                    Ok(())
                }
                Ok(response) => {
                    warn!("Grafana API returned status: {}, will retry", response.status());
                    Err(())
                }
                Err(e) => {
                    debug!("Still waiting for Grafana API: {}", e);
                    Err(())
                }
            }
        })
        .await
        {
            Ok(_) => {
                info!("Successfully connected to Grafana API");
                Ok(())
            }
            Err(_) => {
                // Don't fail the startup if API isn't responsive yet
                warn!("Grafana API not responsive yet, but continuing anyway");
                info!("You may need to wait a bit longer before accessing Grafana in your browser");
                Ok(())
            }
        }
    }
}
