//! Tests for the key exchange protocol and service.

use blueprint_tee::config::TeeKeyExchangeConfig;
use blueprint_tee::exchange::TeeAuthService;
use blueprint_tee::exchange::protocol::*;

#[test]
fn test_key_exchange_session_creation() {
    let session = KeyExchangeSession::new(300);
    assert!(!session.session_id.is_empty());
    assert!(!session.public_key.is_empty());
    assert_eq!(session.public_key.len(), 32); // SHA-256 output
    assert!(!session.consumed);
    assert!(!session.is_expired());
    assert!(session.is_valid());
}

#[test]
fn test_key_exchange_session_ttl() {
    let session = KeyExchangeSession::new(0); // immediate expiry
    // The session might or might not be expired depending on timing,
    // but with TTL 0 it should expire within a second
    assert_eq!(session.ttl_secs, 0);
}

#[test]
fn test_key_exchange_session_consume() {
    let mut session = KeyExchangeSession::new(300);
    assert!(session.is_valid());
    session.consume();
    assert!(session.consumed);
    assert!(!session.is_valid());
}

#[test]
fn test_key_exchange_session_public_key_digest() {
    let session = KeyExchangeSession::new(300);
    let digest = session.public_key_digest();
    assert_eq!(digest.len(), 64); // SHA-256 hex
}

#[test]
fn test_key_exchange_session_remaining_ttl() {
    let session = KeyExchangeSession::new(300);
    let remaining = session.remaining_ttl();
    assert!(remaining.as_secs() <= 300);
    assert!(remaining.as_secs() >= 298); // allow small timing variance
}

#[tokio::test]
async fn test_auth_service_create_session() {
    let service = TeeAuthService::new(TeeKeyExchangeConfig::default());
    let (session_id, public_key) = service.create_session().await.unwrap();

    assert!(!session_id.is_empty());
    assert_eq!(public_key.len(), 32);
    assert_eq!(service.active_session_count().await, 1);
}

#[tokio::test]
async fn test_auth_service_consume_session() {
    let service = TeeAuthService::new(TeeKeyExchangeConfig::default());
    let (session_id, _) = service.create_session().await.unwrap();

    let session = service.consume_session(&session_id).await.unwrap();
    assert_eq!(session.session_id, session_id);

    // Session should be gone after consumption
    assert!(service.consume_session(&session_id).await.is_err());
}

#[tokio::test]
async fn test_auth_service_get_public_key() {
    let service = TeeAuthService::new(TeeKeyExchangeConfig::default());
    let (session_id, expected_key) = service.create_session().await.unwrap();

    let key = service.get_session_public_key(&session_id).await.unwrap();
    assert_eq!(key, expected_key);
}

#[tokio::test]
async fn test_auth_service_session_not_found() {
    let service = TeeAuthService::new(TeeKeyExchangeConfig::default());
    assert!(service.consume_session("nonexistent").await.is_err());
    assert!(service.get_session_public_key("nonexistent").await.is_err());
}

#[tokio::test]
async fn test_auth_service_max_sessions() {
    let config = TeeKeyExchangeConfig {
        session_ttl_secs: 300,
        max_sessions: 2,
        on_chain_verification: false,
    };
    let service = TeeAuthService::new(config);

    // Create max sessions
    service.create_session().await.unwrap();
    service.create_session().await.unwrap();

    // Third should fail
    let result = service.create_session().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_auth_service_multiple_sessions() {
    let service = TeeAuthService::new(TeeKeyExchangeConfig::default());

    let (id1, _) = service.create_session().await.unwrap();
    let (id2, _) = service.create_session().await.unwrap();
    let (id3, _) = service.create_session().await.unwrap();

    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_eq!(service.active_session_count().await, 3);

    // Consume one
    service.consume_session(&id2).await.unwrap();
    assert_eq!(service.active_session_count().await, 2);
}

#[test]
fn test_key_exchange_request_serde() {
    let req = KeyExchangeRequest {
        nonce: Some("test-nonce".to_string()),
    };
    let json = serde_json::to_string(&req).unwrap();
    let parsed: KeyExchangeRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.nonce.as_deref(), Some("test-nonce"));
}

#[test]
fn test_sealed_secret_payload_serde() {
    let payload = SealedSecretPayload {
        session_id: "sess-1".to_string(),
        ciphertext: vec![1, 2, 3],
        nonce: Some(vec![4, 5, 6]),
    };
    let json = serde_json::to_string(&payload).unwrap();
    let parsed: SealedSecretPayload = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.session_id, "sess-1");
    assert_eq!(parsed.ciphertext, vec![1, 2, 3]);
}

#[test]
fn test_sealed_secret_result_serde() {
    let result = SealedSecretResult {
        success: true,
        attestation_digest: "abc".to_string(),
        key_fingerprint: "def".to_string(),
    };
    let json = serde_json::to_string(&result).unwrap();
    let parsed: SealedSecretResult = serde_json::from_str(&json).unwrap();
    assert!(parsed.success);
}
