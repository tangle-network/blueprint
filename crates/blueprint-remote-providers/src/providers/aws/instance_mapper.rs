//! AWS instance type mapping using real pricing API

use crate::core::error::Result;
use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

/// Maps resource requirements to optimal AWS instance types using real pricing
pub struct AwsInstanceMapper;

impl AwsInstanceMapper {
    /// Map resource spec to optimal AWS instance type using real pricing data
    pub async fn map_async(spec: &ResourceSpec, region: &str) -> Result<InstanceSelection> {
        use crate::infra::traits::CloudProviderAdapter;
        super::AwsAdapter::new().await?.quote(spec, region).await
    }

    /// Legacy synchronous mapping - use map_async for real pricing
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        Self::fallback_mapping(spec)
    }

    fn fallback_mapping(spec: &ResourceSpec) -> InstanceSelection {
        let gpu_count = spec.gpu_count;
        if gpu_count.is_some() || spec.gpu_class.is_some() {
            return crate::providers::common::gpu_adapter::catalog_selection(spec, "aws");
        }
        let instance_type = match (spec.cpu, spec.memory_gb, gpu_count) {
            // Memory optimized - check this BEFORE general CPU/memory patterns
            (cpu, mem, _) if mem > cpu * 8.0 => "r6i.2xlarge", // Memory optimized

            // CPU/Memory optimized - use modern instance types
            (cpu, mem, _) if cpu <= 1.0 && mem <= 2.0 => "t3.small",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 4.0 => "t3.medium",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 8.0 => "t3.large",
            (cpu, mem, _) if cpu <= 4.0 && mem <= 16.0 => "m6i.xlarge",
            (cpu, mem, _) if cpu <= 8.0 && mem <= 32.0 => "m6i.2xlarge",
            (cpu, mem, _) if cpu <= 16.0 && mem <= 64.0 => "m6i.4xlarge",
            (cpu, _, _) if cpu > 48.0 => "c6i.12xlarge", // Compute optimized
            _ => "m6i.large",
        };

        // Spot capability logic:
        // - No spot for GPU instances (any instance with GPU)
        // - No spot for small instances (t3.small, t3.medium for reliability)
        // - Only allow spot for larger instances when explicitly requested
        let spot_capable = spec.allow_spot
            && gpu_count.is_none() // No GPU instances
            && !matches!(instance_type, "t3.small" | "t3.medium"); // No small instances

        InstanceSelection {
            gpu_class: spec.gpu_class.clone(),
            instance_type: instance_type.to_string(),
            spot_capable,
            estimated_hourly_cost: None,
        }
    }
}
