//! RPC module for the Tangle Cloud Pricing Engine
//!
//! This module provides JSON-RPC server functionality for the pricing engine.

use blueprint_crypto::KeyType;
use serde::{Deserialize, Serialize};

pub mod client;
pub mod server;

// Re-exports
pub use client::RpcClient;
pub use server::ServiceRequestHandler;

/// Operator information returned by the RPC API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "K: KeyType")]
pub struct OperatorInfo<K: KeyType> {
    /// Operator public key
    pub public_key: K::Public,
    /// Operator name
    pub name: String,
    /// Operator description
    pub description: Option<String>,
    /// Supported blueprint IDs
    pub supported_blueprints: Vec<String>,
}
