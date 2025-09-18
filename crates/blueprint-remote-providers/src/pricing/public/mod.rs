//! Public pricing sources (no authentication required)
//!
//! All cloud providers use the PricingFetcher with live APIs:
//! - AWS: Vantage.sh API (live pricing) 
//! - Azure: Vantage.sh API (live pricing)
//! - GCP: Simplified pricing with regional multipliers
//! - DigitalOcean: Web scraping (live pricing)
//! - Vultr: Hardcoded fallback only

pub mod vantage;
pub mod vultr;

pub use vantage::VantageAggregator;
pub use vultr::VultrPublicPricing;
