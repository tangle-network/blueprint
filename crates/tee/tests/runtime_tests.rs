//! Tests for the runtime backend and registry.

use blueprint_tee::config::TeeProvider;
use blueprint_tee::runtime::backend::*;
use blueprint_tee::runtime::direct::*;
use blueprint_tee::runtime::registry::BackendRegistry;

#[tokio::test]
async fn test_direct_backend_deploy() {
    let backend = DirectBackend::tdx();
    let req = TeeDeployRequest::new("test-image:latest");
    let handle = backend.deploy(req).await.unwrap();

    assert!(!handle.id.is_empty());
    assert_eq!(handle.provider, TeeProvider::IntelTdx);
    assert_eq!(handle.metadata.get("backend").unwrap(), "direct");
}

#[tokio::test]
async fn test_direct_backend_lifecycle() {
    let backend = DirectBackend::sev_snp();
    let req = TeeDeployRequest::new("test-image:latest")
        .with_env("FOO", "bar")
        .with_provider(TeeProvider::AmdSevSnp);

    let handle = backend.deploy(req).await.unwrap();
    assert_eq!(handle.provider, TeeProvider::AmdSevSnp);

    // Check status
    let status = backend.status(&handle).await.unwrap();
    assert_eq!(status, TeeDeploymentStatus::Running);

    // Get attestation
    let report = backend.get_attestation(&handle).await.unwrap();
    assert_eq!(report.provider, TeeProvider::AmdSevSnp);

    // Stop
    backend.stop(&handle).await.unwrap();
    let status = backend.status(&handle).await.unwrap();
    assert_eq!(status, TeeDeploymentStatus::Stopped);

    // Destroy
    backend.destroy(&handle).await.unwrap();
    assert!(backend.status(&handle).await.is_err());
}

#[tokio::test]
async fn test_direct_backend_multiple_deployments() {
    let backend = DirectBackend::tdx();

    let h1 = backend.deploy(TeeDeployRequest::new("img1")).await.unwrap();
    let h2 = backend.deploy(TeeDeployRequest::new("img2")).await.unwrap();

    assert_ne!(h1.id, h2.id);

    // Both should be running
    assert_eq!(
        backend.status(&h1).await.unwrap(),
        TeeDeploymentStatus::Running
    );
    assert_eq!(
        backend.status(&h2).await.unwrap(),
        TeeDeploymentStatus::Running
    );

    // Destroy one, other should still be running
    backend.destroy(&h1).await.unwrap();
    assert!(backend.status(&h1).await.is_err());
    assert_eq!(
        backend.status(&h2).await.unwrap(),
        TeeDeploymentStatus::Running
    );
}

#[test]
fn test_deploy_request_builder() {
    let req = TeeDeployRequest::new("my-image:v1")
        .with_env("KEY", "value")
        .with_provider(TeeProvider::IntelTdx);

    assert_eq!(req.image, "my-image:v1");
    assert_eq!(req.env.get("KEY").unwrap(), "value");
    assert_eq!(req.preferred_provider, Some(TeeProvider::IntelTdx));
}

#[test]
fn test_deploy_request_serde() {
    let req = TeeDeployRequest::new("test:latest").with_env("A", "B");
    let json = serde_json::to_string(&req).unwrap();
    let parsed: TeeDeployRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.image, "test:latest");
    assert_eq!(parsed.env.get("A").unwrap(), "B");
}

#[test]
fn test_deployment_handle_serde() {
    let handle = TeeDeploymentHandle {
        id: "test-1".to_string(),
        provider: TeeProvider::IntelTdx,
        metadata: Default::default(),
        cached_attestation: None,
        port_mapping: Default::default(),
        lifecycle_policy: blueprint_tee::RuntimeLifecyclePolicy::CloudManaged,
    };
    let json = serde_json::to_string(&handle).unwrap();
    let parsed: TeeDeploymentHandle = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.id, "test-1");
    assert_eq!(parsed.provider, TeeProvider::IntelTdx);
}

#[tokio::test]
async fn test_backend_registry() {
    let mut registry = BackendRegistry::new();

    assert!(!registry.has_provider(TeeProvider::IntelTdx));

    registry.register(TeeProvider::IntelTdx, DirectBackend::tdx());
    assert!(registry.has_provider(TeeProvider::IntelTdx));
    assert!(!registry.has_provider(TeeProvider::AmdSevSnp));

    let providers = registry.providers();
    assert_eq!(providers.len(), 1);
    assert!(providers.contains(&"intel_tdx".to_string()));
}

