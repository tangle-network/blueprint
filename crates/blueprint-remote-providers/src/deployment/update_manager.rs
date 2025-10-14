//! Blueprint update and rollback management
//!
//! Provides safe blueprint updates with automatic rollback on failure,
//! blue-green deployments, and version history tracking.

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::deployment::ssh::SshDeploymentClient;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use blueprint_core::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use blueprint_std::collections::{HashMap, VecDeque};
use blueprint_std::time::{Duration, SystemTime};
use tokio::time::{sleep, timeout};

/// Maximum number of deployment versions to keep
const MAX_VERSION_HISTORY: usize = 10;

/// Parameters for deployment updates
#[derive(Debug, Clone)]
pub struct UpdateParams {
    pub version: String,
    pub new_image: String,
    pub resource_spec: ResourceSpec,
    pub env_vars: HashMap<String, String>,
}

/// Parameters for rolling deployment updates
#[derive(Debug, Clone)]
pub struct RollingUpdateParams {
    pub base: UpdateParams,
    pub max_unavailable: u32,
    pub max_surge: u32,
}

/// Parameters for canary deployment updates
#[derive(Debug, Clone)]
pub struct CanaryUpdateParams {
    pub base: UpdateParams,
    pub initial_percentage: u8,
    pub increment: u8,
    pub interval: Duration,
}

/// Deployment update strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateStrategy {
    /// Replace existing deployment immediately
    RollingUpdate {
        max_unavailable: u32,
        max_surge: u32,
    },
    /// Deploy new version alongside old, switch traffic when ready
    BlueGreen {
        switch_timeout: Duration,
        health_check_duration: Duration,
    },
    /// Gradually shift traffic to new version
    Canary {
        initial_percentage: u8,
        increment: u8,
        interval: Duration,
    },
    /// Replace in-place without safety checks (fast but risky)
    Recreate,
}

impl Default for UpdateStrategy {
    fn default() -> Self {
        Self::BlueGreen {
            switch_timeout: Duration::from_secs(300),
            health_check_duration: Duration::from_secs(60),
        }
    }
}

/// Deployment version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentVersion {
    pub version: String,
    pub blueprint_image: String,
    pub resource_spec: ResourceSpec,
    pub env_vars: HashMap<String, String>,
    pub deployment_time: SystemTime,
    pub status: VersionStatus,
    pub metadata: HashMap<String, String>,
    pub container_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VersionStatus {
    Active,
    Inactive,
    Failed,
    RolledBack,
    Staging,
}

/// Manages blueprint updates and rollbacks
pub struct UpdateManager {
    versions: VecDeque<DeploymentVersion>,
    active_version: Option<String>,
    strategy: UpdateStrategy,
}

impl UpdateManager {
    pub fn new(strategy: UpdateStrategy) -> Self {
        Self {
            versions: VecDeque::new(),
            active_version: None,
            strategy,
        }
    }

    /// Add a new deployment version
    pub fn add_version(&mut self, version: DeploymentVersion) {
        info!("Adding deployment version: {}", version.version);

        // Keep only the latest versions
        if self.versions.len() >= MAX_VERSION_HISTORY {
            self.versions.pop_front();
        }

        self.versions.push_back(version);
    }

    /// Get the currently active version
    pub fn active_version(&self) -> Option<&DeploymentVersion> {
        self.active_version
            .as_ref()
            .and_then(|v| self.versions.iter().find(|ver| ver.version == *v))
    }

    /// Get a specific version
    pub fn get_version(&self, version: &str) -> Option<&DeploymentVersion> {
        self.versions.iter().find(|v| v.version == version)
    }

    /// List all versions
    pub fn list_versions(&self) -> Vec<&DeploymentVersion> {
        self.versions.iter().collect()
    }

