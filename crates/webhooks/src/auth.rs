//! Webhook authentication verification.

use crate::config::WebhookEndpoint;
use crate::error::WebhookError;
use axum::http::HeaderMap;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;

type HmacSha256 = Hmac<Sha256>;

/// Verify a webhook request against the endpoint's auth configuration.
///
/// Returns `Ok(())` if the request is authorized, or an error describing the failure.
pub fn verify(
    endpoint: &WebhookEndpoint,
    headers: &HeaderMap,
    body: &[u8],
) -> Result<(), WebhookError> {
    match endpoint.auth.as_str() {
        "none" => Ok(()),
        "bearer" => verify_bearer(endpoint, headers),
        "hmac-sha256" => verify_hmac(endpoint, headers, body),
        "api-key" => verify_api_key(endpoint, headers),
        other => Err(WebhookError::AuthFailed(format!(
            "unknown auth method: {other}"
        ))),
    }
}

/// Bearer token: `Authorization: Bearer <token>`
fn verify_bearer(endpoint: &WebhookEndpoint, headers: &HeaderMap) -> Result<(), WebhookError> {
    let expected = endpoint
        .resolve_secret()
        .ok_or_else(|| WebhookError::AuthFailed("bearer secret not configured".into()))?;

    let header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| WebhookError::AuthFailed("missing Authorization header".into()))?;

    let token = header
        .strip_prefix("Bearer ")
        .ok_or_else(|| WebhookError::AuthFailed("Authorization must be 'Bearer <token>'".into()))?;

    if token.as_bytes().ct_eq(expected.as_bytes()).into() {
        Ok(())
    } else {
        Err(WebhookError::AuthFailed("invalid bearer token".into()))
    }
}

/// HMAC-SHA256: signature in a header, computed over the raw body.
///
/// Checks these headers in order: `X-Signature-256`, `X-Hub-Signature-256`,
/// `X-Webhook-Signature`. The signature value should be hex-encoded, optionally
/// prefixed with `sha256=`.
fn verify_hmac(
    endpoint: &WebhookEndpoint,
    headers: &HeaderMap,
    body: &[u8],
) -> Result<(), WebhookError> {
    let secret = endpoint
        .resolve_secret()
        .ok_or_else(|| WebhookError::AuthFailed("HMAC secret not configured".into()))?;

    // Find the signature header
    let sig_header = ["x-signature-256", "x-hub-signature-256", "x-webhook-signature"]
        .iter()
        .find_map(|name| headers.get(*name).and_then(|v| v.to_str().ok()))
        .ok_or_else(|| {
            WebhookError::AuthFailed(
                "missing signature header (X-Signature-256, X-Hub-Signature-256, or X-Webhook-Signature)".into(),
            )
        })?;

    // Strip optional `sha256=` prefix
    let sig_hex = sig_header.strip_prefix("sha256=").unwrap_or(sig_header);

    let sig_bytes = hex::decode(sig_hex)
        .map_err(|_| WebhookError::AuthFailed("signature is not valid hex".into()))?;

    // Compute expected HMAC
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|e| WebhookError::AuthFailed(format!("HMAC init failed: {e}")))?;
    mac.update(body);
    let expected = mac.finalize().into_bytes();

    if expected.as_slice().ct_eq(&sig_bytes).into() {
        Ok(())
    } else {
        Err(WebhookError::AuthFailed("HMAC signature mismatch".into()))
    }
}

