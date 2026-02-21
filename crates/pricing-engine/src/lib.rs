//! Tangle Pricing Engine
//!
//! A flexible pricing system for Tangle Blueprints.
//! The pricing engine calculates costs for service deployments based on
//! resource requirements and provider pricing models, supporting
//! competitive bidding in a decentralized marketplace.

pub mod app;
pub mod benchmark;
pub mod benchmark_cache;
pub mod cloud;
pub mod config;
pub mod error;
pub mod handlers;
pub mod pow;
pub mod pricing;
pub mod service;
pub mod signer;
pub mod types;
pub mod utils;

pub mod pricing_engine {
    include!(concat!(env!("OUT_DIR"), "/pricing_engine.rs"));
}

#[cfg(test)]
pub mod tests;

pub use app::{
    cleanup, init_operator_signer, load_operator_config, spawn_event_processor,
    start_blockchain_listener, wait_for_shutdown,
};
pub use benchmark::cpu::CpuBenchmarkResult;
pub use benchmark::{BenchmarkProfile, BenchmarkRunConfig, run_benchmark, run_benchmark_suite};
pub use benchmark_cache::BenchmarkCache;
pub use benchmark_cache::BlueprintId;
pub use cloud::faas::{FaasPricing, FaasPricingFetcher};
pub use cloud::vm::{InstanceInfo, PricingFetcher};
pub use config::{OperatorConfig, load_config_from_path};
pub use error::{PricingError, Result};
pub use handlers::handle_blueprint_update;
pub use pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof, verify_proof};
pub use pricing::{
    PriceModel, ResourcePricing, SubscriptionPricing, calculate_price, load_job_pricing_from_toml,
    load_pricing_from_toml, load_subscription_pricing_from_toml,
};
pub use service::blockchain::event::BlockchainEvent;
pub use service::rpc::server::{
    JobPricingConfig, PricingEngineService, SubscriptionPricingConfig, run_rpc_server,
};
pub use signer::{OperatorId, OperatorSigner, SignableQuote, SignedJobQuote, SignedQuote};

use blueprint_core::info;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn init_benchmark_cache(config: &OperatorConfig) -> Result<Arc<BenchmarkCache>> {
    let cache_path = format!("{}/benchmark_cache", config.database_path);
    let cache = BenchmarkCache::new(cache_path)?;
    info!("Benchmark cache initialized");
    Ok(Arc::new(cache))
}

pub async fn init_pricing_config(
    config_path: impl AsRef<Path>,
) -> Result<Arc<Mutex<HashMap<Option<u64>, Vec<ResourcePricing>>>>> {
    let content = tokio::fs::read_to_string(config_path.as_ref())
        .await
        .map_err(|e| PricingError::Io(e))?;
    let pricing_config = pricing::load_pricing_from_toml(&content)?;
    info!(
        "Pricing configuration loaded from {}",
        config_path.as_ref().display()
    );
    Ok(Arc::new(Mutex::new(pricing_config)))
}

pub async fn init_job_pricing_config(
    config_path: impl AsRef<Path>,
) -> Result<Arc<Mutex<service::rpc::server::JobPricingConfig>>> {
    let content = tokio::fs::read_to_string(config_path.as_ref())
        .await
        .map_err(|e| PricingError::Io(e))?;
    let job_config = pricing::load_job_pricing_from_toml(&content)?;
    info!(
        "Job pricing configuration loaded from {} ({} entries)",
        config_path.as_ref().display(),
        job_config.len()
    );
    Ok(Arc::new(Mutex::new(job_config)))
}

/// Load subscription pricing config from the same TOML file used for resource pricing.
/// Sections with `pricing_model = "subscription"` are extracted.
pub fn init_subscription_pricing_config(
    config_path: impl AsRef<Path>,
) -> Result<SubscriptionPricingConfig> {
    let content = fs::read_to_string(config_path.as_ref())?;
    let config = pricing::load_subscription_pricing_from_toml(&content)?;
    if config.is_empty() {
        info!(
            "No subscription pricing sections in {}; subscription/event requests will be rejected",
            config_path.as_ref().display()
        );
    } else {
        info!(
            "Subscription pricing loaded from {} ({} entries)",
            config_path.as_ref().display(),
            config.len()
        );
    }
    Ok(config)
}
