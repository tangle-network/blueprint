//! Errors that can occur within the Tangle EVM protocol runner

/// Errors that can occur within the Tangle EVM protocol runner
#[derive(Debug, thiserror::Error)]
pub enum TangleEvmError {
    /// Attempted registration despite not being an active operator
    #[error("Not an active operator")]
    NotActiveOperator,

    /// Transport/RPC error
    #[error("Transport error: {0}")]
    Transport(String),

    /// Contract interaction error
    #[error("Contract error: {0}")]
    Contract(String),

    /// Unable to open/interact with the provided [`Keystore`](blueprint_keystore::Keystore)
    #[error("Keystore error: {0}")]
    Keystore(String),

    /// Unable to open/interact with the provided [`Keystore`](blueprint_keystore::Keystore)
    #[error("Keystore error: {0}")]
    KeystoreError(#[from] blueprint_keystore::Error),

    /// Unable to decompress the provided ECDSA key
    #[error("Unable to convert compressed ECDSA key to uncompressed key")]
    DecompressEcdsaKey,

    /// Not configured for Tangle EVM
    #[error("Not configured for Tangle EVM")]
    NotTangleEvm,

    /// Missing required configuration
    #[error("Missing configuration: {0}")]
    MissingConfig(String),

    /// Transaction failed
    #[error("Transaction error: {0}")]
    Transaction(String),
}
