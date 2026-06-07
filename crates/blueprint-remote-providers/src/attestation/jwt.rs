//! Shared OIDC/JWT attestation-token verification (GCP Confidential Space OIDC
//! and Azure MAA both emit JWTs).
//!
//! Verification uses the maintained [`jsonwebtoken`] crate against the issuer's
//! published JWKS — no hand-rolled signature math. The flow is:
//!
//! 1. `decode_header(token)` → read `kid` + `alg`.
//! 2. Fetch the issuer JWKS, find the key with that `kid`.
//! 3. Build a [`DecodingKey`] from the JWK and verify signature + standard
//!    claims (`exp`, `nbf`, `iss`, `aud`) via [`Validation`].
//! 4. Enforce the attestation-specific claim policy (nonce, image digest,
//!    debug-mode, freshness) on the decoded payload.
//!
//! All failures are fail-closed: a token that cannot be parsed, whose `kid` is
//! unknown, whose signature is invalid, or whose claims do not match policy is
//! rejected with an [`AttestationError`].

use super::{AttestationError, AttestationPolicy, VerifiedTeeDeployment};
use crate::core::remote::CloudProvider;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde_json::Value;
use std::collections::BTreeMap;

/// How to obtain the issuer's JWKS for a given token.
#[derive(Clone)]
pub struct JwksSource {
    /// HTTPS URL of the JWKS document.
    pub jwks_url: String,
}

impl JwksSource {
    pub fn new(jwks_url: impl Into<String>) -> Self {
        Self {
            jwks_url: jwks_url.into(),
        }
    }
}

/// Verify a JWT attestation token end to end.
///
/// `provider` only labels errors. `allowed_issuers` (if non-empty) restricts the
/// `iss` claim. The returned [`VerifiedTeeDeployment`] carries the verified
/// claims for auditing.
pub async fn verify_jwt_attestation(
    provider: CloudProvider,
    token: &str,
    jwks: &JwksSource,
    allowed_issuers: &[&str],
    policy: &AttestationPolicy,
    http: &reqwest::Client,
) -> Result<VerifiedTeeDeployment, AttestationError> {
    // 1. Parse header for kid + alg without trusting the signature yet.
    let header = decode_header(token).map_err(|e| AttestationError::Malformed {
        provider: provider.clone(),
        reason: format!("invalid JWT header: {e}"),
    })?;
    let kid = header
        .kid
        .clone()
        .ok_or_else(|| AttestationError::Malformed {
            provider: provider.clone(),
            reason: "JWT header missing `kid`".to_string(),
        })?;

    // Only asymmetric algorithms are acceptable for attestation tokens; an `alg`
    // of `none` or an HMAC family would let a forger sign their own token.
    if !is_asymmetric(header.alg) {
        return Err(AttestationError::Signature {
            provider,
            reason: format!("refusing non-asymmetric JWT alg {:?}", header.alg),
        });
    }

    // 2. Fetch + parse the JWKS, locate the signing key.
    let jwk_set = fetch_jwks(provider.clone(), jwks, http).await?;
    let jwk = jwk_set.find(&kid).ok_or_else(|| AttestationError::Keys {
        provider: provider.clone(),
        reason: format!("no JWKS key matches kid `{kid}`"),
    })?;
    let decoding_key = DecodingKey::from_jwk(jwk).map_err(|e| AttestationError::Keys {
        provider: provider.clone(),
        reason: format!("invalid JWK for kid `{kid}`: {e}"),
    })?;

    // 3. Verify signature + standard claims.
    let mut validation = Validation::new(header.alg);
    validation.validate_exp = true;
    validation.validate_nbf = true;
    if let Some(leeway) = policy.max_age_secs {
        // Bound clock skew; freshness is separately enforced below on `iat`.
        validation.leeway = leeway.min(300);
    }
    if !allowed_issuers.is_empty() {
        validation.set_issuer(allowed_issuers);
    }
    match &policy.expected_audience {
        Some(aud) => validation.set_audience(&[aud.as_str()]),
        // No expected audience pinned: do not silently accept any audience.
        // `validate_aud` stays true and we require the `aud` claim to be present,
        // matched below against nothing — so we disable the built-in check but
        // re-assert presence in the claim policy step.
        None => validation.validate_aud = false,
    }

    let decoded = decode::<Value>(token, &decoding_key, &validation).map_err(|e| {
        // jsonwebtoken folds signature, exp, aud and iss failures into one error
        // kind; classify the common ones so the caller gets a precise reason.
        use jsonwebtoken::errors::ErrorKind;
        match e.kind() {
            ErrorKind::InvalidSignature => AttestationError::Signature {
                provider: provider.clone(),
                reason: "JWT signature does not verify against JWKS".to_string(),
            },
            ErrorKind::ExpiredSignature => AttestationError::Claim {
                provider: provider.clone(),
                reason: "attestation token expired".to_string(),
            },
            ErrorKind::InvalidAudience => AttestationError::Claim {
                provider: provider.clone(),
                reason: "attestation token audience mismatch".to_string(),
            },
            ErrorKind::InvalidIssuer => AttestationError::Claim {
                provider: provider.clone(),
                reason: "attestation token issuer not allowed".to_string(),
            },
            other => AttestationError::Signature {
                provider: provider.clone(),
                reason: format!("JWT verification failed: {other:?}"),
            },
        }
    })?;

    let payload = decoded.claims;

    // 4. Attestation-specific claim policy.
    enforce_claim_policy(&provider, &payload, policy)?;

    // Surface a stable subset of claims for auditing.
    let mut claims = BTreeMap::new();
    for key in [
        "iss",
        "sub",
        "aud",
        "nonce",
        "eat_nonce",
        "dbgstat",
        "swname",
    ] {
        if let Some(v) = payload.get(key) {
            claims.insert(key.to_string(), value_to_string(v));
        }
    }
    let issued_at = payload.get("iat").and_then(Value::as_u64);
    let measurement = extract_measurement(&payload);

    Ok(VerifiedTeeDeployment::new(
        provider,
        measurement,
        claims,
        issued_at,
    ))
}

