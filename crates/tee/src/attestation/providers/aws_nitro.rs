//! AWS Nitro Enclave attestation verifier.
//!
//! Currently validates Nitro attestation structurally:
//! - PCR0 measurement comparison (enclave image hash)
//! - Enclave debug mode detection
//!
//! **Limitation:** COSE signature verification and certificate chain validation
//! against the AWS Nitro root CA are not yet implemented. These require
//! provider-specific dependencies (`aws-nitro-enclaves-cose`) that will be
//! added in a future release. Until then, the evidence blob should be
//! verified externally or via the on-chain attestation hash.

use crate::attestation::report::AttestationReport;
use crate::attestation::verifier::{AttestationVerifier, VerifiedAttestation};
use crate::config::TeeProvider;
use crate::errors::TeeError;

/// Verifier for AWS Nitro Enclave attestation documents.
pub struct NitroVerifier {
    /// Expected PCR0 measurement (enclave image hash), if enforced.
    pub expected_pcr0: Option<String>,
    /// Whether to allow debug-mode enclaves.
    pub allow_debug: bool,
}

impl NitroVerifier {
    /// Create a new Nitro verifier.
    pub fn new() -> Self {
        Self {
            expected_pcr0: None,
            allow_debug: false,
        }
    }

    /// Set the expected PCR0 measurement.
    pub fn with_expected_pcr0(mut self, pcr0: impl Into<String>) -> Self {
        self.expected_pcr0 = Some(pcr0.into());
        self
    }

    /// Allow debug-mode enclaves (not recommended for production).
    pub fn allow_debug(mut self, allow: bool) -> Self {
        self.allow_debug = allow;
        self
    }
}

impl Default for NitroVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl AttestationVerifier for NitroVerifier {
    /// Verify an AWS Nitro attestation report.
    ///
    /// # Security Warning
    ///
    /// This verifier performs structural validation only (provider match, PCR0,
    /// debug mode). It does **not** verify COSE signatures or the AWS Nitro root
    /// CA certificate chain. Full cryptographic verification requires the
    /// `aws-nitro-enclaves-cose` crate.
    fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError> {
        if report.provider != TeeProvider::AwsNitro {
            return Err(TeeError::AttestationVerification(format!(
                "expected AWS Nitro provider, got {}",
                report.provider
            )));
        }

        if report.claims.debug_mode && !self.allow_debug {
            return Err(TeeError::AttestationVerification(
                "debug mode enclaves are not permitted".to_string(),
            ));
        }

        if let Some(expected) = &self.expected_pcr0 {
            if report.measurement.digest != *expected {
                return Err(TeeError::MeasurementMismatch {
                    expected: expected.clone(),
                    actual: report.measurement.digest.clone(),
                });
            }
        }

        // NOTE: This verifier performs structural checks (provider, PCR0, debug mode)
        // but does not yet validate COSE signatures or the certificate chain against
        // the AWS Nitro root CA. Full cryptographic verification requires the
        // `aws-nitro-enclaves-cose` crate and will be added when provider-specific
        // dependencies are integrated. Until then, the evidence blob should be
        // verified externally (e.g., by the on-chain contract's attestation hash).

        tracing::debug!("structural validation only — cryptographic signature verification requires aws-nitro-enclaves-cose");

        Ok(VerifiedAttestation::new(
            report.clone(),
            TeeProvider::AwsNitro,
        ))
    }

    fn supported_provider(&self) -> TeeProvider {
        TeeProvider::AwsNitro
    }
}
