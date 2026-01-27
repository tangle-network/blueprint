//! Core deployment tracking implementation

use super::cleanup::*;
use super::types::{CleanupHandler, DeploymentRecord, DeploymentStatus, DeploymentType};
use crate::core::error::{Error, Result};
use blueprint_core::{debug, error, info, warn};
use blueprint_std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use chrono::Utc;
use tokio::sync::RwLock;

/// Global deployment tracker for mapping Blueprint instances to infrastructure
pub struct DeploymentTracker {
    /// Active deployments indexed by Blueprint instance ID
    deployments: Arc<RwLock<HashMap<String, DeploymentRecord>>>,
    /// Persistent state file path
    state_file: PathBuf,
    /// Cleanup handlers for different deployment types
    cleanup_handlers: Arc<RwLock<HashMap<DeploymentType, Box<dyn CleanupHandler>>>>,
}

impl DeploymentTracker {
    /// Create a new deployment tracker
    pub async fn new(state_dir: &Path) -> Result<Self> {
        let state_file = state_dir.join("deployment_state.json");

        // Load existing state if available
        let deployments = if state_file.exists() {
            Self::load_state(&state_file).await?
        } else {
            HashMap::new()
        };

        let mut tracker = Self {
            deployments: Arc::new(RwLock::new(deployments)),
            state_file,
            cleanup_handlers: Arc::new(RwLock::new(HashMap::new())),
        };

        // Register default cleanup handlers
        tracker.register_default_handlers().await;

        Ok(tracker)
    }

    /// Register a new deployment
    pub async fn register_deployment(
        &self,
        blueprint_id: String,
        deployment: DeploymentRecord,
    ) -> Result<()> {
        info!(
            "Registering deployment for Blueprint instance: {}",
            blueprint_id
        );

        let mut deployments = self.deployments.write().await;
        deployments.insert(blueprint_id.clone(), deployment.clone());
        drop(deployments);

        // Persist state
        self.save_state().await?;

        // Schedule TTL check if applicable
        if let Some(ttl) = deployment.ttl_seconds {
            self.schedule_ttl_cleanup(blueprint_id, ttl).await;
        }

        Ok(())
    }

    /// Handle Blueprint termination event
    pub async fn handle_termination(&self, blueprint_id: &str) -> Result<()> {
        info!(
            "Handling termination for Blueprint instance: {}",
            blueprint_id
        );

        let deployments = self.deployments.read().await;
        let deployment = deployments
            .get(blueprint_id)
            .ok_or_else(|| {
                Error::ConfigurationError(format!("No deployment found for {blueprint_id}"))
            })?
            .clone();
        drop(deployments);

        // Perform cleanup
        self.cleanup_deployment(blueprint_id, &deployment).await?;

        // Remove from tracking
        let mut deployments = self.deployments.write().await;
        deployments.remove(blueprint_id);
        drop(deployments);

        // Update persistent state
        self.save_state().await?;

        Ok(())
    }

    /// Handle TTL expiry
    pub async fn handle_ttl_expiry(&self, blueprint_id: &str) -> Result<()> {
        info!(
            "Handling TTL expiry for Blueprint instance: {}",
            blueprint_id
        );

        let deployments = self.deployments.read().await;
        if let Some(deployment) = deployments.get(blueprint_id) {
            let now = Utc::now();
            if let Some(expiry) = deployment.expires_at {
                if now >= expiry {
                    info!("TTL expired for {}, initiating cleanup", blueprint_id);
                    drop(deployments);
                    return self.handle_termination(blueprint_id).await;
                } else {
                    debug!(
                        "TTL not yet expired for {} (expires at {})",
                        blueprint_id, expiry
                    );
                }
            }
        }

        Ok(())
    }

