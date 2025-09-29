//! Integration with the Pricing Engine for cost calculations
//!
//! This module bridges the remote-providers resource model with the existing
//! pricing engine to provide accurate cost calculations for both local and
//! remote deployments.

use crate::core::error::Result;
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Pricing calculator that integrates with the Pricing Engine
///
/// Provides cost calculations for both local and remote deployments using
/// the resource model.
pub struct PricingCalculator {
    /// Pricing configuration loaded from TOML files
    pricing_config: PricingConfig,

    /// Provider-specific multipliers for cloud markup
    cloud_multipliers: HashMap<CloudProvider, f64>,
}

impl PricingCalculator {
    /// Create a new pricing calculator with default configuration
    pub fn new() -> Result<Self> {
        let pricing_config = Self::load_default_config()?;

        let mut cloud_multipliers = HashMap::new();
        // Cloud providers typically have markup over raw resource costs
        cloud_multipliers.insert(CloudProvider::AWS, 1.2);
        cloud_multipliers.insert(CloudProvider::GCP, 1.15);
        cloud_multipliers.insert(CloudProvider::Azure, 1.25);
        cloud_multipliers.insert(CloudProvider::DigitalOcean, 1.1);
        cloud_multipliers.insert(CloudProvider::Vultr, 1.05);
        cloud_multipliers.insert(CloudProvider::Generic, 1.0); // Self-hosted

        Ok(Self {
            pricing_config,
            cloud_multipliers,
        })
    }

    /// Load pricing configuration from a specific file
    pub fn from_config_file(path: &Path) -> Result<Self> {
        let config_str = std::fs::read_to_string(path)
            .map_err(|e| crate::core::error::Error::ConfigurationError(e.to_string()))?;

        let pricing_config: PricingConfig = toml::from_str(&config_str)
            .map_err(|e| crate::core::error::Error::ConfigurationError(e.to_string()))?;

        let cloud_multipliers = Self::default_multipliers();

        Ok(Self {
            pricing_config,
            cloud_multipliers,
        })
    }

    /// Calculate pricing for a resource specification
    pub fn calculate_cost(
        &self,
        spec: &ResourceSpec,
        provider: &CloudProvider,
        duration_hours: f64,
    ) -> DetailedCostReport {
        // Convert to pricing units
        let units = crate::core::resources::to_pricing_units(spec);

        // Get base resource costs
        let mut resource_costs = HashMap::new();
        let mut total_hourly = 0.0;

        for (resource_type, quantity) in &units {
            if let Some(rate) = self.get_resource_rate(resource_type) {
                let hourly_cost = quantity * rate;
                resource_costs.insert(
                    resource_type.to_string(),
                    ResourceCost {
                        quantity: *quantity,
                        rate_per_unit: rate,
                        total_hourly: hourly_cost,
                    },
                );
                total_hourly += hourly_cost;
            }
        }

        // Apply cloud provider multiplier
        let cloud_multiplier = self.cloud_multipliers.get(provider).unwrap_or(&1.0);

        let adjusted_hourly = total_hourly * cloud_multiplier;

        // Apply spot instance discount (real provider feature)
        let spot_multiplier = if spec.allow_spot { 0.7 } else { 1.0 };

        let final_hourly = adjusted_hourly * spot_multiplier;

        // Calculate totals
        let total_cost = final_hourly * duration_hours;
        let monthly_estimate = final_hourly * 730.0; // Average hours in a month

        DetailedCostReport {
            provider: provider.clone(),
            resource_costs,
            base_hourly_cost: total_hourly,
            cloud_markup: cloud_multiplier - 1.0,
            spot_discount: if spec.allow_spot { 0.3 } else { 0.0 },
            final_hourly_cost: final_hourly,
            total_cost,
            monthly_estimate,
            duration_hours,
            currency: "USD".to_string(),
        }
    }

