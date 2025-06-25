//! Tangle Pricing Engine
//!
//! A flexible pricing system for Tangle Blueprints.
//! The pricing engine calculates costs for service deployments based on
//! resource requirements and provider pricing models, supporting
//! competitive bidding in a decentralized marketplace.

pub mod app;
pub mod benchmark;
pub mod benchmark_cache;
pub mod cache;
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
pub use cache::{BlueprintId, PriceCache};
pub use config::{OperatorConfig, load_config_from_path};
pub use error::{PricingError, Result};
pub use handlers::handle_blueprint_update;
pub use pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof, verify_proof};
pub use pricing::{PriceModel, ResourcePricing, calculate_price, load_pricing_from_toml};
pub use service::blockchain::event::BlockchainEvent;
pub use service::blockchain::listener::EventListener;
pub use service::rpc::server::{PricingEngineService, run_rpc_server};
pub use signer::{OperatorId, OperatorSigner, SignedQuote};

use blueprint_core::info;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

pub const DEFAULT_CONFIG: &str = include_str!("../config/default_pricing.toml");

pub async fn init_benchmark_cache(config: &OperatorConfig) -> Result<Arc<BenchmarkCache>> {
    let cache_path = format!("{}/benchmark_cache", config.database_path);
    let cache = BenchmarkCache::new(cache_path)?;
    info!("Benchmark cache initialized");
    Ok(Arc::new(cache))
}

pub async fn init_pricing_config(
    config_path: impl AsRef<Path>,
) -> Result<Arc<Mutex<HashMap<Option<u64>, Vec<ResourcePricing>>>>> {
    let content = fs::read_to_string(config_path.as_ref())?;
    let pricing_config = pricing::load_pricing_from_toml(&content)?;
    info!(
        "Pricing configuration loaded from {}",
        config_path.as_ref().display()
    );
    Ok(Arc::new(Mutex::new(pricing_config)))
}
