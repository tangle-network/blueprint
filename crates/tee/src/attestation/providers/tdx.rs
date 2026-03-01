//! Intel TDX attestation verifier.
//!
//! This module provides a backward-compatible `TdxVerifier` type that wraps
//! the unified [`NativeVerifier`](super::native::NativeVerifier).

use crate::attestation::report::AttestationReport;
use crate::attestation::verifier::{AttestationVerifier, VerifiedAttestation};
use crate::config::TeeProvider;
use crate::errors::TeeError;

use super::native::NativeVerifier;

/// Verifier for Intel TDX attestation quotes.
///
/// Wraps [`NativeVerifier`] with TDX-specific defaults. TDX and SEV-SNP share
/// the same ioctl-based attestation pattern; see the `native` module for the
/// unified implementation.
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

    fn to_native(&self) -> NativeVerifier {
        let mut v = NativeVerifier::tdx().with_allow_debug(self.allow_debug);
        if let Some(mrtd) = &self.expected_mrtd {
            v = v.with_expected_measurement(mrtd.clone());
        }
        v
    }
}

impl Default for TdxVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl AttestationVerifier for TdxVerifier {
    fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError> {
        self.to_native().verify(report)
    }

    fn supported_provider(&self) -> TeeProvider {
        TeeProvider::IntelTdx
    }
}
