//! End-to-end tests for remote provider deployments
//! 
//! These tests verify real-world scenarios including:
//! - Multi-provider deployments
//! - Bridge connectivity and health checks
//! - Failover and recovery
//! - Resource management under load
//! - Cost tracking accuracy

use blueprint_remote_providers::{
    bridge::{RemoteBridgeManager, ConnectionStatus},
    docker::{DockerConfig, DockerProvider},
    kubernetes::{KubernetesConfig, KubernetesProvider},
    provider::{ProviderRegistry, RemoteInfrastructureProvider},
    types::{
        ContainerImage, DeploymentSpec, InstanceStatus, PortMapping, 
        Protocol, ResourceLimits, TunnelHub,
    },
};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{sleep, timeout};

/// Test deployment lifecycle across multiple providers
#[tokio::test]
#[ignore] // Requires real infrastructure
async fn test_multi_provider_deployment_lifecycle() {
    let registry = ProviderRegistry::new();
    let bridge_manager = RemoteBridgeManager::new();
    
    // Set up providers
    let docker_provider = setup_docker_provider().await;
    let k8s_provider = setup_kubernetes_provider().await;
    
    if docker_provider.is_none() && k8s_provider.is_none() {
        eprintln!("Skipping test: No providers available");
        return;
    }
    
    // Register available providers
    if let Some(docker) = docker_provider {
        registry.register("docker", docker.clone()).await;
    }
    if let Some(k8s) = k8s_provider {
        registry.register("kubernetes", k8s.clone()).await;
    }
    
    // Deploy to all available providers
    let spec = create_test_deployment_spec();
    let mut deployments = Vec::new();
    
    for provider_name in registry.list().await {
        let provider = registry.get(&provider_name).await.unwrap();
        
        // Deploy instance
        match provider.deploy_instance(spec.clone()).await {
            Ok(instance) => {
                println!("Deployed to {}: {}", provider_name, instance.id);
                
                // Establish bridge connection
                match bridge_manager.connect_to_instance(provider.clone(), &instance.id).await {
                    Ok(connection) => {
                        assert_eq!(connection.status, ConnectionStatus::Connected);
                        deployments.push((provider_name, provider, instance));
                    }
                    Err(e) => {
                        eprintln!("Bridge connection failed: {}", e);
                        // Clean up failed deployment
                        let _ = provider.terminate_instance(&instance.id).await;
                    }
                }
            }
            Err(e) => eprintln!("Deployment to {} failed: {}", provider_name, e),
        }
    }
    
    assert!(!deployments.is_empty(), "At least one deployment should succeed");
    
    // Wait for all deployments to be ready
    sleep(Duration::from_secs(5)).await;
    
    // Verify all deployments are running
    for (name, provider, instance) in &deployments {
        let status = provider.get_instance_status(&instance.id).await.unwrap();
        assert!(
            matches!(status, InstanceStatus::Running | InstanceStatus::Pending),
            "Instance on {} should be running or pending, got: {:?}",
            name,
            status
        );
        
        // Health check bridge connection
        let healthy = bridge_manager.health_check(&instance.id).await.unwrap();
        assert!(healthy, "Bridge connection to {} should be healthy", name);
    }
    
    // Clean up all deployments
    for (name, provider, instance) in deployments {
        provider.terminate_instance(&instance.id).await.unwrap();
        bridge_manager.disconnect_from_instance(&instance.id).await.unwrap();
        println!("Cleaned up deployment on {}", name);
    }
}

