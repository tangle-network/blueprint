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
    Benchmark(String),

    #[error("Pricing calculation error: {0}")]
    Pricing(String),

    #[error("Invalid blueprint ID: {0}")]
    InvalidBlueprintId(String),

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
    TomlParsing(#[from] toml::de::Error),

    #[error("Resource unit parsing error")]
    ResourceUnitParsing(#[from] ParseResourceUnitError),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<blueprint_keystore::Error> for PricingError {
    fn from(error: blueprint_keystore::Error) -> Self {
        PricingError::Signing(format!("Error signing quote: {:?}", error))
    }
}

pub type Result<T> = std::result::Result<T, PricingError>;