    /// Cleanup a deployment
    async fn cleanup_deployment(
        &self,
        blueprint_id: &str,
        deployment: &DeploymentRecord,
    ) -> Result<()> {
        info!(
            "Cleaning up deployment: {} (type: {:?})",
            blueprint_id, deployment.deployment_type
        );

        let handlers = self.cleanup_handlers.read().await;
        let handler = handlers.get(&deployment.deployment_type).ok_or_else(|| {
            Error::ConfigurationError(format!("No handler for {:?}", deployment.deployment_type))
        })?;

        // Perform cleanup with retries
        let mut attempts = 0;
        let max_attempts = 3;

        while attempts < max_attempts {
            match handler.cleanup(deployment).await {
                Ok(_) => {
                    info!("Successfully cleaned up deployment: {}", blueprint_id);

                    // Send notification if configured
                    if let Some(webhook) = &deployment.cleanup_webhook {
                        self.send_cleanup_notification(webhook, blueprint_id, "success")
                            .await;
                    }

                    return Ok(());
                }
                Err(e) => {
                    attempts += 1;
                    error!(
                        "Cleanup attempt {} failed for {}: {}",
                        attempts, blueprint_id, e
                    );

                    if attempts >= max_attempts {
                        // Send failure notification
                        if let Some(webhook) = &deployment.cleanup_webhook {
                            self.send_cleanup_notification(webhook, blueprint_id, "failed")
                                .await;
                        }
                        return Err(e);
                    }

                    // Wait before retry
                    tokio::time::sleep(tokio::time::Duration::from_secs(5 * attempts)).await;
                }
            }
        }

        Ok(())
    }

    /// Schedule TTL-based cleanup
    async fn schedule_ttl_cleanup(&self, blueprint_id: String, ttl_seconds: u64) {
        let tracker = self.clone();

        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(ttl_seconds)).await;

