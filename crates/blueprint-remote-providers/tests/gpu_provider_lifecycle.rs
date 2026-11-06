use blueprint_remote_providers::{
    ResourceSpec, infra::traits::CloudProviderAdapter, providers::RunPodAdapter,
};
use serde_json::{Value, json};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn runpod_retry_adopts_the_owned_gpu_and_preserves_the_image_entrypoint_and_tls_proxy() {
    let peer = MockServer::start().await;
    let exists = Arc::new(AtomicBool::new(false));
    let state = Arc::new(std::sync::Mutex::new(
        json!({"id":"ownedpod","name":"tangle-gpu-7e0aae0840d7_gpu_lease123","gpuTypeId":"NVIDIA B200","gpuCount":8,
        "imageName":"owner/https-health:immutable","desiredStatus":"RUNNING","ports":["8443/http"],"costPerHr":16.0,"lastStartedAt":"2026-11-06T00:00:00Z"}),
    ));
    let listed = exists.clone();
    let listed_state = state.clone();
    Mock::given(method("GET"))
        .and(path("/pods"))
        .respond_with(move |_: &wiremock::Request| {
            ResponseTemplate::new(200).set_body_json(if listed.load(Ordering::SeqCst) {
                json!([listed_state.lock().unwrap().clone()])
            } else {
                json!([])
            })
        })
        .mount(&peer)
        .await;
    let created_state = state.clone();
    Mock::given(method("POST"))
        .and(path("/pods"))
        .respond_with(move |request: &wiremock::Request| {
            let body: Value = request.body_json().unwrap();
            created_state.lock().unwrap()["env"] = body["env"].clone();
            exists.store(true, Ordering::SeqCst);
            ResponseTemplate::new(201).set_body_json(json!({"id":"ownedpod"}))
        })
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("GET"))
        .and(path("/pods/ownedpod"))
        .respond_with(move |_: &wiremock::Request| {
            ResponseTemplate::new(200).set_body_json(state.lock().unwrap().clone())
        })
        .mount(&peer)
        .await;
    let spec:ResourceSpec=serde_json::from_value(json!({"cpu":4.0,"memory_gb":16.0,"storage_gb":200.0,
        "gpu_class":"nvidia-b200","gpu_count":8,"gpu_vram_gb":180,"allow_spot":false,
        "image":"owner/https-health:immutable","env":{"TANGLE_GPU_WORKER_TOKEN":"synthetic-worker-secret"},"ports":[8443],"endpoint_port":8443,
        "ownership":{"namespace":"production:cell1","lease_id":"gpu_lease123"}})).unwrap();
    let adapter =
        RunPodAdapter::with_endpoint("synthetic-account-secret".into(), &peer.uri()).unwrap();
    let first = adapter
        .provision_resources("NVIDIA B200", "US", &spec, false)
        .await
        .unwrap();
    let requests = peer.received_requests().await.unwrap();
    let create: Value = requests
        .iter()
        .find(|r| r.method == wiremock::http::Method::POST)
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(create["name"], "tangle-gpu-7e0aae0840d7_gpu_lease123");
    assert_eq!(create["containerDiskInGb"], 200);
    assert_eq!(create["countryCodes"], json!(["US"]));
    assert_eq!(create["ports"], json!(["8443/http"]));
    assert_eq!(
        create["env"]["TANGLE_GPU_WORKER_TOKEN"],
        "synthetic-worker-secret"
    );
    assert!(create.get("dockerEntrypoint").is_none());
    assert!(create.get("dockerStartCmd").is_none());
    let second = adapter
        .provision_resources("NVIDIA B200", "US", &spec, false)
        .await
        .unwrap();
    assert_eq!(first.id, second.id);
    assert_eq!(
        adapter
            .application_endpoint(&second, &spec)
            .await
            .unwrap()
            .as_deref(),
        Some("https://ownedpod-8443.proxy.runpod.net")
    );
    let resources = adapter.list_managed("production:cell1").await.unwrap();
    assert_eq!(resources.len(), 1);
    assert_eq!(resources[0].lease_id, "gpu_lease123");
    assert_eq!(resources[0].hourly_cost_usd, Some(16.0));
    assert!(adapter.list_managed("other:cell").await.unwrap().is_empty());
}

