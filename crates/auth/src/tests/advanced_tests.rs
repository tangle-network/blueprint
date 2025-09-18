//! Advanced tests for Paseto persistence, service deletion, header validation, and PII protection

use std::collections::BTreeMap;
use std::time::Duration;
use tempfile::tempdir;

use crate::{
    auth_token::{TokenExchangeRequest, TokenExchangeResponse},
    models::ServiceModel,
    paseto_tokens::{PasetoKey, PasetoTokenManager},
    proxy::AuthenticatedProxy,
    test_client::TestClient,
    types::{
        ChallengeRequest, ChallengeResponse, KeyType, ServiceId, VerifyChallengeResponse, headers,
    },
    validation,
};

/// Test Paseto key persistence across restarts
#[tokio::test]
async fn test_paseto_key_persistence() {
    let tmp = tempdir().unwrap();
    let db_path = tmp.path();

    // Create first instance with a Paseto manager
    let proxy1 = AuthenticatedProxy::new(db_path).unwrap();
    let db1 = proxy1.db();

    // Create service
    let service_id = ServiceId::new(1);
    let service = ServiceModel {
        api_key_prefix: "pst_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };
    service.save(service_id, &db1).unwrap();

    // Generate a Paseto token
    let manager1 = PasetoTokenManager::new(Duration::from_secs(900));
    let token1 = manager1
        .generate_token(
            service_id,
            "test_key_id".to_string(),
            Some("tenant123".to_string()),
            BTreeMap::new(),
            None,
            None,
        )
        .unwrap();

    // Get the key bytes for comparison
    let key_bytes1 = manager1.get_key().as_bytes();

    // Simulate restart by creating new instance with same key
    let key = PasetoKey::from_bytes(key_bytes1.clone().try_into().unwrap());
    let manager2 = PasetoTokenManager::with_key(key, Duration::from_secs(900));

    // Should be able to validate token from first manager
    let claims = manager2.validate_token(&token1).unwrap();
    assert_eq!(claims.service_id, service_id);
    assert_eq!(claims.tenant_id, Some("tenant123".to_string()));

    // Generate new token with second manager
    let token2 = manager2
        .generate_token(
            service_id,
            "test_key_id2".to_string(),
            Some("tenant456".to_string()),
            BTreeMap::new(),
            None,
            None,
        )
        .unwrap();

    // First manager with same key should validate second token
    let manager1_copy = PasetoTokenManager::with_key(
        PasetoKey::from_bytes(key_bytes1.clone().try_into().unwrap()),
        Duration::from_secs(900),
    );
    let claims2 = manager1_copy.validate_token(&token2).unwrap();
    assert_eq!(claims2.tenant_id, Some("tenant456".to_string()));
}

/// Test that service deletion invalidates all associated tokens
#[tokio::test]
async fn test_service_deletion_impact() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let db = proxy.db();

    // Create two services
    let service1_id = ServiceId::new(1);
    let service2_id = ServiceId::new(2);

    let mut service1 = ServiceModel {
        api_key_prefix: "svc1_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8081".to_string(),
    };

    let mut service2 = ServiceModel {
        api_key_prefix: "svc2_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8082".to_string(),
    };

    // Add owners to both services
    let signing_key1 = k256::ecdsa::SigningKey::random(&mut rng);
    let public_key1 = signing_key1.verifying_key().to_sec1_bytes();
    service1.add_owner(KeyType::Ecdsa, public_key1.clone().into());
    service1.save(service1_id, &db).unwrap();

    let signing_key2 = k256::ecdsa::SigningKey::random(&mut rng);
    let public_key2 = signing_key2.verifying_key().to_sec1_bytes();
    service2.add_owner(KeyType::Ecdsa, public_key2.clone().into());
    service2.save(service2_id, &db).unwrap();

    let router = proxy.router();
    let client = TestClient::new(router);

    // Get API key for service 1
    let challenge_req1 = ChallengeRequest {
        pub_key: public_key1.into(),
        key_type: KeyType::Ecdsa,
    };

    let res = client
        .post("/v1/auth/challenge")
        .header(headers::X_SERVICE_ID, service1_id.to_string())
        .json(&challenge_req1)
        .await;
    let challenge_res1: ChallengeResponse = res.json().await;

    let (signature1, _) = signing_key1
        .sign_prehash_recoverable(&challenge_res1.challenge)
        .unwrap();

    let verify_req1 = crate::types::VerifyChallengeRequest {
        challenge: challenge_res1.challenge,
        signature: signature1.to_bytes().into(),
        challenge_request: challenge_req1,
        expires_at: 0,
        additional_headers: BTreeMap::new(),
    };

    let res = client
        .post("/v1/auth/verify")
        .header(headers::X_SERVICE_ID, service1_id.to_string())
        .json(&verify_req1)
        .await;

    let verify_res1: VerifyChallengeResponse = res.json().await;
    let api_key1 = match verify_res1 {
        VerifyChallengeResponse::Verified { api_key, .. } => api_key,
        _ => panic!("Expected verified response"),
    };

    // Verify service 1 key works
    let res = client
        .get("/test")
        .header(headers::AUTHORIZATION, format!("Bearer {api_key1}"))
        .await;
    assert_ne!(res.status(), 401, "Service 1 key should work");

    // Delete service 1
    ServiceModel::delete(service1_id, &db).unwrap();

    // Service 1 key should no longer work
    let res = client
        .get("/test")
        .header(headers::AUTHORIZATION, format!("Bearer {api_key1}"))
        .await;
    assert_eq!(
        res.status(),
        404,
        "Service 1 key should fail after deletion"
    );

    // Service 2 should still work (get key first)
    let challenge_req2 = ChallengeRequest {
        pub_key: public_key2.into(),
        key_type: KeyType::Ecdsa,
    };

    let res = client
        .post("/v1/auth/challenge")
        .header(headers::X_SERVICE_ID, service2_id.to_string())
        .json(&challenge_req2)
        .await;
    assert!(res.status().is_success(), "Service 2 should still work");
}