    /// Update blueprint with new version
    pub async fn update_blueprint<A: CloudProviderAdapter>(
        &mut self,
        adapter: &A,
        new_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
        current_deployment: &BlueprintDeploymentResult,
    ) -> Result<BlueprintDeploymentResult> {
        let new_version = self.generate_version();
        info!("Starting blueprint update to version {}", new_version);

        match &self.strategy {
            UpdateStrategy::BlueGreen {
                switch_timeout,
                health_check_duration,
            } => {
                let params = UpdateParams {
                    version: new_version.clone(),
                    new_image: new_image.to_string(),
                    resource_spec: resource_spec.clone(),
                    env_vars,
                };
                self.blue_green_update(
                    adapter,
                    &params,
                    current_deployment,
                    *switch_timeout,
                    *health_check_duration,
                )
                .await
            }
            UpdateStrategy::RollingUpdate {
                max_unavailable,
                max_surge,
            } => {
                let params = RollingUpdateParams {
                    base: UpdateParams {
                        version: new_version.clone(),
                        new_image: new_image.to_string(),
                        resource_spec: resource_spec.clone(),
                        env_vars,
                    },
                    max_unavailable: *max_unavailable,
                    max_surge: *max_surge,
                };
                self.rolling_update(adapter, &params, current_deployment)
                    .await
            }
            UpdateStrategy::Canary {
                initial_percentage,
                increment,
                interval,
            } => {
                let params = CanaryUpdateParams {
                    base: UpdateParams {
                        version: new_version.clone(),
                        new_image: new_image.to_string(),
                        resource_spec: resource_spec.clone(),
                        env_vars,
                    },
                    initial_percentage: *initial_percentage,
                    increment: *increment,
                    interval: *interval,
                };
                self.canary_update(adapter, &params, current_deployment)
                    .await
            }
            UpdateStrategy::Recreate => {
                self.recreate_update(
                    adapter,
                    &new_version,
                    new_image,
                    resource_spec,
                    env_vars,
                    current_deployment,
                )
                .await
            }
        }
    }

    /// Blue-green deployment update
    async fn blue_green_update<A: CloudProviderAdapter>(
        &mut self,
        adapter: &A,
        params: &UpdateParams,
        current_deployment: &BlueprintDeploymentResult,
        _switch_timeout: Duration,
        health_check_duration: Duration,
    ) -> Result<BlueprintDeploymentResult> {
        info!(
            "Starting blue-green deployment for version {}",
            params.version
        );

        // Deploy new version (green)
        let mut green_env = params.env_vars.clone();
        green_env.insert("DEPLOYMENT_VERSION".to_string(), params.version.clone());
        green_env.insert("DEPLOYMENT_COLOR".to_string(), "green".to_string());

        let green_deployment = adapter
            .deploy_blueprint(
                &current_deployment.instance,
                &params.new_image,
                &params.resource_spec,
                green_env.clone(),
            )
            .await
            .map_err(|e| {
                error!("Failed to deploy green version: {}", e);
                e
            })?;

        // Add to version history
        self.add_version(DeploymentVersion {
            version: params.version.clone(),
            blueprint_image: params.new_image.clone(),
            resource_spec: params.resource_spec.clone(),
            env_vars: green_env,
            deployment_time: SystemTime::now(),
            status: VersionStatus::Staging,
            metadata: green_deployment.metadata.clone(),
            container_id: Some(green_deployment.blueprint_id.clone()),
        });

        // Health check green deployment
        info!("Performing health checks on green deployment");
        let health_check_result = timeout(
            health_check_duration,
            self.wait_for_healthy(&green_deployment, adapter),
        )
        .await;

        match health_check_result {
            Ok(Ok(true)) => {
                info!("Green deployment is healthy, switching traffic");

                // Switch traffic to green
                if let Err(e) = self
                    .switch_traffic(&green_deployment, current_deployment)
                    .await
                {
                    warn!("Failed to switch traffic: {}, rolling back", e);
                    adapter.cleanup_blueprint(&green_deployment).await?;
                    return Err(e);
                }

                // Mark green as active
                if let Some(v) = self
                    .versions
                    .iter_mut()
                    .find(|v| v.version == params.version)
                {
                    v.status = VersionStatus::Active;
                }

                // Mark old version as inactive
                if let Some(old_version) = &self.active_version {
                    if let Some(v) = self.versions.iter_mut().find(|v| v.version == *old_version) {
                        v.status = VersionStatus::Inactive;
                    }
                }

                self.active_version = Some(params.version.clone());

                // Cleanup old deployment after switch
                sleep(Duration::from_secs(30)).await;
                if let Err(e) = adapter.cleanup_blueprint(current_deployment).await {
                    warn!("Failed to cleanup old deployment: {}", e);
                }

                Ok(green_deployment)
            }
            _ => {
                error!("Green deployment health check failed, cleaning up");

                // Mark as failed
                if let Some(v) = self
                    .versions
                    .iter_mut()
                    .find(|v| v.version == params.version)
                {
                    v.status = VersionStatus::Failed;
                }

                // Cleanup failed green deployment
                adapter.cleanup_blueprint(&green_deployment).await?;

                Err(Error::Other("Green deployment health check failed".into()))
            }
        }
    }

