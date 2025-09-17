//! Tests for manager <-> remote provider integration bridge
//!
//! These tests verify the critical integration points

use blueprint_remote_providers::{
    deployment::manager_integration::{RemoteDeploymentRegistry, TtlManager},
    deployment::tracker::DeploymentTracker,
    resources::ResourceSpec,
    remote::CloudProvider,
};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::mpsc;

/// Test that manager correctly triggers remote deployments
#[tokio::test]
async fn test_manager_triggers_deployment() {
    let temp_dir = TempDir::new().unwrap();
    let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());
    let registry = Arc::new(RemoteDeploymentRegistry::new(tracker.clone()));
    
    // Simulate manager triggering a deployment
    let blueprint_id = 1;
    let service_id = 100;
    
    let config = blueprint_remote_providers::deployment::manager_integration::RemoteDeploymentConfig {
        deployment_type: blueprint_remote_providers::deployment::tracker::DeploymentType::AwsEc2,
        provider: Some(CloudProvider::AWS),
        region: Some("us-east-1".to_string()),
        instance_id: "i-test123".to_string(),
        resource_spec: ResourceSpec::minimal(),
        ttl_seconds: Some(3600),
        deployed_at: chrono::Utc::now(),
    };
    
    registry.register(blueprint_id, service_id, config.clone()).await;
    
    // Verify deployment is tracked
    let retrieved = registry.get(blueprint_id, service_id).await;
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().instance_id, "i-test123");
}

/// Test TTL manager expiry detection
#[tokio::test]
async fn test_ttl_expiry_detection() {
    let temp_dir = TempDir::new().unwrap();
    let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());
    let registry = Arc::new(RemoteDeploymentRegistry::new(tracker.clone()));
    
    let (tx, _rx) = mpsc::unbounded_channel();
    let ttl_manager = TtlManager::new(registry.clone(), tx);
    
    // Register deployment with 1 second TTL
    let config = blueprint_remote_providers::deployment::manager_integration::RemoteDeploymentConfig {
        deployment_type: blueprint_remote_providers::deployment::tracker::DeploymentType::AwsEc2,
        provider: Some(CloudProvider::AWS),
        region: Some("us-east-1".to_string()),
        instance_id: "i-ttl-test".to_string(),
        resource_spec: ResourceSpec::minimal(),
        ttl_seconds: Some(1),
        deployed_at: chrono::Utc::now(),
    };
    
    let blueprint_id = 2;
    let service_id = 200;
    registry.register(blueprint_id, service_id, config).await;
    ttl_manager.register_ttl(blueprint_id, service_id, 1).await;
    
    // Wait for TTL to expire
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    
    // Check for expired services
    let expired = ttl_manager.check_expired_services().await.unwrap();
    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0], (blueprint_id, service_id));
}

/// Test service lifecycle tracking
#[tokio::test]
async fn test_service_lifecycle_tracking() {
    let temp_dir = TempDir::new().unwrap();
    let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());
    let registry = Arc::new(RemoteDeploymentRegistry::new(tracker.clone()));
    
    let blueprint_id = 3;
    let service_id = 300;
    
    // Service initiated
    let config = blueprint_remote_providers::deployment::manager_integration::RemoteDeploymentConfig {
        deployment_type: blueprint_remote_providers::deployment::tracker::DeploymentType::AwsEc2,
        provider: Some(CloudProvider::AWS),
        region: Some("us-west-2".to_string()),
        instance_id: "i-lifecycle".to_string(),
        resource_spec: ResourceSpec::basic(),
        ttl_seconds: None,
        deployed_at: chrono::Utc::now(),
    };
    
    registry.register(blueprint_id, service_id, config).await;
    assert!(registry.get(blueprint_id, service_id).await.is_some());
    
    // Service terminated
    registry.cleanup(blueprint_id, service_id).await.unwrap();
    assert!(registry.get(blueprint_id, service_id).await.is_none());
}

/// Test concurrent deployments for same service
#[tokio::test]
async fn test_concurrent_deployment_safety() {
    let temp_dir = TempDir::new().unwrap();
    let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());
    let registry = Arc::new(RemoteDeploymentRegistry::new(tracker.clone()));
    
    let blueprint_id = 4;
    let service_id = 400;
    
    // Attempt concurrent registrations for same service
    let reg1 = registry.clone();
    let reg2 = registry.clone();
    
    let handle1 = tokio::spawn(async move {
        let config = blueprint_remote_providers::deployment::manager_integration::RemoteDeploymentConfig {
            deployment_type: blueprint_remote_providers::deployment::tracker::DeploymentType::AwsEc2,
            provider: Some(CloudProvider::AWS),
            region: Some("eu-west-1".to_string()),
            instance_id: "i-concurrent-1".to_string(),
            resource_spec: ResourceSpec::minimal(),
            ttl_seconds: None,
            deployed_at: chrono::Utc::now(),
        };
        reg1.register(blueprint_id, service_id, config).await;
    });
    
    let handle2 = tokio::spawn(async move {
        let config = blueprint_remote_providers::deployment::manager_integration::RemoteDeploymentConfig {
            deployment_type: blueprint_remote_providers::deployment::tracker::DeploymentType::GcpGce,
            provider: Some(CloudProvider::GCP),
            region: Some("us-central1".to_string()),
            instance_id: "i-concurrent-2".to_string(),
            resource_spec: ResourceSpec::minimal(),
            ttl_seconds: None,
            deployed_at: chrono::Utc::now(),
        };
        reg2.register(blueprint_id, service_id, config).await;
    });
    
    handle1.await.unwrap();
    handle2.await.unwrap();
    
    // Only one deployment should win (last write wins currently)
    let final_deployment = registry.get(blueprint_id, service_id).await.unwrap();
    assert!(
        final_deployment.instance_id == "i-concurrent-1" || 
        final_deployment.instance_id == "i-concurrent-2"
    );
}

/// Test that cleanup properly releases resources
#[tokio::test]
async fn test_cleanup_releases_resources() {
    let temp_dir = TempDir::new().unwrap();
    let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());
    let registry = Arc::new(RemoteDeploymentRegistry::new(tracker.clone()));
    
    // Register multiple deployments
    for i in 0..5 {
        let config = blueprint_remote_providers::deployment::manager_integration::RemoteDeploymentConfig {
            deployment_type: blueprint_remote_providers::deployment::tracker::DeploymentType::AwsEc2,
            provider: Some(CloudProvider::AWS),
            region: Some("us-east-1".to_string()),
            instance_id: format!("i-cleanup-{}", i),
            resource_spec: ResourceSpec::minimal(),
            ttl_seconds: None,
            deployed_at: chrono::Utc::now(),
        };
        registry.register(i, i * 100, config).await;
    }
    
    // Cleanup all
    for i in 0..5 {
        registry.cleanup(i, i * 100).await.unwrap();
    }
    
    // Verify all cleaned up
    for i in 0..5 {
        assert!(registry.get(i, i * 100).await.is_none());
    }
}