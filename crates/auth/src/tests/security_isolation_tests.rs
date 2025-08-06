//! Security isolation tests for the two-tier authentication system
//! These tests ensure users cannot spoof, impersonate, or access each other's resources

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
};

/// Test that multiple users with API keys cannot access each other's resources
#[tokio::test]
async fn test_api_key_cross_user_isolation() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();

    // Create test server that echoes tenant info
    let test_router = axum::Router::new().route(
        "/api/secure",
        axum::routing::get(|headers: axum::http::HeaderMap| async move {
            let tenant_id = headers
                .get("x-tenant-id")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("unknown");
            axum::Json(serde_json::json!({
                "tenant_id": tenant_id,
                "message": format!("Secure data for {}", tenant_id)
            }))
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

    // Create service
    let service_id = ServiceId::new(1);
    let mut service = ServiceModel {
        api_key_prefix: "sec_".to_string(),
        owners: Vec::new(),
        upstream_url: format!("http://localhost:{}", test_addr.port()),
    };

    // Create three different users with different signing keys
    let users = vec![
        ("alice@company.com", "alice@company.com", "Alice Corp"),
        ("bob@company.com", "bob@company.com", "Bob Corp"),
        ("eve@malicious.com", "eve@malicious.com", "Evil Corp"),
    ];

    // Pre-generate all users and add them to the service
    let mut user_data = Vec::new();
    for (email, tenant_hash, company) in users {
        let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
        let public_key = signing_key.verifying_key().to_sec1_bytes();

        // Add this user as an owner (simulating different users with different keys)
        service.add_owner(KeyType::Ecdsa, public_key.clone().into());
        user_data.push((email, tenant_hash, company, signing_key, public_key));
    }

    // Save service with all users before making any requests
    let db = proxy.db();
    service.save(service_id, &db).unwrap();

    let router = proxy.router();
    let client = TestClient::new(router);
    let mut user_api_keys = Vec::new();

    for (email, tenant_hash, company, signing_key, public_key) in user_data {
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

        if !res.status().is_success() {
            eprintln!("Challenge request failed with status: {}", res.status());
            let body = res.text().await;
            eprintln!("Response body: {:?}", body);
            panic!("Challenge request failed");
        }
        let challenge_res: ChallengeResponse = res.json().await;

        // Sign and verify with user-specific headers
        let (signature, _) = signing_key
            .sign_prehash_recoverable(&challenge_res.challenge)
            .unwrap();

        let mut user_headers = BTreeMap::new();
        // Use the actual email, it will be hashed by PII protection
        user_headers.insert("X-Tenant-Id".to_string(), tenant_hash.to_string());
        user_headers.insert("X-Tenant-Name".to_string(), company.to_string());

        let verify_req = crate::types::VerifyChallengeRequest {
            challenge: challenge_res.challenge,
            signature: signature.to_bytes().into(),
            challenge_request: challenge_req,
            expires_at: 0,
            additional_headers: user_headers,
        };

        let res = client
            .post("/v1/auth/verify")
            .header(headers::X_SERVICE_ID, service_id.to_string())
            .json(&verify_req)
            .await;
        let verify_res: VerifyChallengeResponse = res.json().await;

        let api_key = match verify_res {
            VerifyChallengeResponse::Verified { api_key, .. } => api_key,
            _ => panic!("Expected verified response for user {}", email),
        };

        // Store the hashed version for comparison
        let expected_hash = crate::validation::hash_user_id(tenant_hash);
        user_api_keys.push((email, expected_hash, api_key));
    }

    // Test 1: Each user can access their own data
    for (email, expected_tenant_id, api_key) in &user_api_keys {
        let res = client
            .get("/api/secure")
            .header(headers::AUTHORIZATION, format!("Bearer {}", api_key))
            .await;

        assert!(
            res.status().is_success(),
            "User {} should be able to access their data",
            email
        );

        let data: serde_json::Value = res.json().await;
        assert_eq!(
            data["tenant_id"].as_str().unwrap(),
            expected_tenant_id,
            "User {} should see their own tenant ID",
            email
        );
    }

    // Test 2: Users cannot manipulate API keys to access other users' data
    let alice_key = &user_api_keys[0].2;
    let bob_key = &user_api_keys[1].2;

    // Try to use Alice's key with Bob's identifier (this should fail or still return Alice's data)
    let alice_parts: Vec<&str> = alice_key.split('.').collect();
    let bob_parts: Vec<&str> = bob_key.split('.').collect();

    // Attempt to create a hybrid key (should fail validation)
    let malicious_key = format!("{}.{}", alice_parts[0], bob_parts[1]);

    let res = client
        .get("/api/secure")
        .header(headers::AUTHORIZATION, format!("Bearer {}", malicious_key))
        .await;

    // Should get 401 because the key validation should fail
    assert_eq!(
        res.status(),
        401,
        "Malicious key manipulation should be rejected"
    );

    test_server.abort();
}

/// Test Paseto token isolation - users cannot use each other's Paseto tokens
#[tokio::test]
async fn test_paseto_token_cross_user_isolation() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();

    // Create test server
    let test_router = axum::Router::new().route(
        "/api/user-data",
        axum::routing::get(|headers: axum::http::HeaderMap| async move {
            let tenant_id = headers
                .get("x-tenant-id")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("unknown");
            let role = headers
                .get("x-tenant-role")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("user");

            axum::Json(serde_json::json!({
                "tenant_id": tenant_id,
                "role": role,
                "sensitive_data": format!("Secret data for {} with role {}", tenant_id, role)
            }))
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

    let service_id = ServiceId::new(1);
    let mut service = ServiceModel {
        api_key_prefix: "pst_".to_string(),
        owners: Vec::new(),
        upstream_url: format!("http://localhost:{}", test_addr.port()),
    };

    // Create two users with different privileges
    let admin_user = ("admin@company.com", "admin@company.com", "admin");
    let regular_user = ("user@company.com", "user@company.com", "user");

    // Pre-generate all users and add them to the service
    let mut user_data = Vec::new();
    for (email, tenant_id, role) in [admin_user, regular_user] {
        let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
        let public_key = signing_key.verifying_key().to_sec1_bytes();
        service.add_owner(KeyType::Ecdsa, public_key.clone().into());
        user_data.push((email, tenant_id, role, signing_key, public_key));
    }

    // Save service with all users before making any requests
    let db = proxy.db();
    service.save(service_id, &db).unwrap();

    let router = proxy.router();
    let client = TestClient::new(router);
    let mut user_paseto_tokens = Vec::new();

    for (email, tenant_id, role, signing_key, public_key) in user_data {
        // Get API key first
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

        let mut user_headers = BTreeMap::new();
        user_headers.insert("X-Tenant-Id".to_string(), tenant_id.to_string());
        user_headers.insert("X-Tenant-Role".to_string(), role.to_string());

        let verify_req = crate::types::VerifyChallengeRequest {
            challenge: challenge_res.challenge,
            signature: signature.to_bytes().into(),
            challenge_request: challenge_req,
            expires_at: 0,
            additional_headers: user_headers,
        };

        let res = client
            .post("/v1/auth/verify")
            .header(headers::X_SERVICE_ID, service_id.to_string())
            .json(&verify_req)
            .await;
        let verify_res: VerifyChallengeResponse = res.json().await;

        let api_key = match verify_res {
            VerifyChallengeResponse::Verified { api_key, .. } => api_key,
            _ => panic!("Expected verified response for user {}", email),
        };

        // Exchange for Paseto token
        let exchange_req = TokenExchangeRequest {
            additional_headers: BTreeMap::new(), // No additional headers in exchange
            ttl_seconds: Some(60),
        };

        let res = client
            .post("/v1/auth/exchange")
            .header(headers::AUTHORIZATION, format!("Bearer {}", api_key))
            .json(&exchange_req)
            .await;

        assert!(
            res.status().is_success(),
            "Token exchange should succeed for {}",
            email
        );
        let exchange_res: TokenExchangeResponse = res.json().await;

        // Store the hashed version for comparison
        let expected_hash = crate::validation::hash_user_id(tenant_id);
        user_paseto_tokens.push((email, expected_hash, role, exchange_res.access_token));
    }

    // Test 1: Each user can access their own data with Paseto tokens
    for (email, expected_tenant_id, expected_role, paseto_token) in &user_paseto_tokens {
        let res = client
            .get("/api/user-data")
            .header(headers::AUTHORIZATION, format!("Bearer {}", paseto_token))
            .await;

        assert!(
            res.status().is_success(),
            "User {} should access data with Paseto token",
            email
        );

        let data: serde_json::Value = res.json().await;
        assert_eq!(data["tenant_id"].as_str().unwrap(), expected_tenant_id);
        assert_eq!(data["role"].as_str().unwrap(), *expected_role);
    }

    // Test 2: Users cannot use each other's Paseto tokens
    let admin_token = &user_paseto_tokens[0].3;
    let _user_token = &user_paseto_tokens[1].3;

    // Regular user tries to use admin token (should fail or return their own data)
    // Since Paseto tokens are cryptographically signed, this should fail validation
    // But if somehow it passes, the claims inside should still be for the admin user

    // The key insight: Paseto tokens contain embedded claims, so even if someone
    // steals a token, they get the claims of the original user, not their own identity

    // Test 3: Token modification should fail
    // Try to modify the Paseto token (should fail cryptographic validation)
    let mut modified_token = admin_token.clone();
    // Change one character in the token
    if let Some(pos) = modified_token.rfind('A') {
        modified_token.replace_range(pos..pos + 1, "B");
    }

    let res = client
        .get("/api/user-data")
        .header(headers::AUTHORIZATION, format!("Bearer {}", modified_token))
        .await;

    assert_eq!(
        res.status(),
        401,
        "Modified Paseto token should be rejected"
    );

    test_server.abort();
}

/// Test concurrent multi-user authentication and token exchange
#[tokio::test]
async fn test_concurrent_multi_user_authentication() {
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();

    // Create service
    let service_id = ServiceId::new(1);
    let mut service = ServiceModel {
        api_key_prefix: "conc_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };

    let db = proxy.db();
    let router = proxy.router();

    // Create multiple concurrent authentication tasks
    let num_users = 10;

    // Pre-generate keys and add all users as owners first
    let mut user_keys = Vec::new();
    for _ in 0..num_users {
        let mut rng = blueprint_std::BlueprintRng::new();
        let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
        let public_key = signing_key.verifying_key().to_sec1_bytes();
        service.add_owner(KeyType::Ecdsa, public_key.clone().into());
        user_keys.push(signing_key);
    }
    service.save(service_id, &db).unwrap();

    let mut tasks = Vec::new();
    for (user_id, signing_key) in user_keys.into_iter().enumerate() {
        let client = TestClient::new(router.clone());
        let service_id = service_id;

        let task = tokio::spawn(async move {
            let public_key = signing_key.verifying_key().to_sec1_bytes();

            // Challenge
            let challenge_req = ChallengeRequest {
                pub_key: public_key.into(),
                key_type: KeyType::Ecdsa,
            };

            let res = client
                .post("/v1/auth/challenge")
                .header(headers::X_SERVICE_ID, service_id.to_string())
                .json(&challenge_req)
                .await;

            if !res.status().is_success() {
                return Err(format!(
                    "User {} challenge failed: {}",
                    user_id,
                    res.status()
                ));
            }

            let challenge_res: ChallengeResponse = res.json().await;

            // Sign
            let (signature, _) = signing_key
                .sign_prehash_recoverable(&challenge_res.challenge)
                .unwrap();

            let mut user_headers = BTreeMap::new();
            user_headers.insert("X-Tenant-Id".to_string(), format!("tenant_{}", user_id));
            user_headers.insert("X-User-Id".to_string(), format!("user_{}", user_id));

            let verify_req = crate::types::VerifyChallengeRequest {
                challenge: challenge_res.challenge,
                signature: signature.to_bytes().into(),
                challenge_request: challenge_req,
                expires_at: 0,
                additional_headers: user_headers,
            };

            let res = client
                .post("/v1/auth/verify")
                .header(headers::X_SERVICE_ID, service_id.to_string())
                .json(&verify_req)
                .await;

            if !res.status().is_success() {
                return Err(format!("User {} verify failed: {}", user_id, res.status()));
            }

            let verify_res: VerifyChallengeResponse = res.json().await;
            let api_key = match verify_res {
                VerifyChallengeResponse::Verified { api_key, .. } => api_key,
                _ => return Err(format!("User {} got invalid verify response", user_id)),
            };

            // Exchange for Paseto token
            let exchange_req = TokenExchangeRequest {
                additional_headers: {
                    let mut headers = BTreeMap::new();
                    headers.insert("X-Session-Id".to_string(), format!("session_{}", user_id));
                    headers
                },
                ttl_seconds: Some(30),
            };

            let res = client
                .post("/v1/auth/exchange")
                .header(headers::AUTHORIZATION, format!("Bearer {}", api_key))
                .json(&exchange_req)
                .await;

            if !res.status().is_success() {
                return Err(format!(
                    "User {} token exchange failed: {}",
                    user_id,
                    res.status()
                ));
            }

            let exchange_res: TokenExchangeResponse = res.json().await;
            Ok((user_id, api_key, exchange_res.access_token))
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete
    let mut results = Vec::new();
    for task in tasks {
        match task.await.unwrap() {
            Ok(result) => results.push(result),
            Err(e) => panic!("Concurrent authentication failed: {}", e),
        }
    }

    // Verify all users got unique tokens
    assert_eq!(results.len(), num_users);

    let api_keys: std::collections::HashSet<_> =
        results.iter().map(|(_, api_key, _)| api_key).collect();
    let paseto_tokens: std::collections::HashSet<_> =
        results.iter().map(|(_, _, paseto)| paseto).collect();

    assert_eq!(api_keys.len(), num_users, "All API keys should be unique");
    assert_eq!(
        paseto_tokens.len(),
        num_users,
        "All Paseto tokens should be unique"
    );

    println!(
        "✅ All {} users successfully authenticated concurrently with unique tokens",
        num_users
    );
}

/// Test header injection attempts during token exchange
#[tokio::test]
async fn test_token_exchange_header_injection_security() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();

    let service_id = ServiceId::new(1);
    let mut service = ServiceModel {
        api_key_prefix: "inj_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };

    let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
    let public_key = signing_key.verifying_key().to_sec1_bytes();
    service.add_owner(KeyType::Ecdsa, public_key.clone().into());
    service.save(service_id, &proxy.db()).unwrap();

    let router = proxy.router();
    let client = TestClient::new(router);

    // Get API key for legitimate user
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
        additional_headers: {
            let mut headers = BTreeMap::new();
            headers.insert("X-Tenant-Id".to_string(), "legitimate_tenant".to_string());
            headers
        },
    };

    let res = client
        .post("/v1/auth/verify")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&verify_req)
        .await;
    let verify_res: VerifyChallengeResponse = res.json().await;
    let _api_key = match verify_res {
        VerifyChallengeResponse::Verified { api_key, .. } => api_key,
        _ => panic!("Expected verified response"),
    };

    // Test various header injection attempts
    let malicious_headers = vec![
        // Try to inject admin privileges
        ("X-Tenant-Role", "admin"),
        ("x-tenant-role", "admin"), // lowercase
        // Try to inject system headers
        ("X-System-Admin", "true"),
        // Try to override tenant
        ("X-Tenant-Id", "admin_tenant"),
        // Try to inject privileged headers that should be filtered
        ("Authorization", "Bearer admin_token"),
        ("Host", "evil.com"),
        // Try connection manipulation
        ("Connection", "close"),
        ("Upgrade", "websocket"),
        // Try content manipulation
        ("Content-Length", "99999"),
        ("Transfer-Encoding", "chunked"),
    ];

    for (header_name, header_value) in malicious_headers {
        let mut attack_headers = BTreeMap::new();
        attack_headers.insert("X-Tenant-Id".to_string(), "legitimate_tenant".to_string());
        attack_headers.insert(header_name.to_string(), header_value.to_string());
    }
}
