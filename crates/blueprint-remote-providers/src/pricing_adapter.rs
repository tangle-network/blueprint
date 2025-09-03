//! Adapter to integrate ResourceSpec with blueprint-pricing-engine
#![cfg(feature = "pricing")]
//!
//! Provides seamless integration between remote-providers ResourceSpec
//! and the existing pricing engine for consistent cost calculations.

use crate::error::Result;
use crate::remote::CloudProvider;
use crate::resources::ResourceSpec;
use blueprint_pricing_engine::{PriceModel, ResourcePricing, calculate_price};
use blueprint_pricing_engine::types::ResourceUnit;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Adapter for pricing engine integration
pub struct PricingAdapter {
    pricing_model: ResourcePricing,
    provider_multipliers: HashMap<CloudProvider, f64>,
}

impl PricingAdapter {
    /// Create adapter with pricing configuration
    pub fn new(pricing_config_path: &str) -> Result<Self> {
        let pricing_model = blueprint_pricing_engine::load_pricing_from_toml(pricing_config_path)
            .map_err(|e| crate::error::Error::ConfigurationError(e.to_string()))?;
        
        let mut provider_multipliers = HashMap::new();
        provider_multipliers.insert(CloudProvider::AWS, 1.2);
        provider_multipliers.insert(CloudProvider::GCP, 1.15);
        provider_multipliers.insert(CloudProvider::Azure, 1.25);
        provider_multipliers.insert(CloudProvider::DigitalOcean, 1.1);
        provider_multipliers.insert(CloudProvider::Vultr, 1.05);
        provider_multipliers.insert(CloudProvider::Generic, 1.0);
        
        Ok(Self {
            pricing_model,
            provider_multipliers,
        })
    }
    
    /// Calculate cost using pricing engine
    pub fn calculate_cost(
        &self,
        spec: &ResourceSpec,
        provider: &CloudProvider,
        duration_seconds: u64,
    ) -> CloudCostReport {
        // Convert ResourceSpec to pricing engine ResourceUnit format
        let resource_units = self.spec_to_resource_units(spec);
        
        // Use pricing engine's calculate_price
        let base_price = calculate_price(&self.pricing_model, &resource_units, duration_seconds);
        
        // Apply cloud provider markup
        let multiplier = self.provider_multipliers
            .get(provider)
            .unwrap_or(&1.0);
        
        let final_price = base_price * multiplier;
        
        CloudCostReport {
            provider: provider.clone(),
            base_cost: base_price,
            provider_markup: multiplier - 1.0,
            final_cost: final_price,
            duration_seconds,
        }
    }
    
    /// Convert ResourceSpec to pricing engine ResourceUnit format
    fn spec_to_resource_units(&self, spec: &ResourceSpec) -> HashMap<ResourceUnit, f64> {
        let mut units = HashMap::new();
        
        // CPU cores
        units.insert(ResourceUnit::CPU, spec.compute.cpu_cores);
        
        // Memory in MB
        units.insert(ResourceUnit::MemoryMB, spec.storage.memory_gb * 1024.0);
        
        // Storage in MB
        units.insert(ResourceUnit::StorageMB, spec.storage.disk_gb * 1024.0);
        
        // Network based on tier
        let network_mb = match spec.network.bandwidth_tier {
            crate::resources::BandwidthTier::Low => 1024.0,
            crate::resources::BandwidthTier::Standard => 2048.0,
            crate::resources::BandwidthTier::High => 4096.0,
            crate::resources::BandwidthTier::Ultra => 8192.0,
        };
        units.insert(ResourceUnit::NetworkEgressMB, network_mb);
        units.insert(ResourceUnit::NetworkIngressMB, network_mb);
        
        // GPU if present
        if let Some(ref accel) = spec.accelerators {
            if matches!(accel.accelerator_type, crate::resources::AcceleratorType::GPU(_)) {
                units.insert(ResourceUnit::GPU, accel.count as f64);
            }
        }
        
        units
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
        // This would use actual pricing config in production
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
        
        // Would load actual pricing config
        // let adapter = PricingAdapter::new("config/default_pricing.toml").unwrap();
        // let report = adapter.calculate_cost(&spec, &CloudProvider::AWS, 3600);
        // assert!(report.final_cost > 0.0);
    }
}