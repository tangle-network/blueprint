//! BLS Signature Aggregation for Tangle EVM Jobs
//!
//! This module provides types and utilities for submitting aggregated BLS signatures
//! to the Tangle v2 contracts for jobs that require signature aggregation.
//!
//! ## Overview
//!
//! When a blueprint's service manager enables aggregation for a job, operators must
//! collectively sign the job result using BLS signatures. The signatures are then
//! aggregated and submitted to the contract for verification.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use blueprint_tangle_evm_extra::aggregation::{AggregatedResult, SignerBitmap};
//!
//! // Create an aggregated result from signatures
//! let result = AggregatedResult {
//!     service_id: 1,
//!     call_id: 42,
//!     output: output_bytes,
//!     signer_bitmap: SignerBitmap::from_indices(&[0, 1, 3]),
//!     signature: aggregated_sig,
//!     pubkey: aggregated_pubkey,
//! };
//!
//! // Submit to contract
//! result.submit(&client).await?;
//! ```

use alloy_primitives::{Bytes, U256};
use blueprint_client_tangle_evm::TangleEvmClient;
use std::sync::Arc;
use thiserror::Error;

/// Error types for aggregation operations
#[derive(Debug, Error)]
pub enum AggregationError {
    /// Client error
    #[error("Client error: {0}")]
    Client(String),
    /// Not enough signers
    #[error("Threshold not met: got {0} signers, need {1}")]
    ThresholdNotMet(usize, usize),
    /// Invalid signature
    #[error("Invalid BLS signature")]
    InvalidSignature,
    /// Contract call failed
    #[error("Contract call failed: {0}")]
    ContractError(String),
    /// Missing operator key
    #[error("Missing BLS key for operator at index {0}")]
    MissingOperatorKey(usize),
}

/// Bitmap indicating which operators signed
///
/// Bit i is set if operator i (in service operator list order) signed.
#[derive(Debug, Clone, Default)]
pub struct SignerBitmap(pub U256);

impl SignerBitmap {
    /// Create a new empty bitmap
    pub fn new() -> Self {
        Self(U256::ZERO)
    }

    /// Create a bitmap from a list of signer indices
    pub fn from_indices(indices: &[usize]) -> Self {
        let mut bitmap = U256::ZERO;
        for &idx in indices {
            bitmap |= U256::from(1u64) << idx;
        }
        Self(bitmap)
    }

    /// Check if operator at index is a signer
    pub fn is_signer(&self, index: usize) -> bool {
        (self.0 >> index) & U256::from(1u64) == U256::from(1u64)
    }

    /// Add a signer at the given index
    pub fn add_signer(&mut self, index: usize) {
        self.0 |= U256::from(1u64) << index;
    }

    /// Remove a signer at the given index
    pub fn remove_signer(&mut self, index: usize) {
        self.0 &= !(U256::from(1u64) << index);
    }

    /// Count the number of signers
    pub fn count_signers(&self) -> usize {
        let mut count = 0;
        let mut bitmap = self.0;
        while bitmap > U256::ZERO {
            if bitmap & U256::from(1u64) == U256::from(1u64) {
                count += 1;
            }
            bitmap >>= 1;
        }
        count
    }

    /// Get the raw U256 value
    pub fn as_u256(&self) -> U256 {
        self.0
    }
}

/// BLS G1 point (signature) in contract format
///
/// Represented as two 256-bit integers [x, y]
#[derive(Debug, Clone)]
pub struct G1Point {
    /// X coordinate
    pub x: U256,
    /// Y coordinate
    pub y: U256,
}

impl G1Point {
    /// Create a new G1 point
    pub fn new(x: U256, y: U256) -> Self {
        Self { x, y }
    }

    /// Create from raw bytes (64 bytes: 32 for x, 32 for y)
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 64 {
            return None;
        }
        let x = U256::from_be_slice(&bytes[0..32]);
        let y = U256::from_be_slice(&bytes[32..64]);
        Some(Self { x, y })
    }

    /// Convert to array format for contract call
    pub fn to_array(&self) -> [U256; 2] {
        [self.x, self.y]
    }
}

/// BLS G2 point (public key) in contract format
///
/// Represented as four 256-bit integers [x0, x1, y0, y1]
#[derive(Debug, Clone)]
pub struct G2Point {
    /// X coordinate (first part)
    pub x0: U256,
    /// X coordinate (second part)
    pub x1: U256,
    /// Y coordinate (first part)
    pub y0: U256,
    /// Y coordinate (second part)
    pub y1: U256,
}

impl G2Point {
    /// Create a new G2 point
    pub fn new(x0: U256, x1: U256, y0: U256, y1: U256) -> Self {
        Self { x0, x1, y0, y1 }
    }

