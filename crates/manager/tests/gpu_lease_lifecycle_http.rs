//! Manager GPU allocation, restart and failed retirement through a real HTTP boundary.
#![cfg(feature = "remote-providers")]

use axum::body::{Body, to_bytes};
use axum::extract::State;
use axum::http::{Method, Request, StatusCode};
use axum::{Json, Router};
use blueprint_manager::executor::remote_provider_integration::{
    DeployArtifact, RemoteProviderManager,
};
use blueprint_remote_providers::deployment::DeploymentType;
use blueprint_remote_providers::deployment::manager_integration::{
    RemoteDeploymentConfig, RemoteDeploymentRegistry,
};
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, DeploymentTracker, ResourceSpec,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[cfg(unix)]
#[path = "../../blueprint-remote-providers/tests/fixtures/ssh_process.rs"]
mod ssh_process;

#[derive(Default)]
struct Seller {
    pods: BTreeMap<String, Value>,
    launches: Vec<Value>,
    deny_delete: bool,
    vast: BTreeMap<String, Value>,
    keys: BTreeMap<String, Vec<String>>,
    barrier: Option<Arc<tokio::sync::Barrier>>,
    delay_create: bool,
    with_ssh: bool,
}

async fn seller(
    State(state): State<Arc<Mutex<Seller>>>,
    request: Request<Body>,
) -> (StatusCode, Json<Value>) {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let body = to_bytes(request.into_body(), 1024 * 1024).await.unwrap();
    if path == "/synchronize" {
        let barrier = state.lock().unwrap().barrier.clone().unwrap();
        barrier.wait().await;
        return (StatusCode::OK, Json(Value::Null));
    }
    if (method == Method::POST && path == "/pods")
        || (method == Method::PUT && path == "/asks/7001/")
    {
        let delay = state.lock().unwrap().delay_create;
        if delay {
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
    }
    let mut state = state.lock().unwrap();
    if path == "/catalog/gpus" {
        return (
            StatusCode::OK,
            Json(json!({"gpus":[{
                "id":"NVIDIA H200","memory":141,"secure":true,"community":false,
                "maxCount":{"secure":8,"community":0},"price":{"secure":4.5,"community":0},
                "availability":"HIGH","dataCenters":[{"id":"US-TEST-1","availability":"HIGH"}]
            }]})),
        );
    }
    if path == "/pods" && method == Method::POST {
        let launch: Value = serde_json::from_slice(&body).unwrap();
        state.launches.push(launch.clone());
        let id = format!("lease-{}", state.launches.len());
        let mut pod = json!({"id":id,"name":launch["name"],"env":launch["env"],
            "gpu":{"id":launch["gpu"]["id"],"count":launch["gpu"]["count"],"vcpuCount":8,"memory":64},
            "ports":launch["ports"],"image":launch["image"],"mounts":launch["mounts"],
            "status":"RUNNING","dataCenterId":"US-TEST-1","cost":9.0});
        if state.with_ssh {
            pod["ssh"] = json!({"direct":{"host":"gpu-peer.invalid","port":22,"username":"root"}});
        }
        state.pods.insert(id, pod.clone());
        return (StatusCode::CREATED, Json(pod));
    }
    if path == "/pods" {
        return (
            StatusCode::OK,
            Json(
                json!({"pods":state.pods.values().collect::<Vec<_>>(),"pagination":{"hasNextPage":false}}),
            ),
        );
    }
    if path == "/bundles/" {
        return (
            StatusCode::OK,
            Json(json!({"offers":[{
                "id":7001,"gpu_name":"H200","num_gpus":2,"dph_total":9.0,"storage_total_cost":0.0,
                "reliability2":0.999,"verified":true,"rentable":true,"rented":false,
                "cpu_cores_effective":8,"cpu_ram":64000,"disk_space":1000,"geolocation":"US-TEST-1"
            }]})),
        );
    }
    if path == "/v1/instances" {
        return (
            StatusCode::OK,
            Json(json!({"instances":state.vast.values().collect::<Vec<_>>(),"next_token":null})),
        );
    }
    if path == "/asks/7001/" {
        let launch: Value = serde_json::from_slice(&body).unwrap();
        state.launches.push(launch.clone());
        state.vast.insert("8001".into(), json!({"id":8001,"label":launch["label"],"image_runtype":launch["runtype"],
            "actual_status":"running","intended_status":"running","gpu_name":"H200","num_gpus":2,"dph_total":9.0,"geolocation":"US-TEST-1"}));
        return (
            StatusCode::OK,
            Json(json!({"success":true,"new_contract":8001})),
        );
    }
    if path.starts_with("/instances/") {
        let id = path.split('/').nth(2).unwrap().to_string();
        if path.ends_with("/ssh/") {
            if method == Method::POST {
                let key: Value = serde_json::from_slice(&body).unwrap();
                state
                    .keys
                    .entry(id)
                    .or_default()
                    .push(key["ssh_key"].as_str().unwrap().into());
                return (StatusCode::OK, Json(json!({"success":true})));
            }
            let keys = state
                .keys
                .get(&id)
                .cloned()
                .unwrap_or_default()
                .iter()
                .map(|key| json!({"public_key":key}))
                .collect::<Vec<_>>();
            return (
                StatusCode::OK,
                Json(json!({"success":true,"ssh_keys":serde_json::to_string(&keys).unwrap()})),
            );
        }
        if method == Method::DELETE {
            state.vast.remove(&id);
            return (StatusCode::OK, Json(json!({"success":true})));
        }
        return match state.vast.get(&id) {
            Some(instance) => (StatusCode::OK, Json(json!({"instances":instance}))),
            None => (StatusCode::NOT_FOUND, Json(Value::Null)),
        };
    }
    let id = path.trim_start_matches("/pods/");
    if method == Method::DELETE {
        if state.deny_delete {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({"error":"synthetic retirement refusal"})),
            );
        }
        state.pods.remove(id);
        return (StatusCode::NO_CONTENT, Json(Value::Null));
    }
    match state.pods.get(id) {
        Some(pod) => (StatusCode::OK, Json(pod.clone())),
        None => (StatusCode::NOT_FOUND, Json(Value::Null)),
    }
}

