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

#[derive(Default)]
struct Seller {
    pods: BTreeMap<String, Value>,
    launches: Vec<Value>,
    deny_delete: bool,
}

async fn seller(
    State(state): State<Arc<Mutex<Seller>>>,
    request: Request<Body>,
) -> (StatusCode, Json<Value>) {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let body = to_bytes(request.into_body(), 1024 * 1024).await.unwrap();
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
        let pod = json!({"id":id,"name":launch["name"],"env":launch["env"],
            "gpu":{"id":launch["gpu"]["id"],"count":launch["gpu"]["count"],"vcpuCount":8,"memory":64},
            "ports":launch["ports"],"image":launch["image"],"mounts":launch["mounts"],
            "status":"RUNNING","dataCenterId":"US-TEST-1","cost":9.0});
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

async fn manager(endpoint: &str, state: &std::path::Path) -> RemoteProviderManager {
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
    .unwrap()
}

#[tokio::test]
async fn manager_retires_owned_gpu_leases_after_restart_and_retries_refused_cleanup()
-> anyhow::Result<()> {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let endpoint = format!("http://{}", listener.local_addr()?);
    let seller_state = Arc::new(Mutex::new(Seller::default()));
    let router = Router::new()
        .fallback(seller)
        .with_state(seller_state.clone());
    let server = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    let own_state = tempfile::tempdir()?;
    let foreign_state = tempfile::tempdir()?;
    let own = manager(&endpoint, own_state.path()).await;
    let foreign = manager(&endpoint, foreign_state.path()).await;
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
    let restarted = manager(&endpoint, own_state.path()).await;
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
