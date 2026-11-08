//! Real CLI processes and shared native HTTP peers; no paid resources or hardware.
#![cfg(all(feature = "remote-providers", unix))]
use serde_json::{Value, json};
use std::{path::Path, process::Stdio, time::Duration};
use tokio::{
    io::AsyncWriteExt,
    process::{Child, Command},
};
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{method, path_regex},
};

#[path = "../../crates/blueprint-remote-providers/tests/fixtures/measured_recipe.rs"]
mod measured_recipe;
#[path = "../../crates/blueprint-remote-providers/tests/fixtures/runpod_http.rs"]
mod peer;

const KEY: &str =
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMo32AY0vFK7g5FBIBcyPdxaxSEM5rEc0kEzMVveA9b+";

async fn spawn(verb: &str, input: &[u8], home: &Path) -> Child {
    let mut child = Command::new(env!("CARGO_BIN_EXE_cargo-tangle"))
        .args(["tangle", "cloud", verb, "--json"])
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("HOME", home)
        .env("XDG_CACHE_HOME", home)
        .env("RUNPOD_API_KEY", "inherited-credential-must-not-escape")
        .env("RUST_LOG", "trace")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    // An over-limit document may close the pipe before all bytes are written.
    let _ = child.stdin.take().unwrap().write_all(input).await;
    child
}

