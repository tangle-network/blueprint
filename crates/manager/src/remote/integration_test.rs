use super::provider_selector::{CloudProvider, ProviderPreferences, ProviderSelector, ResourceSpec};
use super::service::{RemoteDeploymentPolicy, RemoteDeploymentService};

#[tokio::test]
async fn test_provider_selection_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test GPU workload selection
    let selector = ProviderSelector::with_defaults();
    let gpu_spec = ResourceSpec {
        cpu: 4.0,
        memory_gb: 16.0,
        storage_gb: 100.0,
        gpu_count: Some(1),
        allow_spot: false,
    };
    
    let provider = selector.select_provider(&gpu_spec)?;
    assert_eq!(provider, CloudProvider::GCP, "GPU workloads should select GCP first");
    
    // Test CPU-intensive workload selection
    let cpu_spec = ResourceSpec {
        cpu: 16.0,
        memory_gb: 32.0,
        storage_gb: 200.0,
        gpu_count: None,
        allow_spot: false,
    };
    
    let provider = selector.select_provider(&cpu_spec)?;
    assert_eq!(provider, CloudProvider::Vultr, "CPU-intensive workloads should select Vultr first");
    
    // Test cost-optimized workload selection
    let cost_spec = ResourceSpec {
        cpu: 2.0,
        memory_gb: 4.0,
        storage_gb: 20.0,
        gpu_count: None,
        allow_spot: true,
    };
    
    let provider = selector.select_provider(&cost_spec)?;
    assert_eq!(provider, CloudProvider::Vultr, "Cost-optimized workloads should select Vultr first");
    
    Ok(())
}

#[tokio::test]
async fn test_remote_deployment_service_integration() -> Result<(), Box<dyn std::error::Error>> {
    let policy = RemoteDeploymentPolicy {
        provider_preferences: ProviderPreferences::default(),
        max_hourly_cost: Some(10.0),
        prefer_spot: true,
        auto_terminate_hours: Some(2),
    };
    
    let service = RemoteDeploymentService::new(policy).await?;
    
    // Test deployment registry initially empty
    let deployments = service.list_deployments().await;
    assert!(deployments.is_empty(), "Initial deployment registry should be empty");
    
    // Test cleanup of expired deployments (should not error on empty registry)
    service.cleanup_expired_deployments().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_custom_provider_preferences() -> Result<(), Box<dyn std::error::Error>> {
    let custom_preferences = ProviderPreferences {
        gpu_providers: vec![CloudProvider::AWS, CloudProvider::Azure],
        cpu_intensive: vec![CloudProvider::DigitalOcean, CloudProvider::GCP],
        memory_intensive: vec![CloudProvider::Azure, CloudProvider::Vultr],
        cost_optimized: vec![CloudProvider::DigitalOcean, CloudProvider::Vultr],
    };
    
    let selector = ProviderSelector::new(custom_preferences);
    
    // Test custom GPU preference
    let gpu_spec = ResourceSpec {
        cpu: 4.0,
        memory_gb: 16.0,
        storage_gb: 100.0,
        gpu_count: Some(1),
        allow_spot: false,
    };
    
    let provider = selector.select_provider(&gpu_spec)?;
    assert_eq!(provider, CloudProvider::AWS, "Custom GPU preferences should select AWS first");
    
    Ok(())
}

#[tokio::test]
async fn test_fallback_providers() -> Result<(), Box<dyn std::error::Error>> {
    let selector = ProviderSelector::with_defaults();
    
    let gpu_spec = ResourceSpec {
        cpu: 4.0,
        memory_gb: 16.0,
        storage_gb: 100.0,
        gpu_count: Some(1),
        allow_spot: false,
    };
    
    let fallbacks = selector.get_fallback_providers(&gpu_spec);
    
    // Should include CPU-intensive providers as fallback for GPU workloads
    assert!(fallbacks.contains(&CloudProvider::Vultr), "Should have Vultr as fallback");
    assert!(fallbacks.contains(&CloudProvider::DigitalOcean), "Should have DigitalOcean as fallback");
    
    // Should not include the primary selection (GCP)
    assert!(!fallbacks.contains(&CloudProvider::GCP), "Should not include primary provider in fallbacks");
    
    Ok(())
}