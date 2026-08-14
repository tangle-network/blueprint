use blueprint_std::string::String;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Bn254Error {
    #[error("Invalid seed: {0}")]
    InvalidSeed(String),
    #[error("Invalid signature: {0}")]
    SignatureFailed(String),
    #[error("Signature not in subgroup")]
    SignatureNotInSubgroup,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Tangle hash-to-curve did not find a point within 256 attempts")]
    HashToPointFailed,
}

pub type Result<T> = blueprint_std::result::Result<T, Bn254Error>;
