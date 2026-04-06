//! Maps `ResourceSpec` to Fluidstack plan identifiers.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct FluidstackInstanceMapper;

impl FluidstackInstanceMapper {
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let plan = Self::select(spec);
        InstanceSelection {
            instance_type: plan.to_string(),
            spot_capable: false,
            estimated_hourly_cost: Some(Self::estimate_hourly_cost(plan)),
        }
    }

    fn select(spec: &ResourceSpec) -> &'static str {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        match (spec.memory_gb, gpu_count) {
            (_, c) if c >= 8 => "h100_sxm5_8x",
            (_, c) if c >= 4 => "a100_80gb_pcie_4x",
            (_, c) if c >= 2 => "a100_80gb_pcie_2x",
            (m, _) if m >= 200.0 => "h100_80gb_pcie",
            (m, _) if m >= 80.0 => "a100_80gb_pcie",
            (m, _) if m >= 40.0 => "a100_40gb_pcie",
            (m, _) if m >= 24.0 => "rtx_a6000",
            (m, _) if m >= 16.0 => "rtx_a5000",
            _ => "rtx_a4000",
        }
    }

    pub fn estimate_hourly_cost(plan: &str) -> f64 {
        match plan {
            "rtx_a4000" => 0.15,
            "rtx_a5000" => 0.25,
            "rtx_a6000" => 0.55,
            "a100_40gb_pcie" => 1.10,
            "a100_80gb_pcie" => 1.40,
            "a100_80gb_pcie_2x" => 2.80,
            "a100_80gb_pcie_4x" => 5.60,
            "h100_80gb_pcie" => 2.89,
            "h100_sxm5_8x" => 27.12,
            _ => 1.50,
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
    fn small_spec_picks_a4000() {
        assert_eq!(
            FluidstackInstanceMapper::map(&spec(8.0, 1)).instance_type,
            "rtx_a4000"
        );
    }

    #[test]
    fn huge_spec_picks_h100_8x() {
        assert_eq!(
            FluidstackInstanceMapper::map(&spec(1024.0, 8)).instance_type,
            "h100_sxm5_8x"
        );
    }

    #[test]
    fn single_h100_for_big_memory() {
        assert_eq!(
            FluidstackInstanceMapper::map(&spec(256.0, 1)).instance_type,
            "h100_80gb_pcie"
        );
    }
}
