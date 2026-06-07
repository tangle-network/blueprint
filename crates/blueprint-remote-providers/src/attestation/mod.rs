//! Post-launch TEE attestation: fetch + cryptographically verify provider
//! attestation before a `require_tee` deployment is reported trusted.
//!
//! # Why this module exists
//!
//! Provisioning a VM with a TEE instance-type flag set (AWS
//! `EnclaveOptions.enabled`, GCP `enableConfidentialCompute`, Azure
//! `ConfidentialVM`) only asks the cloud control plane to *place* the workload
//! on confidential hardware. It proves nothing about the running guest. The
//! flag is cosmetic until something **fetches the provider attestation and
//! verifies it**. This module is that something.
//!
//! # Cardinal rule (fail closed)
//!
//! A deployment must never be reported as TEE-trusted without a genuine
//! attestation verification. If attestation cannot be fetched *and*
//! cryptographically verified, the `require_tee` gate for that provider returns
//! [`Err`] — it does not silently pass. See [`enforce_require_tee`].
//!
//! # Per-provider model
//!
//! | Provider      | Evidence            | Fetch | Verify |
//! |---------------|---------------------|-------|--------|
//! | GCP Conf.Space| OIDC JWT            | in-VM metadata service | JWKS sig + claims |
//! | Azure         | MAA JWT             | MAA REST endpoint      | JWKS sig + claims |
//! | AWS Nitro     | NSM COSE_Sign1 doc  | **in-enclave only**    | AWS Nitro root cert + PCRs |
//!
//! GCP and Azure emit **tokens** (JWT) that are verifiable in-process against
//! published JWKS. AWS Nitro emits a COSE_Sign1 document produced by the NSM
//! device (`/dev/nsm`), which is reachable **only from inside the enclave** —
//! the provisioner runs outside it and there is no AWS API to pull an enclave's
//! attestation document remotely. The COSE verification is implemented and
//! tested here (in the `aws_nitro` module, behind `tee-attestation-nitro`), but
//! the *fetch* fails closed by design.
//!
//! # Seam, not duplication
//!
//! Deep hardware-quote verification (SEV-SNP / TDX DCAP) lives in the
//! `blueprint-tee` crate behind [`blueprint_tee::attestation::AttestationVerifier`].
//! This module verifies the **cloud-token / COSE envelope** layer and, under the
//! `tee-attestation-seam` feature, can convert a fetched report into a
//! `blueprint_tee` report to hand to that verifier rather than reimplementing it.

#[cfg(feature = "tee-attestation-nitro")]
pub mod aws_nitro;
pub mod azure;
pub mod gcp;
pub mod jwt;

#[cfg(feature = "tee-attestation-seam")]
pub mod seam;

use crate::core::remote::CloudProvider;
use std::collections::BTreeMap;

/// Errors raised while fetching or verifying TEE attestation.
///
/// Every variant represents a *fail-closed* outcome: if any of these is
/// returned for a `require_tee` deployment, the deployment must not be treated
/// as a usable TEE deployment.
#[derive(Debug, thiserror::Error)]
pub enum AttestationError {
    /// The attestation evidence could not be retrieved from the provider.
    #[error("attestation fetch failed for {provider:?}: {reason}")]
    Fetch {
        provider: CloudProvider,
        reason: String,
    },

    /// The attestation evidence was retrieved but failed cryptographic
    /// signature verification.
    #[error("attestation signature verification failed for {provider:?}: {reason}")]
    Signature {
        provider: CloudProvider,
        reason: String,
    },

    /// The signature verified but a required claim did not match policy
    /// (audience, issuer, nonce, image digest, PCR, expiry, debug mode, ...).
    #[error("attestation claim rejected for {provider:?}: {reason}")]
    Claim {
        provider: CloudProvider,
        reason: String,
    },

    /// The JWKS / signing-key material could not be obtained or parsed.
    #[error("could not obtain signing keys for {provider:?}: {reason}")]
    Keys {
        provider: CloudProvider,
        reason: String,
    },

    /// `require_tee` was requested for a provider whose attestation cannot be
    /// completed in this environment. This is the explicit fail-closed stub.
    #[error("require_tee unsatisfiable for {provider:?}: {reason}")]
    Unsatisfiable {
        provider: CloudProvider,
        reason: String,
    },

