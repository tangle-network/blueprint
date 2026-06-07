//! Post-launch TEE attestation gate for remote cloud deployments.
//!
//! This crate does not decide what counts as a verified TEE attestation.
//! `blueprint-tee` owns that security boundary and constructs the
//! [`VerifiedAttestation`] proof. Remote providers only fetch live evidence from
//! the provisioned workload, call the appropriate TEE verifier, and fail closed
//! before reporting a `require_tee` deployment as trusted.

#[cfg(feature = "tee-attestation-nitro")]
pub mod aws_nitro;
pub mod azure;
pub mod gcp;

use crate::core::remote::CloudProvider;
pub use blueprint_tee::attestation::{
    AttestationError, AttestationPolicy, VerifiedAttestation, fresh_nonce,
};

/// The seam over provider-specific post-launch attestation.
///
/// Each provider implements fetch (pull evidence from the live VM / metadata /
/// MAA endpoint) and verify (delegate evidence verification to `blueprint-tee`).
#[async_trait::async_trait]
pub trait TeeAttestationGate: Send + Sync {
    /// Which cloud provider this gate attests.
    fn provider(&self) -> CloudProvider;

    /// Fetch raw attestation evidence from the running deployment.
    async fn fetch(&self, endpoint: &str, nonce: &str) -> Result<Vec<u8>, AttestationError>;

    /// Verify previously-fetched evidence against `policy`.
    async fn verify(
        &self,
        evidence: &[u8],
        policy: &AttestationPolicy,
    ) -> Result<VerifiedAttestation, AttestationError>;

    /// Convenience: fetch then verify in one step.
    async fn attest(
        &self,
        endpoint: &str,
        nonce: &str,
        policy: &AttestationPolicy,
    ) -> Result<VerifiedAttestation, AttestationError> {
        let evidence = self.fetch(endpoint, nonce).await?;
        self.verify(&evidence, policy).await
    }
}

/// Construct the attestation gate for a provider, or fail closed if the provider
/// does not support a verifiable confidential-compute attestation in this build.
pub fn gate_for(provider: &CloudProvider) -> Result<Box<dyn TeeAttestationGate>, AttestationError> {
    match provider {
        CloudProvider::GCP => Ok(Box::new(gcp::GcpConfidentialSpaceGate::new())),
        CloudProvider::Azure => Ok(Box::new(azure::AzureMaaGate::new())),
        #[cfg(feature = "tee-attestation-nitro")]
        CloudProvider::AWS => Ok(Box::new(aws_nitro::AwsNitroGate::new())),
        #[cfg(not(feature = "tee-attestation-nitro"))]
        CloudProvider::AWS => Err(AttestationError::Unsatisfiable {
            provider: cloud_provider_label(provider),
            reason: "AWS Nitro attestation requires the `tee-attestation-nitro` feature; \
                     this build cannot verify a Nitro document"
                .to_string(),
        }),
        other => Err(AttestationError::Unsatisfiable {
            provider: cloud_provider_label(other),
            reason: "provider does not expose a verifiable TEE attestation".to_string(),
        }),
    }
}

/// Enforce the `require_tee` contract for a freshly provisioned deployment.
pub async fn enforce_require_tee(
    provider: &CloudProvider,
    endpoint: &str,
    nonce: &str,
    policy: &AttestationPolicy,
) -> Result<VerifiedAttestation, AttestationError> {
    let gate = gate_for(provider)?;
    gate.attest(endpoint, nonce, policy).await
}

/// Enforce `require_tee` for a freshly provisioned VM and stamp the verified
/// verdict into deployment metadata.
pub async fn enforce_and_record(
    provider: &CloudProvider,
    endpoint: &str,
    nonce: &str,
    policy: &AttestationPolicy,
    metadata: &mut std::collections::HashMap<String, String>,
) -> Result<VerifiedAttestation, AttestationError> {
    metadata.insert("tee_attested".to_string(), "false".to_string());

    let verified = enforce_require_tee(provider, endpoint, nonce, policy).await?;

    let workload_bound = policy.is_workload_bound();
    let verdict = if workload_bound {
        "true"
    } else {
        "hardware-only"
    };
    metadata.insert("tee_attested".to_string(), verdict.to_string());
    metadata.insert("tee_workload_bound".to_string(), workload_bound.to_string());

    let report = verified.report();
    if !report.measurement.digest.is_empty() {
        metadata.insert(
            "tee_measurement".to_string(),
            report.measurement.digest.clone(),
        );
    }
    if report.issued_at_unix != 0 {
        metadata.insert(
            "tee_attested_at".to_string(),
            report.issued_at_unix.to_string(),
        );
    }
    Ok(verified)
}

