use crate::config::OperatorConfig;
use crate::error::{PricingError, Result};
use crate::pricing_engine;
use bincode;
use blueprint_crypto::KeyType;
use prost::Message;
use tiny_keccak::{Hasher, Keccak};

pub type BlueprintId = u64;
pub type OperatorId = [u8; 32];

#[derive(Debug, Clone)]
pub struct SignedQuote<K: KeyType> {
    pub quote_details: pricing_engine::QuoteDetails,
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

    /// Signs a quote by hashing the proto message and signing the hash.
    pub fn sign_quote(
        &mut self,
        quote_details: pricing_engine::QuoteDetails,
        proof_of_work: Vec<u8>,
    ) -> Result<SignedQuote<K>> {
        // Hash the quote details
        let hash = hash_quote_details(&quote_details)?;

        // Sign the hash
        let signature = K::sign_with_secret(&mut self.keypair, &hash)
            .map_err(|e| PricingError::Signing(format!("Error {:?} signing quote hash", e)))?;

        Ok(SignedQuote {
            quote_details,
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

/// Creates a deterministic hash of the quote details that can be reproduced in any language.
/// Uses protobuf serialization followed by keccak256 hashing for on-chain verification.
pub fn hash_quote_details(quote_details: &pricing_engine::QuoteDetails) -> Result<Vec<u8>> {
    // Serialize the quote details using protobuf
    let mut serialized = Vec::new();
    quote_details.encode(&mut serialized).map_err(|e| {
        PricingError::Serialization(Box::new(bincode::ErrorKind::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to encode protobuf: {:?}", e),
        ))))
    })?;

    // Hash the serialized bytes using keccak256
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(&serialized);
    hasher.finalize(&mut output);

    Ok(output.to_vec())
}

/// Verify a quote signature by checking the signature against the hash of the quote details.
pub fn verify_quote<K: KeyType>(quote: &SignedQuote<K>, public_key: &K::Public) -> Result<bool> {
    let hash = hash_quote_details(&quote.quote_details)?;
    Ok(K::verify(public_key, &hash, &quote.signature))
}
