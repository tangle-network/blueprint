use blueprint_remote_providers::ResourceSpec;

fn request(class: &str, count: u32, vram: u32) -> ResourceSpec {
    serde_json::from_value(serde_json::json!({
        "cpu": 4, "memory_gb": 16, "storage_gb": 100, "allow_spot": false,
        "gpu_class": class, "gpu_count": count, "gpu_vram_gb": vram
    }))
    .unwrap()
}

#[test]
fn gpu_resources_reject_unknown_classes_and_impossible_vram() {
    assert!(request("nvidia-unknown", 1, 24).validate().is_err());
    assert!(request("nvidia-b200", 8, 181).validate().is_err());
    assert!(request("nvidia-b200", 9, 180).validate().is_err());
    assert!(request("nvidia-b200", 8, 180).validate().is_ok());
}

#[test]
fn gpu_resources_preserve_class_and_per_device_vram() {
    let encoded = serde_json::to_value(request("nvidia-b200", 8, 180)).unwrap();
    assert_eq!(encoded["gpu_class"], "nvidia-b200");
    assert_eq!(encoded["gpu_vram_gb"], 180);
}
