//! Unified pricing service

use crate::core::error::Result;
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cost breakdown for a specific resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostItem {
    pub resource: String,
    pub unit_cost: f64,
    pub quantity: f64,
    pub total_cost: f64,
}

/// Comprehensive cost report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostReport {
    pub provider: CloudProvider,
    pub total_cost: f64,
    pub cost_breakdown: Vec<CostItem>,
    pub duration_hours: f64,
    pub generated_at: DateTime<Utc>,
}

/// Unified pricing service
pub struct PricingService {
    cloud_markups: HashMap<CloudProvider, f64>,
}

impl PricingService {
    pub fn new() -> Self {
        let mut cloud_markups = HashMap::new();
        cloud_markups.insert(CloudProvider::AWS, 1.2);
        cloud_markups.insert(CloudProvider::GCP, 1.15);
        cloud_markups.insert(CloudProvider::Azure, 1.25);
        cloud_markups.insert(CloudProvider::DigitalOcean, 1.1);
        cloud_markups.insert(CloudProvider::Vultr, 1.05);

        Self { cloud_markups }
    }

    pub fn calculate_cost(
        &self,
        spec: &ResourceSpec,
        provider: CloudProvider,
        duration_hours: f64,
    ) -> Result<CostReport> {
        self.calculate_simple(spec, provider, duration_hours)
    }

    fn calculate_simple(
        &self,
        spec: &ResourceSpec,
        provider: CloudProvider,
        duration_hours: f64,
    ) -> Result<CostReport> {
        let base_cpu_cost = 0.05;
        let base_memory_cost = 0.01;
        let base_storage_cost = 0.001;

        let markup = *self.cloud_markups.get(&provider).unwrap_or(&1.0);

        let cpu_cost = base_cpu_cost * spec.cpu as f64 * markup;
        let memory_cost = base_memory_cost * spec.memory_gb as f64 * markup;
        let storage_cost = base_storage_cost * spec.storage_gb as f64 * markup;

        let total_hourly = cpu_cost + memory_cost + storage_cost;
        let total_cost = total_hourly * duration_hours;

        Ok(CostReport {
            provider,
            total_cost,
            cost_breakdown: vec![
                CostItem {
                    resource: "compute".to_string(),
                    unit_cost: cpu_cost,
                    quantity: spec.cpu as f64,
                    total_cost: cpu_cost * duration_hours,
                },
                CostItem {
                    resource: "memory".to_string(),
                    unit_cost: memory_cost,
                    quantity: spec.memory_gb as f64,
                    total_cost: memory_cost * duration_hours,
                },
                CostItem {
                    resource: "storage".to_string(),
                    unit_cost: storage_cost,
                    quantity: spec.storage_gb as f64,
                    total_cost: storage_cost * duration_hours,
                },
            ],
            duration_hours,
            generated_at: Utc::now(),
        })
    }
}

impl Default for PricingService {
    fn default() -> Self {
        Self::new()
    }
}