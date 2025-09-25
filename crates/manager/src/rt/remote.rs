//! Remote service runtime implementation for cloud deployments

use super::service::Status;
use crate::error::{Error, Result};
use blueprint_core::{error, info, warn};
use blueprint_remote_providers::deployment::manager_integration::RemoteDeploymentConfig;
use blueprint_remote_providers::deployment::tracker::DeploymentTracker;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A remote service instance running on a cloud provider
pub struct RemoteServiceInstance {
    config: RemoteDeploymentConfig,
    tracker: Arc<DeploymentTracker>,
    status: Arc<RwLock<Status>>,
}

impl RemoteServiceInstance {
    pub fn new(config: RemoteDeploymentConfig, tracker: Arc<DeploymentTracker>) -> Self {
        Self {
            config,
            tracker,
            status: Arc::new(RwLock::new(Status::NotStarted)),
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        let provider = self
            .config
            .provider
            .as_ref()
            .map(|p| format!("{:?}", p))
            .unwrap_or_else(|| "unknown".to_string());
        info!(
            "Starting remote service on {} (instance: {})",
            provider, self.config.instance_id
        );

        *self.status.write().await = Status::Pending;

        // The deployment was already created in try_remote_deployment
        // Just verify it's running
        match self.tracker.get_deployment(&self.config.instance_id).await {
            Ok(Some(deployment)) if deployment.is_running() => {
                *self.status.write().await = Status::Running;
                Ok(())
            }
            Ok(Some(_)) => {
                *self.status.write().await = Status::Error;
                Err(Error::Other("Remote deployment is not running".into()))
            }
            _ => {
                *self.status.write().await = Status::Unknown;
                Err(Error::Other("Remote deployment not found".into()))
            }
        }
    }

    pub async fn status(&self) -> Result<Status> {
        Ok(*self.status.read().await)
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        let provider = self
            .config
            .provider
            .as_ref()
            .map(|p| format!("{:?}", p))
            .unwrap_or_else(|| "unknown".to_string());
        info!(
            "Shutting down remote service on {} (instance: {})",
            provider, self.config.instance_id
        );

        // Mark for termination in tracker
        self.tracker
            .mark_for_termination(&self.config.instance_id)
            .await?;

        *self.status.write().await = Status::Finished;
        Ok(())
    }

    pub async fn logs(&self, _lines: usize) -> Result<Vec<String>> {
        let provider = self
            .config
            .provider
            .as_ref()
            .map(|p| format!("{:?}", p))
            .unwrap_or_else(|| "unknown".to_string());
        // This would fetch logs from the remote provider
        // For now, return a placeholder
        Ok(vec![format!(
            "[Remote logs from {} instance {} - not yet implemented]",
            provider, self.config.instance_id
        )])
    }
}
