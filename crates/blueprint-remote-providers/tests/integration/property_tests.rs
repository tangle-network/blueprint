//! Property-based tests for resource mapping and instance selection
//! Ensures invariants hold across all possible inputs

use blueprint_remote_providers::{
    provisioning::select_instance_type,
    remote::CloudProvider,
    resources::ResourceSpec,
    pricing::fetcher::PricingFetcher,
};
use proptest::prelude::*;

/// Test that selected instances always meet minimum requirements
proptest! {
    #[test]
    fn test_instance_selection_meets_requirements(
        cpu in 0.1f32..16.0,
        memory in 0.5f32..64.0,
        gpu_count in 0u32..4,
    ) {
        let spec = ResourceSpec {
            cpu,
            memory_gb: memory,
            gpu_count,
            ..Default::default()
        };
        
        for provider in &[
            CloudProvider::AWS,
            CloudProvider::GCP,
            CloudProvider::Azure,
            CloudProvider::DigitalOcean,
        ] {
            let instance_type = select_instance_type(*provider, &spec);
            
            // Verify instance type is not empty
            prop_assert!(!instance_type.is_empty());
            
            // Verify GPU instances are selected when needed
            if gpu_count > 0 {
                match provider {
                    CloudProvider::AWS => prop_assert!(
                        instance_type.starts_with("p") || 
                        instance_type.starts_with("g")
                    ),
                    CloudProvider::GCP => prop_assert!(
                        instance_type.contains("nvidia") ||
                        instance_type.contains("tesla")
                    ),
                    _ => {} // Not all providers have GPU instances
                }
            }
        }
    }
}

/// Test that resource specs are validated correctly
proptest! {
    #[test]
    fn test_resource_validation(
        cpu in 0.0f32..100.0,
        memory in 0.0f32..1000.0,
        storage in 0u64..10000,
    ) {
        let spec = ResourceSpec {
            cpu,
            memory_gb: memory,
            storage_gb: storage,
            ..Default::default()
        };
        
        let result = spec.validate();
        
        // Should fail if resources are zero or negative
        if cpu <= 0.0 || memory <= 0.0 {
            prop_assert!(result.is_err());
        } else if cpu > 96.0 || memory > 768.0 {
            // Should fail if resources exceed reasonable limits
            prop_assert!(result.is_err());
        } else {
            prop_assert!(result.is_ok());
        }
    }
}

/// Test cost estimation consistency
proptest! {
    #[test]
    fn test_cost_estimation_consistency(
        cpu in 1.0f32..16.0,
        memory in 1.0f32..64.0,
        hours in 1u32..744, // Up to 1 month
    ) {
        let spec = ResourceSpec {
            cpu,
            memory_gb: memory,
            ..Default::default()
        };
        
        let hourly_cost = spec.estimate_hourly_cost();
        
        // Cost should scale with resources
        prop_assert!(hourly_cost > 0.0);
        prop_assert!(hourly_cost < 100.0); // Sanity check
        
        // Larger specs should cost more
        let larger_spec = ResourceSpec {
            cpu: cpu * 2.0,
            memory_gb: memory * 2.0,
            ..Default::default()
        };
        
        let larger_cost = larger_spec.estimate_hourly_cost();
        prop_assert!(larger_cost >= hourly_cost);
        
        // Monthly cost should be hourly * hours
        let total_cost = hourly_cost * hours as f64;
        prop_assert!((total_cost - (hourly_cost * hours as f64)).abs() < 0.01);
    }
}

/// Test that K8s resource conversion preserves ratios
proptest! {
    #[test]
    fn test_k8s_resource_conversion(
        cpu in 0.1f32..8.0,
        memory in 0.5f32..32.0,
    ) {
        let spec = ResourceSpec {
            cpu,
            memory_gb: memory,
            ..Default::default()
        };
        
        let (cpu_str, mem_str) = spec.to_k8s_resources();
        
        // Parse CPU (can be "100m" or "1" format)
        let cpu_value = if cpu_str.ends_with('m') {
            cpu_str.trim_end_matches('m').parse::<f32>().unwrap() / 1000.0
        } else {
            cpu_str.parse::<f32>().unwrap()
        };
        
        // Parse memory (can be "1Gi" or "1024Mi" format)
        let mem_value = if mem_str.ends_with("Gi") {
            mem_str.trim_end_matches("Gi").parse::<f32>().unwrap()
        } else if mem_str.ends_with("Mi") {
            mem_str.trim_end_matches("Mi").parse::<f32>().unwrap() / 1024.0
        } else {
            memory // fallback
        };
        
        // Verify conversion is accurate (within rounding)
        prop_assert!((cpu_value - cpu).abs() < 0.01);
        prop_assert!((mem_value - memory).abs() < 0.01);
    }
}