/// Provisioner-facing `require_tee` gate.
pub async fn gate_provisioned(
    provider: &CloudProvider,
    endpoint: Option<&str>,
    custom_config: &std::collections::HashMap<String, String>,
    metadata: &mut std::collections::HashMap<String, String>,
) -> crate::core::error::Result<()> {
    use crate::core::error::Error;

    metadata.insert("tee_attested".to_string(), "false".to_string());

    let endpoint = endpoint.ok_or_else(|| {
        Error::ConfigurationError(format!(
            "require_tee set for {provider:?} but the provisioned VM has no reachable \
             endpoint to attest against; refusing to report it as a TEE deployment"
        ))
    })?;

    let nonce = fresh_nonce();
    metadata.insert("tee_nonce".to_string(), nonce.clone());
    let policy = AttestationPolicy::from_custom_config(custom_config, &nonce);

    if policy.expected_image_digest.is_none() || policy.expected_audience.is_none() {
        blueprint_core::warn!(
            provider = ?provider,
            audience_pinned = policy.expected_audience.is_some(),
            image_digest_pinned = policy.expected_image_digest.is_some(),
            "require_tee gate running without audience and/or image-digest pinning: \
             the attestation will prove a genuine confidential VM but not the expected \
             workload/relying-party"
        );
    }

    match enforce_and_record(provider, endpoint, &nonce, &policy, metadata).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::ConfigurationError(format!(
            "require_tee attestation failed (fail-closed): {e}"
        ))),
    }
}

/// HTTP client for fetching attestation evidence from provisioned workloads.
pub(crate) fn default_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_default()
}

/// Hard ceiling on an attestation token body.
pub(crate) const MAX_TOKEN_BYTES: usize = 64 * 1024;

/// Read an HTTP response body with a hard byte cap.
pub(crate) async fn read_body_capped<F>(
    resp: reqwest::Response,
    max: usize,
    make_err: F,
) -> Result<Vec<u8>, AttestationError>
where
    F: Fn(String) -> AttestationError,
{
    if let Some(declared) = resp.content_length() {
        if declared > max as u64 {
            return Err(make_err(format!(
                "response body too large: declared {declared} bytes exceeds {max} cap"
            )));
        }
    }
    let mut resp = resp;
    let mut body = Vec::new();
    loop {
        match resp.chunk().await {
            Ok(Some(chunk)) => {
                if body.len() + chunk.len() > max {
                    return Err(make_err(format!(
                        "response body exceeds {max}-byte cap; aborting read"
                    )));
                }
                body.extend_from_slice(&chunk);
            }
            Ok(None) => break,
            Err(e) => return Err(make_err(format!("reading response body failed: {e}"))),
        }
    }
    Ok(body)
}

/// Trim leading/trailing ASCII whitespace from a token body without allocating.
pub(crate) fn trim_ascii(bytes: &[u8]) -> &[u8] {
    let start = bytes.iter().position(|b| !b.is_ascii_whitespace());
    let Some(start) = start else { return &[] };
    let end = bytes
        .iter()
        .rposition(|b| !b.is_ascii_whitespace())
        .unwrap_or(start);
    &bytes[start..=end]
}

fn cloud_provider_label(provider: &CloudProvider) -> String {
    format!("{provider:?}")
}

#[cfg(all(test, feature = "tee-attestation"))]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn unsupported_provider_fails_closed() {
        let err = enforce_require_tee(
            &CloudProvider::DigitalOcean,
            "1.2.3.4",
            "nonce",
            &AttestationPolicy::production(),
        )
        .await
        .expect_err("non-confidential provider must fail closed");
        assert!(matches!(err, AttestationError::Unsatisfiable { .. }));
    }

    #[test]
    fn from_custom_config_enforces_freshness_floor() {
        let cfg = HashMap::new();
        let policy = AttestationPolicy::from_custom_config(&cfg, "nonce-xyz");
        assert_eq!(policy.max_age_secs, Some(600));
        assert!(!policy.allow_debug);
        assert_eq!(policy.expected_nonce.as_deref(), Some("nonce-xyz"));
    }

    #[test]
    fn from_custom_config_clamps_oversized_max_age() {
        let mut cfg = HashMap::new();
        cfg.insert("tee_max_age_secs".to_string(), "999999999".to_string());
        let policy = AttestationPolicy::from_custom_config(&cfg, "n");
        assert_eq!(
            policy.max_age_secs,
            Some(AttestationPolicy::MAX_AGE_CEILING_SECS)
        );
    }

    #[test]
    fn from_custom_config_honours_max_age_below_ceiling() {
        let mut cfg = HashMap::new();
        cfg.insert("tee_max_age_secs".to_string(), "120".to_string());
        let policy = AttestationPolicy::from_custom_config(&cfg, "n");
        assert_eq!(policy.max_age_secs, Some(120));
    }

    #[test]
    #[cfg(not(feature = "testing"))]
    fn from_custom_config_refuses_debug_outside_testing_build() {
        let mut cfg = HashMap::new();
        cfg.insert("tee_allow_debug".to_string(), "true".to_string());
        let policy = AttestationPolicy::from_custom_config(&cfg, "n");
        assert!(!policy.allow_debug);
    }

    #[test]
    fn workload_bound_requires_audience_or_image_digest() {
        let hw_only = AttestationPolicy::production().with_nonce("n");
        assert!(!hw_only.is_workload_bound());
        assert!(
            AttestationPolicy::production()
                .with_audience("svc")
                .is_workload_bound()
        );
        assert!(
            AttestationPolicy::production()
                .with_image_digest("sha256:abc")
                .is_workload_bound()
        );
    }
}
