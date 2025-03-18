#![cfg_attr(not(feature = "std"), no_std)]

pub mod error;
use error::{Ed25519Error, Result};

#[cfg(test)]
mod tests;

use blueprint_crypto_core::BytesEncoding;
use blueprint_crypto_core::{KeyType, KeyTypeId};
use gadget_std::{
    hash::Hash,
    string::{String, ToString},
    vec::Vec,
};
use serde::{Deserialize, Serialize};

/// Ed25519 key type
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Ed25519Zebra;

macro_rules! impl_zebra_serde {
    ($name:ident, $inner:ty) => {
        #[derive(Clone)]
        pub struct $name(pub $inner);

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.to_bytes() == other.to_bytes()
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<gadget_std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Hash for $name {
            fn hash<H: gadget_std::hash::Hasher>(&self, state: &mut H) {
                self.to_bytes().hash(state);
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> gadget_std::cmp::Ordering {
                self.to_bytes().cmp(&other.to_bytes())
            }
        }

        impl gadget_std::fmt::Debug for $name {
            fn fmt(&self, f: &mut gadget_std::fmt::Formatter<'_>) -> gadget_std::fmt::Result {
                write!(f, "{:?}", self.to_bytes())
            }
        }

        impl BytesEncoding for $name {
            fn to_bytes(&self) -> Vec<u8> {
                self.to_bytes_impl()
            }

            fn from_bytes(bytes: &[u8]) -> core::result::Result<Self, serde::de::value::Error> {
                Self::from_bytes_impl(bytes).map_err(|e| serde::de::Error::custom(e.to_string()))
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                // Get the raw bytes
                let bytes = self.to_bytes();
                Vec::serialize(&bytes, serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                // Deserialize as bytes
                let bytes = <Vec<u8>>::deserialize(deserializer)?;

                // Convert bytes back to inner type
                let inner = <$inner>::try_from(bytes.as_slice())
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;

                Ok($name(inner))
            }
        }
    };
}

impl_zebra_serde!(Ed25519SigningKey, ed25519_zebra::SigningKey);

impl Ed25519SigningKey {
    fn to_bytes_impl(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bytes_impl(bytes: &[u8]) -> Result<Self> {
        let inner = ed25519_zebra::SigningKey::try_from(bytes)
            .map_err(|e| Ed25519Error::InvalidSigner(e.to_string()))?;
        Ok(Ed25519SigningKey(inner))
    }
}

impl_zebra_serde!(Ed25519VerificationKey, ed25519_zebra::VerificationKey);

impl Ed25519VerificationKey {
    fn to_bytes_impl(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bytes_impl(bytes: &[u8]) -> Result<Self> {
        let inner = ed25519_zebra::VerificationKey::try_from(bytes)
            .map_err(|e| Ed25519Error::InvalidVerifyingKey(e.to_string()))?;
        Ok(Ed25519VerificationKey(inner))
    }
}

impl_zebra_serde!(Ed25519Signature, ed25519_zebra::Signature);

impl Ed25519Signature {
    fn to_bytes_impl(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    fn from_bytes_impl(bytes: &[u8]) -> Result<Self> {
        let inner = ed25519_zebra::Signature::try_from(bytes)
            .map_err(|e| Ed25519Error::InvalidSignature(e.to_string()))?;
        Ok(Ed25519Signature(inner))
    }
}

impl KeyType for Ed25519Zebra {
    type Public = Ed25519VerificationKey;
    type Secret = Ed25519SigningKey;
    type Signature = Ed25519Signature;
    type Error = Ed25519Error;

    fn key_type_id() -> KeyTypeId {
        KeyTypeId::Ed25519
    }

    fn generate_with_seed(seed: Option<&[u8]>) -> Result<Self::Secret> {
        if let Some(seed) = seed {
            let mut seed_bytes = [0u8; 32];
            let len = seed.len().min(32);
            seed_bytes[..len].copy_from_slice(&seed[..len]);
            let seed = seed_bytes;
            Ok(Ed25519SigningKey(ed25519_zebra::SigningKey::from(seed)))
        } else {
            let mut rng = Self::get_rng();
            Ok(Ed25519SigningKey(ed25519_zebra::SigningKey::new(&mut rng)))
        }
    }

    fn generate_with_string(secret: String) -> Result<Self::Secret> {
        let hex_encoded = hex::decode(secret)?;
        let secret = ed25519_zebra::SigningKey::try_from(hex_encoded.as_slice())
            .map_err(|e| Ed25519Error::InvalidSeed(e.to_string()))?;
        Ok(Ed25519SigningKey(secret))
    }

    fn public_from_secret(secret: &Self::Secret) -> Self::Public {
        Ed25519VerificationKey((&secret.0).into())
    }

    fn sign_with_secret(secret: &mut Self::Secret, msg: &[u8]) -> Result<Self::Signature> {
        Ok(Ed25519Signature(secret.0.sign(msg)))
    }

    fn sign_with_secret_pre_hashed(
        secret: &mut Self::Secret,
        msg: &[u8; 32],
    ) -> Result<Self::Signature> {
        Ok(Ed25519Signature(secret.0.sign(msg)))
    }

    fn verify(public: &Self::Public, msg: &[u8], signature: &Self::Signature) -> bool {
        public.0.verify(&signature.0, msg).is_ok()
    }
}