            if let Err(e) = tracker.handle_ttl_expiry(&blueprint_id).await {
                error!("Failed to handle TTL expiry for {}: {}", blueprint_id, e);
            }
        });
    }

    /// Register default cleanup handlers
    async fn register_default_handlers(&mut self) {
        let mut handlers = self.cleanup_handlers.write().await;

        // Local deployment handlers
        handlers.insert(DeploymentType::LocalDocker, Box::new(LocalDockerCleanup));
        handlers.insert(
            DeploymentType::LocalKubernetes,
            Box::new(LocalKubernetesCleanup),
        );
        handlers.insert(
            DeploymentType::LocalHypervisor,
            Box::new(LocalHypervisorCleanup),
        );

        // Cloud deployment handlers
        handlers.insert(DeploymentType::AwsEc2, Box::new(AwsCleanup));
        handlers.insert(DeploymentType::GcpGce, Box::new(GcpCleanup));
        handlers.insert(DeploymentType::AzureVm, Box::new(AzureCleanup));
        handlers.insert(
            DeploymentType::DigitalOceanDroplet,
            Box::new(DigitalOceanCleanup),
        );
        handlers.insert(DeploymentType::VultrInstance, Box::new(VultrCleanup));

        // Kubernetes cluster handlers
        handlers.insert(DeploymentType::AwsEks, Box::new(EksCleanup));
        handlers.insert(DeploymentType::GcpGke, Box::new(GkeCleanup));
        handlers.insert(DeploymentType::AzureAks, Box::new(AksCleanup));

        // SSH/Bare metal handler
        handlers.insert(DeploymentType::SshRemote, Box::new(SshCleanup));
    }

    /// Send cleanup notification webhook
    async fn send_cleanup_notification(&self, webhook_url: &str, blueprint_id: &str, status: &str) {
        let client = reqwest::Client::new();
        let body = serde_json::json!({
            "blueprint_id": blueprint_id,
            "event": "cleanup",
            "status": status,
            "timestamp": Utc::now().to_rfc3339(),
        });

        if let Err(e) = client.post(webhook_url).json(&body).send().await {
            warn!("Failed to send cleanup notification: {}", e);
        }
    }

    /// Load state from disk
    async fn load_state(path: &Path) -> Result<HashMap<String, DeploymentRecord>> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to read state: {e}")))?;

        serde_json::from_str(&content)
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse state: {e}")))
    }

    /// Save state to disk
    async fn save_state(&self) -> Result<()> {
        let deployments = self.deployments.read().await;
        let json = serde_json::to_string_pretty(&*deployments)
            .map_err(|e| Error::ConfigurationError(format!("Failed to serialize state: {e}")))?;

        tokio::fs::write(&self.state_file, json)
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to write state: {e}")))?;

        Ok(())
    }

    /// Check all deployments for expired TTLs
    pub async fn check_all_ttls(&self) -> Result<()> {
        let deployments = self.deployments.read().await;
        let now = Utc::now();

        let expired: Vec<String> = deployments
            .iter()
            .filter_map(|(id, record)| {
                record
                    .expires_at
                    .filter(|expiry| now >= *expiry)
                    .map(|_| id.clone())
            })
            .collect();

        drop(deployments);

        for blueprint_id in expired {
            if let Err(e) = self.handle_ttl_expiry(&blueprint_id).await {
                error!("Failed to handle TTL expiry for {}: {}", blueprint_id, e);
            }
        }

        Ok(())
    }

    /// Get deployment status
    pub async fn get_deployment_status(&self, blueprint_id: &str) -> Option<DeploymentStatus> {
        let deployments = self.deployments.read().await;
        deployments.get(blueprint_id).map(|d| d.status.clone())
    }

    /// List all active deployments
    pub async fn list_deployments(&self) -> Vec<(String, DeploymentRecord)> {
        let deployments = self.deployments.read().await;
        deployments
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// List only active deployments
    pub async fn list_active(&self) -> Result<Vec<DeploymentRecord>> {
        let deployments = self.deployments.read().await;
        Ok(deployments
            .values()
            .filter(|d| d.status == DeploymentStatus::Active)
            .cloned()
            .collect())
    }

    /// List all deployment records (values only)
    pub async fn list_all(&self) -> Result<Vec<DeploymentRecord>> {
        let deployments = self.deployments.read().await;
        Ok(deployments.values().cloned().collect())
    }

    /// Get a deployment by instance ID (linear search)
    pub async fn get_by_instance_id(&self, instance_id: &str) -> Result<Option<DeploymentRecord>> {
        let deployments = self.deployments.read().await;
        Ok(deployments.values().find(|d| d.id == instance_id).cloned())
    }

    /// Remove a deployment by instance ID
    pub async fn remove_by_instance_id(&self, instance_id: &str) -> Result<()> {
        let mut deployments = self.deployments.write().await;
        // Find the key for this instance_id
        let key = deployments
            .iter()
            .find(|(_, d)| d.id == instance_id)
            .map(|(k, _)| k.clone());

        if let Some(k) = key {
            deployments.remove(&k);
            drop(deployments);
            self.save_state().await?;
        }
        Ok(())
    }

    /// Get a specific deployment
    pub async fn get(&self, deployment_id: &str) -> Result<Option<DeploymentRecord>> {
        let deployments = self.deployments.read().await;
        Ok(deployments.get(deployment_id).cloned())
    }

    /// Update instance ID for a deployment (used during recovery)
    pub async fn update_instance_id(&self, old_id: &str, new_id: &str) -> Result<()> {
        let mut deployments = self.deployments.write().await;
        if let Some(mut deployment) = deployments.remove(old_id) {
            deployment
                .resource_ids
                .insert("instance_id".to_string(), new_id.to_string());
            deployments.insert(new_id.to_string(), deployment);
            drop(deployments);
            self.save_state().await?;
        }
        Ok(())
    }

    #[cfg(test)]
    pub async fn set_cleanup_handler(
        &self,
        deployment_type: DeploymentType,
        handler: Box<dyn CleanupHandler>,
    ) {
        let mut handlers = self.cleanup_handlers.write().await;
        handlers.insert(deployment_type, handler);
    }
}

impl Clone for DeploymentTracker {
    fn clone(&self) -> Self {
        Self {
            deployments: self.deployments.clone(),
            state_file: self.state_file.clone(),
            cleanup_handlers: self.cleanup_handlers.clone(),
        }
    }
}

/// Periodic TTL checker task
pub async fn ttl_checker_task(tracker: DeploymentTracker) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));

    loop {
        interval.tick().await;

        if let Err(e) = tracker.check_all_ttls().await {
            error!("TTL check failed: {}", e);
        }
    }
}
