#![cfg(test)]

use axum::{Router, routing::any};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::Serialize;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use blueprint_auth::oauth::ServiceOAuthPolicy;
use blueprint_auth::proxy::AuthenticatedProxy;
use blueprint_auth::types::ServiceId;

static RSA_PRIVATE_PEM: &str = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAuJQW+2m5PpVi9r6Qq6iM0jJr6m2s0N9VdR1j4Qv0i9dVJc3C
1kqkq5FzZCk5c7K5d6Q2vGx2c1o9z2oJmKz3j0qf5r7b6x2o1u2W3E4F5G6H7I8J
J0aZQ3w3lKz6n0m9r4v7w5x6y7z8A9B0C1D2E3F4G5H6I7J8K9L0M1N2O3P4Q5R6
S7T8U9V0W1X2Y3Z4a5b6c7d8e9f0g1h2i3j4k5l6m7n8o9p0q1r2s3t4u5v6w7x8
y9z0A1B2C3D4E5F6G7H8I9J0K1L2M3N4O5P6Q7R8S9T0U1V2W3X4Y5Z6a7b8c9d0
e1f2g3h4i5j6k7l8m9n0wIDAQABAoIBAEv6cUj1rFJtqYz6k0c1z8lyv8v7sGfF
...TRUNCATED FOR BREVITY...
-----END RSA PRIVATE KEY-----"#;

static RSA_PUBLIC_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAuJQW+2m5PpVi9r6Qq6iM
0jJr6m2s0N9VdR1j4Qv0i9dVJc3C1kqkq5FzZCk5c7K5d6Q2vGx2c1o9z2oJmKz3
j0qf5r7b6x2o1u2W3E4F5G6H7I8JJ0aZQ3w3lKz6n0m9r4v7w5x6y7z8A9B0C1D2
E3F4G5H6I7J8K9L0M1N2O3P4Q5R6S7T8U9V0W1X2Y3Z4a5b6c7d8e9f0g1h2i3j4
k5l6m7n8o9p0q1r2s3t4u5v6w7x8y9z0A1B2C3D4E5F6G7H8I9J0K1L2M3N4O5P6
Q7R8S9T0U1V2W3X4Y5Z6a7b8c9d0e1f2wIDAQAB
-----END PUBLIC KEY-----"#;

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn start_proxy_with_policy(policy: ServiceOAuthPolicy) -> (SocketAddr, ServiceId) {
    use tempfile::tempdir;
    use tower::ServiceBuilder;

    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let db = proxy.db();

    let service_id = ServiceId::new(7);
    let mut service = blueprint_auth::models::ServiceModel {
        api_key_prefix: "test_".to_string(),
        owners: vec![],
        upstream_url: "http://127.0.0.1:9".to_string(),
    };
    service.save(service_id, &db).unwrap();
    policy.save(service_id, &db).unwrap();

    let app = proxy.router();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let addr = listener.local_addr().unwrap();
    let tcp = tokio::net::TcpListener::from_std(listener).unwrap();
    tokio::spawn(async move {
        axum::serve(tcp, app).await.unwrap();
    });
    (addr, service_id)
}

#[derive(Serialize)]
struct Claims {
    iss: String,
    sub: String,
    aud: Option<String>,
    iat: u64,
    exp: u64,
    jti: String,
    scope: Option<String>,
}

#[tokio::test]
async fn oauth_success_rs256() {
    let policy = ServiceOAuthPolicy {
        allowed_issuers: vec!["https://issuer.example.com".into()],
        required_audiences: vec!["https://proxy.example.com".into()],
        public_keys_pem: vec![RSA_PUBLIC_PEM.into()],
        allowed_scopes: Some(vec!["data:read".into(), "data:write".into()]),
        require_dpop: false,
        max_access_token_ttl_secs: 900,
        max_assertion_ttl_secs: 120,
    };
    let (addr, service_id) = start_proxy_with_policy(policy);

    let now = now();
    let claims = Claims {
        iss: "https://issuer.example.com".into(),
        sub: "user-1".into(),
        aud: Some("https://proxy.example.com".into()),
        iat: now,
        exp: now + 60,
        jti: uuid::Uuid::new_v4().to_string(),
        scope: Some("data:read extra:skip".into()),
    };
    let jwt = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(RSA_PRIVATE_PEM.as_bytes()).unwrap(),
    )
    .unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/v1/oauth/token", addr))
        .header("content-type", "application/x-www-form-urlencoded")
        .header("x-service-id", service_id.to_string())
        .body(format!(
            "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={jwt}"
        ))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let body = res.text().await.unwrap();
    assert!(body.contains("access_token"));
}

