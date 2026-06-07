//! Azure attestation gate (Microsoft Azure Attestation, MAA).
//!
//! An Azure Confidential VM obtains its hardware evidence (SEV-SNP report via
//! the vTPM) inside the guest and presents it to a regional MAA instance, which
//! returns a signed **MAA token** (a JWT). The provisioner cannot read the
//! guest's vTPM remotely, so it fetches the already-minted MAA token over an
//! HTTPS endpoint the workload exposes (challenged with our nonce).
//!
//! Verification is a real JWT check against the MAA instance's JWKS
//! (`https://<instance>/certs`) plus claim policy. See [`super::jwt`].

use super::jwt::{JwksSource, verify_jwt_attestation};
use super::{AttestationError, AttestationPolicy, TeeAttestationGate, VerifiedTeeDeployment};
use crate::core::remote::CloudProvider;

/// Default shared MAA instance (per-region instances are preferred in prod).
pub const DEFAULT_MAA_INSTANCE: &str = "https://sharedeus.eus.attest.azure.net";

/// Path on the VM endpoint that returns the MAA token.
const DEFAULT_TOKEN_PATH: &str = "/.well-known/maa-token";

/// Gate for Azure Confidential VM / MAA.
pub struct AzureMaaGate {
    http: reqwest::Client,
    jwks: JwksSource,
    token_path: String,
    allowed_issuers: Vec<String>,
}

impl AzureMaaGate {
    /// Construct against the default shared MAA instance.
    pub fn new() -> Self {
        Self::for_instance(DEFAULT_MAA_INSTANCE)
    }

    /// Construct against a specific MAA instance URL (e.g. a per-region one).
    ///
    /// The MAA issuer equals the instance URL, and its JWKS is at
    /// `<instance>/certs`.
    pub fn for_instance(instance_url: &str) -> Self {
        let instance = instance_url.trim_end_matches('/').to_string();
        Self {
            http: super::default_http_client(),
            jwks: JwksSource::new(format!("{instance}/certs")),
            token_path: DEFAULT_TOKEN_PATH.to_string(),
            allowed_issuers: vec![instance],
        }
    }

    /// Override the JWKS URL (used by tests).
    pub fn with_jwks_url(mut self, url: impl Into<String>) -> Self {
        self.jwks = JwksSource::new(url);
        self
    }

    /// Override the token fetch path on the VM endpoint.
    pub fn with_token_path(mut self, path: impl Into<String>) -> Self {
        self.token_path = path.into();
        self
    }

    /// Override allowed issuers (used by tests).
    pub fn with_allowed_issuers(mut self, issuers: Vec<String>) -> Self {
        self.allowed_issuers = issuers;
        self
    }
}

impl Default for AzureMaaGate {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl TeeAttestationGate for AzureMaaGate {
    fn provider(&self) -> CloudProvider {
        CloudProvider::Azure
    }

    async fn fetch(&self, endpoint: &str, nonce: &str) -> Result<Vec<u8>, AttestationError> {
        let base = normalize_endpoint(endpoint);
        let url = format!("{base}{}", self.token_path);
        let resp = self
            .http
            .get(&url)
            .query(&[("nonce", nonce)])
            .send()
            .await
            .map_err(|e| AttestationError::Fetch {
                provider: CloudProvider::Azure,
                reason: format!("MAA token fetch from {url} failed: {e}"),
            })?;
        if !resp.status().is_success() {
            return Err(AttestationError::Fetch {
                provider: CloudProvider::Azure,
                reason: format!("MAA token endpoint returned HTTP {}", resp.status()),
            });
        }
        let body = super::read_body_capped(resp, super::MAX_TOKEN_BYTES, |reason| {
            AttestationError::Fetch {
                provider: CloudProvider::Azure,
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
    ) -> Result<VerifiedTeeDeployment, AttestationError> {
        let token = std::str::from_utf8(evidence).map_err(|e| AttestationError::Malformed {
            provider: CloudProvider::Azure,
            reason: format!("MAA token is not UTF-8: {e}"),
        })?;
        let issuers: Vec<&str> = self.allowed_issuers.iter().map(String::as_str).collect();
        verify_jwt_attestation(
            CloudProvider::Azure,
            token,
            &self.jwks,
            &issuers,
            policy,
            &self.http,
        )
        .await
    }
}

fn normalize_endpoint(endpoint: &str) -> String {
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        endpoint.trim_end_matches('/').to_string()
    } else {
        format!("https://{}", endpoint.trim_end_matches('/'))
    }
}
