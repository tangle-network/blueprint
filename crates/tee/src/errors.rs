//! Error types for the TEE subsystem.

/// Errors that can occur in the TEE subsystem.
#[derive(Debug, thiserror::Error)]
pub enum TeeError {
    /// TEE configuration error.
    #[error("tee config error: {0}")]
    Config(String),

    /// Attestation verification failed.
    #[error("attestation verification failed: {0}")]
    AttestationVerification(String),

    /// Attestation report expired.
    #[error("attestation report expired: issued_at={issued_at}, max_age_secs={max_age_secs}")]
    AttestationExpired { issued_at: u64, max_age_secs: u64 },

    /// Unsupported TEE provider.
    #[error("unsupported tee provider: {0}")]
    UnsupportedProvider(String),

    /// TEE deployment failed.
    #[error("tee deployment failed: {0}")]
    DeploymentFailed(String),

    /// TEE runtime not available.
    #[error("tee runtime not available: {0}")]
    RuntimeUnavailable(String),

    /// Key exchange failed.
    #[error("key exchange failed: {0}")]
    KeyExchange(String),

    /// Sealed secret operation failed.
    #[error("sealed secret error: {0}")]
    SealedSecret(String),

    /// Measurement mismatch.
    #[error("measurement mismatch: expected {expected}, got {actual}")]
    MeasurementMismatch { expected: String, actual: String },

    /// Public key binding missing or invalid.
    #[error("public key binding error: {0}")]
    PublicKeyBinding(String),

    /// Backend-specific error.
    #[error("backend error: {0}")]
    Backend(String),

    /// I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Serialization error.
    #[error("serialization error: {0}")]
    Serialization(String),
}
