use super::bn254::Bn254Backend;
use super::ecdsa::EcdsaBackend;
use crate::Result;
use blueprint_crypto::k256::{K256Signature, K256SigningKey, K256VerifyingKey};

pub trait EigenlayerBackend: Bn254Backend + EcdsaBackend {
    /// Generate a new ECDSA key pair from seed.
    fn ecdsa_generate_new(&self, seed: Option<&[u8]>) -> Result<K256VerifyingKey> {
        EcdsaBackend::ecdsa_generate_new(self, seed)
    }

    /// Generate an ECDSA key pair from a string seed.
    fn ecdsa_generate_from_string(&self, secret: &str) -> Result<K256VerifyingKey> {
        EcdsaBackend::ecdsa_generate_from_string(self, secret)
    }

    /// Sign a message using ECDSA key.
    fn ecdsa_sign(&self, public: &K256VerifyingKey, msg: &[u8]) -> Result<K256Signature> {
        EcdsaBackend::ecdsa_sign(self, public, msg)
    }

    /// Get the secret key for an ECDSA public key.
    fn expose_ecdsa_secret(&self, public: &K256VerifyingKey) -> Result<Option<K256SigningKey>> {
        EcdsaBackend::expose_ecdsa_secret(self, public)
    }

    /// Iterate over all ECDSA public keys.
    fn iter_ecdsa(&self) -> impl Iterator<Item = K256VerifyingKey> + '_ {
        EcdsaBackend::iter_ecdsa(self)
    }
}

impl<T> EigenlayerBackend for T where T: Bn254Backend + EcdsaBackend {}
