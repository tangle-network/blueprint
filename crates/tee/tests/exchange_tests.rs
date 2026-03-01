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

// Edge case tests

#[tokio::test]
async fn test_auth_service_consume_expired_session() {
    let config = TeeKeyExchangeConfig {
        session_ttl_secs: 0, // immediate expiry
        max_sessions: 64,
        on_chain_verification: false,
    };
    let service = TeeAuthService::new(config);
    let (session_id, _) = service.create_session().await.unwrap();

    // Expiry uses second granularity: `elapsed > ttl_secs` where ttl_secs=0.
    // We need at least 1 full second to elapse so elapsed=1 > 0.
    tokio::time::sleep(tokio::time::Duration::from_millis(1100)).await;

    // Consuming an expired session should fail
    let result = service.consume_session(&session_id).await;
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("expired"),
        "error should mention expired: {err}"
    );
}

#[tokio::test]
async fn test_auth_service_get_public_key_expired_session() {
    let config = TeeKeyExchangeConfig {
        session_ttl_secs: 0,
        max_sessions: 64,
        on_chain_verification: false,
    };
    let service = TeeAuthService::new(config);
    let (session_id, _) = service.create_session().await.unwrap();

    // Wait for at least 1 second so the session expires (second granularity)
    tokio::time::sleep(tokio::time::Duration::from_millis(1100)).await;

    // Getting public key for an expired session should fail
    let result = service.get_session_public_key(&session_id).await;
    assert!(result.is_err());
}

#[test]
fn test_key_exchange_session_unique_ids() {
    let s1 = KeyExchangeSession::new(300);
    let s2 = KeyExchangeSession::new(300);
    assert_ne!(s1.session_id, s2.session_id, "session IDs must be unique");
    assert_ne!(s1.public_key, s2.public_key, "public keys must be unique");
}

#[test]
fn test_key_exchange_request_without_nonce() {
    let req = KeyExchangeRequest { nonce: None };
    let json = serde_json::to_string(&req).unwrap();
    // nonce should be skipped
    assert!(!json.contains("nonce"));
    let parsed: KeyExchangeRequest = serde_json::from_str(&json).unwrap();
    assert!(parsed.nonce.is_none());
}

#[test]
fn test_sealed_secret_payload_without_nonce() {
    let payload = SealedSecretPayload {
        session_id: "sess-1".to_string(),
        ciphertext: vec![1, 2, 3],
        nonce: None,
    };
    let json = serde_json::to_string(&payload).unwrap();
    assert!(!json.contains("nonce"));
    let parsed: SealedSecretPayload = serde_json::from_str(&json).unwrap();
    assert!(parsed.nonce.is_none());
}

#[tokio::test]
async fn test_auth_service_evicts_expired_on_create() {
    let config = TeeKeyExchangeConfig {
        session_ttl_secs: 0,
        max_sessions: 2,
        on_chain_verification: false,
    };
    let service = TeeAuthService::new(config);

    // Create 2 sessions (max)
    let _ = service.create_session().await.unwrap();
    let _ = service.create_session().await.unwrap();

    // Wait for them to expire (second granularity)
    tokio::time::sleep(tokio::time::Duration::from_millis(1100)).await;

    // Creating a new session should succeed because expired sessions get evicted
    let result = service.create_session().await;
    assert!(
        result.is_ok(),
        "should evict expired sessions before checking limit"
    );
}

#[test]
fn test_key_exchange_session_consume_is_irreversible() {
    let mut session = KeyExchangeSession::new(300);
    session.consume();
    assert!(!session.is_valid());
    // Even though not expired, consumed sessions are invalid
    assert!(!session.is_expired());
    assert!(session.consumed);
}

#[tokio::test]
async fn test_auth_service_ttl_and_max_sessions_accessors() {
    let config = TeeKeyExchangeConfig {
        session_ttl_secs: 600,
        max_sessions: 128,
        on_chain_verification: true,
    };
    let service = TeeAuthService::new(config);
    assert_eq!(service.session_ttl_secs(), 600);
    assert_eq!(service.max_sessions(), 128);
}

#[test]
fn test_key_exchange_response_serde() {
    use blueprint_tee::attestation::claims::AttestationClaims;
    use blueprint_tee::attestation::report::*;
    use blueprint_tee::config::TeeProvider;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let response = KeyExchangeResponse {
        session_id: "sess-abc".to_string(),
        public_key_hex: "deadbeef".to_string(),
        attestation: AttestationReport {
            provider: TeeProvider::IntelTdx,
            format: AttestationFormat::TdxQuote,
            issued_at_unix: now,
            measurement: Measurement::sha256("a".repeat(64)),
            public_key_binding: None,
            claims: AttestationClaims::new(),
            evidence: vec![1, 2, 3],
        },
    };

    let json = serde_json::to_string(&response).unwrap();
    let parsed: KeyExchangeResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.session_id, "sess-abc");
    assert_eq!(parsed.public_key_hex, "deadbeef");
    assert_eq!(parsed.attestation.provider, TeeProvider::IntelTdx);
}

#[test]
fn test_key_exchange_session_long_ttl() {
    let session = KeyExchangeSession::new(86400); // 24 hours
    assert!(!session.is_expired());
    assert!(session.is_valid());
    assert!(session.remaining_ttl().as_secs() > 86390);
}

#[tokio::test]
async fn test_auth_service_consume_removes_from_count() {
    let service = TeeAuthService::new(TeeKeyExchangeConfig::default());
    let (id1, _) = service.create_session().await.unwrap();
    let (id2, _) = service.create_session().await.unwrap();
    assert_eq!(service.active_session_count().await, 2);

    service.consume_session(&id1).await.unwrap();
    assert_eq!(service.active_session_count().await, 1);

    service.consume_session(&id2).await.unwrap();
    assert_eq!(service.active_session_count().await, 0);
}

#[tokio::test]
async fn test_auth_service_double_consume_fails() {
    let service = TeeAuthService::new(TeeKeyExchangeConfig::default());
    let (id, _) = service.create_session().await.unwrap();

    // First consume succeeds
    service.consume_session(&id).await.unwrap();
    // Second consume fails (session was removed)
    assert!(service.consume_session(&id).await.is_err());
}

#[test]
fn test_sealed_secret_result_failure() {
    let result = SealedSecretResult {
        success: false,
        attestation_digest: "".to_string(),
        key_fingerprint: "".to_string(),
    };
    let json = serde_json::to_string(&result).unwrap();
    let parsed: SealedSecretResult = serde_json::from_str(&json).unwrap();
    assert!(!parsed.success);
}

#[test]
fn test_sealed_secret_payload_large_ciphertext() {
    let payload = SealedSecretPayload {
        session_id: "sess-1".to_string(),
        ciphertext: vec![0u8; 1024 * 1024], // 1 MB
        nonce: Some(vec![0u8; 12]),
    };
    let json = serde_json::to_string(&payload).unwrap();
    let parsed: SealedSecretPayload = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.ciphertext.len(), 1024 * 1024);
}
