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
    /// Match a complete hardware name, alias or SKU without guessing substrings.
    pub fn matches_name(&self, name: &str) -> bool {
        let normalize = |name: &str| -> String {
            name.to_ascii_lowercase()
                .trim_start_matches("nvidia")
                .chars()
                .filter(char::is_ascii_alphanumeric)
                .collect()
        };
        let key = normalize(name);
        self.aliases
            .iter()
            .chain(core::iter::once(&self.id))
            .chain(self.provider_skus.values().flat_map(|skus| skus.values()))
            .any(|name| normalize(name) == key)
    }

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

/// One measured local device. Memory is the driver's usable total in MiB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDevice {
    pub name: String,
    pub memory_mib: u32,
    pub class: Option<String>,
}

impl GpuDevice {
    /// Decimal GB, matching the catalog's hardware capacity unit.
    pub fn vram_gb(&self) -> u32 {
        (u64::from(self.memory_mib) * 1_048_576 / 1_000_000_000) as u32
    }
}

/// Detect actual operator GPUs through the NVIDIA process boundary.
#[cfg(feature = "std")]
pub fn detect() -> Result<Vec<GpuDevice>, String> {
    use alloc::{format, string::ToString};
    let output = std::process::Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,memory.total",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .map_err(|error| format!("GPU detection failed: {error}"))?;
    if !output.status.success() {
        return Err("nvidia-smi failed".into());
    }
    let csv = String::from_utf8(output.stdout).map_err(|_| "Invalid GPU detector output")?;
    let classes = catalog();
    csv.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (name, memory) = line.rsplit_once(',').ok_or("Invalid nvidia-smi row")?;
            let mut device = GpuDevice {
                name: name.trim().into(),
                memory_mib: memory
                    .trim()
                    .parse()
                    .map_err(|_| "Invalid GPU memory total")?,
                class: None,
            };
            device.class = classes
                .iter()
                .filter(|gpu| gpu.vram_gb <= device.vram_gb())
                .filter(|gpu| gpu.matches_name(&device.name))
                .max_by_key(|gpu| gpu.vram_gb)
                .map(|gpu| gpu.id.to_string());
            Ok(device)
        })
        .collect()
}
