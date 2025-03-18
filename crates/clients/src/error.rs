#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] blueprint_client_core::error::Error),
    #[error(transparent)]
    #[cfg(feature = "eigenlayer")]
    Eigenlayer(#[from] blueprint_client_eigenlayer::error::Error),
    #[error(transparent)]
    #[cfg(feature = "evm")]
    Evm(#[from] blueprint_client_evm::error::Error),
    #[error(transparent)]
    #[cfg(feature = "tangle")]
    Tangle(#[from] blueprint_client_tangle::error::Error),
}

impl Error {
    pub fn msg<T: blueprint_std::fmt::Debug>(msg: T) -> Self {
        let err = blueprint_client_core::error::Error::msg(msg);
        Error::Core(err)
    }
}
