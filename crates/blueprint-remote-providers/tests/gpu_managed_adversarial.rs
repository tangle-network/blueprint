//! Independent proofs through SDK exports and a provider HTTP peer.
use blueprint_core::resources::ManagedLeaseIdentity;
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, ResourceSpec, managed::ProviderCredentials,
};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

fn request() -> ResourceSpec {
    serde_json::from_value(
        json!({"provider":"runpod","cpu":4,"memory_gb":16,"storage_gb":40,
        "gpu_count":8,"gpu_class":"nvidia-b200","gpu_vram_gb":180,"allow_spot":false,"region":"EU",
        "image":"review/worker:immutable","env":{"WORKER_TOKEN":"workload-secret-review"},
        "lease_duration_secs":300,"max_hourly_cost_usd":16,
        "ownership":{"namespace":"review:managed","lease_id":"lease-1"}}),
    )
    .unwrap()
}

fn provisioner(endpoint: &str) -> CloudProvisioner {
    provisioner_region(endpoint, "EU")
}
fn provisioner_region(endpoint: &str, region: &str) -> CloudProvisioner {
    CloudProvisioner::from_credentials(HashMap::from([(
        CloudProvider::RunPod,
        ProviderCredentials {
            api_key: "account-secret-review".into(),
            api_secret: None,
            rate_keys: None,
            endpoint: Some(endpoint.into()),
            region: Some(region.into()),
        },
    )]))
    .unwrap()
}

fn pod(id: &str, name: &str, gpu: &str, count: u32, image: &str) -> Value {
    json!({"id":id,"name":name,"gpuTypeId":gpu,"gpuCount":count,"imageName":image,
        "dataCenterId":"EU","desiredStatus":"RUNNING","costPerHr":8,
        "lastStartedAt":"2026-11-06T00:00:00Z"})
}

#[derive(Default)]
struct State {
    pods: Vec<Value>,
    creates: usize,
    deletes: Vec<String>,
    metadata_delay: bool,
    metadata_failure: bool,
}

#[derive(Clone)]
struct ProviderPeer(Arc<Mutex<State>>);
impl Respond for ProviderPeer {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        let mut state = self.0.lock().unwrap();
        match (request.method.as_str(),request.url.path()) {
            ("GET","/pods") => {
                if state.creates>0 && state.metadata_failure { return ResponseTemplate::new(503); }
                let response = ResponseTemplate::new(200).set_body_json(json!(state.pods));
                if state.creates>0 && state.metadata_delay { response.set_delay(Duration::from_secs(2)) } else { response }
            }
            ("POST","/graphql") => ResponseTemplate::new(200).set_body_json(json!({"data":{"gpuTypes":[
                {"id":"NVIDIA B200","lowestPrice":{"stockStatus":"High","uninterruptablePrice":1}}
            ]}})),
            ("POST","/pods") => {
                let input:Value=request.body_json().unwrap();
                state.creates+=1;
                let mut created=pod("native-created-id",input["name"].as_str().unwrap(),"NVIDIA B200",8,"review/worker:immutable");
                created["env"]=input["env"].clone();
                state.pods.push(created);
                ResponseTemplate::new(201).set_body_json(json!({"id":"native-created-id"}))
            }
            ("DELETE",path) if path.starts_with("/pods/") => {
                let id=path.strip_prefix("/pods/").unwrap();
                state.deletes.push(id.into());state.pods.retain(|p|p["id"]!=id);
                ResponseTemplate::new(204)
            }
            ("GET",path) if path.starts_with("/pods/") => {
                let id=path.strip_prefix("/pods/").unwrap();
                match state.pods.iter().find(|p|p["id"]==id) {
                    Some(pod)=>ResponseTemplate::new(200).set_body_json(pod),None=>ResponseTemplate::new(404),
                }
            }
            _=>ResponseTemplate::new(500),
        }
    }
}

async fn server(state: State) -> (MockServer, Arc<Mutex<State>>) {
    let server = MockServer::start().await;
    let state = Arc::new(Mutex::new(state));
    Mock::given(|_: &Request| true)
        .respond_with(ProviderPeer(state.clone()))
        .mount(&server)
        .await;
    (server, state)
}