async fn peer(
    initial: Seller,
) -> anyhow::Result<(String, Arc<Mutex<Seller>>, tokio::task::JoinHandle<()>)> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let endpoint = format!("http://{}", listener.local_addr()?);
    let state = Arc::new(Mutex::new(initial));
    let router = Router::new().fallback(seller).with_state(state.clone());
    let server = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    Ok((endpoint, state, server))
}

fn spec(ttl: u64) -> ResourceSpec {
    ResourceSpec {
        gpu_class: Some("nvidia-h200".into()),
        gpu_count: Some(2),
        gpu_vram_gb: Some(140),
        image: Some("example.invalid/ssh-ready-image:v1".into()),
        lease_duration_secs: Some(ttl),
        ..ResourceSpec::basic()
    }
}

async fn manager(endpoint: &str, state: &std::path::Path) -> anyhow::Result<RemoteProviderManager> {
    let credentials = serde_json::from_value(
        json!({"provider":"runpod","api_key":"synthetic-local-peer-key","endpoint":endpoint}),
    )
    .unwrap();
    RemoteProviderManager::with_provisioner(
        CloudProvisioner::from_credentials(vec![credentials]).unwrap(),
        state,
        HashMap::from([(CloudProvider::RunPod, "US-TEST-1".into())]),
        20.0,
    )
    .await
    .map_err(Into::into)
}

