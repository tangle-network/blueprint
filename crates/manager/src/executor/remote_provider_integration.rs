//! Remote provider integration for Blueprint Manager
//!
//! Handles automatic cloud deployment when services are initiated

use crate::config::BlueprintManagerContext;
use crate::error::Result;
use blueprint_core::{error, info};
use blueprint_remote_providers::deployment::manager_integration::{
    RemoteDeploymentRegistry, RemoteEventHandler, TtlManager,
};
use blueprint_remote_providers::{
    AwsConfig, AzureConfig, CloudConfig, CloudProvider, DigitalOceanConfig, GcpConfig,
    ResourceSpec, VultrConfig,
};
use blueprint_remote_providers::{CloudProvisioner, DeploymentTracker};
use blueprint_std::collections::HashMap;
use blueprint_std::sync::Arc;
use tokio::sync::RwLock;

/// Remote provider manager that handles cloud deployments
pub struct RemoteProviderManager {
    provisioner: Arc<CloudProvisioner>,
    registry: Arc<RemoteDeploymentRegistry>,
    ttl_manager: Arc<TtlManager>,
    enabled: bool,
}

impl RemoteProviderManager {
    /// Initialize from Blueprint Manager config
    pub async fn new(ctx: &BlueprintManagerContext) -> Result<Option<Self>> {
        // Check if remote providers are configured
        let cloud_config = ctx.cloud_config();
        if cloud_config.is_none() || !cloud_config.as_ref().unwrap().enabled {
            info!("Remote cloud providers not configured");
            return Ok(None);
        }

        let config = cloud_config.unwrap();

        // Create deployment tracker
        let tracker_path = ctx.data_dir().join("remote_deployments");
        let tracker = Arc::new(DeploymentTracker::new(&tracker_path).await?);

        // Create registry and provisioner
        let registry = Arc::new(RemoteDeploymentRegistry::new(tracker.clone()));
        let provisioner = Arc::new(CloudProvisioner::new().await?);

        // Create TTL manager for automatic cleanup
        let (expiry_tx, _expiry_rx) = tokio::sync::mpsc::unbounded_channel();
        let ttl_manager = Arc::new(TtlManager::new(registry.clone(), expiry_tx));

        Ok(Some(Self {
            provisioner,
            registry,
            ttl_manager,
            enabled: true,
        }))
    }

    /// Handle service initiated event
    pub async fn on_service_initiated(
        &self,
        blueprint_id: u64,
        service_id: u64,
        resource_requirements: Option<ResourceSpec>,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        info!(
            "Remote provider handling service initiation: blueprint={}, service={}",
            blueprint_id, service_id
        );

        // Use provided resources or default
        let resource_spec = resource_requirements.unwrap_or_else(ResourceSpec::minimal);

        // Use intelligent provider selection based on resource requirements
        let provider = if resource_spec.gpu_count.is_some() {
            // GPU workloads prefer GCP or AWS
            CloudProvider::GCP
        } else if resource_spec.cpu > 8.0 {
            // High CPU workloads prefer cost-optimized providers
            CloudProvider::Vultr
        } else if resource_spec.memory_gb > 32.0 {
            // High memory workloads prefer AWS or GCP
            CloudProvider::AWS
        } else {
            // Standard workloads use cost-optimized providers
            CloudProvider::DigitalOcean
        };

        // Get appropriate region for the provider
        let region = match provider {
            CloudProvider::AWS => "us-east-1",
            CloudProvider::GCP => "us-central1",
            CloudProvider::Azure => "eastus",
            CloudProvider::DigitalOcean => "nyc3",
            CloudProvider::Vultr => "ewr",
            _ => "default",
        };

        match self
            .provisioner
            .provision(provider, &resource_spec, region)
            .await
        {
            Ok(instance) => {
                info!(
                    "Service deployed to {}: instance={}",
                    provider, instance.instance_id
                );

                // Register with TTL manager for automatic cleanup
                self.ttl_manager
                    .register_ttl(blueprint_id, service_id, 3600)
                    .await; // 1 hour default
            }
            Err(e) => {
                error!("Failed to deploy service: {}", e);
            }
        }

        Ok(())
    }

    /// Handle service terminated event
    pub async fn on_service_terminated(&self, blueprint_id: u64, service_id: u64) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        info!(
            "Remote provider handling service termination: blueprint={}, service={}",
            blueprint_id, service_id
        );

        // Remove TTL registration for the terminated service
        self.ttl_manager
            .unregister_ttl(blueprint_id, service_id)
            .await;

        // Clean up deployment from registry
        if let Err(e) = self.registry.cleanup(blueprint_id, service_id).await {
            error!("Failed to cleanup deployment from registry: {}", e);
        }

        Ok(())
    }
}

// Cloud configuration types are now imported from blueprint_remote_providers