    /// Create from raw bytes (128 bytes: 32 each for x0, x1, y0, y1)
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 128 {
            return None;
        }
        let x0 = U256::from_be_slice(&bytes[0..32]);
        let x1 = U256::from_be_slice(&bytes[32..64]);
        let y0 = U256::from_be_slice(&bytes[64..96]);
        let y1 = U256::from_be_slice(&bytes[96..128]);
        Some(Self { x0, x1, y0, y1 })
    }

    /// Convert to array format for contract call
    pub fn to_array(&self) -> [U256; 4] {
        [self.x0, self.x1, self.y0, self.y1]
    }
}

/// An aggregated job result ready for submission
#[derive(Debug, Clone)]
pub struct AggregatedResult {
    /// Service ID
    pub service_id: u64,
    /// Job call ID
    pub call_id: u64,
    /// The job output data
    pub output: Bytes,
    /// Bitmap of signers
    pub signer_bitmap: SignerBitmap,
    /// Aggregated BLS signature (G1 point)
    pub signature: G1Point,
    /// Aggregated BLS public key (G2 point)
    pub pubkey: G2Point,
}

impl AggregatedResult {
    /// Create a new aggregated result
    pub fn new(
        service_id: u64,
        call_id: u64,
        output: Bytes,
        signer_bitmap: SignerBitmap,
        signature: G1Point,
        pubkey: G2Point,
    ) -> Self {
        Self {
            service_id,
            call_id,
            output,
            signer_bitmap,
            signature,
            pubkey,
        }
    }

    /// Submit the aggregated result to the Tangle contract
    ///
    /// This calls `submitAggregatedResult` on the Tangle contract.
    pub async fn submit(&self, client: &Arc<TangleEvmClient>) -> Result<(), AggregationError> {
        let contract = client.tangle_contract();

        let _call = contract.submitAggregatedResult(
            self.service_id,
            self.call_id,
            self.output.clone(),
            self.signer_bitmap.as_u256(),
            self.signature.to_array(),
            self.pubkey.to_array(),
        );

        // TODO: Sign and send the transaction
        // For now, log that we would submit
        blueprint_core::info!(
            target: "tangle-evm-aggregation",
            "Would submit aggregated result for service {} call {} with {} signers",
            self.service_id,
            self.call_id,
            self.signer_bitmap.count_signers()
        );

        Ok(())
    }
}

/// Metadata key for storing job index in job result
pub const JOB_INDEX_KEY: &str = "tangle.job_index";

#[cfg(test)]
mod tests {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════════════
    // SignerBitmap tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_signer_bitmap() {
        let mut bitmap = SignerBitmap::new();
        assert_eq!(bitmap.count_signers(), 0);

        bitmap.add_signer(0);
        bitmap.add_signer(2);
        bitmap.add_signer(5);

        assert!(bitmap.is_signer(0));
        assert!(!bitmap.is_signer(1));
        assert!(bitmap.is_signer(2));
        assert!(!bitmap.is_signer(3));
        assert!(!bitmap.is_signer(4));
        assert!(bitmap.is_signer(5));
        assert_eq!(bitmap.count_signers(), 3);

