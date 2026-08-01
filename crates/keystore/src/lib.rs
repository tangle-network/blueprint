#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

macro_rules! cfg_remote {
    ($($item:item)*) => {
        $(
        #[cfg(any(
            feature = "aws-signer",
            feature = "gcp-signer",
            feature = "ledger-browser",
            feature = "ledger-node"
        ))]
        $item
        )*
    };
}

// Re-exported for the remote-backend modules; whether any call site is compiled
// depends on the feature combination, so this is unused in some builds.
#[allow(
    unused_imports,
    reason = "macro is consumed only under the remote/ledger feature combinations"
)]
pub(crate) use cfg_remote;

pub mod error;
pub use error::*;
mod keystore;
pub use keystore::*;

pub use blueprint_crypto as crypto;

cfg_remote! {
    pub mod remote;
}

pub mod storage;
