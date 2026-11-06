//! Cloud behavior through the shipped CLI process boundary.
#![cfg(feature = "remote-providers")]

use std::process::{Command, Output};

fn command(args: &[&str]) -> Output {
    let home = tempfile::tempdir().unwrap();
    Command::new(env!("CARGO_BIN_EXE_cargo-tangle"))
        .args(["tangle", "cloud"])
        .args(args)
        .env_clear()
        .env("HOME", home.path())
        .env("XDG_CONFIG_HOME", home.path())
        .env("PATH", "/usr/bin:/bin")
        .current_dir(home.path())
        .output()
        .unwrap()
}

#[test]
fn zero_duration_is_not_priced() {
    let duration = command(&["estimate", "--duration", "0h"]);
    assert!(
        !duration.status.success(),
        "zero duration was priced: {}",
        String::from_utf8_lossy(&duration.stdout)
    );
    assert!(
        duration.stdout.is_empty(),
        "invalid requests must not print estimates"
    );
}

#[test]
fn missing_deployment_is_not_fabricated() {
    let status = command(&["status", "missing-deployment"]);
    assert!(
        !status.status.success(),
        "missing deployment was shown as running"
    );
    assert!(!String::from_utf8_lossy(&status.stdout).contains("54.123.45.67"));
}

use serde_json::{Value, json};
use std::{
    process::Stdio,
    sync::{Arc, Mutex},
    time::Duration,
};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

async fn cloud_json(verb: &str, request: &Value, home: &std::path::Path) -> (bool, Value) {
    use tokio::io::AsyncWriteExt;
    let mut child = tokio::process::Command::new(env!("CARGO_BIN_EXE_cargo-tangle"))
        .args(["tangle", "cloud", verb, "--json"])
        .env_clear()
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home)
        .env("PATH", "/usr/bin:/bin")
        .current_dir(home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    stdin
        .write_all(&serde_json::to_vec(request).unwrap())
        .await
        .unwrap();
    drop(stdin);
    let output = tokio::time::timeout(Duration::from_secs(30), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    let mut records = serde_json::Deserializer::from_slice(&output.stdout).into_iter::<Value>();
    let value = records
        .next()
        .expect("one JSON stdout record")
        .unwrap_or_else(|error| {
            panic!(
                "invalid JSON: {error}; stderr: {}",
                String::from_utf8_lossy(&output.stderr)
            )
        });
    assert!(records.next().is_none(), "extra stdout record");
    for secret in [
        "account-secret-7359",
        "worker-token-2841",
        "worker-cert-3209",
        "worker-key-6083",
    ] {
        assert!(!String::from_utf8_lossy(&output.stdout).contains(secret));
        assert!(!String::from_utf8_lossy(&output.stderr).contains(secret));
    }
    assert_eq!(value["schema_version"], 1);
    (output.status.success(), value)
}

#[derive(Default)]
struct ProviderState {
    pod: Option<Value>,
    creates: usize,
    deletes: usize,
    payload: Option<Value>,
    unknown_price: bool,
}

#[derive(Clone)]
struct RunpodHttp(Arc<Mutex<ProviderState>>);

impl Respond for RunpodHttp {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        assert_eq!(
            request.headers.get("authorization").unwrap(),
            "Bearer account-secret-7359"
        );
        let mut state = self.0.lock().unwrap();
        match (request.method.as_str(), request.url.path()) {
            ("POST", "/graphql") => ResponseTemplate::new(200).set_body_json(json!({"data":{"gpuTypes":[{"id":"NVIDIA B200","lowestPrice":{"stockStatus":"High","uninterruptablePrice":if state.unknown_price { None } else { Some(1.25) }}}]}})),
            ("GET", "/pods") => {
                let mut pods = vec![json!({"id":"foreign","name":"unmanaged","desiredStatus":"RUNNING"})];
                pods.extend(state.pod.iter().cloned());
                ResponseTemplate::new(200).set_body_json(pods)
            }
            ("POST", "/pods") => {
                let payload: Value = request.body_json().unwrap();
                state.creates += 1;
                state.payload = Some(payload.clone());
                state.pod = Some(json!({"id":"owned-pod","name":payload["name"],"gpuTypeId":"NVIDIA B200","gpuCount":2,"imageName":payload["imageName"],"env":payload["env"],"dataCenterId":"EU","desiredStatus":"RUNNING","ports":["8787/http"],"costPerHr":2.5,"lastStartedAt":"2026-11-06T00:00:00Z"}));
                ResponseTemplate::new(200).set_body_json(json!({"id":"owned-pod"}))
            }
            ("GET", "/pods/owned-pod") => match &state.pod {
                Some(pod) => ResponseTemplate::new(200).set_body_json(pod),
                None => ResponseTemplate::new(404),
            },
            ("DELETE", "/pods/owned-pod") => {
                state.deletes += 1;
                state.pod = None;
                ResponseTemplate::new(204)
            }
            _ => panic!("unexpected provider HTTP boundary: {} {}", request.method, request.url),
        }
    }
}

