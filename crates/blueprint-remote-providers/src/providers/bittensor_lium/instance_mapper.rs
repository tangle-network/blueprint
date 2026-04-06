//! Maps `ResourceSpec` to Bittensor Lium GPU types.
//!
//! Catalog source: Lium subnet 51 inventory listings as of 2026-04. Lium's
//! deepest pool is H100 (500+ cards across the subnet), so the mapper defaults
//! to H100 whenever the spec is heavy enough — that's where Lium has the best
//! liquidity and the most consistent miner SLAs.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct BittensorLiumInstanceMapper;

impl BittensorLiumInstanceMapper {
    /// Select the cheapest Lium GPU type that satisfies the spec.
    ///
    /// Lium's H100 inventory is the broadest, so multi-GPU rentals always pick
    /// H100 — sub-H100 multi-GPU configs are rare on the subnet and pricing is
    /// rarely competitive against single-GPU H100 rentals.
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        let gpu_type = match gpu_count {
            1 => select_single_gpu(spec),
            _ => "H100-80GB",
        };

        InstanceSelection {
            instance_type: gpu_type.to_string(),
            // Lium pricing is auction-based per miner; there's no separate spot tier.
            spot_capable: false,
            estimated_hourly_cost: Some(Self::estimate_hourly_cost(gpu_type) * gpu_count as f64),
        }
    }

    /// Hourly cost estimates in USD as of 2026-04 subnet pricing.
    /// Lium pricing is denominated in TAO; these are the USD-equivalent rates
    /// observed at the marketplace level. Actual settlement happens in TAO.
    pub fn estimate_hourly_cost(gpu_type: &str) -> f64 {
        match gpu_type {
            "H100-80GB" => 2.20,
            "A100-80GB" => 1.40,
            "A100-40GB" => 0.95,
            "RTX_4090" => 0.42,
            "RTX_3090" => 0.19,
            _ => 1.50,
        }
    }
}

fn select_single_gpu(spec: &ResourceSpec) -> &'static str {
    // Default to H100 (Lium's strength) above ~80GB system RAM. Below that,
    // smaller cards are usually cheaper on the subnet.
    if spec.memory_gb >= 200.0 {
        "H100-80GB"
    } else if spec.memory_gb >= 100.0 {
        "A100-80GB"
    } else if spec.memory_gb >= 60.0 {
        "A100-40GB"
    } else if spec.memory_gb >= 16.0 {
        "RTX_4090"
    } else {
        "RTX_3090"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(memory_gb: f32, gpu_count: u32) -> ResourceSpec {
        ResourceSpec {
            cpu: 8.0,
            memory_gb,
            storage_gb: 100.0,
            gpu_count: Some(gpu_count),
            allow_spot: false,
            qos: Default::default(),
        }
    }

    #[test]
    fn small_workload_picks_rtx_3090() {
        let selection = BittensorLiumInstanceMapper::map(&spec(8.0, 1));
        assert_eq!(selection.instance_type, "RTX_3090");
    }

    #[test]
    fn mid_workload_picks_a100_80gb() {
        let selection = BittensorLiumInstanceMapper::map(&spec(128.0, 1));
        assert_eq!(selection.instance_type, "A100-80GB");
    }

    #[test]
    fn heavy_single_gpu_picks_h100() {
        let selection = BittensorLiumInstanceMapper::map(&spec(256.0, 1));
        assert_eq!(selection.instance_type, "H100-80GB");
    }

    #[test]
    fn multi_gpu_always_h100() {
        let selection = BittensorLiumInstanceMapper::map(&spec(64.0, 4));
        assert_eq!(selection.instance_type, "H100-80GB");
    }

    #[test]
    fn cost_scales_with_gpu_count() {
        let single = BittensorLiumInstanceMapper::map(&spec(256.0, 1))
            .estimated_hourly_cost
            .unwrap();
        let octa = BittensorLiumInstanceMapper::map(&spec(256.0, 8))
            .estimated_hourly_cost
            .unwrap();
        assert!(octa > single * 7.0);
    }

    #[test]
    fn unknown_gpu_falls_back() {
        let cost = BittensorLiumInstanceMapper::estimate_hourly_cost("MI300X");
        assert!(
            (cost - 1.50).abs() < f64::EPSILON,
            "expected 1.50, got {cost}"
        );
    }
}
