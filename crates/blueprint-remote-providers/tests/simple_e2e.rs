//! Simple E2E test that actually compiles and runs

use blueprint_remote_providers::{CloudProvider, ResourceSpec, PricingService};

#[test]
fn test_provider_selection() {
    // Test that provider selection works for different workloads
    let pricing = PricingService::new();
    
    // GPU workload
    let gpu_spec = ResourceSpec {
        cpu: 4.0,
        memory_gb: 16.0,
        storage_gb: 100.0,
        gpu_count: Some(1),
        allow_spot: false,
    };
    
    let (provider, report) = pricing.find_cheapest_provider(&gpu_spec, 24.0);
    println!("GPU workload selected provider: {:?}", provider);
    // For now, just verify we get a provider with cost
    assert!(report.total_cost > 0.0);
    
    // Budget workload
    let budget_spec = ResourceSpec {
        cpu: 0.5,
        memory_gb: 1.0,
        storage_gb: 10.0,
        gpu_count: None,
        allow_spot: true,
    };
    
    let (provider, report) = pricing.find_cheapest_provider(&budget_spec, 730.0);
    assert!(matches!(provider, CloudProvider::Vultr | CloudProvider::DigitalOcean));
    assert!(report.total_cost < 100.0); // Should be under $100/month
}

#[test]
fn test_resource_validation() {
    // Valid specs
    assert!(ResourceSpec::minimal().validate().is_ok());
    assert!(ResourceSpec::basic().validate().is_ok());
    assert!(ResourceSpec::recommended().validate().is_ok());
    
    // Invalid specs
    let invalid = ResourceSpec {
        cpu: 0.05, // Too low
        memory_gb: 1.0,
        storage_gb: 10.0,
        gpu_count: None,
        allow_spot: false,
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_cost_calculations() {
    let spec = ResourceSpec::basic();
    let pricing = PricingService::new();
    
    // Compare costs across providers
    let reports = pricing.compare_providers(&spec, 730.0);
    
    // Should have different costs for different providers
    let aws_cost = reports.iter()
        .find(|r| r.provider == CloudProvider::AWS)
        .map(|r| r.total_cost)
        .unwrap_or(0.0);
        
    let vultr_cost = reports.iter()
        .find(|r| r.provider == CloudProvider::Vultr)
        .map(|r| r.total_cost)
        .unwrap_or(0.0);
    
    // AWS should be more expensive than Vultr for basic workloads
    assert!(aws_cost > vultr_cost);
}

#[cfg(feature = "kubernetes")]
#[test]
fn test_k8s_resource_conversion() {
    let spec = ResourceSpec::recommended();
    let k8s_resources = spec.to_k8s_resources();
    
    assert!(k8s_resources.limits.is_some());
    assert!(k8s_resources.requests.is_some());
    
    let limits = k8s_resources.limits.unwrap();
    assert!(limits.contains_key("cpu"));
    assert!(limits.contains_key("memory"));
}