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
