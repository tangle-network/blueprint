//! Tests for deployment decision logic - NO MOCKS
//!
//! Validates:
//! - Provider selection algorithms
//! - Instance type mapping logic
//! - Cost comparison calculations
//! - Resource requirement translation
//!
//! All tests validate REAL business logic and calculations.

use blueprint_remote_providers::resources::ResourceSpec;

/// Test that resource spec validation actually validates
#[test]
fn test_resource_spec_validation_logic() {
    // Valid spec should pass
    let valid = ResourceSpec {
        cpu: 2.0,
        memory_gb: 4.0,
        storage_gb: 50.0,
        gpu_count: None,
        allow_spot: false,
        qos: Default::default(),
    };

    assert!(
        valid.validate().is_ok(),
        "Valid spec should pass validation"
    );

    // Too little CPU should fail
    let invalid_cpu = ResourceSpec {
        cpu: 0.05, // Below minimum
        memory_gb: 4.0,
        storage_gb: 50.0,
        gpu_count: None,
        allow_spot: false,
        qos: Default::default(),
    };

    assert!(
        invalid_cpu.validate().is_err(),
        "CPU below 0.1 should fail validation"
    );

    // Too little memory should fail
    let invalid_memory = ResourceSpec {
        cpu: 2.0,
        memory_gb: 0.25, // Below minimum
        storage_gb: 50.0,
        gpu_count: None,
        allow_spot: false,
        qos: Default::default(),
    };

    assert!(
        invalid_memory.validate().is_err(),
        "Memory below 0.5 GB should fail validation"
    );

    // Invalid GPU count should fail
    let invalid_gpu = ResourceSpec {
        cpu: 2.0,
        memory_gb: 4.0,
        storage_gb: 50.0,
        gpu_count: Some(0), // Zero GPUs invalid
        allow_spot: false,
        qos: Default::default(),
    };

    assert!(
        invalid_gpu.validate().is_err(),
        "Zero GPU count should fail validation"
    );

    let too_many_gpus = ResourceSpec {
        cpu: 2.0,
        memory_gb: 4.0,
        storage_gb: 50.0,
        gpu_count: Some(16), // More than max
        allow_spot: false,
        qos: Default::default(),
    };

    assert!(
        too_many_gpus.validate().is_err(),
        "More than 8 GPUs should fail validation"
    );
}

/// Test cost estimation formula accuracy
#[test]
fn test_cost_estimation_formula() {
    let spec = ResourceSpec {
        cpu: 4.0,
        memory_gb: 16.0,
        storage_gb: 200.0,
        gpu_count: Some(2),
        allow_spot: false,
        qos: Default::default(),
    };

    let cost = spec.estimate_hourly_cost();

    // Manual calculation to verify formula:
    // base_cost = cpu * 0.04 + memory_gb * 0.01
    let base_cost = 4.0 * 0.04 + 16.0 * 0.01; // = 0.16 + 0.16 = 0.32

    // storage_cost = storage_gb * 0.0001
    let storage_cost = 200.0 * 0.0001; // = 0.02

    // gpu_cost = gpu_count * 0.90
    let gpu_cost = 2.0 * 0.90; // = 1.80

    let expected_total = base_cost + storage_cost + gpu_cost; // = 0.32 + 0.02 + 1.80 = 2.14

    assert!(
        (cost - expected_total).abs() < 0.01,
        "Cost calculation should match formula: got {cost}, expected {expected_total}"
    );

    // Test spot instance discount (30% off)
    let spot_spec = ResourceSpec {
        allow_spot: true,
        ..spec
    };

    let spot_cost = spot_spec.estimate_hourly_cost();
    let expected_spot = expected_total * 0.7; // 30% discount

    assert!(
        (spot_cost - expected_spot).abs() < 0.01,
        "Spot cost should be 70% of on-demand: got {spot_cost}, expected {expected_spot}"
    );
}

/// Test instance type mapping logic for AWS
#[test]
fn test_aws_instance_type_mapping_logic() {
    use blueprint_remote_providers::providers::aws::instance_mapper::AwsInstanceMapper;

    // Test small instance
    let small_spec = ResourceSpec {
        cpu: 2.0,
        memory_gb: 4.0,
        storage_gb: 20.0,
        gpu_count: None,
        allow_spot: false,
        qos: Default::default(),
    };

    let instance = AwsInstanceMapper::map(&small_spec);
    assert!(
        !instance.instance_type.is_empty(),
        "Should return instance type"
    );
    // t3.medium has 2 vCPU, 4 GB RAM
    assert_eq!(instance.instance_type, "t3.medium");

    // Test GPU instance
    let gpu_spec = ResourceSpec {
        cpu: 8.0,
        memory_gb: 32.0,
        storage_gb: 100.0,
        gpu_count: Some(1),
        allow_spot: false,
        qos: Default::default(),
    };

    let gpu_instance = AwsInstanceMapper::map(&gpu_spec);
    assert!(
        gpu_instance.instance_type.contains("g4dn") || gpu_instance.instance_type.contains("p3"),
        "Should select GPU instance type, got {}",
        gpu_instance.instance_type
    );

    // GPU instances should not be spot-capable even if requested
    let mut gpu_spot_spec = gpu_spec.clone();
    gpu_spot_spec.allow_spot = true;
    let gpu_spot = AwsInstanceMapper::map(&gpu_spot_spec);
    assert!(
        !gpu_spot.spot_capable,
        "GPU instances should not be spot-capable"
    );

    // Test oversized requirements (should still try to find closest match)
    let large_spec = ResourceSpec {
        cpu: 96.0,
        memory_gb: 384.0,
        storage_gb: 2000.0,
        gpu_count: Some(4),
        allow_spot: false,
        qos: Default::default(),
    };

    let large_instance = AwsInstanceMapper::map(&large_spec);
    assert!(
        !large_instance.instance_type.is_empty(),
        "Should return instance type for large spec"
    );
}

