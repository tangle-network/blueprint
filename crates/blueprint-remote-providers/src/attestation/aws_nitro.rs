//! AWS Nitro Enclaves attestation gate.
//!
//! The out-of-enclave provisioner cannot fetch an NSM attestation document:
//! `/dev/nsm` is reachable only from inside the enclave and AWS exposes no
//! remote API for it. Fetch therefore fails closed. Verification of a document
//! obtained through an in-enclave channel is delegated to `blueprint-tee`.

use super::{AttestationError, AttestationPolicy, TeeAttestationGate, VerifiedAttestation};
use crate::core::remote::CloudProvider;
use blueprint_tee::attestation::providers::aws_nitro::NitroVerifier;

pub use blueprint_tee::attestation::providers::aws_nitro::AWS_NITRO_ROOT_CERT_PEM;

/// Gate for AWS Nitro Enclaves.
pub struct AwsNitroGate {
    verifier: NitroVerifier,
}

impl AwsNitroGate {
    /// Construct with the pinned AWS Nitro root certificate.
    pub fn new() -> Self {
        Self {
            verifier: NitroVerifier::new(),
        }
    }

    /// Override the trusted root certificate, primarily for tests.
    pub fn with_root_cert_pem(mut self, pem: impl Into<String>) -> Self {
        self.verifier = self.verifier.with_root_cert_pem(pem);
        self
    }

    /// Verify a Nitro COSE_Sign1 attestation document.
    pub fn verify_document(
        &self,
        cose_bytes: &[u8],
        policy: &AttestationPolicy,
    ) -> Result<VerifiedAttestation, AttestationError> {
        self.verifier.verify_document(cose_bytes, policy)
    }
}

impl Default for AwsNitroGate {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl TeeAttestationGate for AwsNitroGate {
    fn provider(&self) -> CloudProvider {
        CloudProvider::AWS
    }

    async fn fetch(&self, _endpoint: &str, _nonce: &str) -> Result<Vec<u8>, AttestationError> {
        Err(AttestationError::Unsatisfiable {
            provider: "AWS".to_string(),
            reason: "Nitro NSM attestation is producible only inside the enclave; the \
                     out-of-enclave provisioner cannot fetch it. Supply the COSE document \
                     via an in-enclave channel and call verify_document()."
                .to_string(),
        })
    }

    async fn verify(
        &self,
        evidence: &[u8],
        policy: &AttestationPolicy,
    ) -> Result<VerifiedAttestation, AttestationError> {
        self.verify_document(evidence, policy)
    }
}
