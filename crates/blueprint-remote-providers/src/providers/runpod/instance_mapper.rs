//! Maps `ResourceSpec` to RunPod `gpuTypeId` strings.
//!
//! RunPod identifies GPUs by a string like `NVIDIA GeForce RTX 4090` or
//! `NVIDIA A100 80GB PCIe`. These map onto their REST `pods` endpoint.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct RunPodInstanceMapper;

impl RunPodInstanceMapper {
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        let gpu_type_id = Self::select_gpu_type(spec, gpu_count);

        InstanceSelection {
            instance_type: gpu_type_id.to_string(),
            // COMMUNITY cloud is the spot-equivalent (cheaper, less reliable).
            spot_capable: spec.allow_spot,
            estimated_hourly_cost: Some(Self::estimate_hourly_cost(gpu_type_id)),
        }
    }

    fn select_gpu_type(spec: &ResourceSpec, gpu_count: u32) -> &'static str {
        if gpu_count >= 8 && spec.memory_gb >= 768.0 {
            return "NVIDIA H100 80GB HBM3";
        }
        if gpu_count >= 4 && spec.memory_gb >= 256.0 {
            return "NVIDIA A100 80GB PCIe";
        }
        if gpu_count >= 2 {
            return "NVIDIA A100 80GB PCIe";
        }
        // Single GPU — pick by memory.
        if spec.memory_gb >= 200.0 {
            "NVIDIA H100 80GB HBM3"
        } else if spec.memory_gb >= 100.0 {
            "NVIDIA A100 80GB PCIe"
        } else if spec.memory_gb >= 48.0 {
            "NVIDIA RTX A6000"
        } else if spec.memory_gb >= 24.0 {
            "NVIDIA GeForce RTX 4090"
        } else {
            "NVIDIA GeForce RTX 3090"
        }
    }

    pub fn estimate_hourly_cost(gpu_type_id: &str) -> f64 {
        match gpu_type_id {
            "NVIDIA GeForce RTX 3090" => 0.22,
            "NVIDIA GeForce RTX 4090" => 0.34,
            "NVIDIA RTX A6000" => 0.49,
            "NVIDIA A100 80GB PCIe" => 1.19,
            "NVIDIA H100 80GB HBM3" => 2.69,
            _ => 1.00,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(cpu: f32, mem: f32, gpu: Option<u32>, spot: bool) -> ResourceSpec {
        ResourceSpec {
            cpu,
            memory_gb: mem,
            storage_gb: 100.0,
            gpu_count: gpu,
            allow_spot: spot,
            qos: Default::default(),
        }
    }

    #[test]
    fn single_3090_for_modest_memory() {
        let selection = RunPodInstanceMapper::map(&spec(4.0, 16.0, Some(1), false));
        assert_eq!(selection.instance_type, "NVIDIA GeForce RTX 3090");
    }

    #[test]
    fn single_h100_for_big_memory() {
        let selection = RunPodInstanceMapper::map(&spec(16.0, 256.0, Some(1), false));
        assert_eq!(selection.instance_type, "NVIDIA H100 80GB HBM3");
    }

    #[test]
    fn eight_h100_for_large_cluster() {
        let selection = RunPodInstanceMapper::map(&spec(128.0, 1024.0, Some(8), false));
        assert_eq!(selection.instance_type, "NVIDIA H100 80GB HBM3");
    }

    #[test]
    fn community_cloud_is_spot() {
        let selection = RunPodInstanceMapper::map(&spec(4.0, 16.0, Some(1), true));
        assert!(selection.spot_capable);
    }
}
