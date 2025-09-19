//! Remote provider integration for Blueprint Manager
//!
//! Handles automatic cloud deployment when services are initiated

use crate::config::BlueprintManagerContext;
use crate::error::Result;
use blueprint_remote_providers::{
    CloudConfig, AwsConfig, GcpConfig, AzureConfig, DigitalOceanConfig, VultrConfig,
    CloudProvider, ResourceSpec
};
use blueprint_remote_providers::auto_deployment::{AutoDeploymentManager, EnabledProvider};
use blueprint_remote_providers::deployment::manager_integration::{
    RemoteDeploymentRegistry, RemoteEventHandler,
};
use blueprint_remote_providers::deployment::tracker::DeploymentTracker;
use blueprint_remote_providers::infrastructure::InfrastructureProvisioner;
use blueprint_std::collections::HashMap;
use blueprint_std::sync::Arc;
use blueprint_core::{error, info};
use tokio::sync::RwLock;

/// Remote provider manager that handles cloud deployments
pub struct RemoteProviderManager {
    auto_deployment: Arc<AutoDeploymentManager>,
    event_handler: Arc<RemoteEventHandler>,
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
        let tracker = Arc::new(
            DeploymentTracker::new(&tracker_path).await?
        );

        // Create registry and provisioner
        let registry = Arc::new(RemoteDeploymentRegistry::new(tracker.clone()));
        let provisioner = Arc::new(
            InfrastructureProvisioner::new(CloudProvider::AWS).await?
        );

        // Create auto-deployment manager
        let auto_deployment = Arc::new(
            AutoDeploymentManager::new(registry.clone(), provisioner)?
        );

        // Configure enabled providers from config
        let providers = Self::parse_cloud_config(config);
        auto_deployment.configure_providers(providers).await;

        // Create event handler
        let event_handler = Arc::new(
            RemoteEventHandler::new(registry)
        );

        Ok(Some(Self {
            auto_deployment,
            event_handler,
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

        info!("Remote provider handling service initiation: blueprint={}, service={}", 
            blueprint_id, service_id);

        // Use provided resources or default
        let resource_spec = resource_requirements.unwrap_or_else(ResourceSpec::minimal);

        // Auto-deploy to cheapest provider
        match self.auto_deployment.auto_deploy_service(
            blueprint_id,
            service_id,
            resource_spec,
            None, // No TTL by default
        ).await {
            Ok(deployment) => {
                info!("Service deployed to {}: instance={}", 
                    deployment.provider.unwrap_or(CloudProvider::AWS),
                    deployment.instance_id
                );
            }
            Err(e) => {
                error!("Failed to deploy service: {}", e);
            }
        }

        Ok(())
    }

    /// Handle service terminated event
    pub async fn on_service_terminated(
        &self,
        blueprint_id: u64,
        service_id: u64,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        info!("Remote provider handling service termination: blueprint={}, service={}", 
            blueprint_id, service_id);

        self.event_handler
            .on_service_terminated(blueprint_id, service_id)
            .await?;

        Ok(())
    }

    fn parse_cloud_config(config: &CloudConfig) -> Vec<EnabledProvider> {
        let mut providers = Vec::new();

        if let Some(aws) = &config.aws {
            providers.push(EnabledProvider {
                provider: CloudProvider::AWS,
                region: aws.region.clone(),
                credentials_env: HashMap::from([
                    ("AWS_ACCESS_KEY_ID".to_string(), aws.access_key.clone()),
                    ("AWS_SECRET_ACCESS_KEY".to_string(), aws.secret_key.clone()),
                ]),
                enabled: aws.enabled,
                priority: aws.priority.unwrap_or(10),
            });
        }

        if let Some(do_config) = &config.digital_ocean {
            providers.push(EnabledProvider {
                provider: CloudProvider::DigitalOcean,
                region: do_config.region.clone(),
                credentials_env: HashMap::from([
                    ("DO_API_TOKEN".to_string(), do_config.api_token.clone()),
                ]),
                enabled: do_config.enabled,
                priority: do_config.priority.unwrap_or(5),
            });
        }

        if let Some(gcp) = &config.gcp {
            providers.push(EnabledProvider {
                provider: CloudProvider::GCP,
                region: gcp.region.clone(),
                credentials_env: HashMap::from([
                    ("GCP_PROJECT_ID".to_string(), gcp.project_id.clone()),
                    ("GOOGLE_APPLICATION_CREDENTIALS".to_string(), gcp.service_account_path.clone()),
                ]),
                enabled: gcp.enabled,
                priority: gcp.priority.unwrap_or(8),
            });
        }

        if let Some(azure) = &config.azure {
            providers.push(EnabledProvider {
                provider: CloudProvider::Azure,
                region: azure.region.clone(),
                credentials_env: HashMap::from([
                    ("AZURE_CLIENT_ID".to_string(), azure.client_id.clone()),
                    ("AZURE_CLIENT_SECRET".to_string(), azure.client_secret.clone()),
                    ("AZURE_TENANT_ID".to_string(), azure.tenant_id.clone()),
                ]),
                enabled: azure.enabled,
                priority: azure.priority.unwrap_or(7),
            });
        }

        if let Some(vultr) = &config.vultr {
            providers.push(EnabledProvider {
                provider: CloudProvider::Vultr,
                region: vultr.region.clone(),
                credentials_env: HashMap::from([
                    ("VULTR_API_KEY".to_string(), vultr.api_key.clone()),
                ]),
                enabled: vultr.enabled,
                priority: vultr.priority.unwrap_or(3),
            });
        }

        providers
    }
}

// Cloud configuration types are now imported from blueprint_remote_providers