#[tokio::test]
async fn manager_retires_owned_gpu_leases_after_restart_and_retries_refused_cleanup()
-> anyhow::Result<()> {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let (endpoint, seller_state, server) = peer(Seller::default()).await?;
    let own_state = tempfile::tempdir()?;
    let foreign_state = tempfile::tempdir()?;
    let own = manager(&endpoint, own_state.path()).await?;
    let foreign = manager(&endpoint, foreign_state.path()).await?;
    let owner_file = std::path::Path::new("blueprint/manager-owner");
    let original_owner = std::fs::read_to_string(own_state.path().join(owner_file))?;
    assert_ne!(
        original_owner,
        std::fs::read_to_string(foreign_state.path().join(owner_file))?
    );
    own.on_service_initiated(1, 1, Some(spec(1)), None, HashMap::new())
        .await?;
    own.on_service_initiated(1, 2, Some(spec(3600)), None, HashMap::new())
        .await?;
    foreign
        .on_service_initiated(1, 1, Some(spec(3600)), None, HashMap::new())
        .await?;
    assert!(
        own.on_service_initiated(1, 2, Some(spec(3600)), None, HashMap::new())
            .await
            .is_err(),
        "replayed initiation must not rent another node"
    );
    let mut capped = spec(3600);
    capped.max_hourly_cost_usd = Some(8.0);
    assert!(
        own.on_service_initiated(1, 3, Some(capped), None, HashMap::new())
            .await
            .is_err()
    );
    let mut tee = spec(3600);
    tee.tee_required = true;
    assert!(
        own.on_service_initiated(1, 4, Some(tee), None, HashMap::new())
            .await
            .is_err(),
        "GPU migration must not weaken the confidential-compute gate"
    );
    let mut upload = spec(3600);
    upload
        .env
        .insert("WORKER_TOKEN".into(), "synthetic-worker-secret".into());
    assert!(
        own.on_service_initiated(
            1,
            5,
            Some(upload),
            Some(DeployArtifact::ContainerImage(
                "example.invalid/workload:v1"
            )),
            HashMap::new()
        )
        .await
        .is_err(),
        "an allocated node without an upload endpoint must be retired on deployment failure"
    );
    {
        let mut state = seller_state.lock().unwrap();
        assert_eq!(state.launches.len(), 4);
        assert_eq!(state.pods.len(), 3);
        assert!(state.launches.iter().all(|body| {
            body["env"]["PUBLIC_KEY"]
                .as_str()
                .is_some_and(|key| key.starts_with("ssh-ed25519 "))
        }));
        assert!(!state.launches.iter().any(|body| {
            body.to_string().contains("synthetic-local-peer-key")
                || body.to_string().contains("PRIVATE KEY")
                || body.to_string().contains("synthetic-worker-secret")
                || body.to_string().contains("WORKER_TOKEN")
        }));
        state.deny_delete = true;
    }
    assert!(
        own.on_service_terminated(1, 2).await.is_err(),
        "a refused deletion must remain visible to its caller"
    );
    assert_eq!(seller_state.lock().unwrap().pods.len(), 3);
    tokio::time::sleep(Duration::from_millis(1100)).await;
    drop(own);
    let archive = tempfile::tempdir()?;
    let recovered = archive.path().join("state");
    std::fs::rename(own_state.path(), &recovered)?;
    std::fs::create_dir(own_state.path())?;
    let replacement = manager(&endpoint, own_state.path()).await?;
    assert_ne!(
        original_owner,
        std::fs::read_to_string(own_state.path().join(owner_file))?
    );
    replacement.on_service_terminated(1, 2).await?;
    assert_eq!(seller_state.lock().unwrap().pods.len(), 3);
    let restarted = manager(&endpoint, &recovered).await?;
    assert_eq!(
        original_owner,
        std::fs::read_to_string(recovered.join(owner_file))?
    );
    assert!(
        !restarted.retire_expired().await.failures.is_empty(),
        "failed TTL cleanup must be retryable after restart"
    );
    seller_state.lock().unwrap().deny_delete = false;
    assert!(restarted.retire_expired().await.failures.is_empty());
    assert_eq!(
        seller_state
            .lock()
            .unwrap()
            .pods
            .keys()
            .cloned()
            .collect::<Vec<_>>(),
        ["lease-2", "lease-3"],
        "only the expired node in this manager namespace is retired"
    );
    restarted.on_service_terminated(1, 2).await?;
    assert_eq!(
        seller_state
            .lock()
            .unwrap()
            .pods
            .keys()
            .cloned()
            .collect::<Vec<_>>(),
        ["lease-3"],
        "the other manager's same service identity survives"
    );
    assert!(
        !own_state
            .path()
            .join("remote_deployments/deployment_state.json")
            .exists(),
        "GPU deadlines are not duplicated in the legacy tracker"
    );
    foreign.on_service_terminated(1, 1).await?;
    assert!(seller_state.lock().unwrap().pods.is_empty());
    drop(replacement);
    for invalid in ["", "truncated-owner"] {
        std::fs::write(own_state.path().join(owner_file), invalid)?;
        assert!(manager(&endpoint, own_state.path()).await.is_err());
        assert_eq!(
            std::fs::read_to_string(own_state.path().join(owner_file))?,
            invalid
        );
    }
    server.abort();
    Ok(())
}

