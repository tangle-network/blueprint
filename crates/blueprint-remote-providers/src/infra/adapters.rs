//! Cloud provider adapter registry and factory
//!
//! This module provides a centralized registry for accessing cloud provider adapters
//! Each provider implements CloudProviderAdapter in their specific provider module
//! with proper security configurations and performance optimizations.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::infra::traits::CloudProviderAdapter;
pub use crate::providers::aws::AwsAdapter;
pub use crate::providers::gcp::GcpAdapter;
// pub use crate::providers::azure::AzureAdapter;
pub use crate::providers::digitalocean::adapter::DigitalOceanAdapter;
// pub use crate::providers::vultr::VultrAdapter;
use std::sync::Arc;

/// Factory for creating cloud provider adapters
pub struct AdapterFactory;

impl AdapterFactory {
    /// Create a cloud provider adapter for the specified provider
    pub async fn create_adapter(provider: CloudProvider) -> Result<Arc<dyn CloudProviderAdapter>> {
        match provider {
            CloudProvider::AWS => {
                let adapter = AwsAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            CloudProvider::GCP => {
                let adapter = GcpAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            CloudProvider::Azure => Err(Error::Other(
                "Azure provider not yet implemented".to_string(),
            )),
            CloudProvider::DigitalOcean => {
                let adapter = DigitalOceanAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            CloudProvider::Vultr => Err(Error::Other(
                "Vultr provider not yet implemented".to_string(),
            )),
            _ => Err(Error::Other(format!(
                "Provider {:?} not supported yet",
                provider
            ))),
        }
    }

    /// List all supported providers
    pub fn supported_providers() -> Vec<CloudProvider> {
        vec![
            CloudProvider::AWS,
            CloudProvider::GCP,
            CloudProvider::DigitalOcean,
        ]
    }

    /// Check if a provider is supported
    pub fn is_supported(provider: &CloudProvider) -> bool {
        matches!(
            provider,
            CloudProvider::AWS | CloudProvider::GCP | CloudProvider::DigitalOcean
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_aws_adapter_creation() {
        let adapter = AdapterFactory::create_adapter(CloudProvider::AWS).await;
        assert!(adapter.is_ok(), "AWS adapter should be available");
    }

    #[tokio::test]
    async fn test_unsupported_provider() {
        let adapter = AdapterFactory::create_adapter(CloudProvider::GCP).await;
        assert!(
            adapter.is_err(),
            "GCP adapter should not be implemented yet"
        );
    }

    #[test]
    fn test_supported_providers() {
        let providers = AdapterFactory::supported_providers();
        assert!(providers.contains(&CloudProvider::AWS));
        assert_eq!(providers.len(), 1);
    }
}
