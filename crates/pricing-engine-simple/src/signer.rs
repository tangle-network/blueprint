// src/signer.rs
use crate::config::OperatorConfig;
use crate::error::{PricingError, Result};
use blueprint_crypto::KeyType;
use serde::{Deserialize, Serialize};

pub type BlueprintId = u64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuotePayload {
    pub blueprint_id: BlueprintId,
    pub price_wei: u128,
    /// Expiry timestamp (Unix epoch seconds) or block number
    pub expiry: u64,
    /// Timestamp when the quote was generated (Unix epoch seconds)
    pub timestamp: u64,
}

impl QuotePayload {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(PricingError::Serialization)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedQuote<K: KeyType> {
    pub payload: QuotePayload,
    pub signature: K::Signature,
    pub signer_pubkey: K::Public,
}

pub struct OperatorSigner<K: KeyType> {
    keypair: K::Secret,
}

impl<K: KeyType> OperatorSigner<K> {
    /// Loads a keypair from a file or generates a new one if it doesn't exist.
    pub fn new(_config: &OperatorConfig, keypair: K::Secret) -> Result<Self> {
        Ok(OperatorSigner { keypair })
    }

    /// Signs a quote payload.
    pub fn sign_quote(&mut self, payload: QuotePayload) -> Result<SignedQuote<K>> {
        let msg = payload.to_bytes()?;
        let signature = K::sign_with_secret(&mut self.keypair, &msg)
            .map_err(|e| PricingError::Signing(format!("{:?}", e)))?;

        Ok(SignedQuote {
            payload,
            signature,
            signer_pubkey: self.public_key(),
        })
    }

    /// Returns the public key associated with the signer.
    pub fn public_key(&self) -> K::Public {
        K::public_from_secret(&self.keypair)
    }
}

pub fn verify_quote<K: KeyType>(quote: &SignedQuote<K>) -> Result<bool> {
    let msg = quote.payload.to_bytes()?;
    Ok(K::verify(&quote.signer_pubkey, &msg, &quote.signature))
}
