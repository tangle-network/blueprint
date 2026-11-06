use blueprint_remote_providers::ResourceSpec;
use serde_json::json;

#[test]
fn public_resource_requests_preserve_gpu_identity_and_enforce_catalog_limits() {
    let request = |class: &str, count: u32, vram: u32| {
        serde_json::from_value::<ResourceSpec>(json!({
            "cpu": 2.0, "memory_gb": 16.0, "storage_gb": 20.0,
            "allow_spot": false, "gpu_class": class,
            "gpu_count": count, "gpu_vram_gb": vram
        }))
        .unwrap()
    };
    let valid = request("nvidia-b200", 8, 180);
    assert!(valid.validate().is_ok());
    let wire = serde_json::to_value(valid).unwrap();
    assert_eq!(wire["gpu_class"], "nvidia-b200");
    assert_eq!(wire["gpu_vram_gb"], 180);
    for invalid in [
        request("nvidia-gb300", 5, 288),
        request("nvidia-b200", 8, 181),
        request("nvidia-unknown", 1, 1),
        request("nvidia-b200", 0, 180),
    ] {
        assert!(
            invalid.validate().is_err(),
            "invalid GPU request was accepted"
        );
    }
}
