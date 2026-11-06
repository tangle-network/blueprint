//! Simple provider selection logic for remote deployments.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

pub use blueprint_core::resources::{CloudProvider, ResourceSpec};

/// Deployment target options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentTarget {
    /// Deploy to a cloud provider instance
    CloudInstance(CloudProvider),
    /// Deploy to Kubernetes cluster
    Kubernetes { context: String, namespace: String },
    /// Hybrid deployment with fallback
    Hybrid {
        primary: CloudProvider,
        fallback_k8s: String,
    },
}

/// Provider preferences for different workload types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderPreferences {
    /// Providers for GPU workloads (ordered by preference)
    pub gpu_providers: Vec<CloudProvider>,
    /// Providers for CPU-intensive workloads
    pub cpu_intensive: Vec<CloudProvider>,
    /// Providers for memory-intensive workloads
    pub memory_intensive: Vec<CloudProvider>,
    /// Providers for cost-optimized workloads
    pub cost_optimized: Vec<CloudProvider>,
    /// Providers that can satisfy TEE requirements
    pub tee_capable: Vec<CloudProvider>,
}

impl Default for ProviderPreferences {
    fn default() -> Self {
        Self {
            gpu_providers: vec![
                CloudProvider::RunPod,
                CloudProvider::LambdaLabs,
                CloudProvider::VastAi,
                CloudProvider::PrimeIntellect,
                CloudProvider::Crusoe,
                CloudProvider::GCP,
                CloudProvider::AWS,
            ],
            cpu_intensive: vec![
                CloudProvider::Hetzner,
                CloudProvider::Vultr,
                CloudProvider::DigitalOcean,
                CloudProvider::AWS,
            ],
            memory_intensive: vec![
                CloudProvider::AWS,
                CloudProvider::GCP,
                CloudProvider::Hetzner,
            ],
            cost_optimized: vec![
                CloudProvider::VastAi,
                CloudProvider::Hetzner,
                CloudProvider::Vultr,
                CloudProvider::DigitalOcean,
            ],
            tee_capable: vec![CloudProvider::AWS, CloudProvider::GCP, CloudProvider::Azure],
        }
    }
}

/// Simple provider selector using first-match strategy.
pub struct ProviderSelector {
    preferences: ProviderPreferences,
}

impl ProviderSelector {
    /// Create new provider selector with preferences.
    #[must_use]
    pub fn new(preferences: ProviderPreferences) -> Self {
        Self { preferences }
    }

    /// Create provider selector with default preferences.
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(ProviderPreferences::default())
    }

    /// Select deployment target based on resource requirements.
    ///
    /// Uses simple first-match strategy:
    /// - GPU needed → Try GPU providers first
    /// - High CPU (>8 cores) → Try CPU-intensive providers
    /// - High memory (>32GB) → Try memory-intensive providers  
    /// - Otherwise → Try cost-optimized providers
    pub fn select_target(&self, requirements: &ResourceSpec) -> Result<DeploymentTarget> {
        info!(
            "Selecting deployment target for requirements: {:?}",
            requirements
        );

        let provider = self.select_provider(requirements)?;
        Ok(DeploymentTarget::CloudInstance(provider))
    }

    /// Select cloud provider based on resource requirements.
    pub fn select_provider(&self, requirements: &ResourceSpec) -> Result<CloudProvider> {
        let candidates = if requirements.tee_required {
            info!("TEE required, selecting from TEE-capable providers");
            &self.preferences.tee_capable
        } else if requirements.gpu_count.is_some() {
            info!("GPU required, selecting from GPU providers");
            &self.preferences.gpu_providers
        } else if requirements.cpu > 8.0 {
            info!(
                "High CPU requirement ({}), selecting from CPU-intensive providers",
                requirements.cpu
            );
            &self.preferences.cpu_intensive
        } else if requirements.memory_gb > 32.0 {
            info!(
                "High memory requirement ({}GB), selecting from memory-intensive providers",
                requirements.memory_gb
            );
            &self.preferences.memory_intensive
        } else {
            info!("Standard workload, selecting from cost-optimized providers");
            &self.preferences.cost_optimized
        };

        // Simple first-match strategy
        match candidates.first() {
            Some(provider) => {
                info!("Selected provider: {:?}", provider);
                Ok(provider.clone())
            }
            None => {
                warn!("No providers configured for workload requirements");
                Err(Error::Other(
                    "No providers configured for the given resource requirements".into(),
                ))
            }
        }
    }

    /// Try fallback providers if primary selection fails.
    pub fn get_fallback_providers(&self, requirements: &ResourceSpec) -> Vec<CloudProvider> {
        let mut fallbacks = Vec::new();

        if requirements.tee_required {
            fallbacks.extend(self.preferences.tee_capable.iter().cloned());
            let primary = self.select_provider(requirements).ok();
            fallbacks.retain(|p| Some(p) != primary.as_ref());
            fallbacks.dedup();
            info!("TEE fallback providers: {:?}", fallbacks);
            return fallbacks;
        }

        // Add all other provider categories as fallbacks
        if requirements.gpu_count.is_some() {
            // For GPU workloads, fallback to CPU-intensive providers
            fallbacks.extend(self.preferences.cpu_intensive.iter().cloned());
        } else {
            // For other workloads, try all categories
            fallbacks.extend(self.preferences.cost_optimized.iter().cloned());
            fallbacks.extend(self.preferences.cpu_intensive.iter().cloned());
            fallbacks.extend(self.preferences.memory_intensive.iter().cloned());
        }

        // Remove duplicates and the already-tried primary provider
        let primary = self.select_provider(requirements).ok();
        fallbacks.retain(|p| Some(p) != primary.as_ref());
        fallbacks.dedup();

        info!("Fallback providers: {:?}", fallbacks);
        fallbacks
    }
}
