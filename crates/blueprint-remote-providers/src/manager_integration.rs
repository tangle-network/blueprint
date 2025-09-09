//! Integration hooks for remote deployments with Blueprint Manager
//!
//! This module provides extension points for the existing Blueprint Manager
//! to handle remote cloud deployments without modifying core code.

use crate::deployment_tracker::{DeploymentRecord, DeploymentTracker, DeploymentType};
use crate::error::{Error, Result};
use crate::remote::CloudProvider;
use crate::resources::ResourceSpec;
use chrono::{DateTime, Utc};
use blueprint_std::collections::HashMap;
use blueprint_std::path::PathBuf;
use blueprint_std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

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
        if let Some(config) = deployments.remove(&(blueprint_id, service_id)) {
            info!(
                "Cleaning up remote deployment {} for blueprint {} service {}",
                config.instance_id, blueprint_id, service_id
            );
            self.tracker.handle_termination(&config.instance_id).await?;
        }
        Ok(())
    }
}

/// TTL Manager that runs alongside the Blueprint Manager's event loop
pub struct TtlManager {
    /// Registry for remote deployments
    registry: Arc<RemoteDeploymentRegistry>,
    /// Mapping of (blueprint_id, service_id) to TTL expiry time
    ttl_registry: Arc<RwLock<HashMap<(u64, u64), DateTime<Utc>>>>,
    /// Channel to notify main event loop of TTL expirations
    expiry_tx: tokio::sync::mpsc::UnboundedSender<(u64, u64)>,
}

impl TtlManager {
    /// Create a new TTL manager
    pub fn new(
        registry: Arc<RemoteDeploymentRegistry>,
        expiry_tx: tokio::sync::mpsc::UnboundedSender<(u64, u64)>,
    ) -> Self {
        Self {
            registry,
            ttl_registry: Arc::new(RwLock::new(HashMap::new())),
            expiry_tx,
        }
    }

    /// Register a service with TTL
    pub async fn register_ttl(&self, blueprint_id: u64, service_id: u64, ttl_seconds: u64) {
        let expiry = Utc::now() + chrono::Duration::seconds(ttl_seconds as i64);
        let mut registry = self.ttl_registry.write().await;
        registry.insert((blueprint_id, service_id), expiry);
        info!(
            "Registered TTL for blueprint {} service {}: expires at {}",
            blueprint_id, service_id, expiry
        );
    }

    /// Check for expired services
    pub async fn check_expired_services(&self) -> Result<Vec<(u64, u64)>> {
        let now = Utc::now();
        let registry = self.ttl_registry.read().await;

        let expired: Vec<(u64, u64)> = registry
            .iter()
            .filter(|(_, expiry)| now >= **expiry)
            .map(|(id, _)| *id)
            .collect();

        drop(registry);

        let mut cleaned = Vec::new();

        for (blueprint_id, service_id) in expired {
            info!(
                "TTL expired for blueprint {} service {}",
                blueprint_id, service_id
            );

            // Send expiry notification to main event loop
            if self.expiry_tx.send((blueprint_id, service_id)).is_ok() {
                cleaned.push((blueprint_id, service_id));

                // Remove from TTL registry
                let mut registry = self.ttl_registry.write().await;
                registry.remove(&(blueprint_id, service_id));
            }
        }

        Ok(cleaned)
    }
}

/// Hook for service shutdown with remote cleanup
/// Call this when a service is being terminated
pub async fn handle_service_shutdown(
    blueprint_id: u64,
    service_id: u64,
    registry: &RemoteDeploymentRegistry,
) -> Result<()> {
    if let Some(config) = registry.get(blueprint_id, service_id).await {
        info!(
            "Performing remote cleanup for deployment {}",
            config.instance_id
        );
        registry.cleanup(blueprint_id, service_id).await?;
    }
    Ok(())
}

/// Event handler extension for remote deployments
/// Call this from the Blueprint Manager's event handler
pub struct RemoteEventHandler {
    registry: Arc<RemoteDeploymentRegistry>,
    ttl_manager: Option<Arc<TtlManager>>,
}

impl RemoteEventHandler {
    pub fn new(registry: Arc<RemoteDeploymentRegistry>) -> Self {
        Self {
            registry,
            ttl_manager: None,
        }
    }

    /// Enable TTL management
    pub fn with_ttl_manager(mut self, ttl_manager: Arc<TtlManager>) -> Self {
        self.ttl_manager = Some(ttl_manager);
        self
    }

