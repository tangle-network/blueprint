//! Attestation gating for `AUTO` swaps.
//!
//! The current protocol allows an optional 32-byte `attestationHash` on a
//! `BinaryVersion` row. A zero hash means "no bundle published"; any other
//! value means the publisher claims a sigstore / SLSA bundle is fetchable and
//! its digest matches the on-chain value.
//!
//! Production verification (sigstore bundle fetch + chain validation) is
//! out of scope for this PR — the cargo-tangle work track owns that. Here we
//! ship the gate so the watcher does the right thing **today**:
//!
//! - `attestationHash == 0`  -> pass (no claim, nothing to verify).
//! - `attestationHash != 0`, `allow_unchecked == true` -> pass with a warning.
//! - `attestationHash != 0`, `allow_unchecked == false` -> fail.
//!
//! Failing the gate flips an AUTO swap into an APPROVE-style pending
//! notification per spec — the watcher catches the error and downgrades.

use super::error::{Result, UpgradeError};
use super::types::BinaryVersionInfo;
use blueprint_core::warn;

#[derive(Debug, Clone, Copy)]
pub struct AttestationVerifier {
    /// Mirrors the existing `--allow-unchecked-attestations` manager flag.
    /// When true, we log a warning instead of failing — same posture the
    /// existing GitHub-attestation path takes for backwards compatibility.
    allow_unchecked: bool,
}

impl AttestationVerifier {
    #[must_use]
    pub fn new(allow_unchecked: bool) -> Self {
        Self { allow_unchecked }
    }

    /// Verify the attestation associated with a binary version.
    ///
    /// # Errors
    ///
    /// `UpgradeError::AttestationFailed` when the version carries a non-zero
    /// `attestationHash` and we are configured to fail closed.
    pub fn verify(&self, service_id: u64, version: &BinaryVersionInfo) -> Result<()> {
        if !version.has_attestation() {
            return Ok(());
        }
        if self.allow_unchecked {
            warn!(
                target: "upgrade",
                service_id,
                version_id = version.version_id,
                attestation_hash = %format!("0x{}", hex::encode(version.attestation_hash.as_slice())),
                "attestation present but --allow-unchecked-attestations is set; skipping verify"
            );
            return Ok(());
        }
        // TODO: hook into the cargo-tangle attestation track once it lands:
        //  1. Resolve the bundle URI (sigstore registry / IPFS pointer).
        //  2. Fetch the bundle.
        //  3. Verify the bundle signature against the configured trust root.
        //  4. Hash the bundle and compare to `attestationHash`.
        // Until then we fail closed for AUTO; the watcher catches this and
        // downgrades to APPROVE-style notification rather than swapping.
        Err(UpgradeError::AttestationFailed {
            service_id,
            version_id: version.version_id,
            reason: "attestation verification not implemented; pass --allow-unchecked-attestations to override".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::B256;

    fn version(attestation: B256) -> BinaryVersionInfo {
        BinaryVersionInfo {
            version_id: 1,
            sha256: B256::repeat_byte(0x11),
            binary_uri: "ipfs://demo".into(),
            attestation_hash: attestation,
            published_at: 0,
            deprecated: false,
        }
    }

    #[test]
    fn zero_attestation_always_passes() {
        // Empty attestation means "no claim", so the watcher should swap on
        // sha256 alone — same as the pre-attestation manager behavior.
        let verifier = AttestationVerifier::new(false);
        verifier.verify(7, &version(B256::ZERO)).unwrap();
    }

    #[test]
    fn unknown_attestation_fails_closed_by_default() {
        let verifier = AttestationVerifier::new(false);
        let err = verifier
            .verify(7, &version(B256::repeat_byte(0xab)))
            .unwrap_err();
        assert!(
            matches!(
                err,
                UpgradeError::AttestationFailed {
                    service_id: 7,
                    version_id: 1,
                    ..
                }
            ),
            "expected AttestationFailed, got {err:?}"
        );
    }

    #[test]
    fn allow_unchecked_lets_attestation_pass() {
        let verifier = AttestationVerifier::new(true);
        verifier
            .verify(7, &version(B256::repeat_byte(0xab)))
            .unwrap();
    }
}
