//! Error types for the Tangle EVM client

extern crate alloc;

use alloc::string::{String, ToString};
use alloy_primitives::Address;
use thiserror::Error;

/// Result type alias for Tangle EVM operations
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that can occur when interacting with Tangle EVM contracts
#[derive(Debug, Error)]
pub enum Error {
    /// Transport/RPC error from Alloy
    #[error("Transport error: {0}")]
    Transport(#[from] alloy_transport::TransportError),

    /// Pending transaction error
    #[error("Transaction error: {0}")]
    PendingTransaction(#[from] alloy_provider::PendingTransactionError),

    /// Contract call error
    #[error("Contract error: {0}")]
    Contract(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Keystore error
    #[error("Keystore error: {0}")]
    Keystore(#[from] blueprint_keystore::Error),

    /// Blueprint not found
    #[error("Blueprint {0} not found")]
    BlueprintNotFound(u64),

    /// Service not found
    #[error("Service {0} not found")]
    ServiceNotFound(u64),

    /// Operator not found
    #[error("Operator {0} not found")]
    OperatorNotFound(Address),

    /// Operator not registered for blueprint
    #[error("Operator {0} not registered for blueprint {1}")]
    OperatorNotRegistered(Address, u64),

    /// Missing ECDSA key for operator
    #[error("Missing ECDSA key for operator {0}")]
    MissingEcdsa(Address),

    /// Not configured for Tangle EVM
    #[error("Not configured for Tangle EVM")]
    NotTangleEvm,

    /// Party not found in operators list
    #[error("Current party not found in operators list")]
    PartyNotFound,

    /// Invalid address format
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    /// Serde/serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Provider not initialized
    #[error("Provider not initialized - call connect() first")]
    ProviderNotInitialized,

    /// Client core error
    #[error("Client error: {0}")]
    ClientCore(#[from] blueprint_client_core::error::Error),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}