/// Test maximum header count and size validation in production flow
#[tokio::test]
async fn test_max_header_validation_production() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();

    let service_id = ServiceId::new(1);
    let mut service = ServiceModel {
        api_key_prefix: "hdr_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };

    let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
    let public_key = signing_key.verifying_key().to_sec1_bytes();
    service.add_owner(KeyType::Ecdsa, public_key.clone().into());
    service.save(service_id, &proxy.db()).unwrap();

    let router = proxy.router();
    let client = TestClient::new(router);

    // Test 1: Too many headers (more than 8)
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

    let (signature, _) = signing_key
        .sign_prehash_recoverable(&challenge_res.challenge)
        .unwrap();

    // Create too many headers
    let mut too_many_headers = BTreeMap::new();
    for i in 0..10 {
        too_many_headers.insert(format!("X-Header-{i}"), format!("value{i}"));
    }

    let verify_req = crate::types::VerifyChallengeRequest {
        challenge: challenge_res.challenge,
        signature: signature.to_bytes().into(),
        challenge_request: challenge_req.clone(),
        expires_at: 0,
        additional_headers: too_many_headers,
    };

    let res = client
        .post("/v1/auth/verify")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&verify_req)
        .await;

    let verify_res: VerifyChallengeResponse = res.json().await;
    assert!(
        matches!(verify_res, VerifyChallengeResponse::UnexpectedError { message } if message.contains("Too many headers")),
        "Should reject too many headers"
    );

    // Test 2: Header name too long
    let res = client
        .post("/v1/auth/challenge")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&challenge_req)
        .await;
    let challenge_res: ChallengeResponse = res.json().await;

    let (signature, _) = signing_key
        .sign_prehash_recoverable(&challenge_res.challenge)
        .unwrap();

    let mut long_name_headers = BTreeMap::new();
    let long_name = "X-".to_string() + &"a".repeat(300);
    long_name_headers.insert(long_name, "value".to_string());

    let verify_req = crate::types::VerifyChallengeRequest {
        challenge: challenge_res.challenge,
        signature: signature.to_bytes().into(),
        challenge_request: challenge_req.clone(),
        expires_at: 0,
        additional_headers: long_name_headers,
    };

    let res = client
        .post("/v1/auth/verify")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&verify_req)
        .await;

    let verify_res: VerifyChallengeResponse = res.json().await;
    assert!(
        matches!(verify_res, VerifyChallengeResponse::UnexpectedError { message } if message.contains("Header name too long")),
        "Should reject header name too long"
    );

    // Test 3: Header value too long
    let res = client
        .post("/v1/auth/challenge")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&challenge_req)
        .await;
    let challenge_res: ChallengeResponse = res.json().await;

    let (signature, _) = signing_key
        .sign_prehash_recoverable(&challenge_res.challenge)
        .unwrap();

    let mut long_value_headers = BTreeMap::new();
    let long_value = "a".repeat(600);
    long_value_headers.insert("X-Test".to_string(), long_value);

    let verify_req = crate::types::VerifyChallengeRequest {
        challenge: challenge_res.challenge,
        signature: signature.to_bytes().into(),
        challenge_request: challenge_req,
        expires_at: 0,
        additional_headers: long_value_headers,
    };

    let res = client
        .post("/v1/auth/verify")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&verify_req)
        .await;

    let verify_res: VerifyChallengeResponse = res.json().await;
    assert!(
        matches!(verify_res, VerifyChallengeResponse::UnexpectedError { message } if message.contains("Header value too long")),
        "Should reject header value too long"
    );
}

