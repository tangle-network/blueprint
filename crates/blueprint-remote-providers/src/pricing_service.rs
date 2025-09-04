//! Unified pricing service that consolidates all pricing logic
//!
//! Integrates with the existing blueprint-pricing-engine where available

use crate::error::{Error, Result};
use crate::remote::CloudProvider;
use crate::resources_simple::ResourceSpec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified pricing service
pub struct PricingService {
    #[cfg(feature = "pricing")]
    pricing_engine: Option<blueprint_pricing_engine::PricingEngine>,
    cloud_markups: HashMap<CloudProvider, f64>,
}

impl PricingService {
    pub fn new() -> Self {
        let mut cloud_markups = HashMap::new();
        // Cloud provider markups over base cost
        cloud_markups.insert(CloudProvider::AWS, 1.2);
        cloud_markups.insert(CloudProvider::GCP, 1.15);
        cloud_markups.insert(CloudProvider::Azure, 1.25);
        cloud_markups.insert(CloudProvider::DigitalOcean, 1.1);
        cloud_markups.insert(CloudProvider::Vultr, 1.05);

        Self {
            #[cfg(feature = "pricing")]
            pricing_engine: blueprint_pricing_engine::PricingEngine::new().ok(),
            cloud_markups,
        }
    }

    /// Calculate cost for a resource specification
    pub fn calculate_cost(
        &self,
        spec: &ResourceSpec,
        provider: CloudProvider,
        duration_hours: f64,
    ) -> CostReport {
        // Use pricing engine if available
        #[cfg(feature = "pricing")]
        if let Some(engine) = &self.pricing_engine {
            if let Ok(cost) = self.calculate_with_engine(engine, spec, provider, duration_hours) {
                return cost;
            }
        }

        // Fallback to simplified calculation
        self.calculate_simple(spec, provider, duration_hours)
    }

    #[cfg(feature = "pricing")]
    fn calculate_with_engine(
        &self,
        engine: &blueprint_pricing_engine::PricingEngine,
        spec: &ResourceSpec,
        provider: CloudProvider,
        duration_hours: f64,
    ) -> Result<CostReport> {
        use blueprint_pricing_engine::{PriceRequest, ResourceUnit};

        let request = PriceRequest {
            cpu_cores: spec.cpu as u32,
            memory_gb: spec.memory_gb as u32,
            storage_gb: spec.storage_gb as u32,
            gpu_count: spec.gpu_count.unwrap_or(0),
            duration_hours,
        };

        let base_cost = engine.calculate_price(&request)?;
        let markup = self.cloud_markups.get(&provider).unwrap_or(&1.0);
        let final_cost = base_cost * markup;

        Ok(CostReport {
            provider,
            resource_spec: spec.clone(),
            duration_hours,
            base_hourly_cost: base_cost / duration_hours,
            final_hourly_cost: final_cost / duration_hours,
            total_cost: final_cost,
            discount_applied: spec.allow_spot,
            discount_percentage: if spec.allow_spot { 30.0 } else { 0.0 },
        })
    }

    fn calculate_simple(
        &self,
        spec: &ResourceSpec,
        provider: CloudProvider,
        duration_hours: f64,
    ) -> CostReport {
        let base_hourly = spec.estimate_hourly_cost();
        let markup = self.cloud_markups.get(&provider).unwrap_or(&1.0);
        let final_hourly = base_hourly * markup;

        CostReport {
            provider,
            resource_spec: spec.clone(),
            duration_hours,
            base_hourly_cost: base_hourly,
            final_hourly_cost: final_hourly,
            total_cost: final_hourly * duration_hours,
            discount_applied: spec.allow_spot,
            discount_percentage: if spec.allow_spot { 30.0 } else { 0.0 },
        }
    }

    /// Compare costs across all providers
    pub fn compare_providers(&self, spec: &ResourceSpec, duration_hours: f64) -> Vec<CostReport> {
        let providers = vec![
            CloudProvider::AWS,
            CloudProvider::GCP,
            CloudProvider::Azure,
            CloudProvider::DigitalOcean,
            CloudProvider::Vultr,
        ];

        providers
            .into_iter()
            .map(|provider| self.calculate_cost(spec, provider, duration_hours))
            .collect()
    }

