//! Unified pricing service

use crate::core::error::Result;
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
///
/// ALL HARDCODED PRICING HAS BEEN REMOVED.
/// This service now requires real pricing data from:
/// - `PricingFetcher` for VM instances
/// - `FaasPricingFetcher` for serverless
pub struct PricingService;

impl PricingService {
    pub fn new() -> Self {
        Self
    }

    /// Calculate cost using real pricing APIs
    ///
    /// This method NO LONGER uses hardcoded rates.
    /// Returns an error directing users to use `PricingFetcher` or `FaasPricingFetcher`.
    pub fn calculate_cost(
        &self,
        _spec: &ResourceSpec,
        _provider: CloudProvider,
        _duration_hours: f64,
    ) -> Result<CostReport> {
        Err(crate::core::error::Error::ConfigurationError(
            "PricingService with hardcoded rates has been removed. \
            Use PricingFetcher::new() for real VM pricing or \
            FaasPricingFetcher::new() for serverless pricing."
                .to_string(),
        ))
    }
}

impl Default for PricingService {
    fn default() -> Self {
        Self::new()
    }
}