    /// The evidence was malformed (not valid JWT/COSE/CBOR).
    #[error("malformed attestation for {provider:?}: {reason}")]
    Malformed {
        provider: CloudProvider,
        reason: String,
    },
}

/// Policy applied when verifying a fetched attestation.
///
/// Empty/`None` fields mean "do not enforce this dimension". The defaults
/// enforce only what is universally required (signature + non-debug + freshness);
/// callers tighten the policy with expected measurements/nonce/audience.
#[derive(Debug, Clone, Default)]
pub struct AttestationPolicy {
    /// Expected audience claim (GCP/Azure JWT `aud`). When set, the token's
    /// audience must contain this value.
    pub expected_audience: Option<String>,
    /// Expected nonce echoed back in the attestation (binds the report to this
    /// provisioning request, defeating replay).
    pub expected_nonce: Option<String>,
    /// Expected workload/image digest (GCP `submods.container.image_digest`,
    /// Azure `x-ms-..` image identity, Nitro PCR8/usermeta).
    pub expected_image_digest: Option<String>,
    /// Expected primary measurement (Nitro PCR0, GCP/Azure platform measurement).
    pub expected_measurement: Option<String>,
    /// Whether debug-mode TEEs are tolerated. Defaults to `false` (rejected).
    pub allow_debug: bool,
    /// Maximum tolerated age of the attestation, in seconds. `None` disables the
    /// freshness check (not recommended).
    pub max_age_secs: Option<u64>,
}

impl AttestationPolicy {
    /// A baseline policy: reject debug TEEs and require freshness within ten
    /// minutes. It does **not** by itself pin audience, image digest, or
    /// measurement — with those unset the gate proves only "a genuine, fresh,
    /// non-debug confidential VM answered our nonce", not "running OUR expected
    /// workload for OUR relying party". Pin [`with_audience`](Self::with_audience)
    /// and [`with_image_digest`](Self::with_image_digest) (and, for Nitro,
    /// [`with_measurement`](Self::with_measurement)) for a workload-bound verdict.
    /// [`gate_provisioned`] warns when a `require_tee` gate runs without them.
    pub fn production() -> Self {
        Self {
            allow_debug: false,
            max_age_secs: Some(600),
            ..Self::default()
        }
    }

    /// Pin the expected audience.
    pub fn with_audience(mut self, aud: impl Into<String>) -> Self {
        self.expected_audience = Some(aud.into());
        self
    }

    /// Pin the expected nonce.
    pub fn with_nonce(mut self, nonce: impl Into<String>) -> Self {
        self.expected_nonce = Some(nonce.into());
        self
    }

    /// Pin the expected workload image digest.
    pub fn with_image_digest(mut self, digest: impl Into<String>) -> Self {
        self.expected_image_digest = Some(digest.into());
        self
    }

    /// Pin the expected platform measurement / PCR0.
    pub fn with_measurement(mut self, m: impl Into<String>) -> Self {
        self.expected_measurement = Some(m.into());
        self
    }

    /// Hard ceiling on the configurable freshness window. `custom_config` is set
    /// by the service requester/operator — i.e. not fully trusted by the relying
    /// party — so an arbitrarily large `tee_max_age_secs` must not be honoured
    /// silently; it would defeat the freshness gate. One hour bounds the worst
    /// case while leaving room for slow provisioning.
    pub const MAX_AGE_CEILING_SECS: u64 = 3600;