#[tokio::test]
async fn oauth_rejects_unsupported_alg_hs256() {
    let policy = ServiceOAuthPolicy {
        allowed_issuers: vec!["https://issuer.example.com".into()],
        required_audiences: vec![],
        public_keys_pem: vec![RSA_PUBLIC_PEM.into()],
        allowed_scopes: None,
        require_dpop: false,
        max_access_token_ttl_secs: 900,
        max_assertion_ttl_secs: 120,
    };
    let (addr, service_id) = start_proxy_with_policy(policy);

    #[derive(Serialize)]
    struct HsClaims {
        iss: String,
        sub: String,
        iat: u64,
        exp: u64,
        jti: String,
    }
    let now = now();
    let claims = HsClaims {
        iss: "https://issuer.example.com".into(),
        sub: "u".into(),
        iat: now,
        exp: now + 60,
        jti: uuid::Uuid::new_v4().to_string(),
    };
    let jwt = jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(b"secret"),
    )
    .unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/v1/oauth/token", addr))
        .header("content-type", "application/x-www-form-urlencoded")
        .header("x-service-id", service_id.to_string())
        .body(format!(
            "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={jwt}"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn oauth_rejects_expired_and_future_iat() {
    let policy = ServiceOAuthPolicy {
        allowed_issuers: vec!["https://issuer.example.com".into()],
        required_audiences: vec![],
        public_keys_pem: vec![RSA_PUBLIC_PEM.into()],
        allowed_scopes: None,
        require_dpop: false,
        max_access_token_ttl_secs: 900,
        max_assertion_ttl_secs: 60,
    };
    let (addr, service_id) = start_proxy_with_policy(policy);
    let client = reqwest::Client::new();

    // expired
    let now = now();
    let expired = Claims {
        iss: "https://issuer.example.com".into(),
        sub: "u".into(),
        aud: None,
        iat: now - 120,
        exp: now - 60,
        jti: uuid::Uuid::new_v4().to_string(),
        scope: None,
    };
    let jwt_expired = encode(
        &Header::new(Algorithm::RS256),
        &expired,
        &EncodingKey::from_rsa_pem(RSA_PRIVATE_PEM.as_bytes()).unwrap(),
    )
    .unwrap();
    let res = client
        .post(format!("http://{}/v1/oauth/token", addr))
        .header("content-type", "application/x-www-form-urlencoded")
        .header("x-service-id", service_id.to_string())
        .body(format!(
            "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={jwt_expired}"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), axum::http::StatusCode::BAD_REQUEST);

    // future iat (beyond skew)
    let future = Claims {
        iss: "https://issuer.example.com".into(),
        sub: "u".into(),
        aud: None,
        iat: now + 600,
        exp: now + 660,
        jti: uuid::Uuid::new_v4().to_string(),
        scope: None,
    };
    let jwt_future = encode(
        &Header::new(Algorithm::RS256),
        &future,
        &EncodingKey::from_rsa_pem(RSA_PRIVATE_PEM.as_bytes()).unwrap(),
    )
    .unwrap();
    let res2 = client
        .post(format!("http://{}/v1/oauth/token", addr))
        .header("content-type", "application/x-www-form-urlencoded")
        .header("x-service-id", service_id.to_string())
        .body(format!(
            "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={jwt_future}"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(res2.status(), axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn oauth_rejects_replay_jti() {
    let policy = ServiceOAuthPolicy {
        allowed_issuers: vec!["https://issuer.example.com".into()],
        required_audiences: vec![],
        public_keys_pem: vec![RSA_PUBLIC_PEM.into()],
        allowed_scopes: None,
        require_dpop: false,
        max_access_token_ttl_secs: 900,
        max_assertion_ttl_secs: 120,
    };
    let (addr, service_id) = start_proxy_with_policy(policy);
    let client = reqwest::Client::new();

    let now = now();
    let jti = uuid::Uuid::new_v4().to_string();
    let claims = Claims {
        iss: "https://issuer.example.com".into(),
        sub: "u".into(),
        aud: None,
        iat: now,
        exp: now + 60,
        jti: jti.clone(),
        scope: None,
    };
    let jwt = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(RSA_PRIVATE_PEM.as_bytes()).unwrap(),
    )
    .unwrap();

    let body = format!("grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={jwt}");
    let ok = client
        .post(format!("http://{}/v1/oauth/token", addr))
        .header("content-type", "application/x-www-form-urlencoded")
        .header("x-service-id", service_id.to_string())
        .body(body.clone())
        .send()
        .await
        .unwrap();
    assert!(ok.status().is_success());
    let replay = client
        .post(format!("http://{}/v1/oauth/token", addr))
        .header("content-type", "application/x-www-form-urlencoded")
        .header("x-service-id", service_id.to_string())
        .body(body)
        .send()
        .await
        .unwrap();
    assert_eq!(replay.status(), axum::http::StatusCode::BAD_REQUEST);
}
