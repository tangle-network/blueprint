use crate::error::{PricingError, Result};
use bincode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::Duration;

/// Default difficulty for proof of work
pub const DEFAULT_POW_DIFFICULTY: u32 = 20;

/// Default time limit for proof of work generation
pub const DEFAULT_POW_TIME_LIMIT: Duration = Duration::from_secs(10);

/// A proof of work containing both the hash and the nonce used to generate it
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    /// The hash result
    pub hash: Vec<u8>,
    /// The nonce used to generate the hash
    pub nonce: u64,
}

/// Generate a proof of work for the given challenge
pub async fn generate_proof(challenge: &[u8], difficulty: u32) -> Result<Vec<u8>> {
    let mut nonce: u64 = 0;

    loop {
        let hash = calculate_hash(challenge, nonce);
        if check_difficulty(&hash, difficulty) {
            // Return serialized proof containing both hash and nonce
            let proof = Proof { hash, nonce };
            return bincode::serialize(&proof).map_err(PricingError::Serialization);
        }
        nonce += 1;

        // Yield to the executor occasionally to prevent blocking
        if nonce % 1000 == 0 {
            tokio::task::yield_now().await;
        }
    }
}

/// Verify a proof of work against the given challenge
pub fn verify_proof(challenge: &[u8], proof_bytes: &[u8], difficulty: u32) -> Result<bool> {
    // Deserialize the proof
    let proof: Proof = bincode::deserialize(proof_bytes).map_err(PricingError::Serialization)?;

    // First check if the hash meets the difficulty requirement
    if !check_difficulty(&proof.hash, difficulty) {
        return Ok(false);
    }

    // Recalculate the hash using the challenge and nonce
    let recalculated_hash = calculate_hash(challenge, proof.nonce);

    // Verify the recalculated hash matches the one in the proof
    Ok(recalculated_hash == proof.hash)
}

/// Calculate a hash from a challenge and nonce
fn calculate_hash(challenge: &[u8], nonce: u64) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(challenge);
    hasher.update(nonce.to_be_bytes());
    hasher.finalize().to_vec()
}

/// Check if a hash meets the difficulty requirement (has N leading zero bits)
fn check_difficulty(hash: &[u8], difficulty: u32) -> bool {
    // Calculate how many complete bytes and remaining bits we need to check
    let zero_bytes = difficulty / 8;
    let zero_bits = difficulty % 8;

    // Check complete zero bytes
    for i in 0..zero_bytes as usize {
        if i >= hash.len() || hash[i] != 0 {
            return false;
        }
    }

    // Check remaining bits in the next byte
    if zero_bits > 0 {
        let byte_idx = zero_bytes as usize;
        if byte_idx >= hash.len() {
            return false;
        }

        // Create a mask for the remaining bits
        let mask = 0xFF << (8 - zero_bits);
        if (hash[byte_idx] & mask) != 0 {
            return false;
        }
    }

    true
}

/// Generate a challenge from a blueprint ID and timestamp
pub fn generate_challenge(blueprint_id: u64, timestamp: u64) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(blueprint_id.to_be_bytes());
    hasher.update(timestamp.to_be_bytes());
    hasher.finalize().to_vec()
}
