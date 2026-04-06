//! Maps `ResourceSpec` to CoreWeave GPU node labels.
//!
//! CoreWeave identifies GPUs by node labels on their Kubernetes nodes, e.g.
//! `gpu.nvidia.com/class=A100_NVLINK` or `gpu.nvidia.com/model=H100`.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct CoreWeaveInstanceMapper;

impl CoreWeaveInstanceMapper {
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let instance_type = Self::select_gpu_class(spec);
        InstanceSelection {
            instance_type: instance_type.to_string(),
            spot_capable: false,
            estimated_hourly_cost: Some(Self::estimate_hourly_cost(instance_type)),
        }
    }

    fn select_gpu_class(spec: &ResourceSpec) -> &'static str {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        if gpu_count >= 8 {
            return "H100_NVLINK_80GB";
        }
        if spec.memory_gb >= 200.0 {
            "H100_PCIE_80GB"
        } else if spec.memory_gb >= 80.0 {
            "A100_NVLINK_80GB"
        } else if spec.memory_gb >= 40.0 {
            "A100_PCIE_40GB"
        } else if spec.memory_gb >= 24.0 {
            "A40"
        } else {
            "Quadro_RTX_4000"
        }
    }

    pub fn estimate_hourly_cost(class: &str) -> f64 {
        match class {
            "Quadro_RTX_4000" => 0.24,
            "A40" => 1.28,
            "A100_PCIE_40GB" => 2.06,
            "A100_NVLINK_80GB" => 2.21,
            "H100_PCIE_80GB" => 4.25,
            "H100_NVLINK_80GB" => 4.76,
            _ => 2.00,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(mem: f32, gpus: u32) -> ResourceSpec {
        ResourceSpec {
            cpu: 8.0,
            memory_gb: mem,
            storage_gb: 200.0,
            gpu_count: Some(gpus),
            allow_spot: false,
            qos: Default::default(),
        }
    }

    #[test]
    fn small_spec_picks_quadro() {
        assert_eq!(
            CoreWeaveInstanceMapper::map(&spec(16.0, 1)).instance_type,
            "Quadro_RTX_4000"
        );
    }

    #[test]
    fn big_memory_picks_h100_pcie() {
        assert_eq!(
            CoreWeaveInstanceMapper::map(&spec(256.0, 1)).instance_type,
            "H100_PCIE_80GB"
        );
    }

    #[test]
    fn eight_gpu_picks_h100_nvlink() {
        assert_eq!(
            CoreWeaveInstanceMapper::map(&spec(1024.0, 8)).instance_type,
            "H100_NVLINK_80GB"
        );
    }

    #[test]
    fn never_spot_capable() {
        assert!(!CoreWeaveInstanceMapper::map(&spec(32.0, 1)).spot_capable);
    }
}
