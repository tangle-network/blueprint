//! Auto-deployment manager for Blueprint Manager integration
//!
//! This module provides the core logic for automatically selecting and deploying
//! to the cheapest available cloud provider based on resource requirements.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::deployment::manager_integration::RemoteDeploymentConfig;
use crate::pricing::fetcher::PricingFetcher;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
// removed unused imports

/// Deployment preferences configured by operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentPreferences {
    /// Preferred deployment type (if available)
    pub preferred_type: Option<crate::deployment::tracker::DeploymentType>,
    /// List of allowed deployment types in priority order
    pub allowed_types: Vec<crate::deployment::tracker::DeploymentType>,
    /// Whether to allow fallback to default if preferences unavailable
    pub allow_fallback: bool,
}

impl Default for DeploymentPreferences {
    fn default() -> Self {
        Self {
            preferred_type: None,
            // Default: prefer VMs over managed K8s (cost and simplicity)
            allowed_types: vec![
                DeploymentType::AwsEc2,
                DeploymentType::GcpGce,
                DeploymentType::AzureVm,
                DeploymentType::DigitalOceanDroplet,
                DeploymentType::VultrInstance,
                DeploymentType::SshRemote,
            ],
            allow_fallback: true,
        }
    }
}

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
    /// Deployment preferences loaded from config
    deployment_preferences: Arc<RwLock<DeploymentPreferences>>,
}

impl AutoDeploymentManager {
    pub fn new() -> Self {
        Self {
            enabled_providers: Arc::new(RwLock::new(Vec::new())),
            pricing_fetcher: Arc::new(RwLock::new(PricingFetcher::new())),
            max_hourly_cost: 1.0,
            deployment_preferences: Arc::new(RwLock::new(DeploymentPreferences::default())),
        }
    }

    /// Create a new manager with deployment preferences loaded from config file
    pub fn from_config_file(config_path: &std::path::Path) -> Result<Self> {
        let mut manager = Self::new();
        manager.load_deployment_preferences(config_path)?;
        Ok(manager)
    }

    /// Load deployment preferences from a TOML configuration file
    pub fn load_deployment_preferences(&mut self, config_path: &std::path::Path) -> Result<()> {
        let config_str = std::fs::read_to_string(config_path)
            .map_err(|e| Error::ConfigurationError(format!("Failed to read config file: {}", e)))?;

        let preferences: DeploymentPreferences = toml::from_str(&config_str)
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse config: {}", e)))?;

        // Validate that deployment types are available with current feature flags
        for deployment_type in &preferences.allowed_types {
            if !Self::is_deployment_type_compiled(*deployment_type) {
                tracing::warn!(
                    "Deployment type {:?} is not available (missing feature flag), will be skipped",
                    deployment_type
                );
            }
        }

        let manager_preferences = self.deployment_preferences.clone();
        tokio::spawn(async move {
            *manager_preferences.write().await = preferences;
        });

        tracing::info!("Loaded deployment preferences from config file");
        Ok(())
    }

