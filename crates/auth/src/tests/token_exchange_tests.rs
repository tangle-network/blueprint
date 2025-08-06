//! Tests for the token exchange endpoint and two-tier authentication flow

use std::collections::BTreeMap;
use std::net::Ipv4Addr;
use tempfile::tempdir;

use crate::{
    auth_token::{TokenExchangeRequest, TokenExchangeResponse},
    models::ServiceModel,
    proxy::AuthenticatedProxy,
    test_client::TestClient,
    types::ServiceId,
    types::{ChallengeRequest, ChallengeResponse, KeyType, VerifyChallengeResponse, headers},
    validation,
};

#[tokio::test]
async fn test_token_exchange_flow() {
    let _guard = tracing::subscriber::set_default(
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_line_number(true)
            .with_file(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .with_test_writer()
            .finish(),
    );

    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();

    // Create a test service
    let service_id = ServiceId::new(1);
    let mut service = ServiceModel {
        api_key_prefix: "test_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };

    let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
    let public_key = signing_key.verifying_key().to_sec1_bytes();
    service.add_owner(KeyType::Ecdsa, public_key.clone().into());
    service.save(service_id, &proxy.db()).unwrap();

    let router = proxy.router();
    let client = TestClient::new(router);

    // Step 1: Get challenge
    let challenge_req = ChallengeRequest {
        pub_key: public_key.clone().into(),
        key_type: KeyType::Ecdsa,
    };

    let res = client
        .post("/v1/auth/challenge")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&challenge_req)
        .await;

    let challenge_res: ChallengeResponse = res.json().await;

    // Step 2: Sign challenge and verify to get API key
    let (signature, _) = signing_key
        .sign_prehash_recoverable(&challenge_res.challenge)
        .unwrap();

    let mut additional_headers = BTreeMap::new();
    let user_id = "alice@example.com";
    let tenant_id = validation::hash_user_id(user_id);
    additional_headers.insert("X-Tenant-Id".to_string(), tenant_id.clone());
    additional_headers.insert("X-Tenant-Name".to_string(), "Example Corp".to_string());

    let verify_req = crate::types::VerifyChallengeRequest {
        challenge: challenge_res.challenge,
        signature: signature.to_bytes().into(),
        challenge_request: challenge_req,
        expires_at: 0,
        additional_headers,
    };

    let res = client
        .post("/v1/auth/verify")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&verify_req)
        .await;

    let verify_res: VerifyChallengeResponse = res.json().await;
    let api_key = match verify_res {
        VerifyChallengeResponse::Verified { api_key, .. } => api_key,
        _ => panic!("Expected verified response"),
    };

    // Step 3: Exchange API key for Paseto access token
    let exchange_req = TokenExchangeRequest {
        additional_headers: {
            let mut headers = BTreeMap::new();
            headers.insert("X-Custom-Header".to_string(), "custom-value".to_string());
            headers
        },
        ttl_seconds: Some(300), // 5 minutes
    };

    let res = client
        .post("/v1/auth/exchange")
        .header(headers::AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&exchange_req)
        .await;

    assert!(
        res.status().is_success(),
        "Token exchange failed: {:?}",
        res
    );
    let exchange_res: TokenExchangeResponse = res.json().await;

    // Verify response structure
    assert_eq!(exchange_res.token_type, "Bearer");
    assert!(exchange_res.access_token.starts_with("v4.local."));
    assert!(exchange_res.expires_at > 0);
    assert!(exchange_res.expires_in <= 300);

    // Step 4: Verify the Paseto token can be used directly
    // (This would be tested in reverse proxy integration tests)
    println!("Generated Paseto token: {}", exchange_res.access_token);
    println!("Expires in: {} seconds", exchange_res.expires_in);
}

#[tokio::test]
async fn test_token_exchange_invalid_api_key() {
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let router = proxy.router();
    let client = TestClient::new(router);

    let exchange_req = TokenExchangeRequest {
        additional_headers: BTreeMap::new(),
        ttl_seconds: None,
    };

    // Test with invalid API key
    let res = client
        .post("/v1/auth/exchange")
        .header(headers::AUTHORIZATION, "Bearer ak_invalid.key")
        .json(&exchange_req)
        .await;

    assert_eq!(res.status(), 401);
    let error: serde_json::Value = res.json().await;
    assert_eq!(error["error"], "invalid_api_key");
}

#[tokio::test]
async fn test_token_exchange_missing_auth_header() {
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let router = proxy.router();
    let client = TestClient::new(router);

    let exchange_req = TokenExchangeRequest {
        additional_headers: BTreeMap::new(),
        ttl_seconds: None,
    };

    let res = client.post("/v1/auth/exchange").json(&exchange_req).await;

    assert_eq!(res.status(), 401);
    let error: serde_json::Value = res.json().await;
    assert_eq!(error["error"], "missing_authorization_header");
}

