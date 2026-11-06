//! Pricing and cost estimation
//!
//! This module provides pricing and cost estimation for cloud deployments.
//! Real pricing APIs (FaaS and VM) are now provided by blueprint-pricing-engine.

pub mod cost;
pub mod integration;
pub mod public;
pub mod service;

// Re-export from pricing-engine (single source of truth)
pub use blueprint_pricing_engine_lib::{
    FaasPricing, FaasPricingFetcher, InstanceInfo, PricingFetcher,
};

pub use cost::{CostEstimator, CostReport};
pub use integration::PricingCalculator;
pub use service::{CostReport as ServiceCostReport, PricingService};

/// Account contract price for one exact allocation, including access and storage.
/// Unknown public rates remain `None`; callers must not substitute a GPU-only rate.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DatedSkuPrice {
    pub sku: String,
    pub region: String,
    pub gpu_count: u32,
    pub cpu: f32,
    pub memory_gb: f32,
    pub storage_gb: f32,
    pub hourly_usd: Option<f64>,
    pub as_of: String,
    pub source_url: String,
    pub note: String,
}

impl DatedSkuPrice {
    #[cfg(feature = "kubernetes")]
    pub(crate) fn price_for(
        &self,
        sku: &str,
        region: &str,
        spec: &crate::ResourceSpec,
    ) -> Option<f64> {
        let date = chrono::NaiveDate::parse_from_str(&self.as_of, "%Y-%m-%d").ok()?;
        let source = url::Url::parse(&self.source_url).ok()?;
        let price = self.hourly_usd?;
        (self.sku == sku
            && self.region == region
            && self.gpu_count == spec.gpu_count.unwrap_or(0)
            && self.cpu == spec.cpu
            && self.memory_gb == spec.memory_gb
            && self.storage_gb == spec.storage_gb
            && date <= chrono::Utc::now().date_naive()
            && source.scheme() == "https"
            && source.host_str().is_some()
            && source.username().is_empty()
            && source.password().is_none()
            && price.is_finite()
            && price > 0.0)
            .then_some(price)
    }
}

/// Dated source evidence for providers with account-specific prices and no quote API.
pub const DATED_GPU_RATES_JSON: &str = include_str!("dated-rates.json");

pub fn dated_gpu_rates() -> Vec<DatedSkuPrice> {
    let book: serde_json::Value =
        serde_json::from_str(DATED_GPU_RATES_JSON).expect("checked GPU rate book");
    serde_json::from_value(book["coreweave"].clone()).expect("checked CoreWeave price evidence")
}
