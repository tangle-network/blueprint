//! GCP Confidential Space attestation gate.
//!
//! Confidential Space VMs obtain an OIDC **attestation token** (a JWT) from the
//! in-VM launcher socket (`/run/container_launcher/teeserver.sock`). That socket
//! is reachable only from inside the guest, so the provisioner fetches the token
//! over an HTTPS endpoint the deployed workload exposes (the agent forwards the
//! launcher token, optionally challenged with our nonce).
//!
//! Verification is delegated to
//! [`blueprint_tee::attestation::providers::gcp_confidential::GcpConfidentialVerifier`].

use super::{AttestationError, AttestationPolicy, TeeAttestationGate, VerifiedAttestation};
use crate::core::remote::CloudProvider;
use blueprint_tee::attestation::providers::gcp_confidential::GcpConfidentialVerifier;

/// Default in-VM path (over HTTPS the workload re-exposes) for the attestation
/// token. The port/path is a convention the deployed agent honours; callers can
/// override the full URL template.
const DEFAULT_TOKEN_PATH: &str = "/.well-known/attestation-token";

/// Gate for GCP Confidential Space.
pub struct GcpConfidentialSpaceGate {
    http: reqwest::Client,
    verifier: GcpConfidentialVerifier,
    /// Path on the VM endpoint that returns the OIDC attestation token.
    token_path: String,
}

impl GcpConfidentialSpaceGate {
    /// Construct with production Google JWKS + issuer.
    pub fn new() -> Self {
        Self {
            http: super::default_http_client(),
            verifier: GcpConfidentialVerifier::new(),
            token_path: DEFAULT_TOKEN_PATH.to_string(),
        }
    }

    /// Override the JWKS URL (used by tests to point at a local mock).
    pub fn with_jwks_url(mut self, url: impl Into<String>) -> Self {
        self.verifier = self.verifier.with_jwks_url(url);
        self
    }

    /// Override the token fetch path on the VM endpoint.
    pub fn with_token_path(mut self, path: impl Into<String>) -> Self {
        self.token_path = path.into();
        self
    }

    /// Override allowed issuers (used by tests).
    pub fn with_allowed_issuers(mut self, issuers: Vec<String>) -> Self {
        self.verifier = self.verifier.with_allowed_issuers(issuers);
        self
    }
}

impl Default for GcpConfidentialSpaceGate {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl TeeAttestationGate for GcpConfidentialSpaceGate {
    fn provider(&self) -> CloudProvider {
        CloudProvider::GCP
    }

    async fn fetch(&self, endpoint: &str, nonce: &str) -> Result<Vec<u8>, AttestationError> {
        let base = normalize_endpoint(endpoint);
        let url = format!("{base}{}", self.token_path);
        let resp = self
            .http
            .get(&url)
            // The workload's launcher binds this nonce into `eat_nonce`.
            .query(&[("nonce", nonce)])
            .send()
            .await
            .map_err(|e| AttestationError::Fetch {
                provider: "GCP".to_string(),
                reason: format!("token fetch from {url} failed: {e}"),
            })?;
        if !resp.status().is_success() {
            return Err(AttestationError::Fetch {
                provider: "GCP".to_string(),
                reason: format!("token endpoint returned HTTP {}", resp.status()),
            });
        }
        let body = super::read_body_capped(resp, super::MAX_TOKEN_BYTES, |reason| {
            AttestationError::Fetch {
                provider: "GCP".to_string(),
                reason,
            }
        })
        .await?;
        Ok(super::trim_ascii(&body).to_vec())
    }

    async fn verify(
        &self,
        evidence: &[u8],
        policy: &AttestationPolicy,
    ) -> Result<VerifiedAttestation, AttestationError> {
        let token = std::str::from_utf8(evidence).map_err(|e| AttestationError::Malformed {
            provider: "GCP".to_string(),
            reason: format!("token is not UTF-8: {e}"),
        })?;
        self.verifier.verify_token(token, policy).await
    }
}

/// Ensure the endpoint carries a scheme; default to HTTPS.
fn normalize_endpoint(endpoint: &str) -> String {
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        endpoint.trim_end_matches('/').to_string()
    } else {
        format!("https://{}", endpoint.trim_end_matches('/'))
    }
}
