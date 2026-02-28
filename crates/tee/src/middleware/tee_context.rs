//! TEE context extractor for job handlers.
//!
//! Provides [`TeeContext`] as an extractor that job handlers can use to
//! access TEE attestation information during job execution.

use crate::attestation::verifier::VerifiedAttestation;
use crate::config::TeeProvider;

/// TEE context available to job handlers.
///
/// Carries the verified attestation and deployment metadata for the
/// current TEE environment. Job handlers can extract this to make
/// TEE-aware decisions.
///
/// # Examples
///
/// ```rust,ignore
/// use blueprint_tee::TeeContext;
/// use blueprint_core::extract::Extension;
///
/// async fn my_job(
///     Extension(tee): Extension<TeeContext>,
///     body: bytes::Bytes,
/// ) -> Vec<u8> {
///     if let Some(attestation) = &tee.attestation {
///         tracing::info!(
///             provider = %attestation.verified_by(),
///             "running in verified TEE"
///         );
///     }
///     body.to_vec()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TeeContext {
    /// The verified attestation for the current TEE, if available.
    pub attestation: Option<VerifiedAttestation>,
    /// The active TEE provider, if any.
    pub provider: Option<TeeProvider>,
    /// The deployment identifier, if applicable.
    pub deployment_id: Option<String>,
}

impl TeeContext {
    /// Create a TEE context with no attestation (TEE not active).
    pub fn none() -> Self {
        Self {
            attestation: None,
            provider: None,
            deployment_id: None,
        }
    }

    /// Create a TEE context with a verified attestation.
    pub fn with_attestation(attestation: VerifiedAttestation) -> Self {
        let provider = Some(attestation.verified_by());
        Self {
            attestation: Some(attestation),
            provider,
            deployment_id: None,
        }
    }

    /// Set the deployment identifier.
    pub fn with_deployment_id(mut self, id: impl Into<String>) -> Self {
        self.deployment_id = Some(id.into());
        self
    }

    /// Returns true if a verified attestation is present.
    pub fn is_attested(&self) -> bool {
        self.attestation.is_some()
    }

    /// Returns true if any TEE provider is active.
    pub fn is_tee_active(&self) -> bool {
        self.provider.is_some()
    }
}

impl Default for TeeContext {
    fn default() -> Self {
        Self::none()
    }
}
