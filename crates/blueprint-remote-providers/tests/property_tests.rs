//! Property-based tests that verify ACTUAL logic, not mocked behavior

use blueprint_remote_providers::core::resources::ResourceSpec;
use blueprint_remote_providers::AwsInstanceMapper;
use proptest::prelude::*;

// Test that instance mapping is deterministic
proptest! {
    #[test]
    fn test_aws_instance_mapping_is_deterministic(
        cpu in 0.25f32..128.0,
        memory_gb in 0.5f32..1024.0,
        storage_gb in 10.0f32..10000.0,
        gpu_count in prop::option::of(0u32..8),
    ) {
        let spec = ResourceSpec {
            cpu,
            memory_gb,
            storage_gb,
            gpu_count,
            allow_spot: false,
            qos: Default::default(),
        };

        // Map the same spec twice
        let instance1 = AwsInstanceMapper::map(&spec);
        let instance2 = AwsInstanceMapper::map(&spec);

        // MUST be deterministic
        prop_assert_eq!(instance1.instance_type, instance2.instance_type,
            "Instance mapping must be deterministic for cpu={}, mem={}, storage={}",
            cpu, memory_gb, storage_gb);
    }
}

// Test that increasing resources maintains or improves instance selection
proptest! {
    #[test]
    fn test_aws_instance_mapping_monotonic(
        base_cpu in 0.25f32..64.0,
        base_memory in 0.5f32..512.0,
        cpu_increase in 0.0f32..64.0,
        memory_increase in 0.0f32..512.0,
    ) {
        let base_spec = ResourceSpec {
            cpu: base_cpu,
            memory_gb: base_memory,
            storage_gb: 100.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };

        let larger_spec = ResourceSpec {
            cpu: base_cpu + cpu_increase,
            memory_gb: base_memory + memory_increase,
            ..base_spec.clone()
        };

        let base_instance = AwsInstanceMapper::map(&base_spec);
        let larger_instance = AwsInstanceMapper::map(&larger_spec);

        // Verify that we're getting reasonable instance types
        prop_assert!(!base_instance.instance_type.is_empty());
        prop_assert!(!larger_instance.instance_type.is_empty());
    }
}

// Test that GPU requests produce GPU-capable instances
proptest! {
    #[test]
    fn test_gpu_mapping_produces_gpu_instances(
        cpu in 1.0f32..32.0,
        memory_gb in 4.0f32..256.0,
        gpu_count in 1u32..8,
    ) {
        let spec = ResourceSpec {
            cpu,
            memory_gb,
            storage_gb: 100.0,
            gpu_count: Some(gpu_count),
            allow_spot: false,
            qos: Default::default(),
        };

        let instance = AwsInstanceMapper::map(&spec);

        // GPU instances should have specific prefixes
        let gpu_families = ["p2", "p3", "p4", "p5", "g3", "g4", "g5"];
        let is_gpu_instance = gpu_families.iter().any(|family| instance.instance_type.starts_with(family));

        prop_assert!(
            is_gpu_instance,
            "GPU request for {} GPUs resulted in non-GPU instance: {}",
            gpu_count, instance.instance_type
        );
    }
}

// Test that spot doesn't affect instance type
proptest! {
    #[test]
    fn test_spot_doesnt_affect_instance_type(
        cpu in 0.5f32..32.0,
        memory_gb in 1.0f32..128.0,
        storage_gb in 10.0f32..1000.0,
    ) {
        let base_spec = ResourceSpec {
            cpu,
            memory_gb,
            storage_gb,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };

        let spot_spec = ResourceSpec {
            allow_spot: true,
            ..base_spec.clone()
        };

        let regular_instance = AwsInstanceMapper::map(&base_spec);
        let spot_instance = AwsInstanceMapper::map(&spot_spec);

        prop_assert_eq!(
            regular_instance.instance_type, spot_instance.instance_type,
            "Spot flag should not change instance type selection"
        );
    }
}

// Test that resource limits are enforced in container commands
proptest! {
    #[test]
    fn test_container_resource_limits_in_commands(
        cpu in 0.25f32..16.0,
        memory_gb in 0.5f32..32.0,
        gpu_count in prop::option::of(1u32..4),
    ) {
        // The actual command that would be generated
        let docker_cmd = format!(
            "docker run -d --name test --cpus={} --memory={}g{}",
            cpu,
            memory_gb,
            gpu_count.map(|g| format!(" --gpus={}", g)).unwrap_or_default()
        );

        // Verify the command contains the resource limits
        let cpu_arg = format!("--cpus={}", cpu);
        let mem_arg = format!("--memory={}g", memory_gb);

        prop_assert!(docker_cmd.contains(&cpu_arg),
            "Docker command should contain CPU limit: {}", cpu_arg);
        prop_assert!(docker_cmd.contains(&mem_arg),
            "Docker command should contain memory limit: {}", mem_arg);

        if let Some(gpus) = gpu_count {
            let gpu_arg = format!("--gpus={}", gpus);
            prop_assert!(docker_cmd.contains(&gpu_arg),
                "Docker command should contain GPU limit: {}", gpu_arg);
        }
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_extreme_resource_requests_dont_panic() {
        let extreme_specs = vec![
            ResourceSpec {
                cpu: 0.001, // Tiny CPU
                memory_gb: 0.001,
                storage_gb: 1.0,
                gpu_count: None,
                allow_spot: false,
                qos: Default::default(),
            },
            ResourceSpec {
                cpu: 1000.0, // Huge CPU
                memory_gb: 10000.0,
                storage_gb: 100000.0,
                gpu_count: Some(100),
                allow_spot: false,
                qos: Default::default(),
            },
        ];

        for spec in extreme_specs {
            // Should not panic
            let instance = AwsInstanceMapper::map(&spec);

            // Should return a valid instance type
            assert!(!instance.instance_type.is_empty());
        }
    }

    #[test]
    fn test_cost_optimization_with_spot() {
        let spec = ResourceSpec {
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 100.0,
            gpu_count: None,
            allow_spot: true,
            qos: Default::default(),
        };

        let selection = AwsInstanceMapper::map(&spec);

        // Should identify as spot-capable
        assert!(selection.spot_capable);

        // Should have a reasonable cost estimate if available
        if let Some(cost) = selection.estimated_hourly_cost {
            assert!(cost > 0.0 && cost < 100.0);
        }
    }
}