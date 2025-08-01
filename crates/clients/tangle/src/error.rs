use blueprint_std::io;
use blueprint_std::string::{String, ToString};
use tangle_subxt::subxt;
use tangle_subxt::subxt_core::utils::AccountId32;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Tangle error: {0}")]
    Tangle(TangleDispatchError),
    #[error("Not a Tangle instance")]
    NotTangle,

    #[error("Missing ECDSA key for operator: {0}")]
    MissingEcdsa(AccountId32),
    #[error("Party not found in operator list")]
    PartyNotFound,

    #[error("{0}")]
    Other(String),

    #[error(transparent)]
    Keystore(#[from] blueprint_keystore::Error),
    #[error(transparent)]
    Core(#[from] blueprint_client_core::error::Error),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Subxt error: {0}")]
    Subxt(#[from] subxt::Error),
}

impl From<Error> for blueprint_client_core::error::Error {
    fn from(value: Error) -> Self {
        blueprint_client_core::error::Error::Tangle(value.to_string())
    }
}

pub type Result<T> = blueprint_std::result::Result<T, Error>;

#[derive(Debug)]
pub struct TangleDispatchError(
    pub tangle_subxt::tangle_testnet_runtime::api::runtime_types::sp_runtime::DispatchError,
);

impl From<tangle_subxt::tangle_testnet_runtime::api::runtime_types::sp_runtime::DispatchError>
    for TangleDispatchError
{
    fn from(
        error: tangle_subxt::tangle_testnet_runtime::api::runtime_types::sp_runtime::DispatchError,
    ) -> Self {
        TangleDispatchError(error)
    }
}

impl From<TangleDispatchError> for Error {
    fn from(error: TangleDispatchError) -> Self {
        Error::Tangle(error)
    }
}

impl blueprint_std::fmt::Display for TangleDispatchError {
    fn fmt(&self, f: &mut blueprint_std::fmt::Formatter<'_>) -> blueprint_std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
