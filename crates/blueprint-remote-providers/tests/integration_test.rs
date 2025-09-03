use blueprint_remote_providers::{
    provider::{ProviderRegistry, RemoteInfrastructureProvider},
    testing::MockProvider,
    types::{DeploymentSpec, InstanceStatus, TunnelHub},
};
use std::sync::Arc;

#[tokio::test]
async fn test_provider_registry_integration() {
    let registry = ProviderRegistry::new();
    
    // Register multiple providers
    let aws_provider = Arc::new(MockProvider::new("aws-mock"));
    let gcp_provider = Arc::new(MockProvider::new("gcp-mock"));
    let docker_provider = Arc::new(MockProvider::new("docker-mock"));
    
    registry.register("aws", aws_provider.clone()).await;
    registry.register("gcp", gcp_provider.clone()).await;
    registry.register("docker", docker_provider.clone()).await;
    
    // List all providers
    let providers = registry.list().await;
    assert_eq!(providers.len(), 3);
    assert!(providers.contains(&"aws".to_string()));
    assert!(providers.contains(&"gcp".to_string()));
    assert!(providers.contains(&"docker".to_string()));
    
    // Deploy to each provider
    for provider_name in &providers {
        let provider = registry.get(provider_name).await.unwrap();
        
        let mut spec = DeploymentSpec::default();
        spec.name = format!("test-{}", provider_name);
        
        let instance = provider.deploy_instance(spec).await.unwrap();
        assert_eq!(instance.provider, provider.name());
        
        let status = provider.get_instance_status(&instance.id).await.unwrap();
        assert!(matches!(status, InstanceStatus::Running));
    }
}

#[tokio::test]
async fn test_multi_provider_deployment() {
    let registry = ProviderRegistry::new();
    
    // Create providers with different configurations
    let provider1 = Arc::new(MockProvider::new("region-us-west"));
    let provider2 = Arc::new(MockProvider::new("region-eu-central"));
    
    registry.register("us-west", provider1.clone()).await;
    registry.register("eu-central", provider2.clone()).await;
    
    // Deploy the same spec to multiple regions
    let spec = DeploymentSpec {
        name: "multi-region-app".to_string(),
        replicas: 2,
        ..Default::default()
    };
    
    let mut instances = Vec::new();
    
    for region in ["us-west", "eu-central"] {
        let provider = registry.get(region).await.unwrap();
        let instance = provider.deploy_instance(spec.clone()).await.unwrap();
        instances.push((region, instance));
    }
    
    // Verify all deployments succeeded
    assert_eq!(instances.len(), 2);
    
    for (region, instance) in &instances {
        let provider = registry.get(region).await.unwrap();
        let status = provider.get_instance_status(&instance.id).await.unwrap();
        assert!(matches!(status, InstanceStatus::Running));
    }
    
    // Clean up
    for (region, instance) in instances {
        let provider = registry.get(region).await.unwrap();
        provider.terminate_instance(&instance.id).await.unwrap();
    }
}

#[tokio::test]
async fn test_tunnel_establishment() {
    let provider = MockProvider::new("tunnel-test");
    
    let hub = TunnelHub::new("hub.example.com", 51820, "test-public-key");
    
    let tunnel = provider.establish_tunnel(&hub).await.unwrap();
    
    assert!(!tunnel.interface.is_empty());
    assert!(!tunnel.local_address.is_empty());
    assert!(!tunnel.remote_address.is_empty());
    assert_eq!(tunnel.peer_endpoint, "hub.example.com:51820");
}

#[tokio::test]
async fn test_resource_management() {
    let provider = MockProvider::new("resource-test");
    
    // Check available resources
    let resources = provider.get_available_resources().await.unwrap();
    assert!(resources.current_instances == 0);
    
    // Deploy instances
    let mut instances = Vec::new();
    for i in 0..3 {
        let mut spec = DeploymentSpec::default();
        spec.name = format!("instance-{}", i);
        let instance = provider.deploy_instance(spec).await.unwrap();
        instances.push(instance);
    }
    
    // Check instance list
    let deployed = provider.list_instances().await.unwrap();
    assert_eq!(deployed.len(), 3);
    
    // Scale one instance
    provider.scale_instance(&instances[0].id, 3).await.unwrap();
    
    // Terminate all instances
    for instance in instances {
        provider.terminate_instance(&instance.id).await.unwrap();
    }
    
    // Verify cleanup
    let remaining = provider.list_instances().await.unwrap();
    assert_eq!(remaining.len(), 0);
}

#[tokio::test]
async fn test_cost_estimation() {
    let provider = MockProvider::new("cost-test");
    
    let mut spec = DeploymentSpec::default();
    spec.resources.cpu = Some("4".to_string());
    spec.resources.memory = Some("8Gi".to_string());
    spec.replicas = 3;
    
    let cost = provider.estimate_cost(&spec).await.unwrap();
    
    assert!(cost.estimated_hourly >= 0.0);
    assert!(cost.estimated_monthly >= 0.0);
    assert_eq!(cost.currency, "USD");
}

#[tokio::test]
async fn test_error_handling() {
    let provider = MockProvider::new("error-test");
    
    // Set up provider to return errors
    provider.set_deployment_result(Err(blueprint_remote_providers::Error::DeploymentFailed(
        "Insufficient resources".to_string(),
    )))
    .await;
    
    let spec = DeploymentSpec::default();
    let result = provider.deploy_instance(spec).await;
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, blueprint_remote_providers::Error::DeploymentFailed(_)));
    }
}