    /// Compare costs across multiple providers
    pub fn compare_providers(
        &self,
        spec: &ResourceSpec,
        duration_hours: f64,
    ) -> Vec<DetailedCostReport> {
        let providers = vec![
            CloudProvider::AWS,
            CloudProvider::GCP,
            CloudProvider::Azure,
            CloudProvider::DigitalOcean,
            CloudProvider::Vultr,
            CloudProvider::Generic,
        ];

        providers
            .into_iter()
            .map(|provider| self.calculate_cost(spec, &provider, duration_hours))
            .collect()
    }

    /// Calculate resource rate based on pricing configuration
    fn get_resource_rate(&self, resource_type: &str) -> Option<f64> {
        self.pricing_config
            .default
            .resources
            .iter()
            .find(|r| r.kind == resource_type)
            .map(|r| r.price_per_unit_rate)
    }

    /// Load default pricing configuration
    fn load_default_config() -> Result<PricingConfig> {
        // This would normally load from the pricing engine's default config
        // Uses default pricing model format
        Ok(PricingConfig {
            default: PricingTier {
                resources: vec![
                    ResourcePrice {
                        kind: "CPU".to_string(),
                        count: 1,
                        price_per_unit_rate: 0.001,
                    },
                    ResourcePrice {
                        kind: "MemoryMB".to_string(),
                        count: 1024,
                        price_per_unit_rate: 0.00005,
                    },
                    ResourcePrice {
                        kind: "StorageMB".to_string(),
                        count: 1024,
                        price_per_unit_rate: 0.00002,
                    },
                    ResourcePrice {
                        kind: "NetworkEgressMB".to_string(),
                        count: 1024,
                        price_per_unit_rate: 0.00003,
                    },
                    ResourcePrice {
                        kind: "NetworkIngressMB".to_string(),
                        count: 1024,
                        price_per_unit_rate: 0.00001,
                    },
                    ResourcePrice {
                        kind: "GPU".to_string(),
                        count: 1,
                        price_per_unit_rate: 0.005,
                    },
                ],
            },
            blueprint_overrides: HashMap::new(),
        })
    }

    fn default_multipliers() -> HashMap<CloudProvider, f64> {
        let mut multipliers = HashMap::new();
        multipliers.insert(CloudProvider::AWS, 1.2);
        multipliers.insert(CloudProvider::GCP, 1.15);
        multipliers.insert(CloudProvider::Azure, 1.25);
        multipliers.insert(CloudProvider::DigitalOcean, 1.1);
        multipliers.insert(CloudProvider::Vultr, 1.05);
        multipliers.insert(CloudProvider::Generic, 1.0);
        multipliers
    }
}

impl Default for PricingCalculator {
    fn default() -> Self {
        Self::new().expect("Failed to create default pricing calculator")
    }
}

