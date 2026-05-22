use thiserror::Error;

/// Errors raised by the upgrade watcher and its supporting code.
///
/// The watcher's job is to never run a binary that fails the sha256 / attestation
/// gate, so almost every variant here represents an abort path. Variants that
/// downgrade an upgrade to a notification (rather than killing the watcher) are
/// noted inline.
#[derive(Debug, Error)]
pub enum UpgradeError {
    /// The blueprint has zero published versions — `effectiveBinaryVersion`
    /// reverts. Treated as "not yet provisioned"; the watcher logs and waits
    /// for a publish event.
    #[error("blueprint {blueprint_id} has no published binary versions yet")]
    NoVersionsPublished { blueprint_id: u64 },

    /// On-chain digest does not match the bytes we downloaded.
    /// Trust-root violation — never run the binary.
    #[error(
        "sha256 mismatch for service {service_id} version {version_id}: \
         expected {expected}, got {actual}"
    )]
    Sha256Mismatch {
        service_id: u64,
        version_id: u64,
        expected: String,
        actual: String,
    },

    /// Attestation hash was non-zero but verification failed. Per spec: the
    /// watcher downgrades from AUTO to APPROVE-style notification rather than
    /// swapping.
    #[error(
        "attestation verification failed for service {service_id} version {version_id}: {reason}"
    )]
    AttestationFailed {
        service_id: u64,
        version_id: u64,
        reason: String,
    },

    /// Caller asked for a state mutation that requires the service to be
    /// tracked, but the manager has no record of it.
    #[error("service {service_id} is not tracked by this manager")]
    ServiceNotTracked { service_id: u64 },

    /// An on-chain read failed.
    #[error("chain read failed: {0}")]
    ChainRead(String),

    /// An on-chain write failed (e.g. ack tx reverted).
    #[error("chain write failed: {0}")]
    ChainWrite(String),

    /// Underlying I/O error (download, file rename, etc.).
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Underlying manager error.
    #[error(transparent)]
    Manager(#[from] crate::error::Error),
}

pub type Result<T> = std::result::Result<T, UpgradeError>;
