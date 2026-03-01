//! GCP Confidential Space attestation verifier.
//!
//! Currently validates GCP Confidential Space attestation structurally:
//! - Measurement comparison
//! - Debug mode detection
//!
//! **Limitation:** Token signature verification, workload identity validation,
//! and machine family TEE type derivation are not yet implemented. These require
//! provider-specific dependencies that will be added in a future release.

use crate::attestation::report::AttestationReport;
use crate::attestation::verifier::{AttestationVerifier, VerifiedAttestation};
use crate::config::TeeProvider;
use crate::errors::TeeError;

/// Verifier for GCP Confidential Space attestation tokens.
pub struct GcpConfidentialVerifier {
    /// Expected measurement digest, if enforced.
    pub expected_measurement: Option<String>,
    /// Whether to allow debug-mode VMs.
    pub allow_debug: bool,
}

impl GcpConfidentialVerifier {
    /// Create a new GCP Confidential Space verifier.
    pub fn new() -> Self {
        Self {
            expected_measurement: None,
            allow_debug: false,
        }
    }

    /// Set the expected measurement.
    pub fn with_expected_measurement(mut self, measurement: impl Into<String>) -> Self {
        self.expected_measurement = Some(measurement.into());
        self
    }

    /// Allow debug-mode VMs (not recommended for production).
    pub fn allow_debug(mut self, allow: bool) -> Self {
        self.allow_debug = allow;
        self
    }
}

impl Default for GcpConfidentialVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl AttestationVerifier for GcpConfidentialVerifier {
    fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError> {
        if report.provider != TeeProvider::GcpConfidential {
            return Err(TeeError::AttestationVerification(format!(
                "expected GCP Confidential provider, got {}",
                report.provider
            )));
        }

        if report.claims.debug_mode && !self.allow_debug {
            return Err(TeeError::AttestationVerification(
                "debug mode VMs are not permitted".to_string(),
            ));
        }

        if let Some(expected) = &self.expected_measurement {
            if report.measurement.digest != *expected {
                return Err(TeeError::MeasurementMismatch {
                    expected: expected.clone(),
                    actual: report.measurement.digest.clone(),
                });
            }
        }

        Ok(VerifiedAttestation::new(
            report.clone(),
            TeeProvider::GcpConfidential,
        ))
    }

    fn supported_provider(&self) -> TeeProvider {
        TeeProvider::GcpConfidential
    }
}
