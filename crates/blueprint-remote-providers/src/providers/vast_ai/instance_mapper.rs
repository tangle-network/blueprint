//! Maps `ResourceSpec` to a Vast.ai search query string.
//!
//! Vast.ai's `search` endpoint accepts a JSON query body with keys matching
//! offer fields: `gpu_name`, `num_gpus`, `cpu_cores`, `ram_gb`, `disk_space`,
//! `dph_total` (dollars per hour), `reliability2` (reliability score).
//!
//! Rather than a single "instance type" string, we emit a JSON query that the
//! adapter submits to the search endpoint. The `instance_type` field carries
//! the serialized query to pass it through `CloudProviderAdapter::provision_instance`.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;
use serde_json::json;

pub struct VastAiInstanceMapper;

impl VastAiInstanceMapper {
    /// Build a JSON search query for Vast.ai that will match offers matching
    /// the spec. Returns the query as a serialized string inside
    /// `InstanceSelection::instance_type`.
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let query = Self::build_query(spec, None, None);
        let serialized = serde_json::to_string(&query).unwrap_or_default();
        InstanceSelection {
            instance_type: serialized,
            spot_capable: true,
            estimated_hourly_cost: Some(Self::estimate_hourly_cost(spec)),
        }
    }

    /// Build a Vast.ai search query with optional price/reliability ceilings.
    pub fn build_query(
        spec: &ResourceSpec,
        max_price_per_hour: Option<f64>,
        min_reliability: Option<f64>,
    ) -> serde_json::Value {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        let preferred_gpu = Self::preferred_gpu(spec);

        let mut query = json!({
            "verified": { "eq": true },
            "rentable": { "eq": true },
            "num_gpus": { "eq": gpu_count as i64 },
            "cpu_cores": { "gte": spec.cpu.max(2.0) as i64 },
            "ram_gb": { "gte": spec.memory_gb.max(16.0) as i64 },
            "disk_space": { "gte": spec.storage_gb.max(50.0) as i64 },
            "gpu_name": { "eq": preferred_gpu },
            "order": [["dph_total", "asc"]],
        });
        if let Some(max) = max_price_per_hour {
            query["dph_total"] = json!({ "lte": max });
        }
        if let Some(min) = min_reliability {
            query["reliability2"] = json!({ "gte": min });
        }
        query
    }

    /// Pick the preferred GPU model based on the spec's memory hint.
    pub fn preferred_gpu(spec: &ResourceSpec) -> &'static str {
        if spec.memory_gb >= 200.0 {
            "H100 SXM"
        } else if spec.memory_gb >= 80.0 {
            "A100 SXM4"
        } else if spec.memory_gb >= 48.0 {
            "RTX 6000Ada"
        } else if spec.memory_gb >= 24.0 {
            "RTX 4090"
        } else {
            "RTX 3090"
        }
    }

    /// Rough hourly cost estimate (spot market floor prices as of 2026-04).
    pub fn estimate_hourly_cost(spec: &ResourceSpec) -> f64 {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1) as f64;
        let per_gpu = match Self::preferred_gpu(spec) {
            "H100 SXM" => 1.85,
            "A100 SXM4" => 0.99,
            "RTX 6000Ada" => 0.65,
            "RTX 4090" => 0.32,
            "RTX 3090" => 0.18,
            _ => 0.40,
        };
        per_gpu * gpu_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(mem: f32, gpus: u32) -> ResourceSpec {
        ResourceSpec {
            cpu: 4.0,
            memory_gb: mem,
            storage_gb: 100.0,
            gpu_count: Some(gpus),
            allow_spot: true,
            qos: Default::default(),
        }
    }

    #[test]
    fn prefers_rtx3090_for_tiny_memory() {
        assert_eq!(
            VastAiInstanceMapper::preferred_gpu(&spec(8.0, 1)),
            "RTX 3090"
        );
    }

    #[test]
    fn prefers_h100_for_big_memory() {
        assert_eq!(
            VastAiInstanceMapper::preferred_gpu(&spec(256.0, 1)),
            "H100 SXM"
        );
    }

    #[test]
    fn query_contains_price_and_reliability_ceilings() {
        let query = VastAiInstanceMapper::build_query(&spec(32.0, 2), Some(1.5), Some(0.95));
        assert_eq!(query["num_gpus"]["eq"].as_i64(), Some(2));
        assert!(query["dph_total"]["lte"].as_f64().is_some());
        assert!(query["reliability2"]["gte"].as_f64().is_some());
    }

    #[test]
    fn map_serializes_to_instance_type_string() {
        let selection = VastAiInstanceMapper::map(&spec(32.0, 1));
        assert!(!selection.instance_type.is_empty());
        assert!(selection.spot_capable);
        let parsed: serde_json::Value = serde_json::from_str(&selection.instance_type).unwrap();
        assert!(parsed["num_gpus"].is_object());
    }

    #[test]
    fn cost_scales_with_gpu_count() {
        let one = VastAiInstanceMapper::estimate_hourly_cost(&spec(32.0, 1));
        let two = VastAiInstanceMapper::estimate_hourly_cost(&spec(32.0, 2));
        assert!((two - 2.0 * one).abs() < f64::EPSILON);
    }
}
