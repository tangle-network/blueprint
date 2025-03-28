//! Error types for the pricing engine
//!
//! This module defines the error types used throughout the pricing engine.

#[cfg(feature = "std")]
use {
    serde::{Deserialize, Serialize},
    thiserror::Error,
};

use jsonrpsee::{core::RegisterMethodError, types::ErrorObjectOwned};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

/// Result type for operations in the pricing engine
pub type Result<T> = std::result::Result<T, Error>;

/// Pricing engine errors
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Error))]
pub enum Error {
    /// Pricing calculation error
    #[cfg_attr(feature = "std", error("Pricing calculation error: {0}"))]
    Calculation(String),

    /// Service initialization error
    #[cfg_attr(feature = "std", error("Service initialization error: {0}"))]
    ServiceInitialization(String),

    /// Service shutdown error
    #[cfg_attr(feature = "std", error("Service shutdown error: {0}"))]
    ServiceShutdown(String),

    /// Chain connection error
    #[cfg_attr(feature = "std", error("Chain connection error: {0}"))]
    ChainConnection(String),

    /// RPC error
    #[cfg_attr(feature = "std", error("RPC error: {0}"))]
    Rpc(#[from] ErrorObjectOwned),

    /// RPC client error
    #[cfg_attr(feature = "std", error("RPC client error: {0}"))]
    RpcClient(String),

    /// Register method error
    #[cfg_attr(feature = "std", error("Register method error: {0}"))]
    RegisterMethod(#[from] RegisterMethodError),

    /// Codec error from parity-scale-codec
    #[cfg_attr(feature = "std", error("Codec error: {0}"))]
    Codec(#[from] parity_scale_codec::Error),

    /// IO error
    #[cfg_attr(feature = "std", error("IO error: {0}"))]
    Io(#[from] std::io::Error),

    /// Other error
    #[cfg_attr(feature = "std", error("{0}"))]
    Other(String),
}

#[cfg(not(feature = "std"))]
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Calculation(msg) => write!(f, "Pricing calculation error: {}", msg),
            Error::ServiceInitialization(msg) => write!(f, "Service initialization error: {}", msg),
            Error::ServiceShutdown(msg) => write!(f, "Service shutdown error: {}", msg),
            Error::ChainConnection(msg) => write!(f, "Chain connection error: {}", msg),
            Error::Rpc(msg) => write!(f, "RPC error: {}", msg),
            Error::RpcClient(msg) => write!(f, "RPC client error: {}", msg),
            Error::Codec(msg) => write!(f, "Codec error: {}", msg),
            Error::Io(msg) => write!(f, "IO error: {}", msg),
            Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

/// Pricing-related errors for calculation
#[derive(Debug, Clone, PartialEq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Error))]
pub enum PricingError {
    /// Quantity is below the minimum threshold
    #[cfg_attr(feature = "std", error("Quantity {0} is below minimum threshold"))]
    QuantityBelowMinimum(u128),

    /// Quantity is above the maximum threshold
    #[cfg_attr(feature = "std", error("Quantity {0} is above maximum threshold"))]
    QuantityAboveMaximum(u128),

    /// No pricing model found for the given resource
    #[cfg_attr(feature = "std", error("No pricing model found for resource"))]
    NoPricingModelForResource,

    /// Unsupported resource unit
    #[cfg_attr(feature = "std", error("Unsupported resource unit: {0}"))]
    UnsupportedResourceUnit(String),

    /// Invalid pricing model configuration
    #[cfg_attr(feature = "std", error("Invalid pricing model configuration: {0}"))]
    InvalidModelConfiguration(String),

    /// Token mismatch when combining prices
    #[cfg_attr(feature = "std", error("Token mismatch: {0} vs {1}"))]
    TokenMismatch(String, String),

    /// General calculation error
    #[cfg_attr(feature = "std", error("Pricing calculation error: {0}"))]
    CalculationError(String),

    /// Missing required resources for calculation
    #[cfg_attr(feature = "std", error("Missing required resources for calculation"))]
    MissingResources,
}

impl From<PricingError> for Error {
    fn from(e: PricingError) -> Self {
        Error::Calculation(e.to_string())
    }
}

#[cfg(not(feature = "std"))]
impl core::fmt::Display for PricingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PricingError::QuantityBelowMinimum(val) => {
                write!(f, "Quantity {} is below minimum threshold", val)
            }
            PricingError::QuantityAboveMaximum(val) => {
                write!(f, "Quantity {} is above maximum threshold", val)
            }
            PricingError::NoPricingModelForResource => {
                write!(f, "No pricing model found for resource")
            }
            PricingError::UnsupportedResourceUnit(unit) => {
                write!(f, "Unsupported resource unit: {}", unit)
            }
            PricingError::InvalidModelConfiguration(reason) => {
                write!(f, "Invalid pricing model configuration: {}", reason)
            }
            PricingError::TokenMismatch(t1, t2) => {
                write!(f, "Token mismatch: {} vs {}", t1, t2)
            }
            PricingError::CalculationError(reason) => {
                write!(f, "Pricing calculation error: {}", reason)
            }
            PricingError::MissingResources => {
                write!(f, "Missing required resources for calculation")
            }
        }
    }
}