    /// Build a policy from a provisioner's `custom_config` map.
    ///
    /// Recognised keys: `tee_expected_audience`, `tee_expected_image_digest`,
    /// `tee_expected_measurement`, `tee_allow_debug` (`true`/`false`),
    /// `tee_max_age_secs`. The nonce is supplied separately (generated per
    /// provisioning request).
    ///
    /// Security-relaxing knobs are never applied silently. `tee_max_age_secs` is
    /// clamped to [`MAX_AGE_CEILING_SECS`] (with a warn when the request asked for
    /// more), and `tee_allow_debug=true` only takes effect in a `testing` build —
    /// in a normal build it is refused with a loud warn so a debug TEE is never
    /// silently accepted on the say-so of deployment config.
    pub fn from_custom_config(
        config: &std::collections::HashMap<String, String>,
        nonce: &str,
    ) -> Self {
        let mut policy = Self::production();
        policy.expected_nonce = Some(nonce.to_string());
        if let Some(v) = config.get("tee_expected_audience") {
            policy.expected_audience = Some(v.clone());
        }
        if let Some(v) = config.get("tee_expected_image_digest") {
            policy.expected_image_digest = Some(v.clone());
        }
        if let Some(v) = config.get("tee_expected_measurement") {
            policy.expected_measurement = Some(v.clone());
        }
        if let Some(v) = config.get("tee_allow_debug") {
            if v.eq_ignore_ascii_case("true") {
                if cfg!(feature = "testing") {
                    policy.allow_debug = true;
                    blueprint_core::warn!(
                        "tee_allow_debug=true honoured: accepting debug-mode TEEs \
                         (testing build only)"
                    );
                } else {
                    blueprint_core::warn!(
                        "tee_allow_debug=true requested via deployment config but refused: \
                         debug-mode TEEs are only accepted in a `testing` build. \
                         Keeping debug TEEs rejected."
                    );
                }
            }
        }
        if let Some(v) = config.get("tee_max_age_secs") {
            if let Ok(secs) = v.parse::<u64>() {
                if secs > Self::MAX_AGE_CEILING_SECS {
                    blueprint_core::warn!(
                        requested_secs = secs,
                        ceiling_secs = Self::MAX_AGE_CEILING_SECS,
                        "tee_max_age_secs exceeds the freshness ceiling; clamping. \
                         A larger window would weaken replay protection."
                    );
                }
                policy.max_age_secs = Some(secs.min(Self::MAX_AGE_CEILING_SECS));
            }
        }
        policy
    }
}

/// Generate a fresh, unguessable attestation challenge nonce.
pub fn fresh_nonce() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

/// Whether a policy binds the attested VM to a specific workload / relying party.
///
/// A genuine confidential VM that merely answers our nonce is *hardware-only*: an
/// attacker who legitimately owns a confidential VM in the same provider fleet can
/// pass it. Pinning the audience (relying-party binding) and/or the image digest
/// (workload binding) is what upgrades the verdict to workload-bound, which is the
/// only result downstream consumers may treat as `tee_attested=true`. Kept as a
/// single source of truth so the marker stamped into deployment metadata cannot
/// drift from the policy that earned it.
pub(crate) fn policy_is_workload_bound(policy: &AttestationPolicy) -> bool {
    policy.expected_audience.is_some() || policy.expected_image_digest.is_some()
}

/// A verified TEE attestation result.
///
/// This type can only be constructed by a verifier in this module after a
/// successful signature + claim check. Holding a `VerifiedTeeDeployment` is a
/// type-level proof that the `require_tee` gate passed for `provider`. It is the
/// local analogue of `blueprint_tee::attestation::VerifiedAttestation` (whose
/// constructor is `pub(crate)` to that crate and therefore not reachable here).
#[derive(Debug, Clone)]
pub struct VerifiedTeeDeployment {
    provider: CloudProvider,
    /// The primary measurement that was verified (PCR0 / platform measurement).
    measurement: Option<String>,
    /// Selected verified claims, surfaced for auditing / on-chain anchoring.
    claims: BTreeMap<String, String>,
    /// Unix issue time of the attestation, if present.
    issued_at_unix: Option<u64>,
}

impl VerifiedTeeDeployment {
    /// Construct a verified result. Restricted to this module so external code
    /// cannot fabricate a "trusted" deployment without going through a verifier.
    pub(crate) fn new(
        provider: CloudProvider,
        measurement: Option<String>,
        claims: BTreeMap<String, String>,
        issued_at_unix: Option<u64>,
    ) -> Self {
        Self {
            provider,
            measurement,
            claims,
            issued_at_unix,
        }
    }

    /// The provider whose attestation was verified.
    pub fn provider(&self) -> &CloudProvider {
        &self.provider
    }

