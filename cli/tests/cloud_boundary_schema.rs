//! The portable contract comes from the same public process that rents GPUs.
#![cfg(feature = "remote-providers")]

use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Output, Stdio};

fn invoke(verb: &str, request: &Value) -> Output {
    let cache = tempfile::tempdir().unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_cargo-tangle"))
        .args(["tangle", "cloud", verb, "--json"])
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("XDG_CACHE_HOME", cache.path())
        .env("RUNPOD_API_KEY", "inherited-credential-must-not-escape")
        .current_dir(cache.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(request).unwrap())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    for bytes in [&output.stdout, &output.stderr] {
        assert!(!String::from_utf8_lossy(bytes).contains("inherited-credential-must-not-escape"));
    }
    output
}

fn visit(value: &Value, predicate: &impl Fn(&Value) -> bool) -> bool {
    predicate(value)
        || match value {
            Value::Object(object) => object.values().any(|value| visit(value, predicate)),
            Value::Array(array) => array.iter().any(|value| visit(value, predicate)),
            _ => false,
        }
}

#[test]
fn catalog_emits_a_deterministic_complete_boundary_without_user_configuration() {
    let first = invoke("catalog", &json!({}));
    assert!(
        first.status.success(),
        "catalog process failed: {}",
        String::from_utf8_lossy(&first.stderr)
    );
    let catalog: Value = serde_json::from_slice(&first.stdout).expect("one JSON record");
    assert_eq!(catalog["schema_version"], 1);
    assert_eq!(catalog["ok"], true);
    let data = &catalog["data"];
    assert!(data["gpus"].as_array().is_some_and(|gpus| !gpus.is_empty()));
    assert!(
        data["providers"]
            .as_array()
            .is_some_and(|providers| !providers.is_empty())
    );
    let schema = &data["boundary_schema"];
    assert!(schema["$schema"].as_str().is_some(), "missing JSON Schema");
    for field in [
        "schema_version",
        "credentials",
        "spec",
        "candidate",
        "provider_id",
        "gpu_class",
        "gpu_count",
        "gpu_vram_gb",
        "weights",
        "arms",
        "worker_url",
        "endpoint_url",
        "quotes",
    ] {
        assert!(
            visit(schema, &|node| node["properties"].get(field).is_some()),
            "generated boundary omits {field}"
        );
    }
    let required_codes = [
        "no_stock",
        "sku_unknown",
        "unsupported",
        "still_present",
        "preempted",
        "quota",
    ];
    assert!(
        visit(schema, &|node| {
            node["enum"].as_array().is_some_and(|values| {
                required_codes
                    .iter()
                    .all(|code| values.iter().any(|value| value == code))
            })
        }),
        "provider failures must have one closed generated code enum"
    );
    let second = invoke("catalog", &json!({"operation":"catalog"}));
    assert!(second.status.success());
    assert_eq!(
        first.stdout, second.stdout,
        "catalog depends on process state"
    );
}

#[test]
fn caller_credentials_work_without_process_global_tls_setup() {
    let output = invoke(
        "quote",
        &json!({
            "schema_version":1,
            "credentials":[{"provider":"runpod","api_key":"caller-canary-secret","endpoint":"http://127.0.0.1:0"}],
            "spec":{"cpu":1,"memory_gb":2,"storage_gb":8,"gpu_count":1,"gpu_class":"nvidia-h200","gpu_vram_gb":140,"allow_spot":false}
        }),
    );
    assert!(
        output.status.success(),
        "fresh factory process: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let response: Value = serde_json::from_slice(&output.stdout).expect("one quote envelope");
    assert_eq!(response["ok"], true);
    assert!(response["data"]["quotes"].as_array().unwrap().is_empty());
    assert!(!response["data"]["errors"].as_array().unwrap().is_empty());
    for bytes in [&output.stdout, &output.stderr] {
        assert!(!String::from_utf8_lossy(bytes).contains("caller-canary-secret"));
    }
}

#[test]
fn invalid_requests_use_the_same_generated_error_envelope() {
    let output = invoke("quote", &json!({"schema_version": 2}));
    assert!(!output.status.success());
    let error: Value = serde_json::from_slice(&output.stdout).expect("one error JSON record");
    assert_eq!(error["schema_version"], 1);
    assert_eq!(error["ok"], false);
    assert_eq!(error["error"]["code"], "invalid_request");
    assert_eq!(error["error"]["retryable"], false);
}
