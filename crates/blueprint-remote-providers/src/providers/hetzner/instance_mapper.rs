//! Maps `ResourceSpec` to Hetzner Cloud server types.
//!
//! Hetzner uses names like `cpx11` (shared vCPU), `ccx13` (dedicated vCPU),
//! and `GX11` (GPU — A100 40GB).

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct HetznerInstanceMapper;

impl HetznerInstanceMapper {
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let (server_type, cost) = if spec.gpu_count.unwrap_or(0) > 0 {
            Self::select_gpu_type(spec)
        } else {
            Self::select_cpu_type(spec)
        };
        InstanceSelection {
            instance_type: server_type.to_string(),
            spot_capable: false, // Hetzner does not offer spot instances
            estimated_hourly_cost: Some(cost),
        }
    }

    fn select_gpu_type(spec: &ResourceSpec) -> (&'static str, f64) {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        if gpu_count >= 4 && spec.memory_gb >= 320.0 {
            ("GX44", 7.96)
        } else if gpu_count >= 2 && spec.memory_gb >= 160.0 {
            ("GX22", 3.98)
        } else {
            ("GX11", 1.99)
        }
    }

    fn select_cpu_type(spec: &ResourceSpec) -> (&'static str, f64) {
        if spec.cpu <= 2.0 && spec.memory_gb <= 4.0 {
            ("cpx11", 0.007)
        } else if spec.cpu <= 3.0 && spec.memory_gb <= 8.0 {
            ("cpx21", 0.012)
        } else if spec.cpu <= 4.0 && spec.memory_gb <= 16.0 {
            ("cpx31", 0.023)
        } else if spec.cpu <= 8.0 && spec.memory_gb <= 32.0 {
            ("cpx41", 0.046)
        } else {
            ("cpx51", 0.092)
        }
    }
}