    /// The verified primary measurement (PCR0 / platform measurement), if any.
    pub fn measurement(&self) -> Option<&str> {
        self.measurement.as_deref()
    }

    /// The verified claims surfaced for auditing.
    pub fn claims(&self) -> &BTreeMap<String, String> {
        &self.claims
    }

    /// The attestation issue time (unix seconds), if present.
    pub fn issued_at_unix(&self) -> Option<u64> {
        self.issued_at_unix
    }
}

/// The seam over provider-specific post-launch attestation.
///
/// Each provider implements fetch (pull evidence from the live VM / metadata /
/// MAA endpoint) and verify (cryptographic signature + claim policy). The two
/// are split so a caller that already holds evidence (e.g. forwarded from an
/// in-enclave agent) can verify it without re-fetching.
#[async_trait::async_trait]
pub trait TeeAttestationGate: Send + Sync {
    /// Which provider this gate attests.
    fn provider(&self) -> CloudProvider;

    /// Fetch raw attestation evidence from the running deployment.
    ///
    /// `endpoint` is the reachable host/IP of the provisioned VM; `nonce` is the
    /// challenge to bind into the evidence. Returns the raw provider evidence
    /// (JWT bytes for GCP/Azure, COSE document bytes for Nitro).
    async fn fetch(&self, endpoint: &str, nonce: &str) -> Result<Vec<u8>, AttestationError>;

    /// Verify previously-fetched evidence against `policy`.
    ///
    /// Performs real signature verification (JWKS for JWT, root-cert chain for
    /// COSE) and claim enforcement. Returns a [`VerifiedTeeDeployment`] only on
    /// success.
    async fn verify(
        &self,
        evidence: &[u8],
        policy: &AttestationPolicy,
    ) -> Result<VerifiedTeeDeployment, AttestationError>;

    /// Convenience: fetch then verify in one step.
    async fn attest(
        &self,
        endpoint: &str,
        nonce: &str,
        policy: &AttestationPolicy,
    ) -> Result<VerifiedTeeDeployment, AttestationError> {
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
        // Built without the Nitro COSE verifier: refuse rather than bless an
        // AWS deployment we cannot attest. Note that even WITH the verifier the
        // out-of-enclave provisioner cannot fetch the NSM document, so the Nitro
        // happy path is in-enclave forwarding to `aws_nitro::verify_document`.
        #[cfg(not(feature = "tee-attestation-nitro"))]
        CloudProvider::AWS => Err(AttestationError::Unsatisfiable {
            provider: CloudProvider::AWS,
            reason: "AWS Nitro attestation requires the `tee-attestation-nitro` feature \
                     (COSE verifier); this build cannot verify a Nitro document"
                .to_string(),
        }),
        other => Err(AttestationError::Unsatisfiable {
            provider: other.clone(),
            reason: "provider does not expose a verifiable TEE attestation".to_string(),
        }),
    }
}

/// Enforce the `require_tee` contract for a freshly provisioned deployment.
///
/// This is the single chokepoint the provisioners call. It fetches the
/// provider's attestation from `endpoint`, verifies it against `policy`, and
/// returns a [`VerifiedTeeDeployment`] proof on success. **Any** failure to
/// fetch or verify returns [`Err`] — the deployment must then be treated as
/// untrusted (and the caller should terminate it).
///
/// `endpoint` is the reachable address of the VM; `nonce` is a fresh,
/// unguessable challenge that the caller also recorded so it can be matched in
/// `policy.expected_nonce`.
pub async fn enforce_require_tee(
    provider: &CloudProvider,
    endpoint: &str,
    nonce: &str,
    policy: &AttestationPolicy,
) -> Result<VerifiedTeeDeployment, AttestationError> {
    let gate = gate_for(provider)?;
    gate.attest(endpoint, nonce, policy).await
}

