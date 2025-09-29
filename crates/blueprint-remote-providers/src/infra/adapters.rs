//! Cloud provider adapter registry and factory
//!
//! This module provides a centralized registry for accessing cloud provider adapters
//! Each provider implements CloudProviderAdapter in their specific provider module
//! with proper security configurations and performance optimizations.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::infra::traits::CloudProviderAdapter;
#[cfg(feature = "aws")]
pub use crate::providers::aws::AwsAdapter;
pub use crate::providers::gcp::GcpAdapter;
pub use crate::providers::azure::adapter::AzureAdapter;
pub use crate::providers::digitalocean::adapter::DigitalOceanAdapter;
pub use crate::providers::vultr::adapter::VultrAdapter;
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
            CloudProvider::Azure => {
                let adapter = AzureAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            CloudProvider::DigitalOcean => {
                let adapter = DigitalOceanAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            CloudProvider::Vultr => {
                let adapter = VultrAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
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
            CloudProvider::Azure,
            CloudProvider::DigitalOcean,
            CloudProvider::Vultr,
        ]
    }

    /// Check if a provider is supported
    pub fn is_supported(provider: &CloudProvider) -> bool {
        matches!(
            provider,
            CloudProvider::AWS | CloudProvider::GCP | CloudProvider::Azure | CloudProvider::DigitalOcean | CloudProvider::Vultr
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
    async fn test_gcp_adapter_creation() {
        // GCP requires project ID to be set
        if std::env::var("GCP_PROJECT_ID").is_ok() {
            let adapter = AdapterFactory::create_adapter(CloudProvider::GCP).await;
            assert!(adapter.is_ok(), "GCP adapter should be available");
        }
    }

    #[tokio::test]
    async fn test_azure_adapter_creation() {
        // Azure requires env vars to be set
        if std::env::var("AZURE_SUBSCRIPTION_ID").is_ok() {
            let adapter = AdapterFactory::create_adapter(CloudProvider::Azure).await;
            assert!(adapter.is_ok(), "Azure adapter should be available");
        }
    }

    #[tokio::test]
    async fn test_vultr_adapter_creation() {
        // Vultr requires API key
        if std::env::var("VULTR_API_KEY").is_ok() {
            let adapter = AdapterFactory::create_adapter(CloudProvider::Vultr).await;
            assert!(adapter.is_ok(), "Vultr adapter should be available");
        }
    }

    #[test]
    fn test_supported_providers() {
        let providers = AdapterFactory::supported_providers();
        assert!(providers.contains(&CloudProvider::AWS));
        assert!(providers.contains(&CloudProvider::Azure));
        assert!(providers.contains(&CloudProvider::DigitalOcean));
        assert!(providers.contains(&CloudProvider::Vultr));
        assert!(providers.contains(&CloudProvider::GCP));
        assert_eq!(providers.len(), 5);
    }
}