/// Test bridge connectivity and failover
#[tokio::test]
#[ignore] // Requires real infrastructure
async fn test_bridge_connectivity_and_failover() {
    let bridge_manager = RemoteBridgeManager::new();
    
    // Use mock provider for controlled testing
    let provider = Arc::new(blueprint_remote_providers::testing::MockProvider::new("test"));
    
    // Deploy primary instance
    let primary_spec = create_test_deployment_spec();
    let primary = provider.deploy_instance(primary_spec).await.unwrap();
    
    // Set up endpoint
    provider.set_endpoint_result(Ok(Some(blueprint_remote_providers::types::ServiceEndpoint {
        host: "test.example.com".to_string(),
        port: 8080,
        protocol: Protocol::TCP,
        tunnel_required: false,
    })))
    .await;
    
    // Connect bridge
    let connection = bridge_manager
        .connect_to_instance(provider.clone(), &primary.id)
        .await
        .unwrap();
    assert_eq!(connection.status, ConnectionStatus::Connected);
    
    // Simulate connection health checks over time
    for _ in 0..5 {
        let healthy = bridge_manager.health_check(&primary.id).await.unwrap();
        assert!(healthy, "Connection should remain healthy");
        sleep(Duration::from_millis(100)).await;
    }
    
    // Deploy backup instance
    let backup_spec = create_test_deployment_spec();
    let backup = provider.deploy_instance(backup_spec).await.unwrap();
    
    // Connect to backup
    let backup_connection = bridge_manager
        .connect_to_instance(provider.clone(), &backup.id)
        .await
        .unwrap();
    assert_eq!(backup_connection.status, ConnectionStatus::Connected);
    
    // Verify both connections are active
    let connections = bridge_manager.list_connections().await;
    assert_eq!(connections.len(), 2);
    for (_, status) in &connections {
        assert_eq!(*status, ConnectionStatus::Connected);
    }
    
    // Simulate primary failure by disconnecting
    bridge_manager.disconnect_from_instance(&primary.id).await.unwrap();
    
    // Verify backup is still connected
    let backup_healthy = bridge_manager.health_check(&backup.id).await.unwrap();
    assert!(backup_healthy, "Backup should remain healthy after primary failure");
    
    // Clean up
    bridge_manager.disconnect_from_instance(&backup.id).await.unwrap();
}

/// Test resource management and scaling
#[tokio::test]
#[ignore] // Requires real infrastructure
async fn test_resource_management_and_scaling() {
    let provider = setup_docker_provider().await;
    if provider.is_none() {
        eprintln!("Skipping test: Docker not available");
        return;
    }
    let provider = provider.unwrap();
    
    // Check initial resources
    let initial_resources = provider.get_available_resources().await.unwrap();
    let initial_instances = initial_resources.current_instances;
    
    // Deploy multiple instances
    let mut instances = Vec::new();
    for i in 0..3 {
        let mut spec = create_test_deployment_spec();
        spec.name = format!("resource-test-{}", i);
        spec.resources = ResourceLimits {
            cpu: Some("0.1".to_string()),
            memory: Some("64Mi".to_string()),
            storage: None,
        };
        
        match provider.deploy_instance(spec).await {
            Ok(instance) => instances.push(instance),
            Err(e) => eprintln!("Failed to deploy instance {}: {}", i, e),
        }
    }
    
    // Verify resource consumption
    let current_resources = provider.get_available_resources().await.unwrap();
    assert_eq!(
        current_resources.current_instances,
        initial_instances + instances.len() as u32,
        "Instance count should increase"
    );
    
    // Test scaling (Docker only supports 0 or 1)
    if !instances.is_empty() {
        let instance_id = &instances[0].id;
        
        // Scale to 0 (stop)
        provider.scale_instance(instance_id, 0).await.unwrap();
        sleep(Duration::from_secs(1)).await;
        
        let status = provider.get_instance_status(instance_id).await.unwrap();
        assert!(
            matches!(status, InstanceStatus::Stopped | InstanceStatus::Unknown),
            "Instance should be stopped after scaling to 0"
        );
        
        // Scale back to 1 (start)
        provider.scale_instance(instance_id, 1).await.unwrap();
        sleep(Duration::from_secs(1)).await;
        
        let status = provider.get_instance_status(instance_id).await.unwrap();
        assert!(
            matches!(status, InstanceStatus::Running | InstanceStatus::Pending),
            "Instance should be running after scaling to 1"
        );
    }
    
    // Clean up
    for instance in instances {
        let _ = provider.terminate_instance(&instance.id).await;
    }
    
    // Verify cleanup
    sleep(Duration::from_secs(2)).await;
    let final_resources = provider.get_available_resources().await.unwrap();
    assert_eq!(
        final_resources.current_instances,
        initial_instances,
        "Instance count should return to initial value after cleanup"
    );
}