/// Enforce `require_tee` for a freshly provisioned VM and stamp the verdict into
/// its metadata.
///
/// This is the helper the provider provisioners call right after the VM is
/// reachable. On success it returns `Ok(())` and records the verified
/// measurement plus a `tee_attested` marker in `metadata`. The marker is
/// `true` only when the policy binds the workload (audience and/or image-digest
/// pinned); without that binding it is `hardware-only` (a genuine confidential
/// VM answered our nonce, but the running workload/relying-party is unproven),
/// accompanied by `tee_workload_bound=false`. On any fetch/verify failure it
/// returns the [`AttestationError`] — the caller must treat the deployment as
/// untrusted and tear it down. It NEVER records `tee_attested=true` without a
/// verified, workload-bound result.
///
/// `policy` is read from `AttestationPolicy::from_metadata` style env/config by
/// the caller; `nonce` must be a fresh, unguessable challenge that the caller
/// also passes inside `policy.expected_nonce`.
pub async fn enforce_and_record(
    provider: &CloudProvider,
    endpoint: &str,
    nonce: &str,
    policy: &AttestationPolicy,
    metadata: &mut std::collections::HashMap<String, String>,
) -> Result<VerifiedTeeDeployment, AttestationError> {
    // Default to untrusted; only a verified result flips this.
    metadata.insert("tee_attested".to_string(), "false".to_string());

    let verified = enforce_require_tee(provider, endpoint, nonce, policy).await?;

    // Distinguish a workload-bound verdict from a hardware-only one. With neither
    // audience (relying-party binding) nor image-digest (workload binding) pinned,
    // a genuine-but-unpinned confidential VM the attacker legitimately owns can
    // answer our nonce and pass — so it must NOT be reported with the same
    // `tee_attested=true` marker a workload-bound result gets. Downstream
    // consumers can treat only `tee_attested=true` as workload-bound.
    let workload_bound = policy_is_workload_bound(policy);
    let verdict = if workload_bound {
        "true"
    } else {
        "hardware-only"
    };
    metadata.insert("tee_attested".to_string(), verdict.to_string());
    metadata.insert("tee_workload_bound".to_string(), workload_bound.to_string());
    if let Some(m) = verified.measurement() {
        metadata.insert("tee_measurement".to_string(), m.to_string());
    }
    if let Some(iat) = verified.issued_at_unix() {
        metadata.insert("tee_attested_at".to_string(), iat.to_string());
    }
    Ok(verified)
}

/// Provisioner-facing `require_tee` gate.
///
/// Generates a fresh nonce, derives the policy from `custom_config`, enforces
/// attestation against `endpoint`, and stamps the verdict into `metadata`.
/// Returns the crate [`crate::core::error::Error`] (not [`AttestationError`]) so
/// provisioners can `?` it directly. Fails closed when there is no reachable
/// endpoint.
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

    // Surface the residual-guarantee gap: without audience + image-digest pinning
    // the gate proves "a genuine fresh confidential VM answered our nonce", not
    // "running our expected workload for our relying party". Loud but non-fatal —
    // pin via `tee_expected_audience` / `tee_expected_image_digest` (or the
    // blueprint deployment spec) to bind the verdict to the workload.
    if policy.expected_image_digest.is_none() || policy.expected_audience.is_none() {
        blueprint_core::warn!(
            provider = ?provider,
            audience_pinned = policy.expected_audience.is_some(),
            image_digest_pinned = policy.expected_image_digest.is_some(),
            "require_tee gate running without audience and/or image-digest pinning: \
             the attestation will prove a genuine confidential VM but not the expected \
             workload/relying-party. Set tee_expected_audience / tee_expected_image_digest."
        );
    }

    match enforce_and_record(provider, endpoint, &nonce, &policy, metadata).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::ConfigurationError(format!(
            "require_tee attestation failed (fail-closed): {e}"
        ))),
    }
}

/// HTTP client for fetching attestation tokens and JWKS documents.
///
/// Short timeout (attestation endpoints are local/regional), TLS via the
/// workspace `rustls` backend already configured for `reqwest`.
pub(crate) fn default_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_default()
}

/// Hard ceiling on an attestation token body. OIDC/MAA JWTs are < 16 KiB; this
/// leaves generous headroom while bounding heap growth from a hostile endpoint.
pub(crate) const MAX_TOKEN_BYTES: usize = 64 * 1024;

/// Hard ceiling on a JWKS document. Provider JWKS are a handful of keys.
pub(crate) const MAX_JWKS_BYTES: usize = 256 * 1024;

