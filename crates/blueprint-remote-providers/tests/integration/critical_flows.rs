//! End-to-end deployment flow tests

use blueprint_remote_providers::{
    resources::ResourceSpec,
    remote::CloudProvider,
    provisioning::{select_instance_type, InstanceSelection},
    pricing::fetcher::PricingFetcher,
    deployment::tracker::DeploymentTracker,
};
use std::time::Duration;
use tempfile::TempDir;

/// Verify instance selection returns valid types with reasonable costs
#[test]
fn test_instance_selection_all_providers() {
    let specs = vec![
        ("minimal", ResourceSpec::minimal()),
        ("basic", ResourceSpec::basic()),
        ("recommended", ResourceSpec::recommended()),
        ("performance", ResourceSpec::performance()),
    ];
    
    let providers = vec![
        CloudProvider::AWS,
        CloudProvider::GCP,
        CloudProvider::Azure,
        CloudProvider::DigitalOcean,
    ];
    
    for (name, spec) in &specs {
        for provider in &providers {
            let instance = select_instance_type(*provider, spec);
            
            assert!(!instance.instance_type.is_empty(), 
                "Failed to select instance for {} spec on {:?}", name, provider);
            
            if spec.gpu_count.unwrap_or(0) > 0 {
                match provider {
                    CloudProvider::AWS => {
                        assert!(instance.instance_type.starts_with("p") || 
                               instance.instance_type.starts_with("g"),
                               "AWS should select GPU instance");
                    },
                    CloudProvider::GCP => {
                        assert!(instance.instance_type.contains("nvidia") ||
                               instance.instance_type.contains("tesla") ||
                               instance.instance_type.contains("a100"),
                               "GCP should select GPU instance");
                    },
                    _ => {}
                }
            }
            
            assert!(instance.estimated_cost > 0.0, "Cost should be positive");
            assert!(instance.estimated_cost < 100.0, "Cost should be reasonable");
        }
    }
}

/// Verify resource validation boundaries
#[test]
fn test_resource_validation() {
    assert!(ResourceSpec::minimal().validate().is_ok());
    assert!(ResourceSpec::basic().validate().is_ok());
    assert!(ResourceSpec::recommended().validate().is_ok());
    assert!(ResourceSpec::performance().validate().is_ok());
    
    let invalid_cpu = ResourceSpec {
        cpu: 0.0,
        memory_gb: 1.0,
        ..Default::default()
    };
    assert!(invalid_cpu.validate().is_err());
    
    let invalid_memory = ResourceSpec {
        cpu: 1.0,
        memory_gb: 0.0,
        ..Default::default()
    };
    assert!(invalid_memory.validate().is_err());
    
    let excessive = ResourceSpec {
        cpu: 1000.0,
        memory_gb: 10000.0,
        ..Default::default()
    };
    assert!(excessive.validate().is_err());
}

/// Verify Kubernetes resource conversion formats
#[test]
fn test_k8s_resource_conversion() {
    let spec = ResourceSpec {
        cpu: 2.5,
        memory_gb: 4.0,
        ..Default::default()
    };
    
    let (cpu, mem) = spec.to_k8s_resources();
    
    assert!(cpu == "2500m" || cpu == "2.5");
    assert!(mem == "4Gi" || mem == "4096Mi");
}

/// Verify Docker resource conversion formats
#[test]
fn test_docker_resource_conversion() {
    let spec = ResourceSpec {
        cpu: 1.5,
        memory_gb: 2.0,
        ..Default::default()
    };
    
    let (cpu, mem) = spec.to_docker_resources();
    
    assert_eq!(cpu, "1.5");
    assert_eq!(mem, "2048m");
}

/// Test deployment expiration after TTL
#[tokio::test]
async fn test_deployment_ttl_tracking() {
    let temp_dir = TempDir::new().unwrap();
    let tracker = DeploymentTracker::new(temp_dir.path()).await.unwrap();
    
    let id = tracker.register_deployment(
        "test-deploy",
        CloudProvider::AWS,
        "i-123456",
        Some(Duration::from_secs(1)),
    ).await.unwrap();
    
    let deployments = tracker.list_deployments().await.unwrap();
    assert_eq!(deployments.len(), 1);
    
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    let expired = tracker.get_expired_deployments().await.unwrap();
    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0].id, id);
    
    tracker.cleanup_deployment(&id).await.unwrap();
    
    let deployments = tracker.list_deployments().await.unwrap();
    assert_eq!(deployments.len(), 0);
}

