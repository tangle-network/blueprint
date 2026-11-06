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

#[test]
fn weight_identity_and_alternative_arms_cross_the_same_public_request_boundary() {
    let input = json!({
        "cpu": 2.0, "memory_gb": 16.0, "storage_gb": 100.0,
        "allowSpot": true, "maxHourlyUsd": 10.0,
        "weights": {"manifestSha":"a".repeat(64),"sizeBytes":1_000_000},
        "arms": [{"family":"example","arm":"fp4","gpu_class":"nvidia-b200", "gpu_count":8,"tp":8,"replicas":1}]
    });
    let spec: ResourceSpec = serde_json::from_value(input.clone()).unwrap();
    assert!(spec.validate().is_ok());
    let wire = serde_json::to_value(spec).unwrap();
    assert_eq!(wire["weights"], input["weights"]);
    assert_eq!(wire["arms"][0]["gpu_class"], "nvidia-b200");
    assert_eq!(wire["max_hourly_cost_usd"], 10.0);
    for (pointer, value) in [
        ("/weights/manifestSha", json!("unknown")),
        ("/arms/0/gpu_class", json!("NVIDIA B200")),
        ("/arms/0/gpu_count", json!(9)),
        ("/arms/0/tp", json!(0)),
        ("/arms/0/replicas", json!(2)),
    ] {
        let mut invalid = input.clone();
        *invalid.pointer_mut(pointer).unwrap() = value;
        let spec: ResourceSpec = serde_json::from_value(invalid).unwrap();
        assert!(spec.validate().is_err());
    }
}
