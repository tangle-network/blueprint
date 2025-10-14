//! Pricing and cost estimation
//!
//! This module provides pricing and cost estimation for cloud deployments.
//! Real pricing APIs (FaaS and VM) are now provided by blueprint-pricing-engine.

pub mod cost;
pub mod integration;
pub mod public;
pub mod service;

// Re-export from pricing-engine (single source of truth)
pub use blueprint_pricing_engine_lib::{FaasPricing, FaasPricingFetcher, InstanceInfo, PricingFetcher};

pub use cost::{CostEstimator, CostReport};
pub use integration::PricingCalculator;
pub use service::{CostReport as ServiceCostReport, PricingService};

// Deprecated: Old pricing modules are now in pricing-engine
// These files can be removed in a future version:
// - faas_pricing.rs (use blueprint_pricing_engine_lib::FaasPricingFetcher)
// - fetcher.rs (use blueprint_pricing_engine_lib::PricingFetcher)
