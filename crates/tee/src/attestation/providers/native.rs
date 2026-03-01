//! Native attestation verifier for TDX and SEV-SNP.
//!
//! Both Intel TDX and AMD SEV-SNP share the same attestation pattern:
//! open a device node, write report data, read back a report, and extract
//! a measurement at a known offset. This module provides a single
//! [`NativeVerifier`] with platform dispatch rather than duplicating logic
//! across separate modules.

use crate::attestation::report::AttestationReport;
use crate::attestation::verifier::{AttestationVerifier, VerifiedAttestation};
use crate::config::TeeProvider;
use crate::errors::TeeError;

/// Platform-specific configuration for native attestation verification.
#[derive(Debug, Clone)]
pub enum NativePlatform {
    /// Intel TDX: validates MRTD (TD measurement register).
    Tdx {
        /// Expected MRTD value, if enforced.
        expected_mrtd: Option<String>,
    },
    /// AMD SEV-SNP: validates launch measurement.
    SevSnp {
        /// Expected launch measurement, if enforced.
        expected_measurement: Option<String>,
    },
}

/// Unified verifier for native TEE platforms (TDX and SEV-SNP).
///
/// Both platforms use the same ioctl-based attestation pattern with different
/// device nodes (`/dev/tdx_guest` vs `/dev/sev-guest`) and report sizes.
/// This verifier handles both with platform dispatch.
pub struct NativeVerifier {
    /// Which native platform to verify for.
    pub platform: NativePlatform,
    /// Whether to allow debug enclaves/guests.
    pub allow_debug: bool,
}

impl NativeVerifier {
    /// Create a new TDX verifier.
    pub fn tdx() -> Self {
        Self {
            platform: NativePlatform::Tdx {
                expected_mrtd: None,
            },
            allow_debug: false,
        }
    }

    /// Create a new SEV-SNP verifier.
    pub fn sev_snp() -> Self {
        Self {
            platform: NativePlatform::SevSnp {
                expected_measurement: None,
            },
            allow_debug: false,
        }
    }

    /// Set the expected measurement for the platform.
    pub fn with_expected_measurement(mut self, measurement: impl Into<String>) -> Self {
        let m = measurement.into();
        match &mut self.platform {
            NativePlatform::Tdx { expected_mrtd } => *expected_mrtd = Some(m),
            NativePlatform::SevSnp {
                expected_measurement,
            } => *expected_measurement = Some(m),
        }
        self
    }

    /// Allow or deny debug mode enclaves/guests.
    pub fn with_allow_debug(mut self, allow: bool) -> Self {
        self.allow_debug = allow;
        self
    }

    fn expected_provider(&self) -> TeeProvider {
        match &self.platform {
            NativePlatform::Tdx { .. } => TeeProvider::IntelTdx,
            NativePlatform::SevSnp { .. } => TeeProvider::AmdSevSnp,
        }
    }

    fn expected_measurement(&self) -> Option<&str> {
        match &self.platform {
            NativePlatform::Tdx { expected_mrtd } => expected_mrtd.as_deref(),
            NativePlatform::SevSnp {
                expected_measurement,
            } => expected_measurement.as_deref(),
        }
    }

    fn platform_name(&self) -> &'static str {
        match &self.platform {
            NativePlatform::Tdx { .. } => "Intel TDX",
            NativePlatform::SevSnp { .. } => "AMD SEV-SNP",
        }
    }

    fn debug_entity(&self) -> &'static str {
        match &self.platform {
            NativePlatform::Tdx { .. } => "debug mode TDs",
            NativePlatform::SevSnp { .. } => "debug mode guests",
        }
    }
}

impl AttestationVerifier for NativeVerifier {
    /// Verify a native TDX or SEV-SNP attestation report.
    ///
    /// # Security Warning
    ///
    /// This verifier performs structural validation only (provider match,
    /// measurement, debug mode). It does **not** verify ioctl-based
    /// attestation evidence or platform-specific cryptographic signatures.
    fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError> {
        let expected_provider = self.expected_provider();

        if report.provider != expected_provider {
            return Err(TeeError::AttestationVerification(format!(
                "expected {} provider, got {}",
                self.platform_name(),
                report.provider
            )));
        }

        if report.claims.debug_mode && !self.allow_debug {
            return Err(TeeError::AttestationVerification(format!(
                "{} are not permitted",
                self.debug_entity()
            )));
        }

        if let Some(expected) = self.expected_measurement() {
            if report.measurement.digest != expected {
                return Err(TeeError::MeasurementMismatch {
                    expected: expected.to_string(),
                    actual: report.measurement.digest.clone(),
                });
            }
        }

        tracing::debug!("structural validation only — cryptographic signature verification requires platform SDK");

        Ok(VerifiedAttestation::new(report.clone(), expected_provider))
    }

    fn supported_provider(&self) -> TeeProvider {
        self.expected_provider()
    }
}
