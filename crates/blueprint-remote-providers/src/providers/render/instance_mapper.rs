//! Maps `ResourceSpec` to Render Network Dispersed compute tiers.
//!
//! Catalog source: <https://dispersed.com/docs> public preview pricing as of
//! 2026-04. Render exposes a small number of opinionated tiers rather than raw
//! GPU SKUs — the mapper picks the cheapest tier whose VRAM headroom satisfies
//! the requested memory.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct RenderInstanceMapper;

impl RenderInstanceMapper {
    /// Select the cheapest Render Dispersed tier for the spec.
    ///
    /// Render's catalog is intentionally narrow (entry through 8x H100 cluster).
    /// We use `gpu_count` as the primary signal and `memory_gb` as a tiebreaker
    /// for single-node selection.
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        let tier = match gpu_count {
            1 => select_single_tier(spec),
            2..=4 => "cluster-4x",
            _ => "cluster-8x",
        };

        InstanceSelection {
            instance_type: tier.to_string(),
            // Dispersed has no separate spot tier — the network already prices
            // against decentralized supply.
            spot_capable: false,
            estimated_hourly_cost: Some(Self::estimate_hourly_cost(tier)),
        }
    }

    /// Hourly cost estimates in USD as of 2026-04 published Dispersed pricing.
    pub fn estimate_hourly_cost(tier: &str) -> f64 {
        match tier {
            "entry" => 0.35,
            "standard" => 1.29,
            "performance" => 1.59,
            "premium" => 2.45,
            "cluster-4x" => 6.36,
            "cluster-8x" => 19.60,
            _ => 1.50,
        }
    }
}

fn select_single_tier(spec: &ResourceSpec) -> &'static str {
    if spec.memory_gb >= 200.0 {
        "premium"
    } else if spec.memory_gb >= 80.0 {
        "performance"
    } else if spec.memory_gb >= 40.0 {
        "standard"
    } else {
        "entry"
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
    fn entry_tier_for_small_workload() {
        let selection = RenderInstanceMapper::map(&spec(8.0, 1));
        assert_eq!(selection.instance_type, "entry");
    }

    #[test]
    fn premium_tier_for_heavy_memory() {
        let selection = RenderInstanceMapper::map(&spec(256.0, 1));
        assert_eq!(selection.instance_type, "premium");
    }

    #[test]
    fn cluster_tier_for_4_gpus() {
        let selection = RenderInstanceMapper::map(&spec(256.0, 4));
        assert_eq!(selection.instance_type, "cluster-4x");
    }

    #[test]
    fn cluster_8x_for_8_gpus() {
        let selection = RenderInstanceMapper::map(&spec(640.0, 8));
        assert_eq!(selection.instance_type, "cluster-8x");
    }

    #[test]
    fn never_spot_capable() {
        let selection = RenderInstanceMapper::map(&spec(64.0, 1));
        assert!(!selection.spot_capable);
    }

    #[test]
    fn unknown_tier_falls_back() {
        assert_eq!(RenderInstanceMapper::estimate_hourly_cost("ultra"), 1.50);
    }
}
