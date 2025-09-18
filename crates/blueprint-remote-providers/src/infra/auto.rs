//! Auto-deployment manager for Blueprint Manager integration
//!
//! This module provides the core logic for automatically selecting and deploying
//! to the cheapest available cloud provider based on resource requirements.

use crate::deployment::manager_integration::RemoteDeploymentConfig;
use crate::core::error::{Error, Result};
use crate::pricing::fetcher::PricingFetcher;
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use blueprint_std::collections::HashMap;
use blueprint_std::sync::Arc;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;

/// Configuration for a cloud provider that the operator has enabled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnabledProvider {
    pub provider: CloudProvider,
    pub region: String,
    pub credentials_env: HashMap<String, String>,
    pub enabled: bool,
    pub priority: u8, // Higher = prefer this provider (tie-breaker)
}

/// Auto-deployment manager that integrates with Blueprint Manager
pub struct AutoDeploymentManager {
    /// Enabled cloud providers from operator config
    enabled_providers: Arc<RwLock<Vec<EnabledProvider>>>,
    /// Real pricing data fetcher
    pricing_fetcher: Arc<RwLock<PricingFetcher>>,
    /// Maximum hourly cost limit
    max_hourly_cost: f64,
}

impl AutoDeploymentManager {
    pub fn new() -> Self {
        Self {
            enabled_providers: Arc::new(RwLock::new(Vec::new())),
            pricing_fetcher: Arc::new(RwLock::new(PricingFetcher::new())),
            max_hourly_cost: 1.0,
        }
    }
    
    /// Set maximum hourly cost limit
    pub fn set_max_hourly_cost(&mut self, cost: f64) {
        self.max_hourly_cost = cost;
    }

    /// Configure enabled providers from Blueprint Manager's cloud config
    pub async fn configure_providers(&self, providers: Vec<EnabledProvider>) {
        let mut enabled = self.enabled_providers.write().await;
        *enabled = providers.into_iter().filter(|p| p.enabled).collect();

        info!("Configured {} enabled cloud providers", enabled.len());
        for provider in enabled.iter() {
            info!(
                "  - {} in region {} (priority {})",
                provider.provider, provider.region, provider.priority
            );
        }
    }

    /// Find the cheapest deployment option for a given resource spec
    pub async fn find_cheapest_provider(
        &self,
        spec: &ResourceSpec,
    ) -> Result<(CloudProvider, String, f64)> {
        let enabled_providers = self.enabled_providers.read().await;

        if enabled_providers.is_empty() {
            // Default to AWS if no providers configured
            return Ok((CloudProvider::AWS, "us-west-2".to_string(), 0.10));
        }

        let mut best_option = None;
        let mut best_price = f64::MAX;
        
        {
            let mut fetcher = self.pricing_fetcher.write().await;

            // Get real pricing for each provider
            for provider_config in enabled_providers.iter() {
                // Find best instance dynamically based on requirements
                match fetcher.find_best_instance(
                    provider_config.provider.clone(),
                    &provider_config.region,
                    spec.cpu,
                    spec.memory_gb,
                    self.max_hourly_cost,
                ).await {
                    Ok(instance) if instance.hourly_price < best_price => {
                        best_price = instance.hourly_price;
                        best_option = Some((
                            provider_config.provider.clone(),
                            provider_config.region.clone(),
                            instance.hourly_price,
                        ));
                    }
                    Err(e) => {
                        tracing::debug!("No suitable instance for {:?}: {}", provider_config.provider, e);
                    }
                    _ => {}
                }
            }
        }

        best_option.ok_or_else(|| Error::ConfigurationError(
            "No affordable deployment options available".into()
        ))
    }

    /// Automatically deploy a service to the cheapest provider
    pub async fn auto_deploy_service(
        &self,
        blueprint_id: u64,
        service_id: u64,
        spec: ResourceSpec,
        ttl_seconds: Option<u64>,
    ) -> Result<RemoteDeploymentConfig> {
        info!(
            "Auto-deploying service blueprint:{} service:{}",
            blueprint_id, service_id
        );

        // Find cheapest provider with real pricing
        let (provider, region, price) = self.find_cheapest_provider(&spec).await?;

        info!(
            "Deploying to {} in {} (${:.4}/hour)",
            provider, region, price
        );

        // Create a simple deployment config
        // In production, this would actually provision infrastructure
        let config = RemoteDeploymentConfig {
            deployment_type: match &provider {
                CloudProvider::AWS => crate::deployment::tracker::DeploymentType::AwsEks,
                CloudProvider::GCP => crate::deployment::tracker::DeploymentType::GcpGke,
                CloudProvider::Azure => crate::deployment::tracker::DeploymentType::AzureAks,
                CloudProvider::DigitalOcean => crate::deployment::tracker::DeploymentType::DigitalOceanDoks,
                CloudProvider::Vultr => crate::deployment::tracker::DeploymentType::VultrVke,
                _ => crate::deployment::tracker::DeploymentType::SshRemote,
            },
            provider: Some(provider),
            region: Some(region.clone()),
            instance_id: format!("blueprint-{}-{}-{}", blueprint_id, service_id, uuid::Uuid::new_v4()),
            resource_spec: spec,
            ttl_seconds,
            deployed_at: Utc::now(),
        };
        
        // TODO: Actually provision infrastructure here
        
        Ok(config)
    }
    
    /// Load cloud credentials from a file
    pub fn load_credentials_from_file(&mut self, path: &std::path::Path) -> Result<()> {
        // TODO: Implement credential loading
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_cheapest_provider() {
        let manager = AutoDeploymentManager::new();
        
        let spec = ResourceSpec::basic();
        
        // Should return default AWS without configured providers
        let result = manager.find_cheapest_provider(&spec).await;
        assert!(result.is_ok());
        
        let (provider, region, price) = result.unwrap();
        assert_eq!(provider, CloudProvider::AWS);
        assert_eq!(region, "us-west-2");
        assert!(price > 0.0);
    }
}