#[tokio::test]
async fn registry_keeps_a_record_when_the_persistent_tracker_refuses_cleanup() -> anyhow::Result<()>
{
    let state = tempfile::tempdir()?;
    let tracker = Arc::new(DeploymentTracker::new(state.path()).await?);
    let registry = RemoteDeploymentRegistry::new(tracker);
    registry
        .register(
            1,
            2,
            RemoteDeploymentConfig {
                deployment_type: DeploymentType::RunPodInstance,
                provider: Some(CloudProvider::RunPod),
                region: Some("US-TEST-1".into()),
                instance_id: "untracked-legacy-lease".into(),
                resource_spec: spec(3600),
                ttl_seconds: Some(3600),
                deployed_at: chrono::Utc::now(),
            },
        )
        .await;
    assert!(registry.cleanup(1, 2).await.is_err());
    assert!(
        registry.get(1, 2).await.is_some(),
        "failed cleanup must not erase the only retry record"
    );
    Ok(())
}

// This subprocess exercises real adapters with isolated synthetic credentials.
#[tokio::test]
async fn allocation_worker() -> anyhow::Result<()> {
    let Ok(endpoint) = std::env::var("GPU_LIFECYCLE_PEER") else {
        return Ok(());
    };
    if std::env::var_os("GPU_LIFECYCLE_INIT_FAILURE").is_some() {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        let provisioner = CloudProvisioner::new().await?;
        assert!(
            provisioner
                .list_managed("synthetic-inventory")
                .await
                .is_err(),
            "an uninitialized configured adapter cannot become an absence proof"
        );
        return Ok(());
    }

    use blueprint_remote_providers::infra::traits::CloudProviderAdapter;
    use blueprint_remote_providers::providers::digitalocean::adapter::DigitalOceanAdapter;
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let mut adapters: HashMap<CloudProvider, Box<dyn CloudProviderAdapter>> = HashMap::new();
    let mut credentials = vec![serde_json::from_value(
        json!({"provider":"runpod","api_key":"synthetic","endpoint":endpoint}),
    )?];
    if std::env::var_os("VAST_AI_API_KEY").is_some() {
        credentials.push(serde_json::from_value(
            json!({"provider":"vast-ai","api_key":"synthetic","endpoint":endpoint}),
        )?);
    }
    adapters.insert(
        CloudProvider::DigitalOcean,
        Box::new(DigitalOceanAdapter::new().await?),
    );
    let provider = if std::env::var("GPU_LIFECYCLE_PROVIDER")? == "runpod" {
        CloudProvider::RunPod
    } else {
        CloudProvider::VastAi
    };
    let state = std::path::PathBuf::from(std::env::var_os("GPU_LIFECYCLE_STATE").unwrap());
    let manager = RemoteProviderManager::with_provisioner(
        CloudProvisioner::with_providers(adapters).with_credentials(credentials)?,
        &state,
        HashMap::from([
            (CloudProvider::RunPod, "US-TEST-1".into()),
            (CloudProvider::VastAi, "US-TEST-1".into()),
            (CloudProvider::DigitalOcean, "nyc3".into()),
        ]),
        20.0,
    )
    .await?;
    let alternatives = std::env::var_os("GPU_LIFECYCLE_ALTERNATIVES").is_some();
    if !alternatives {
        reqwest::get(format!("{endpoint}/synchronize"))
            .await?
            .error_for_status()?;
    }
    let mut request = spec(3600);
    request.provider = Some(provider);
    if alternatives {
        request.gpu_class = None;
        request.gpu_count = None;
        request.ports = vec![8000, 12345];
        request.endpoint_port = Some(8000);
        request.arms = serde_json::from_value(json!([("nvidia-h200",2),("nvidia-b200",8)].map(|(gpu_class,gpu_count)|json!({
            "family":"fixture","arm":gpu_class,"gpu_class":gpu_class,"gpu_count":gpu_count,"tp":1,"replicas":1
        }))))?;
    }
    let artifact = alternatives.then_some(DeployArtifact::ContainerImage(
        "example.invalid/workload:v1",
    ));
    let env = if alternatives {
        HashMap::from([("SECRET_TOKEN".into(), std::env::var("SSH_PROCESS_SECRET")?)])
    } else {
        HashMap::new()
    };
    let result = manager
        .on_service_initiated(9, 9, Some(request), artifact, env)
        .await
        .inspect_err(|error| eprintln!("manager allocation refused: {error}"));
    if alternatives {
        result?;
        manager.on_service_terminated(9, 9).await?;
        return Ok(());
    }
    std::fs::write(
        std::env::var_os("GPU_LIFECYCLE_RESULT").unwrap(),
        result.map_or("refused", |_| "allocated"),
    )?;
    Ok(())
}

