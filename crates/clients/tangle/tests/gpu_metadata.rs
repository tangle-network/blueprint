use blueprint_client_tangle::blueprint_metadata::{
    inject_execution_profile, resolve_execution_profile_from_profiling_data,
};

#[test]
fn registration_metadata_preserves_class_and_legacy_gpu_requirements() {
    for gpu in [
        r#"{"policy":"required","min_count":2,"min_vram_gb":24}"#,
        r#"{"policy":"required","class":"nvidia-b200","min_count":8,"min_vram_gb":180}"#,
    ] {
        let input = format!(r#"{{"execution_profile":{{"gpu":{gpu}}}}}"#);
        let profile = resolve_execution_profile_from_profiling_data(&input)
            .expect("valid GPU registration metadata must be accepted")
            .unwrap();
        let output = inject_execution_profile("", profile);
        let output: serde_json::Value = serde_json::from_str(&output).unwrap();
        let expected: serde_json::Value = serde_json::from_str(gpu).unwrap();
        for (key, value) in expected.as_object().unwrap() {
            assert_eq!(&output["execution_profile"]["gpu"][key], value);
        }
    }
    for gpu in [
        r#"{"policy":"required","class":"nvidia-unknown","min_count":1}"#,
        r#"{"policy":"required","class":"nvidia-gb300","min_count":5}"#,
        r#"{"policy":"required","class":"nvidia-b200","min_count":8,"min_vram_gb":181}"#,
        r#"{"policy":"required","unexpected":1}"#,
    ] {
        let input = format!(r#"{{"execution_profile":{{"gpu":{gpu}}}}}"#);
        assert!(resolve_execution_profile_from_profiling_data(&input).is_err());
    }
}