#[tokio::test]
async fn catalog_and_typed_errors_cross_the_public_process_boundary() {
    let home = tempfile::tempdir().unwrap();
    let (ok, catalog) = cloud_json("catalog", &json!({}), home.path()).await;
    assert!(ok);
    let b200 = catalog["data"]["gpus"]
        .as_array()
        .unwrap()
        .iter()
        .find(|gpu| gpu["id"] == "nvidia-b200")
        .unwrap();
    assert_eq!(b200["vramGb"], 180);
    assert_eq!(b200["maxPerNode"], 8);
    assert!(
        catalog["data"]["providers"]
            .as_array()
            .unwrap()
            .iter()
            .any(|provider| provider["id"] == "runpod")
    );
    let (ok, error) = cloud_json(
        "quote",
        &json!({"schema_version":2,"credentials":{"runpod":{"api_key":"account-secret-7359"}}}),
        home.path(),
    )
    .await;
    assert!(!ok);
    assert_eq!(error["error"]["code"], "INVALID_REQUEST");
    assert_eq!(error["error"]["retryable"], false);
    let (ok, error) = cloud_json("quote", &json!({"schema_version":1,
        "spec":{"cpu":4,"memory_gb":16,"storage_gb":40,"allow_spot":false,"unsupported_workload_field":true}}), home.path()).await;
    assert!(!ok);
    assert_eq!(error["error"]["code"], "INVALID_REQUEST");
}