#[tokio::test]
async fn lium_real_inventory_quotes_exact_eight_blackwell_gpus_and_respects_tier_and_stock() {
    use blueprint_remote_providers::providers::BittensorLiumAdapter;
    let peer = MockServer::start().await;
    let nodes: Value = serde_json::from_str(include_str!(
        "fixtures/provider-api/lium-nodes-2026-11-06.json"
    ))
    .unwrap();
    let pricing: Value = serde_json::from_str(include_str!(
        "fixtures/provider-api/lium-pricing-2026-11-06.json"
    ))
    .unwrap();
    Mock::given(method("GET"))
        .and(path("/public/v1/nodes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(nodes))
        .mount(&peer)
        .await;
    Mock::given(method("GET"))
        .and(path("/pricing.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(pricing))
        .mount(&peer)
        .await;
    let adapter =
        BittensorLiumAdapter::with_endpoint("synthetic-account-secret".into(), &peer.uri())
            .unwrap();
    let spec: ResourceSpec =
        serde_json::from_value(json!({"cpu":4.0,"memory_gb":16.0,"storage_gb":200.0,
        "gpu_class":"nvidia-b200","gpu_count":8,"gpu_vram_gb":180,"allow_spot":false}))
        .unwrap();
    let secure = adapter.quote(&spec, "").await.unwrap();
    assert_eq!(secure.estimated_hourly_cost, Some(48.24));
    let pick: Value = serde_json::from_str(&secure.instance_type).unwrap();
    assert_eq!(pick["executor_id"], "b5864202-5da2-4707-886b-4805fecb4e3c");
    let mut spot = spec.clone();
    spot.allow_spot = true;
    assert_eq!(
        adapter
            .quote(&spot, "")
            .await
            .unwrap()
            .estimated_hourly_cost,
        Some(44.0)
    );
    spot.gpu_class = Some("nvidia-b300".into());
    spot.gpu_vram_gb = Some(288);
    assert_eq!(
        adapter
            .quote(&spot, "")
            .await
            .unwrap()
            .estimated_hourly_cost,
        Some(51.2)
    );
    spot.allow_spot = false;
    assert!(
        adapter.quote(&spot, "").await.is_err(),
        "a spot node cannot satisfy a secure request"
    );
    spot.gpu_count = Some(4);
    spot.gpu_class = Some("nvidia-gb300".into());
    assert!(
        adapter.quote(&spot, "").await.is_err(),
        "a reference price without a rentable node must not be quoted"
    );
    assert!(
        peer.received_requests()
            .await
            .unwrap()
            .iter()
            .all(|r| r.method == wiremock::http::Method::GET)
    );
}

#[tokio::test]
async fn vast_recorded_eight_b200_offer_uses_args_mode_tls_and_recoverable_ownership() {
    use blueprint_remote_providers::providers::VastAiAdapter;
    let peer = MockServer::start().await;
    let offers: Value = serde_json::from_str(include_str!(
        "fixtures/provider-api/vast-b200-2026-11-06.json"
    ))
    .unwrap();
    Mock::given(method("POST"))
        .and(path("/bundles/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(offers))
        .mount(&peer)
        .await;
    let created = Arc::new(AtomicBool::new(false));
    let environment = Arc::new(std::sync::Mutex::new(json!({})));
    let listing = created.clone();
    let listed_env = environment.clone();
    Mock::given(method("GET")).and(path("/instances/"))
        .respond_with(move |_:&wiremock::Request|ResponseTemplate::new(200).set_body_json(json!({"instances":if listing.load(Ordering::SeqCst){vec![json!({
            "id":54321,"label":"tangle-gpu-7e0aae0840d7_gpu_lease123","gpu_name":"B200","num_gpus":8,
            "actual_status":"running","public_ipaddr":"203.0.113.8","ports":{"8443/tcp":[{"HostPort":"30443"}]},
            "env":listed_env.lock().unwrap().clone(),"dph_total":80.05208333333333,"start_date":1793920000
        })]}else{vec![]}}))).mount(&peer).await;
    let rent = created.clone();
    let rented_env = environment.clone();
    Mock::given(method("PUT"))
        .and(path("/asks/58977952/"))
        .respond_with(move |request: &wiremock::Request| {
            let body: Value = request.body_json().unwrap();
            *rented_env.lock().unwrap() = body["env"].clone();
            rent.store(true, Ordering::SeqCst);
            ResponseTemplate::new(200).set_body_json(json!({"new_contract":54321}))
        })
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/instances/54321/"))
        .respond_with(move |_: &wiremock::Request| {
            created.store(false, Ordering::SeqCst);
            ResponseTemplate::new(204)
        })
        .expect(1)
        .mount(&peer)
        .await;
    let pair = rcgen::generate_simple_self_signed(vec!["tangle-gpu-worker".into()]).unwrap();
    let spec:ResourceSpec=serde_json::from_value(json!({"cpu":4.0,"memory_gb":16.0,"storage_gb":200.0,"gpu_class":"nvidia-b200","gpu_count":8,"gpu_vram_gb":180,"allow_spot":false,
        "image":"owner/https-health:immutable","endpoint_port":8443,"ports":[8443],"ownership":{"namespace":"production:cell1","lease_id":"gpu_lease123"},
        "env":{"TANGLE_GPU_WORKER_TOKEN":"synthetic-worker-secret","TANGLE_GPU_WORKER_TLS_CERT":pair.cert.pem(),"TANGLE_GPU_WORKER_TLS_KEY":pair.key_pair.serialize_pem()}})).unwrap();
    let strict =
        VastAiAdapter::with_endpoint("synthetic-account-secret".into(), &peer.uri()).unwrap();
    assert!(
        strict.quote(&spec, "").await.is_err(),
        "recorded unverified host does not satisfy the default trust policy"
    );
    let adapter = strict.with_offer_policy(0.93, false).unwrap();
    let quote = adapter.quote(&spec, "").await.unwrap();
    assert_eq!(quote.gpu_class.as_deref(), Some("nvidia-b200"));
    assert_eq!(quote.estimated_hourly_cost, Some(80.05208333333333));
    let first = adapter
        .provision_resources(&quote.instance_type, "", &spec, false)
        .await
        .unwrap();
    let second = adapter
        .provision_resources(&quote.instance_type, "", &spec, false)
        .await
        .unwrap();
    assert_eq!(first.id, second.id);
    assert_eq!(
        adapter
            .application_endpoint(&second, &spec)
            .await
            .unwrap()
            .as_deref(),
        Some("https://203.0.113.8:30443")
    );
    let managed = adapter.list_managed("production:cell1").await.unwrap();
    assert_eq!(managed.len(), 1);
    assert_eq!(managed[0].lease_id, "gpu_lease123");
    assert_eq!(
        managed[0].endpoint.as_deref(),
        Some("https://203.0.113.8:30443")
    );
    assert!(managed[0].request_fingerprint.is_some());
    assert!(adapter.list_managed("other:cell").await.unwrap().is_empty());
    let requests = peer.received_requests().await.unwrap();
    let create: Value = requests
        .iter()
        .find(|r| r.method == wiremock::http::Method::PUT)
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(create["runtype"], "args");
    assert_eq!(create["disk"], 200.0);
    assert!(
        create.get("args").is_none(),
        "custom image keeps its own CMD and ENTRYPOINT"
    );
    assert_eq!(create["env"]["-p 8443:8443"], "1");
    assert_eq!(
        create["env"]["TANGLE_GPU_WORKER_TOKEN"],
        "synthetic-worker-secret"
    );
    assert!(!create.to_string().contains("synthetic-account-secret"));
    adapter.terminate_instance(&first.id).await.unwrap();
    assert_eq!(
        adapter.get_instance_status(&first.id).await.unwrap(),
        blueprint_remote_providers::InstanceStatus::Terminated
    );
}
