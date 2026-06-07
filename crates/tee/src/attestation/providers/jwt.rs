//! Shared JWT attestation-token verification.
//!
//! GCP Confidential Space and Azure MAA both emit signed JWTs. This module owns
//! signature verification, JWKS retrieval, standard claim validation, and
//! attestation-specific policy checks for those providers.

use crate::attestation::{
    AttestationClaims, AttestationError, AttestationFormat, AttestationPolicy, AttestationReport,
    Measurement, VerifiedAttestation,
};
use crate::config::TeeProvider;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde_json::Value;

const MAX_JWKS_BYTES: usize = 256 * 1024;

/// How to obtain the issuer's JWKS for a given token.
#[derive(Debug, Clone)]
pub struct JwksSource {
    /// HTTPS URL of the JWKS document.
    pub jwks_url: String,
}

impl JwksSource {
    /// Construct a JWKS source from a URL.
    pub fn new(jwks_url: impl Into<String>) -> Self {
        Self {
            jwks_url: jwks_url.into(),
        }
    }
}

/// Verify a JWT attestation token end to end.
pub async fn verify_jwt_attestation(
    provider: TeeProvider,
    format: AttestationFormat,
    token: &str,
    jwks: &JwksSource,
    allowed_issuers: &[&str],
    policy: &AttestationPolicy,
    http: &reqwest::Client,
) -> Result<VerifiedAttestation, AttestationError> {
    let provider_name = provider.to_string();

    let header = decode_header(token).map_err(|e| AttestationError::Malformed {
        provider: provider_name.clone(),
        reason: format!("invalid JWT header: {e}"),
    })?;
    let kid = header
        .kid
        .clone()
        .ok_or_else(|| AttestationError::Malformed {
            provider: provider_name.clone(),
            reason: "JWT header missing `kid`".to_string(),
        })?;

    if !is_asymmetric(header.alg) {
        return Err(AttestationError::Signature {
            provider: provider_name,
            reason: format!("refusing non-asymmetric JWT alg {:?}", header.alg),
        });
    }

    let jwk_set = fetch_jwks(provider, jwks, http).await?;
    let jwk = jwk_set.find(&kid).ok_or_else(|| AttestationError::Keys {
        provider: provider_name.clone(),
        reason: format!("no JWKS key matches kid `{kid}`"),
    })?;
    let decoding_key = DecodingKey::from_jwk(jwk).map_err(|e| AttestationError::Keys {
        provider: provider_name.clone(),
        reason: format!("invalid JWK for kid `{kid}`: {e}"),
    })?;

    let mut validation = Validation::new(header.alg);
    validation.validate_exp = true;
    validation.validate_nbf = true;
    if let Some(leeway) = policy.max_age_secs {
        validation.leeway = leeway.min(300);
    }
    if !allowed_issuers.is_empty() {
        validation.set_issuer(allowed_issuers);
    }
    match &policy.expected_audience {
        Some(aud) => validation.set_audience(&[aud.as_str()]),
        None => validation.validate_aud = false,
    }

    let decoded = decode::<Value>(token, &decoding_key, &validation).map_err(|e| {
        use jsonwebtoken::errors::ErrorKind;
        match e.kind() {
            ErrorKind::InvalidSignature => AttestationError::Signature {
                provider: provider_name.clone(),
                reason: "JWT signature does not verify against JWKS".to_string(),
            },
            ErrorKind::ExpiredSignature => AttestationError::Claim {
                provider: provider_name.clone(),
                reason: "attestation token expired".to_string(),
            },
            ErrorKind::InvalidAudience => AttestationError::Claim {
                provider: provider_name.clone(),
                reason: "attestation token audience mismatch".to_string(),
            },
            ErrorKind::InvalidIssuer => AttestationError::Claim {
                provider: provider_name.clone(),
                reason: "attestation token issuer not allowed".to_string(),
            },
            other => AttestationError::Signature {
                provider: provider_name.clone(),
                reason: format!("JWT verification failed: {other:?}"),
            },
        }
    })?;

    let payload = decoded.claims;
    enforce_claim_policy(provider, &payload, policy)?;

    let issued_at = payload.get("iat").and_then(Value::as_u64).unwrap_or(0);
    let measurement = extract_measurement(&payload)
        .map(|digest| Measurement::new("provider", digest))
        .unwrap_or_else(|| Measurement::new("unknown", ""));
    let claims = claims_from_payload(&payload);

    let report = AttestationReport {
        provider,
        format,
        issued_at_unix: issued_at,
        measurement,
        public_key_binding: None,
        claims,
        evidence: token.as_bytes().to_vec(),
    };

    Ok(VerifiedAttestation::new(report, provider))
}

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
    provider: TeeProvider,
    jwks: &JwksSource,
    http: &reqwest::Client,
) -> Result<JwkSet, AttestationError> {
    let provider_name = provider.to_string();
    let resp = http
        .get(&jwks.jwks_url)
        .send()
        .await
        .map_err(|e| AttestationError::Keys {
            provider: provider_name.clone(),
            reason: format!("JWKS fetch failed: {e}"),
        })?;
    if !resp.status().is_success() {
        return Err(AttestationError::Keys {
            provider: provider_name,
            reason: format!("JWKS endpoint returned HTTP {}", resp.status()),
        });
    }
    let body = read_body_capped(resp, MAX_JWKS_BYTES, |reason| AttestationError::Keys {
        provider: provider_name.clone(),
        reason,
    })
    .await?;
    serde_json::from_slice::<JwkSet>(&body).map_err(|e| AttestationError::Keys {
        provider: provider_name,
        reason: format!("malformed JWKS document: {e}"),
    })
}

