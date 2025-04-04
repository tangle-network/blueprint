//! Tangle Cloud Pricing Engine
//!
//! A flexible pricing system for the Tangle Cloud service platform.
//! The pricing engine calculates costs for service deployments based on
//! resource requirements and provider pricing models, supporting
//! competitive bidding in a decentralized marketplace.

// src/lib.rs

// Re-export modules to make them accessible
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
pub use benchmark::{BenchmarkProfile, BenchmarkRunConfig, run_benchmark};
pub use cache::PriceCache;
pub use config::{OperatorConfig, load_config};
pub use error::{PricingError, Result};
pub use handlers::handle_blueprint_update;
pub use pricing::PriceModel;
pub use service::blockchain;
pub use service::{
    blockchain::{event::BlockchainEvent, listener::EventListener},
    rpc::server::{pricing_proto, run_rpc_server},
};
pub use signer::{BlueprintHashBytes, OperatorSigner, QuotePayload, SignedQuote};