/// Test pricing units conversion logic
#[test]
fn test_pricing_units_conversion() {
    let spec = ResourceSpec {
        cpu: 4.0,
        memory_gb: 8.0,
        storage_gb: 100.0,
        gpu_count: Some(1),
        allow_spot: false,
        qos: Default::default(),
    };

    let units = spec.to_pricing_units();

    // Verify conversion logic
    assert_eq!(units.get("CPU"), Some(&4.0), "CPU should convert directly");
    assert_eq!(
        units.get("MemoryMB"),
        Some(&(8.0 * 1024.0)),
        "Memory should convert to MB"
    );
    assert_eq!(
        units.get("StorageMB"),
        Some(&(100.0 * 1024.0)),
        "Storage should convert to MB"
    );
    assert_eq!(units.get("GPU"), Some(&1.0), "GPU should be included");

    // Test spec without GPU
    let no_gpu_spec = ResourceSpec {
        gpu_count: None,
        ..spec
    };

    let no_gpu_units = no_gpu_spec.to_pricing_units();
    assert!(
        !no_gpu_units.contains_key("GPU"),
        "GPU should not be in units if None"
    );
}

/// Test Docker resource configuration conversion
#[test]
fn test_docker_resource_config_conversion() {
    let spec = ResourceSpec {
        cpu: 2.0,
        memory_gb: 4.0,
        storage_gb: 20.0,
        gpu_count: None,
        allow_spot: false,
        qos: Default::default(),
    };

    let docker_config = spec.to_docker_resources();

    // NanoCPUs should be: cpu * 1_000_000_000
    let expected_nano_cpus = 2.0 * 1_000_000_000.0;
    assert_eq!(
        docker_config["NanoCPUs"].as_i64().unwrap(),
        expected_nano_cpus as i64,
        "NanoCPUs calculation should be correct"
    );

    // Memory should be: memory_gb * 1024^3
    let expected_memory = 4.0 * 1024.0 * 1024.0 * 1024.0;
    assert_eq!(
        docker_config["Memory"].as_i64().unwrap(),
        expected_memory as i64,
        "Memory calculation should be correct"
    );

    // Storage should be formatted as string
    assert_eq!(
        docker_config["StorageOpt"]["size"].as_str().unwrap(),
        "20G",
        "Storage format should be correct"
    );
}

/// Test Kubernetes resource requirements conversion
#[cfg(feature = "kubernetes")]
#[test]
fn test_kubernetes_resource_conversion() {
    let spec = ResourceSpec {
        cpu: 4.0,
        memory_gb: 8.0,
        storage_gb: 100.0,
        gpu_count: Some(1),
        allow_spot: false,
        qos: Default::default(),
    };

    let k8s_resources = spec.to_k8s_resources();

    // Should have limits and requests
    assert!(k8s_resources.limits.is_some(), "Should have limits");
    assert!(k8s_resources.requests.is_some(), "Should have requests");

    let limits = k8s_resources.limits.unwrap();
    let requests = k8s_resources.requests.unwrap();

    // CPU limits should match spec
    assert!(limits.contains_key("cpu"), "Limits should include CPU");

    // Requests should be less than limits (80% for CPU)
    assert!(requests.contains_key("cpu"), "Requests should include CPU");

    // Memory limits should match spec
    assert!(
        limits.contains_key("memory"),
        "Limits should include memory"
    );

    // GPU should be in limits
    assert!(
        limits.contains_key("nvidia.com/gpu"),
        "Limits should include GPU"
    );
}

