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
            estimated_hourly_cost: None,
        }
    }

    /// Build a Vast.ai search query with optional price/reliability ceilings.
    pub fn build_query(
        spec: &ResourceSpec,
        max_price_per_hour: Option<f64>,
        min_reliability: Option<f64>,
    ) -> serde_json::Value {
        let gpu_count = spec.gpu_count.unwrap_or(1).max(1);
        let skus = crate::providers::common::gpu_adapter::catalog_skus(spec, "vast-ai")
            .unwrap_or_default();

        let mut query = json!({
            "verified": { "eq": true },
            "rentable": { "eq": true },
            "num_gpus": { "eq": gpu_count as i64 },
            "cpu_cores_effective": { "gte": spec.cpu.ceil() as i64 },
            "cpu_ram": { "gte": (spec.memory_gb*1024.0).ceil() as i64 },
            "disk_space": { "gte": spec.storage_gb.max(50.0) as i64 },
            "gpu_name": if skus.len()==1{json!({"eq":skus[0]})}else{json!({"in":skus})},
            "allocated_storage":spec.storage_gb,
            "type":"on-demand",
            "rented":{"eq":false},
            "order": [["dph_total", "asc"]],
        });
        let minimum = spec.gpu_vram_gb.unwrap_or(0).max(
            spec.gpu_class
                .as_deref()
                .and_then(blueprint_core::gpu::find)
                .map(|class| class.vram_gb)
                .unwrap_or(0),
        );
        query["gpu_ram"] =
            json!({"gte":crate::providers::common::gpu_adapter::gpu_memory_mib(minimum)});
        if let Some(max) = max_price_per_hour {
            query["dph_total"] = json!({ "lte": max });
        }
        if let Some(min) = min_reliability {
            query["reliability2"] = json!({ "gte": min });
        }
        query
    }

    /// Provider name comes only from the SDK catalog, never host RAM.
    pub fn preferred_gpu(spec: &ResourceSpec) -> String {
        crate::providers::common::gpu_adapter::catalog_sku(spec, "vast-ai").unwrap_or_default()
    }
}
