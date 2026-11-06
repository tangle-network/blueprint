//! Public metadata → manager runtime → Kubernetes HTTP contract.
#![cfg(all(feature = "containers", not(feature = "vm-sandbox")))]

use axum::body::{Body, to_bytes};
use axum::extract::State;
use axum::http::{Method, Request, StatusCode};
use axum::{Json, Router};
use blueprint_client_tangle::resolve_execution_profile_from_profiling_data;
use blueprint_core::resources::ResourceSpec;
use blueprint_manager::blueprint::native::FilteredBlueprint;
use blueprint_manager::config::{BlueprintManagerConfig, BlueprintManagerContext};
use blueprint_manager::rt::ResourceLimits;
use blueprint_manager::rt::container::ContainerInstance;
use blueprint_manager::rt::service::Status;
use blueprint_manager::sources::{BlueprintArgs, BlueprintEnvVars};
use blueprint_runner::config::{BlueprintEnvironment, Protocol};
use serde_json::{Value, json};
use serial_test::serial;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

type Pods = Arc<Mutex<BTreeMap<String, Value>>>;

async fn kube_peer(State(pods): State<Pods>, request: Request<Body>) -> (StatusCode, Json<Value>) {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    if method == Method::DELETE {
        pods.lock().unwrap().remove(&path);
        return (
            StatusCode::OK,
            Json(json!({"kind":"Status","apiVersion":"v1","status":"Success"})),
        );
    }
    if method == Method::GET {
        let value = pods.lock().unwrap().get(&path).cloned();
        return match value {
            Some(pod) => (StatusCode::OK, Json(pod)),
            None => (
                StatusCode::NOT_FOUND,
                Json(
                    json!({"kind":"Status","apiVersion":"v1","status":"Failure","reason":"NotFound","code":404}),
                ),
            ),
        };
    }
    let body = to_bytes(request.into_body(), 1024 * 1024).await.unwrap();
    let mut value: Value = serde_json::from_slice(&body).unwrap();
    if path.contains("/pods/") {
        value["status"] = json!({"phase":"Running"});
        pods.lock().unwrap().insert(path, value.clone());
    }
    (StatusCode::OK, Json(value))
}

struct KubeconfigRestore(Option<std::ffi::OsString>);
impl Drop for KubeconfigRestore {
    fn drop(&mut self) {
        unsafe {
            match self.0.take() {
                Some(value) => std::env::set_var("KUBECONFIG", value),
                None => std::env::remove_var("KUBECONFIG"),
            }
        }
    }
}

#[tokio::test]
#[serial(env)]
async fn metadata_class_count_and_minimum_vram_reach_the_kubernetes_wire() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let pods = Pods::default();
    let router = Router::new().fallback(kube_peer).with_state(pods.clone());
    let server = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    let temp = tempfile::tempdir()?;
    let kubeconfig = temp.path().join("kubeconfig.json");
    std::fs::write(
        &kubeconfig,
        serde_json::to_vec(&json!({
            "apiVersion":"v1","kind":"Config","current-context":"gpu-test",
            "clusters":[{"name":"gpu-test","cluster":{"server":format!("http://{address}")}}],
            "contexts":[{"name":"gpu-test","context":{"cluster":"gpu-test","user":"empty"}}],
            "users":[{"name":"empty","user":{}}]
        }))?,
    )?;
    let _restore = KubeconfigRestore(std::env::var_os("KUBECONFIG"));
    unsafe { std::env::set_var("KUBECONFIG", &kubeconfig) };
    let mut config = BlueprintManagerConfig::default();
    config.paths.data_dir = temp.path().join("data");
    config.paths.cache_dir = temp.path().join("cache");
    config.paths.runtime_dir = temp.path().join("runtime");
    config.paths.keystore_uri = temp.path().join("keystore").display().to_string();
    config.container_options.kube_service_port = 0;
    let ctx = BlueprintManagerContext::new(config).await?;
    let environment = BlueprintEnvironment::default();
    for policy in ["required", "preferred"] {
        let profile = resolve_execution_profile_from_profiling_data(&json!({
            "execution_profile":{"gpu":{"policy":policy,"class":"nvidia-h200","min_count":2,"min_vram_gb":140}}
        }).to_string())?.unwrap();
        let limits = ResourceLimits::from(&profile.gpu);
        let spec: ResourceSpec = (&limits).into();
        let wire = serde_json::to_value(&spec)?;
        assert_eq!(wire["gpu_class"], "nvidia-h200");
        assert_eq!(wire["gpu_count"], 2);
        assert_eq!(wire["gpu_vram_gb"], 140);
        let name = format!("gpu-{policy}");
        let filtered = FilteredBlueprint {
            blueprint_id: 1,
            services: vec![2],
            sources: vec![],
            name: name.clone(),
            registration_mode: false,
            registration_capture_only: false,
            protocol: Protocol::Tangle,
        };
        let variables = BlueprintEnvVars::new(&environment, &ctx, 1, 2, &filtered, &name);
        let mut instance = ContainerInstance::new(
            &ctx,
            limits,
            &name,
            "example.invalid/image@sha256:fixture".into(),
            variables,
            BlueprintArgs::new(&ctx),
            profile.confidentiality,
            false,
        )
        .await?;
        instance.start().await?;
        assert_eq!(instance.status().await?, Status::Running);
        let path = format!("/api/v1/namespaces/blueprint-manager/pods/{name}");
        let pod = pods.lock().unwrap().get(&path).cloned().unwrap();
        let node_affinity = &pod["spec"]["affinity"]["nodeAffinity"];
        let term = if policy == "required" {
            assert_eq!(
                pod["spec"]["containers"][0]["resources"]["limits"]["nvidia.com/gpu"],
                "2"
            );
            assert_eq!(
                pod["spec"]["containers"][0]["resources"]["requests"]["nvidia.com/gpu"],
                "2"
            );
            &node_affinity["requiredDuringSchedulingIgnoredDuringExecution"]["nodeSelectorTerms"][0]
        } else {
            assert!(
                pod["spec"]["containers"][0]["resources"]["limits"]
                    .get("nvidia.com/gpu")
                    .is_none()
            );
            assert!(
                node_affinity
                    .get("requiredDuringSchedulingIgnoredDuringExecution")
                    .is_none()
            );
            &node_affinity["preferredDuringSchedulingIgnoredDuringExecution"][0]["preference"]
        };
        let expressions = term["matchExpressions"].as_array().unwrap();
        let class = expressions
            .iter()
            .find(|entry| entry["key"] == "gpu.tangle.tools/class")
            .unwrap();
        assert_eq!(class["operator"], "In");
        assert_eq!(class["values"], json!(["nvidia-h200"]));
        let vram = expressions
            .iter()
            .find(|entry| entry["key"] == "gpu.tangle.tools/vram-gb")
            .unwrap();
        assert_eq!(vram["operator"], "Gt");
        let threshold: u32 = vram["values"][0].as_str().unwrap().parse()?;
        assert!(
            141 > threshold,
            "141 decimal GB must satisfy a 140 GB minimum"
        );
        assert!(140 > threshold, "the requested minimum itself must qualify");
        assert!(139 <= threshold, "less VRAM must not qualify");
        assert!(pod["spec"].get("nodeSelector").is_none());
        instance.shutdown().await?;
        assert!(!pods.lock().unwrap().contains_key(&path));
    }
    server.abort();
    Ok(())
}
