use eigensdk::{
    client_avsregistry::error::AvsRegistryError, client_elcontracts::error::ElContractsError,
};
use thiserror::Error;

/// Errors that can occur within the Eigenlayer protocol runner
#[derive(Debug, Error)]
pub enum EigenlayerError {
    /// Errors from the Eigenlayer `AvsRegistry`
    #[error("AVS Registry error: {0}")]
    AvsRegistry(#[from] AvsRegistryError),

    /// Errors that occur when interacting with contracts
    #[error("Contract error: {0}")]
    Contract(#[from] alloy_contract::Error),

    /// Errors that occur when interacting with Eigenlayer contracts
    #[error("EL Contracts error: {0}")]
    ElContracts(#[from] ElContractsError),

    /// An error occured during operator registration
    #[error("Registration error: {0}")]
    Registration(String),

    /// Unable to open/interact with the provided [`Keystore`](blueprint_keystore::Keystore)
    #[error("Keystore error: {0}")]
    Keystore(#[from] blueprint_keystore::Error),

    /// Errors that occur when interacting with possibly malformed keys
    #[error("Crypto error: {0}")]
    Crypto(#[from] blueprint_crypto::CryptoCoreError),

    /// Unable to sign a message
    #[error("Signature error: {0}")]
    SignatureError(#[from] alloy_signer::Error),

    #[error("Other error: {0}")]
    Other(Box<dyn core::error::Error + Send + Sync>),
}
