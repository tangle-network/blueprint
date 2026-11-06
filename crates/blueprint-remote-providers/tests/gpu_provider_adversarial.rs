//! Independent D12 checks through exported SDK operations and native HTTP peers.
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, InstanceStatus, ResourceSpec,
    infra::traits::CloudProviderAdapter, managed::ProviderCredentials,
    providers::BittensorLiumAdapter,
};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

#[derive(Default)]
struct PodState {
    pod: Option<Value>,
    creates: usize,
    deletes: usize,
}
#[derive(Clone)]
struct RunPodPeer(Arc<Mutex<PodState>>);
impl Respond for RunPodPeer {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        let mut state = self.0.lock().unwrap();
        match (request.method.as_str(), request.url.path()) {
            ("POST", "/graphql") => ResponseTemplate::new(200).set_body_json(json!({
                "data":{"gpuTypes":[{"id":"NVIDIA B200","lowestPrice":{
                    "uninterruptablePrice":2,"stockStatus":"High"}}]}
            })),
            ("POST", "/pods") => {
                let mut pod: Value = request.body_json().unwrap();
                pod["id"] = json!("review-owned");
                pod["gpuTypeId"] = pod["gpuTypeIds"][0].clone();
                pod["desiredStatus"] = json!("RUNNING");
                pod["costPerHr"] = json!(16);
                pod["lastStartedAt"] = json!("2026-11-06T00:00:00Z");
                state.creates += 1;
                state.pod = Some(pod);
                ResponseTemplate::new(201).set_body_json(json!({"id":"review-owned"}))
            }
            ("GET", "/pods") => ResponseTemplate::new(200)
                .set_body_json(json!(state.pod.iter().cloned().collect::<Vec<_>>())),
            ("GET", "/pods/review-owned") => match &state.pod {
                Some(pod) => ResponseTemplate::new(200).set_body_json(pod),
                None => ResponseTemplate::new(404),
            },
            ("DELETE", "/pods/review-owned") => {
                state.deletes += 1;
                state.pod = None;
                ResponseTemplate::new(204)
            }
            _ => ResponseTemplate::new(500),
        }
    }
}

fn spec() -> ResourceSpec {
    serde_json::from_value(json!({
        "provider":"runpod","cpu":4,"memory_gb":16,"storage_gb":200,
        "gpu_class":"nvidia-b200","gpu_count":8,"gpu_vram_gb":180,"allow_spot":false,
        "image":"review/worker:immutable","ports":[8443],"endpoint_port":8443,
        "env":{"TANGLE_GPU_WORKER_TOKEN":"synthetic-workload-secret"},
        "lease_duration_secs":300,"max_hourly_cost_usd":16,"region":"US",
        "ownership":{"namespace":"independent:d12","lease_id":"lease-1"}
    }))
    .unwrap()
}

async fn runpod() -> (MockServer, Arc<Mutex<PodState>>, CloudProvisioner) {
    let peer = MockServer::start().await;
    let state = Arc::new(Mutex::new(PodState::default()));
    Mock::given(|_: &Request| true)
        .respond_with(RunPodPeer(state.clone()))
        .mount(&peer)
        .await;
    let sdk = CloudProvisioner::from_credentials(HashMap::from([(
        CloudProvider::RunPod,
        ProviderCredentials {
            api_key: "synthetic-account-secret".into(),
            api_secret: None,
            rate_keys: None,
            endpoint: Some(peer.uri()),
            region: Some("US".into()),
        },
    )]))
    .unwrap();
    (peer, state, sdk)
}