/// Read an HTTP response body with a hard byte cap.
///
/// The attestation token and JWKS are fetched from sources this module is built
/// to distrust (the provisioned VM whose trust is in question, or a JWKS URL).
/// `reqwest::Response::{bytes,text}` buffer the *entire* body into memory, so a
/// multi-GB or chunked-streamed response would cause unbounded heap growth / OOM
/// in the control plane before any signature check runs. This streams chunks and
/// aborts the moment the cumulative size exceeds `max` — and rejects up front
/// when the declared `Content-Length` already exceeds `max`. `make_err` builds
/// the provider-specific error so callers keep their existing error variant.
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

/// Trim leading/trailing ASCII whitespace from a token body without allocating
/// (provider token endpoints sometimes append a trailing newline).
pub(crate) fn trim_ascii(bytes: &[u8]) -> &[u8] {
    let start = bytes.iter().position(|b| !b.is_ascii_whitespace());
    let Some(start) = start else { return &[] };
    let end = bytes
        .iter()
        .rposition(|b| !b.is_ascii_whitespace())
        .unwrap_or(start);
    &bytes[start..=end]
}

/// Current unix time in seconds (verifier-internal helper).
pub(crate) fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(all(test, feature = "tee-attestation"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn unsupported_provider_fails_closed() {
        // A non-confidential provider must never yield a trusted deployment.
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
    fn verified_deployment_is_constructor_gated() {
        // Compile-time contract: VerifiedTeeDeployment::new is pub(crate); this
        // test merely documents that a trusted result can only originate here.
        let v = VerifiedTeeDeployment::new(
            CloudProvider::GCP,
            Some("abc".into()),
            BTreeMap::new(),
            Some(now_unix()),
        );
        assert_eq!(v.measurement(), Some("abc"));
    }

    use std::collections::HashMap;

    #[test]
    fn from_custom_config_enforces_freshness_floor() {
        // A bare config (the operator pins nothing) must still inherit the
        // production floor: freshness on, debug rejected, and the per-request
        // nonce bound. None of these is downgradable to "off" via config.
        let cfg = HashMap::new();
        let policy = AttestationPolicy::from_custom_config(&cfg, "nonce-xyz");
        assert_eq!(
            policy.max_age_secs,
            Some(600),
            "freshness must stay on at the production default"
        );
        assert!(!policy.allow_debug, "debug TEEs must stay rejected");
        assert_eq!(policy.expected_nonce.as_deref(), Some("nonce-xyz"));
    }

    #[test]
    fn from_custom_config_clamps_oversized_max_age() {
        // The service requester/operator is not fully trusted, so an enormous
        // freshness window must be clamped to the ceiling, never honoured as-is.
        let mut cfg = HashMap::new();
        cfg.insert("tee_max_age_secs".to_string(), "999999999".to_string());
        let policy = AttestationPolicy::from_custom_config(&cfg, "n");
        assert_eq!(
            policy.max_age_secs,
            Some(AttestationPolicy::MAX_AGE_CEILING_SECS),
            "an oversized max-age must be clamped to the ceiling"
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
        // tee_allow_debug=true from deployment config must NOT relax the gate in a
        // normal (non-testing) build — a debug TEE is never silently accepted.
        let mut cfg = HashMap::new();
        cfg.insert("tee_allow_debug".to_string(), "true".to_string());
        let policy = AttestationPolicy::from_custom_config(&cfg, "n");
        assert!(
            !policy.allow_debug,
            "tee_allow_debug must be refused outside a testing build"
        );
    }

    #[test]
    fn workload_bound_requires_audience_or_image_digest() {
        // Hardware-only: nonce + freshness only → not workload-bound.
        let hw_only = AttestationPolicy::production().with_nonce("n");
        assert!(!policy_is_workload_bound(&hw_only));
        // Audience pinned → workload-bound (relying-party binding present).
        assert!(policy_is_workload_bound(
            &AttestationPolicy::production().with_audience("svc")
        ));
        // Image digest pinned → workload-bound (workload binding present).
        assert!(policy_is_workload_bound(
            &AttestationPolicy::production().with_image_digest("sha256:abc")
        ));
    }
}
