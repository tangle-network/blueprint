//! Maps `ResourceSpec` to Crusoe Cloud VM types.
//!
//! Crusoe focuses on GPU workloads powered by clean energy.
//! Types follow `{gpu}.{count}x` naming, e.g. `a100.1x`, `h100.8x`.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct CrusoeInstanceMapper;

impl CrusoeInstanceMapper {
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let (vm_type, cost) = if spec.gpu_count.unwrap_or(0) > 0 {
            Self::select_gpu_type(spec)
        } else {
            Self::select_cpu_type(spec)
        };
        InstanceSelection {
            instance_type: vm_type.to_string(),
            spot_capable: false,
            estimated_hourly_cost: Some(cost),
        }
    }

    fn select_gpu_type(spec: &ResourceSpec) -> (&'static str, f64) {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        if gpu_count >= 8 && spec.memory_gb >= 640.0 {
            ("h100.8x", 25.60)
        } else if gpu_count >= 4 && spec.memory_gb >= 320.0 {
            ("a100.4x", 8.24)
        } else if gpu_count >= 2 && spec.memory_gb >= 160.0 {
            ("a100.2x", 4.12)
        } else if spec.memory_gb >= 80.0 {
            ("a100.1x", 2.06)
        } else {
            ("l40s.1x", 1.24)
        }
    }

    fn select_cpu_type(spec: &ResourceSpec) -> (&'static str, f64) {
        if spec.cpu <= 4.0 && spec.memory_gb <= 16.0 {
            ("c1.4x16", 0.12)
        } else if spec.cpu <= 8.0 && spec.memory_gb <= 32.0 {
            ("c1.8x32", 0.24)
        } else {
            ("c1.16x64", 0.48)
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
    fn single_l40s_for_small_gpu() {
        let s = CrusoeInstanceMapper::map(&spec(8.0, 32.0, Some(1), false));
        assert_eq!(s.instance_type, "l40s.1x");
    }

    #[test]
    fn single_a100_for_large_memory() {
        let s = CrusoeInstanceMapper::map(&spec(16.0, 128.0, Some(1), false));
        assert_eq!(s.instance_type, "a100.1x");
    }

    #[test]
    fn eight_h100_for_cluster() {
        let s = CrusoeInstanceMapper::map(&spec(128.0, 1024.0, Some(8), false));
        assert_eq!(s.instance_type, "h100.8x");
    }

    #[test]
    fn cpu_only() {
        let s = CrusoeInstanceMapper::map(&spec(4.0, 16.0, None, false));
        assert_eq!(s.instance_type, "c1.4x16");
    }
}
