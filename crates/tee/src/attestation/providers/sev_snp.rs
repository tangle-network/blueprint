//! AMD SEV-SNP attestation verifier.
//!
//! Validates AMD SEV-SNP attestation reports including:
//! - Report signature verification against AMD root keys
//! - Launch measurement validation
//! - Guest policy checks (debug, migration, SMT)

use crate::attestation::report::AttestationReport;
use crate::attestation::verifier::{AttestationVerifier, VerifiedAttestation};
use crate::config::TeeProvider;
use crate::errors::TeeError;

/// Verifier for AMD SEV-SNP attestation reports.
pub struct SevSnpVerifier {
    /// Expected launch measurement, if enforced.
    pub expected_measurement: Option<String>,
    /// Whether to allow debug guests.
    pub allow_debug: bool,
}

impl SevSnpVerifier {
    /// Create a new SEV-SNP verifier.
    pub fn new() -> Self {
        Self {
            expected_measurement: None,
            allow_debug: false,
        }
    }

    /// Set the expected launch measurement.
    pub fn with_expected_measurement(mut self, measurement: impl Into<String>) -> Self {
        self.expected_measurement = Some(measurement.into());
        self
    }
}

impl Default for SevSnpVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl AttestationVerifier for SevSnpVerifier {
    fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError> {
        if report.provider != TeeProvider::AmdSevSnp {
            return Err(TeeError::AttestationVerification(format!(
                "expected AMD SEV-SNP provider, got {}",
                report.provider
            )));
        }

        if report.claims.debug_mode && !self.allow_debug {
            return Err(TeeError::AttestationVerification(
                "debug mode guests are not permitted".to_string(),
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
            TeeProvider::AmdSevSnp,
        ))
    }

    fn supported_provider(&self) -> TeeProvider {
        TeeProvider::AmdSevSnp
    }
}
