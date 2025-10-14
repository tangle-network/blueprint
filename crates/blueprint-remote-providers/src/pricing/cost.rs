use crate::core::remote::CloudProvider;
use blueprint_std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Cost estimator for cloud deployments
///
/// This struct has been REMOVED. All hardcoded pricing has been eliminated.
/// Use the following for real pricing:
/// - `PricingFetcher` for VM instance pricing (AWS, Azure, DigitalOcean, Vultr)
/// - `FaasPricingFetcher` for serverless pricing (Lambda, Cloud Functions, Azure Functions)
/// - `PricingCalculator` for integration with user config files
pub struct CostEstimator;

impl CostEstimator {
    /// Creates a cost estimator - NO LONGER SUPPORTED
    ///
    /// All hardcoded pricing has been removed. Use real pricing APIs:
    /// - `PricingFetcher::new()` for VM instances
    /// - `FaasPricingFetcher::new()` for serverless
    pub fn new() -> Self {
        Self
    }

    /// Estimate costs for a deployment
    ///
    /// This method no longer works as all hardcoded pricing has been removed.
    /// Returns an error directing users to real pricing APIs.
    pub fn estimate(
        &self,
        _provider: &CloudProvider,
        _cpu_cores: f64,
        _memory_gb: f64,
        _storage_gb: f64,
        _replicas: u32,
    ) -> Result<CostReport, String> {
        Err(
            "CostEstimator has been removed. All hardcoded pricing eliminated. \
            Use PricingFetcher for real VM pricing or FaasPricingFetcher for serverless pricing."
                .to_string(),
        )
    }

    /// Track usage for cost reporting
    ///
    /// This method no longer works as all hardcoded pricing has been removed.
    /// Returns an error directing users to real pricing APIs.
    pub fn track_usage(
        &self,
        _provider: &CloudProvider,
        _cpu_hours: f64,
        _memory_gb_hours: f64,
        _storage_gb_days: f64,
        _network_gb: f64,
    ) -> Result<f64, String> {
        Err(
            "CostEstimator has been removed. All hardcoded pricing eliminated. \
            Use PricingFetcher for real VM pricing or FaasPricingFetcher for serverless pricing."
                .to_string(),
        )
    }
}

impl Default for CostEstimator {
    fn default() -> Self {
        Self::new()
    }
}

/// Cost report for deployments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostReport {
    pub estimated_hourly: f64,
    pub estimated_monthly: f64,
    pub currency: String,
    pub breakdown: HashMap<String, f64>,
}

impl CostReport {
    /// Create a simple cost alert message
    pub fn alert_if_exceeds(&self, monthly_limit: f64) -> Option<String> {
        if self.estimated_monthly > monthly_limit {
            Some(format!(
                "WARNING: Estimated monthly cost ${:.2} exceeds limit ${:.2}",
                self.estimated_monthly, monthly_limit
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_estimation_returns_error() {
        let estimator = CostEstimator::new();

        // Should return error since all hardcoded pricing removed
        let result = estimator.estimate(
            &CloudProvider::AWS,
            2.0,  // 2 CPUs
            4.0,  // 4GB RAM
            10.0, // 10GB storage
            3,    // 3 replicas
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("hardcoded pricing eliminated"));
    }

    #[test]
    fn test_usage_tracking_returns_error() {
        let estimator = CostEstimator::new();

        let result = estimator.track_usage(
            &CloudProvider::DigitalOcean,
            100.0, // 100 CPU hours
            200.0, // 200 GB-hours memory
            300.0, // 300 GB-days storage
            50.0,  // 50 GB network
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("hardcoded pricing eliminated"));
    }
}