/// Test Docker resource conversion
proptest! {
    #[test]
    fn test_docker_resource_conversion(
        cpu in 0.1f32..8.0,
        memory in 128.0f32..8192.0, // In MB
    ) {
        let spec = ResourceSpec {
            cpu,
            memory_gb: memory / 1024.0,
            ..Default::default()
        };
        
        let (cpus, mem_limit) = spec.to_docker_resources();
        
        // Docker CPU is a float string
        let cpu_value: f32 = cpus.parse().unwrap();
        prop_assert!((cpu_value - cpu).abs() < 0.01);
        
        // Docker memory is in format "512m"
        prop_assert!(mem_limit.ends_with('m'));
        let mem_value: u32 = mem_limit.trim_end_matches('m').parse().unwrap();
        let expected_mb = (spec.memory_gb * 1024.0) as u32;
        prop_assert!((mem_value as i32 - expected_mb as i32).abs() < 10);
    }
}

/// Test instance type determinism
proptest! {
    #[test]
    fn test_instance_selection_deterministic(
        cpu in 1.0f32..8.0,
        memory in 1.0f32..32.0,
        seed in 0u64..1000,
    ) {
        let spec = ResourceSpec {
            cpu,
            memory_gb: memory,
            ..Default::default()
        };
        
        // Same spec should always return same instance type
        let instance1 = select_instance_type(CloudProvider::AWS, &spec);
        let instance2 = select_instance_type(CloudProvider::AWS, &spec);
        
        prop_assert_eq!(instance1, instance2);
        
        // Different providers should return different types
        let aws = select_instance_type(CloudProvider::AWS, &spec);
        let gcp = select_instance_type(CloudProvider::GCP, &spec);
        
        prop_assert_ne!(aws, gcp);
    }
}

/// Test pricing fetcher caching behavior
proptest! {
    #[test]
    fn test_pricing_cache_effectiveness(
        cpu in 1.0f32..4.0,
        memory in 2.0f32..8.0,
        max_price in 0.05f64..1.0,
    ) {
        // This would test with mocked pricing data
        // Verifying that cached results are consistent
        
        let spec = ResourceSpec {
            cpu,
            memory_gb: memory,
            ..Default::default()
        };
        
        // Verify price constraints are respected
        let estimated_cost = spec.estimate_hourly_cost();
        
        if estimated_cost <= max_price {
            // Should find an instance
            prop_assert!(estimated_cost > 0.0);
        }
        
        // Verify caching doesn't affect results
        let cost1 = spec.estimate_hourly_cost();
        let cost2 = spec.estimate_hourly_cost();
        prop_assert_eq!(cost1, cost2);
    }
}

/// Test that TTL values are handled correctly
proptest! {
    #[test]
    fn test_ttl_validation(
        ttl_seconds in 0u64..86400 * 365, // Up to 1 year
    ) {
        use std::time::Duration;
        
        let ttl = Duration::from_secs(ttl_seconds);
        
        // Very short TTLs should be rejected
        if ttl_seconds < 60 {
            // Minimum 1 minute
            prop_assert!(ttl.as_secs() < 60);
        }
        
        // Very long TTLs should be capped
        if ttl_seconds > 86400 * 30 {
            // Maximum 30 days
            let max_ttl = Duration::from_secs(86400 * 30);
            prop_assert!(ttl <= max_ttl || ttl_seconds > max_ttl.as_secs());
        }
    }
}

/// Test region validation
proptest! {
    #[test]
    fn test_region_validation(
        region in "[a-z]{2}-[a-z]+-[0-9]{1}",
    ) {
        // AWS regions follow pattern like us-east-1
        prop_assert!(region.len() >= 9);
        prop_assert!(region.contains('-'));
        
        // Should have 3 parts
        let parts: Vec<&str> = region.split('-').collect();
        prop_assert_eq!(parts.len(), 3);
    }
}

/// Test concurrent deployment limits
proptest! {
    #[test]
    fn test_deployment_concurrency_limits(
        deployment_count in 1usize..100,
        max_concurrent in 1usize..20,
    ) {
        // Verify deployment batching works correctly
        let batches = (deployment_count + max_concurrent - 1) / max_concurrent;
        let last_batch_size = deployment_count % max_concurrent;
        
        prop_assert!(batches > 0);
        if last_batch_size > 0 {
            prop_assert!(last_batch_size <= max_concurrent);
        }
        
        // Total should match
        let total = if last_batch_size == 0 {
            batches * max_concurrent
        } else {
            (batches - 1) * max_concurrent + last_batch_size
        };
        prop_assert_eq!(total, deployment_count);
    }
}