/// Errors that can occur within the Tangle protocol runner
#[derive(Debug, thiserror::Error)]
pub enum TangleError {
    /// Attempted registration despite not being an active operator
    #[error("Not an active operator")]
    NotActiveOperator,

    #[error("Network error: {0}")]
    Network(tangle_subxt::subxt::Error),

    /// Unable to open/interact with the provided [`Keystore`](blueprint_keystore::Keystore)
    #[error("Keystore error: {0}")]
    Keystore(#[from] blueprint_keystore::Error),

    /// Unable to decompress the provided ECDSA key
    #[error("Unable to convert compressed ECDSA key to uncompressed key")]
    DecompressEcdsaKey,
}
