//! Maps `ResourceSpec` to TensorDock GPU model strings.
//!
//! TensorDock identifies GPUs by model code like `rtx4090-24gb` or `a100-80gb`.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct TensorDockInstanceMapper;

impl TensorDockInstanceMapper {
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let gpu_model = Self::select(spec);
        InstanceSelection {
            instance_type: gpu_model.to_string(),
            spot_capable: false,
            estimated_hourly_cost: Some(Self::estimate_hourly_cost(gpu_model)),
        }
    }

    fn select(spec: &ResourceSpec) -> &'static str {
        if spec.memory_gb >= 200.0 {
            "h100-80gb-sxm5"
        } else if spec.memory_gb >= 80.0 {
            "a100-80gb"
        } else if spec.memory_gb >= 40.0 {
            "a100-40gb"
        } else if spec.memory_gb >= 24.0 {
            "rtx4090-24gb"
        } else if spec.memory_gb >= 16.0 {
            "rtxa5000-24gb"
        } else {
            "rtx3090-24gb"
        }
    }

    pub fn estimate_hourly_cost(model: &str) -> f64 {
        match model {
            "rtx3090-24gb" => 0.22,
            "rtxa5000-24gb" => 0.35,
            "rtx4090-24gb" => 0.44,
            "a100-40gb" => 0.95,
            "a100-80gb" => 1.45,
            "h100-80gb-sxm5" => 2.95,
            _ => 1.00,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(mem: f32) -> ResourceSpec {
        ResourceSpec {
            cpu: 8.0,
            memory_gb: mem,
            storage_gb: 100.0,
            gpu_count: Some(1),
            allow_spot: false,
            qos: Default::default(),
        }
    }

    #[test]
    fn tiny_picks_rtx3090() {
        assert_eq!(
            TensorDockInstanceMapper::map(&spec(8.0)).instance_type,
            "rtx3090-24gb"
        );
    }

    #[test]
    fn huge_picks_h100() {
        assert_eq!(
            TensorDockInstanceMapper::map(&spec(256.0)).instance_type,
            "h100-80gb-sxm5"
        );
    }
}