    /// Check if a deployment type is compiled in (has required feature flags)
    fn is_deployment_type_compiled(
        deployment_type: crate::deployment::tracker::DeploymentType,
    ) -> bool {

        match deployment_type {
            // Kubernetes deployments require the kubernetes feature
            #[cfg(feature = "kubernetes")]
            DeploymentType::AwsEks
            | DeploymentType::GcpGke
            | DeploymentType::AzureAks
            | DeploymentType::DigitalOceanDoks
            | DeploymentType::VultrVke => true,

            #[cfg(not(feature = "kubernetes"))]
            DeploymentType::AwsEks
            | DeploymentType::GcpGke
            | DeploymentType::AzureAks
            | DeploymentType::DigitalOceanDoks
            | DeploymentType::VultrVke => false,

            // VM deployments and SSH are always available
            DeploymentType::AwsEc2
            | DeploymentType::GcpGce
            | DeploymentType::AzureVm
            | DeploymentType::DigitalOceanDroplet
            | DeploymentType::VultrInstance
            | DeploymentType::SshRemote
            | DeploymentType::BareMetal => true,

            // Local deployments are not managed by remote providers
            DeploymentType::LocalDocker
            | DeploymentType::LocalKubernetes
            | DeploymentType::LocalHypervisor => false,
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

        tracing::info!("Configured {} enabled cloud providers", enabled.len());
        for provider in enabled.iter() {
            tracing::info!(
                "  - {} in region {} (priority {})",
                provider.provider,
                provider.region,
                provider.priority
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
                match fetcher
                    .find_best_instance(
                        provider_config.provider.clone(),
                        &provider_config.region,
                        spec.cpu,
                        spec.memory_gb,
                        self.max_hourly_cost,
                    )
                    .await
                {
                    Ok(instance) if instance.hourly_price < best_price => {
                        best_price = instance.hourly_price;
                        best_option = Some((
                            provider_config.provider.clone(),
                            provider_config.region.clone(),
                            instance.hourly_price,
                        ));
                    }
                    Err(e) => {
                        tracing::debug!(
                            "No suitable instance for {:?}: {}",
                            provider_config.provider,
                            e
                        );
                    }
                    _ => {}
                }
            }
        }

        best_option.ok_or_else(|| {
            Error::ConfigurationError("No affordable deployment options available".into())
        })
    }

    /// Automatically deploy a service to the cheapest provider
    pub async fn auto_deploy_service(
        &self,
        blueprint_id: u64,
        service_id: u64,
        spec: ResourceSpec,
        ttl_seconds: Option<u64>,
    ) -> Result<RemoteDeploymentConfig> {
        tracing::info!(
            "Auto-deploying service blueprint:{} service:{}",
            blueprint_id,
            service_id
        );

        // Find cheapest provider with real pricing
        let (provider, region, price) = self.find_cheapest_provider(&spec).await?;

        tracing::info!(
            "Deploying to {} in {} (${:.4}/hour)",
            provider,
            region,
            price
        );

        // Actually provision infrastructure and deploy Blueprint
        let provisioner = crate::infra::provisioner::CloudProvisioner::new().await?;

        // Step 1: Provision cloud instance
        tracing::info!("Provisioning {} instance in {}", provider, region);
        let instance = provisioner
            .provision(provider.clone(), &spec, &region)
            .await?;

        // Step 2: Wait for instance to be running and get public IP
        let mut attempts = 0;
        let max_attempts = 30; // 5 minutes max wait time
        let mut updated_instance = instance;

        while updated_instance.public_ip.is_none() && attempts < max_attempts {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

            // Get updated instance info to check for public IP
            match provisioner
                .get_instance_status(&provider, &updated_instance.id)
                .await
            {
                Ok(status) if status == crate::infra::types::InstanceStatus::Running => {
                    // Try to get the instance details with public IP
                    // For now, we'll use a placeholder IP since getting public IP requires provider-specific calls
                    updated_instance.public_ip = Some("pending".to_string());
                    break;
                }
                Ok(_) => {
                    attempts += 1;
                    continue;
                }
                Err(e) => {
                    tracing::warn!("Failed to check instance status: {}", e);
                    attempts += 1;
                }
            }
        }

        if updated_instance.public_ip.is_none() {
            return Err(Error::Other(
                "Instance failed to get public IP within timeout".into(),
            ));
        }

        // Step 3: Deploy Blueprint to the instance
        tracing::info!("Deploying Blueprint to provisioned instance");
        let blueprint_image = format!("blueprint:{}-{}", blueprint_id, service_id); // TODO: Get actual image
        let env_vars = std::collections::HashMap::new(); // TODO: Add environment variables

        let deployment_result = provisioner
            .deploy_blueprint_to_instance(
                &provider,
                &updated_instance,
                &blueprint_image,
                &spec,
                env_vars,
            )
            .await?;

        tracing::info!(
            "Successfully deployed Blueprint with QoS endpoint: {:?}",
            deployment_result.qos_grpc_endpoint()
        );

        // Choose deployment type based on operator preferences and feature availability
        let deployment_preferences = self.deployment_preferences.read().await;
        let deployment_type = self.get_deployment_type(&provider, Some(&deployment_preferences));

        // Create deployment config with actual deployment info
        let config = RemoteDeploymentConfig {
            deployment_type,
            provider: Some(provider),
            region: Some(region.clone()),
            instance_id: deployment_result.blueprint_id,
            resource_spec: spec,
            ttl_seconds,
            deployed_at: Utc::now(),
        };

        Ok(config)
    }

    /// Get deployment type based on operator preferences and feature availability
    fn get_deployment_type(
        &self,
        provider: &CloudProvider,
        preferences: Option<&DeploymentPreferences>,
    ) -> crate::deployment::tracker::DeploymentType {

        // If operator specified a preference, use it (if available)
        if let Some(prefs) = preferences {
            if let Some(preferred) = prefs.preferred_type {
                if self.is_deployment_type_available(preferred, provider) {
                    return preferred;
                }
            }

            // Try allowed types in order
            for &deployment_type in &prefs.allowed_types {
                if self.is_deployment_type_available(deployment_type, provider) {
                    return deployment_type;
                }
            }
        }

        // Default fallback: prioritize VMs (simpler, cheaper) over managed K8s
        self.get_default_deployment_type(provider)
    }

    /// Check if a deployment type is available (compiled in and configured)
    fn is_deployment_type_available(
        &self,
        deployment_type: crate::deployment::tracker::DeploymentType,
        provider: &CloudProvider,
    ) -> bool {

        // First check if it's compiled in
        if !Self::is_deployment_type_compiled(deployment_type) {
            return false;
        }

        // Then check if provider matches deployment type
        match deployment_type {
            // Kubernetes deployments (already verified to be compiled in)
            DeploymentType::AwsEks => matches!(provider, CloudProvider::AWS),
            DeploymentType::GcpGke => matches!(provider, CloudProvider::GCP),
            DeploymentType::AzureAks => matches!(provider, CloudProvider::Azure),
            DeploymentType::DigitalOceanDoks => matches!(provider, CloudProvider::DigitalOcean),
            DeploymentType::VultrVke => matches!(provider, CloudProvider::Vultr),

            // VM deployments
            DeploymentType::AwsEc2 => matches!(provider, CloudProvider::AWS),
            DeploymentType::GcpGce => matches!(provider, CloudProvider::GCP),
            DeploymentType::AzureVm => matches!(provider, CloudProvider::Azure),
            DeploymentType::DigitalOceanDroplet => matches!(provider, CloudProvider::DigitalOcean),
            DeploymentType::VultrInstance => matches!(provider, CloudProvider::Vultr),

            // SSH remote is always available
            DeploymentType::SshRemote => true,
            DeploymentType::BareMetal => true,

            // Local deployments are not managed by remote providers
            DeploymentType::LocalDocker
            | DeploymentType::LocalKubernetes
            | DeploymentType::LocalHypervisor => false,
        }
    }

    /// Get the default deployment type for a provider (prefer VMs over managed K8s)
    fn get_default_deployment_type(
        &self,
        provider: &CloudProvider,
    ) -> crate::deployment::tracker::DeploymentType {

        match provider {
            CloudProvider::AWS => DeploymentType::AwsEc2,
            CloudProvider::GCP => DeploymentType::GcpGce,
            CloudProvider::Azure => DeploymentType::AzureVm,
            CloudProvider::DigitalOcean => DeploymentType::DigitalOceanDroplet,
            CloudProvider::Vultr => DeploymentType::VultrInstance,
            _ => DeploymentType::SshRemote,
        }
    }

    /// Generate an example configuration file for deployment preferences
    pub fn generate_example_config(output_path: &Path) -> Result<()> {
        let example_config = DeploymentPreferences::default();

        let config_toml = toml::to_string_pretty(&example_config)
            .map_err(|e| Error::ConfigurationError(format!("Failed to serialize config: {}", e)))?;

        let config_with_comments = format!(
            r#"# Blueprint Remote Providers - Deployment Preferences Configuration
# 
# This file configures how the auto-deployment manager selects deployment types
# when deploying Blueprints to remote cloud providers.
#
# Feature flags control which deployment types are available:
# - Default: VM deployments (EC2, GCE, etc.)
# - 'kubernetes' feature: Managed Kubernetes (EKS, GKE, etc.)

# Preferred deployment type (if available with current provider)
# Options: "AwsEc2", "AwsEks", "GcpGce", "GcpGke", "AzureVm", "AzureAks", 
#          "DigitalOceanDroplet", "DigitalOceanDoks", "VultrInstance", "VultrVke",
#          "SshRemote", "BareMetal"
preferred_type = {{ type = "AwsEc2" }}

# List of allowed deployment types in priority order
# The manager will try these in order if the preferred type is unavailable
allowed_types = [
    {{ type = "AwsEc2" }},
    {{ type = "GcpGce" }},
    {{ type = "AzureVm" }},
    {{ type = "DigitalOceanDroplet" }},
    {{ type = "VultrInstance" }},
    {{ type = "SshRemote" }},
]

# Whether to allow fallback to default if preferences unavailable
allow_fallback = true

# Example with Kubernetes enabled (requires 'kubernetes' feature):
# preferred_type = {{ type = "AwsEks" }}
# allowed_types = [
#     {{ type = "AwsEks" }},
#     {{ type = "GcpGke" }},
#     {{ type = "AwsEc2" }},    # Fallback to VMs
#     {{ type = "GcpGce" }},
# ]
"#
        );

        std::fs::write(output_path, config_with_comments).map_err(|e| {
            Error::ConfigurationError(format!("Failed to write config file: {}", e))
        })?;

        tracing::info!(
            "Generated example deployment preferences config at: {:?}",
            output_path
        );
        Ok(())
    }

    /// Load cloud credentials from a file
    pub fn load_credentials_from_file(&mut self, _path: &Path) -> Result<()> {
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