        bitmap.remove_signer(2);
        assert!(!bitmap.is_signer(2));
        assert_eq!(bitmap.count_signers(), 2);
    }

    #[test]
    fn test_signer_bitmap_from_indices() {
        let bitmap = SignerBitmap::from_indices(&[1, 3, 7]);
        assert!(!bitmap.is_signer(0));
        assert!(bitmap.is_signer(1));
        assert!(!bitmap.is_signer(2));
        assert!(bitmap.is_signer(3));
        assert!(bitmap.is_signer(7));
        assert_eq!(bitmap.count_signers(), 3);
    }

    #[test]
    fn test_signer_bitmap_empty_indices() {
        let bitmap = SignerBitmap::from_indices(&[]);
        assert_eq!(bitmap.count_signers(), 0);
        assert_eq!(bitmap.as_u256(), U256::ZERO);
    }

    #[test]
    fn test_signer_bitmap_large_indices() {
        // Test with large indices (up to 255 operators supported by U256)
        let bitmap = SignerBitmap::from_indices(&[0, 100, 200, 255]);
        assert!(bitmap.is_signer(0));
        assert!(bitmap.is_signer(100));
        assert!(bitmap.is_signer(200));
        assert!(bitmap.is_signer(255));
        assert!(!bitmap.is_signer(50));
        assert_eq!(bitmap.count_signers(), 4);
    }

    #[test]
    fn test_signer_bitmap_duplicate_indices() {
        // Adding same index twice should not change count
        let bitmap = SignerBitmap::from_indices(&[1, 1, 1, 2]);
        assert_eq!(bitmap.count_signers(), 2);
    }

    #[test]
    fn test_signer_bitmap_as_u256() {
        let bitmap = SignerBitmap::from_indices(&[0, 1, 2]);
        // Bits 0, 1, 2 set = 0b111 = 7
        assert_eq!(bitmap.as_u256(), U256::from(7u64));
    }

    #[test]
    fn test_signer_bitmap_default() {
        let bitmap = SignerBitmap::default();
        assert_eq!(bitmap.count_signers(), 0);
        assert_eq!(bitmap.as_u256(), U256::ZERO);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // G1Point tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_g1_point_new() {
        let x = U256::from(123u64);
        let y = U256::from(456u64);
        let point = G1Point::new(x, y);
        assert_eq!(point.x, x);
        assert_eq!(point.y, y);
    }

    #[test]
    fn test_g1_point_from_bytes() {
        let mut bytes = [0u8; 64];
        // Set x = 1 (big endian)
        bytes[31] = 1;
        // Set y = 2 (big endian)
        bytes[63] = 2;

        let point = G1Point::from_bytes(&bytes).expect("should parse 64 bytes");
        assert_eq!(point.x, U256::from(1u64));
        assert_eq!(point.y, U256::from(2u64));
    }

    #[test]
    fn test_g1_point_from_bytes_invalid_length() {
        let bytes = [0u8; 32]; // Too short
        assert!(G1Point::from_bytes(&bytes).is_none());

        let bytes = [0u8; 128]; // Too long
        assert!(G1Point::from_bytes(&bytes).is_none());
    }

    #[test]
    fn test_g1_point_to_array() {
        let x = U256::from(100u64);
        let y = U256::from(200u64);
        let point = G1Point::new(x, y);
        let arr = point.to_array();
        assert_eq!(arr[0], x);
        assert_eq!(arr[1], y);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // G2Point tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_g2_point_new() {
        let x0 = U256::from(1u64);
        let x1 = U256::from(2u64);
        let y0 = U256::from(3u64);
        let y1 = U256::from(4u64);
        let point = G2Point::new(x0, x1, y0, y1);
        assert_eq!(point.x0, x0);
        assert_eq!(point.x1, x1);
        assert_eq!(point.y0, y0);
        assert_eq!(point.y1, y1);
    }

    #[test]
    fn test_g2_point_from_bytes() {
        let mut bytes = [0u8; 128];
        // Set x0 = 1, x1 = 2, y0 = 3, y1 = 4 (big endian)
        bytes[31] = 1;
        bytes[63] = 2;
        bytes[95] = 3;
        bytes[127] = 4;

        let point = G2Point::from_bytes(&bytes).expect("should parse 128 bytes");
        assert_eq!(point.x0, U256::from(1u64));
        assert_eq!(point.x1, U256::from(2u64));
        assert_eq!(point.y0, U256::from(3u64));
        assert_eq!(point.y1, U256::from(4u64));
    }

    #[test]
    fn test_g2_point_from_bytes_invalid_length() {
        let bytes = [0u8; 64]; // Too short
        assert!(G2Point::from_bytes(&bytes).is_none());

        let bytes = [0u8; 256]; // Too long
        assert!(G2Point::from_bytes(&bytes).is_none());
    }

    #[test]
    fn test_g2_point_to_array() {
        let x0 = U256::from(10u64);
        let x1 = U256::from(20u64);
        let y0 = U256::from(30u64);
        let y1 = U256::from(40u64);
        let point = G2Point::new(x0, x1, y0, y1);
        let arr = point.to_array();
        assert_eq!(arr[0], x0);
        assert_eq!(arr[1], x1);
        assert_eq!(arr[2], y0);
        assert_eq!(arr[3], y1);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // AggregatedResult tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_aggregated_result_new() {
        let service_id = 1u64;
        let call_id = 42u64;
        let output = Bytes::from(vec![1, 2, 3, 4]);
        let signer_bitmap = SignerBitmap::from_indices(&[0, 1, 2]);
        let signature = G1Point::new(U256::from(100u64), U256::from(200u64));
        let pubkey = G2Point::new(
            U256::from(1u64),
            U256::from(2u64),
            U256::from(3u64),
            U256::from(4u64),
        );

        let result = AggregatedResult::new(
            service_id,
            call_id,
            output.clone(),
            signer_bitmap.clone(),
            signature.clone(),
            pubkey.clone(),
        );

        assert_eq!(result.service_id, service_id);
        assert_eq!(result.call_id, call_id);
        assert_eq!(result.output, output);
        assert_eq!(result.signer_bitmap.count_signers(), 3);
    }
}
