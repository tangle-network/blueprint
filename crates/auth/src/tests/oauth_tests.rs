#![cfg(test)]

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use openssl::rsa::Rsa;
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::oauth::ServiceOAuthPolicy;
use crate::proxy::AuthenticatedProxy;
use crate::types::ServiceId;

struct RsaMaterial {
    der: Vec<u8>,
    public_pem: String,
}

static RSA_MATERIAL: OnceLock<RsaMaterial> = OnceLock::new();

fn rsa_material() -> &'static RsaMaterial {
    RSA_MATERIAL.get_or_init(|| {
        let rsa = Rsa::generate(2048).unwrap();
        let der = rsa.private_key_to_der().unwrap();
        // Provide PKCS#1 public key as well (BEGIN RSA PUBLIC KEY)
        let public_pem = String::from_utf8(rsa.public_key_to_pem_pkcs1().unwrap()).unwrap();
        RsaMaterial { der, public_pem }
    })
}

fn rsa_encoding_key() -> EncodingKey {
    EncodingKey::from_rsa_der(&rsa_material().der)
}

fn rsa_public_pem() -> String {
    rsa_material().public_pem.clone()
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn start_proxy_with_policy(policy: ServiceOAuthPolicy) -> (SocketAddr, ServiceId) {
    use tempfile::tempdir;

    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let db = proxy.db();

    let service_id = ServiceId::new(7);
    let service = crate::models::ServiceModel {
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
        public_keys_pem: vec![rsa_public_pem()],
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
    let jwt = encode(&Header::new(Algorithm::RS256), &claims, &rsa_encoding_key()).unwrap();

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
    let status = res.status();
    let body = res.text().await.unwrap();
    assert!(status.is_success(), "status={} body={}", status, body);
    assert!(body.contains("access_token"));
}

#[tokio::test]
async fn oauth_rejects_unsupported_alg_hs256() {
    let policy = ServiceOAuthPolicy {
        allowed_issuers: vec!["https://issuer.example.com".into()],
        required_audiences: vec![],
        public_keys_pem: vec![rsa_public_pem()],
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
        public_keys_pem: vec![rsa_public_pem()],
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
        &rsa_encoding_key(),
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
    let jwt_future = encode(&Header::new(Algorithm::RS256), &future, &rsa_encoding_key()).unwrap();
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
        public_keys_pem: vec![rsa_public_pem()],
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
    let jwt = encode(&Header::new(Algorithm::RS256), &claims, &rsa_encoding_key()).unwrap();

    let body = format!("grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={jwt}");
    let ok = client
        .post(format!("http://{}/v1/oauth/token", addr))
        .header("content-type", "application/x-www-form-urlencoded")
        .header("x-service-id", service_id.to_string())
        .body(body.clone())
        .send()
        .await
        .unwrap();
    let s = ok.status();
    let b = ok.text().await.unwrap();
    assert!(s.is_success(), "status={} body={}", s, b);
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

#[derive(serde::Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_at: u64,
    expires_in: u64,
}

#[tokio::test]
async fn oauth_scopes_are_forwarded_and_normalized_and_client_scopes_stripped() {
    use axum::{Json, Router, routing::get};
    use std::collections::BTreeMap;
    use std::net::Ipv4Addr;
    use tempfile::tempdir;

    // Upstream echo server that returns all headers as JSON (lowercased keys)
    let echo_router = Router::new().route(
        "/echo",
        get(|headers: axum::http::HeaderMap| async move {
            let mut map = BTreeMap::new();
            for (name, value) in headers.iter() {
                let k = name.as_str().to_ascii_lowercase();
                let v = value.to_str().unwrap_or("").to_string();
                map.insert(k, v);
            }
            Json(map)
        }),
    );
    let (echo_task, echo_addr) = {
        let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .await
            .expect("bind echo");
        let addr = listener.local_addr().unwrap();
        let task = tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, echo_router).await {
                eprintln!("echo server error: {e}");
            }
        });
        (task, addr)
    };

    // Start proxy with service pointing to echo server
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let db = proxy.db();
    let service_id = ServiceId::new(8);
    let service = crate::models::ServiceModel {
        api_key_prefix: "test_".to_string(),
        owners: vec![],
        upstream_url: format!("http://{}", echo_addr),
    };
    service.save(service_id, &db).unwrap();
    let policy = ServiceOAuthPolicy {
        allowed_issuers: vec!["https://issuer.example.com".into()],
        required_audiences: vec!["https://proxy.example.com".into()],
        public_keys_pem: vec![rsa_public_pem()],
        allowed_scopes: Some(vec!["data:read".into(), "mcp:invoke".into()]),
        require_dpop: false,
        max_access_token_ttl_secs: 900,
        max_assertion_ttl_secs: 120,
    };
    policy.save(service_id, &db).unwrap();

    let app = proxy.router();
    let listener = std::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    listener.set_nonblocking(true).unwrap();
    let proxy_addr = listener.local_addr().unwrap();
    let tcp = tokio::net::TcpListener::from_std(listener).unwrap();
    tokio::spawn(async move {
        axum::serve(tcp, app).await.unwrap();
    });

    // Mint OAuth assertion containing messy/mixed scopes; intersection should yield [data:read, mcp:invoke]
    let now = now();
    let claims = Claims {
        iss: "https://issuer.example.com".into(),
        sub: "user-42".into(),
        aud: Some("https://proxy.example.com".into()),
        iat: now,
        exp: now + 60,
        jti: uuid::Uuid::new_v4().to_string(),
        scope: Some("DATA:READ data:read extra:skip Mcp:InvokE".into()),
    };
    let jwt = encode(&Header::new(Algorithm::RS256), &claims, &rsa_encoding_key()).unwrap();

    // Exchange for Paseto
    let client = reqwest::Client::new();
    let token_res = client
        .post(format!("http://{}/v1/oauth/token", proxy_addr))
        .header("content-type", "application/x-www-form-urlencoded")
        .header("x-service-id", service_id.to_string())
        .body(format!(
            "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={jwt}"
        ))
        .send()
        .await
        .unwrap();
    let status = token_res.status();
    assert!(status.is_success());
    let token_body = token_res.text().await.unwrap();
    let token: TokenResponse = serde_json::from_str(&token_body).unwrap();

    // Call upstream via proxy with malicious client x-scopes header; it must be stripped and replaced by canonical
    let res = client
        .get(format!("http://{}/echo", proxy_addr))
        .header("authorization", format!("Bearer {}", token.access_token))
        .header("x-scopes", "evil:root")
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());

    let echoed: BTreeMap<String, String> = res.json().await.unwrap();
    // Expect normalized, deduped scopes injected by proxy
    assert_eq!(
        echoed.get("x-scopes").cloned(),
        Some("data:read mcp:invoke".to_string())
    );
    // Ensure lowercased header names
    assert!(echoed.get("X-Scopes").is_none());

    // Shutdown echo server task
    drop(echo_task);
}