/// Test cost estimation accuracy
#[tokio::test]
async fn test_cost_estimation_accuracy() {
    // Test with various resource configurations
    let configurations = vec![
        ("small", "0.5", "512Mi", 1, 0.05),   // Small instance
        ("medium", "2", "4Gi", 1, 0.2),       // Medium instance
        ("large", "4", "8Gi", 1, 0.4),        // Large instance
        ("scaled", "2", "4Gi", 5, 1.0),       // Scaled deployment
    ];
    
    // Test against different providers
    let providers: Vec<Arc<dyn RemoteInfrastructureProvider>> = vec![
        Arc::new(blueprint_remote_providers::testing::MockProvider::new("mock")),
    ];
    
    // Add real providers if available
    if let Some(docker) = setup_docker_provider().await {
        providers.push(docker);
    }
    
    for provider in providers {
        for (name, cpu, memory, replicas, max_hourly) in &configurations {
            let spec = DeploymentSpec {
                name: format!("cost-test-{}", name),
                resources: ResourceLimits {
                    cpu: Some(cpu.to_string()),
                    memory: Some(memory.to_string()),
                    storage: None,
                },
                replicas: *replicas,
                ..Default::default()
            };
            
            let cost = provider.estimate_cost(&spec).await.unwrap();
            
            // Verify cost structure
            assert_eq!(cost.currency, "USD");
            assert!(cost.estimated_hourly > 0.0, "Hourly cost should be positive");
            assert!(cost.estimated_monthly > 0.0, "Monthly cost should be positive");
            assert!(
                cost.estimated_monthly > cost.estimated_hourly * 24.0 * 28.0,
                "Monthly should be at least 28 days of hourly"
            );
            assert!(
                cost.estimated_monthly < cost.estimated_hourly * 24.0 * 31.0,
                "Monthly should be at most 31 days of hourly"
            );
            
            // Check reasonable bounds
            assert!(
                cost.estimated_hourly <= *max_hourly * 2.0,
                "{} hourly cost ${:.2} exceeds expected maximum ${:.2}",
                name,
                cost.estimated_hourly,
                max_hourly * 2.0
            );
            
            // Verify breakdown
            assert!(cost.breakdown.contains_key("compute"));
            assert!(cost.breakdown.contains_key("memory"));
            let compute_cost = cost.breakdown.get("compute").unwrap();
            let memory_cost = cost.breakdown.get("memory").unwrap();
            assert!(compute_cost >= &0.0);
            assert!(memory_cost >= &0.0);
        }
    }
}

/// Test concurrent deployments and race conditions
#[tokio::test]
async fn test_concurrent_deployments() {
    let provider = Arc::new(blueprint_remote_providers::testing::MockProvider::new("concurrent"));
    let bridge_manager = Arc::new(RemoteBridgeManager::new());
    
    // Deploy multiple instances concurrently
    let mut handles = Vec::new();
    for i in 0..5 {
        let provider_clone = provider.clone();
        let bridge_clone = bridge_manager.clone();
        
        let handle = tokio::spawn(async move {
            let mut spec = create_test_deployment_spec();
            spec.name = format!("concurrent-{}", i);
            
            // Deploy
            let instance = provider_clone.deploy_instance(spec).await.unwrap();
            
            // Connect bridge
            provider_clone.set_endpoint_result(Ok(Some(
                blueprint_remote_providers::types::ServiceEndpoint {
                    host: format!("host-{}.example.com", i),
                    port: 8080 + i as u16,
                    protocol: Protocol::TCP,
                    tunnel_required: i % 2 == 0, // Alternate tunnel requirement
                }
            )))
            .await;
            
            let connection = bridge_clone
                .connect_to_instance(provider_clone.clone(), &instance.id)
                .await
                .unwrap();
            
            (instance, connection)
        });
        
        handles.push(handle);
    }
    
    // Wait for all deployments
    let mut results = Vec::new();
    for handle in handles {
        match timeout(Duration::from_secs(10), handle).await {
            Ok(Ok(result)) => results.push(result),
            Ok(Err(e)) => eprintln!("Deployment failed: {}", e),
            Err(_) => eprintln!("Deployment timed out"),
        }
    }
    
    assert!(results.len() >= 3, "At least 3 deployments should succeed");
    
    // Verify all connections are healthy
    for (instance, connection) in &results {
        assert_eq!(connection.status, ConnectionStatus::Connected);
        let healthy = bridge_manager.health_check(&instance.id).await.unwrap();
        assert!(healthy);
    }
    
    // Clean up concurrently
    let mut cleanup_handles = Vec::new();
    for (instance, _) in results {
        let provider_clone = provider.clone();
        let bridge_clone = bridge_manager.clone();
        
        let handle = tokio::spawn(async move {
            provider_clone.terminate_instance(&instance.id).await.unwrap();
            bridge_clone.disconnect_from_instance(&instance.id).await.unwrap();
        });
        
        cleanup_handles.push(handle);
    }
    
    // Wait for cleanup
    for handle in cleanup_handles {
        let _ = timeout(Duration::from_secs(5), handle).await;
    }
}