#[tokio::test]
async fn cli_quotes_creates_adopts_observes_and_retires_through_one_sdk_adapter() {
    let server = MockServer::start().await;
    let state = Arc::new(Mutex::new(ProviderState::default()));
    Mock::given(|_: &Request| true)
        .respond_with(RunpodHttp(state.clone()))
        .mount(&server)
        .await;
    let home = tempfile::tempdir().unwrap();
    let mut request = json!({"schema_version":1,"provider":"runpod",
        "credentials":{"runpod":{"api_key":"account-secret-7359","endpoint":server.uri(),"region":"EU"}},
        "ownership":{"namespace":"central-gpu","lease_id":"00000000-0000-4000-8000-000000000001"},
        "spec":{"cpu":4,"memory_gb":16,"storage_gb":40,"gpu_count":2,"gpu_class":"nvidia-b200","gpu_vram_gb":180,
            "allow_spot":false,"image":"worker:caller-entrypoint","env":{"TANGLE_GPU_WORKER_TOKEN":"worker-token-2841"},
            "ports":[8787],"endpoint_port":8787,"lease_duration_secs":300,"max_hourly_cost_usd":5}});
    let (ok, quote) = cloud_json("quote", &request, home.path()).await;
    assert!(ok, "{quote}");
    assert_eq!(quote["data"]["hourly_cost_usd"], 2.5);
    assert_eq!(quote["data"]["instance_type"], "NVIDIA B200");
    for _ in 0..2 {
        let (ok, allocation) = cloud_json("create", &request, home.path()).await;
        assert!(ok, "{allocation}");
        assert_eq!(allocation["data"]["provider_id"], "owned-pod");
    }
    {
        let state = state.lock().unwrap();
        assert_eq!(
            state.creates, 1,
            "idempotent identity created two provider resources"
        );
        let payload = state.payload.as_ref().unwrap();
        assert_eq!(payload["gpuCount"], 2);
        assert_eq!(payload["imageName"], "worker:caller-entrypoint");
        assert_eq!(
            payload["env"]["TANGLE_GPU_WORKER_TOKEN"],
            "worker-token-2841"
        );
        assert!(payload.get("dockerEntrypoint").is_none());
        assert!(payload.get("dockerStartCmd").is_none());
    }
    request.as_object_mut().unwrap().remove("spec");
    request["provider_id"] = json!("owned-pod");
    let (ok, status) = cloud_json("status", &request, home.path()).await;
    assert!(ok, "{status}");
    assert_eq!(status["data"]["status"], "ready");
    assert_eq!(
        status["data"]["worker_url"],
        "https://owned-pod-8787.proxy.runpod.net"
    );
    request["ownership"]
        .as_object_mut()
        .unwrap()
        .remove("lease_id");
    request.as_object_mut().unwrap().remove("provider_id");
    let (ok, inventory) = cloud_json("list", &request, home.path()).await;
    assert!(ok, "{inventory}");
    assert_eq!(inventory["data"]["resources"].as_array().unwrap().len(), 1);
    request["ownership"]["lease_id"] = json!("00000000-0000-4000-8000-000000000001");
    let (ok, retired) = cloud_json("destroy", &request, home.path()).await;
    assert!(ok, "{retired}");
    assert_eq!(retired["data"]["destroyed"], true);
    assert_eq!(state.lock().unwrap().deletes, 1);
    assert!(state.lock().unwrap().pod.is_none());
    request["ownership"]
        .as_object_mut()
        .unwrap()
        .remove("lease_id");
    let (ok, inventory) = cloud_json("list", &request, home.path()).await;
    assert!(ok, "{inventory}");
    assert!(
        inventory["data"]["resources"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn unknown_prices_are_unselectable_and_provider_error_bodies_are_private() {
    let server = MockServer::start().await;
    let state = Arc::new(Mutex::new(ProviderState {
        unknown_price: true,
        ..Default::default()
    }));
    Mock::given(|_: &Request| true)
        .respond_with(RunpodHttp(state.clone()))
        .mount(&server)
        .await;
    let home = tempfile::tempdir().unwrap();
    let request = json!({"schema_version":1,"provider":"runpod",
        "credentials":{"runpod":{"api_key":"account-secret-7359","endpoint":server.uri()}},
        "ownership":{"namespace":"central-gpu","lease_id":"unknown-price"},
        "spec":{"cpu":4,"memory_gb":16,"storage_gb":40,"gpu_count":2,"gpu_class":"nvidia-b200","allow_spot":false,"lease_duration_secs":300}});
    let (ok, quote) = cloud_json("quote", &request, home.path()).await;
    assert!(!ok);
    assert_eq!(quote["error"]["code"], "NO_PRICE");
    let (ok, _) = cloud_json("create", &request, home.path()).await;
    assert!(!ok);
    assert_eq!(state.lock().unwrap().creates, 0);
    server.reset().await;
    Mock::given(|_: &Request| true)
        .respond_with(
            ResponseTemplate::new(401).set_body_string("account-secret-7359 worker-token-2841"),
        )
        .mount(&server)
        .await;
    let request = json!({"schema_version":1,"provider":"runpod",
        "credentials":{"runpod":{"api_key":"account-secret-7359","endpoint":server.uri()}},
        "ownership":{"namespace":"central-gpu"}});
    let (ok, error) = cloud_json("list", &request, home.path()).await;
    assert!(!ok);
    assert_eq!(error["ok"], false);
    assert_eq!(error["error"]["code"], "AUTH_REJECTED");
    assert_eq!(error["error"]["retryable"], false);
}