#[tokio::test]
async fn test_backend_registry_deploy() {
    let mut registry = BackendRegistry::new();
    registry.register(TeeProvider::IntelTdx, DirectBackend::tdx());

    let handle = registry
        .deploy(TeeProvider::IntelTdx, TeeDeployRequest::new("test"))
        .await
        .unwrap();
    assert_eq!(handle.provider, TeeProvider::IntelTdx);
}

#[tokio::test]
async fn test_backend_registry_deploy_unregistered() {
    let registry = BackendRegistry::new();
    let result = registry
        .deploy(TeeProvider::AwsNitro, TeeDeployRequest::new("test"))
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_direct_backend_cached_attestation() {
    let backend = DirectBackend::tdx();
    let req = TeeDeployRequest::new("test-image:latest");
    let handle = backend.deploy(req).await.unwrap();

    // Before any attestation, cache should be None
    let cached = backend.cached_attestation(&handle).await.unwrap();
    assert!(cached.is_none());

    // Get attestation (which caches it)
    let report = backend.get_attestation(&handle).await.unwrap();
    assert_eq!(report.provider, TeeProvider::IntelTdx);

    // Now cached attestation should be Some
    let cached = backend.cached_attestation(&handle).await.unwrap();
    assert!(cached.is_some());
    assert_eq!(cached.unwrap().provider, TeeProvider::IntelTdx);
}

#[tokio::test]
async fn test_direct_backend_derive_public_key() {
    let backend = DirectBackend::tdx();
    let req = TeeDeployRequest::new("test-image:latest");
    let handle = backend.deploy(req).await.unwrap();

    let pubkey = backend.derive_public_key(&handle).await.unwrap();
    assert!(!pubkey.key.is_empty());
    assert_eq!(pubkey.key_type, "x25519");
    assert!(!pubkey.fingerprint.is_empty());
}

#[tokio::test]
async fn test_deploy_request_extra_ports() {
    let backend = DirectBackend::tdx();
    let req = TeeDeployRequest::new("test-image:latest").with_extra_ports([8080, 9090]);
    let handle = backend.deploy(req).await.unwrap();

    // Direct backend maps 1:1
    assert_eq!(handle.port_mapping.len(), 2);
    assert_eq!(handle.port_mapping.get(&8080), Some(&8080));
    assert_eq!(handle.port_mapping.get(&9090), Some(&9090));
}

#[tokio::test]
async fn test_deployment_handle_lifecycle_policy() {
    let backend = DirectBackend::tdx();
    let req = TeeDeployRequest::new("test-image:latest");
    let handle = backend.deploy(req).await.unwrap();

    assert_eq!(
        handle.lifecycle_policy,
        blueprint_tee::RuntimeLifecyclePolicy::CloudManaged
    );
}

#[tokio::test]
async fn test_backend_registry_get_attestation() {
    let mut registry = BackendRegistry::new();
    registry.register(TeeProvider::IntelTdx, DirectBackend::tdx());

    let handle = registry
        .deploy(TeeProvider::IntelTdx, TeeDeployRequest::new("test"))
        .await
        .unwrap();

    let report = registry.get_attestation(&handle).await.unwrap();
    assert_eq!(report.provider, TeeProvider::IntelTdx);
}

// BackendRegistry lifecycle delegation tests

#[tokio::test]
async fn test_backend_registry_status() {
    let mut registry = BackendRegistry::new();
    registry.register(TeeProvider::IntelTdx, DirectBackend::tdx());

    let handle = registry
        .deploy(TeeProvider::IntelTdx, TeeDeployRequest::new("test"))
        .await
        .unwrap();

    let status = registry.status(&handle).await.unwrap();
    assert_eq!(status, TeeDeploymentStatus::Running);
}

#[tokio::test]
async fn test_backend_registry_stop() {
    let mut registry = BackendRegistry::new();
    registry.register(TeeProvider::IntelTdx, DirectBackend::tdx());

    let handle = registry
        .deploy(TeeProvider::IntelTdx, TeeDeployRequest::new("test"))
        .await
        .unwrap();

    // Stop should succeed
    registry.stop(&handle).await.unwrap();

    // Status should be Stopped
    let status = registry.status(&handle).await.unwrap();
    assert_eq!(status, TeeDeploymentStatus::Stopped);
}

#[tokio::test]
async fn test_backend_registry_destroy() {
    let mut registry = BackendRegistry::new();
    registry.register(TeeProvider::IntelTdx, DirectBackend::tdx());

    let handle = registry
        .deploy(TeeProvider::IntelTdx, TeeDeployRequest::new("test"))
        .await
        .unwrap();

    // Destroy should succeed
    registry.destroy(&handle).await.unwrap();

    // Status should now fail (deployment gone)
    assert!(registry.status(&handle).await.is_err());
}

#[tokio::test]
async fn test_backend_registry_stop_unregistered_provider() {
    let registry = BackendRegistry::new();
    let handle = TeeDeploymentHandle {
        id: "fake-1".to_string(),
        provider: TeeProvider::AwsNitro,
        metadata: Default::default(),
        cached_attestation: None,
        port_mapping: Default::default(),
        lifecycle_policy: blueprint_tee::RuntimeLifecyclePolicy::CloudManaged,
    };
    assert!(registry.stop(&handle).await.is_err());
    assert!(registry.destroy(&handle).await.is_err());
    assert!(registry.status(&handle).await.is_err());
}

#[tokio::test]
async fn test_backend_registry_full_lifecycle() {
    let mut registry = BackendRegistry::new();
    registry.register(TeeProvider::AmdSevSnp, DirectBackend::sev_snp());

    let handle = registry
        .deploy(
            TeeProvider::AmdSevSnp,
            TeeDeployRequest::new("lifecycle-test"),
        )
        .await
        .unwrap();

    // Running
    assert_eq!(
        registry.status(&handle).await.unwrap(),
        TeeDeploymentStatus::Running
    );

    // Get attestation
    let report = registry.get_attestation(&handle).await.unwrap();
    assert_eq!(report.provider, TeeProvider::AmdSevSnp);

    // Cached attestation
    let cached = registry.cached_attestation(&handle).await.unwrap();
    assert!(cached.is_some());

    // Derive public key
    let pubkey = registry.derive_public_key(&handle).await.unwrap();
    assert!(!pubkey.key.is_empty());

    // Stop
    registry.stop(&handle).await.unwrap();
    assert_eq!(
        registry.status(&handle).await.unwrap(),
        TeeDeploymentStatus::Stopped
    );

    // Destroy
    registry.destroy(&handle).await.unwrap();
    assert!(registry.status(&handle).await.is_err());
}

#[tokio::test]
async fn test_backend_registry_multiple_providers() {
    let mut registry = BackendRegistry::new();
    registry.register(TeeProvider::IntelTdx, DirectBackend::tdx());
    registry.register(TeeProvider::AmdSevSnp, DirectBackend::sev_snp());

    assert!(registry.has_provider(TeeProvider::IntelTdx));
    assert!(registry.has_provider(TeeProvider::AmdSevSnp));
    assert!(!registry.has_provider(TeeProvider::AwsNitro));

    let providers = registry.providers();
    assert_eq!(providers.len(), 2);

    // Deploy on each
    let h1 = registry
        .deploy(TeeProvider::IntelTdx, TeeDeployRequest::new("tdx-img"))
        .await
        .unwrap();
    let h2 = registry
        .deploy(TeeProvider::AmdSevSnp, TeeDeployRequest::new("snp-img"))
        .await
        .unwrap();

    assert_eq!(h1.provider, TeeProvider::IntelTdx);
    assert_eq!(h2.provider, TeeProvider::AmdSevSnp);
}

// Edge case: deploy request with empty extra ports
#[tokio::test]
async fn test_deploy_request_empty_extra_ports() {
    let backend = DirectBackend::tdx();
    let req = TeeDeployRequest::new("test-image:latest").with_extra_ports(std::iter::empty());
    let handle = backend.deploy(req).await.unwrap();
    assert!(handle.port_mapping.is_empty());
}

// Edge case: destroy a non-existent deployment
#[tokio::test]
async fn test_direct_backend_destroy_nonexistent() {
    let backend = DirectBackend::tdx();
    let handle = TeeDeploymentHandle {
        id: "nonexistent-123".to_string(),
        provider: TeeProvider::IntelTdx,
        metadata: Default::default(),
        cached_attestation: None,
        port_mapping: Default::default(),
        lifecycle_policy: blueprint_tee::RuntimeLifecyclePolicy::CloudManaged,
    };
    // Destroy of non-existent deployment should not error (idempotent)
    // It just removes from the map, and if not present, that's fine
    backend.destroy(&handle).await.unwrap();
}

// Edge case: stop a non-existent deployment
#[tokio::test]
async fn test_direct_backend_stop_nonexistent() {
    let backend = DirectBackend::tdx();
    let handle = TeeDeploymentHandle {
        id: "nonexistent-456".to_string(),
        provider: TeeProvider::IntelTdx,
        metadata: Default::default(),
        cached_attestation: None,
        port_mapping: Default::default(),
        lifecycle_policy: blueprint_tee::RuntimeLifecyclePolicy::CloudManaged,
    };
    // Stop of non-existent deployment should error
    assert!(backend.stop(&handle).await.is_err());
}
