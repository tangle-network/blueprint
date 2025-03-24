//! Type definitions for blockchain integration
//!
//! This module provides type definitions for working with the Tangle Network
//! blockchain using the tangle-subxt crate.

use std::sync::Arc;

// Import tangle-subxt directly
use tangle_subxt as tangle;

/// Re-export subxt types for convenience
pub use tangle::subxt::{
    Error as SubxtError, OnlineClient, PolkadotConfig,
    backend::rpc::{RpcClient, RpcClientT},
    config::DefaultExtrinsicParamsBuilder,
    tx::Signer,
};

/// Tangle Client type for interacting with the blockchain
pub type TangleClient = Arc<OnlineClient<PolkadotConfig>>;
