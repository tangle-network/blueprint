use super::keystore;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // General Errors
    #[error("Client error: {0}")]
    Client(#[from] gadget_clients::Error),
    #[error("Keystore error: {0}")]
    Keystore(#[from] keystore::Error),
    #[error("Runner error: {0}")]
    Runner(#[from] blueprint_runner::error::RunnerError),
    #[error("Config error: {0}")]
    Config(#[from] blueprint_runner::error::ConfigError),
    #[error("Other Error: {0}")]
    Other(String),

    // Specific to Tangle
    #[cfg(feature = "tangle")]
    #[error("Tangle Subxt error: {0}")]
    TangleSubxt(#[from] tangle_subxt::subxt::Error),

    // EVM and EigenLayer
    #[cfg(any(feature = "evm", feature = "eigenlayer"))]
    #[error("EVM error: {0}")]
    Alloy(#[from] AlloyError),
    #[cfg(feature = "eigenlayer")]
    #[error("Eigenlayer error: {0}")]
    Eigenlayer(#[from] eigensdk::types::avs::SignatureVerificationError),

    // Specific to Networking
    #[cfg(feature = "networking")]
    #[error("Networking error: {0}")]
    Networking(#[from] gadget_networking::error::Error),

    #[cfg(any(feature = "local-store"))]
    #[error("Database error: {0}")]
    Stores(#[from] gadget_stores::Error),
}

#[cfg(feature = "local-store")]
impl From<gadget_stores::local_database::Error> for Error {
    fn from(value: gadget_stores::local_database::Error) -> Self {
        Error::Stores(value.into())
    }
}

#[cfg(any(feature = "evm", feature = "eigenlayer"))]
#[derive(thiserror::Error, Debug)]
pub enum AlloyError {
    #[error("Alloy signer error: {0}")]
    Signer(#[from] alloy::signers::Error),
    #[error("Alloy contract error: {0}")]
    Contract(#[from] alloy::contract::Error),
    #[error("Alloy transaction error: {0}")]
    Conversion(#[from] alloy::rpc::types::transaction::ConversionError),
    #[error("Alloy local signer error: {0}")]
    LocalSigner(#[from] alloy::signers::local::LocalSignerError),
}

// Two-layer Client conversions
macro_rules! implement_client_error {
    ($feature:literal, $client_type:path) => {
        #[cfg(feature = $feature)]
        impl From<$client_type> for Error {
            fn from(value: $client_type) -> Self {
                Error::Client(value.into())
            }
        }
    };
}
implement_client_error!("eigenlayer", gadget_clients::eigenlayer::error::Error);
implement_client_error!("evm", gadget_clients::evm::error::Error);
implement_client_error!("tangle", gadget_clients::tangle::error::Error);

#[cfg(any(feature = "evm", feature = "eigenlayer"))]
macro_rules! implement_from_alloy_error {
    ($($path:ident)::+, $variant:ident) => {
        impl From<alloy::$($path)::+> for Error {
            fn from(value: alloy::$($path)::+) -> Self {
                Error::Alloy(AlloyError::$variant(value))
            }
        }
    };
}
#[cfg(any(feature = "evm", feature = "eigenlayer"))]
implement_from_alloy_error!(signers::Error, Signer);
#[cfg(any(feature = "evm", feature = "eigenlayer"))]
implement_from_alloy_error!(contract::Error, Contract);
#[cfg(any(feature = "evm", feature = "eigenlayer"))]
implement_from_alloy_error!(rpc::types::transaction::ConversionError, Conversion);
#[cfg(any(feature = "evm", feature = "eigenlayer"))]
implement_from_alloy_error!(signers::local::LocalSignerError, LocalSigner);
