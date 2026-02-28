//! Provider-specific attestation verifiers.
//!
//! Each provider module is feature-gated and implements the [`AttestationVerifier`]
//! trait for its specific TEE platform.

#[cfg(feature = "aws-nitro")]
pub mod aws_nitro;

#[cfg(feature = "azure-snp")]
pub mod azure_snp;

#[cfg(feature = "gcp-confidential")]
pub mod gcp_confidential;

#[cfg(feature = "tdx")]
pub mod tdx;

#[cfg(feature = "sev-snp")]
pub mod sev_snp;
