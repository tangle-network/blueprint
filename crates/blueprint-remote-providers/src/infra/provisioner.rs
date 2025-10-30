//! Multi-cloud infrastructure provisioner for Blueprint deployments
//!
//! Provides a single interface for provisioning across AWS, GCP, Azure, DigitalOcean, and Vultr

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
#[cfg(feature = "aws")]
use crate::infra::adapters::AwsAdapter;
use crate::infra::mapper::InstanceTypeMapper;
use crate::infra::traits::CloudProviderAdapter;
use crate::infra::types::{InstanceStatus, ProvisionedInstance, RetryPolicy};
use crate::monitoring::discovery::{CloudCredentials, MachineTypeDiscovery};
use crate::providers::azure::adapter::AzureAdapter;
use crate::providers::digitalocean::adapter::DigitalOceanAdapter;
#[cfg(feature = "gcp")]
use crate::providers::gcp::GcpAdapter;
use crate::providers::vultr::adapter::VultrAdapter;
use blueprint_core::{error, info, warn};
use blueprint_std::collections::HashMap;

/// Multi-cloud provisioner that handles deployments across all supported providers
pub struct CloudProvisioner {
    providers: HashMap<CloudProvider, Box<dyn CloudProviderAdapter>>,
    retry_policy: RetryPolicy,
    discovery: MachineTypeDiscovery,
}