async fn raw(verb: &str, input: &[u8], home: &Path) -> (bool, Value) {
    let child = spawn(verb, input, home).await;
    let output = tokio::time::timeout(Duration::from_secs(30), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    for bytes in [&output.stdout, &output.stderr] {
        let text = String::from_utf8_lossy(bytes);
        for secret in [
            "inherited-credential-must-not-escape",
            "synthetic-local-peer-key",
            "reflected-secret",
        ]
        .into_iter()
        .chain(measured_recipe::CANARIES)
        .chain(["PRIVATE_ENDPOINT_CANARY"])
        {
            assert!(
                !text.contains(secret),
                "credential appeared in process output"
            );
        }
    }
    let response: Value =
        serde_json::from_slice(&output.stdout).expect("one public JSON envelope and EOF");
    assert_eq!(response["schema_version"], 2);
    assert_eq!(response["ok"], output.status.success());
    (output.status.success(), response)
}

async fn call(verb: &str, request: &Value, home: &Path) -> Value {
    let (ok, response) = raw(verb, &serde_json::to_vec(request).unwrap(), home).await;
    assert!(ok, "{response}");
    response["data"].clone()
}
async fn refused(verb: &str, request: &Value, home: &Path, code: &str) {
    let (ok, response) = raw(verb, &serde_json::to_vec(request).unwrap(), home).await;
    assert!(!ok);
    assert_eq!(response["error"]["code"], code);
}
/// `state-init` enrolls only an owner-only directory; `tempdir()` follows the umask.
fn state_root() -> tempfile::TempDir {
    use std::os::unix::fs::PermissionsExt;
    tempfile::Builder::new()
        .permissions(std::fs::Permissions::from_mode(0o700))
        .tempdir()
        .unwrap()
}
fn account(endpoint: &str) -> Value {
    json!([{"provider":"runpod","api_key":"synthetic-local-peer-key","endpoint":endpoint}])
}
fn request(endpoint: &str, root: &Path) -> Value {
    json!({"schema_version":2,"credentials":account(endpoint),"state_dir":root})
}
fn spec(namespace: &str, lease: &str) -> Value {
    json!({"cpu":2,"memory_gb":4,"storage_gb":20,"gpu_class":"nvidia-h200","gpu_count":2,
        "gpu_vram_gb":140,"allow_spot":false,"image":"example.invalid/workload:v1",
        "region":"US-TEST-1","volume_gb":37,"ports":[8000],"endpoint_port":8000,"env":{},
        "lease_duration_secs":3600,"max_hourly_cost_usd":10,"ssh_public_key":KEY,
        "ownership":{"namespace":namespace,"lease_id":lease}})
}
fn quote_request(base: &Value, spec: &Value) -> Value {
    json!({"schema_version":2,"credentials":base["credentials"],"spec":spec})
}
fn scoped(base: &Value, spec: &Value) -> Value {
    let mut request = base.clone();
    request["provider"] = json!("runpod");
    request["identity"] = spec["ownership"].clone();
    request
}
fn visit(value: &Value, check: &impl Fn(&Value) -> bool) -> bool {
    check(value)
        || match value {
            Value::Object(map) => map.values().any(|value| visit(value, check)),
            Value::Array(values) => values.iter().any(|value| visit(value, check)),
            _ => false,
        }
}

#[tokio::test]
async fn offline_contract_and_closed_parser_are_real_process_boundaries() {
    let root = tempfile::tempdir().unwrap();
    let first = call("catalog", &json!({}), root.path()).await;
    assert_eq!(first, call("catalog", &json!({}), root.path()).await);
    assert!(
        first["schema_sha256"]
            .as_str()
            .is_some_and(|digest| digest.len() == 64)
    );
    let providers = first["providers"].as_array().unwrap();
    assert_eq!(
        providers
            .iter()
            .map(|p| p["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["bittensor-lium", "lambda-labs", "runpod", "vast-ai"]
    );
    assert!(providers.iter().all(|provider| {
        provider["credentials"]
            .as_array()
            .unwrap()
            .iter()
            .any(|field| {
                field["field"] == "api_key" && field["required"] == true && field["env"].is_string()
            })
    }));
    assert!(visit(
        &first["boundary_schema"]["requests"]["quote"],
        &|node| node["properties"].get("env").is_some()
    ));
    assert!(visit(
        &first["boundary_schema"]["responses"]["error"],
        &|node| node["enum"]
            .as_array()
            .is_some_and(|codes| codes.len() == 14
                && codes.iter().any(|code| code == "indeterminate")
                && codes.iter().any(|code| code == "preempted"))
    ));
    let limit = first["max_json_bytes"].as_u64().unwrap() as usize;
    for input in [b"{\"schema_version\":2,\"schema_version\":2}".to_vec(),
        b"{} {}".to_vec(),
        br#"{"schema_version":2,"credentials":[{"provider":"runpod","api_key":"reflected-secret","api_key":"reflected-secret"}]}"#.to_vec(),
        vec![b' '; limit + 1],
        format!("{}0{}", "[".repeat(200), "]".repeat(200)).into_bytes()] {
        let (ok, response) = raw("quote", &input, root.path()).await;
        assert!(!ok);
        assert_eq!(response["error"]["code"], "invalid_request");
    }
    // Legacy argument semantics remain separate from managed stdin.
    for args in [
        vec!["tangle", "cloud", "list"],
        vec!["tangle", "cloud", "status"],
        vec!["tangle", "cloud", "configure", "--help"],
    ] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-tangle"));
        command
            .args(args)
            .env_clear()
            .env("HOME", root.path())
            .env("PATH", "/usr/bin:/bin")
            .kill_on_drop(true);
        let output = tokio::time::timeout(Duration::from_secs(30), command.output())
            .await
            .unwrap()
            .unwrap();
        assert!(output.status.success());
    }
}

#[tokio::test]
async fn exact_quote_lifecycle_keeps_foreign_scope_and_original_authority() {
    let (server, state) = peer::runpod_peer_state().await;
    let root = state_root();
    let base = request(&server.uri(), root.path());
    call("state-init", &base, root.path()).await;
    refused("state-init", &base, root.path(), "conflict").await;
    let original = spec("cell-a", "lease-a");
    let quote = call("quote", &quote_request(&base, &original), root.path()).await;
    assert_eq!(quote["quotes"][0]["hourly_cost_usd"], 9.0);
    assert!(quote["errors"].as_array().unwrap().is_empty());
    let mut create = base.clone();
    create["spec"] = original.clone();
    create["candidate"] = quote["quotes"][0]["candidate"].clone();
    let mut invalid = create.clone();
    invalid["candidate"] = json!("invalid-fresh-candidate");
    refused("create", &invalid, root.path(), "invalid_request").await;
    let mut mismatch = create.clone();
    mismatch["spec"]["max_hourly_cost_usd"] = json!(8);
    refused("create", &mismatch, root.path(), "conflict").await;
    assert_eq!(state.lock().unwrap().0, 0);
    let resource = call("create", &create, root.path()).await;
    let expiry = resource["expires_at"].as_str().unwrap();
    assert_eq!(resource["gpu_count"], 2);
    assert_eq!(resource["hourly_cost_usd"], 9.0);
    assert_eq!(resource["status"], "Running");
    assert_eq!(resource["preempted"], false);
    assert!(resource.get("started_at").is_none() && resource.get("created_at").is_some());
    assert!(resource["worker_url"].is_null());
    assert!(
        resource["endpoint_url"]
            .as_str()
            .unwrap()
            .starts_with("https://")
    );
    assert!(
        resource["ssh_endpoint"]
            .as_str()
            .unwrap()
            .starts_with("ssh://")
    );
    assert!(resource.get("env").is_none() && resource.get("instance").is_none());
    let launch = server
        .received_requests()
        .await
        .unwrap()
        .into_iter()
        .find(|r| r.method == "POST")
        .unwrap()
        .body_json::<Value>()
        .unwrap();
    assert_eq!(launch["mounts"]["persistent"]["size"], 37);
    assert_eq!(launch["ports"], json!(["22/tcp", "8000/http"]));
    let mut status = scoped(&base, &original);
    status["provider_id"] = resource["provider_id"].clone();
    assert_eq!(
        call("status", &status, root.path()).await["expires_at"],
        expiry
    );
    for (native, canonical) in [
        ("EXITED", "Stopped"),
        ("TERMINATED", "Terminated"),
        ("OTHER", "Unknown"),
    ] {
        state
            .lock()
            .unwrap()
            .1
            .get_mut(resource["provider_id"].as_str().unwrap())
            .unwrap()["status"] = json!(native);
        assert_eq!(
            call("status", &status, root.path()).await["status"],
            canonical
        );
    }
    state
        .lock()
        .unwrap()
        .1
        .get_mut(resource["provider_id"].as_str().unwrap())
        .unwrap()["status"] = json!("RUNNING");
    let mut wrong_id = status.clone();
    wrong_id["provider_id"] = json!("missing");
    refused("status", &wrong_id, root.path(), "not_found").await;
    // SDK deliberately ignores replacement candidate on an attempted identity.
    create["candidate"] = json!("replacement-is-not-decoded-on-replay");
    assert_eq!(
        call("create", &create, root.path()).await["provider_id"],
        resource["provider_id"]
    );
    let mut changed = create.clone();
    changed["spec"]["lease_duration_secs"] = json!(7200);
    refused("create", &changed, root.path(), "conflict").await;
    changed = create.clone();
    changed["credentials"][0]["api_key"] = json!("reflected-secret");
    refused("create", &changed, root.path(), "conflict").await;
    changed = create.clone();
    changed["spec"]["ssh_public_key"] =
        json!("ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAICYAqclxVqvBAc0OzItjBj4L68M4hqJPZbsnOUPuk0Ug");
    refused("create", &changed, root.path(), "conflict").await;
    assert_eq!(state.lock().unwrap().0, 1);
    let foreign_root = state_root();
    let foreign_base = request(&server.uri(), foreign_root.path());
    call("state-init", &foreign_base, foreign_root.path()).await;
    let foreign_spec = spec("cell-b", "lease-b");
    let foreign_quote = call(
        "quote",
        &quote_request(&foreign_base, &foreign_spec),
        foreign_root.path(),
    )
    .await;
    let mut foreign_create = foreign_base.clone();
    foreign_create["spec"] = foreign_spec.clone();
    foreign_create["candidate"] = foreign_quote["quotes"][0]["candidate"].clone();
    let foreign = call("create", &foreign_create, foreign_root.path()).await;
    let mut foreign_delete = scoped(&base, &foreign_spec);
    foreign_delete["provider_id"] = foreign["provider_id"].clone();
    refused("destroy", &foreign_delete, root.path(), "conflict").await;
    let list = json!({"schema_version":2,"state_dir":root.path(),"credentials":base["credentials"],"namespace":"cell-a"});
    assert_eq!(
        call("list", &list, root.path()).await["resources"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let mut obsolete = list.clone();
    obsolete["provider"] = json!("runpod");
    refused("list", &obsolete, root.path(), "invalid_request").await;
    let mut unavailable = list.clone();
    unavailable["credentials"].as_array_mut().unwrap().push(
        json!({"provider":"vast-ai","api_key":"synthetic-local-peer-key","endpoint":server.uri()}),
    );
    let (ok, failed_inventory) = raw(
        "list",
        &serde_json::to_vec(&unavailable).unwrap(),
        root.path(),
    )
    .await;
    assert!(
        !ok && failed_inventory.get("data").is_none(),
        "unknown account inventory is not partial absence"
    );
    call("destroy", &status, root.path()).await;
    assert!(
        call("list", &list, root.path()).await["resources"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let mut recovery = base.clone();
    recovery["spec"] = original;
    assert!(call("recover", &recovery, root.path()).await["resource"].is_null());
    refused("create", &create, root.path(), "conflict").await;
    assert_eq!(
        state.lock().unwrap().0,
        2,
        "vanished attempted identity must not rent again"
    );
    assert!(
        state
            .lock()
            .unwrap()
            .1
            .contains_key(foreign["provider_id"].as_str().unwrap())
    );
    let mut foreign_delete = scoped(&foreign_base, &foreign_spec);
    foreign_delete["provider_id"] = foreign["provider_id"].clone();
    call("destroy", &foreign_delete, foreign_root.path()).await;
    assert!(state.lock().unwrap().1.is_empty());
}

#[tokio::test]
async fn killed_create_recovers_in_new_process_and_lost_state_never_repairs() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let (server, state) = peer::runpod_peer_state().await;
    let root = state_root();
    let base = request(&server.uri(), root.path());
    call("state-init", &base, root.path()).await;
    let original = spec("restart-cell", "lease");
    let quotes = call("quote", &quote_request(&base, &original), root.path()).await;
    let mut create = base.clone();
    create["spec"] = original.clone();
    create["candidate"] = quotes["quotes"][0]["candidate"].clone();
    // One-shot, not scoped: wiremock 0.6 sorts mocks by priority in place, so a
    // scoped guard's stale index would deactivate the base peer on drop.
    Mock::given(method("GET"))
        .and(path_regex("^/pods/[^/]+$"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(60)))
        .with_priority(1)
        .up_to_n_times(1)
        .mount(&server)
        .await;
    let mut child = spawn("create", &serde_json::to_vec(&create).unwrap(), root.path()).await;
    // Kill only once the acknowledged create is blocked reading its pod back.
    tokio::time::timeout(Duration::from_secs(10), async {
        while !server
            .received_requests()
            .await
            .unwrap()
            .iter()
            .any(|request| request.method == "GET" && request.url.path().starts_with("/pods/"))
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    child.kill().await.unwrap();
    child.wait().await.unwrap();
    let mut recovery = base.clone();
    recovery["spec"] = original.clone();
    let recovered = call("recover", &recovery, root.path()).await["resource"].clone();
    assert!(recovered["expires_at"].is_string());
    assert_eq!(call("create", &create, root.path()).await, recovered);
    assert_eq!(state.lock().unwrap().0, 1);
    let authorization = root.path().join("blueprint/lease-authorizations.json");
    let saved = std::fs::read(&authorization).unwrap();
    std::fs::write(&authorization, b"broken").unwrap();
    refused("create", &create, root.path(), "conflict").await;
    std::fs::remove_file(&authorization).unwrap();
    refused("recover", &recovery, root.path(), "conflict").await;
    refused("state-init", &base, root.path(), "conflict").await;
    std::fs::write(&authorization, saved).unwrap();
    std::fs::set_permissions(&authorization, std::fs::Permissions::from_mode(0o600)).unwrap();
    let link = tempfile::tempdir().unwrap();
    let alias = link.path().join("alias");
    symlink(root.path(), &alias).unwrap();
    let mut unsafe_request = create.clone();
    unsafe_request["state_dir"] = json!(alias);
    refused("create", &unsafe_request, root.path(), "conflict").await;
    std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o755)).unwrap();
    refused("create", &create, root.path(), "conflict").await;
    std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    assert_eq!(state.lock().unwrap().0, 1);
    let mut cleanup = scoped(&base, &original);
    cleanup["provider_id"] = recovered["provider_id"].clone();
    call("destroy", &cleanup, root.path()).await;
}

#[tokio::test]
async fn measured_quotes_keep_public_provenance_and_bounded_failures() {
    let (server, state) = peer::runpod_peer_state().await;
    let root = tempfile::tempdir().unwrap();
    let base = request(&server.uri(), root.path());
    let original = quote_request(&base, &spec("parse-cell", "lease"));
    for (pointer, value, expected) in [
        (
            "/ownership",
            json!({"namespace":"obsolete","lease_id":"duplicate"}),
            "invalid_request",
        ),
        ("/ttl", json!(3600), "invalid_request"),
        ("/spec/ownership/unknown", json!(true), "invalid_request"),
        ("/spec/gpu_count", json!(1.5), "invalid_request"),
        (
            "/spec/qos",
            json!({"priority":0,"sla_target":0,"reliability_multiplier":0,"unknown":0}),
            "invalid_request",
        ),
        (
            "/spec/qos",
            json!({"priority":1,"sla_target":0,"reliability_multiplier":0}),
            "unsupported",
        ),
        ("/spec/cpu", json!(1e100), "invalid_request"),
        (
            "/spec/env",
            json!({"TANGLE_GPU_WORKER_TOKEN":"reflected-secret"}),
            "invalid_request",
        ),
        ("/credentials/0/unknown", json!(true), "invalid_request"),
    ] {
        let mut changed = original.clone();
        // Existing fields are changed by pointer; unknown object members are inserted explicitly.
        if let Some(slot) = changed.pointer_mut(pointer) {
            *slot = value;
        } else {
            let (parent, field) = pointer.rsplit_once('/').unwrap();
            changed
                .pointer_mut(parent)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .insert(field.into(), value);
        }
        refused("quote", &changed, root.path(), expected).await;
    }
    assert!(
        server.received_requests().await.unwrap().is_empty(),
        "boundary restrictions must precede provider work"
    );
    // Full ranking and per-provider failures survive the process envelope.
    state.lock().unwrap().2[1]["price"]["secure"] = json!(3.0);
    let mut ranked = original.clone();
    ranked["credentials"].as_array_mut().unwrap().push(
        json!({"provider":"vast-ai","api_key":"synthetic-local-peer-key","endpoint":server.uri()}),
    );
    ranked["spec"]["gpu_class"] = Value::Null;
    ranked["spec"]["gpu_count"] = Value::Null;
    ranked["spec"]["arms"] = json!([("nvidia-h200", "h200"), ("nvidia-b200", "b200")].map(
        |(class, arm)| json!({
            "family":"fixture","arm":arm,"gpu_class":class,"gpu_count":2,"tp":2,"replicas":1
        })
    ));
    let book = call("quote", &ranked, root.path()).await;
    assert_eq!(book["quotes"].as_array().unwrap().len(), 2);
    assert_eq!(book["quotes"][0]["gpu_class"], "nvidia-b200");
    assert_eq!(book["quotes"][1]["gpu_class"], "nvidia-h200");
    assert!(
        book["errors"]
            .as_array()
            .unwrap()
            .iter()
            .any(|error| error["provider"] == "vast-ai")
    );
    let (private_server, private_state) =
        peer::runpod_peer_state_at(measured_recipe::PRIVATE_ENDPOINT).await;
    private_state.lock().unwrap().2[1]["price"]["secure"] = json!(3.0);
    let private_root = state_root();
    let endpoint = format!(
        "{}{}",
        private_server.uri(),
        measured_recipe::PRIVATE_ENDPOINT
    );
    let private_base = request(&endpoint, private_root.path());
    let private_spec =
        measured_recipe::recipe_value(&measured_recipe::measured_spec("measured-cell", "lease"));
    let measured = call(
        "quote",
        &quote_request(&private_base, &private_spec),
        private_root.path(),
    )
    .await;
    assert_eq!(measured["quotes"][0]["gpu_class"], "nvidia-h200");
    assert_eq!(measured["quotes"][0]["hourly_cost_usd"], 9.0);
    assert_eq!(measured["quotes"][1]["hourly_cost_usd"], 6.0);
    assert_eq!(measured["quotes"][0]["usd_per_million_output_tokens"], 2.5);
    assert_eq!(measured["quotes"][0]["output_tokens_per_second"], 1000.0);
    assert_eq!(
        measured["quotes"][0]["pricing_source"],
        private_server.uri()
    );
    let receipt = &measured["quotes"][0]["measurement"];
    assert_eq!(
        receipt,
        &json!({
            "weights_manifest_sha256":"aa".repeat(32),"raw_result_sha256":"11".repeat(32),
            "measured_at":"2026-11-07T00:00:00Z","input_tokens":6000,"output_tokens":1000,
            "elapsed_ms":1000,"concurrency":8,"tp":2,"replicas":1
        })
    );
    call("state-init", &private_base, private_root.path()).await;
    let mut private_create = private_base.clone();
    private_create["spec"] = private_spec.clone();
    private_create["candidate"] = measured["quotes"][0]["candidate"].clone();
    let assigned = call("create", &private_create, private_root.path()).await;
    let mut private_recover = private_base.clone();
    private_recover["spec"] = private_spec.clone();
    assert_eq!(
        call("recover", &private_recover, private_root.path()).await["resource"]["expires_at"],
        assigned["expires_at"]
    );
    let mut changed_recipe = private_create.clone();
    changed_recipe["spec"]["arms"][0]["measurements"][0]["raw_result_uri"] =
        json!("changed-private-uri");
    refused("create", &changed_recipe, private_root.path(), "conflict").await;
    assert_eq!(private_state.lock().unwrap().0, 1);
    let mut private_delete = scoped(&private_base, &private_spec);
    private_delete["provider_id"] = assigned["provider_id"].clone();
    call("destroy", &private_delete, private_root.path()).await;
    let limit = call("catalog", &json!({}), root.path()).await["max_json_bytes"]
        .as_u64()
        .unwrap() as usize;
    let mut large = original.clone();
    large["spec"]["arms"] = json!([{"family":"x".repeat(limit/8),"arm":"native","gpu_class":"nvidia-h200","gpu_count":2,"tp":2,"replicas":1}]);
    refused("quote", &large, root.path(), "invalid_request").await;
    let mut reflected = original.clone();
    reflected["credentials"][0]["api_key"] = json!("reflected-secret");
    let response = call("quote", &reflected, root.path()).await;
    assert!(response["quotes"].as_array().unwrap().is_empty());
    assert_eq!(response["errors"][0]["code"], "auth_rejected");
    assert_eq!(state.lock().unwrap().0, 0);
}

#[tokio::test]
async fn actual_827_authority_recovers_without_rewriting_or_second_post() {
    use sha2::{Digest, Sha256};
    let producer = std::path::PathBuf::from(
        std::env::var_os("D12_LEGACY_PRODUCER")
            .expect("mandatory actual827 producer artifact missing"),
    );
    let receipt_path = std::path::PathBuf::from(
        std::env::var_os("D12_LEGACY_PRODUCER_RECEIPT")
            .expect("mandatory actual827 producer provenance missing"),
    );
    let provenance: Value = serde_json::from_slice(&std::fs::read(receipt_path).unwrap()).unwrap();
    assert_eq!(
        provenance["sdk_sha"],
        "827ed21658f0da37b426f61d990a832e43f48209"
    );
    assert_eq!(
        provenance["sdk_tree"],
        "f95c02e492dd82d3dc23bd21604caaa2856576d7"
    );
    assert_eq!(
        provenance["binary_sha256"],
        format!("{:x}", Sha256::digest(std::fs::read(&producer).unwrap()))
    );
    assert_eq!(
        provenance["helper_sha256"],
        format!(
            "{:x}",
            Sha256::digest(include_bytes!("support/legacy_d12_producer.rs"))
        )
    );
    let (server, state) = peer::runpod_peer_state_at(measured_recipe::PRIVATE_ENDPOINT).await;
    state.lock().unwrap().2[1]["price"]["secure"] = json!(3.0);
    let root = state_root();
    let endpoint = format!("{}{}", server.uri(), measured_recipe::PRIVATE_ENDPOINT);
    let original =
        measured_recipe::recipe_value(&measured_recipe::measured_spec("legacy-cell", "lease"));
    let input = json!({"endpoint":endpoint,"state_dir":root.path(),"spec":original});
    let mut child = Command::new(&producer)
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("HOME", root.path())
        .env("XDG_CACHE_HOME", root.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(&input).unwrap())
        .await
        .unwrap();
    let output = tokio::time::timeout(Duration::from_secs(30), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    for bytes in [&output.stdout, &output.stderr] {
        let text = String::from_utf8_lossy(bytes);
        for canary in measured_recipe::CANARIES
            .into_iter()
            .chain(["PRIVATE_ENDPOINT_CANARY", "synthetic-local-peer-key"])
        {
            assert!(!text.contains(canary));
        }
    }
    assert!(output.status.success(), "actual old producer failed");
    let old: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(old["legacy_full_arm"], true);
    assert_eq!(old["store_version"], 1);
    assert_eq!(state.lock().unwrap().0, 1);
    let store_path = root.path().join("blueprint/lease-authorizations.json");
    let bytes = std::fs::read(&store_path).unwrap();
    let store: Value = serde_json::from_slice(&bytes).unwrap();
    let candidate = store["entries"]["lease"]["candidate"].clone();
    assert!(
        candidate
            .as_str()
            .is_some_and(|candidate| !candidate.is_empty())
    );
    assert_eq!(store["version"], 1);
    let base = request(&endpoint, root.path());
    let mut recovery = base.clone();
    recovery["spec"] = original.clone();
    let recovered = call("recover", &recovery, root.path()).await["resource"].clone();
    assert_eq!(recovered["provider_id"], old["provider_id"]);
    assert_eq!(recovered["expires_at"], old["expires_at"]);
    assert_eq!(recovered["status"], "Running");
    let mut replay = recovery.clone();
    replay["candidate"] = json!("ignored-replacement-for-old-authority");
    assert_eq!(call("create", &replay, root.path()).await, recovered);
    let mut changed = recovery.clone();
    changed["spec"]["arms"][0]["measurements"][0]["raw_result_uri"] =
        json!("changed-private-recipe");
    refused("recover", &changed, root.path(), "conflict").await;
    changed = recovery.clone();
    changed["spec"]["lease_duration_secs"] = json!(7200);
    refused("recover", &changed, root.path(), "conflict").await;
    changed = recovery.clone();
    changed["credentials"][0]["api_key"] = json!("reflected-secret");
    refused("recover", &changed, root.path(), "conflict").await;
    changed = recovery.clone();
    changed["spec"]["ssh_public_key"] =
        json!("ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAICYAqclxVqvBAc0OzItjBj4L68M4hqJPZbsnOUPuk0Ug");
    refused("recover", &changed, root.path(), "conflict").await;
    assert_eq!(state.lock().unwrap().0, 1);
    assert_eq!(std::fs::read(&store_path).unwrap(), bytes);
    let mut cleanup = scoped(&base, &original);
    cleanup["provider_id"] = old["provider_id"].clone();
    call("destroy", &cleanup, root.path()).await;
    assert_eq!(state.lock().unwrap().0, 1);
    assert!(state.lock().unwrap().1.is_empty());
    let final_bytes = std::fs::read(&store_path).unwrap();
    assert_eq!(
        final_bytes, bytes,
        "old authorization must remain byte-identical"
    );
    let final_store: Value = serde_json::from_slice(&final_bytes).unwrap();
    assert_eq!(final_store["version"], 1);
    assert_eq!(final_store["entries"]["lease"]["candidate"], candidate);
}