#[tokio::test]
async fn managed_adoption_cannot_satisfy_eight_blackwell_with_one_ampere() {
    let (server, state) = server(State {
        pods: vec![pod(
            "old-a100",
            "tangle-gpu-62bb8f79bcd9_lease-1",
            "NVIDIA A100 80GB PCIe",
            1,
            "review/worker:immutable",
        )],
        ..Default::default()
    })
    .await;
    assert!(
        provisioner(&server.uri())
            .create_managed(&request())
            .await
            .is_err(),
        "same ownership identity bypassed the requested GPU class and count"
    );
    assert_eq!(state.lock().unwrap().creates, 0);
}

#[tokio::test]
async fn managed_adoption_cannot_silently_change_the_workload_image() {
    let (server, state) = server(State {
        pods: vec![pod(
            "old-image",
            "tangle-gpu-62bb8f79bcd9_lease-1",
            "NVIDIA B200",
            8,
            "other/workload:unrelated",
        )],
        ..Default::default()
    })
    .await;
    assert!(
        provisioner(&server.uri())
            .create_managed(&request())
            .await
            .is_err(),
        "same ownership identity silently adopted a different workload"
    );
    assert_eq!(state.lock().unwrap().creates, 0);
}

#[tokio::test]
async fn accepted_native_id_is_returned_before_followup_metadata_can_stall() {
    let (server, state) = server(State {
        metadata_delay: true,
        ..Default::default()
    })
    .await;
    let provisioner = provisioner(&server.uri());
    let result = tokio::time::timeout(
        Duration::from_millis(500),
        provisioner.create_managed(&request()),
    )
    .await;
    assert!(
        result.is_ok(),
        "provider created a billable resource but a follow-up read hid its identifier"
    );
    let created = result.unwrap().unwrap();
    assert_eq!(created.instance.id, "native-created-id");
    assert_eq!(state.lock().unwrap().creates, 1);
}

#[tokio::test]
async fn accepted_native_id_does_not_fabricate_actual_price_or_start_time() {
    let (server, _) = server(State {
        metadata_failure: true,
        ..Default::default()
    })
    .await;
    let created = provisioner(&server.uri())
        .create_managed(&request())
        .await
        .unwrap();
    assert_eq!(created.instance.id, "native-created-id");
    assert!(
        created.hourly_cost_usd.is_none(),
        "estimated quote was presented as a billed rate"
    );
    assert!(
        created.created_at.is_none(),
        "local time was presented as a provider start time"
    );
    assert!(created.endpoint.is_none());
}

#[tokio::test]
async fn retiring_a_lease_does_not_touch_a_different_namespace_with_the_same_lease_id() {
    let (server, state) = server(State {
        pods: vec![
            pod(
                "owned",
                "tangle-gpu-62bb8f79bcd9_lease-1",
                "NVIDIA B200",
                8,
                "review/worker:immutable",
            ),
            pod(
                "foreign",
                "tangle-gpu-057be051e1ef_lease-1",
                "NVIDIA B200",
                8,
                "review/worker:immutable",
            ),
        ],
        ..Default::default()
    })
    .await;
    provisioner(&server.uri())
        .destroy_managed(
            &CloudProvider::RunPod,
            &ManagedLeaseIdentity {
                namespace: "review:managed".into(),
                lease_id: "lease-1".into(),
            },
        )
        .await
        .unwrap();
    let state = state.lock().unwrap();
    assert_eq!(state.deletes, vec!["owned"]);
    assert_eq!(state.pods.len(), 1);
    assert_eq!(state.pods[0]["id"], "foreign");
}

#[tokio::test]
async fn managed_adoption_respects_region_selected_through_caller_configuration() {
    let (server, state) = server(State::default()).await;
    let mut spec = request();
    spec.region = None;
    let first = provisioner_region(&server.uri(), "EU")
        .create_managed(&spec)
        .await
        .unwrap();
    assert_eq!(first.instance.id, "native-created-id");
    assert!(
        provisioner_region(&server.uri(), "US")
            .create_managed(&spec)
            .await
            .is_err(),
        "changing the requested region in caller configuration silently adopted another region"
    );
    assert_eq!(state.lock().unwrap().creates, 1);
}

#[tokio::test]
async fn provider_401_echo_is_excluded_from_sdk_errors() {
    let server = MockServer::start().await;
    Mock::given(|_: &Request| true)
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_string("account-secret-review workload-secret-review"),
        )
        .mount(&server)
        .await;
    let error = provisioner(&server.uri())
        .list_managed("review:managed")
        .await
        .unwrap_err()
        .to_string();
    assert!(!error.contains("account-secret-review"));
    assert!(!error.contains("workload-secret-review"));
}
