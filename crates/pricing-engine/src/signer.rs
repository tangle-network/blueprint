use crate::config::OperatorConfig;
use crate::error::{PricingError, Result};
use crate::pricing::ResourcePricing;
use blueprint_crypto::KeyType;
use serde::{Deserialize, Serialize};

pub type BlueprintId = u64;
pub type OperatorId = [u8; 32];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuotePayload {
    pub blueprint_id: BlueprintId,
    pub ttl_blocks: u64,
    pub total_cost_rate: f64,
    pub resources: Vec<ResourcePricing>,
    /// Expiry timestamp (Unix epoch seconds)
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
    pub operator_id: OperatorId,
    pub proof_of_work: Vec<u8>,
}

pub struct OperatorSigner<K: KeyType> {
    keypair: K::Secret,
    operator_id: OperatorId,
}

impl<K: KeyType> OperatorSigner<K> {
    /// Loads a keypair from a file or generates a new one if it doesn't exist.
    pub fn new(
        _config: &OperatorConfig,
        keypair: K::Secret,
        operator_id: OperatorId,
    ) -> Result<Self> {
        Ok(OperatorSigner {
            keypair,
            operator_id,
        })
    }

    /// Signs a quote payload.
    pub fn sign_quote(
        &mut self,
        payload: QuotePayload,
        proof_of_work: Vec<u8>,
    ) -> Result<SignedQuote<K>> {
        let msg = payload.to_bytes()?;
        let signature = K::sign_with_secret(&mut self.keypair, &msg)
            .map_err(|e| PricingError::Signing(format!("Error signing quote: {:?}", msg)))?;

        Ok(SignedQuote {
            payload,
            signature,
            operator_id: self.operator_id,
            proof_of_work,
        })
    }

    /// Returns the public key associated with the signer.
    pub fn public_key(&self) -> K::Public {
        K::public_from_secret(&self.keypair)
    }

    /// Returns the operator ID
    pub fn operator_id(&self) -> OperatorId {
        self.operator_id
    }
}

pub fn verify_quote<K: KeyType>(quote: &SignedQuote<K>, public_key: &K::Public) -> Result<bool> {
    let msg = quote.payload.to_bytes()?;
    Ok(K::verify(public_key, &msg, &quote.signature))
}
