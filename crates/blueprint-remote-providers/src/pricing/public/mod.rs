//! Public pricing sources (no authentication required)
//!
//! Best sources by provider:
//! - AWS: Vantage.sh or EC2Instances.info  
//! - Azure: Official API (prices.azure.com) or Vantage.sh
//! - GCP: Pricing Calculator or hardcoded (NO Vantage support)
//! - DigitalOcean: HTML scraping or hardcoded
//! - Vultr: Hardcoded from pricing page

pub mod aws;
pub mod azure;
pub mod digitalocean;
pub mod gcp;
pub mod vantage;
pub mod vultr;

pub use aws::AwsPublicPricing;
pub use azure::AzurePublicPricing;
pub use digitalocean::DigitalOceanPublicPricing;
pub use gcp::GcpPublicPricing;
pub use vantage::VantageAggregator;
pub use vultr::VultrPublicPricing;