async fn read_body_capped<F>(
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

fn enforce_claim_policy(
    provider: TeeProvider,
    payload: &Value,
    policy: &AttestationPolicy,
) -> Result<(), AttestationError> {
    let provider_name = provider.to_string();

    if let Some(expected) = &policy.expected_nonce {
        let nonce = claim_nonce(payload).ok_or_else(|| AttestationError::Claim {
            provider: provider_name.clone(),
            reason: "expected a nonce claim but token has none".to_string(),
        })?;
        if &nonce != expected {
            return Err(AttestationError::Claim {
                provider: provider_name.clone(),
                reason: "attestation nonce mismatch".to_string(),
            });
        }
    }

    if policy.expected_audience.is_none() && payload.get("aud").is_none() {
        return Err(AttestationError::Claim {
            provider: provider_name.clone(),
            reason: "token has no audience claim".to_string(),
        });
    }

    if let Some(expected) = &policy.expected_image_digest {
        let digest = claim_image_digest(payload).ok_or_else(|| AttestationError::Claim {
            provider: provider_name.clone(),
            reason: "expected an image-digest claim but token has none".to_string(),
        })?;
        if !digest.eq_ignore_ascii_case(expected) {
            return Err(AttestationError::Claim {
                provider: provider_name.clone(),
                reason: "workload image digest mismatch".to_string(),
            });
        }
    }

    if let Some(expected) = &policy.expected_measurement {
        let measurement = extract_measurement(payload).ok_or_else(|| AttestationError::Claim {
            provider: provider_name.clone(),
            reason: "expected a launch measurement claim but token has none".to_string(),
        })?;
        if !measurement.eq_ignore_ascii_case(expected) {
            return Err(AttestationError::Claim {
                provider: provider_name.clone(),
                reason: "launch measurement mismatch".to_string(),
            });
        }
    }

    if !policy.allow_debug && claim_is_debug(payload) {
        return Err(AttestationError::Claim {
            provider: provider_name.clone(),
            reason: "TEE reports debug mode; rejected by policy".to_string(),
        });
    }

    if let Some(max_age) = policy.max_age_secs {
        let iat =
            payload
                .get("iat")
                .and_then(Value::as_u64)
                .ok_or_else(|| AttestationError::Claim {
                    provider: provider_name.clone(),
                    reason: "token has no iat; cannot enforce freshness".to_string(),
                })?;
        let now = crate::attestation::policy::now_unix();
        if now.saturating_sub(iat) > max_age {
            return Err(AttestationError::Claim {
                provider: provider_name,
                reason: format!("attestation older than {max_age}s"),
            });
        }
    }

    Ok(())
}

fn claim_nonce(payload: &Value) -> Option<String> {
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
        Value::Array(arr) => arr.first().and_then(Value::as_str).map(str::to_string),
        _ => None,
    }
}

fn claim_image_digest(payload: &Value) -> Option<String> {
    if let Some(d) = payload
        .pointer("/submods/container/image_digest")
        .and_then(Value::as_str)
    {
        return Some(d.to_string());
    }
    payload
        .get("image_digest")
        .or_else(|| payload.pointer("/x-ms-runtime/image_digest"))
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn claim_is_debug(payload: &Value) -> bool {
    if let Some(dbg) = payload.get("dbgstat").and_then(Value::as_str) {
        if dbg != "disabled-since-boot" {
            return true;
        }
    }
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

fn extract_measurement(payload: &Value) -> Option<String> {
    payload
        .get("x-ms-sevsnpvm-launchmeasurement")
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn claims_from_payload(payload: &Value) -> AttestationClaims {
    let mut claims = AttestationClaims::new();
    claims.debug_mode = claim_is_debug(payload);
    for key in [
        "iss",
        "sub",
        "aud",
        "nonce",
        "eat_nonce",
        "dbgstat",
        "swname",
        "x-ms-sevsnpvm-is-debuggable",
        "x-ms-sevsnpvm-launchmeasurement",
    ] {
        if let Some(value) = payload.get(key) {
            claims.custom.insert(key.to_string(), value.clone());
        }
    }
    if let Some(value) = payload.pointer("/submods/container/image_digest") {
        claims
            .custom
            .insert("image_digest".to_string(), value.clone());
    }
    claims
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn debug_mode_is_detected() {
        assert!(claim_is_debug(&json!({ "dbgstat": "enabled" })));
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
    fn claim_policy_rejects_missing_iat_when_freshness_required() {
        let policy = AttestationPolicy::production();
        let payload = json!({ "aud": "x" });
        let err =
            enforce_claim_policy(TeeProvider::GcpConfidential, &payload, &policy).unwrap_err();
        match err {
            AttestationError::Claim { reason, .. } => {
                assert!(reason.contains("iat"), "reason was: {reason}");
            }
            other => panic!("expected Claim error, got {other:?}"),
        }
    }
}
