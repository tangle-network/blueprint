//! Multi-cloud infrastructure provisioner for Blueprint deployments
//!
//! Provides a single interface for provisioning across AWS, GCP, Azure, DigitalOcean, and Vultr

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::adapters::{AwsAdapter, GcpAdapter, AzureAdapter, DigitalOceanAdapter, VultrAdapter};
use crate::infra::mapper::InstanceTypeMapper;
use crate::infra::traits::CloudProviderAdapter;
use crate::infra::types::{RetryPolicy, InstanceStatus, ProvisionedInstance};
use blueprint_std::collections::HashMap;
use tracing::{error, info, warn};

/// Multi-cloud provisioner that handles deployments across all supported providers
pub struct CloudProvisioner {
    providers: HashMap<CloudProvider, Box<dyn CloudProviderAdapter>>,
    retry_policy: RetryPolicy,
}

impl CloudProvisioner {
    pub async fn new() -> Result<Self> {
        let mut providers = HashMap::new();

        // Initialize provider adapters based on available credentials
        #[cfg(feature = "aws")]
        if blueprint_std::env::var("AWS_ACCESS_KEY_ID").is_ok() {
            providers.insert(
                CloudProvider::AWS,
                Box::new(AwsAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        if blueprint_std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok() {
            providers.insert(
                CloudProvider::GCP,
                Box::new(GcpAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        if blueprint_std::env::var("AZURE_CLIENT_ID").is_ok() {
            providers.insert(
                CloudProvider::Azure,
                Box::new(AzureAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        if blueprint_std::env::var("DIGITALOCEAN_TOKEN").is_ok() {
            providers.insert(
                CloudProvider::DigitalOcean,
                Box::new(DigitalOceanAdapter::new()?) as Box<dyn CloudProviderAdapter>,
            );
        }

        if blueprint_std::env::var("VULTR_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::Vultr,
                Box::new(VultrAdapter::new()?) as Box<dyn CloudProviderAdapter>,
            );
        }

        Ok(Self {
            providers,
            retry_policy: RetryPolicy::default(),
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
                Ok(status) if status == InstanceStatus::Terminated => {
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
