#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod gossip;
pub mod handlers;
pub mod networking;
#[cfg(feature = "round-based-compat")]
pub mod round_based_compat;
#[cfg(feature = "round-based-compat")]
pub use round_based;

pub mod behaviours;
pub mod error;
pub mod setup;
pub mod types;

pub use key_types::*;

#[cfg(all(
    feature = "sp-core-ecdsa",
    not(feature = "sp-core-sr25519"),
    not(feature = "sp-core-ed25519")
))]
pub mod key_types {
    pub use gadget_crypto::sp_core::{
        SpEcdsa as Curve, SpEcdsaPair as GossipMsgKeyPair, SpEcdsaPublic as GossipMsgPublicKey,
        SpEcdsaSignature as GossipSignedMsgSignature,
    };
}

#[cfg(all(
    feature = "sp-core-sr25519",
    not(feature = "sp-core-ecdsa"),
    not(feature = "sp-core-ed25519")
))]
pub mod key_types {
    pub use gadget_crypto::sp_core::{
        SpSr25519 as Curve, SpSr25519Pair as GossipMsgKeyPair,
        SpSr25519Public as GossipMsgPublicKey, SpSr25519Signature as GossipSignedMsgSignature,
    };
}

#[cfg(all(
    feature = "sp-core-ed25519",
    not(feature = "sp-core-ecdsa"),
    not(feature = "sp-core-sr25519")
))]
pub mod key_types {
    pub use gadget_crypto::sp_core::{
        SpEd25519 as Curve, SpEd25519Pair as GossipMsgKeyPair,
        SpEd25519Public as GossipMsgPublicKey, SpEd25519Signature as GossipSignedMsgSignature,
    };
}

#[cfg(all(
    not(feature = "sp-core-ecdsa"),
    not(feature = "sp-core-sr25519"),
    not(feature = "sp-core-ed25519")
))]
pub mod key_types {
    // Default to k256 ECDSA implementation
    pub use gadget_crypto::k256::{
        K256Ecdsa as Curve, K256Signature as GossipSignedMsgSignature,
        K256SigningKey as GossipMsgKeyPair, K256VerifyingKey as GossipMsgPublicKey,
    };
}

// Compile-time assertion to ensure only one feature is enabled
#[cfg(any(
    all(feature = "sp-core-ecdsa", feature = "sp-core-sr25519"),
    all(feature = "sp-core-ecdsa", feature = "sp-core-ed25519"),
    all(feature = "sp-core-sr25519", feature = "sp-core-ed25519")
))]
compile_error!(
    "Only one of 'sp-core-ecdsa', 'sp-core-sr25519', or 'sp-core-ed25519' features can be enabled at a time"
);
