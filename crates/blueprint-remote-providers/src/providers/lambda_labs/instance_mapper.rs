//! Maps `ResourceSpec` to Lambda Labs instance types.
//!
//! Catalog source: <https://lambdalabs.com/service/gpu-cloud#pricing>. These are
//! the on-demand SKUs exposed via the Lambda Labs REST API as `instance_type_name`.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct LambdaLabsInstanceMapper;

impl LambdaLabsInstanceMapper {
    /// Select the cheapest Lambda Labs instance type that satisfies the spec.
    ///
    /// Lambda Labs exclusively sells GPU instances — there is no CPU-only SKU.
    /// When `gpu_count` is `None` we still pick the smallest GPU box because the
    /// caller explicitly asked for a Lambda Labs instance.
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        let instance_type = match gpu_count {
            1 => select_single_gpu(spec),
            2 => "gpu_2x_a100_pcie",
            4 => "gpu_4x_a100",
            8 if spec.memory_gb > 512.0 => "gpu_8x_h100_sxm",
            8 => "gpu_8x_a100",
            _ => "gpu_8x_h100_sxm",
        };

        InstanceSelection {
            instance_type: instance_type.to_string(),
            // Lambda Labs has no spot tier for on-demand.
            spot_capable: false,
            estimated_hourly_cost: Some(Self::estimate_hourly_cost(instance_type)),
        }
    }

    /// Hourly cost estimates in USD as of 2026-04 published pricing.
    /// These are used for logging and cost ceiling checks; billing is still
    /// reconciled against the Lambda Labs invoice.
    pub fn estimate_hourly_cost(instance_type: &str) -> f64 {
        match instance_type {
            "gpu_1x_a10" => 0.75,
            "gpu_1x_a100_pcie" => 1.29,
            "gpu_1x_a100" => 1.29,
            "gpu_1x_a6000" => 0.80,
            "gpu_1x_h100_pcie" => 2.49,
            "gpu_2x_a100_pcie" => 2.58,
            "gpu_4x_a100" => 5.16,
            "gpu_8x_a100" => 10.32,
            "gpu_8x_h100_sxm" => 23.92,
            _ => 2.00,
        }
    }
}

/// Pick the best single-GPU SKU based on VRAM and memory hints from the spec.
fn select_single_gpu(spec: &ResourceSpec) -> &'static str {
    // Interpret memory_gb as a rough proxy for "how hefty a GPU do you need".
    // A10 ~24GB VRAM, A6000 ~48GB, A100 ~40/80GB, H100 ~80GB.
    if spec.memory_gb >= 200.0 {
        "gpu_1x_h100_pcie"
    } else if spec.memory_gb >= 100.0 {
        "gpu_1x_a100"
    } else if spec.memory_gb >= 48.0 {
        "gpu_1x_a6000"
    } else {
        "gpu_1x_a10"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_single_a10_for_small_spec() {
        let spec = ResourceSpec {
            cpu: 4.0,
            memory_gb: 16.0,
            storage_gb: 100.0,
            gpu_count: Some(1),
            allow_spot: false,
            qos: Default::default(),
        };
        let selection = LambdaLabsInstanceMapper::map(&spec);
        assert_eq!(selection.instance_type, "gpu_1x_a10");
        assert!(!selection.spot_capable);
    }

    #[test]
    fn selects_h100_for_heavy_memory() {
        let spec = ResourceSpec {
            cpu: 16.0,
            memory_gb: 256.0,
            storage_gb: 1000.0,
            gpu_count: Some(1),
            allow_spot: false,
            qos: Default::default(),
        };
        let selection = LambdaLabsInstanceMapper::map(&spec);
        assert_eq!(selection.instance_type, "gpu_1x_h100_pcie");
    }

    #[test]
    fn selects_8x_h100_for_sxm_class() {
        let spec = ResourceSpec {
            cpu: 64.0,
            memory_gb: 1024.0,
            storage_gb: 8000.0,
            gpu_count: Some(8),
            allow_spot: false,
            qos: Default::default(),
        };
        let selection = LambdaLabsInstanceMapper::map(&spec);
        assert_eq!(selection.instance_type, "gpu_8x_h100_sxm");
    }

    #[test]
    fn never_spot_capable() {
        let spec = ResourceSpec {
            cpu: 4.0,
            memory_gb: 16.0,
            storage_gb: 100.0,
            gpu_count: Some(2),
            allow_spot: true,
            qos: Default::default(),
        };
        let selection = LambdaLabsInstanceMapper::map(&spec);
        assert!(!selection.spot_capable);
    }

    #[test]
    fn cost_estimate_scales_with_gpu_count() {
        let single = LambdaLabsInstanceMapper::estimate_hourly_cost("gpu_1x_a100");
        let eight = LambdaLabsInstanceMapper::estimate_hourly_cost("gpu_8x_a100");
        assert!(eight > single * 4.0, "8x should be more than 4x single");
    }

    #[test]
    fn unknown_instance_falls_back_to_default_cost() {
        let cost = LambdaLabsInstanceMapper::estimate_hourly_cost("gpu_42x_unobtainium");
        assert!(
            (cost - 2.0).abs() < f64::EPSILON,
            "expected 2.0, got {cost}"
        );
    }
}
