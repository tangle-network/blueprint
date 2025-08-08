//! Tests for API key lifecycle operations (rotation, renewal, revocation)

use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::tempdir;

use crate::{
    api_keys::{ApiKeyGenerator, ApiKeyModel},
    models::ServiceModel,
    proxy::AuthenticatedProxy,
    test_client::TestClient,
    types::{
        ChallengeRequest, ChallengeResponse, KeyType, ServiceId, VerifyChallengeResponse, headers,
    },
};

/// Test API key rotation - replacing an old key with a new one
#[tokio::test]
async fn test_api_key_rotation() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let db = proxy.db();

    // Create a service
    let service_id = ServiceId::new(1);
    let mut service = ServiceModel {
        api_key_prefix: "rot_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };

    let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
    let public_key = signing_key.verifying_key().to_sec1_bytes();
    service.add_owner(KeyType::Ecdsa, public_key.into());
    service.save(service_id, &db).unwrap();

    // Generate first API key
    let key_gen = ApiKeyGenerator::with_prefix("rot_");
    let expires_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600; // 1 hour from now

    let old_key = key_gen.generate_key(service_id, expires_at, BTreeMap::new(), &mut rng);

    let mut old_model = ApiKeyModel::from(&old_key);
    old_model.save(&db).unwrap();

    // Verify old key works
    let found = ApiKeyModel::find_by_key_id(&old_model.key_id, &db).unwrap();
    assert!(found.is_some());
    assert!(found.unwrap().validates_key(old_key.full_key()));

    // Rotate: disable old key and create new one
    old_model.is_enabled = false;
    old_model.save(&db).unwrap();

    // Generate new API key with longer expiration
    let new_expires_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + (90 * 24 * 3600); // 90 days

    let new_key = key_gen.generate_key(service_id, new_expires_at, BTreeMap::new(), &mut rng);

    let mut new_model = ApiKeyModel::from(&new_key);
    new_model.save(&db).unwrap();

    // Verify old key is disabled
    let old_check = ApiKeyModel::find_by_key_id(&old_model.key_id, &db)
        .unwrap()
        .unwrap();
    assert!(!old_check.is_enabled, "Old key should be disabled");

    // Verify new key works
    let new_check = ApiKeyModel::find_by_key_id(&new_model.key_id, &db)
        .unwrap()
        .unwrap();
    assert!(new_check.is_enabled, "New key should be enabled");
    assert!(new_check.validates_key(new_key.full_key()));
}

/// Test API key renewal - extending expiration of existing key
#[tokio::test]
async fn test_api_key_renewal() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let db = proxy.db();

    // Create a service
    let service_id = ServiceId::new(1);
    let service = ServiceModel {
        api_key_prefix: "ren_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };
    service.save(service_id, &db).unwrap();

    // Generate API key with short expiration
    let key_gen = ApiKeyGenerator::with_prefix("ren_");
    let short_expires = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 300; // 5 minutes

    let api_key = key_gen.generate_key(service_id, short_expires, BTreeMap::new(), &mut rng);

    let mut model = ApiKeyModel::from(&api_key);
    model.save(&db).unwrap();

    // Verify initial expiration
    assert_eq!(model.expires_at, short_expires);

    // Renew the key with longer expiration
    let new_expires = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + (30 * 24 * 3600); // 30 days

    model.expires_at = new_expires;
    model.save(&db).unwrap();

    // Verify renewal worked
    let renewed = ApiKeyModel::find_by_key_id(&model.key_id, &db)
        .unwrap()
        .unwrap();
    assert_eq!(renewed.expires_at, new_expires);
    assert!(!renewed.is_expired());
}

/// Test API key revocation and disabling
#[tokio::test]
async fn test_api_key_revocation() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let db = proxy.db();

    // Create service
    let service_id = ServiceId::new(1);
    let mut service = ServiceModel {
        api_key_prefix: "rev_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };

    let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
    let public_key = signing_key.verifying_key().to_sec1_bytes();
    service.add_owner(KeyType::Ecdsa, public_key.clone().into());
    service.save(service_id, &db).unwrap();

    // Use full proxy for integration test
    let router = proxy.router();
    let client = TestClient::new(router);

    // Get API key through normal flow
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

    // Test key works initially
    let res = client
        .get("/test")
        .header(headers::AUTHORIZATION, format!("Bearer {}", api_key))
        .await;
    // Will fail with 502 since no backend, but auth should pass
    assert_ne!(res.status(), 401, "Key should be valid initially");

    // Extract key_id and revoke the key
    let key_id = api_key.split('.').next().unwrap();
    let mut model = ApiKeyModel::find_by_key_id(key_id, &db).unwrap().unwrap();

    // Revoke by disabling
    model.is_enabled = false;
    model.save(&db).unwrap();

    // Test key no longer works
    let res = client
        .get("/test")
        .header(headers::AUTHORIZATION, format!("Bearer {}", api_key))
        .await;
    assert_eq!(res.status(), 401, "Revoked key should be rejected");
}

/// Test that expired keys are rejected
#[tokio::test]
async fn test_expired_api_key_rejection() {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let db = proxy.db();

    let service_id = ServiceId::new(1);
    let service = ServiceModel {
        api_key_prefix: "exp_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };
    service.save(service_id, &db).unwrap();

    // Generate API key that's already expired
    let key_gen = ApiKeyGenerator::with_prefix("exp_");
    let past_time = 1; // Way in the past

    let api_key = key_gen.generate_key(service_id, past_time, BTreeMap::new(), &mut rng);

    let mut model = ApiKeyModel::from(&api_key);
    model.save(&db).unwrap();

    // Verify the key is expired
    assert!(model.is_expired(), "Key should be expired");

    // Try to use expired key through proxy
    let router = proxy.router();
    let client = TestClient::new(router);

    let res = client
        .get("/test")
        .header(
            headers::AUTHORIZATION,
            format!("Bearer {}", api_key.full_key()),
        )
        .await;

    assert_eq!(res.status(), 401, "Expired key should be rejected");
}

/// Test concurrent API key operations
#[tokio::test]
async fn test_concurrent_api_key_operations() {
    let tmp = tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();
    let db = proxy.db();

    let service_id = ServiceId::new(1);
    let service = ServiceModel {
        api_key_prefix: "con_".to_string(),
        owners: Vec::new(),
        upstream_url: "http://localhost:8080".to_string(),
    };
    service.save(service_id, &db).unwrap();

    // Spawn multiple tasks creating API keys concurrently
    let mut handles = Vec::new();
    for i in 0..10 {
        let db = db.clone();
        let handle = tokio::spawn(async move {
            let mut rng = blueprint_std::BlueprintRng::new();
            let key_gen = ApiKeyGenerator::with_prefix(&format!("con{}_", i));

            let expires = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 3600;

            let key = key_gen.generate_key(service_id, expires, BTreeMap::new(), &mut rng);

            let mut model = ApiKeyModel::from(&key);
            model.save(&db).unwrap();

            (model.key_id.clone(), key.full_key().to_string())
        });
        handles.push(handle);
    }

    // Wait for all to complete
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    // Verify all keys were created and are unique
    assert_eq!(results.len(), 10);

    let key_ids: std::collections::HashSet<_> = results.iter().map(|(id, _)| id).collect();
    assert_eq!(key_ids.len(), 10, "All key IDs should be unique");

    // Verify all keys can be retrieved
    for (key_id, full_key) in results {
        let model = ApiKeyModel::find_by_key_id(&key_id, &db).unwrap().unwrap();
        assert!(model.validates_key(&full_key));
    }
}
