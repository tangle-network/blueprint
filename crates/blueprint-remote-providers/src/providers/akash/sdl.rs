//! Single-service Akash SDL. GPU identity comes from the shared catalog.
use crate::ResourceSpec;

/// Build a deployment with an explicit micro-USD-per-block bid ceiling.
pub(crate) fn build_sdl_manifest(
    image: &str,
    model: &str,
    spec: &ResourceSpec,
    ports: &[u16],
    max_bid_uact: u64,
) -> String {
    let vram = spec
        .gpu_vram_gb
        .or_else(|| {
            spec.gpu_class
                .as_deref()
                .and_then(blueprint_core::gpu::find)
                .map(|g| g.vram_gb)
        })
        .unwrap_or(0);
    let expose: Vec<_> = ports
        .iter()
        .map(|port| serde_json::json!({"port":port,"as":port,"proto":"tcp","to":[{"global":true}]}))
        .collect();
    let resources = serde_json::json!({
        "cpu":{"units":spec.cpu},"memory":{"size":format!("{}Mi",(spec.memory_gb*1024.0).ceil() as u64)},
        "storage":[{"size":format!("{}Mi",(spec.storage_gb*1024.0).ceil() as u64)}],
        "gpu":{"units":spec.gpu_count.unwrap_or(0),"attributes":{"vendor":{"nvidia":[{"model":model,"ram":format!("{vram}Gi")}]}}}
    });
    let document = serde_json::json!({"version":"2.0",
        "services":{"blueprint":{"image":image,"env":[format!("PUBLIC_KEY={}",spec.ssh_public_key.as_deref().unwrap_or(""))],"expose":expose}},
        "profiles":{"compute":{"blueprint":{"resources":resources}},
            "placement":{"akash":{"pricing":{"blueprint":{"denom":"uact","amount":max_bid_uact}}}}},
        "deployment":{"blueprint":{"akash":{"profile":"blueprint","count":1}}}
    });
    serde_yaml::to_string(&document).expect("JSON SDL serializes to YAML")
}
