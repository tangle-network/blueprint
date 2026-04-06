//! Maps `ResourceSpec` to io.net GPU types.
//!
//! Catalog source: <https://docs.io.net/reference> and the io.cloud pricing page.
//! io.net exposes GPU SKUs via the cluster launch API as a `gpu_type` field.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct IoNetInstanceMapper;

/// Selection of an io.net GPU SKU plus the per-node count to request.
#[derive(Debug, Clone)]
pub struct IoNetSelection {
    /// io.net `gpu_type` identifier.
    pub gpu_type: String,
    /// GPUs per node (we always provision a single-node cluster).
    pub gpu_count: u32,
    /// Estimated USD cost per hour.
    pub estimated_hourly_cost: f64,
}

impl IoNetInstanceMapper {
    /// Pick the cheapest io.net GPU SKU that satisfies the spec.
    ///
    /// Selection is driven by `memory_gb` (interpreted as a rough VRAM/system-memory
    /// proxy) and `gpu_count`. io.net does not offer CPU-only SKUs, so even when
    /// `gpu_count` is `None` we still allocate a single low-end GPU.
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let selection = Self::select(spec);
        InstanceSelection {
            instance_type: selection.gpu_type,
            // io.net has no spot tier — clusters are reserved for the duration.
            spot_capable: false,
            estimated_hourly_cost: Some(
                selection.estimated_hourly_cost * f64::from(selection.gpu_count),
            ),
        }
    }

    /// Detailed selection used by the adapter when launching a cluster.
    pub fn select(spec: &ResourceSpec) -> IoNetSelection {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        let memory_gb = spec.memory_gb;

        // Order matters: H100 wins for very high memory or 8-GPU jobs, then A100
        // tiers, then RTX_A6000 (48 GB workstation), RTX_4090 (24 GB), RTX_3090.
        let gpu_type = if (gpu_count >= 8 && memory_gb >= 768.0) || memory_gb >= 200.0 {
            "H100"
        } else if memory_gb >= 80.0 {
            "A100-80GB"
        } else if memory_gb >= 48.0 {
            "RTX_A6000"
        } else if memory_gb >= 40.0 {
            "A100-40GB"
        } else if memory_gb >= 24.0 {
            "RTX_4090"
        } else {
            "RTX_3090"
        };

        IoNetSelection {
            gpu_type: gpu_type.to_string(),
            gpu_count,
            estimated_hourly_cost: Self::estimate_hourly_cost(gpu_type),
        }
    }

    /// Per-GPU hourly cost estimates in USD as of 2026-04 published io.cloud pricing.
    /// Used for logging and cost ceiling enforcement; billing reconciles against the
    /// io.net invoice.
    pub fn estimate_hourly_cost(gpu_type: &str) -> f64 {
        match gpu_type {
            "H100" => 2.49,
            "A100-80GB" => 1.87,
            "A100-40GB" => 1.29,
            "L40S" => 1.19,
            "L40" => 0.99,
            "RTX_4090" => 0.74,
            "RTX_A6000" => 0.55,
            "RTX_3090" => 0.29,
            _ => 1.00,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(memory_gb: f32, gpu_count: Option<u32>) -> ResourceSpec {
        ResourceSpec {
            cpu: 4.0,
            memory_gb,
            storage_gb: 100.0,
            gpu_count,
            allow_spot: false,
            qos: Default::default(),
        }
    }

    #[test]
    fn small_spec_picks_rtx_3090() {
        let selection = IoNetInstanceMapper::select(&spec(8.0, Some(1)));
        assert_eq!(selection.gpu_type, "RTX_3090");
        assert_eq!(selection.gpu_count, 1);
    }

    #[test]
    fn medium_memory_picks_rtx_4090() {
        let selection = IoNetInstanceMapper::select(&spec(32.0, Some(1)));
        assert_eq!(selection.gpu_type, "RTX_4090");
    }

    #[test]
    fn workstation_class_picks_rtx_a6000() {
        let selection = IoNetInstanceMapper::select(&spec(48.0, Some(1)));
        assert_eq!(selection.gpu_type, "RTX_A6000");
    }

    #[test]
    fn big_memory_picks_h100() {
        let selection = IoNetInstanceMapper::select(&spec(256.0, Some(1)));
        assert_eq!(selection.gpu_type, "H100");
    }

    #[test]
    fn multi_gpu_cluster_picks_h100() {
        let selection = IoNetInstanceMapper::select(&spec(1024.0, Some(8)));
        assert_eq!(selection.gpu_type, "H100");
        assert_eq!(selection.gpu_count, 8);
    }

    #[test]
    fn map_aggregates_cost_across_gpus() {
        let selection = IoNetInstanceMapper::map(&spec(1024.0, Some(8)));
        assert_eq!(selection.instance_type, "H100");
        assert!(!selection.spot_capable);
        // 8 * 2.49 = 19.92
        let cost = selection.estimated_hourly_cost.unwrap();
        assert!((cost - 19.92).abs() < 0.001);
    }

    #[test]
    fn map_defaults_to_single_gpu_when_unspecified() {
        let selection = IoNetInstanceMapper::map(&spec(8.0, None));
        assert_eq!(selection.instance_type, "RTX_3090");
    }

    #[test]
    fn cost_lookup_falls_back_to_default() {
        let cost = IoNetInstanceMapper::estimate_hourly_cost("UNKNOWN_GPU");
        assert!(
            (cost - 1.00).abs() < f64::EPSILON,
            "expected 1.00, got {cost}"
        );
    }
}