    /// Rolling update deployment
    async fn rolling_update<A: CloudProviderAdapter>(
        &mut self,
        adapter: &A,
        params: &RollingUpdateParams,
        current_deployment: &BlueprintDeploymentResult,
    ) -> Result<BlueprintDeploymentResult> {
        info!("Starting rolling update to version {}", params.base.version);

        // For single instance, this is similar to recreate with health checks
        let mut new_env = params.base.env_vars.clone();
        new_env.insert(
            "DEPLOYMENT_VERSION".to_string(),
            params.base.version.clone(),
        );

        // Deploy new version
        let new_deployment = adapter
            .deploy_blueprint(
                &current_deployment.instance,
                &params.base.new_image,
                &params.base.resource_spec,
                new_env.clone(),
            )
            .await?;

        // Wait for new deployment to be healthy
        if !self.wait_for_healthy(&new_deployment, adapter).await? {
            // Rollback if health check fails
            adapter.cleanup_blueprint(&new_deployment).await?;
            return Err(Error::Other("New deployment failed health check".into()));
        }

        // Cleanup old deployment
        adapter.cleanup_blueprint(current_deployment).await?;

        // Update version tracking
        self.add_version(DeploymentVersion {
            version: params.base.version.clone(),
            blueprint_image: params.base.new_image.clone(),
            resource_spec: params.base.resource_spec.clone(),
            env_vars: new_env,
            deployment_time: SystemTime::now(),
            status: VersionStatus::Active,
            metadata: new_deployment.metadata.clone(),
            container_id: Some(new_deployment.blueprint_id.clone()),
        });

        self.active_version = Some(params.base.version.clone());

        Ok(new_deployment)
    }

    /// Canary deployment update
    async fn canary_update<A: CloudProviderAdapter>(
        &mut self,
        adapter: &A,
        params: &CanaryUpdateParams,
        current_deployment: &BlueprintDeploymentResult,
    ) -> Result<BlueprintDeploymentResult> {
        info!(
            "Starting canary deployment for version {}",
            params.base.version
        );

        // Deploy canary version
        let mut canary_env = params.base.env_vars.clone();
        canary_env.insert(
            "DEPLOYMENT_VERSION".to_string(),
            params.base.version.clone(),
        );
        canary_env.insert("DEPLOYMENT_TYPE".to_string(), "canary".to_string());

        let canary_deployment = adapter
            .deploy_blueprint(
                &current_deployment.instance,
                &params.base.new_image,
                &params.base.resource_spec,
                canary_env.clone(),
            )
            .await?;

        // Gradually increase traffic percentage
        let mut current_percentage = params.initial_percentage;

        while current_percentage < 100 {
            info!("Canary at {}% traffic", current_percentage);

            // Monitor canary health
            if !adapter.health_check_blueprint(&canary_deployment).await? {
                warn!(
                    "Canary health check failed at {}%, rolling back",
                    current_percentage
                );
                adapter.cleanup_blueprint(&canary_deployment).await?;
                return Err(Error::Other(format!(
                    "Canary failed at {current_percentage}%"
                )));
            }

            // Wait before increasing traffic
            sleep(params.interval).await;

            current_percentage = (current_percentage + params.increment).min(100);
        }

        // Full rollout successful
        info!("Canary deployment successful, completing rollout");

        // Cleanup old deployment
        adapter.cleanup_blueprint(current_deployment).await?;

        // Update version tracking
        self.add_version(DeploymentVersion {
            version: params.base.version.clone(),
            blueprint_image: params.base.new_image.clone(),
            resource_spec: params.base.resource_spec.clone(),
            env_vars: canary_env,
            deployment_time: SystemTime::now(),
            status: VersionStatus::Active,
            metadata: canary_deployment.metadata.clone(),
            container_id: Some(canary_deployment.blueprint_id.clone()),
        });

        self.active_version = Some(params.base.version.clone());

        Ok(canary_deployment)
    }

    /// Recreate deployment (fast but with downtime)
    async fn recreate_update<A: CloudProviderAdapter>(
        &mut self,
        adapter: &A,
        version: &str,
        new_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
        current_deployment: &BlueprintDeploymentResult,
    ) -> Result<BlueprintDeploymentResult> {
        info!("Starting recreate deployment for version {}", version);

        // Cleanup old deployment first (causes downtime)
        adapter.cleanup_blueprint(current_deployment).await?;

        // Deploy new version
        let mut new_env = env_vars.clone();
        new_env.insert("DEPLOYMENT_VERSION".to_string(), version.to_string());

        let new_deployment = adapter
            .deploy_blueprint(
                &current_deployment.instance,
                new_image,
                resource_spec,
                new_env.clone(),
            )
            .await?;

        // Update version tracking
        self.add_version(DeploymentVersion {
            version: version.to_string(),
            blueprint_image: new_image.to_string(),
            resource_spec: resource_spec.clone(),
            env_vars: new_env,
            deployment_time: SystemTime::now(),
            status: VersionStatus::Active,
            metadata: new_deployment.metadata.clone(),
            container_id: Some(new_deployment.blueprint_id.clone()),
        });

        self.active_version = Some(version.to_string());

        Ok(new_deployment)
    }

