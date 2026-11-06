use blueprint_client_tangle::blueprint_metadata::{
    GpuPolicy, inject_execution_profile, resolve_execution_profile_from_profiling_data,
};

#[test]
fn gpu_class_and_legacy_metadata_round_trip_through_public_parser() {
    let legacy =
        r#"{"execution_profile":{"gpu":{"policy":"required","min_count":2,"min_vram_gb":24}}}"#;
    let profile = resolve_execution_profile_from_profiling_data(legacy)
        .unwrap()
        .unwrap();
    assert_eq!(profile.gpu.policy, GpuPolicy::Required);
    assert!(profile.gpu.class.is_none());
    let current = r#"{"execution_profile":{"gpu":{"policy":"required","class":"nvidia-b200","min_count":8,"min_vram_gb":180}}}"#;
    let profile = resolve_execution_profile_from_profiling_data(current)
        .unwrap()
        .unwrap();
    let output = inject_execution_profile("", profile.clone());
    assert_eq!(
        resolve_execution_profile_from_profiling_data(&output).unwrap(),
        Some(profile)
    );
    for gpu in [
        r#"{"policy":"required","class":"nvidia-unknown","min_count":1}"#,
        r#"{"policy":"required","class":"nvidia-gb300","min_count":5}"#,
        r#"{"policy":"required","class":"nvidia-b200","min_count":8,"min_vram_gb":181}"#,
        r#"{"policy":"required","unexpected":1}"#,
    ] {
        assert!(
            resolve_execution_profile_from_profiling_data(&format!(
                r#"{{"execution_profile":{{"gpu":{gpu}}}}}"#
            ))
            .is_err()
        );
    }
}
