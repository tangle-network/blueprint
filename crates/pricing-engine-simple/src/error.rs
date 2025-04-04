// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PricingError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Cache error: {0}")]
    Cache(#[from] sled::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),

    #[error("Signing error: {0}")]
    Signing(#[from] ed25519_dalek::SignatureError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Benchmark error: {0}")]
    Benchmark(String), // Keep simple for now

    #[error("Pricing calculation error: {0}")]
    Pricing(String),

    #[error("Invalid blueprint hash: {0}")]
    InvalidBlueprintHash(String),

    #[error("Price not found for blueprint: {0}")]
    PriceNotFound(String),

    #[error("Blockchain interaction error: {0}")]
    Blockchain(String), // Placeholder for specific blockchain errors

    #[error("Initialization error: {0}")]
    Initialization(String),

    #[error("Hex decoding error: {0}")]
    HexDecode(#[from] hex::FromHexError),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, PricingError>;
