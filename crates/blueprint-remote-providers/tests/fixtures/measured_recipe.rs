//! Private recipe canaries and synthetic counters shared by real HTTP/process proofs.
use blueprint_remote_providers::ResourceSpec;
use serde_json::json;

pub(crate) const PRIVATE_ENDPOINT: &str = "/PRIVATE_ENDPOINT_CANARY";
pub(crate) const CANARIES: [&str; 4] = [
    "PRIVATE_MODEL",
    "PRIVATE_REVISION",
    "PRIVATE_IMAGE",
    "PRIVATE_URI",
];

pub(crate) fn measured_spec(namespace: &str, lease: &str) -> ResourceSpec {
    serde_json::from_value(json!({
        "cpu":2,"memory_gb":4,"storage_gb":20,"allow_spot":false,
        "image":"registry.example.invalid/PRIVATE_IMAGE:fixture","env":{"FIXTURE_POLICY":"immutable-public"},
        "region":"US-TEST-1","ports":[8000],"endpoint_port":8000,"volume_gb":37,
        "lease_duration_secs":3600,"max_hourly_cost_usd":10,
        "ssh_public_key":"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMo32AY0vFK7g5FBIBcyPdxaxSEM5rEc0kEzMVveA9b+",
        "ownership":{"namespace":namespace,"lease_id":lease},
        "weights":{"manifestSha":"aa".repeat(32),"sizeBytes":5},
        "arms":([("nvidia-h200","h200","NVIDIA H200",1000, "11"), ("nvidia-b200","b200","NVIDIA B200",100,"22")].map(|(class,arm,sku,tokens,digest)| {
            let row = |selected: bool| json!({
                "family":"fixture-family","arm":arm,"gpu_class":class,"tp":2,"replicas":1,"allocated_gpu_count":2,
                "provider":"runpod","provider_sku":sku,"region":"US-TEST-1","concurrency":8,
                "model":format!("PRIVATE_MODEL-{arm}-{selected}"),"revision":format!("PRIVATE_REVISION-{arm}-{selected}"),
                "weights_manifest_sha256":"aa".repeat(32),"image":"registry.example.invalid/PRIVATE_IMAGE:fixture",
                "measured_at":if selected {"2026-11-07T00:00:00Z"} else {"2026-11-06T00:00:00Z"},
                "input_tokens":6000,"output_tokens":tokens,"elapsed_ms":1000,
                "raw_result_uri":format!("https://results.example.invalid/PRIVATE_URI-{arm}-{selected}?signature=PRIVATE_URI-signed"),
                "raw_result_sha256":if selected {digest.repeat(32)} else {"33".repeat(32)}
            });
            json!({"family":"fixture-family","arm":arm,"gpu_class":class,"gpu_count":2,"tp":2,"replicas":1,
                "measurements":[row(false),row(true)]})
        }))
    })).unwrap()
}

/// Requests must rehydrate input-only env instead of serializing a recipe as state.
pub(crate) fn recipe_value(spec: &ResourceSpec) -> serde_json::Value {
    let mut value = serde_json::to_value(spec).unwrap();
    value["env"] = serde_json::to_value(&spec.env).unwrap();
    value
}
