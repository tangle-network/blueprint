//! Attestation verifier trait and verified attestation wrapper.

use crate::attestation::report::AttestationReport;
use crate::config::TeeProvider;
use crate::errors::TeeError;

/// A verified attestation report.
///
/// This type can only be constructed through an [`AttestationVerifier`],
/// providing a type-level guarantee that the report has been verified.
#[derive(Debug, Clone)]
pub struct VerifiedAttestation {
    /// The verified attestation report.
    report: AttestationReport,
    /// The provider that verified this report.
    verified_by: TeeProvider,
}

impl VerifiedAttestation {
    /// Create a new verified attestation.
    ///
    /// This should only be called by [`AttestationVerifier`] implementations
    /// after successful verification.
    pub fn new(report: AttestationReport, verified_by: TeeProvider) -> Self {
        Self {
            report,
            verified_by,
        }
    }

    /// Get the underlying attestation report.
    pub fn report(&self) -> &AttestationReport {
        &self.report
    }

    /// Get the provider that verified this attestation.
    pub fn verified_by(&self) -> TeeProvider {
        self.verified_by
    }

    /// Consume and return the inner report.
    pub fn into_report(self) -> AttestationReport {
        self.report
    }
}

/// Trait for verifying TEE attestation reports.
///
/// Implementations are provider-specific and validate the cryptographic
/// evidence, claims freshness, and measurement policy compliance.
///
/// # Examples
///
/// ```rust,ignore
/// use blueprint_tee::{AttestationVerifier, AttestationReport, VerifiedAttestation, TeeError};
///
/// struct MyVerifier;
///
/// impl AttestationVerifier for MyVerifier {
///     fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError> {
///         // Validate evidence, check measurements, verify signatures...
///         Ok(VerifiedAttestation::new(report.clone(), report.provider))
///     }
///
///     fn supported_provider(&self) -> TeeProvider {
///         TeeProvider::IntelTdx
///     }
/// }
/// ```
pub trait AttestationVerifier: Send + Sync {
    /// Verify an attestation report.
    ///
    /// Returns a [`VerifiedAttestation`] if the report passes all checks,
    /// or a [`TeeError`] describing the verification failure.
    fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError>;

    /// The TEE provider this verifier supports.
    fn supported_provider(&self) -> TeeProvider;
}
