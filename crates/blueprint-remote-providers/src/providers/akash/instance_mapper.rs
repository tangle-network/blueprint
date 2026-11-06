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
