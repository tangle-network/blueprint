//! GPU hardware and provider SKU catalog shared by quoting and provisioning.

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// The published source catalog, also pinned by centralized GPU consumers.
pub const CATALOG_JSON: &str = include_str!("../gpu-catalog.json");

/// Hardware properties of one GPU, independent of host RAM or rental prices.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuClass {
    pub id: String,
    pub aliases: Vec<String>,
    pub vendor: String,
    pub generation: String,
    /// Decimal GB per GPU.
    pub vram_gb: u32,
    /// Decimal GB per second.
    pub memory_bandwidth_gbps: u32,
    pub fp8: bool,
    pub fp4: bool,
    pub interconnect: String,
    pub max_per_node: u32,
    /// Provider key → exact count (or `*`) → provider SKU.
    pub provider_skus: BTreeMap<String, BTreeMap<String, String>>,
}

impl GpuClass {
    /// Fixed node SKUs require an exact count; flexible SKUs accept any valid count.
    pub fn sku(&self, provider: &str, count: u32) -> Option<&str> {
        if count == 0 || count > self.max_per_node {
            return None;
        }
        let skus = self.provider_skus.get(provider)?;
        skus.get(&alloc::string::ToString::to_string(&count))
            .or_else(|| skus.get("*"))
            .map(String::as_str)
    }
}

/// Read the embedded catalog using alloc only, including in `no_std` builds.
pub fn catalog() -> Vec<GpuClass> {
    #[derive(Deserialize)]
    struct Catalog {
        gpus: Vec<GpuClass>,
    }
    serde_json::from_str::<Catalog>(CATALOG_JSON)
        .expect("published GPU catalog must be valid")
        .gpus
}

/// Look up a canonical class id. Unknown hardware is never guessed.
pub fn find(id: &str) -> Option<GpuClass> {
    catalog().into_iter().find(|gpu| gpu.id == id)
}