    /// Handle service initialization events
    pub async fn on_service_initiated(
        &self,
        blueprint_id: u64,
        service_id: u64,
        config: Option<RemoteDeploymentConfig>,
    ) -> Result<()> {
        if let Some(config) = config {
            // Register the remote deployment
            self.registry
                .register(blueprint_id, service_id, config.clone())
                .await;

            // Register TTL if specified
            if let Some(ttl_seconds) = config.ttl_seconds {
                if let Some(ttl_manager) = &self.ttl_manager {
                    ttl_manager
                        .register_ttl(blueprint_id, service_id, ttl_seconds)
                        .await;
                }
            }
        }
        Ok(())
    }

    /// Handle service termination events  
    pub async fn on_service_terminated(&self, blueprint_id: u64, service_id: u64) -> Result<()> {
        handle_service_shutdown(blueprint_id, service_id, &self.registry).await
    }

    /// Handle TTL expiry notifications
    pub async fn on_ttl_expired(&self, blueprint_id: u64, service_id: u64) -> Result<()> {
        info!(
            "Handling TTL expiry for blueprint {} service {}",
            blueprint_id, service_id
        );
        self.on_service_terminated(blueprint_id, service_id).await
    }
}

/// TTL checking task that runs alongside the Blueprint Manager
pub async fn ttl_checking_task(ttl_manager: Arc<TtlManager>, check_interval: blueprint_std::time::Duration) {
    let mut interval = tokio::time::interval(check_interval);

    loop {
        interval.tick().await;

        match ttl_manager.check_expired_services().await {
            Ok(expired) if !expired.is_empty() => {
                info!("Found {} services with expired TTL", expired.len());
            }
            Err(e) => {
                error!("TTL check failed: {}", e);
            }
            _ => {}
        }
    }
}

/// Extension for Blueprint sources to support remote deployments
pub struct RemoteSourceExtension {
    registry: Arc<RemoteDeploymentRegistry>,
    provisioner: Arc<crate::infrastructure::InfrastructureProvisioner>,
}

impl RemoteSourceExtension {
    pub fn new(
        registry: Arc<RemoteDeploymentRegistry>,
        provisioner: Arc<crate::infrastructure::InfrastructureProvisioner>,
    ) -> Self {
        Self {
            registry,
            provisioner,
        }
    }

    /// Spawn a remote deployment for a service
    pub async fn spawn_remote(
        &self,
        blueprint_id: u64,
        service_id: u64,
        resource_spec: ResourceSpec,
        provider: CloudProvider,
        region: String,
        ttl_seconds: Option<u64>,
    ) -> Result<RemoteDeploymentConfig> {
        // Create provisioning config
        let config = crate::infrastructure::ProvisioningConfig {
            name: format!("{}-{}", blueprint_id, service_id),
            region: region.clone(),
            ..Default::default()
        };
        
        // Provision the infrastructure
        let instance = self.provisioner.provision(&resource_spec, &config).await?;

        let config = RemoteDeploymentConfig {
            deployment_type: deployment_type_from_provider(&provider),
            provider: Some(provider),
            region: Some(region),
            instance_id: instance.instance_id,
            resource_spec,
            ttl_seconds,
            deployed_at: Utc::now(),
        };

        // Register the deployment
        self.registry
            .register(blueprint_id, service_id, config.clone())
            .await;

        Ok(config)
    }
}

fn deployment_type_from_provider(provider: &CloudProvider) -> DeploymentType {
    match provider {
        CloudProvider::AWS => DeploymentType::AwsEc2,
        CloudProvider::GCP => DeploymentType::GcpGce,
        CloudProvider::Azure => DeploymentType::AzureVm,
        CloudProvider::DigitalOcean => DeploymentType::DigitalOceanDroplet,
        CloudProvider::Vultr => DeploymentType::VultrInstance,
        _ => DeploymentType::SshRemote,
    }
}

/// Initialize remote deployment extensions for Blueprint Manager
pub struct RemoteDeploymentExtensions {
    pub registry: Arc<RemoteDeploymentRegistry>,
    pub event_handler: Arc<RemoteEventHandler>,
    pub ttl_manager: Option<Arc<TtlManager>>,
    pub source_extension: Arc<RemoteSourceExtension>,
}

