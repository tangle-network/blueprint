//! Integration hooks for remote deployments with Blueprint Manager

use crate::core::error::Result;
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::deployment::tracker::{DeploymentTracker, DeploymentType};
use blueprint_core::info;
use blueprint_std::{collections::HashMap, sync::Arc};
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

/// Remote deployment configuration that extends a service
#[derive(Debug, Clone)]
pub struct RemoteDeploymentConfig {
    pub deployment_type: DeploymentType,
    pub provider: Option<CloudProvider>,
    pub region: Option<String>,
    pub instance_id: String,
    pub resource_spec: ResourceSpec,
    pub ttl_seconds: Option<u64>,
    pub deployed_at: DateTime<Utc>,
}

/// Registry for tracking remote deployments associated with services
pub struct RemoteDeploymentRegistry {
    /// Map of (blueprint_id, service_id) -> deployment config
    deployments: Arc<RwLock<HashMap<(u64, u64), RemoteDeploymentConfig>>>,
    /// The deployment tracker for lifecycle management
    tracker: Arc<DeploymentTracker>,
}

impl RemoteDeploymentRegistry {
    pub fn new(tracker: Arc<DeploymentTracker>) -> Self {
        Self {
            deployments: Arc::new(RwLock::new(HashMap::new())),
            tracker,
        }
    }

    /// Register a remote deployment for a service
    pub async fn register(
        &self,
        blueprint_id: u64,
        service_id: u64,
        config: RemoteDeploymentConfig,
    ) {
        let mut deployments = self.deployments.write().await;
        deployments.insert((blueprint_id, service_id), config);
        info!(
            "Registered remote deployment for blueprint {} service {}",
            blueprint_id, service_id
        );
    }

    /// Get deployment config for a service
    pub async fn get(&self, blueprint_id: u64, service_id: u64) -> Option<RemoteDeploymentConfig> {
        let deployments = self.deployments.read().await;
        deployments.get(&(blueprint_id, service_id)).cloned()
    }

    /// Remove and cleanup a deployment
    pub async fn cleanup(&self, blueprint_id: u64, service_id: u64) -> Result<()> {
        let mut deployments = self.deployments.write().await;
        if let Some(config) = deployments.get(&(blueprint_id, service_id)) {
            info!(
                "Cleaning up remote deployment {} for blueprint {} service {}",
                config.instance_id, blueprint_id, service_id
            );
            self.tracker.handle_termination(&config.instance_id).await?;
            deployments.remove(&(blueprint_id, service_id));
        }
        Ok(())
    }
}
