//! Maps `ResourceSpec` to Akash SDL GPU profile identifiers.
//!
//! Akash providers advertise GPU SKUs via SDL `profiles.compute.*.resources.gpu`
//! attributes. We hardcode a small catalog of named profiles that the relay
//! understands and translates to a full SDL manifest.
//!
//! Pricing is in USD/hour and reflects published Akash marketplace ranges as of
//! 2026-04. Final settlement is reconciled against the on-chain lease price.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct AkashInstanceMapper;

impl AkashInstanceMapper {
    /// Select the cheapest Akash GPU profile that satisfies the spec.
    ///
    /// Akash's marketplace is GPU-centric for blueprint workloads — when no
    /// `gpu_count` is provided we still pick the smallest GPU SKU because the
    /// caller explicitly chose the Akash provider.
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        let instance_type = match gpu_count {
            1 => select_single_gpu(spec),
            8 if spec.memory_gb >= 1024.0 => "gpu-h100-8x",
            8 => "gpu-a100-80gb-8x",
            _ if spec.memory_gb >= 256.0 => "gpu-h100",
            _ => "gpu-a100-80gb",
        };

        InstanceSelection {
            instance_type: instance_type.to_string(),
            // Akash leases are bid-based; spot semantics don't apply.
            spot_capable: false,
            estimated_hourly_cost: Some(Self::estimate_hourly_cost(instance_type)),
        }
    }

    /// Hourly cost estimates in USD. Used for logging and cost ceilings; the
    /// authoritative price is the on-chain lease price negotiated via bids.
    pub fn estimate_hourly_cost(instance_type: &str) -> f64 {
        match instance_type {
            "gpu-t4-small" => 0.12,
            "gpu-t4" => 0.20,
            "gpu-a10" => 0.45,
            "gpu-a100-40gb" => 1.10,
            "gpu-a100-80gb" => 1.50,
            "gpu-h100" => 2.50,
            "gpu-h200" => 3.20,
            "gpu-a100-80gb-8x" => 10.00,
            "gpu-h100-8x" => 18.00,
            _ => 1.00,
        }
    }
}

/// Pick the cheapest single-GPU profile based on memory hints.
fn select_single_gpu(spec: &ResourceSpec) -> &'static str {
    if spec.memory_gb >= 256.0 {
        "gpu-h200"
    } else if spec.memory_gb >= 160.0 {
        "gpu-h100"
    } else if spec.memory_gb >= 80.0 {
        "gpu-a100-80gb"
    } else if spec.memory_gb >= 40.0 {
        "gpu-a100-40gb"
    } else if spec.memory_gb >= 24.0 {
        "gpu-a10"
    } else if spec.memory_gb >= 16.0 {
        "gpu-t4"
    } else {
        "gpu-t4-small"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(cpu: f32, mem: f32, gpus: Option<u32>) -> ResourceSpec {
        ResourceSpec {
            cpu,
            memory_gb: mem,
            storage_gb: 100.0,
            gpu_count: gpus,
            allow_spot: false,
            qos: Default::default(),
        }
    }

    #[test]
    fn selects_t4_small_for_minimal_spec() {
        let selection = AkashInstanceMapper::map(&spec(2.0, 8.0, Some(1)));
        assert_eq!(selection.instance_type, "gpu-t4-small");
        assert!(!selection.spot_capable);
    }

    #[test]
    fn selects_a100_80gb_for_mid_memory() {
        let selection = AkashInstanceMapper::map(&spec(8.0, 96.0, Some(1)));
        assert_eq!(selection.instance_type, "gpu-a100-80gb");
    }

    #[test]
    fn selects_h200_for_huge_memory() {
        let selection = AkashInstanceMapper::map(&spec(16.0, 320.0, Some(1)));
        assert_eq!(selection.instance_type, "gpu-h200");
    }

    #[test]
    fn selects_8x_h100_for_large_cluster() {
        let selection = AkashInstanceMapper::map(&spec(64.0, 1536.0, Some(8)));
        assert_eq!(selection.instance_type, "gpu-h100-8x");
    }

    #[test]
    fn selects_8x_a100_for_smaller_cluster() {
        let selection = AkashInstanceMapper::map(&spec(64.0, 512.0, Some(8)));
        assert_eq!(selection.instance_type, "gpu-a100-80gb-8x");
    }

    #[test]
    fn defaults_to_one_gpu_when_unset() {
        let selection = AkashInstanceMapper::map(&spec(2.0, 8.0, None));
        assert_eq!(selection.instance_type, "gpu-t4-small");
    }

    #[test]
    fn never_spot_capable() {
        let selection = AkashInstanceMapper::map(&ResourceSpec {
            allow_spot: true,
            ..spec(4.0, 16.0, Some(1))
        });
        assert!(!selection.spot_capable);
    }

    #[test]
    fn cost_estimate_increases_with_class() {
        let t4 = AkashInstanceMapper::estimate_hourly_cost("gpu-t4-small");
        let a100 = AkashInstanceMapper::estimate_hourly_cost("gpu-a100-80gb");
        let h100_8x = AkashInstanceMapper::estimate_hourly_cost("gpu-h100-8x");
        assert!(a100 > t4);
        assert!(h100_8x > a100);
    }

    #[test]
    fn unknown_profile_falls_back_to_default_cost() {
        let cost = AkashInstanceMapper::estimate_hourly_cost("gpu-unknown");
        assert!(
            (cost - 1.00).abs() < f64::EPSILON,
            "expected 1.00, got {cost}"
        );
    }
}
