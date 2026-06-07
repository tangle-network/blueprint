//! Attestation verification errors.
//!
//! Every variant represents a fail-closed outcome: callers must not treat a
//! workload as TEE-trusted when any of these errors is returned.

use crate::errors::TeeError;

/// Errors raised while fetching or verifying TEE attestation evidence.
#[derive(Debug, thiserror::Error)]
pub enum AttestationError {
    /// The attestation evidence could not be retrieved from the provider.
    #[error("attestation fetch failed for {provider}: {reason}")]
    Fetch { provider: String, reason: String },

    /// The evidence was retrieved but failed cryptographic signature
    /// verification.
    #[error("attestation signature verification failed for {provider}: {reason}")]
    Signature { provider: String, reason: String },

    /// The signature verified but a required claim did not match policy
    /// (audience, issuer, nonce, image digest, PCR, expiry, debug mode, ...).
    #[error("attestation claim rejected for {provider}: {reason}")]
    Claim { provider: String, reason: String },

    /// The JWKS / signing-key material could not be obtained or parsed.
    #[error("could not obtain signing keys for {provider}: {reason}")]
    Keys { provider: String, reason: String },

    /// The requested attestation cannot be completed in this environment.
    #[error("require_tee unsatisfiable for {provider}: {reason}")]
    Unsatisfiable { provider: String, reason: String },

    /// The evidence was malformed (not valid JWT/COSE/CBOR).
    #[error("malformed attestation for {provider}: {reason}")]
    Malformed { provider: String, reason: String },
}

impl From<AttestationError> for TeeError {
    fn from(err: AttestationError) -> Self {
        Self::AttestationVerification(err.to_string())
    }
}
