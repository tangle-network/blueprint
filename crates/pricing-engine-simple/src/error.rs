// src/error.rs
use crate::types::ParseResourceUnitError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PricingError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),

    #[error("Signing error: {0}")]
    Signing(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Benchmark error: {0}")]
    Benchmark(String), // Keep simple for now

    #[error("Pricing calculation error: {0}")]
    Pricing(String),

    #[error("Invalid blueprint ID: {0}")]
    InvalidBlueprintId(String),

    #[error("Price not found for blueprint: {0}")]
    PriceNotFound(String),

    #[error("Blockchain interaction error: {0}")]
    Blockchain(String), // Placeholder for specific blockchain errors

    #[error("Initialization error: {0}")]
    Initialization(String),

    #[error("Hex decoding error: {0}")]
    HexDecode(String),

    #[error("Proof of work error: {0}")]
    ProofOfWork(String),

    #[error("Invalid proof of work")]
    InvalidProofOfWork,

    #[error("Resource requirement error: {0}")]
    ResourceRequirement(String),

    #[error("TOML parsing error: {0}")]
    TomlParsing(String),

    #[error("Resource unit parsing error")]
    ResourceUnitParsing,

    #[error("Other error: {0}")]
    Other(String),
}

// Implement From<blueprint_keystore::Error> for PricingError
impl From<blueprint_keystore::Error> for PricingError {
    fn from(err: blueprint_keystore::Error) -> Self {
        PricingError::Signing(format!("Keystore error: {:?}", err))
    }
}

// Implement From<toml::de::Error> for PricingError
impl From<toml::de::Error> for PricingError {
    fn from(err: toml::de::Error) -> Self {
        PricingError::TomlParsing(format!("TOML parsing error: {}", err))
    }
}

// Implement From<ParseResourceUnitError> for PricingError
impl From<ParseResourceUnitError> for PricingError {
    fn from(_: ParseResourceUnitError) -> Self {
        PricingError::ResourceUnitParsing
    }
}

pub type Result<T> = std::result::Result<T, PricingError>;