/// Reject `alg`s that would let a caller sign their own attestation token.
fn is_asymmetric(alg: Algorithm) -> bool {
    matches!(
        alg,
        Algorithm::RS256
            | Algorithm::RS384
            | Algorithm::RS512
            | Algorithm::PS256
            | Algorithm::PS384
            | Algorithm::PS512
            | Algorithm::ES256
            | Algorithm::ES384
            | Algorithm::EdDSA
    )
}

async fn fetch_jwks(
    provider: CloudProvider,
    jwks: &JwksSource,
    http: &reqwest::Client,
) -> Result<JwkSet, AttestationError> {
    let resp = http
        .get(&jwks.jwks_url)
        .send()
        .await
        .map_err(|e| AttestationError::Keys {
            provider: provider.clone(),
            reason: format!("JWKS fetch failed: {e}"),
        })?;
    if !resp.status().is_success() {
        return Err(AttestationError::Keys {
            provider,
            reason: format!("JWKS endpoint returned HTTP {}", resp.status()),
        });
    }
    let provider_for_body = provider.clone();
    let body = super::read_body_capped(resp, super::MAX_JWKS_BYTES, move |reason| {
        AttestationError::Keys {
            provider: provider_for_body.clone(),
            reason,
        }
    })
    .await?;
    serde_json::from_slice::<JwkSet>(&body).map_err(|e| AttestationError::Keys {
        provider,
        reason: format!("malformed JWKS document: {e}"),
    })
}

