use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum EigenlayerExtraError {
    #[error("Keystore error: {0}")]
    Keystore(#[from] blueprint_keystore::Error),

    #[error("Contract interaction failed: {0}")]
    Contract(String),

    #[error("Transaction failed: {0}")]
    Transaction(String),

    #[error("Operator not registered")]
    OperatorNotRegistered,

    #[error("No rewards available to claim")]
    NoRewardsAvailable,

    #[error("Operator is slashed")]
    OperatorSlashed,

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("EigenSDK error: {0}")]
    EigenSdk(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Other error: {0}")]
    Other(String),
}

/// Type alias for convenience
pub type Error = EigenlayerExtraError;

pub type Result<T> = std::result::Result<T, EigenlayerExtraError>;