#[tokio::test]
async fn identical_managed_runpod_retry_adopts_through_the_public_sdk() {
    let (_peer, state, sdk) = runpod().await;
    let request = spec();
    let first = sdk.create_managed(&request).await.unwrap();
    let retried = sdk
        .create_managed(&request)
        .await
        .expect("an identical SDK retry must adopt its tagged native pod");
    assert_eq!(first.instance.id, retried.instance.id);
    assert_eq!(state.lock().unwrap().creates, 1);
    let mut changed = request.clone();
    changed
        .env
        .insert("TANGLE_GPU_WORKER_TOKEN".into(), "different-secret".into());
    assert!(sdk.create_managed(&changed).await.is_err());
    sdk.destroy_managed(&CloudProvider::RunPod, request.ownership.as_ref().unwrap())
        .await
        .unwrap();
    let state = state.lock().unwrap();
    assert!(state.pod.is_none());
    assert_eq!(state.deletes, 1);
}

#[tokio::test]
async fn public_runpod_status_returns_the_native_https_endpoint_without_secrets() {
    let (peer, state, sdk) = runpod().await;
    let request = spec();
    let created = sdk.create_managed(&request).await.unwrap();
    let observed = sdk
        .status_managed(
            &CloudProvider::RunPod,
            request.ownership.as_ref().unwrap(),
            &created.instance.id,
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(observed.instance.status, InstanceStatus::Running);
    assert_eq!(
        observed.endpoint.as_deref(),
        Some("https://review-owned-8443.proxy.runpod.net")
    );
    let public = serde_json::to_string(&observed).unwrap();
    assert!(!public.contains("synthetic-workload-secret"));
    assert!(!public.contains("synthetic-account-secret"));
    let requests = peer.received_requests().await.unwrap();
    let create: Value = requests
        .iter()
        .find(|r| r.method.as_str() == "POST" && r.url.path() == "/pods")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(create["gpuCount"], 8);
    assert_eq!(create["imageName"], "review/worker:immutable");
    assert_eq!(create["ports"], json!(["8443/http"]));
    assert!(create.get("dockerEntrypoint").is_none());
    assert!(create.get("dockerStartCmd").is_none());
    assert!(
        create["env"]["TANGLE_GPU_REQUEST_FINGERPRINT"]
            .as_str()
            .is_some()
    );
    sdk.destroy_managed(&CloudProvider::RunPod, request.ownership.as_ref().unwrap())
        .await
        .unwrap();
    assert!(state.lock().unwrap().pod.is_none());
}

#[tokio::test]
async fn native_lium_host_outage_cannot_be_reported_as_a_ready_lease() {
    let peer = MockServer::start().await;
    Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/pods/offline"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id":"offline","status":"RUNNING","node_offline":true,
            "gpu_name":"B200","gpu_count":"8",
            "executor":{"executor_ip_address":"203.0.113.9","country_code":"US"}
        })))
        .mount(&peer)
        .await;
    let adapter =
        BittensorLiumAdapter::with_endpoint("synthetic-account-secret".into(), &peer.uri())
            .unwrap();
    assert_ne!(
        adapter.get_instance_status("offline").await.unwrap(),
        InstanceStatus::Running,
        "Lium preserves RUNNING during a native host outage; its node_offline flag is authoritative"
    );
}

#[tokio::test]
async fn lium_cannot_quote_an_unenforced_caller_network_policy() {
    let peer = MockServer::start().await;
    for (path, body) in [
        (
            "/public/v1/nodes",
            include_str!("fixtures/provider-api/lium-nodes-2026-11-06.json"),
        ),
        (
            "/pricing.json",
            include_str!("fixtures/provider-api/lium-pricing-2026-11-06.json"),
        ),
    ] {
        Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path(path))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::from_str::<Value>(body).unwrap()),
            )
            .mount(&peer)
            .await;
    }
    let adapter =
        BittensorLiumAdapter::with_endpoint("synthetic-account-secret".into(), &peer.uri())
            .unwrap();
    let mut request = spec();
    request.provider = Some(CloudProvider::BittensorLium);
    request.region = None;
    request.allow_spot = true;
    request.max_hourly_cost_usd = None;
    adapter
        .quote(&request, "")
        .await
        .expect("recorded stock must quote without a network restriction");
    request.source_network = Some("192.0.2.1/32".into());
    assert!(
        adapter.quote(&request, "").await.is_err(),
        "unsupported caller ingress restriction must be rejected rather than silently dropped"
    );
}

