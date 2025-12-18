//! Tangle EVM Extra - Producer/Consumer for Tangle v2 EVM contracts
//!
//! This crate provides the producer/consumer pattern for processing jobs on Tangle v2
//! EVM contracts. It mirrors the functionality of `blueprint-tangle-extra` but uses
//! EVM events and transactions instead of Substrate extrinsics.
//!
//! ## Overview
//!
//! - **Producer**: Polls for `JobSubmitted` events and converts them to `JobCall` streams
//! - **Consumer**: Submits job results via the `submitResult` contract function
//! - **Extractors**: Extract metadata from job calls (call_id, service_id, etc.)
//! - **Keepers**: Background services for lifecycle automation (epoch, round, stream)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use blueprint_tangle_evm_extra::{TangleEvmProducer, TangleEvmConsumer};
//! use blueprint_client_tangle_evm::TangleEvmClient;
//!
//! async fn example(client: TangleEvmClient) {
//!     // Create producer to receive job events
//!     let producer = TangleEvmProducer::new(client.clone(), service_id);
//!
//!     // Create consumer to submit results
//!     let consumer = TangleEvmConsumer::new(client);
//!
//!     // Process jobs with the blueprint runner...
//! }
//! ```
//!
//! ## Keepers (feature: `keepers`)
//!
//! Background services that automate lifecycle operations:
//!
//! ```rust,ignore
//! use blueprint_tangle_evm_extra::services::{
//!     EpochKeeper, RoundKeeper, StreamKeeper, KeeperConfig, BackgroundKeeper,
//! };
//!
//! // Configure keepers
//! let config = KeeperConfig::new(rpc_url, keystore)
//!     .with_inflation_pool(inflation_pool_address)
//!     .with_multi_asset_delegation(mad_address);
//!
//! // Start background services
//! let epoch_handle = EpochKeeper::start(config.clone(), shutdown.subscribe());
//! let round_handle = RoundKeeper::start(config.clone(), shutdown.subscribe());
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod aggregating_consumer;
pub mod aggregation;
pub mod cache;
pub mod consumer;
pub mod extract;
pub mod layers;
pub mod producer;
pub mod strategy;

/// Lifecycle automation services (keepers)
///
/// Requires the `keepers` feature to be enabled.
#[cfg(feature = "keepers")]
pub mod services;

pub use aggregating_consumer::AggregatingConsumer;
#[cfg(feature = "aggregation")]
pub use aggregating_consumer::AggregationServiceConfig;
pub use aggregation::{AggregatedResult, AggregationError, G1Point, G2Point, SignerBitmap};
pub use cache::{
    CacheError, CacheInvalidationEvent, CacheStats, CacheSyncService, DEFAULT_CACHE_TTL,
    OperatorWeights, ServiceConfigCache, ServiceOperators, SharedServiceConfigCache, shared_cache,
    shared_cache_with_ttl,
};
pub use consumer::TangleEvmConsumer;
pub use layers::TangleEvmLayer;
pub use producer::TangleEvmProducer;

// Strategy exports
#[cfg(feature = "aggregation")]
pub use strategy::HttpServiceConfig;
#[cfg(feature = "p2p-aggregation")]
pub use strategy::P2PGossipConfig;
pub use strategy::{AggregatedSignatureResult, AggregationStrategy, StrategyError, ThresholdType};
