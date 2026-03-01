//! Tests for the TeeLayer middleware and TeeContext.

use blueprint_tee::attestation::claims::AttestationClaims;
use blueprint_tee::attestation::report::*;
use blueprint_tee::attestation::verifier::VerifiedAttestation;
use blueprint_tee::config::TeeProvider;
use blueprint_tee::middleware::tee_context::TeeContext;
use blueprint_tee::middleware::tee_layer::*;

fn sample_report() -> AttestationReport {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    AttestationReport {
        provider: TeeProvider::IntelTdx,
        format: AttestationFormat::TdxQuote,
        issued_at_unix: now,
        measurement: Measurement::sha256("a".repeat(64)),
        public_key_binding: None,
        claims: AttestationClaims::new(),
        evidence: b"test-evidence".to_vec(),
    }
}

#[test]
fn test_tee_context_none() {
    let ctx = TeeContext::none();
    assert!(!ctx.is_attested());
    assert!(!ctx.is_tee_active());
    assert!(ctx.attestation.is_none());
    assert!(ctx.provider.is_none());
    assert!(ctx.deployment_id.is_none());
}

#[test]
fn test_tee_context_with_attestation() {
    let report = sample_report();
    let verified = VerifiedAttestation::new(report, TeeProvider::IntelTdx);
    let ctx = TeeContext::with_attestation(verified);

    assert!(ctx.is_attested());
    assert!(ctx.is_tee_active());
    assert_eq!(ctx.provider, Some(TeeProvider::IntelTdx));
}

#[test]
fn test_tee_context_with_deployment_id() {
    let report = sample_report();
    let verified = VerifiedAttestation::new(report, TeeProvider::IntelTdx);
    let ctx = TeeContext::with_attestation(verified).with_deployment_id("deploy-123");

    assert_eq!(ctx.deployment_id.as_deref(), Some("deploy-123"));
}

#[test]
fn test_tee_context_default() {
    let ctx = TeeContext::default();
    assert!(!ctx.is_attested());
    assert!(!ctx.is_tee_active());
}

#[test]
fn test_tee_layer_new() {
    let layer = TeeLayer::new();
    // Verify it can be cloned (required for tower Layer)
    let _cloned = layer.clone();
}

#[test]
fn test_tee_layer_default() {
    let layer = TeeLayer::default();
    let _cloned = layer.clone();
}

#[tokio::test]
async fn test_tee_layer_set_attestation() {
    let layer = TeeLayer::new();
    let report = sample_report();
    layer.set_attestation(report).await;

    // Verify the attestation handle has data
    let handle = layer.attestation_handle();
    let guard = handle.lock().await;
    assert!(guard.is_some());
}

#[test]
fn test_tee_layer_with_attestation() {
    let report = sample_report();
    let _layer = TeeLayer::with_attestation(report);
}

#[test]
fn test_metadata_keys() {
    assert_eq!(TEE_ATTESTATION_DIGEST_KEY, "tee.attestation.digest");
    assert_eq!(TEE_PROVIDER_KEY, "tee.provider");
    assert_eq!(TEE_MEASUREMENT_KEY, "tee.measurement");
}

// Integration test: TeeLayer applied to a router
#[tokio::test]
async fn test_tee_layer_with_router() {
    use blueprint_core::{Bytes, JobCall};
    use blueprint_router::Router;
    use tower::Service;

    let report = sample_report();
    let tee_layer = TeeLayer::with_attestation(report);

    let mut router = Router::new().route(0, async || vec![42u8]).layer(tee_layer);

    let call = JobCall::new(0u32, Bytes::new());
    let result = router.call(call).await;

    // The router returns Option<Vec<JobResult>>
    let results = result
        .expect("router call should succeed")
        .expect("should return Some");
    assert!(!results.is_empty(), "should have at least one result");

    match &results[0] {
        blueprint_core::JobResult::Ok { head, .. } => {
            // TEE metadata should be attached
            assert!(head.metadata.get(TEE_PROVIDER_KEY).is_some());
            assert!(head.metadata.get(TEE_ATTESTATION_DIGEST_KEY).is_some());
            assert!(head.metadata.get(TEE_MEASUREMENT_KEY).is_some());
        }
        blueprint_core::JobResult::Err(_) => {
            panic!("expected Ok result");
        }
    }
}