/// Pricing configuration structure matching the pricing engine format
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PricingConfig {
    default: PricingTier,
    #[serde(flatten)]
    blueprint_overrides: HashMap<String, PricingTier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PricingTier {
    resources: Vec<ResourcePrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResourcePrice {
    kind: String,
    count: u32,
    price_per_unit_rate: f64,
}

/// Detailed cost report with breakdown by resource type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedCostReport {
    pub provider: CloudProvider,
    pub resource_costs: HashMap<String, ResourceCost>,
    pub base_hourly_cost: f64,
    pub cloud_markup: f64,
    pub spot_discount: f64,
    pub final_hourly_cost: f64,
    pub total_cost: f64,
    pub monthly_estimate: f64,
    pub duration_hours: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCost {
    pub quantity: f64,
    pub rate_per_unit: f64,
    pub total_hourly: f64,
}

impl DetailedCostReport {
    /// Generate a human-readable summary
    pub fn summary(&self) -> String {
        let mut summary = format!("Cost Report for {}\n", self.provider);
        summary.push_str(&format!("Duration: {:.1} hours\n", self.duration_hours));
        summary.push_str(&format!(
            "Base Hourly Cost: ${:.4}\n",
            self.base_hourly_cost
        ));

        if self.cloud_markup > 0.0 {
            summary.push_str(&format!(
                "Cloud Markup: {:.1}%\n",
                self.cloud_markup * 100.0
            ));
        }

        if self.spot_discount > 0.0 {
            summary.push_str(&format!(
                "Spot Discount: -{:.1}%\n",
                self.spot_discount * 100.0
            ));
        }

        summary.push_str(&format!(
            "Final Hourly Cost: ${:.4}\n",
            self.final_hourly_cost
        ));
        summary.push_str(&format!("Total Cost: ${:.2}\n", self.total_cost));
        summary.push_str(&format!(
            "Monthly Estimate: ${:.2}\n",
            self.monthly_estimate
        ));

        summary
    }

    /// Check if costs exceed a threshold
    pub fn exceeds_threshold(&self, max_hourly: f64) -> bool {
        self.final_hourly_cost > max_hourly
    }
}

/// Integration with existing Pricing Engine types
pub mod pricing_engine_compat {
    use super::*;

    /// Convert resource spec to pricing engine ResourceUnit enum
    /// Integrates with the pricing engine crate
    pub fn to_resource_units(spec: &ResourceSpec) -> Vec<(String, f64)> {
        let units = crate::core::resources::to_pricing_units(spec);
        units.into_iter().collect()
    }

    /// Create a benchmark profile from usage metrics
    pub fn create_benchmark_profile(
        _spec: &ResourceSpec,
        actual_usage: &ResourceUsageMetrics,
    ) -> BenchmarkProfile {
        BenchmarkProfile {
            cpu_utilization: actual_usage.cpu_utilization_percent,
            memory_utilization: actual_usage.memory_utilization_percent,
            disk_io_ops: actual_usage.disk_iops,
            network_bandwidth_mbps: actual_usage.network_mbps,
            duration_seconds: actual_usage.duration_seconds,
        }
    }
}

/// Resource usage metrics for cost tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageMetrics {
    pub cpu_utilization_percent: f64,
    pub memory_utilization_percent: f64,
    pub disk_iops: u32,
    pub network_mbps: f64,
    pub duration_seconds: u64,
}

/// Benchmark profile for usage vs estimated comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkProfile {
    pub cpu_utilization: f64,
    pub memory_utilization: f64,
    pub disk_io_ops: u32,
    pub network_bandwidth_mbps: f64,
    pub duration_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::resources::ResourceSpec;

    #[test]
    fn test_pricing_calculation() {
        let calculator = PricingCalculator::new().unwrap();

        let spec = ResourceSpec {
            cpu: 4.0,
            memory_gb: 16.0,
            storage_gb: 100.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };

        let report = calculator.calculate_cost(
            &spec,
            &CloudProvider::AWS,
            24.0, // 24 hours
        );

        assert!(report.final_hourly_cost > 0.0);
        assert!(report.total_cost > 0.0);
        assert_eq!(report.currency, "USD");
    }

    #[test]
    fn test_provider_comparison() {
        let calculator = PricingCalculator::new().unwrap();

        let spec = ResourceSpec::default();

        let reports = calculator.compare_providers(&spec, 730.0);

        assert_eq!(reports.len(), 6);

        // Generic (self-hosted) should be cheapest
        let generic_report = reports
            .iter()
            .find(|r| matches!(r.provider, CloudProvider::Generic))
            .unwrap();

        let aws_report = reports
            .iter()
            .find(|r| matches!(r.provider, CloudProvider::AWS))
            .unwrap();

        assert!(generic_report.final_hourly_cost < aws_report.final_hourly_cost);
    }

    #[test]
    fn test_spot_pricing_discount() {
        let calculator = PricingCalculator::new().unwrap();

        let mut spec = ResourceSpec::default();

        // Regular instance
        let regular_cost = calculator.calculate_cost(&spec, &CloudProvider::AWS, 1.0);

        // Spot instance (should be 30% cheaper)
        spec.allow_spot = true;
        let spot_cost = calculator.calculate_cost(&spec, &CloudProvider::AWS, 1.0);

        assert!(spot_cost.final_hourly_cost < regular_cost.final_hourly_cost);
        assert_eq!(spot_cost.spot_discount, 0.3); // 30% discount

        // Verify actual discount calculation
        let expected_spot_cost = regular_cost.final_hourly_cost * 0.7;
        assert!((spot_cost.final_hourly_cost - expected_spot_cost).abs() < 0.001);
    }
}