#[tokio::test]
async fn oauth_scopes_absent_when_not_allowed_and_client_header_stripped() {
    use axum::{Json, Router, routing::get};
    use std::collections::BTreeMap;
    use std::net::Ipv4Addr;
    use tempfile::tempdir;

    // Upstream echo
    let echo_router = Router::new().route(
        "/echo",
        get(|headers: axum::http::HeaderMap| async move {
            let mut map = BTreeMap::new();
            for (name, value) in headers.iter() {
                map.insert(
                    name.as_str().to_ascii_lowercase(),
                    value.to_str().unwrap_or("").to_string(),
                );
            }
            Json(map)
        }),
    );
    let (echo_task, echo_addr) = {
        let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .await
            .expect("bind echo");
        let addr = listener.local_addr().unwrap();
        let task = tokio::spawn(async move { axum::serve(listener, echo_router).await.unwrap() });
        (task, addr)
    };

    // Proxy + service with no allowed_scopes
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let db = proxy.db();
    let service_id = ServiceId::new(9);
    let service = crate::models::ServiceModel {
        api_key_prefix: "test_".to_string(),
        owners: vec![],
        upstream_url: format!("http://{}", echo_addr),
    };
    service.save(service_id, &db).unwrap();
    let policy = ServiceOAuthPolicy {
        allowed_issuers: vec!["https://issuer.example.com".into()],
        required_audiences: vec![],
        public_keys_pem: vec![rsa_public_pem()],
        allowed_scopes: None, // scopes not allowed -> should not be forwarded
        require_dpop: false,
        max_access_token_ttl_secs: 900,
        max_assertion_ttl_secs: 120,
    };
    policy.save(service_id, &db).unwrap();

    let app = proxy.router();
    let listener = std::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    listener.set_nonblocking(true).unwrap();
    let proxy_addr = listener.local_addr().unwrap();
    let tcp = tokio::net::TcpListener::from_std(listener).unwrap();
    tokio::spawn(async move { axum::serve(tcp, app).await.unwrap() });

    // Assertion with a scope but policy disallows -> Paseto will carry None, proxy must not inject x-scopes
    let now = now();
    let claims = Claims {
        iss: "https://issuer.example.com".into(),
        sub: "user-7".into(),
        aud: None,
        iat: now,
        exp: now + 60,
        jti: uuid::Uuid::new_v4().to_string(),
        scope: Some("logs:read".into()),
    };
    let jwt = encode(&Header::new(Algorithm::RS256), &claims, &rsa_encoding_key()).unwrap();

    let client = reqwest::Client::new();
    let token_res = client
        .post(format!("http://{}/v1/oauth/token", proxy_addr))
        .header("content-type", "application/x-www-form-urlencoded")
        .header("x-service-id", service_id.to_string())
        .body(format!(
            "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={jwt}"
        ))
        .send()
        .await
        .unwrap();
    assert!(token_res.status().is_success());
    let token: TokenResponse = token_res.json().await.unwrap();

    let res = client
        .get(format!("http://{}/echo", proxy_addr))
        .header("authorization", format!("Bearer {}", token.access_token))
        .header("x-scopes", "logs:admin") // should be stripped, not forwarded
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let echoed: BTreeMap<String, String> = res.json().await.unwrap();
    assert!(
        echoed.get("x-scopes").is_none(),
        "x-scopes must not be forwarded when policy disallows scopes"
    );

    drop(echo_task);
}