    /// Find the cheapest provider for given resources
    pub fn find_cheapest_provider(
        &self,
        spec: &ResourceSpec,
        duration_hours: f64,
    ) -> (CloudProvider, CostReport) {
        let reports = self.compare_providers(spec, duration_hours);
        let cheapest = reports
            .into_iter()
            .min_by(|a, b| {
                a.final_hourly_cost
                    .partial_cmp(&b.final_hourly_cost)
                    .unwrap()
            })
            .expect("At least one provider should exist");

        (cheapest.provider, cheapest)
    }
}

/// Cost calculation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostReport {
    pub provider: CloudProvider,
    pub resource_spec: ResourceSpec,
    pub duration_hours: f64,
    pub base_hourly_cost: f64,
    pub final_hourly_cost: f64,
    pub total_cost: f64,
    pub discount_applied: bool,
    pub discount_percentage: f64,
}

impl CostReport {
    /// Check if cost exceeds a threshold
    pub fn exceeds_threshold(&self, max_hourly: f64) -> bool {
        self.final_hourly_cost > max_hourly
    }

    /// Get cost savings compared to another report
    pub fn savings_vs(&self, other: &CostReport) -> f64 {
        other.total_cost - self.total_cost
    }

    /// Format as human-readable string
    pub fn format_summary(&self) -> String {
        format!(
            "{}: ${:.2}/hr (${:.2} total for {:.0} hours)",
            self.provider, self.final_hourly_cost, self.total_cost, self.duration_hours
        )
    }
}

impl Default for PricingService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_calculation() {
        let service = PricingService::new();
        let spec = ResourceSpec::basic();

        let report = service.calculate_cost(spec, CloudProvider::AWS, 24.0);
        assert!(report.final_hourly_cost > 0.0);
        assert_eq!(report.duration_hours, 24.0);
        assert!(report.total_cost > report.final_hourly_cost);
    }

    #[test]
    fn test_provider_comparison() {
        let service = PricingService::new();
        let spec = ResourceSpec::recommended();

        let reports = service.compare_providers(&spec, 730.0); // Monthly
        assert_eq!(reports.len(), 5);

        // Verify different providers have different costs due to markup
        let aws_cost = reports
            .iter()
            .find(|r| r.provider == CloudProvider::AWS)
            .unwrap();
        let vultr_cost = reports
            .iter()
            .find(|r| r.provider == CloudProvider::Vultr)
            .unwrap();
        assert!(aws_cost.final_hourly_cost > vultr_cost.final_hourly_cost);
    }

    #[test]
    fn test_cheapest_provider() {
        let service = PricingService::new();
        let spec = ResourceSpec::performance();

        let (provider, report) = service.find_cheapest_provider(&spec, 1.0);
        assert_eq!(provider, CloudProvider::Vultr); // Should be cheapest due to lowest markup
        assert!(report.final_hourly_cost > 0.0);
    }

    #[test]
    fn test_spot_discount() {
        let service = PricingService::new();

        let regular = ResourceSpec::basic();
        let spot = ResourceSpec {
            allow_spot: true,
            ..regular.clone()
        };

        let regular_cost = service.calculate_cost(regular, CloudProvider::AWS, 1.0);
        let spot_cost = service.calculate_cost(spot, CloudProvider::AWS, 1.0);

        assert!(spot_cost.final_hourly_cost < regular_cost.final_hourly_cost);
        assert_eq!(spot_cost.discount_percentage, 30.0);
    }

    #[test]
    fn test_threshold_check() {
        let service = PricingService::new();
        let spec = ResourceSpec::minimal();

        let report = service.calculate_cost(spec, CloudProvider::AWS, 1.0);
        assert!(!report.exceeds_threshold(1.0)); // Minimal should be under $1/hr
        assert!(report.exceeds_threshold(0.01)); // But over 1 cent
    }
}
