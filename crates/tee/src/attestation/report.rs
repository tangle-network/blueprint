//! Attestation report types.
//!
//! Provides the core [`AttestationReport`] type that captures a TEE attestation
//! with typed claims, measurement, provider identity, and optional public key binding.

use crate::attestation::claims::AttestationClaims;
use crate::config::TeeProvider;
use serde::{Deserialize, Serialize};

/// A hardware or platform measurement (PCR values, MRTD, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Measurement {
    /// The measurement algorithm (e.g., "sha256", "sha384").
    pub algorithm: String,
    /// The measurement digest as hex-encoded bytes.
    pub digest: String,
}

impl Measurement {
    /// Create a new measurement.
    ///
    /// The digest is normalized to lowercase hex to ensure consistent
    /// comparison regardless of input casing.
    pub fn new(algorithm: impl Into<String>, digest: impl Into<String>) -> Self {
        Self {
            algorithm: algorithm.into(),
            digest: digest.into().to_ascii_lowercase(),
        }
    }

    /// Create a SHA-256 measurement from a hex digest.
    pub fn sha256(digest: impl Into<String>) -> Self {
        Self::new("sha256", digest)
    }

    /// Create a SHA-384 measurement from a hex digest.
    pub fn sha384(digest: impl Into<String>) -> Self {
        Self::new("sha384", digest)
    }
}

impl core::fmt::Display for Measurement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}", self.algorithm, self.digest)
    }
}

/// Binding between an attestation report and a public key.
///
/// This proves that a specific public key was generated inside the TEE
/// and is covered by the attestation evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKeyBinding {
    /// The public key bytes (encoding depends on the key type).
    pub public_key: Vec<u8>,
    /// The key type (e.g., "x25519", "secp256k1", "ed25519").
    pub key_type: String,
    /// Hash of the public key included in the attestation report's user data.
    pub binding_digest: String,
}

/// The format of the attestation evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttestationFormat {
    /// AWS Nitro COSE-signed attestation document.
    NitroDocument,
    /// Intel TDX TDREPORT / quote.
    TdxQuote,
    /// AMD SEV-SNP attestation report.
    SevSnpReport,
    /// Azure MAA (Microsoft Azure Attestation) token.
    AzureMaaToken,
    /// GCP Confidential Space attestation token.
    GcpConfidentialToken,
    /// A mock format for testing.
    Mock,
}

/// A TEE attestation report with typed fields.
///
/// This replaces raw byte-vector attestation representations with strongly typed
/// claims and explicit provider formats. The evidence field contains the raw
/// provider-specific attestation blob for verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationReport {
    /// The TEE provider that generated this report.
    pub provider: TeeProvider,
    /// The attestation evidence format.
    pub format: AttestationFormat,
    /// Unix timestamp when this report was issued.
    pub issued_at_unix: u64,
    /// Hardware/platform measurement.
    pub measurement: Measurement,
    /// Optional binding to a public key generated inside the TEE.
    pub public_key_binding: Option<PublicKeyBinding>,
    /// Typed attestation claims.
    pub claims: AttestationClaims,
    /// Raw provider-specific attestation evidence for verification.
    pub evidence: Vec<u8>,
}

impl AttestationReport {
    /// Compute a SHA-256 digest of the attestation evidence.
    pub fn evidence_digest(&self) -> String {
        use sha2::{Digest, Sha256};
        let hash = Sha256::digest(&self.evidence);
        hex::encode(hash)
    }

    /// Check if the report has expired given a maximum age in seconds.
    pub fn is_expired(&self, max_age_secs: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now.saturating_sub(self.issued_at_unix) > max_age_secs
    }
}
