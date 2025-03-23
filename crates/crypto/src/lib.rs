#![cfg_attr(not(feature = "std"), no_std)]

use thiserror::Error;

pub use blueprint_crypto_core::*;

#[cfg(feature = "k256")]
pub use blueprint_crypto_k256 as k256;

#[cfg(feature = "sr25519-schnorrkel")]
pub use blueprint_crypto_sr25519 as sr25519;

#[cfg(feature = "ed25519")]
pub use blueprint_crypto_ed25519 as ed25519;

#[cfg(feature = "bls")]
pub use blueprint_crypto_bls as bls;

#[cfg(feature = "bn254")]
pub use blueprint_crypto_bn254 as bn254;

#[cfg(feature = "sp-core")]
pub use blueprint_crypto_sp_core as sp_core;

#[cfg(feature = "tangle-pair-signer")]
pub use blueprint_crypto_tangle_pair_signer as tangle_pair_signer;

#[cfg(feature = "hashing")]
pub use blueprint_crypto_hashing as hashing;

#[derive(Debug, Error)]
pub enum CryptoCoreError {
    #[cfg(feature = "k256")]
    #[error(transparent)]
    K256(#[from] blueprint_crypto_k256::error::K256Error),
    #[cfg(feature = "sr25519-schnorrkel")]
    #[error(transparent)]
    Sr25519(#[from] blueprint_crypto_sr25519::error::Sr25519Error),
    #[cfg(feature = "ed25519")]
    #[error(transparent)]
    Ed25519(#[from] blueprint_crypto_ed25519::error::Ed25519Error),
    #[cfg(feature = "bls")]
    #[error(transparent)]
    Bls(#[from] blueprint_crypto_bls::error::BlsError),
    #[cfg(feature = "bn254")]
    #[error(transparent)]
    Bn254(#[from] blueprint_crypto_bn254::error::Bn254Error),
    #[cfg(feature = "sp-core")]
    #[error(transparent)]
    SpCore(#[from] blueprint_crypto_sp_core::error::SpCoreError),
}

pub trait IntoCryptoError {
    fn into_crypto_error(self) -> CryptoCoreError;
}

#[cfg(feature = "k256")]
impl IntoCryptoError for blueprint_crypto_k256::error::K256Error {
    fn into_crypto_error(self) -> CryptoCoreError {
        CryptoCoreError::K256(self)
    }
}

#[cfg(feature = "sr25519-schnorrkel")]
impl IntoCryptoError for blueprint_crypto_sr25519::error::Sr25519Error {
    fn into_crypto_error(self) -> CryptoCoreError {
        CryptoCoreError::Sr25519(self)
    }
}

#[cfg(feature = "ed25519")]
impl IntoCryptoError for blueprint_crypto_ed25519::error::Ed25519Error {
    fn into_crypto_error(self) -> CryptoCoreError {
        CryptoCoreError::Ed25519(self)
    }
}

#[cfg(feature = "bls")]
impl IntoCryptoError for blueprint_crypto_bls::error::BlsError {
    fn into_crypto_error(self) -> CryptoCoreError {
        CryptoCoreError::Bls(self)
    }
}

#[cfg(feature = "bn254")]
impl IntoCryptoError for blueprint_crypto_bn254::error::Bn254Error {
    fn into_crypto_error(self) -> CryptoCoreError {
        CryptoCoreError::Bn254(self)
    }
}

#[cfg(feature = "sp-core")]
impl IntoCryptoError for blueprint_crypto_sp_core::error::SpCoreError {
    fn into_crypto_error(self) -> CryptoCoreError {
        CryptoCoreError::SpCore(self)
    }
}