fn worker(
    endpoint: &str,
    state: &std::path::Path,
    provider: &str,
) -> anyhow::Result<tokio::process::Command> {
    let mut command = tokio::process::Command::new(std::env::current_exe()?);
    command
        .args(["--exact", "allocation_worker", "--test-threads=1"])
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("HOME", state)
        .env("RUST_TEST_NOCAPTURE", "1")
        .env("GPU_LIFECYCLE_PEER", endpoint)
        .env("GPU_LIFECYCLE_PROVIDER", provider)
        .env("GPU_LIFECYCLE_STATE", state)
        .env("XDG_CACHE_HOME", state.join(format!("cache-{provider}")))
        .env("GPU_LIFECYCLE_RESULT", state.join(provider))
        .env("DIGITALOCEAN_TOKEN", "synthetic-cpu-provider-key")
        .kill_on_drop(true);
    Ok(command)
}

#[tokio::test]
async fn restart_missing_enabled_vast_credentials_cannot_rent_runpod() -> anyhow::Result<()> {
    let (endpoint, state, server) = peer(Seller {
        barrier: Some(Arc::new(tokio::sync::Barrier::new(1))),
        ..Default::default()
    })
    .await?;
    let directory = tempfile::tempdir()?;
    for (provider, expected) in [("vast-ai", "allocated"), ("runpod", "refused")] {
        let mut command = worker(&endpoint, directory.path(), provider)?;
        if provider == "vast-ai" {
            command.env("VAST_AI_API_KEY", "synthetic-present-key");
        }
        assert!(
            tokio::time::timeout(Duration::from_secs(30), command.status())
                .await??
                .success()
        );
        assert_eq!(
            std::fs::read_to_string(directory.path().join(provider))?,
            expected
        );
    }
    assert_eq!(state.lock().unwrap().vast.len(), 1);
    assert!(state.lock().unwrap().pods.is_empty());
    assert_eq!(state.lock().unwrap().launches.len(), 1);
    server.abort();
    Ok(())
}