/// API key: custom header with a static key value.
fn verify_api_key(endpoint: &WebhookEndpoint, headers: &HeaderMap) -> Result<(), WebhookError> {
    let expected = endpoint
        .resolve_secret()
        .ok_or_else(|| WebhookError::AuthFailed("API key not configured".into()))?;

    let header_name = endpoint.api_key_header.as_deref().unwrap_or("x-api-key");

    let provided = headers
        .get(header_name)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| WebhookError::AuthFailed(format!("missing {header_name} header")))?;

    if provided.as_bytes().ct_eq(expected.as_bytes()).into() {
        Ok(())
    } else {
        Err(WebhookError::AuthFailed("invalid API key".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::WebhookEndpoint;
    use axum::http::HeaderMap;

    fn make_endpoint(auth: &str, secret: &str) -> WebhookEndpoint {
        WebhookEndpoint {
            path: "/test".into(),
            job_id: 1,
            auth: auth.into(),
            secret: Some(secret.into()),
            api_key_header: None,
            name: None,
        }
    }

    #[test]
    fn test_no_auth() {
        let ep = WebhookEndpoint {
            path: "/test".into(),
            job_id: 1,
            auth: "none".into(),
            secret: None,
            api_key_header: None,
            name: None,
        };
        assert!(verify(&ep, &HeaderMap::new(), b"").is_ok());
    }

    #[test]
    fn test_bearer_valid() {
        let ep = make_endpoint("bearer", "my-token");
        let mut headers = HeaderMap::new();
        headers.insert("authorization", "Bearer my-token".parse().unwrap());
        assert!(verify(&ep, &headers, b"").is_ok());
    }

    #[test]
    fn test_bearer_invalid() {
        let ep = make_endpoint("bearer", "my-token");
        let mut headers = HeaderMap::new();
        headers.insert("authorization", "Bearer wrong-token".parse().unwrap());
        assert!(verify(&ep, &headers, b"").is_err());
    }

    #[test]
    fn test_bearer_missing_header() {
        let ep = make_endpoint("bearer", "my-token");
        assert!(verify(&ep, &HeaderMap::new(), b"").is_err());
    }

    #[test]
    fn test_hmac_valid() {
        let ep = make_endpoint("hmac-sha256", "secret-key");
        let body = b"hello world";

        let mut mac = HmacSha256::new_from_slice(b"secret-key").unwrap();
        mac.update(body);
        let sig = hex::encode(mac.finalize().into_bytes());

        let mut headers = HeaderMap::new();
        headers.insert("x-signature-256", format!("sha256={sig}").parse().unwrap());
        assert!(verify(&ep, &headers, body).is_ok());
    }

    #[test]
    fn test_hmac_valid_no_prefix() {
        let ep = make_endpoint("hmac-sha256", "secret-key");
        let body = b"test";

        let mut mac = HmacSha256::new_from_slice(b"secret-key").unwrap();
        mac.update(body);
        let sig = hex::encode(mac.finalize().into_bytes());

        let mut headers = HeaderMap::new();
        headers.insert("x-webhook-signature", sig.parse().unwrap());
        assert!(verify(&ep, &headers, body).is_ok());
    }

    #[test]
    fn test_hmac_invalid_signature() {
        let ep = make_endpoint("hmac-sha256", "secret-key");
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-signature-256",
            "sha256=0000000000000000000000000000000000000000000000000000000000000000"
                .parse()
                .unwrap(),
        );
        assert!(verify(&ep, &headers, b"hello").is_err());
    }

    #[test]
    fn test_api_key_valid() {
        let ep = make_endpoint("api-key", "my-api-key");
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", "my-api-key".parse().unwrap());
        assert!(verify(&ep, &headers, b"").is_ok());
    }

    #[test]
    fn test_api_key_custom_header() {
        let mut ep = make_endpoint("api-key", "my-key");
        ep.api_key_header = Some("X-Custom-Auth".into());
        let mut headers = HeaderMap::new();
        headers.insert("x-custom-auth", "my-key".parse().unwrap());
        assert!(verify(&ep, &headers, b"").is_ok());
    }

    #[test]
    fn test_api_key_invalid() {
        let ep = make_endpoint("api-key", "correct-key");
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", "wrong-key".parse().unwrap());
        assert!(verify(&ep, &headers, b"").is_err());
    }
}
