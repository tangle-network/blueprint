//! Independent cross-process ownership proof against a provider HTTP peer.
#![cfg(feature = "remote-providers")]
use serde_json::{Value, json};
use std::{
    collections::HashSet,
    process::Stdio,
    sync::{Arc, Mutex},
    time::Duration,
};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

#[derive(Default)]
struct State {
    pods: Vec<Value>,
    creates: usize,
    empty_clients: HashSet<String>,
}
#[derive(Clone)]
struct Account(Arc<Mutex<State>>);
impl Respond for Account {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        let mut state = self.0.lock().unwrap();
        let (client, path) = request
            .url
            .path()
            .trim_start_matches('/')
            .split_once('/')
            .unwrap();
        match (request.method.as_str(),path) {
            ("GET","pods") => {
                let response=ResponseTemplate::new(200).set_body_json(json!(state.pods));
                if state.pods.is_empty() {
                    // Bounded empty-list barrier exposes racing lookup-before-create calls.
                    let first=state.empty_clients.insert(client.into());
                    response.set_delay(if first {Duration::from_secs(2)} else {Duration::from_millis(300)})
                } else {response}
            }
            ("POST","graphql")=>ResponseTemplate::new(200).set_body_json(json!({"data":{"gpuTypes":[
                {"id":"NVIDIA B200","lowestPrice":{"stockStatus":"High","uninterruptablePrice":1}}
            ]}})),
            ("POST","pods")=>{
                let input:Value=request.body_json().unwrap();state.creates+=1;
                let id=format!("native-{}",state.creates);
                let mut pod=input.clone();
                pod["id"]=json!(id);pod["gpuTypeId"]=json!("NVIDIA B200");pod["gpuCount"]=json!(1);
                pod["dataCenterId"]=json!("EU");pod["desiredStatus"]=json!("RUNNING");
                pod["costPerHr"]=json!(1);pod["lastStartedAt"]=json!("2026-11-06T00:00:00Z");
                state.pods.push(pod);
                ResponseTemplate::new(201).set_body_json(json!({"id":id}))
            }
            ("GET",path) if path.starts_with("pods/")=>{
                let id=path.strip_prefix("pods/").unwrap();
                match state.pods.iter().find(|p|p["id"]==id) {
                    Some(pod)=>ResponseTemplate::new(200).set_body_json(pod),None=>ResponseTemplate::new(404),
                }
            }
            _=>ResponseTemplate::new(500),
        }
    }
}

async fn create(home: &std::path::Path, endpoint: &str) -> Value {
    use tokio::io::AsyncWriteExt;
    let input = json!({"schema_version":1,"provider":"runpod",
        "credentials":{"runpod":{"api_key":"account-secret-independent","endpoint":endpoint,"region":"EU"}},
        "ownership":{"namespace":"review:race","lease_id":"lease-1"},
        "spec":{"cpu":4,"memory_gb":16,"storage_gb":40,"gpu_count":1,"gpu_class":"nvidia-b200","gpu_vram_gb":180,
            "allow_spot":false,"image":"review/worker:immutable","env":{"WORKER_TOKEN":"worker-secret-independent"},
            "ports":[8787],"endpoint_port":8787,"lease_duration_secs":300,"max_hourly_cost_usd":2}});
    let mut process = tokio::process::Command::new(env!("CARGO_BIN_EXE_cargo-tangle"))
        .args(["tangle", "cloud", "create", "--json"])
        .env_clear()
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home)
        .env("XDG_CACHE_HOME", home.join("cache"))
        .env("PATH", "/usr/bin:/bin")
        .env("TOKIO_WORKER_THREADS", "2")
        .current_dir(home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let mut stdin = process.stdin.take().unwrap();
    stdin
        .write_all(&serde_json::to_vec(&input).unwrap())
        .await
        .unwrap();
    drop(stdin);
    let output = tokio::time::timeout(Duration::from_secs(20), process.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    for bytes in [&output.stdout, &output.stderr] {
        let text = String::from_utf8_lossy(bytes);
        assert!(!text.contains("account-secret-independent"));
        assert!(!text.contains("worker-secret-independent"));
    }
    let mut records = serde_json::Deserializer::from_slice(&output.stdout).into_iter::<Value>();
    let response = records.next().expect("CLI JSON record").unwrap();
    assert!(records.next().is_none());
    assert!(output.status.success(), "CLI failed: {response}");
    assert_eq!(response["ok"], true);
    response
}

#[tokio::test]
async fn two_cli_processes_share_one_native_resource_for_the_same_lease_identity() {
    let server = MockServer::start().await;
    let state = Arc::new(Mutex::new(State::default()));
    Mock::given(|_: &Request| true)
        .respond_with(Account(state.clone()))
        .mount(&server)
        .await;
    let home = tempfile::tempdir().unwrap();
    let left = format!("{}/left", server.uri());
    let right = format!("{}/right", server.uri());
    let (left, right) = tokio::join!(create(home.path(), &left), create(home.path(), &right));
    let state = state.lock().unwrap();
    assert_eq!(
        state.creates, 1,
        "two CLI processes each created a billable resource"
    );
    assert_eq!(state.pods.len(), 1);
    let native_id = left["data"]["provider_id"].as_str().unwrap();
    assert_eq!(native_id, "native-1");
    assert_eq!(right["data"]["provider_id"].as_str(), Some(native_id));
}
