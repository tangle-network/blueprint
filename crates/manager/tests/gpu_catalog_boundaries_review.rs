//! Independent public SDK request/registration boundary; no provider policy doubles.
use blueprint_client_tangle::resolve_execution_profile_from_profiling_data;
use blueprint_core::resources::{CloudProvider, ResourceSpec};
use blueprint_manager::rt::{GpuSchedulingPolicy, ResourceLimits};
use serde_json::json;

#[test]
fn class_specific_caps_survive_registration_manager_and_resource_wire() {
    for (class, count, vram) in [
        ("nvidia-gb300", 4, 288),
        ("nvidia-b300", 5, 288),
        ("nvidia-h200", 8, 141),
    ] {
        let metadata = |count, vram| {
            json!({"execution_profile":{"gpu":{
                "policy":"required", "class":class,"min_count":count,"min_vram_gb":vram
            }}})
            .to_string()
        };
        let profile = resolve_execution_profile_from_profiling_data(&metadata(count, vram))
            .unwrap()
            .unwrap();
        let limits = ResourceLimits::from(&profile.gpu);
        assert!(matches!(limits.gpu_policy, GpuSchedulingPolicy::Required));
        let request: ResourceSpec = (&limits).into();
        assert!(request.validate().is_ok());
        let wire = serde_json::to_value(request).unwrap();
        assert_eq!(wire["gpu_class"], class);
        assert_eq!(wire["gpu_count"], count);
        assert_eq!(wire["gpu_vram_gb"], vram);
        assert_eq!(
            wire["memory_gb"], 4.0,
            "host memory must remain independent"
        );
        let decoded: ResourceSpec = serde_json::from_value(wire).unwrap();
        assert!(decoded.validate().is_ok());
        assert!(resolve_execution_profile_from_profiling_data(&metadata(count, vram + 1)).is_err());
    }
    let gb300_five = r#"{"execution_profile":{"gpu":{"policy":"required","class":"nvidia-gb300","min_count":5}}}"#;
    assert!(resolve_execution_profile_from_profiling_data(gb300_five).is_err());
    let preferred = resolve_execution_profile_from_profiling_data(
        r#"{"execution_profile":{"gpu":{"policy":"preferred","class":"nvidia-b200"}}}"#,
    )
    .unwrap()
    .unwrap();
    let limits = ResourceLimits::from(&preferred.gpu);
    assert_eq!(limits.gpu_count, Some(1));
    assert!(matches!(limits.gpu_policy, GpuSchedulingPolicy::Preferred));
}

#[test]
fn pre_consolidation_provider_names_still_decode_to_the_shared_owner() {
    for (old, key) in [
        ("AWS", "aws"),
        ("GCP", "gcp"),
        ("LambdaLabs", "lambda-labs"),
        ("lambdalabs", "lambda-labs"),
        ("primeintellect", "prime-intellect"),
        ("vastai", "vast-ai"),
        ("DigitalOcean", "digitalocean"),
    ] {
        let provider: CloudProvider = serde_json::from_value(json!(old)).unwrap();
        assert_eq!(provider.catalog_key(), key);
        assert_eq!(serde_json::to_value(provider).unwrap(), key);
    }
}