    /// Rollback to a previous version
    pub async fn rollback<A: CloudProviderAdapter>(
        &mut self,
        adapter: &A,
        target_version: &str,
        current_deployment: &BlueprintDeploymentResult,
    ) -> Result<BlueprintDeploymentResult> {
        info!("Rolling back to version {}", target_version);

        let version = self
            .get_version(target_version)
            .ok_or_else(|| Error::Other(format!("Version {target_version} not found")))?
            .clone();

        if version.status == VersionStatus::Failed {
            return Err(Error::Other("Cannot rollback to a failed version".into()));
        }

        // Deploy the target version
        let rollback_deployment = adapter
            .deploy_blueprint(
                &current_deployment.instance,
                &version.blueprint_image,
                &version.resource_spec,
                version.env_vars.clone(),
            )
            .await?;

        // Wait for rollback to be healthy
        if !self.wait_for_healthy(&rollback_deployment, adapter).await? {
            error!("Rollback deployment failed health check");
            adapter.cleanup_blueprint(&rollback_deployment).await?;
            return Err(Error::Other("Rollback failed health check".into()));
        }

        // Cleanup current deployment
        adapter.cleanup_blueprint(current_deployment).await?;

        // Update version status
        if let Some(current) = &self.active_version {
            if let Some(v) = self.versions.iter_mut().find(|v| v.version == *current) {
                v.status = VersionStatus::RolledBack;
            }
        }

        // Mark rollback version as active
        if let Some(v) = self
            .versions
            .iter_mut()
            .find(|v| v.version == target_version)
        {
            v.status = VersionStatus::Active;
        }

        self.active_version = Some(target_version.to_string());

        Ok(rollback_deployment)
    }

    /// Wait for deployment to become healthy
    async fn wait_for_healthy<A: CloudProviderAdapter>(
        &self,
        deployment: &BlueprintDeploymentResult,
        adapter: &A,
    ) -> Result<bool> {
        let max_attempts = 30;
        let check_interval = Duration::from_secs(10);

        for attempt in 1..=max_attempts {
            debug!("Health check attempt {}/{}", attempt, max_attempts);

            match adapter.health_check_blueprint(deployment).await {
                Ok(true) => {
                    info!("Deployment is healthy");
                    return Ok(true);
                }
                Ok(false) => {
                    if attempt < max_attempts {
                        sleep(check_interval).await;
                    }
                }
                Err(e) => {
                    warn!("Health check error: {}", e);
                    if attempt < max_attempts {
                        sleep(check_interval).await;
                    }
                }
            }
        }

        Ok(false)
    }

    /// Switch traffic from old to new deployment
    async fn switch_traffic(
        &self,
        new_deployment: &BlueprintDeploymentResult,
        old_deployment: &BlueprintDeploymentResult,
    ) -> Result<()> {
        // In a real implementation, this would update load balancer rules,
        // DNS records, or service mesh configuration
        info!(
            "Switching traffic from {} to {}",
            old_deployment.blueprint_id, new_deployment.blueprint_id
        );

        // Simulate traffic switch
        sleep(Duration::from_secs(5)).await;

        Ok(())
    }

    /// Generate a new version identifier
    fn generate_version(&self) -> String {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        format!("v{timestamp}")
    }

    /// Get deployment history
    pub fn get_history(&self, limit: usize) -> Vec<DeploymentVersion> {
        self.versions.iter().rev().take(limit).cloned().collect()
    }

    /// Clean up old inactive versions
    pub async fn cleanup_old_versions<A: CloudProviderAdapter>(
        &mut self,
        adapter: &A,
        keep_count: usize,
    ) -> Result<()> {
        let inactive_versions: Vec<_> = self
            .versions
            .iter()
            .filter(|v| v.status == VersionStatus::Inactive)
            .skip(keep_count)
            .cloned()
            .collect();

        for version in inactive_versions {
            info!("Cleaning up old version: {}", version.version);

            // Create a dummy deployment result for cleanup
            if let Some(container_id) = version.container_id {
                let deployment = BlueprintDeploymentResult {
                    instance: crate::infra::types::ProvisionedInstance {
                        id: format!("update-cleanup-{}", uuid::Uuid::new_v4()),
                        public_ip: None,
                        private_ip: None,
                        status: crate::infra::types::InstanceStatus::Unknown,
                        provider: crate::core::remote::CloudProvider::Generic,
                        region: "unknown".to_string(),
                        instance_type: "unknown".to_string(),
                    },
                    blueprint_id: container_id,
                    port_mappings: HashMap::new(),
                    metadata: version.metadata.clone(),
                };

                if let Err(e) = adapter.cleanup_blueprint(&deployment).await {
                    warn!("Failed to cleanup version {}: {}", version.version, e);
                }
            }

            // Remove from history
            self.versions.retain(|v| v.version != version.version);
        }

        Ok(())
    }
}

