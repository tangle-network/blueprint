//! AWS instance type mapping using real pricing API

use crate::core::error::Result;
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::pricing::fetcher::PricingFetcher;
use crate::providers::common::InstanceSelection;

/// Maps resource requirements to optimal AWS instance types using real pricing
pub struct AwsInstanceMapper;

impl AwsInstanceMapper {
    /// Map resource spec to optimal AWS instance type using real pricing data
    pub async fn map_async(spec: &ResourceSpec, region: &str) -> Result<InstanceSelection> {
        let mut fetcher = PricingFetcher::new();

        // Set reasonable max price based on requirements
        let max_price = if spec.gpu_count.is_some() {
            50.0 // Higher for GPU instances
        } else {
            5.0 // Reasonable for CPU instances
        };

        match fetcher
            .find_best_instance(
                CloudProvider::AWS,
                region,
                spec.cpu,
                spec.memory_gb,
                max_price,
            )
            .await
        {
            Ok(instance) => Ok(InstanceSelection {
                instance_type: instance.name,
                spot_capable: spec.allow_spot,
                estimated_hourly_cost: Some(instance.hourly_price),
            }),
            Err(_) => {
                // Fallback to basic mapping
                Ok(Self::fallback_mapping(spec))
            }
        }
    }

    /// Legacy synchronous mapping - use map_async for real pricing
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        Self::fallback_mapping(spec)
    }

    fn fallback_mapping(spec: &ResourceSpec) -> InstanceSelection {
        let gpu_count = spec.gpu_count;
        let instance_type = match (spec.cpu, spec.memory_gb, gpu_count) {
            // GPU instances
            (_, _, Some(gpu_count)) if gpu_count >= 8 => "p4d.24xlarge",
            (_, _, Some(gpu_count)) if gpu_count >= 4 => "p3.8xlarge",
            (_, _, Some(gpu_count)) if gpu_count >= 1 => "g4dn.xlarge",

            // CPU/Memory optimized - use modern instance types
            (cpu, mem, _) if cpu <= 1.0 && mem <= 2.0 => "t3.small",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 4.0 => "t3.medium",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 8.0 => "t3.large",
            (cpu, mem, _) if cpu <= 4.0 && mem <= 16.0 => "m6i.xlarge",
            (cpu, mem, _) if cpu <= 8.0 && mem <= 32.0 => "m6i.2xlarge",
            (cpu, mem, _) if cpu <= 16.0 && mem <= 64.0 => "m6i.4xlarge",
            (cpu, mem, _) if mem > cpu * 8.0 => "r6i.2xlarge", // Memory optimized
            (cpu, _, _) if cpu > 48.0 => "c6i.12xlarge",       // Compute optimized
            _ => "m6i.large",
        };

        InstanceSelection {
            instance_type: instance_type.to_string(),
            spot_capable: spec.allow_spot && !instance_type.starts_with('p'),
            estimated_hourly_cost: None, // Use map_async for real pricing
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_instance_selection() {
        let spec = ResourceSpec::minimal();
        let result = AwsInstanceMapper::map(&spec);

        assert_eq!(result.instance_type, "t3.small");
        assert!(!result.spot_capable); // Minimal shouldn't use spot
        assert!(result.estimated_hourly_cost.is_none()); // Sync mapping has no price
    }

    #[test]
    fn test_basic_instance_selection() {
        let spec = ResourceSpec::basic();
        let result = AwsInstanceMapper::map(&spec);

        assert_eq!(result.instance_type, "t3.medium");
        assert!(!result.spot_capable);
    }

    #[test]
    fn test_gpu_instance_selection() {
        let test_cases = vec![
            (1, "g4dn.xlarge"),
            (4, "p3.8xlarge"),
            (8, "p4d.24xlarge"),
        ];

        for (gpu_count, expected) in test_cases {
            let mut spec = ResourceSpec::performance();
            spec.gpu_count = Some(gpu_count);

            let result = AwsInstanceMapper::map(&spec);
            assert_eq!(
                result.instance_type, expected,
                "GPU count {} should map to {}",
                gpu_count, expected
            );
            assert!(!result.spot_capable); // GPU instances typically not spot
        }
    }

    #[test]
    fn test_memory_optimized_selection() {
        let mut spec = ResourceSpec::recommended();
        spec.cpu = 4.0;
        spec.memory_gb = 64.0; // High memory-to-CPU ratio

        let result = AwsInstanceMapper::map(&spec);
        assert!(
            result.instance_type.starts_with("r6i"),
            "High memory ratio should select r6i instance, got {}",
            result.instance_type
        );
    }

    #[test]
    fn test_compute_optimized_selection() {
        let mut spec = ResourceSpec::performance();
        spec.cpu = 64.0; // High CPU count
        spec.memory_gb = 128.0;

        let result = AwsInstanceMapper::map(&spec);
        assert!(
            result.instance_type.starts_with("c6i"),
            "High CPU count should select c6i instance, got {}",
            result.instance_type
        );
    }

    #[test]
    fn test_spot_capability() {
        let mut spec = ResourceSpec::recommended();

        // Test with spot disabled
        spec.allow_spot = false;
        let result = AwsInstanceMapper::map(&spec);
        assert!(!result.spot_capable);

        // Test with spot enabled
        spec.allow_spot = true;
        let result = AwsInstanceMapper::map(&spec);
        assert!(result.spot_capable);

        // Test GPU instances never allow spot
        spec.gpu_count = Some(1);
        let result = AwsInstanceMapper::map(&spec);
        assert!(!result.spot_capable, "GPU instances should not be spot-capable");
    }

    #[tokio::test]
    async fn test_async_mapping_fallback() {
        // Test that async mapping falls back gracefully without API
        let spec = ResourceSpec::basic();
        let result = AwsInstanceMapper::map_async(&spec, "us-west-2").await;

        assert!(result.is_ok());
        let selection = result.unwrap();
        assert!(!selection.instance_type.is_empty());
    }
}