#[tokio::test]
async fn vast_cannot_quote_an_unenforced_caller_network_policy() {
    use blueprint_remote_providers::providers::vast_ai::VastAiAdapter;
    let peer = MockServer::start().await;
    Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/bundles/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"offers":[{
            "id":1,"gpu_name":"B200","num_gpus":8,"gpu_ram":184320,
            "cpu_cores_effective":128,"cpu_ram":1048576,"disk_space":8000,
            "dph_total":8,"reliability2":0.999,"verification":"verified",
            "rentable":true,"rented":false,"direct_port_count":32
        }]})))
        .mount(&peer)
        .await;
    let adapter =
        VastAiAdapter::with_endpoint("synthetic-account-secret".into(), &peer.uri()).unwrap();
    let mut request = spec();
    request.provider = Some(CloudProvider::VastAi);
    adapter
        .quote(&request, "")
        .await
        .expect("valid stock must quote without a network restriction");
    request.source_network = Some("192.0.2.1/32".into());
    assert!(
        adapter.quote(&request, "").await.is_err(),
        "unsupported caller ingress restriction must be rejected rather than silently dropped"
    );
}

#[tokio::test]
async fn vast_quote_reserves_every_distinct_workload_port() {
    use blueprint_remote_providers::providers::vast_ai::VastAiAdapter;
    let peer = MockServer::start().await;
    Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/bundles/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"offers":[{
            "id":1,"gpu_name":"B200","num_gpus":8,"gpu_ram":184320,
            "cpu_cores_effective":128,"cpu_ram":1048576,"disk_space":8000,
            "dph_total":8,"reliability2":0.999,"verification":"verified",
            "rentable":true,"rented":false,"direct_port_count":3
        }]})))
        .mount(&peer)
        .await;
    let adapter =
        VastAiAdapter::with_endpoint("synthetic-account-secret".into(), &peer.uri()).unwrap();
    let mut request = spec();
    request.provider = Some(CloudProvider::VastAi);
    request.ports = vec![8000, 8001];
    request.endpoint_port = Some(8443);
    adapter.quote(&request, "").await.unwrap();
    let requests = peer.received_requests().await.unwrap();
    let query: Value = requests[0].body_json().unwrap();
    assert_eq!(
        query["direct_port_count"]["gte"], 3,
        "price/stock search must reserve the same distinct mappings as native rent"
    );
}

#[tokio::test]
async fn vast_shared_a100_sku_cannot_substitute_forty_gb_for_eighty_gb() {
    use blueprint_remote_providers::providers::vast_ai::VastAiAdapter;
    let peer = MockServer::start().await;
    let vram = Arc::new(Mutex::new(81920u64));
    let observed = vram.clone();
    Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/bundles/"))
        .respond_with(move |_: &Request| {
            ResponseTemplate::new(200).set_body_json(json!({"offers":[{
                "id":1,"gpu_name":"A100 SXM4","num_gpus":8,"gpu_ram":*observed.lock().unwrap(),
                "cpu_cores_effective":128,"cpu_ram":1048576,"disk_space":8000,
                "dph_total":8,"reliability2":0.999,"verification":"verified",
                "rentable":true,"rented":false,"direct_port_count":3
            }]}))
        })
        .mount(&peer)
        .await;
    let adapter =
        VastAiAdapter::with_endpoint("synthetic-account-secret".into(), &peer.uri()).unwrap();
    let mut request = spec();
    request.provider = Some(CloudProvider::VastAi);
    request.gpu_class = Some("nvidia-a100-80gb".into());
    request.gpu_vram_gb = Some(80);
    adapter
        .quote(&request, "")
        .await
        .expect("native 80GB offer must remain selectable");
    *vram.lock().unwrap() = 40960;
    assert!(
        adapter.quote(&request, "").await.is_err(),
        "the shared A100 SXM4 SKU cannot prove VRAM; its native gpu_ram must satisfy the request"
    );
}