/// SSH-specific update operations
impl UpdateManager {
    /// Update blueprint via SSH
    pub async fn update_via_ssh(
        &mut self,
        ssh_client: &SshDeploymentClient,
        new_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<String> {
        let version = self.generate_version();
        info!("Starting SSH update to version {}", version);

        match &self.strategy {
            UpdateStrategy::BlueGreen { .. } => {
                // Deploy new container alongside old one with resource limits
                let new_container_name = format!("blueprint-{version}");
                let new_container_id = ssh_client
                    .deploy_container_with_resources(
                        new_image,
                        &new_container_name,
                        env_vars.clone(),
                        Some(resource_spec),
                    )
                    .await?;

                // Health check new container
                if ssh_client.health_check_container(&new_container_id).await? {
                    // Switch traffic (update nginx/haproxy config)
                    ssh_client.switch_traffic_to(&new_container_name).await?;

                    // Stop old container
                    if let Some(old_version) = &self.active_version {
                        let old_container_name = format!("blueprint-{old_version}");
                        ssh_client.stop_container(&old_container_name).await?;
                    }

                    self.active_version = Some(version.clone());
                    Ok(new_container_id)
                } else {
                    // Rollback
                    ssh_client.remove_container(&new_container_id).await?;
                    Err(Error::Other("New container health check failed".into()))
                }
            }
            _ => {
                // Simple replace for other strategies with resource limits
                let new_container_id = ssh_client
                    .update_container_with_resources(new_image, env_vars, Some(resource_spec))
                    .await?;

                self.active_version = Some(version.clone());
                Ok(new_container_id)
            }
        }
    }

    /// Rollback via SSH
    pub async fn rollback_via_ssh(
        &mut self,
        ssh_client: &SshDeploymentClient,
        target_version: &str,
    ) -> Result<()> {
        let version = self
            .get_version(target_version)
            .ok_or_else(|| Error::Other(format!("Version {target_version} not found")))?
            .clone();

        // Redeploy the target version
        ssh_client
            .deploy_container(&version.blueprint_image, version.env_vars)
            .await?;

        self.active_version = Some(target_version.to_string());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_management() {
        let mut manager = UpdateManager::new(UpdateStrategy::default());

        let version1 = DeploymentVersion {
            version: "v1".to_string(),
            blueprint_image: "image:v1".to_string(),
            resource_spec: ResourceSpec::basic(),
            env_vars: HashMap::new(),
            deployment_time: SystemTime::now(),
            status: VersionStatus::Active,
            metadata: HashMap::new(),
            container_id: Some("container1".to_string()),
        };

        manager.add_version(version1.clone());
        manager.active_version = Some("v1".to_string());

        assert_eq!(manager.active_version().unwrap().version, "v1");
        assert_eq!(manager.list_versions().len(), 1);
    }

    #[test]
    fn test_version_history_limit() {
        let mut manager = UpdateManager::new(UpdateStrategy::default());

        // Add more than MAX_VERSION_HISTORY versions
        for i in 0..15 {
            let version = DeploymentVersion {
                version: format!("v{i}"),
                blueprint_image: format!("image:v{i}"),
                resource_spec: ResourceSpec::basic(),
                env_vars: HashMap::new(),
                deployment_time: SystemTime::now(),
                status: VersionStatus::Inactive,
                metadata: HashMap::new(),
                container_id: Some(format!("container{i}")),
            };
            manager.add_version(version);
        }

        // Should keep only MAX_VERSION_HISTORY versions
        assert!(manager.list_versions().len() <= MAX_VERSION_HISTORY);
    }

    #[tokio::test]
    async fn test_generate_version() {
        let manager = UpdateManager::new(UpdateStrategy::default());
        let version1 = manager.generate_version();
        sleep(Duration::from_secs(1)).await;
        let version2 = manager.generate_version();

        assert_ne!(version1, version2);
        assert!(version1.starts_with("v"));
        assert!(version2.starts_with("v"));
    }
}
