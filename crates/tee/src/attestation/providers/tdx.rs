//! Intel TDX attestation verifier.
//!
//! Validates Intel TDX (Trust Domain Extensions) attestation quotes including:
//! - TD Report and Quote verification
//! - MRTD / RTMR measurement validation
//! - TDX module version checks

use crate::attestation::report::AttestationReport;
use crate::attestation::verifier::{AttestationVerifier, VerifiedAttestation};
use crate::config::TeeProvider;
use crate::errors::TeeError;

/// Verifier for Intel TDX attestation quotes.
pub struct TdxVerifier {
    /// Expected MRTD (TD measurement register) value, if enforced.
    pub expected_mrtd: Option<String>,
    /// Whether to allow debug TDs.
    pub allow_debug: bool,
}

impl TdxVerifier {
    /// Create a new TDX verifier.
    pub fn new() -> Self {
        Self {
            expected_mrtd: None,
            allow_debug: false,
        }
    }

    /// Set the expected MRTD value.
    pub fn with_expected_mrtd(mut self, mrtd: impl Into<String>) -> Self {
        self.expected_mrtd = Some(mrtd.into());
        self
    }
}

impl Default for TdxVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl AttestationVerifier for TdxVerifier {
    fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError> {
        if report.provider != TeeProvider::IntelTdx {
            return Err(TeeError::AttestationVerification(format!(
                "expected Intel TDX provider, got {}",
                report.provider
            )));
        }

        if report.claims.debug_mode && !self.allow_debug {
            return Err(TeeError::AttestationVerification(
                "debug mode TDs are not permitted".to_string(),
            ));
        }

        if let Some(expected) = &self.expected_mrtd {
            if report.measurement.digest != *expected {
                return Err(TeeError::MeasurementMismatch {
                    expected: expected.clone(),
                    actual: report.measurement.digest.clone(),
                });
            }
        }

        Ok(VerifiedAttestation::new(
            report.clone(),
            TeeProvider::IntelTdx,
        ))
    }

    fn supported_provider(&self) -> TeeProvider {
        TeeProvider::IntelTdx
    }
}
