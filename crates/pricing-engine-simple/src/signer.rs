// src/signer.rs
use crate::config::OperatorConfig;
use crate::error::{PricingError, Result};
use log::{info, warn};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

pub type BlueprintHashBytes = [u8; 32]; // Assuming SHA256 hash

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuotePayload {
    pub blueprint_hash: BlueprintHashBytes,
    pub price_wei: u128,
    /// Expiry timestamp (Unix epoch seconds) or block number
    pub expiry: u64,
    /// Timestamp when the quote was generated (Unix epoch seconds)
    pub timestamp: u64,
}

impl QuotePayload {
    /// Serializes the payload deterministically and hashes it.
    pub fn hash(&self) -> Result<[u8; 32]> {
        // Use bincode for stable serialization before hashing
        let serialized = bincode::serialize(self)?;
        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        Ok(hasher.finalize().into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedQuote {
    pub payload: QuotePayload,
    pub signature: Vec<u8>,     // Store signature bytes
    pub signer_pubkey: Vec<u8>, // Include public key for easier verification
}

pub struct OperatorSigner<K: KeyType> {
    keypair: K::Secret,
}

impl OperatorSigner {
    /// Loads a keypair from a file or generates a new one if it doesn't exist.
    pub fn new(config: &OperatorConfig) -> Result<Self> {
        let path = &config.keypair_path;
        let keypair = if path.exists() {
            info!("Loading existing keypair from: {:?}", path);
            let bytes = fs::read(path)?;
            SigningKey::from_bytes(&bytes.try_into().map_err(|_| {
                PricingError::Initialization("Invalid keypair file size".to_string())
            })?)
        } else {
            warn!(
                "Keypair file not found at {:?}. Generating new keypair.",
                path
            );
            let mut csprng = OsRng;
            let new_keypair = SigningKey::generate(&mut csprng);
            fs::write(path, new_keypair.to_bytes())?;
            info!("Saved new keypair to: {:?}", path);
            new_keypair
        };
        Ok(OperatorSigner { keypair })
    }

    /// Signs a quote payload.
    pub fn sign_quote(&self, payload: QuotePayload) -> Result<SignedQuote> {
        let message_hash = payload.hash()?;
        let signature: Signature = self.keypair.sign(&message_hash);
        let pubkey_bytes = self.keypair.verifying_key().to_bytes().to_vec();

        Ok(SignedQuote {
            payload,
            signature: signature.to_bytes().to_vec(),
            signer_pubkey: pubkey_bytes,
        })
    }

    /// Returns the public key associated with the signer.
    pub fn public_key(&self) -> VerifyingKey {
        self.keypair.verifying_key()
    }
}

// Optional: Verification function (might live elsewhere, e.g., client-side or contract)
pub fn verify_quote(quote: &SignedQuote) -> Result<bool> {
    let pubkey = VerifyingKey::from_bytes(
        &quote
            .signer_pubkey
            .clone()
            .try_into()
            .map_err(|_| PricingError::Signing("Invalid public key length".into()))?,
    )?;
    let signature = Signature::from_bytes(
        &quote
            .signature
            .clone()
            .try_into()
            .map_err(|_| PricingError::Signing("Invalid signature length".into()))?,
    )?;
    let message_hash = quote.payload.hash()?;

    Ok(pubkey.verify_strict(&message_hash, &signature).is_ok())
}