/// Enforce nonce / image-digest / debug-mode / freshness from the decoded claims.
fn enforce_claim_policy(
    provider: &CloudProvider,
    payload: &Value,
    policy: &AttestationPolicy,
) -> Result<(), AttestationError> {
    // Nonce binding (defeats replay of a previously-captured valid token).
    if let Some(expected) = &policy.expected_nonce {
        let nonce = claim_nonce(payload).ok_or_else(|| AttestationError::Claim {
            provider: provider.clone(),
            reason: "expected a nonce claim but token has none".to_string(),
        })?;
        if &nonce != expected {
            return Err(AttestationError::Claim {
                provider: provider.clone(),
                reason: "attestation nonce mismatch".to_string(),
            });
        }
    }

    // When no audience was pinned, still require the claim to exist so we never
    // bless a token whose audience we did not even look at.
    if policy.expected_audience.is_none() && payload.get("aud").is_none() {
        return Err(AttestationError::Claim {
            provider: provider.clone(),
            reason: "token has no audience claim".to_string(),
        });
    }

    // Workload image digest (GCP `submods.container.image_digest`,
    // Azure CVM `x-ms-runtime`/`x-ms-sevsnpvm-...`, or a top-level digest).
    if let Some(expected) = &policy.expected_image_digest {
        let digest = claim_image_digest(payload).ok_or_else(|| AttestationError::Claim {
            provider: provider.clone(),
            reason: "expected an image-digest claim but token has none".to_string(),
        })?;
        if !digest.eq_ignore_ascii_case(expected) {
            return Err(AttestationError::Claim {
                provider: provider.clone(),
                reason: "workload image digest mismatch".to_string(),
            });
        }
    }

    // Debug mode (GCP `dbgstat == "disabled-since-boot"`, Azure
    // `x-ms-sevsnpvm-is-debuggable == false`).
    if !policy.allow_debug && claim_is_debug(payload) {
        return Err(AttestationError::Claim {
            provider: provider.clone(),
            reason: "TEE reports debug mode; rejected by policy".to_string(),
        });
    }

    // Freshness on `iat` (jsonwebtoken only enforces `exp`/`nbf`). When a max age
    // is in force, a missing `iat` is a policy failure, not a pass: `exp` alone is
    // provider-set and can sit far in the future, so without `iat` a long-lived
    // captured token would defeat the freshness bound. Real GCP/Azure tokens
    // always carry `iat`, so requiring it is a strict tightening with no false
    // positives. Mirrors the COSE path, which enforces its timestamp unconditionally.
    if let Some(max_age) = policy.max_age_secs {
        let iat =
            payload
                .get("iat")
                .and_then(Value::as_u64)
                .ok_or_else(|| AttestationError::Claim {
                    provider: provider.clone(),
                    reason: "token has no iat; cannot enforce freshness".to_string(),
                })?;
        let now = super::now_unix();
        if now.saturating_sub(iat) > max_age {
            return Err(AttestationError::Claim {
                provider: provider.clone(),
                reason: format!("attestation older than {max_age}s"),
            });
        }
    }

    Ok(())
}

/// Extract the nonce from the various claim names providers use.
fn claim_nonce(payload: &Value) -> Option<String> {
    // GCP Confidential Space: `eat_nonce` (string or array). Azure MAA: `nonce`.
    if let Some(v) = payload.get("eat_nonce") {
        return eat_nonce_to_string(v);
    }
    payload
        .get("nonce")
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn eat_nonce_to_string(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        // EAT permits an array of nonces; we bind a single one.
        Value::Array(arr) => arr.first().and_then(Value::as_str).map(str::to_string),
        _ => None,
    }
}

/// Extract the workload image digest across provider claim shapes.
fn claim_image_digest(payload: &Value) -> Option<String> {
    // GCP Confidential Space.
    if let Some(d) = payload
        .pointer("/submods/container/image_digest")
        .and_then(Value::as_str)
    {
        return Some(d.to_string());
    }
    // Generic / Azure runtime claim.
    payload
        .get("image_digest")
        .or_else(|| payload.pointer("/x-ms-runtime/image_digest"))
        .and_then(Value::as_str)
        .map(str::to_string)
}

