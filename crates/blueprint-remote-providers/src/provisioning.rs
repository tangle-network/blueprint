use crate::remote::CloudProvider;
use crate::resources::ResourceSpec;
use serde::{Deserialize, Serialize};

/// Maps resource requirements to cloud instance types
pub struct InstanceTypeMapper;

impl InstanceTypeMapper {
    /// Map resource spec to specific instance type
    pub fn map_to_instance_type(
        spec: &ResourceSpec,
        provider: &CloudProvider,
    ) -> InstanceSelection {
        match provider {
            CloudProvider::AWS => Self::map_aws_instance(spec),
            CloudProvider::GCP => Self::map_gcp_instance(spec),
            CloudProvider::Azure => Self::map_azure_instance(spec),
            CloudProvider::DigitalOcean => Self::map_do_instance(spec),
            CloudProvider::Vultr => Self::map_vultr_instance(spec),
            _ => Self::map_generic_instance(spec),
        }
    }

    fn map_aws_instance(spec: &ResourceSpec) -> InstanceSelection {
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
            estimated_hourly_cost: Self::estimate_aws_cost(instance_type),
        }
    }

    fn map_gcp_instance(spec: &ResourceSpec) -> InstanceSelection {
        let gpu_count = spec.gpu_count;
        let instance_type = match (spec.cpu, spec.memory_gb, gpu_count) {
            // GPU instances
            (_, _, Some(gpu_count)) if gpu_count >= 1 => "n1-standard-4-nvidia-t4",

            // Standard instances
            (cpu, mem, _) if cpu <= 0.5 && mem <= 2.0 => "e2-micro",
            (cpu, mem, _) if cpu <= 1.0 && mem <= 4.0 => "e2-small",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 8.0 => "e2-medium",
            (cpu, mem, _) if cpu <= 4.0 && mem <= 16.0 => "n2-standard-4",
            (cpu, mem, _) if cpu <= 8.0 && mem <= 32.0 => "n2-standard-8",
            (cpu, mem, _) if cpu <= 16.0 && mem <= 64.0 => "n2-standard-16",
            (cpu, mem, _) if mem > cpu * 8.0 => "n2-highmem-4", // Memory optimized
            _ => "e2-standard-2",
        };

        InstanceSelection {
            instance_type: instance_type.to_string(),
            spot_capable: spec.allow_spot,
            estimated_hourly_cost: Self::estimate_gcp_cost(instance_type),
        }
    }

    fn map_azure_instance(spec: &ResourceSpec) -> InstanceSelection {
        let gpu_count = spec.gpu_count;
        let instance_type = match (spec.cpu, spec.memory_gb, gpu_count) {
            // GPU instances
            (_, _, Some(_)) => "Standard_NC6s_v3",

            // Standard instances
            (cpu, mem, _) if cpu <= 1.0 && mem <= 2.0 => "Standard_B1s",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 4.0 => "Standard_B2s",
            (cpu, mem, _) if cpu <= 4.0 && mem <= 16.0 => "Standard_D4s_v5",
            (cpu, mem, _) if cpu <= 8.0 && mem <= 32.0 => "Standard_D8s_v5",
            _ => "Standard_D2s_v5",
        };

        InstanceSelection {
            instance_type: instance_type.to_string(),
            spot_capable: spec.allow_spot,
            estimated_hourly_cost: Self::estimate_azure_cost(instance_type),
        }
    }

    fn map_do_instance(spec: &ResourceSpec) -> InstanceSelection {
        // DigitalOcean droplet types
        let instance_type = match (spec.cpu, spec.memory_gb) {
            (cpu, mem) if cpu <= 1.0 && mem <= 1.0 => "s-1vcpu-1gb",
            (cpu, mem) if cpu <= 1.0 && mem <= 2.0 => "s-1vcpu-2gb",
            (cpu, mem) if cpu <= 2.0 && mem <= 4.0 => "s-2vcpu-4gb",
            (cpu, mem) if cpu <= 4.0 && mem <= 8.0 => "s-4vcpu-8gb",
            (cpu, mem) if cpu <= 8.0 && mem <= 16.0 => "s-8vcpu-16gb",
            _ => "s-2vcpu-2gb",
        };

        InstanceSelection {
            instance_type: instance_type.to_string(),
            spot_capable: false, // DigitalOcean doesn't have spot instances
            estimated_hourly_cost: Self::estimate_do_cost(instance_type),
        }
    }

    fn map_vultr_instance(spec: &ResourceSpec) -> InstanceSelection {
        // Vultr instance types
        let instance_type = match (spec.cpu, spec.memory_gb) {
            (cpu, mem) if cpu <= 1.0 && mem <= 1.0 => "vc2-1c-1gb",
            (cpu, mem) if cpu <= 2.0 && mem <= 4.0 => "vc2-2c-4gb",
            (cpu, mem) if cpu <= 4.0 && mem <= 8.0 => "vc2-4c-8gb",
            (cpu, mem) if cpu <= 8.0 && mem <= 16.0 => "vc2-8c-16gb",
            _ => "vc2-2c-2gb",
        };

        InstanceSelection {
            instance_type: instance_type.to_string(),
            spot_capable: false,
            estimated_hourly_cost: Self::estimate_vultr_cost(instance_type),
        }
    }

    fn map_generic_instance(spec: &ResourceSpec) -> InstanceSelection {
        InstanceSelection {
            instance_type: format!("{}cpu-{}gb", spec.cpu, spec.memory_gb),
            spot_capable: false,
            estimated_hourly_cost: (spec.cpu * 0.05 + spec.memory_gb * 0.01) as f64,
        }
    }

    // Cost estimation helpers
    fn estimate_aws_cost(instance_type: &str) -> f64 {
        match instance_type {
            "t3.micro" => 0.0104,
            "t3.small" => 0.0208,
            "t3.medium" => 0.0416,
            "t3.large" => 0.0832,
            "m6i.xlarge" => 0.192,
            "m6i.2xlarge" => 0.384,
            "m6i.4xlarge" => 0.768,
            "g4dn.xlarge" => 0.526, // GPU
            "p3.8xlarge" => 12.24,  // High-end GPU
            _ => 0.10,
        }
    }

    fn estimate_gcp_cost(instance_type: &str) -> f64 {
        match instance_type {
            "e2-micro" => 0.008,
            "e2-small" => 0.017,
            "e2-medium" => 0.034,
            "n2-standard-4" => 0.17,
            "n2-standard-8" => 0.34,
            _ => 0.10,
        }
    }

    fn estimate_azure_cost(instance_type: &str) -> f64 {
        match instance_type {
            "Standard_B1s" => 0.012,
            "Standard_B2s" => 0.048,
            "Standard_D2s_v5" => 0.096,
            "Standard_D4s_v5" => 0.192,
            _ => 0.10,
        }
    }

    fn estimate_do_cost(instance_type: &str) -> f64 {
        match instance_type {
            "s-1vcpu-1gb" => 0.009,
            "s-1vcpu-2gb" => 0.018,
            "s-2vcpu-4gb" => 0.036,
            "s-4vcpu-8gb" => 0.072,
            _ => 0.05,
        }
    }

    fn estimate_vultr_cost(instance_type: &str) -> f64 {
        match instance_type {
            "vc2-1c-1gb" => 0.007,
            "vc2-2c-4gb" => 0.024,
            "vc2-4c-8gb" => 0.048,
            _ => 0.05,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceSelection {
    pub instance_type: String,
    pub spot_capable: bool,
    pub estimated_hourly_cost: f64,
}

/// Auto-scaling configuration that works for both local and remote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingConfig {
    pub min_replicas: u32,
    pub max_replicas: u32,
    pub target_cpu_percent: f64,
    pub target_memory_percent: f64,
    pub scale_up_cooldown_seconds: u64,
    pub scale_down_cooldown_seconds: u64,
}

impl Default for AutoScalingConfig {
    fn default() -> Self {
        Self {
            min_replicas: 1,
            max_replicas: 10,
            target_cpu_percent: 70.0,
            target_memory_percent: 80.0,
            scale_up_cooldown_seconds: 60,
            scale_down_cooldown_seconds: 300,
        }
    }
}

/// Extension trait for existing ContainerRuntime to apply resource limits
pub trait ResourceLimitsExt {
    /// Apply resource requirements to a deployment
    /// - For local: Sets Kubernetes resource limits
    /// - For remote: Ensures proper node selection
    fn apply_resource_requirements(&mut self, spec: &ResourceSpec);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{ComputeResources, ResourceSpec, StorageResources};

    #[test]
    fn test_aws_instance_mapping() {
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 4.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 16.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let selection = InstanceTypeMapper::map_to_instance_type(&spec, &CloudProvider::AWS);
        assert_eq!(selection.instance_type, "m6i.xlarge");
        assert!(selection.estimated_hourly_cost > 0.0);
    }

    #[test]
    fn test_gpu_instance_selection() {
        use crate::resources::{AcceleratorResources, AcceleratorType, GpuSpec};

        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 4.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 16.0,
                ..Default::default()
            },
            accelerators: Some(AcceleratorResources {
                count: 1,
                accelerator_type: AcceleratorType::GPU(GpuSpec {
                    vendor: "nvidia".to_string(),
                    model: "t4".to_string(),
                    min_vram_gb: 16.0,
                }),
            }),
            ..Default::default()
        };

        let selection = InstanceTypeMapper::map_to_instance_type(&spec, &CloudProvider::AWS);
        assert!(selection.instance_type.contains("g4dn"));
        assert!(!selection.spot_capable); // GPU instances shouldn't use spot by default
    }

    #[test]
    fn test_digital_ocean_mapping() {
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 2.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 4.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let selection =
            InstanceTypeMapper::map_to_instance_type(&spec, &CloudProvider::DigitalOcean);
        assert_eq!(selection.instance_type, "s-2vcpu-4gb");
        assert!(!selection.spot_capable); // DO doesn't have spot
    }

    #[test]
    fn test_cost_aware_selection() {
        use crate::resources::QosParameters;

        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 0.5,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 1.0,
                ..Default::default()
            },
            qos: QosParameters {
                allow_spot: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let selection = InstanceTypeMapper::map_to_instance_type(&spec, &CloudProvider::AWS);
        assert_eq!(selection.instance_type, "t3.micro");
        assert!(selection.spot_capable);
        assert!(selection.estimated_hourly_cost < 0.02); // Should be cheap
    }
}