/// Test PII hashing is working correctly in production
#[tokio::test]
async fn test_pii_hashing_in_production() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let db = proxy.db();

    let service_id = ServiceId::new(1);
    let mut service = ServiceModel {
        api_key_prefix: "pii_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };

    let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
    let public_key = signing_key.verifying_key().to_sec1_bytes();
    service.add_owner(KeyType::Ecdsa, public_key.clone().into());
    service.save(service_id, &db).unwrap();

    let router = proxy.router();
    let client = TestClient::new(router);

    // Get challenge
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

    // Send PII that should be hashed
    let mut pii_headers = BTreeMap::new();
    let email = "alice@example.com";
    let user_id = "user123";

    pii_headers.insert("X-Tenant-Id".to_string(), email.to_string());
    pii_headers.insert("X-User-Id".to_string(), user_id.to_string());
    pii_headers.insert("X-User-Email".to_string(), email.to_string());
    pii_headers.insert(
        "X-Customer-Email".to_string(),
        "bob@company.com".to_string(),
    );
    pii_headers.insert("X-Custom-Header".to_string(), "not-pii".to_string());

    let verify_req = crate::types::VerifyChallengeRequest {
        challenge: challenge_res.challenge,
        signature: signature.to_bytes().into(),
        challenge_request: challenge_req,
        expires_at: 0,
        additional_headers: pii_headers,
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

    // Exchange for Paseto token to inspect headers
    let exchange_req = TokenExchangeRequest {
        additional_headers: BTreeMap::new(),
        ttl_seconds: Some(60),
    };

    let res = client
        .post("/v1/auth/exchange")
        .header(headers::AUTHORIZATION, format!("Bearer {api_key}"))
        .json(&exchange_req)
        .await;

    assert!(res.status().is_success());
    let _exchange_res: TokenExchangeResponse = res.json().await;

    // Parse the Paseto token to check headers are hashed
    // In production, we can't directly inspect the token, but we can verify
    // that the stored values are hashed by checking they're 32 chars of hex

    // Get the API key model to check stored headers
    let key_id = api_key.split('.').next().unwrap();
    let api_key_model = crate::api_keys::ApiKeyModel::find_by_key_id(key_id, &db)
        .unwrap()
        .unwrap();

    let stored_headers = api_key_model.get_default_headers();

    // Verify PII fields are hashed
    let tenant_id = stored_headers.get("x-tenant-id").unwrap();
    assert_eq!(
        tenant_id.len(),
        32,
        "Tenant ID should be hashed to 32 chars"
    );
    assert_ne!(tenant_id, email, "Tenant ID should not be raw email");
    assert_eq!(
        tenant_id,
        &validation::hash_user_id(email),
        "Should match expected hash"
    );

    let user_id_hash = stored_headers.get("x-user-id").unwrap();
    assert_eq!(
        user_id_hash.len(),
        32,
        "User ID should be hashed to 32 chars"
    );
    assert_ne!(user_id_hash, user_id, "User ID should not be raw");
    assert_eq!(
        user_id_hash,
        &validation::hash_user_id(user_id),
        "Should match expected hash"
    );

    let user_email_hash = stored_headers.get("x-user-email").unwrap();
    assert_eq!(
        user_email_hash,
        &validation::hash_user_id(email),
        "Email should be hashed"
    );

    let customer_email_hash = stored_headers.get("x-customer-email").unwrap();
    assert_eq!(
        customer_email_hash,
        &validation::hash_user_id("bob@company.com"),
        "Customer email should be hashed"
    );

    // Non-PII field should not be hashed
    let custom_header = stored_headers.get("x-custom-header").unwrap();
    assert_eq!(
        custom_header, "not-pii",
        "Non-PII headers should not be hashed"
    );
}

/// Test that already hashed tenant IDs are not re-hashed
#[tokio::test]
async fn test_already_hashed_tenant_id_not_rehashed() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let db = proxy.db();

    let service_id = ServiceId::new(1);
    let mut service = ServiceModel {
        api_key_prefix: "hash_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };

    let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
    let public_key = signing_key.verifying_key().to_sec1_bytes();
    service.add_owner(KeyType::Ecdsa, public_key.clone().into());
    service.save(service_id, &db).unwrap();

    let router = proxy.router();
    let client = TestClient::new(router);

    // Get challenge
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

    // Send already hashed tenant ID (32 hex chars)
    let mut headers = BTreeMap::new();
    let already_hashed = "a1b2c3d4e5f678901234567890123456";
    headers.insert("X-Tenant-Id".to_string(), already_hashed.to_string());

    let verify_req = crate::types::VerifyChallengeRequest {
        challenge: challenge_res.challenge,
        signature: signature.to_bytes().into(),
        challenge_request: challenge_req,
        expires_at: 0,
        additional_headers: headers,
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

    // Check stored headers
    let key_id = api_key.split('.').next().unwrap();
    let api_key_model = crate::api_keys::ApiKeyModel::find_by_key_id(key_id, &db)
        .unwrap()
        .unwrap();

    let stored_headers = api_key_model.get_default_headers();
    let stored_tenant_id = stored_headers.get("x-tenant-id").unwrap();

    // Should be the same as what we sent (not re-hashed)
    assert_eq!(
        stored_tenant_id, already_hashed,
        "Already hashed ID should not be re-hashed"
    );
}
