use crate::remote::CloudProvider;
use blueprint_std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Cost estimator for cloud deployments
pub struct CostEstimator {
    providers: HashMap<CloudProvider, ProviderCost>,
}

impl CostEstimator {
    pub fn new() -> Self {
        let mut providers = HashMap::new();

        // AWS pricing (rough estimates)
        providers.insert(
            CloudProvider::AWS,
            ProviderCost {
                cpu_hour: 0.0464,
                memory_gb_hour: 0.004,
                storage_gb_month: 0.10,
                network_gb: 0.09,
            },
        );

        // GCP pricing
        providers.insert(
            CloudProvider::GCP,
            ProviderCost {
                cpu_hour: 0.0475,
                memory_gb_hour: 0.0035,
                storage_gb_month: 0.08,
                network_gb: 0.12,
            },
        );

        // Azure pricing
        providers.insert(
            CloudProvider::Azure,
            ProviderCost {
                cpu_hour: 0.048,
                memory_gb_hour: 0.0045,
                storage_gb_month: 0.115,
                network_gb: 0.087,
            },
        );

        // DigitalOcean pricing
        providers.insert(
            CloudProvider::DigitalOcean,
            ProviderCost {
                cpu_hour: 0.03,
                memory_gb_hour: 0.0025,
                storage_gb_month: 0.10,
                network_gb: 0.01,
            },
        );

        // Generic/self-hosted
        providers.insert(
            CloudProvider::Generic,
            ProviderCost {
                cpu_hour: 0.001,
                memory_gb_hour: 0.0001,
                storage_gb_month: 0.01,
                network_gb: 0.0,
            },
        );

        Self { providers }
    }

    /// Estimate costs for a deployment
    pub fn estimate(
        &self,
        provider: &CloudProvider,
        cpu_cores: f64,
        memory_gb: f64,
        storage_gb: f64,
        replicas: u32,
    ) -> CostReport {
        let provider_cost = self
            .providers
            .get(provider)
            .or_else(|| self.providers.get(&CloudProvider::Generic))
            .unwrap();

        let hourly_compute = cpu_cores * provider_cost.cpu_hour * replicas as f64;
        let hourly_memory = memory_gb * provider_cost.memory_gb_hour * replicas as f64;
        let monthly_storage = storage_gb * provider_cost.storage_gb_month * replicas as f64;

        let hourly_total = hourly_compute + hourly_memory;
        let monthly_total = hourly_total * 730.0 + monthly_storage;

        let mut breakdown = HashMap::new();
        breakdown.insert("compute".to_string(), hourly_compute * 730.0);
        breakdown.insert("memory".to_string(), hourly_memory * 730.0);
        breakdown.insert("storage".to_string(), monthly_storage);

        CostReport {
            estimated_hourly: hourly_total,
            estimated_monthly: monthly_total,
            currency: "USD".to_string(),
            breakdown,
        }
    }

    /// Track usage for cost reporting
    pub fn track_usage(
        &self,
        provider: &CloudProvider,
        cpu_hours: f64,
        memory_gb_hours: f64,
        storage_gb_days: f64,
        network_gb: f64,
    ) -> f64 {
        let provider_cost = self
            .providers
            .get(provider)
            .or_else(|| self.providers.get(&CloudProvider::Generic))
            .unwrap();

        let compute_cost = cpu_hours * provider_cost.cpu_hour;
        let memory_cost = memory_gb_hours * provider_cost.memory_gb_hour;
        let storage_cost = (storage_gb_days / 30.0) * provider_cost.storage_gb_month;
        let network_cost = network_gb * provider_cost.network_gb;

        compute_cost + memory_cost + storage_cost + network_cost
    }
}

impl Default for CostEstimator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
struct ProviderCost {
    cpu_hour: f64,
    memory_gb_hour: f64,
    storage_gb_month: f64,
    network_gb: f64,
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
    fn test_cost_estimation() {
        let estimator = CostEstimator::new();

        // Test AWS costs
        let report = estimator.estimate(
            &CloudProvider::AWS,
            2.0,  // 2 CPUs
            4.0,  // 4GB RAM
            10.0, // 10GB storage
            3,    // 3 replicas
        );

        assert!(report.estimated_hourly > 0.0);
        assert!(report.estimated_monthly > report.estimated_hourly * 24.0 * 28.0);
        assert_eq!(report.currency, "USD");
        assert!(report.breakdown.contains_key("compute"));
        assert!(report.breakdown.contains_key("memory"));
        assert!(report.breakdown.contains_key("storage"));
    }

    #[test]
    fn test_cost_alert() {
        let estimator = CostEstimator::new();

        let report = estimator.estimate(&CloudProvider::AWS, 4.0, 8.0, 100.0, 10);

        // This should exceed $100/month
        let alert = report.alert_if_exceeds(100.0);
        assert!(alert.is_some());

        // This should not exceed $10000/month
        let alert = report.alert_if_exceeds(10000.0);
        assert!(alert.is_none());
    }

    #[test]
    fn test_usage_tracking() {
        let estimator = CostEstimator::new();

        let cost = estimator.track_usage(
            &CloudProvider::DigitalOcean,
            100.0, // 100 CPU hours
            200.0, // 200 GB-hours memory
            300.0, // 300 GB-days storage
            50.0,  // 50 GB network
        );

        assert!(cost > 0.0);
    }
}
