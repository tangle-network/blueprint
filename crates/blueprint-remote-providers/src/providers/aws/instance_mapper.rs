//! AWS instance type mapping

use crate::providers::common::InstanceSelection;
use crate::resources::ResourceSpec;

/// Maps resource requirements to AWS instance types
pub struct AwsInstanceMapper;

impl AwsInstanceMapper {
    /// Map resource spec to AWS instance type
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        // TODO: Fetch instance types dynamically from AWS API
        let gpu_count = spec.gpu_count;
        let instance_type = match (spec.cpu, spec.memory_gb, gpu_count) {
            // GPU instances
            (_, _, Some(gpu_count)) if gpu_count >= 8 => "p4d.24xlarge",
            (_, _, Some(gpu_count)) if gpu_count >= 4 => "p3.8xlarge",
            (_, _, Some(gpu_count)) if gpu_count >= 1 => "g4dn.xlarge",

            // CPU/Memory optimized
            (cpu, mem, _) if cpu <= 0.5 && mem <= 1.0 => "t3.micro",
            (cpu, mem, _) if cpu <= 1.0 && mem <= 2.0 => "t3.small",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 4.0 => "t3.medium",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 8.0 => "t3.large",
            (cpu, mem, _) if cpu <= 4.0 && mem <= 16.0 => "m6i.xlarge",
            (cpu, mem, _) if cpu <= 8.0 && mem <= 32.0 => "m6i.2xlarge",
            (cpu, mem, _) if cpu <= 16.0 && mem <= 64.0 => "m6i.4xlarge",
            (cpu, mem, _) if cpu <= 32.0 && mem <= 128.0 => "m6i.8xlarge",
            (cpu, mem, _) if mem > cpu * 8.0 => "r6i.2xlarge", // Memory optimized
            (cpu, _, _) if cpu > 48.0 => "c6i.12xlarge",       // Compute optimized
            _ => "m6i.large",                                  // Default
        };

        InstanceSelection {
            instance_type: instance_type.to_string(),
            spot_capable: spec.allow_spot && !instance_type.starts_with('p'), // No spot for GPU
            estimated_hourly_cost: Self::estimate_cost(instance_type),
        }
    }

    fn estimate_cost(instance_type: &str) -> Option<f64> {
        // Rough AWS pricing estimates
        Some(match instance_type {
            "t3.micro" => 0.0104,
            "t3.small" => 0.0208,
            "t3.medium" => 0.0416,
            "t3.large" => 0.0832,
            "m6i.large" => 0.096,
            "m6i.xlarge" => 0.192,
            "m6i.2xlarge" => 0.384,
            "m6i.4xlarge" => 0.768,
            "m6i.8xlarge" => 1.536,
            "r6i.2xlarge" => 0.504,
            "c6i.12xlarge" => 2.04,
            "g4dn.xlarge" => 0.526,
            "p3.8xlarge" => 12.24,
            "p4d.24xlarge" => 32.77,
            _ => 0.1,
        })
    }
}