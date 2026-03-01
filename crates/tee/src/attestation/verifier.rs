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
    /// after successful verification. Restricted to `pub(crate)` to prevent
    /// external callers from bypassing verification.
    pub(crate) fn new(report: AttestationReport, verified_by: TeeProvider) -> Self {
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

impl VerifiedAttestation {
    /// Test-only constructor for creating a `VerifiedAttestation` without a verifier.
    #[cfg(any(test, feature = "test-utils"))]
    pub fn new_for_test(report: AttestationReport, verified_by: TeeProvider) -> Self {
        Self::new(report, verified_by)
    }
}

/// The level of verification performed by an [`AttestationVerifier`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationLevel {
    /// Structural validation only: provider match, debug mode, measurement comparison.
    /// No cryptographic signature verification is performed.
    Structural,
    /// Full cryptographic verification: signature validation, certificate chain, etc.
    Cryptographic,
}

/// Trait for verifying TEE attestation reports.
///
/// Implementations are provider-specific and validate the cryptographic
/// evidence, claims freshness, and measurement policy compliance.
///
/// Implementors construct [`VerifiedAttestation`] via the crate-internal
/// `VerifiedAttestation::new()` method, which is `pub(crate)` to prevent
/// external code from bypassing verification.
pub trait AttestationVerifier: Send + Sync {
    /// Verify an attestation report.
    ///
    /// Returns a [`VerifiedAttestation`] if the report passes all checks,
    /// or a [`TeeError`] describing the verification failure.
    fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError>;

    /// The TEE provider this verifier supports.
    fn supported_provider(&self) -> TeeProvider;

    /// The level of verification this implementation performs.
    ///
    /// All built-in verifiers currently return [`VerificationLevel::Structural`]
    /// because cryptographic signature verification requires provider-specific
    /// dependencies that are not yet integrated.
    fn verification_level(&self) -> VerificationLevel {
        VerificationLevel::Structural
    }
}