#[tokio::test]
async fn separate_processes_share_one_service_lock_across_different_gpu_providers_and_cpu_inventory()
-> anyhow::Result<()> {
    let (endpoint, state, server) = peer(Seller {
        barrier: Some(Arc::new(tokio::sync::Barrier::new(2))),
        delay_create: true,
        ..Default::default()
    })
    .await?;
    let directory = tempfile::tempdir()?;
    let mut children = Vec::new();
    for provider in ["runpod", "vast-ai"] {
        let receipt = directory.path().join(provider);
        let child = worker(&endpoint, directory.path(), provider)?
            .env("VAST_AI_API_KEY", "synthetic-present-key")
            .spawn()?;
        children.push((child, receipt));
    }
    let mut allocated = 0;
    for (mut child, receipt) in children {
        assert!(
            tokio::time::timeout(Duration::from_secs(30), child.wait())
                .await??
                .success()
        );
        allocated += usize::from(std::fs::read_to_string(receipt)? == "allocated");
    }
    let status = worker(&endpoint, directory.path(), "runpod")?
        .env("GPU_LIFECYCLE_INIT_FAILURE", "1")
        .env("OVH_APPLICATION_KEY", "synthetic-uninitialized-provider")
        .status()
        .await?;
    assert!(status.success());
    assert_eq!(
        allocated, 1,
        "two manager processes must not rent on different providers for the same service"
    );
    let state = state.lock().unwrap();
    assert_eq!(
        state.launches.len(),
        1,
        "CPU-only inventory must be skipped, while one GPU rental is admitted"
    );
    assert_eq!(state.pods.len() + state.vast.len(), 1);
    drop(state);
    server.abort();
    Ok(())
}

#[tokio::test]
async fn unavailable_adapter_does_not_prevent_other_owned_cleanup() -> anyhow::Result<()> {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let (endpoint, state, server) = peer(Seller::default()).await?;
    let directory = tempfile::tempdir()?;
    let credentials = serde_json::from_value(
        json!({"provider":"runpod","api_key":"synthetic-local-peer-key","endpoint":endpoint}),
    )?;
    let seed = manager(&endpoint, directory.path()).await?;
    seed.on_service_initiated(5, 6, Some(spec(3600)), None, HashMap::new())
        .await?;
    drop(seed);
    let manager = RemoteProviderManager::with_provisioner(
        CloudProvisioner::from_credentials(vec![credentials])?,
        directory.path(),
        HashMap::from([
            (CloudProvider::RunPod, "US-TEST-1".into()),
            (CloudProvider::GCP, "unavailable".into()),
        ]),
        20.0,
    )
    .await?;
    assert!(
        manager.on_service_terminated(5, 6).await.is_err(),
        "unavailable account failure remains visible"
    );
    assert!(
        state.lock().unwrap().pods.is_empty(),
        "a different adapter failure must not strand the owned rental"
    );
    server.abort();
    Ok(())
}

#[cfg(unix)]
#[tokio::test]
async fn selected_arm_reaches_the_manager_uploaded_container() -> anyhow::Result<()> {
    let (endpoint, state, server) = peer(Seller {
        with_ssh: true,
        ..Default::default()
    })
    .await?;
    let directory = tempfile::tempdir()?;
    let mut command = ssh_process::command(
        directory.path(),
        &std::env::current_exe()?,
        "allocation_worker",
        "trusted",
    )?;
    command
        .env("GPU_LIFECYCLE_PEER", &endpoint)
        .env("GPU_LIFECYCLE_PROVIDER", "runpod")
        .env("GPU_LIFECYCLE_STATE", directory.path())
        .env("GPU_LIFECYCLE_ALTERNATIVES", "1")
        .env("VAST_AI_API_KEY", "synthetic-present-key")
        .env("DIGITALOCEAN_TOKEN", "synthetic-cpu-provider-key");
    let output = tokio::process::Command::from(command).output().await?;
    assert!(!String::from_utf8_lossy(&output.stderr).contains(ssh_process::SECRET));
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value =
        serde_json::from_slice(&std::fs::read(directory.path().join("docker.json"))?)?;
    assert!(
        receipt["arguments"]
            .as_array()
            .unwrap()
            .windows(2)
            .any(|pair| pair == [json!("--gpus"), json!("2")])
    );
    assert_eq!(state.lock().unwrap().launches.len(), 1);
    assert!(state.lock().unwrap().pods.is_empty());
    server.abort();
    Ok(())
}
