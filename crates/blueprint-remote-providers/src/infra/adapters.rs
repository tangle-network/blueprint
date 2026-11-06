//! Cloud provider adapter registry and factory
//!
//! This module provides a centralized registry for accessing cloud provider adapters
//! Each provider implements CloudProviderAdapter in their specific provider module
//! with proper security configurations and performance optimizations.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::infra::traits::CloudProviderAdapter;
pub use crate::providers::akash::AkashAdapter;
#[cfg(feature = "aws")]
pub use crate::providers::aws::AwsAdapter;
pub use crate::providers::azure::adapter::AzureAdapter;
pub use crate::providers::bittensor_lium::BittensorLiumAdapter;
pub use crate::providers::coreweave::CoreWeaveAdapter;
pub use crate::providers::digitalocean::adapter::DigitalOceanAdapter;
pub use crate::providers::gcp::GcpAdapter;
pub use crate::providers::lambda_labs::LambdaLabsAdapter;
pub use crate::providers::prime_intellect::PrimeIntellectAdapter;
pub use crate::providers::runpod::RunPodAdapter;
pub use crate::providers::tensordock::TensorDockAdapter;
pub use crate::providers::vast_ai::VastAiAdapter;
pub use crate::providers::vultr::adapter::VultrAdapter;
use blueprint_std::sync::Arc;

/// Factory for creating cloud provider adapters
pub struct AdapterFactory;

impl AdapterFactory {
    /// Create a cloud provider adapter for the specified provider
    pub async fn create_adapter(provider: CloudProvider) -> Result<Arc<dyn CloudProviderAdapter>> {
        match provider {
            #[cfg(feature = "aws")]
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
            CloudProvider::LambdaLabs => {
                let adapter = LambdaLabsAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            CloudProvider::RunPod => {
                let adapter = RunPodAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            CloudProvider::VastAi => {
                let adapter = VastAiAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            CloudProvider::CoreWeave => {
                let adapter = CoreWeaveAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            CloudProvider::TensorDock => {
                let adapter = TensorDockAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            CloudProvider::Akash => {
                let adapter = AkashAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            CloudProvider::PrimeIntellect => {
                let adapter = PrimeIntellectAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            CloudProvider::BittensorLium => {
                let adapter = BittensorLiumAdapter::new().await?;
                Ok(Arc::new(adapter))
            }
            _ => Err(Error::Other(format!(
                "Provider {provider:?} not supported yet"
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
            CloudProvider::LambdaLabs,
            CloudProvider::RunPod,
            CloudProvider::VastAi,
            CloudProvider::CoreWeave,
            CloudProvider::TensorDock,
            CloudProvider::Akash,
            CloudProvider::PrimeIntellect,
            CloudProvider::BittensorLium,
        ]
    }

    /// Check if a provider is supported
    pub fn is_supported(provider: &CloudProvider) -> bool {
        matches!(
            provider,
            CloudProvider::AWS
                | CloudProvider::GCP
                | CloudProvider::Azure
                | CloudProvider::DigitalOcean
                | CloudProvider::Vultr
                | CloudProvider::LambdaLabs
                | CloudProvider::RunPod
                | CloudProvider::VastAi
                | CloudProvider::CoreWeave
                | CloudProvider::TensorDock
                | CloudProvider::Akash
                | CloudProvider::PrimeIntellect
                | CloudProvider::BittensorLium
        )
    }
}
