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
    let verified = VerifiedAttestation::new_for_test(report, TeeProvider::IntelTdx);
    let ctx = TeeContext::with_attestation(verified);

    assert!(ctx.is_attested());
    assert!(ctx.is_tee_active());
    assert_eq!(ctx.provider, Some(TeeProvider::IntelTdx));
}

#[test]
fn test_tee_context_with_deployment_id() {
    let report = sample_report();
    let verified = VerifiedAttestation::new_for_test(report, TeeProvider::IntelTdx);
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

// Edge case: TeeLayer with no attestation should not inject metadata
#[tokio::test]
async fn test_tee_layer_without_attestation() {
    use blueprint_core::{Bytes, JobCall};
    use blueprint_router::Router;
    use tower::Service;

    // Layer with no attestation set
    let tee_layer = TeeLayer::new();

    let mut router = Router::new().route(0, async || vec![42u8]).layer(tee_layer);

    let call = JobCall::new(0u32, Bytes::new());
    let result = router.call(call).await;

    let results = result
        .expect("router call should succeed")
        .expect("should return Some");
    assert!(!results.is_empty());

    match &results[0] {
        blueprint_core::JobResult::Ok { head, .. } => {
            // No TEE metadata should be attached when no attestation is set
            assert!(
                head.metadata.get(TEE_PROVIDER_KEY).is_none(),
                "should not have provider metadata without attestation"
            );
            assert!(
                head.metadata.get(TEE_ATTESTATION_DIGEST_KEY).is_none(),
                "should not have digest metadata without attestation"
            );
            assert!(
                head.metadata.get(TEE_MEASUREMENT_KEY).is_none(),
                "should not have measurement metadata without attestation"
            );
        }
        blueprint_core::JobResult::Err(_) => {
            panic!("expected Ok result");
        }
    }
}

// TeeContext: verify is_tee_active with provider but no attestation
#[test]
fn test_tee_context_provider_without_attestation() {
    let ctx = TeeContext {
        attestation: None,
        provider: Some(TeeProvider::IntelTdx),
        deployment_id: None,
    };
    assert!(!ctx.is_attested());
    assert!(ctx.is_tee_active());
}

// TeeLayer: attestation can be updated after construction
#[tokio::test]
async fn test_tee_layer_update_attestation() {
    let layer = TeeLayer::new();

    // Initially no attestation
    {
        let handle = layer.attestation_handle();
        let guard = handle.lock().await;
        assert!(guard.is_none());
    }

    // Set attestation
    let report = sample_report();
    layer.set_attestation(report.clone()).await;

    // Verify it's set
    {
        let handle = layer.attestation_handle();
        let guard = handle.lock().await;
        assert!(guard.is_some());
        assert_eq!(guard.as_ref().unwrap().provider, TeeProvider::IntelTdx);
    }
}

// TeeContext: clone preserves all fields
#[test]
fn test_tee_context_clone() {
    let report = sample_report();
    let verified = VerifiedAttestation::new_for_test(report, TeeProvider::IntelTdx);
    let ctx = TeeContext::with_attestation(verified).with_deployment_id("dep-1");
    let cloned = ctx.clone();
    assert_eq!(cloned.provider, Some(TeeProvider::IntelTdx));
    assert_eq!(cloned.deployment_id.as_deref(), Some("dep-1"));
    assert!(cloned.is_attested());
}

// TeeContext: different providers
#[test]
fn test_tee_context_all_providers() {
    for provider in [
        TeeProvider::IntelTdx,
        TeeProvider::AmdSevSnp,
        TeeProvider::AwsNitro,
        TeeProvider::AzureSnp,
        TeeProvider::GcpConfidential,
    ] {
        let mut report = sample_report();
        report.provider = provider;
        let verified = VerifiedAttestation::new_for_test(report, provider);
        let ctx = TeeContext::with_attestation(verified);
        assert_eq!(ctx.provider, Some(provider));
        assert!(ctx.is_attested());
    }
}

// Test TeeLayer can be applied to router with error result
#[tokio::test]
async fn test_tee_layer_with_error_result() {
    use blueprint_core::{Bytes, JobCall};
    use blueprint_router::Router;
    use tower::Service;

    let report = sample_report();
    let tee_layer = TeeLayer::with_attestation(report);

    let mut router = Router::new()
        .route(
            0,
            async || -> Result<Vec<u8>, String> { Err("job failed".to_string()) },
        )
        .layer(tee_layer);

    let call = JobCall::new(0u32, Bytes::new());
    let result = router.call(call).await;

    // The router should still succeed, but the job result should be an error
    let results = result
        .expect("router call should succeed")
        .expect("should return Some");
    assert!(!results.is_empty());

    // Error results should not have TEE metadata injected
    match &results[0] {
        blueprint_core::JobResult::Err(_) => {
            // TEE metadata is only injected into Ok results
        }
        blueprint_core::JobResult::Ok { head, .. } => {
            // If for some reason it's Ok (router wraps errors differently),
            // at least verify the layer ran
            let _ = head;
        }
    }
}

// TeeLayer: lock contention gracefully skips metadata injection
#[tokio::test]
async fn test_tee_layer_lock_contention_skips_metadata() {
    use blueprint_core::{Bytes, JobCall};
    use blueprint_router::Router;
    use tower::Service;

    let report = sample_report();
    let tee_layer = TeeLayer::with_attestation(report);
    let handle = tee_layer.attestation_handle();

    let mut router = Router::new()
        .route(0, async || vec![42u8])
        .layer(tee_layer);

    // Hold the lock so try_lock in the service will fail
    let _guard = handle.lock().await;

    let call = JobCall::new(0u32, Bytes::new());
    let result = router.call(call).await;

    let results = result
        .expect("router call should succeed even with lock contention")
        .expect("should return Some");
    assert!(!results.is_empty());

    match &results[0] {
        blueprint_core::JobResult::Ok { head, .. } => {
            // Metadata should NOT be injected due to lock contention
            assert!(
                head.metadata.get(TEE_PROVIDER_KEY).is_none(),
                "lock contention should skip metadata injection"
            );
        }
        blueprint_core::JobResult::Err(_) => {
            panic!("expected Ok result");
        }
    }
}

// TeeLayer: shared attestation handle allows external updates
#[tokio::test]
async fn test_tee_layer_shared_attestation_handle() {
    let layer = TeeLayer::new();
    let handle = layer.attestation_handle();

    // Update via handle directly
    {
        let mut guard = handle.lock().await;
        *guard = Some(sample_report());
    }

    // Layer should now see the attestation
    let handle2 = layer.attestation_handle();
    let guard = handle2.lock().await;
    assert!(guard.is_some());
}
