//! Simple provider selection logic for remote deployments.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// Supported cloud providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloudProvider {
    AWS,
    GCP,
    Azure,
    DigitalOcean,
    Vultr,
}

impl std::fmt::Display for CloudProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AWS => write!(f, "AWS"),
            Self::GCP => write!(f, "Google Cloud"),
            Self::Azure => write!(f, "Azure"),
            Self::DigitalOcean => write!(f, "DigitalOcean"),
            Self::Vultr => write!(f, "Vultr"),
        }
    }
}

/// Resource specification for deployments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    pub cpu: f32,
    pub memory_gb: f32,
    pub storage_gb: f32,
    pub gpu_count: Option<u32>,
    pub allow_spot: bool,
}

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
}

impl Default for ProviderPreferences {
    fn default() -> Self {
        Self {
            gpu_providers: vec![CloudProvider::GCP, CloudProvider::AWS],
            cpu_intensive: vec![
                CloudProvider::Vultr,
                CloudProvider::DigitalOcean,
                CloudProvider::AWS,
            ],
            memory_intensive: vec![CloudProvider::AWS, CloudProvider::GCP],
            cost_optimized: vec![CloudProvider::Vultr, CloudProvider::DigitalOcean],
        }
    }
}

/// Simple provider selector using first-match strategy.
pub struct ProviderSelector {
    preferences: ProviderPreferences,
}

impl ProviderSelector {
    /// Create new provider selector with preferences.
    pub fn new(preferences: ProviderPreferences) -> Self {
        Self { preferences }
    }

    /// Create provider selector with default preferences.
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
    /// - High scale (>10 instances) → Use Kubernetes
    pub fn select_target(&self, requirements: &ResourceSpec) -> Result<DeploymentTarget> {
        info!(
            "Selecting deployment target for requirements: {:?}",
            requirements
        );

        // For high-scale workloads, prefer K8s
        // Note: ResourceSpec doesn't have instance count yet, this is for future expansion
        // if requirements.instance_count.unwrap_or(1) > 10 {
        //     info!("High-scale workload detected, selecting Kubernetes");
        //     return Ok(DeploymentTarget::Kubernetes {
        //         context: "production".to_string(),
        //         namespace: "blueprints".to_string(),
        //     });
        // }

        let provider = self.select_provider(requirements)?;
        Ok(DeploymentTarget::CloudInstance(provider))
    }

    /// Select cloud provider based on resource requirements.
    pub fn select_provider(&self, requirements: &ResourceSpec) -> Result<CloudProvider> {
        let candidates = if requirements.gpu_count.is_some() {
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
                Ok(*provider)
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

        // Add all other provider categories as fallbacks
        if requirements.gpu_count.is_some() {
            // For GPU workloads, fallback to CPU-intensive providers
            fallbacks.extend(&self.preferences.cpu_intensive);
        } else {
            // For other workloads, try all categories
            fallbacks.extend(&self.preferences.cost_optimized);
            fallbacks.extend(&self.preferences.cpu_intensive);
            fallbacks.extend(&self.preferences.memory_intensive);
        }

        // Remove duplicates and the already-tried primary provider
        let primary = self.select_provider(requirements).ok();
        fallbacks.retain(|p| Some(*p) != primary);
        fallbacks.dedup();

        info!("Fallback providers: {:?}", fallbacks);
        fallbacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_provider_selection() {
        let selector = ProviderSelector::with_defaults();
        let requirements = ResourceSpec {
            cpu: 4.0,
            memory_gb: 16.0,
            storage_gb: 100.0,
            gpu_count: Some(1),
            allow_spot: false,
        };

        let provider = selector.select_provider(&requirements).unwrap();
        // Should select first GPU provider (GCP)
        assert_eq!(provider, CloudProvider::GCP);
    }

    #[test]
    fn test_cpu_intensive_selection() {
        let selector = ProviderSelector::with_defaults();
        let requirements = ResourceSpec {
            cpu: 16.0, // High CPU
            memory_gb: 32.0,
            storage_gb: 200.0,
            gpu_count: None,
            allow_spot: false,
        };

        let provider = selector.select_provider(&requirements).unwrap();
        // Should select first CPU-intensive provider (Vultr)
        assert_eq!(provider, CloudProvider::Vultr);
    }

    #[test]
    fn test_cost_optimized_selection() {
        let selector = ProviderSelector::with_defaults();
        let requirements = ResourceSpec {
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 20.0,
            gpu_count: None,
            allow_spot: true,
        };

        let provider = selector.select_provider(&requirements).unwrap();
        // Should select first cost-optimized provider (Vultr)
        assert_eq!(provider, CloudProvider::Vultr);
    }

    #[test]
    fn test_fallback_providers() {
        let selector = ProviderSelector::with_defaults();
        let requirements = ResourceSpec {
            cpu: 4.0,
            memory_gb: 16.0,
            storage_gb: 100.0,
            gpu_count: Some(1),
            allow_spot: false,
        };

        let fallbacks = selector.get_fallback_providers(&requirements);
        // Should include CPU-intensive providers as fallback for GPU workloads
        assert!(fallbacks.contains(&CloudProvider::Vultr));
        assert!(fallbacks.contains(&CloudProvider::DigitalOcean));
        // Should not include the primary selection (GCP)
        assert!(!fallbacks.contains(&CloudProvider::GCP));
    }
}
