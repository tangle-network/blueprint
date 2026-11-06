//! Maps `ResourceSpec` to Prime Intellect GPU types.
//!
//! Catalog source: <https://docs.primeintellect.ai/api-reference> and the Prime
//! Intellect marketplace listings. Because Prime Intellect aggregates multiple
//! sub-providers, the price is a *typical* number — actual billing reflects the
//! winning sub-provider's rate at provisioning time.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct PrimeIntellectInstanceMapper;

impl PrimeIntellectInstanceMapper {
    /// Select the cheapest Prime Intellect GPU type that satisfies the spec.
    ///
    /// Single-GPU selection prefers RTX-class for sub-50GB workloads, A6000 for
    /// 48-80GB, A100 for 80-160GB, and H100/H200 above that. Multi-GPU configs
    /// always pick high-end SKUs because aggregator pricing on commodity GPUs
    /// is rarely competitive at scale.
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        let gpu_type = match gpu_count {
            1 => select_single_gpu(spec),
            2..=4 => {
                if spec.memory_gb >= 600.0 {
                    "H200"
                } else if spec.memory_gb >= 300.0 {
                    "H100-80GB"
                } else {
                    "A100-80GB"
                }
            }
            _ => "H100-80GB",
        };

        InstanceSelection {
            instance_type: gpu_type.to_string(),
            // Prime Intellect aggregates spot inventory from sub-providers but does
            // not expose a separate spot tier on the unified API surface.
            spot_capable: false,
            estimated_hourly_cost: Some(Self::estimate_hourly_cost(gpu_type) * gpu_count as f64),
        }
    }

    /// Hourly cost estimates in USD as of 2026-04 marketplace pricing. These are
    /// per-GPU; the mapper multiplies by `gpu_count` for the total estimate.
    pub fn estimate_hourly_cost(gpu_type: &str) -> f64 {
        match gpu_type {
            "H200" => 3.25,
            "H100-80GB" => 2.35,
            "A100-80GB" => 1.65,
            "A100-40GB" => 1.25,
            "L40S" => 1.29,
            "A6000" => 0.79,
            "RTX_4090" => 0.39,
            "RTX_3090" => 0.24,
            _ => 1.50,
        }
    }
}

/// Pick the best single-GPU SKU based on memory hints from the spec.
fn select_single_gpu(spec: &ResourceSpec) -> &'static str {
    if spec.memory_gb >= 400.0 {
        "H200"
    } else if spec.memory_gb >= 200.0 {
        "H100-80GB"
    } else if spec.memory_gb >= 100.0 {
        "A100-80GB"
    } else if spec.memory_gb >= 60.0 {
        "A100-40GB"
    } else if spec.memory_gb >= 40.0 {
        "L40S"
    } else if spec.memory_gb >= 24.0 {
        "A6000"
    } else if spec.memory_gb >= 16.0 {
        "RTX_4090"
    } else {
        "RTX_3090"
    }
}
