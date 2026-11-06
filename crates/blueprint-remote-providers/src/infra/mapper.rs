use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::providers::akash::AkashInstanceMapper;
use crate::providers::bittensor_lium::BittensorLiumInstanceMapper;
pub use crate::providers::common::InstanceSelection;
use crate::providers::coreweave::CoreWeaveInstanceMapper;
use crate::providers::crusoe::CrusoeInstanceMapper;
use crate::providers::fluidstack::FluidstackInstanceMapper;
use crate::providers::hetzner::HetznerInstanceMapper;
use crate::providers::io_net::IoNetInstanceMapper;
use crate::providers::lambda_labs::LambdaLabsInstanceMapper;
use crate::providers::paperspace::PaperspaceInstanceMapper;
use crate::providers::prime_intellect::PrimeIntellectInstanceMapper;
use crate::providers::render::RenderInstanceMapper;
use crate::providers::runpod::RunPodInstanceMapper;
use crate::providers::tensordock::TensorDockInstanceMapper;
use crate::providers::vast_ai::VastAiInstanceMapper;
use serde::{Deserialize, Serialize};

/// Maps resource requirements to cloud instance types
pub struct InstanceTypeMapper;

impl InstanceTypeMapper {
    /// Map resource spec to specific instance type
    pub fn map_to_instance_type(
        spec: &ResourceSpec,
        provider: &CloudProvider,
    ) -> InstanceSelection {
        Self::map_to_instance_type_with_requirements(spec, provider, false)
    }

    /// Map resource spec to instance type while honoring deployment requirements.
    pub fn map_to_instance_type_with_requirements(
        spec: &ResourceSpec,
        provider: &CloudProvider,
        require_tee: bool,
    ) -> InstanceSelection {
        if require_tee {
            return Self::map_tee_instance(spec, provider);
        }

        match provider {
            CloudProvider::AWS => Self::map_aws_instance(spec),
            CloudProvider::GCP => Self::map_gcp_instance(spec),
            CloudProvider::Azure => Self::map_azure_instance(spec),
            CloudProvider::DigitalOcean => Self::map_do_instance(spec),
            CloudProvider::Vultr => Self::map_vultr_instance(spec),
            CloudProvider::LambdaLabs => Self::from_common(LambdaLabsInstanceMapper::map(spec)),
            CloudProvider::RunPod => Self::from_common(RunPodInstanceMapper::map(spec)),
            CloudProvider::VastAi => Self::from_common(VastAiInstanceMapper::map(spec)),
            CloudProvider::CoreWeave => Self::from_common(CoreWeaveInstanceMapper::map(spec)),
            CloudProvider::Paperspace => Self::from_common(PaperspaceInstanceMapper::map(spec)),
            CloudProvider::Fluidstack => Self::from_common(FluidstackInstanceMapper::map(spec)),
            CloudProvider::TensorDock => Self::from_common(TensorDockInstanceMapper::map(spec)),
            CloudProvider::Akash => Self::from_common(AkashInstanceMapper::map(spec)),
            CloudProvider::IoNet => Self::from_common(IoNetInstanceMapper::map(spec)),
            CloudProvider::PrimeIntellect => {
                Self::from_common(PrimeIntellectInstanceMapper::map(spec))
            }
            CloudProvider::Render => Self::from_common(RenderInstanceMapper::map(spec)),
            CloudProvider::BittensorLium => {
                Self::from_common(BittensorLiumInstanceMapper::map(spec))
            }
            CloudProvider::Hetzner => Self::from_common(HetznerInstanceMapper::map(spec)),
            CloudProvider::Crusoe => Self::from_common(CrusoeInstanceMapper::map(spec)),
            _ => Self::map_generic_instance(spec),
        }
    }

    fn map_tee_instance(spec: &ResourceSpec, provider: &CloudProvider) -> InstanceSelection {
        let instance_type = match provider {
            CloudProvider::AWS => Self::map_aws_tee_instance_type(spec).to_string(),
            CloudProvider::GCP => Self::map_gcp_tee_instance_type(spec).to_string(),
            CloudProvider::Azure => Self::map_azure_tee_instance_type(spec).to_string(),
            // Non-confidential providers must already be filtered by manager/provisioner.
            _ => Self::map_generic_instance(spec).instance_type,
        };

        InstanceSelection {
            instance_type: instance_type.clone(),
            // Confidential-compute SKUs are generally not spot-capable in managed flows.
            spot_capable: false,
            estimated_hourly_cost: None,
        }
    }

    fn map_aws_tee_instance_type(spec: &ResourceSpec) -> &'static str {
        match (spec.cpu, spec.memory_gb, spec.gpu_count) {
            (_, _, Some(_)) => "g5.xlarge",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 8.0 => "m6i.large",
            (cpu, mem, _) if cpu <= 4.0 && mem <= 16.0 => "m6i.xlarge",
            (cpu, mem, _) if cpu <= 8.0 && mem <= 32.0 => "m6i.2xlarge",
            _ => "c6i.4xlarge",
        }
    }

    fn map_gcp_tee_instance_type(spec: &ResourceSpec) -> &'static str {
        match (spec.cpu, spec.memory_gb, spec.gpu_count) {
            (_, _, Some(_)) => "n2d-standard-8",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 8.0 => "n2d-standard-2",
            (cpu, mem, _) if cpu <= 4.0 && mem <= 16.0 => "n2d-standard-4",
            (cpu, mem, _) if cpu <= 8.0 && mem <= 32.0 => "n2d-standard-8",
            _ => "n2d-standard-16",
        }
    }

    fn map_azure_tee_instance_type(spec: &ResourceSpec) -> &'static str {
        match (spec.cpu, spec.memory_gb, spec.gpu_count) {
            (_, _, Some(_)) => "Standard_DC8as_v5",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 8.0 => "Standard_DC2as_v5",
            (cpu, mem, _) if cpu <= 4.0 && mem <= 16.0 => "Standard_DC4as_v5",
            (cpu, mem, _) if cpu <= 8.0 && mem <= 32.0 => "Standard_DC8as_v5",
            _ => "Standard_DC16as_v5",
        }
    }

    fn map_aws_instance(spec: &ResourceSpec) -> InstanceSelection {
        // Instance types based on AWS EC2 offerings
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
            estimated_hourly_cost: None,
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
            estimated_hourly_cost: None,
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
            estimated_hourly_cost: None,
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
            estimated_hourly_cost: None,
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
            estimated_hourly_cost: None,
        }
    }

    fn from_common(common: crate::providers::common::InstanceSelection) -> InstanceSelection {
        InstanceSelection {
            instance_type: common.instance_type,
            spot_capable: common.spot_capable,
            estimated_hourly_cost: None,
        }
    }

    fn map_generic_instance(spec: &ResourceSpec) -> InstanceSelection {
        InstanceSelection {
            instance_type: format!("{}cpu-{}gb", spec.cpu, spec.memory_gb),
            spot_capable: false,
            estimated_hourly_cost: None,
        }
    }
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