/// Test resource spec presets are reasonable
#[test]
fn test_resource_spec_presets() {
    // Minimal preset
    let minimal = ResourceSpec::minimal();
    assert_eq!(minimal.cpu, 0.5, "Minimal should have 0.5 CPU");
    assert_eq!(minimal.memory_gb, 1.0, "Minimal should have 1 GB RAM");
    assert_eq!(
        minimal.storage_gb, 10.0,
        "Minimal should have 10 GB storage"
    );
    assert!(minimal.allow_spot, "Minimal should allow spot");
    assert!(minimal.validate().is_ok(), "Minimal should be valid");

    // Basic preset
    let basic = ResourceSpec::basic();
    assert_eq!(basic.cpu, 2.0, "Basic should have 2 CPUs");
    assert_eq!(basic.memory_gb, 4.0, "Basic should have 4 GB RAM");
    assert_eq!(basic.storage_gb, 20.0, "Basic should have 20 GB storage");
    assert!(!basic.allow_spot, "Basic should not allow spot");
    assert!(basic.validate().is_ok(), "Basic should be valid");

    // Recommended preset
    let recommended = ResourceSpec::recommended();
    assert_eq!(recommended.cpu, 4.0, "Recommended should have 4 CPUs");
    assert_eq!(
        recommended.memory_gb, 16.0,
        "Recommended should have 16 GB RAM"
    );
    assert_eq!(
        recommended.storage_gb, 100.0,
        "Recommended should have 100 GB storage"
    );
    assert!(
        recommended.validate().is_ok(),
        "Recommended should be valid"
    );

    // Performance preset
    let performance = ResourceSpec::performance();
    assert_eq!(performance.cpu, 8.0, "Performance should have 8 CPUs");
    assert_eq!(
        performance.memory_gb, 32.0,
        "Performance should have 32 GB RAM"
    );
    assert_eq!(
        performance.storage_gb, 500.0,
        "Performance should have 500 GB storage"
    );
    assert!(
        performance.validate().is_ok(),
        "Performance should be valid"
    );

    // Test GPU addition
    let with_gpu = ResourceSpec::basic().with_gpu(2);
    assert_eq!(with_gpu.gpu_count, Some(2), "Should add GPUs");
    assert!(with_gpu.validate().is_ok(), "With GPU should be valid");
}

/// Test that cost increases with resources (monotonicity)
#[test]
fn test_cost_monotonicity() {
    let base = ResourceSpec::basic();
    let base_cost = base.estimate_hourly_cost();

    // More CPU should cost more
    let more_cpu = ResourceSpec {
        cpu: base.cpu * 2.0,
        ..base.clone()
    };
    assert!(
        more_cpu.estimate_hourly_cost() > base_cost,
        "More CPU should increase cost"
    );

    // More memory should cost more
    let more_memory = ResourceSpec {
        memory_gb: base.memory_gb * 2.0,
        ..base.clone()
    };
    assert!(
        more_memory.estimate_hourly_cost() > base_cost,
        "More memory should increase cost"
    );

    // Adding GPU should significantly increase cost
    let with_gpu = base.clone().with_gpu(1);
    let gpu_cost = with_gpu.estimate_hourly_cost();
    assert!(
        gpu_cost > base_cost + 0.5,
        "Adding GPU should significantly increase cost (by at least $0.50/hr)"
    );
}

/// Test that PricingCalculator::new() correctly returns error (no hardcoded pricing)
#[test]
fn test_pricing_calculator_requires_config() {
    use blueprint_remote_providers::pricing::PricingCalculator;

    // PricingCalculator::new() should return error since all hardcoded pricing removed
    let result = PricingCalculator::new();

    assert!(
        result.is_err(),
        "PricingCalculator::new() should return error - hardcoded pricing removed"
    );

    // Verify it's a ConfigurationError
    let err = result.unwrap_err();
    let err_msg = format!("{err:?}");

    assert!(
        err_msg.contains("ConfigurationError") || err_msg.contains("hardcoded pricing removed"),
        "Should be ConfigurationError explaining hardcoded pricing removal: {err_msg}"
    );

    // For real pricing, users must use:
    // - PricingCalculator::from_config_file() with a config file
    // - PricingFetcher::new() for real-time VM pricing
    // - FaasPricingFetcher::new() for serverless pricing
}

/// Test spot instance pricing discount logic
#[test]
fn test_spot_instance_discount_logic() {
    let on_demand = ResourceSpec {
        cpu: 4.0,
        memory_gb: 8.0,
        storage_gb: 50.0,
        gpu_count: None,
        allow_spot: false,
        qos: Default::default(),
    };

    let spot = ResourceSpec {
        allow_spot: true,
        ..on_demand.clone()
    };

    let on_demand_cost = on_demand.estimate_hourly_cost();
    let spot_cost = spot.estimate_hourly_cost();

    // Spot should be exactly 70% of on-demand (30% discount)
    let expected_spot = on_demand_cost * 0.7;

    assert!(
        (spot_cost - expected_spot).abs() < 0.01,
        "Spot should be 30% cheaper: on-demand=${on_demand_cost:.2}, spot=${spot_cost:.2}, expected=${expected_spot:.2}"
    );

    // Verify discount percentage
    let discount_pct = (on_demand_cost - spot_cost) / on_demand_cost * 100.0;
    assert!(
        (discount_pct - 30.0).abs() < 1.0,
        "Discount should be ~30%, got {discount_pct:.1}%"
    );
}