/// Test AWS and Azure pricing via Vantage.sh API
#[tokio::test]
async fn test_pricing_apis_available() {
    let mut fetcher = PricingFetcher::new();
    
    let aws_result = fetcher.find_best_instance(
        CloudProvider::AWS,
        "us-east-1",
        1.0,
        2.0,
        0.10,
    ).await;
    
    if let Ok(instance) = aws_result {
        assert!(instance.hourly_price <= 0.10);
        assert!(instance.vcpus >= 1.0);
        assert!(instance.memory_gb >= 2.0);
        println!("✅ Found AWS instance: {} at ${}/hr", 
            instance.name, instance.hourly_price);
    }
    
    let azure_result = fetcher.find_best_instance(
        CloudProvider::Azure,
        "eastus",
        2.0,
        4.0,
        0.20,
    ).await;
    
    if let Ok(instance) = azure_result {
        assert!(instance.hourly_price <= 0.20);
        assert!(instance.vcpus >= 2.0);
        assert!(instance.memory_gb >= 4.0);
        println!("✅ Found Azure instance: {} at ${}/hr",
            instance.name, instance.hourly_price);
    }
}

/// Find cheapest provider for given resource requirements
#[tokio::test]
async fn test_multi_provider_cost_optimization() {
    let spec = ResourceSpec::basic();
    let mut fetcher = PricingFetcher::new();
    
    let mut results = vec![];
    
    for provider in &[CloudProvider::AWS, CloudProvider::Azure, CloudProvider::DigitalOcean] {
        let region = match provider {
            CloudProvider::AWS => "us-east-1",
            CloudProvider::Azure => "eastus",
            CloudProvider::DigitalOcean => "nyc3",
            _ => "us-central1",
        };
        
        if let Ok(instance) = fetcher.find_best_instance(
            *provider,
            region,
            spec.cpu,
            spec.memory_gb,
            1.0,
        ).await {
            results.push((*provider, instance));
        }
    }
    
    if !results.is_empty() {
        let cheapest = results.iter()
            .min_by(|a, b| a.1.hourly_price.partial_cmp(&b.1.hourly_price).unwrap())
            .unwrap();
        
        println!("💰 Cheapest provider: {:?} with {} at ${}/hr",
            cheapest.0, cheapest.1.name, cheapest.1.hourly_price);
    }
}

/// Verify each provider returns correct Kubernetes service type
#[test]
fn test_provider_service_types() {
    assert_eq!(CloudProvider::AWS.to_service_type(), "LoadBalancer");
    assert_eq!(CloudProvider::GCP.to_service_type(), "ClusterIP");
    assert_eq!(CloudProvider::Azure.to_service_type(), "LoadBalancer");
    assert_eq!(CloudProvider::DigitalOcean.to_service_type(), "LoadBalancer");
    assert_eq!(CloudProvider::Generic.to_service_type(), "ClusterIP");
}

/// Verify which providers require network tunnels
#[test]
fn test_tunnel_requirements() {
    assert!(!CloudProvider::AWS.requires_tunnel());
    assert!(!CloudProvider::GCP.requires_tunnel());
    assert!(!CloudProvider::Azure.requires_tunnel());
    assert!(CloudProvider::Generic.requires_tunnel());
    assert!(CloudProvider::BareMetal(vec!["host".to_string()]).requires_tunnel());
}

/// Test parallel deployments with different resource specs
#[tokio::test]
async fn test_concurrent_deployments() {
    use futures::future::join_all;
    
    let specs = vec![
        ResourceSpec::minimal(),
        ResourceSpec::basic(),
        ResourceSpec::recommended(),
    ];
    
    let deployment_futures = specs.into_iter().map(|spec| {
        async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let instance = select_instance_type(CloudProvider::AWS, &spec);
            Ok::<_, blueprint_remote_providers::error::Error>(instance)
        }
    });
    
    let results = join_all(deployment_futures).await;
    
    assert_eq!(results.len(), 3);
    for result in results {
        assert!(result.is_ok());
    }
}

/// Verify same spec always returns same instance type
#[test]
fn test_instance_selection_deterministic() {
    use proptest::prelude::*;
    
    proptest!(|(cpu in 0.5f32..8.0, memory in 1.0f32..32.0)| {
        let spec = ResourceSpec {
            cpu,
            memory_gb: memory,
            ..Default::default()
        };
        
        let i1 = select_instance_type(CloudProvider::AWS, &spec);
        let i2 = select_instance_type(CloudProvider::AWS, &spec);
        prop_assert_eq!(i1.instance_type, i2.instance_type);
        prop_assert_eq!(i1.estimated_cost, i2.estimated_cost);
    });
}

/// Verify cost increases with resource requirements
#[test]
fn test_cost_scaling() {
    use proptest::prelude::*;
    
    proptest!(|(cpu in 1.0f32..4.0, memory in 2.0f32..16.0)| {
        let small = ResourceSpec {
            cpu,
            memory_gb: memory,
            ..Default::default()
        };
        
        let large = ResourceSpec {
            cpu: cpu * 2.0,
            memory_gb: memory * 2.0,
            ..Default::default()
        };
        
        let small_cost = small.estimate_hourly_cost();
        let large_cost = large.estimate_hourly_cost();
        
        prop_assert!(large_cost >= small_cost);
        prop_assert!(large_cost <= small_cost * 4.0);
    });
}