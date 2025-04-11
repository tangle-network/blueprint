//! Tangle Cloud Pricing Engine
//!
//! A flexible pricing system for the Tangle Cloud service platform.
//! The pricing engine calculates costs for service deployments based on
//! resource requirements and provider pricing models, supporting
//! competitive bidding in a decentralized marketplace.

// src/lib.rs

// Define modules
pub mod app;
pub mod benchmark;
pub mod cache;
pub mod config;
pub mod error;
pub mod handlers;
pub mod pricing;
pub mod service;
pub mod signer;
pub mod types;

// Re-export key types and functions for easier use by the binary or other crates
pub use benchmark::cpu::CpuBenchmarkResult;
pub use benchmark::{BenchmarkProfile, BenchmarkRunConfig, run_benchmark, run_benchmark_suite};
pub use cache::PriceCache;
pub use config::{OperatorConfig, load_config, load_config_from_path};
pub use error::{PricingError, Result};
pub use handlers::handle_blueprint_update;
pub use pricing::{PriceModel, calculate_price};
pub use service::blockchain::event::BlockchainEvent;
pub use service::blockchain::listener::EventListener;
pub use signer::{OperatorSigner, QuotePayload, SignedQuote};

// Re-export application-level functions
pub use app::{
    cleanup, init_logging, init_operator_signer, init_operator_signer_ed25519, init_price_cache,
    load_operator_config, spawn_event_processor, start_blockchain_listener, wait_for_shutdown,
};

#[cfg(test)]
mod tests;
