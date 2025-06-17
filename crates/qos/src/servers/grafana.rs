use blueprint_core::{debug, info, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::error::{Error, Result};
use crate::logging::GrafanaConfig;
use crate::logging::loki::LokiConfig;
use crate::servers::ServerManager;
use crate::servers::common::DockerManager;

const HEALTH_CHECK_TIMEOUT_SECS: u64 = 90;
const GRAFANA_IMAGE_NAME_FULL: &str = "grafana/grafana:10.4.3";

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

    /// Optional Loki configuration to be used by the Grafana client.
    pub loki_config: Option<LokiConfig>,
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
            loki_config: None,
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
    ///
    /// # Errors
    /// Returns an error if the Docker manager fails to create a new container
    pub fn new(config: GrafanaServerConfig) -> Result<Self> {
        Ok(Self {
            docker: DockerManager::new().map_err(|e| Error::DockerConnection(e.to_string()))?,
            config,
            container_id: Arc::new(Mutex::new(None)),
        })
    }

    /// Get the Grafana client configuration
    #[must_use]
    pub fn client_config(&self) -> GrafanaConfig {
        GrafanaConfig {
            url: self.url(),
            api_key: None,
            org_id: None,
            folder: None,
            admin_user: Some(self.config.admin_user.clone()),
            admin_password: Some(self.config.admin_password.clone()),
            loki_config: self.config.loki_config.clone(),
            prometheus_datasource_url: None,
        }
    }
}

impl ServerManager for GrafanaServer {
    async fn start(&self, network: Option<&str>, bind_ip: Option<String>) -> Result<()> {
        info!("Starting Grafana server on port {}", self.config.port);

        let mut env_vars = HashMap::new();
        env_vars.insert(
            "GF_SECURITY_ADMIN_USER".to_string(),
            self.config.admin_user.clone(),
        );
        env_vars.insert(
            "GF_SECURITY_ADMIN_PASSWORD".to_string(),
            self.config.admin_password.clone(),
        );
        env_vars.insert("GF_LOG_LEVEL".to_string(), "debug".to_string());

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

        let mut volumes = HashMap::new();
        if !self.config.data_dir.is_empty() && self.config.data_dir != "/var/lib/grafana" {
            volumes.insert(self.config.data_dir.clone(), "/var/lib/grafana".to_string());
        }

        let container_id = self
            .docker
            .run_container(
                GRAFANA_IMAGE_NAME_FULL,
                &self.config.container_name,
                env_vars,
                ports,
                volumes,
                Some(vec!["host.docker.internal:host-gateway".to_string()]),
                None,
                bind_ip,
            )
            .await?;

        if let Some(net) = network {
            info!(
                "Connecting Grafana container {} to network {}",
                &self.config.container_name, net
            );
            self.docker.connect_to_network(&container_id, net).await?;
        }

        {
            let mut id = self.container_id.lock().unwrap();
            *id = Some(container_id.clone());
        }

        self.wait_until_ready(HEALTH_CHECK_TIMEOUT_SECS).await?;

        info!("Grafana server started successfully");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let container_id = {
            let id = self.container_id.lock().unwrap();
            match id.as_ref() {
                Some(id) => id.clone(),
                None => {
                    info!("Grafana server is not running, nothing to stop.");
                    return Ok(());
                }
            }
        };

        info!("Stopping Grafana server: {}", &self.config.container_name);
        self.docker
            .stop_and_remove_container(&container_id, &self.config.container_name)
            .await?;

        let mut id = self.container_id.lock().unwrap();
        *id = None;

        info!("Grafana server stopped successfully.");
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
                .ok_or_else(|| Error::Generic("Grafana server is not running".to_string()))?
        };

        info!("Waiting for Grafana container to be healthy...");
        if let Err(e) = self
            .docker
            .wait_for_container_health(&container_id, timeout_secs)
            .await
        {
            warn!(
                "Grafana container health check failed: {}. Proceeding with API check.",
                e
            );
        } else {
            info!("Grafana container health check passed.");
        }

        info!("Waiting for Grafana API to be responsive...");
        let client = reqwest::Client::new();
        let url = format!("{}/api/health", self.url());
        let start_time = tokio::time::Instant::now();
        let timeout = Duration::from_secs(timeout_secs);

        loop {
            if start_time.elapsed() > timeout {
                return Err(Error::Generic(format!(
                    "Grafana API did not become responsive within {} seconds.",
                    timeout_secs
                )));
            }

            match client.get(&url).send().await {
                Ok(response) if response.status().is_success() => {
                    info!("Grafana API is responsive.");
                    return Ok(());
                }
                Ok(response) => {
                    debug!(
                        "Grafana API check failed with status: {}. Retrying...",
                        response.status()
                    );
                }
                Err(e) => {
                    debug!("Grafana API check failed with error: {}. Retrying...", e);
                }
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
