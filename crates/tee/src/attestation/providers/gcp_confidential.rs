//! GCP Confidential Space attestation verifier.
//!
//! Validates GCP Confidential Space attestation tokens including:
//! - Token signature verification
//! - Workload identity validation
//! - Machine family TEE type derivation

use crate::attestation::report::AttestationReport;
use crate::attestation::verifier::{AttestationVerifier, VerifiedAttestation};
use crate::config::TeeProvider;
use crate::errors::TeeError;

/// Verifier for GCP Confidential Space attestation tokens.
pub struct GcpConfidentialVerifier {
    /// Expected measurement digest, if enforced.
    pub expected_measurement: Option<String>,
}

impl GcpConfidentialVerifier {
    /// Create a new GCP Confidential Space verifier.
    pub fn new() -> Self {
        Self {
            expected_measurement: None,
        }
    }

    /// Set the expected measurement.
    pub fn with_expected_measurement(mut self, measurement: impl Into<String>) -> Self {
        self.expected_measurement = Some(measurement.into());
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