#[tokio::test]
async fn test_token_exchange_with_invalid_headers() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();

    // Create service and API key
    let service_id = ServiceId::new(1);
    let mut service = ServiceModel {
        api_key_prefix: "test_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };

    let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
    let public_key = signing_key.verifying_key().to_sec1_bytes();
    service.add_owner(KeyType::Ecdsa, public_key.clone().into());
    service.save(service_id, &proxy.db()).unwrap();

    // Generate API key through normal flow
    let router = proxy.router();
    let client = TestClient::new(router);

    // Get API key (abbreviated flow)
    let challenge_req = ChallengeRequest {
        pub_key: public_key.into(),
        key_type: KeyType::Ecdsa,
    };

    let res = client
        .post("/v1/auth/challenge")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&challenge_req)
        .await;
    let challenge_res: ChallengeResponse = res.json().await;

    let (signature, _) = signing_key
        .sign_prehash_recoverable(&challenge_res.challenge)
        .unwrap();

    let verify_req = crate::types::VerifyChallengeRequest {
        challenge: challenge_res.challenge,
        signature: signature.to_bytes().into(),
        challenge_request: challenge_req,
        expires_at: 0,
        additional_headers: BTreeMap::new(),
    };

    let res = client
        .post("/v1/auth/verify")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&verify_req)
        .await;

    let verify_res: VerifyChallengeResponse = res.json().await;
    let api_key = match verify_res {
        VerifyChallengeResponse::Verified { api_key, .. } => api_key,
        _ => panic!("Expected verified response"),
    };

    // Now test exchange with forbidden headers
    let mut forbidden_headers = BTreeMap::new();
    forbidden_headers.insert("Connection".to_string(), "close".to_string()); // Forbidden hop-by-hop header

    let exchange_req = TokenExchangeRequest {
        additional_headers: forbidden_headers,
        ttl_seconds: None,
    };

    let res = client
        .post("/v1/auth/exchange")
        .header(headers::AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&exchange_req)
        .await;

    assert_eq!(res.status(), 400);
    let error: serde_json::Value = res.json().await;
    assert_eq!(error["error"], "invalid_headers");
}

#[tokio::test]
async fn test_reverse_proxy_with_paseto_token() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();

    // Create a simple test server
    let test_router = axum::Router::new().route(
        "/api/data",
        axum::routing::get(|headers: axum::http::HeaderMap| async move {
            let mut response_headers = BTreeMap::new();
            for (name, value) in headers.iter() {
                if name.as_str().starts_with("x-tenant-") {
                    response_headers
                        .insert(name.to_string(), value.to_str().unwrap_or("").to_string());
                }
            }
            axum::Json(response_headers)
        }),
    );

    let (test_server, test_addr) = {
        let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .await
            .expect("Failed to bind test server");
        let server = axum::serve(listener, test_router);
        let local_address = server.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            if let Err(e) = server.await {
                eprintln!("Test server error: {}", e);
            }
        });
        (handle, local_address)
    };

    // Create service pointing to test server
    let service_id = ServiceId::new(1);
    let mut service = ServiceModel {
        api_key_prefix: "test_".to_string(),
        owners: Vec::new(),
        upstream_url: format!("http://localhost:{}", test_addr.port()),
    };

    let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
    let public_key = signing_key.verifying_key().to_sec1_bytes();
    service.add_owner(KeyType::Ecdsa, public_key.clone().into());
    service.save(service_id, &proxy.db()).unwrap();

    let router = proxy.router();
    let client = TestClient::new(router);

    // Get API key (full flow)
    let challenge_req = ChallengeRequest {
        pub_key: public_key.into(),
        key_type: KeyType::Ecdsa,
    };

    let res = client
        .post("/v1/auth/challenge")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&challenge_req)
        .await;
    let challenge_res: ChallengeResponse = res.json().await;

    let (signature, _) = signing_key
        .sign_prehash_recoverable(&challenge_res.challenge)
        .unwrap();

    let mut tenant_headers = BTreeMap::new();
    // Use already hashed tenant ID (32 hex chars) to avoid re-hashing
    let tenant_id = crate::validation::hash_user_id("tenant123");
    tenant_headers.insert("X-Tenant-Id".to_string(), tenant_id.clone());
    tenant_headers.insert("X-Tenant-Name".to_string(), "Test Corp".to_string());

    let verify_req = crate::types::VerifyChallengeRequest {
        challenge: challenge_res.challenge,
        signature: signature.to_bytes().into(),
        challenge_request: challenge_req,
        expires_at: 0,
        additional_headers: tenant_headers,
    };

    let res = client
        .post("/v1/auth/verify")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&verify_req)
        .await;

    let verify_res: VerifyChallengeResponse = res.json().await;
    let api_key = match verify_res {
        VerifyChallengeResponse::Verified { api_key, .. } => api_key,
        _ => panic!("Expected verified response"),
    };

    // Exchange for Paseto token
    let exchange_req = TokenExchangeRequest {
        additional_headers: {
            let mut headers = BTreeMap::new();
            headers.insert("X-Tenant-Role".to_string(), "admin".to_string());
            headers
        },
        ttl_seconds: Some(60),
    };

    let res = client
        .post("/v1/auth/exchange")
        .header(headers::AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&exchange_req)
        .await;

    assert!(res.status().is_success());
    let exchange_res: TokenExchangeResponse = res.json().await;

    // Use Paseto token with reverse proxy
    let res = client
        .get("/api/data")
        .header(
            headers::AUTHORIZATION,
            format!("Bearer {}", exchange_res.access_token),
        )
        .await;

    assert!(
        res.status().is_success(),
        "Paseto token request failed: {:?}",
        res
    );
    let response_headers: BTreeMap<String, String> = res.json().await;

    // Verify headers were forwarded (should include both original + additional)
    assert_eq!(response_headers.get("x-tenant-id"), Some(&tenant_id));
    assert_eq!(
        response_headers.get("x-tenant-name"),
        Some(&"Test Corp".to_string())
    );
    assert_eq!(
        response_headers.get("x-tenant-role"),
        Some(&"admin".to_string())
    );

    test_server.abort();
}