/// Test error recovery and retry logic
#[tokio::test]
async fn test_error_recovery() {
    let provider = Arc::new(blueprint_remote_providers::testing::MockProvider::new("error-test"));
    let bridge_manager = RemoteBridgeManager::new();
    
    // Simulate deployment failure
    provider.set_deployment_result(Err(blueprint_remote_providers::Error::DeploymentFailed(
        "Simulated failure".to_string()
    )))
    .await;
    
    let spec = create_test_deployment_spec();
    let result = provider.deploy_instance(spec.clone()).await;
    assert!(result.is_err());
    
    // Recover and retry
    provider.set_deployment_result(Ok(blueprint_remote_providers::RemoteInstance::new(
        "recovered",
        "test-instance",
        "error-test",
    )))
    .await;
    
    let instance = provider.deploy_instance(spec).await.unwrap();
    assert_eq!(instance.id.as_str(), "recovered");
    
    // Test connection recovery
    provider.set_endpoint_result(Ok(None)).await; // No endpoint initially
    
    let connection_result = bridge_manager.connect_to_instance(provider.clone(), &instance.id).await;
    assert!(connection_result.is_err());
    
    // Fix endpoint and retry
    provider.set_endpoint_result(Ok(Some(blueprint_remote_providers::types::ServiceEndpoint {
        host: "recovered.example.com".to_string(),
        port: 8080,
        protocol: Protocol::TCP,
        tunnel_required: false,
    })))
    .await;
    
    let connection = bridge_manager.connect_to_instance(provider.clone(), &instance.id).await.unwrap();
    assert_eq!(connection.status, ConnectionStatus::Connected);
    
    // Clean up
    bridge_manager.disconnect_from_instance(&instance.id).await.unwrap();
}

// Helper functions

async fn setup_docker_provider() -> Option<Arc<dyn RemoteInfrastructureProvider>> {
    match DockerProvider::new("docker-test", DockerConfig::default()).await {
        Ok(provider) => Some(Arc::new(provider)),
        Err(e) => {
            eprintln!("Docker provider not available: {}", e);
            None
        }
    }
}

async fn setup_kubernetes_provider() -> Option<Arc<dyn RemoteInfrastructureProvider>> {
    let config = KubernetesConfig {
        namespace: "blueprint-test".to_string(),
        ..Default::default()
    };
    
    match KubernetesProvider::new("k8s-test", config).await {
        Ok(provider) => Some(Arc::new(provider)),
        Err(e) => {
            eprintln!("Kubernetes provider not available: {}", e);
            None
        }
    }
}

fn create_test_deployment_spec() -> DeploymentSpec {
    DeploymentSpec {
        name: "test-deployment".to_string(),
        image: ContainerImage {
            repository: "nginx".to_string(),
            tag: "alpine".to_string(),
            pull_policy: blueprint_remote_providers::types::PullPolicy::IfNotPresent,
        },
        resources: ResourceLimits {
            cpu: Some("100m".to_string()),
            memory: Some("128Mi".to_string()),
            storage: None,
        },
        ports: vec![PortMapping {
            name: "http".to_string(),
            container_port: 80,
            host_port: None,
            protocol: Protocol::TCP,
        }],
        replicas: 1,
        ..Default::default()
    }
}