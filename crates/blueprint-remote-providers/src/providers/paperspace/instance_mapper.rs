//! Maps `ResourceSpec` to Paperspace machine types.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct PaperspaceInstanceMapper;

impl PaperspaceInstanceMapper {
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let instance_type = Self::select(spec);
        InstanceSelection {
            instance_type: instance_type.to_string(),
            spot_capable: false,
            estimated_hourly_cost: Some(Self::estimate_hourly_cost(instance_type)),
        }
    }

    fn select(spec: &ResourceSpec) -> &'static str {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        if gpu_count >= 8 {
            return "H100x8";
        }
        if gpu_count >= 4 {
            return "A100-80Gx4";
        }
        if gpu_count >= 2 {
            return "A100-80Gx2";
        }
        if spec.memory_gb >= 200.0 {
            "H100"
        } else if spec.memory_gb >= 80.0 {
            "A100-80G"
        } else if spec.memory_gb >= 40.0 {
            "A100"
        } else if spec.memory_gb >= 24.0 {
            "A6000"
        } else if spec.memory_gb >= 16.0 {
            "A5000"
        } else {
            "P4000"
        }
    }

    pub fn estimate_hourly_cost(machine: &str) -> f64 {
        match machine {
            "P4000" => 0.51,
            "A5000" => 0.76,
            "A6000" => 1.89,
            "A100" => 3.09,
            "A100-80G" => 3.18,
            "A100-80Gx2" => 6.36,
            "A100-80Gx4" => 12.72,
            "H100" => 5.95,
            "H100x8" => 47.60,
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
            storage_gb: 100.0,
            gpu_count: Some(gpus),
            allow_spot: false,
            qos: Default::default(),
        }
    }

    #[test]
    fn small_picks_p4000() {
        assert_eq!(
            PaperspaceInstanceMapper::map(&spec(8.0, 1)).instance_type,
            "P4000"
        );
    }

    #[test]
    fn eight_picks_h100x8() {
        assert_eq!(
            PaperspaceInstanceMapper::map(&spec(1024.0, 8)).instance_type,
            "H100x8"
        );
    }

    #[test]
    fn a100_80g_for_80gb_memory() {
        assert_eq!(
            PaperspaceInstanceMapper::map(&spec(96.0, 1)).instance_type,
            "A100-80G"
        );
    }
}
