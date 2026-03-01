//! Provider-specific attestation verifiers.
//!
//! Each provider module is feature-gated and implements the
//! [`AttestationVerifier`](super::AttestationVerifier) trait for its specific TEE platform.
//!
//! TDX and SEV-SNP share the same ioctl-based attestation pattern (open device,
//! write report data, read report, extract measurement). They are unified in the
//! `native` module with platform dispatch. The separate `tdx` and `sev_snp`
//! modules re-export convenience constructors for backward compatibility.

#[cfg(feature = "aws-nitro")]
pub mod aws_nitro;

#[cfg(feature = "azure-snp")]
pub mod azure_snp;

#[cfg(feature = "gcp-confidential")]
pub mod gcp_confidential;

/// Unified native attestation verifier for TDX and SEV-SNP.
#[cfg(any(feature = "tdx", feature = "sev-snp"))]
pub mod native;

#[cfg(feature = "tdx")]
pub mod tdx;

#[cfg(feature = "sev-snp")]
pub mod sev_snp;
