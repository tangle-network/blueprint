//! Tangle Client for Blueprint SDK
//!
//! This crate provides connectivity to Tangle EVM contracts for blueprint operators.
//!
//! ## Overview
//!
//! The Tangle client allows blueprints to:
//! - Query blueprints, services, and operators from the Tangle contract
//! - Monitor events (job submissions, service lifecycle)
//! - Submit job results
//! - Interact with the restaking system
//!
//! ## Usage
//!
//! ```rust,ignore
//! use blueprint_client_tangle::{TangleClient, TangleEvent};
//! use blueprint_runner::config::BlueprintEnvironment;
//!
//! async fn example(config: BlueprintEnvironment) -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client
//!     let client = TangleClient::new(config).await?;
//!
//!     // Query blueprint
//!     let blueprint = client.get_blueprint(1).await?;
//!     println!("Blueprint owner: {:?}", blueprint.owner);
//!
//!     // Monitor events
//!     while let Some(event) = client.next_event().await {
//!         println!("Block {}: {} logs", event.block_number, event.logs.len());
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Contract Interfaces
//!
//! The crate provides bindings for:
//! - `ITangle` - Core Tangle protocol (blueprints, services, jobs, slashing)
//! - `IMultiAssetDelegation` - Restaking and delegation
//! - `IOperatorStatusRegistry` - Operator heartbeats and status
//!
//! ## Features
//!
//! - `std` (default) - Standard library support
//! - `web` - WebAssembly support

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

#[allow(unused_extern_crates)]
extern crate alloc;

use core::future::Future;

pub mod client;
pub mod config;
#[allow(missing_docs)]
pub mod contracts;
pub mod error;
#[allow(missing_docs)]
pub mod services;

// Re-exports
pub use client::{
    AggregationConfig, AssetInfo, AssetKind, BlueprintSelectionMode, DelegationInfo,
    DelegationMode, DelegationRecord, DepositInfo, EcdsaPublicKey, JobSubmissionResult, LockInfo,
    LockMultiplier, OperatorMetadata, OperatorStatusSnapshot, PendingUnstake, PendingWithdrawal,
    RestakingMetadata, RestakingStatus, TangleClient, TangleEvent, ThresholdType,
    TransactionResult,
};
pub use config::{TangleClientConfig, TangleSettings};
pub use contracts::{
    IBlueprintServiceManager, IMultiAssetDelegation, IOperatorStatusRegistry, ITangle,
};
pub use error::{Error, Result};
pub use services::{
    BlueprintConfig, BlueprintInfo, MembershipModel, OperatorSecurityCommitment, PricingModel,
    ServiceInfo, ServiceRequestInfo, ServiceRequestParams, ServiceStatus,
};

/// Trait for clients that provide events
pub trait EventsClient<E> {
    /// Get the next event
    fn next_event(&self) -> impl Future<Output = Option<E>> + Send;

    /// Get the latest event
    fn latest_event(&self) -> impl Future<Output = Option<E>> + Send;
}

impl EventsClient<TangleEvent> for TangleClient {
    async fn next_event(&self) -> Option<TangleEvent> {
        self.next_event().await
    }

    async fn latest_event(&self) -> Option<TangleEvent> {
        self.latest_event().await
    }
}