impl RemoteDeploymentExtensions {
    /// Initialize all remote deployment extensions
    pub async fn initialize(
        state_dir: &blueprint_std::path::Path,
        enable_ttl: bool,
        provisioner: Arc<crate::infrastructure::InfrastructureProvisioner>,
    ) -> Result<Self> {
        // Initialize deployment tracker
        let tracker = Arc::new(DeploymentTracker::new(state_dir).await?);

        // Initialize registry
        let registry = Arc::new(RemoteDeploymentRegistry::new(tracker.clone()));

        // Initialize TTL management if enabled
        let ttl_manager = if enable_ttl {
            let (ttl_tx, mut ttl_rx) = tokio::sync::mpsc::unbounded_channel();
            let ttl_manager = Arc::new(TtlManager::new(registry.clone(), ttl_tx));

            // Start TTL checking task
            let ttl_manager_clone = ttl_manager.clone();
            tokio::spawn(async move {
                ttl_checking_task(ttl_manager_clone, blueprint_std::time::Duration::from_secs(60)).await;
            });

            // Start TTL expiry handler task
            let registry_clone = registry.clone();
            tokio::spawn(async move {
                while let Some((blueprint_id, service_id)) = ttl_rx.recv().await {
                    if let Err(e) =
                        handle_service_shutdown(blueprint_id, service_id, &registry_clone).await
                    {
                        error!("Failed to handle TTL expiry: {}", e);
                    }
                }
            });

            Some(ttl_manager)
        } else {
            None
        };

        // Initialize event handler
        let mut event_handler = RemoteEventHandler::new(registry.clone());
        if let Some(ttl_mgr) = &ttl_manager {
            event_handler = event_handler.with_ttl_manager(ttl_mgr.clone());
        }

        // Initialize source extension
        let source_extension = Arc::new(RemoteSourceExtension::new(registry.clone(), provisioner));

        info!("Initialized remote deployment extensions");

        Ok(Self {
            registry,
            event_handler: Arc::new(event_handler),
            ttl_manager,
            source_extension,
        })
    }

    /// Hook to call when a service is being removed
    pub async fn on_service_removed(&self, blueprint_id: u64, service_id: u64) -> Result<()> {
        self.event_handler
            .on_service_terminated(blueprint_id, service_id)
            .await
    }
}

/// Example integration with Blueprint Manager's event handler
/// This shows how to use the remote deployment extensions
///
/// ```rust,ignore
/// // In your Blueprint Manager initialization:
/// let remote_extensions = RemoteDeploymentExtensions::initialize(
///     &state_dir,
///     true, // enable TTL
///     provisioner,
/// ).await?;
///
/// // In your event handler when processing ServiceInitiated events:
/// if let Some(remote_config) = determine_if_remote(&service) {
///     remote_extensions.event_handler.on_service_initiated(
///         blueprint_id,
///         service_id,
///         Some(remote_config),
///     ).await?;
/// }
///
/// // When removing services in handle_tangle_event:
/// remote_extensions.on_service_removed(blueprint_id, service_id).await?;
/// ```
pub struct IntegrationExample;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_remote_registry() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());
        let registry = RemoteDeploymentRegistry::new(tracker);

        let config = RemoteDeploymentConfig {
            deployment_type: DeploymentType::AwsEc2,
            provider: Some(CloudProvider::AWS),
            region: Some("us-east-1".to_string()),
            instance_id: "i-1234567890".to_string(),
            resource_spec: crate::resources::ResourceSpec::basic(),
            ttl_seconds: Some(3600),
            deployed_at: Utc::now(),
        };

        // Register a deployment
        registry.register(100, 1, config.clone()).await;

        // Retrieve it
        let retrieved = registry.get(100, 1).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().instance_id, "i-1234567890");

        // Cleanup
        registry.cleanup(100, 1).await.unwrap();
        assert!(registry.get(100, 1).await.is_none());
    }

    #[tokio::test]
    async fn test_ttl_manager() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());
        let registry = Arc::new(RemoteDeploymentRegistry::new(tracker));
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        let ttl_manager = TtlManager::new(registry, tx);

        // Register a service with TTL
        ttl_manager.register_ttl(100, 1, 3600).await;

        let ttl_registry = ttl_manager.ttl_registry.read().await;
        assert!(ttl_registry.contains_key(&(100, 1)));
        drop(ttl_registry);

        // No expiry notifications yet
        assert!(rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn test_event_handler() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());
        let registry = Arc::new(RemoteDeploymentRegistry::new(tracker));

        let event_handler = RemoteEventHandler::new(registry.clone());

        let config = RemoteDeploymentConfig {
            deployment_type: DeploymentType::GcpGce,
            provider: Some(CloudProvider::GCP),
            region: Some("us-central1".to_string()),
            instance_id: "instance-123".to_string(),
            resource_spec: crate::resources::ResourceSpec::basic(),
            ttl_seconds: None,
            deployed_at: Utc::now(),
        };

        // Handle service initiated
        event_handler
            .on_service_initiated(200, 2, Some(config))
            .await
            .unwrap();

        // Verify it was registered
        assert!(registry.get(200, 2).await.is_some());

        // Handle termination
        event_handler.on_service_terminated(200, 2).await.unwrap();

        // Verify it was cleaned up
        assert!(registry.get(200, 2).await.is_none());
    }
}
