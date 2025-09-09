//! Adapter to integrate ResourceSpec with Pricing Engine
#![cfg(feature = "pricing")]
//!
//! Provides seamless integration between remote-providers ResourceSpec
//! and the existing pricing engine for consistent cost calculations.

use crate::error::Result;
use crate::remote::CloudProvider;
use crate::resources::ResourceSpec;
// Note: ResourceUnit would be imported from Pricing Engine in production
// For now we define a minimal interface for the adapter
use blueprint_std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Adapter for pricing engine integration
pub struct PricingAdapter {
    provider_multipliers: HashMap<CloudProvider, f64>,
}

impl PricingAdapter {
    /// Create adapter with cloud provider multipliers
    pub fn new() -> Result<Self> {
        let mut provider_multipliers = HashMap::new();
        provider_multipliers.insert(CloudProvider::AWS, 1.2);
        provider_multipliers.insert(CloudProvider::GCP, 1.15);
        provider_multipliers.insert(CloudProvider::Azure, 1.25);
        provider_multipliers.insert(CloudProvider::DigitalOcean, 1.1);
        provider_multipliers.insert(CloudProvider::Vultr, 1.05);
        provider_multipliers.insert(CloudProvider::Generic, 1.0);

        Ok(Self {
            provider_multipliers,
        })
    }

    /// Calculate cost estimation for cloud deployments
    pub fn calculate_cost(
        &self,
        spec: &ResourceSpec,
        provider: &CloudProvider,
        duration_seconds: u64,
    ) -> CloudCostReport {
        // Simple cost estimation based on resource requirements
        // This would integrate with the full pricing engine in production
        let mut base_cost = 0.0;

        // CPU cost (per core per hour)
        base_cost += spec.compute.cpu_cores * 0.05;

        // Memory cost (per GB per hour)
        base_cost += spec.storage.memory_gb * 0.01;

        // Storage cost (per GB per hour)
        base_cost += spec.storage.disk_gb * 0.001;

        // Network cost based on tier
        let network_cost = match spec.network.bandwidth_tier {
            crate::resources::BandwidthTier::Low => 0.002,
            crate::resources::BandwidthTier::Standard => 0.005,
            crate::resources::BandwidthTier::High => 0.01,
            crate::resources::BandwidthTier::Ultra => 0.02,
        };
        base_cost += network_cost;

        // GPU cost if present (per GPU per hour)
        if let Some(ref accel) = spec.accelerators {
            if matches!(
                accel.accelerator_type,
                crate::resources::AcceleratorType::GPU(_)
            ) {
                base_cost += accel.count as f64 * 1.5;
            }
        }

        // Apply duration scaling
        let duration_hours = duration_seconds as f64 / 3600.0;
        base_cost *= duration_hours;

        // Apply cloud provider markup
        let multiplier = self.provider_multipliers.get(provider).unwrap_or(&1.0);

        let final_cost = base_cost * multiplier;

        CloudCostReport {
            provider: provider.clone(),
            base_cost,
            provider_markup: multiplier - 1.0,
            final_cost,
            duration_seconds,
        }
    }

    /// Compare costs across providers using pricing engine
    pub fn compare_providers(
        &self,
        spec: &ResourceSpec,
        duration_seconds: u64,
    ) -> Vec<CloudCostReport> {
        vec![
            CloudProvider::AWS,
            CloudProvider::GCP,
            CloudProvider::Azure,
            CloudProvider::DigitalOcean,
            CloudProvider::Vultr,
            CloudProvider::Generic,
        ]
        .into_iter()
        .map(|provider| self.calculate_cost(spec, &provider, duration_seconds))
        .collect()
    }
}

/// Cost report for cloud deployments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudCostReport {
    pub provider: CloudProvider,
    pub base_cost: f64,
    pub provider_markup: f64,
    pub final_cost: f64,
    pub duration_seconds: u64,
}

impl CloudCostReport {
    /// Get hourly rate
    pub fn hourly_rate(&self) -> f64 {
        self.final_cost / (self.duration_seconds as f64 / 3600.0)
    }

    /// Get monthly estimate (730 hours)
    pub fn monthly_estimate(&self) -> f64 {
        self.hourly_rate() * 730.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{ComputeResources, StorageResources};

    #[test]
    fn test_pricing_engine_integration() {
        use crate::resources::{ComputeResources, StorageResources};

        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 4.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 16.0,
                disk_gb: 100.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let adapter = PricingAdapter::new().unwrap();
        let report = adapter.calculate_cost(&spec, &CloudProvider::AWS, 3600);
        assert!(report.final_cost > 0.0);
        assert_eq!(report.provider, CloudProvider::AWS);
    }
}
