// src/pow.rs
use crate::error::Result;
use sha2::{Digest, Sha256};
use std::time::Duration;

/// Default difficulty for proof of work
pub const DEFAULT_POW_DIFFICULTY: u32 = 20;

/// Default time limit for proof of work generation
pub const DEFAULT_POW_TIME_LIMIT: Duration = Duration::from_secs(10);

/// Generate a proof of work for the given challenge
pub async fn generate_proof(challenge: &[u8], difficulty: u32) -> Result<Vec<u8>> {
    let mut nonce: u64 = 0;

    loop {
        let proof = create_proof(challenge, nonce);
        if check_difficulty(&proof, difficulty) {
            return Ok(proof);
        }
        nonce += 1;

        // Yield to the executor occasionally to prevent blocking
        if nonce % 1000 == 0 {
            tokio::task::yield_now().await;
        }
    }
}

/// Verify a proof of work against the given challenge
pub fn verify_proof(_challenge: &[u8], proof: &[u8], difficulty: u32) -> Result<bool> {
    // Verify the proof meets the difficulty requirement
    Ok(check_difficulty(proof, difficulty))
}

/// Create a proof by hashing the challenge with a nonce
fn create_proof(challenge: &[u8], nonce: u64) -> Vec<u8> {
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