/// Detect debug mode across provider claim shapes.
fn claim_is_debug(payload: &Value) -> bool {
    // GCP: dbgstat is "disabled-since-boot" when production.
    if let Some(dbg) = payload.get("dbgstat").and_then(Value::as_str) {
        if dbg != "disabled-since-boot" {
            return true;
        }
    }
    // Azure SEV-SNP CVM: explicit debuggable flag.
    if let Some(b) = payload
        .get("x-ms-sevsnpvm-is-debuggable")
        .and_then(Value::as_bool)
    {
        if b {
            return true;
        }
    }
    false
}

/// Extract a primary measurement to surface (best-effort across providers).
///
/// Azure SEV-SNP CVMs expose the launch measurement directly; GCP Confidential
/// Space does not surface a single platform PCR in the OIDC token, so this is
/// `None` there (image-digest binding is enforced via the claim policy instead).
fn extract_measurement(payload: &Value) -> Option<String> {
    payload
        .get("x-ms-sevsnpvm-launchmeasurement")
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

#[cfg(all(test, feature = "tee-attestation"))]
mod tests {
    use super::*;
    use serde_json::json;

    fn debug_payload() -> Value {
        json!({ "aud": "x", "dbgstat": "enabled" })
    }

    #[test]
    fn debug_mode_is_detected() {
        assert!(claim_is_debug(&debug_payload()));
        assert!(claim_is_debug(
            &json!({ "x-ms-sevsnpvm-is-debuggable": true })
        ));
        assert!(!claim_is_debug(
            &json!({ "dbgstat": "disabled-since-boot" })
        ));
    }

    #[test]
    fn nonce_extraction_covers_provider_shapes() {
        assert_eq!(
            claim_nonce(&json!({ "eat_nonce": "abc" })),
            Some("abc".to_string())
        );
        assert_eq!(
            claim_nonce(&json!({ "eat_nonce": ["abc", "def"] })),
            Some("abc".to_string())
        );
        assert_eq!(
            claim_nonce(&json!({ "nonce": "zzz" })),
            Some("zzz".to_string())
        );
        assert_eq!(claim_nonce(&json!({})), None);
    }

    #[test]
    fn image_digest_extraction() {
        let gcp = json!({ "submods": { "container": { "image_digest": "sha256:deadbeef" } } });
        assert_eq!(
            claim_image_digest(&gcp),
            Some("sha256:deadbeef".to_string())
        );
    }

    #[test]
    fn asymmetric_alg_gate() {
        assert!(is_asymmetric(Algorithm::RS256));
        assert!(is_asymmetric(Algorithm::ES256));
        assert!(!is_asymmetric(Algorithm::HS256));
    }

    #[test]
    fn claim_policy_rejects_nonce_mismatch() {
        let policy = AttestationPolicy {
            expected_nonce: Some("expected".into()),
            ..Default::default()
        };
        let payload = json!({ "aud": "x", "nonce": "actual" });
        let err = enforce_claim_policy(&CloudProvider::GCP, &payload, &policy).unwrap_err();
        assert!(matches!(err, AttestationError::Claim { .. }));
    }

    #[test]
    fn claim_policy_rejects_missing_audience() {
        let policy = AttestationPolicy::default();
        let payload = json!({ "nonce": "x" });
        let err = enforce_claim_policy(&CloudProvider::GCP, &payload, &policy).unwrap_err();
        assert!(matches!(err, AttestationError::Claim { .. }));
    }

    #[test]
    fn claim_policy_rejects_missing_iat_when_freshness_required() {
        // max_age_secs is set (production default) but the token has no `iat`:
        // freshness cannot be enforced, so the gate must fail closed rather than
        // silently skip the age bound.
        let policy = AttestationPolicy::production();
        assert!(policy.max_age_secs.is_some());
        let payload = json!({ "aud": "x" });
        let err = enforce_claim_policy(&CloudProvider::GCP, &payload, &policy).unwrap_err();
        match err {
            AttestationError::Claim { reason, .. } => {
                assert!(reason.contains("iat"), "reason was: {reason}");
            }
            other => panic!("expected Claim error, got {other:?}"),
        }
    }
}
