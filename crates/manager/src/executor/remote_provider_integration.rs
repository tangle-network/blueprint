//! Remote provider integration for Blueprint Manager
//!
//! Handles automatic cloud deployment when services are initiated

use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use blueprint_core::info;
use blueprint_remote_providers::deployment::DeploymentType;
use blueprint_remote_providers::deployment::manager_integration::{
    RemoteDeploymentConfig, RemoteDeploymentRegistry, TtlManager,
};
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, DeploymentTracker, ResourceSpec,
};
use blueprint_std::collections::HashMap;
use blueprint_std::sync::Arc;
use chrono::Utc;

fn env_bool(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes"
            )
        })
        .unwrap_or(false)
}

fn supports_tee(provider: &CloudProvider) -> bool {
    matches!(
        provider,
        CloudProvider::AWS | CloudProvider::GCP | CloudProvider::Azure
    )
}

/// Remote provider manager that handles cloud deployments
pub struct RemoteProviderManager {
    provisioner: Arc<CloudProvisioner>,
    registry: Arc<RemoteDeploymentRegistry>,
    ttl_manager: Arc<TtlManager>,
    provider_regions: HashMap<CloudProvider, String>,
    enabled: bool,
}

impl RemoteProviderManager {
    /// Initialize from Blueprint Manager config
    pub async fn new(ctx: &BlueprintManagerContext) -> Result<Option<Self>> {
        // Check if remote providers are configured
        if !ctx
            .cloud_config()
            .as_ref()
            .is_some_and(|config| config.enabled)
        {
            info!("Remote cloud providers not configured");
            return Ok(None);
        }

        // Create deployment tracker
        let tracker_path = ctx.data_dir().join("remote_deployments");
        let tracker = Arc::new(DeploymentTracker::new(&tracker_path).await?);

        // Create registry and provisioner
        let registry = Arc::new(RemoteDeploymentRegistry::new(tracker.clone()));
        let provisioner = Arc::new(CloudProvisioner::new().await?);
        let provider_regions = configured_provider_regions(ctx);

        // Create TTL manager for automatic cleanup
        let (expiry_tx, _expiry_rx) = tokio::sync::mpsc::unbounded_channel();
        let ttl_manager = Arc::new(TtlManager::new(registry.clone(), expiry_tx));

        Ok(Some(Self {
            provisioner,
            registry,
            ttl_manager,
            provider_regions,
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
        let tee_required = env_bool("BLUEPRINT_REMOTE_TEE_REQUIRED");

        // Start from intelligent preference based on resource requirements.
        let preferred_provider = if tee_required {
            CloudProvider::AWS
        } else if resource_spec.gpu_count.is_some() {
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
        let provider = self.select_configured_provider(preferred_provider, tee_required)?;

        // Use configured region when available.
        let region = self
            .provider_regions
            .get(&provider)
            .map(String::as_str)
            .unwrap_or_else(|| provider_default_region(&provider));

        if tee_required && !supports_tee(&provider) {
            return Err(Error::TeePrerequisiteMissing {
                prerequisite: format!("{provider} confidential-compute support"),
                hint: "Select AWS, GCP, or Azure when BLUEPRINT_REMOTE_TEE_REQUIRED=true"
                    .to_string(),
            });
        }

        let instance = self
            .provisioner
            .provision_with_requirements(provider.clone(), &resource_spec, region, tee_required)
            .await?;

        self.registry
            .register(
                blueprint_id,
                service_id,
                RemoteDeploymentConfig {
                    deployment_type: deployment_type_from_provider(&provider),
                    provider: Some(provider.clone()),
                    region: Some(region.to_string()),
                    instance_id: instance.id.clone(),
                    resource_spec: resource_spec.clone(),
                    ttl_seconds: Some(3600),
                    deployed_at: Utc::now(),
                },
            )
            .await;

        info!("Service deployed to {}: instance={}", provider, instance.id);
        self.ttl_manager
            .register_ttl(blueprint_id, service_id, 3600)
            .await; // 1 hour default

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
        self.registry.cleanup(blueprint_id, service_id).await?;

        Ok(())
    }
}

// Cloud configuration types are now imported from blueprint_remote_providers

impl RemoteProviderManager {
    fn select_configured_provider(
        &self,
        preferred: CloudProvider,
        tee_required: bool,
    ) -> Result<CloudProvider> {
        let ordered_candidates = if tee_required {
            vec![
                preferred,
                CloudProvider::AWS,
                CloudProvider::GCP,
                CloudProvider::Azure,
            ]
        } else {
            vec![
                preferred,
                CloudProvider::Vultr,
                CloudProvider::DigitalOcean,
                CloudProvider::GCP,
                CloudProvider::AWS,
                CloudProvider::Azure,
            ]
        };

        for candidate in ordered_candidates {
            if self.provider_regions.contains_key(&candidate)
                && (!tee_required || supports_tee(&candidate))
            {
                return Ok(candidate);
            }
        }

        Err(Error::Other(
            "No configured cloud provider can satisfy deployment requirements".to_string(),
        ))
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

fn configured_provider_regions(ctx: &BlueprintManagerContext) -> HashMap<CloudProvider, String> {
    let mut regions = HashMap::new();
    if let Some(config) = ctx.cloud_config() {
        if let Some(aws) = &config.aws {
            if aws.enabled {
                regions.insert(CloudProvider::AWS, aws.region.clone());
            }
        }
        if let Some(gcp) = &config.gcp {
            if gcp.enabled {
                regions.insert(CloudProvider::GCP, gcp.region.clone());
            }
        }
        if let Some(azure) = &config.azure {
            if azure.enabled {
                regions.insert(CloudProvider::Azure, azure.region.clone());
            }
        }
        if let Some(do_cfg) = &config.digital_ocean {
            if do_cfg.enabled {
                regions.insert(CloudProvider::DigitalOcean, do_cfg.region.clone());
            }
        }
        if let Some(vultr) = &config.vultr {
            if vultr.enabled {
                regions.insert(CloudProvider::Vultr, vultr.region.clone());
            }
        }
    }
    regions
}

fn provider_default_region(provider: &CloudProvider) -> &'static str {
    match provider {
        CloudProvider::AWS => "us-east-1",
        CloudProvider::GCP => "us-central1",
        CloudProvider::Azure => "eastus",
        CloudProvider::DigitalOcean => "nyc3",
        CloudProvider::Vultr => "ewr",
        _ => "default",
    }
}
