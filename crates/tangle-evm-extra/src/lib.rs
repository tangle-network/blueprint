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

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod aggregating_consumer;
pub mod aggregation;
pub mod consumer;
pub mod extract;
pub mod layers;
pub mod producer;

pub use aggregating_consumer::AggregatingConsumer;
#[cfg(feature = "aggregation")]
pub use aggregating_consumer::AggregationServiceConfig;
pub use aggregation::{AggregatedResult, AggregationError, G1Point, G2Point, SignerBitmap};
pub use consumer::TangleEvmConsumer;
pub use layers::TangleEvmLayer;
pub use producer::TangleEvmProducer;