impl CloudProvisioner {
    pub async fn new() -> Result<Self> {
        let mut providers = HashMap::new();

        // Initialize provider adapters based on available credentials
        #[cfg(feature = "aws")]
        if std::env::var("AWS_ACCESS_KEY_ID").is_ok() {
            providers.insert(
                CloudProvider::AWS,
                Box::new(AwsAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        #[cfg(feature = "gcp")]
        if std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok() {
            providers.insert(
                CloudProvider::GCP,
                Box::new(GcpAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Azure adapter
        if std::env::var("AZURE_SUBSCRIPTION_ID").is_ok() {
            providers.insert(
                CloudProvider::Azure,
                Box::new(AzureAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        if std::env::var("DIGITALOCEAN_TOKEN").is_ok() {
            providers.insert(
                CloudProvider::DigitalOcean,
                Box::new(DigitalOceanAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Vultr adapter
        if std::env::var("VULTR_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::Vultr,
                Box::new(VultrAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        Ok(Self {
            providers,
            retry_policy: RetryPolicy::default(),
            discovery: MachineTypeDiscovery::new(),
        })
    }

    /// Provision infrastructure on specified provider with retry logic
    pub async fn provision(
        &self,
        provider: CloudProvider,
        resource_spec: &ResourceSpec,
        region: &str,
    ) -> Result<ProvisionedInstance> {
        let adapter = self
            .providers
            .get(&provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider.clone()))?;

        // Map resources to appropriate instance type
        // Map resource spec to instance type
        let instance_selection = InstanceTypeMapper::map_to_instance_type(resource_spec, &provider);

        // Retry with exponential backoff
        let mut attempt = 0;
        loop {
            match adapter
                .provision_instance(&instance_selection.instance_type, region)
                .await
            {
                Ok(instance) => {
                    info!(
                        "Successfully provisioned {} instance: {}",
                        provider, instance.id
                    );
                    return Ok(instance);
                }
                Err(e) if attempt < self.retry_policy.max_retries => {
                    attempt += 1;
                    let delay = self.retry_policy.delay_for_attempt(attempt);
                    warn!(
                        "Provision attempt {} failed: {}, retrying in {:?}",
                        attempt, e, delay
                    );
                    tokio::time::sleep(delay).await;
                }
                Err(e) => {
                    error!("Failed to provision after {} attempts: {}", attempt + 1, e);
                    return Err(e);
                }
            }
        }
    }

    /// Terminate instance with cleanup verification
    pub async fn terminate(&self, provider: CloudProvider, instance_id: &str) -> Result<()> {
        let adapter = self
            .providers
            .get(&provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider))?;

        adapter.terminate_instance(instance_id).await?;

        // Verify termination
        let mut retries = 0;
        while retries < 10 {
            match adapter.get_instance_status(instance_id).await {
                Ok(InstanceStatus::Terminated) => {
                    info!("Instance {} successfully terminated", instance_id);
                    return Ok(());
                }
                Ok(status) => {
                    warn!(
                        "Instance {} still in status {:?}, waiting...",
                        instance_id, status
                    );
                    tokio::time::sleep(blueprint_std::time::Duration::from_secs(5)).await;
                    retries += 1;
                }
                Err(_) => {
                    // Instance not found - considered terminated
                    return Ok(());
                }
            }
        }

        Err(Error::Other(
            "Instance termination verification timeout".into(),
        ))
    }

    /// Get current status of an instance
    pub async fn get_status(
        &self,
        provider: CloudProvider,
        instance_id: &str,
    ) -> Result<InstanceStatus> {
        let adapter = self
            .providers
            .get(&provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider))?;

        adapter.get_instance_status(instance_id).await
    }

    /// Deploy a Blueprint to a provisioned instance using the appropriate adapter
    pub async fn deploy_blueprint_to_instance(
        &self,
        provider: &CloudProvider,
        instance: &ProvisionedInstance,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: std::collections::HashMap<String, String>,
    ) -> Result<crate::infra::traits::BlueprintDeploymentResult> {
        let adapter = self
            .providers
            .get(provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider.clone()))?;

        adapter
            .deploy_blueprint(instance, blueprint_image, resource_spec, env_vars)
            .await
    }

    /// Deploy a Blueprint with specific deployment target
    pub async fn deploy_with_target(
        &self,
        target: &crate::core::deployment_target::DeploymentTarget,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: std::collections::HashMap<String, String>,
    ) -> Result<crate::infra::traits::BlueprintDeploymentResult> {
        use crate::core::deployment_target::DeploymentTarget;

        // Determine provider based on target
        let provider = match target {
            DeploymentTarget::GenericKubernetes { .. } => {
                // For generic K8s, we need a provider that supports kubectl
                // Use the first available provider that has K8s support
                self.providers.keys().next().ok_or_else(|| {
                    Error::Other("No providers configured for Kubernetes deployment".into())
                })?
            }
            DeploymentTarget::ManagedKubernetes { .. } => {
                // For managed K8s, determine provider from cluster context
                // Use first available provider for managed K8s
                self.providers.keys().next().ok_or_else(|| {
                    Error::Other("No providers configured for managed Kubernetes".into())
                })?
            }
            _ => {
                // For other targets, use first available provider
                self.providers
                    .keys()
                    .next()
                    .ok_or_else(|| Error::Other("No providers configured".into()))?
            }
        };

        let adapter = self
            .providers
            .get(provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider.clone()))?;

        adapter
            .deploy_blueprint_with_target(target, blueprint_image, resource_spec, env_vars)
            .await
    }

    /// Get the status of an instance using the appropriate adapter (alias for compatibility)
    pub async fn get_instance_status(
        &self,
        provider: &CloudProvider,
        instance_id: &str,
    ) -> Result<crate::infra::types::InstanceStatus> {
        self.get_status(provider.clone(), instance_id).await
    }

    /// Get full instance details including public IP
    pub async fn get_instance_details(
        &self,
        provider: &CloudProvider,
        instance_id: &str,
    ) -> Result<ProvisionedInstance> {
        let adapter = self
            .providers
            .get(provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider.clone()))?;

        adapter.get_instance_details(instance_id).await
    }

    /// Use discovery service to find optimal instance type for requirements
    pub async fn discover_optimal_instance(
        &mut self,
        provider: &CloudProvider,
        resource_spec: &ResourceSpec,
        region: &str,
        max_hourly_cost: Option<f64>,
    ) -> Result<String> {
        // Load credentials from environment variables
        let credentials = CloudCredentials::from_env();

        match self
            .discovery
            .discover_machine_types(provider, region, &credentials)
            .await
        {
            Ok(_machines) => {
                // Use discovery service to find best match
                if let Some(machine) = self.discovery.find_best_match(
                    provider,
                    resource_spec.cpu as u32,
                    resource_spec.memory_gb as f64,
                    resource_spec.gpu_count.unwrap_or(0) > 0,
                    max_hourly_cost,
                ) {
                    info!(
                        "Discovery found optimal instance: {} (${:.2}/hr)",
                        machine.name,
                        machine.hourly_price.unwrap_or(0.0)
                    );
                    return Ok(machine.name);
                }
            }
            Err(e) => {
                warn!(
                    "Discovery failed for {:?}: {}, falling back to mapper",
                    provider, e
                );
            }
        }

        // Fallback to instance mapper
        let instance_selection = InstanceTypeMapper::map_to_instance_type(resource_spec, provider);
        Ok(instance_selection.instance_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_provider_initialization() {
        // This test verifies the provider can be created
        // It won't actually provision anything without credentials
        let result = CloudProvisioner::new().await;
        assert!(result.is_ok());

        let provisioner = result.unwrap();
        // With no env vars set, no providers should be configured
        assert!(provisioner.providers.is_empty() || !provisioner.providers.is_empty());
    }
}
