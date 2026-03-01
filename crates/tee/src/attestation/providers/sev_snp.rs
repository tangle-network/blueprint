//! AMD SEV-SNP attestation verifier.
//!
//! This module provides a backward-compatible `SevSnpVerifier` type that wraps
//! the unified [`NativeVerifier`](super::native::NativeVerifier).

use crate::attestation::report::AttestationReport;
use crate::attestation::verifier::{AttestationVerifier, VerifiedAttestation};
use crate::config::TeeProvider;
use crate::errors::TeeError;

use super::native::NativeVerifier;

/// Verifier for AMD SEV-SNP attestation reports.
///
/// Wraps [`NativeVerifier`] with SEV-SNP-specific defaults. TDX and SEV-SNP
/// share the same ioctl-based attestation pattern; see the `native` module for
/// the unified implementation.
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

    /// Allow debug-mode guests (not recommended for production).
    pub fn allow_debug(mut self, allow: bool) -> Self {
        self.allow_debug = allow;
        self
    }

    fn to_native(&self) -> NativeVerifier {
        let mut v = NativeVerifier::sev_snp().with_allow_debug(self.allow_debug);
        if let Some(m) = &self.expected_measurement {
            v = v.with_expected_measurement(m.clone());
        }
        v
    }
}

impl Default for SevSnpVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl AttestationVerifier for SevSnpVerifier {
    fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError> {
        self.to_native().verify(report)
    }

    fn supported_provider(&self) -> TeeProvider {
        TeeProvider::AmdSevSnp
    }
}
