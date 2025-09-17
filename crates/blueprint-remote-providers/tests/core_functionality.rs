//! Tests for instance type selection and resource mapping

use blueprint_remote_providers::{
    resources::ResourceSpec,
    remote::CloudProvider,
    provisioning::InstanceSelection,
};

/// Verify instance type selection for AWS, GCP, Azure, and DigitalOcean
#[test]
fn test_instance_type_selection() {
    use blueprint_remote_providers::providers::{
        aws::provisioner::AwsProvisioner,
        gcp::GcpProvisioner,
        azure::AzureProvisioner,
        digitalocean::DigitalOceanProvisioner,
    };
    
    let specs = vec![
        ("minimal", ResourceSpec::minimal()),
        ("basic", ResourceSpec::basic()),
        ("recommended", ResourceSpec::recommended()),
    ];
    
    for (name, spec) in specs {
        // AWS
        let aws_instance = AwsProvisioner::map_instance(&spec);
        assert!(!aws_instance.instance_type.is_empty(), "AWS failed for {}", name);
        assert!(aws_instance.estimated_cost > 0.0);
        
        // GCP
        let gcp_instance = GcpProvisioner::map_instance(&spec);
        assert!(!gcp_instance.instance_type.is_empty(), "GCP failed for {}", name);
        
        // Azure
        let azure_instance = AzureProvisioner::map_instance(&spec);
        assert!(!azure_instance.instance_type.is_empty(), "Azure failed for {}", name);
        
        // DigitalOcean
        let do_instance = DigitalOceanProvisioner::map_instance(&spec);
        assert!(!do_instance.instance_type.is_empty(), "DO failed for {}", name);
    }
}

/// Verify ResourceSpec validation rejects invalid configurations
#[test]
fn test_resource_validation() {
    // Valid cases
    assert!(ResourceSpec::minimal().validate().is_ok());
    assert!(ResourceSpec::basic().validate().is_ok());
    
    // Invalid: zero CPU
    let invalid = ResourceSpec {
        cpu: 0.0,
        memory_gb: 1.0,
        ..Default::default()
    };
    assert!(invalid.validate().is_err());
    
    // Invalid: excessive resources
    let excessive = ResourceSpec {
        cpu: 999.0,
        memory_gb: 9999.0,
        ..Default::default()
    };
    assert!(excessive.validate().is_err());
}

/// Verify Kubernetes resource format conversion
#[test]
fn test_k8s_resources() {
    let spec = ResourceSpec {
        cpu: 1.5,
        memory_gb: 2.5,
        ..Default::default()
    };
    
    let (cpu, mem) = spec.to_k8s_resources();
    
    // CPU can be millicores or decimal
    assert!(cpu == "1500m" || cpu == "1.5");
    
    // Memory should be in Gi or Mi
    assert!(mem.ends_with("Gi") || mem.ends_with("Mi"));
}

/// Verify Docker resource format conversion
#[test]
fn test_docker_resources() {
    let spec = ResourceSpec::basic();
    let (cpu, mem) = spec.to_docker_resources();
    
    // CPU should be decimal string
    assert!(cpu.parse::<f32>().is_ok());
    
    // Memory should end with 'm'
    assert!(mem.ends_with('m'));
    let mem_value = mem.trim_end_matches('m').parse::<u32>();
    assert!(mem_value.is_ok());
}

/// Verify service type and tunnel configuration per provider
#[test]
fn test_provider_properties() {
    // Service types
    assert_eq!(CloudProvider::AWS.to_service_type(), "LoadBalancer");
    assert_eq!(CloudProvider::GCP.to_service_type(), "ClusterIP");
    
    // Tunnel requirements
    assert!(!CloudProvider::AWS.requires_tunnel());
    assert!(CloudProvider::Generic.requires_tunnel());
}

/// Verify cost estimation scales with resource requirements
#[test]
fn test_cost_estimation() {
    let minimal = ResourceSpec::minimal();
    let basic = ResourceSpec::basic();
    let performance = ResourceSpec::performance();
    
    let min_cost = minimal.estimate_hourly_cost();
    let basic_cost = basic.estimate_hourly_cost();
    let perf_cost = performance.estimate_hourly_cost();
    
    // Costs should increase with resources
    assert!(min_cost > 0.0);
    assert!(basic_cost > min_cost);
    assert!(perf_cost > basic_cost);
    
    // Sanity checks
    assert!(min_cost < 1.0);  // Minimal should be cheap
    assert!(perf_cost < 10.0); // Even performance shouldn't be crazy
}

/// Verify GPU instances are selected when GPU count > 0
#[test]
fn test_gpu_instance_selection() {
    use blueprint_remote_providers::providers::aws::provisioner::AwsProvisioner;
    
    let gpu_spec = ResourceSpec {
        cpu: 4.0,
        memory_gb: 16.0,
        storage_gb: 100.0,
        gpu_count: Some(1),
        allow_spot: false,
    };
    
    let instance = AwsProvisioner::map_instance(&gpu_spec);
    
    // Should select a GPU instance type
    assert!(
        instance.instance_type.starts_with("p") || 
        instance.instance_type.starts_with("g"),
        "Should select GPU instance, got: {}", 
        instance.instance_type
    );
}

/// Test Vantage.sh pricing API for AWS instances
#[tokio::test]
async fn test_pricing_api_integration() {
    use blueprint_remote_providers::pricing::fetcher::PricingFetcher;
    
    let mut fetcher = PricingFetcher::new();
    
    // Try to fetch real pricing (may fail without network)
    let result = fetcher.find_best_instance(
        CloudProvider::AWS,
        "us-east-1",
        1.0,
        2.0,
        0.10,
    ).await;
    
    if let Ok(instance) = result {
        assert!(instance.vcpus >= 1.0);
        assert!(instance.memory_gb >= 2.0);
        assert!(instance.hourly_price <= 0.10);
        println!("Found instance: {} at ${}/hr", instance.name, instance.hourly_price);
    } else {
        println!("Pricing API unavailable (expected in CI)");
    }
}

/// Test deployment registration and TTL expiration
#[tokio::test]
async fn test_deployment_tracking() {
    use blueprint_remote_providers::deployment::tracker::{DeploymentTracker, DeploymentRecord};
    use tempfile::TempDir;
    use std::time::Duration;
    
    let temp_dir = TempDir::new().unwrap();
    let tracker = DeploymentTracker::new(temp_dir.path()).await.unwrap();
    
    let record = DeploymentRecord {
        provider: CloudProvider::AWS,
        region: "us-east-1".to_string(),
        instance_id: Some("i-123456".to_string()),
        deployment_type: "docker".to_string(),
        resource_spec: ResourceSpec::minimal(),
        created_at: chrono::Utc::now(),
        ttl: Some(Duration::from_secs(1)),
    };
    
    // Register deployment
    tracker.register_deployment("test-1".to_string(), record.clone()).await.unwrap();
    
    // Should be listed
    let deployments = tracker.list_deployments().await;
    assert_eq!(deployments.len(), 1);
    
    // Wait for TTL
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // Check expired
    let expired = tracker.check_expired_deployments().await;
    assert_eq!(expired.len(), 1);
}