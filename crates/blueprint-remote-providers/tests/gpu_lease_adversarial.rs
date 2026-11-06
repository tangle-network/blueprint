//! Lease lifecycle checks through the SDK and provider HTTP boundary.
use blueprint_remote_providers::deployment::tracker::{DeploymentRecord, DeploymentType};
use blueprint_remote_providers::infra::traits::CloudProviderAdapter;
use blueprint_remote_providers::providers::runpod::RunPodAdapter;
use blueprint_remote_providers::{CloudProvider, CloudProvisioner, ResourceSpec};
use chrono::{Duration as ChronoDuration, Utc};
use std::{collections::HashMap, time::Duration};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

fn provisioner(server: &MockServer, store: &std::path::Path) -> CloudProvisioner {
    let adapter =
        RunPodAdapter::with_endpoint("synthetic-review-key".into(), &server.uri()).unwrap();
    CloudProvisioner::with_providers(HashMap::from([(
        CloudProvider::RunPod,
        Box::new(adapter) as Box<dyn CloudProviderAdapter>,
    )]))
    .with_lease_store(store)
}

#[tokio::test]
async fn invalid_ttl_has_no_provider_effects() {
    let server = MockServer::start().await;
    let store = tempfile::tempdir().unwrap();
    let sdk = provisioner(&server, store.path());
    for seconds in [0, i64::MAX as u64, (i64::MAX / 1_000) as u64, u64::MAX] {
        let result = sdk
            .allocate(&ResourceSpec::default(), Duration::from_secs(seconds))
            .await;
        assert!(
            result.is_err(),
            "unrepresentable TTL must be rejected before provider effects"
        );
    }
    assert!(server.received_requests().await.unwrap().is_empty());
    assert!(!store.path().join("deployment_state.json").exists());
}

#[tokio::test]
async fn resumed_subsecond_deadline_retires_resource_and_removes_record() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/pods/review-pod"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/pods/review-pod"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    let store = tempfile::tempdir().unwrap();
    let mut record = DeploymentRecord::new(
        "review-lease".into(),
        DeploymentType::SshRemote,
        ResourceSpec::default(),
        None,
    );
    record.set_cloud_info(CloudProvider::RunPod, "".into());
    record.add_resource("instance_id".into(), "review-pod".into());
    record.expires_at = Some(Utc::now() + ChronoDuration::milliseconds(450));
    let state = store.path().join("deployment_state.json");
    tokio::fs::write(
        &state,
        serde_json::to_vec(&HashMap::from([("review-lease", record)])).unwrap(),
    )
    .await
    .unwrap();
    let sdk = provisioner(&server, store.path());
    sdk.resume_leases().await.unwrap();
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let bytes = tokio::fs::read(&state).await.unwrap();
            if let Ok(records) = serde_json::from_slice::<HashMap<String, DeploymentRecord>>(&bytes)
            {
                if records.is_empty() {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("resumed lease must retain its timer until the fractional deadline expires");
    let requests = server.received_requests().await.unwrap();
    assert!(
        requests
            .iter()
            .any(|request| request.method.as_str() == "DELETE")
    );
    assert!(
        requests
            .iter()
            .all(|request| request.headers.get("Authorization").unwrap()
                == "Bearer synthetic-review-key")
    );
}

#[tokio::test]
async fn caller_endpoint_never_forwards_account_credentials_across_origins() {
    use blueprint_remote_providers::security::{ApiAuthentication, SecureHttpClient};
    let gateway = MockServer::start().await;
    let outsider = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/redirect"))
        .respond_with(
            ResponseTemplate::new(302)
                .insert_header("Location", format!("{}/stolen", outsider.uri())),
        )
        .mount(&gateway)
        .await;
    let http = SecureHttpClient::with_endpoint(&gateway.uri()).unwrap();
    let auth = ApiAuthentication::Bearer {
        token: "synthetic-review-key".into(),
    };
    let response = http
        .get(&format!("{}/redirect", gateway.uri()), &auth)
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 302);
    assert!(
        http.get(&format!("{}/stolen", outsider.uri()), &auth)
            .await
            .is_err()
    );
    assert!(outsider.received_requests().await.unwrap().is_empty());
    for endpoint in [
        "http://example.invalid",
        "https://user:password@example.invalid",
        "file:///tmp/not-an-api",
    ] {
        assert!(SecureHttpClient::with_endpoint(endpoint).is_err());
    }
}

#[tokio::test]
async fn provider_error_details_do_not_return_echoed_account_credentials() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/pods/review-pod"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "error": "invalid credential: synthetic-echoed-account-secret"
        })))
        .mount(&server)
        .await;
    let adapter =
        RunPodAdapter::with_endpoint("synthetic-echoed-account-secret".into(), &server.uri())
            .unwrap();
    let error = adapter
        .get_instance_details("review-pod")
        .await
        .unwrap_err();
    assert!(
        !error
            .to_string()
            .contains("synthetic-echoed-account-secret"),
        "provider error text must not expose echoed credentials"
    );
}

#[tokio::test]
async fn lambda_never_quotes_forty_gb_hardware_for_an_eighty_gb_request() {
    use blueprint_remote_providers::providers::lambda_labs::LambdaLabsAdapter;
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/instance-types"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {"gpu_1x_a100": {
                "instance_type": {"name":"gpu_1x_a100", "description":"1x A100 (40 GB PCIe)",
                    "gpu_description":"A100 (40 GB PCIe)", "price_cents_per_hour":100,
                    "specs":{"gpus":1,"memory_gib":225,"vcpus":30,"storage_gib":512}},
                "regions_with_capacity_available":[{"name":"us-west-1"}]
            }}
        })))
        .mount(&server)
        .await;
    let adapter = LambdaLabsAdapter::with_endpoint("synthetic-key".into(), &server.uri()).unwrap();
    let spec: ResourceSpec = serde_json::from_value(serde_json::json!({
        "cpu":4, "memory_gb":16, "storage_gb":100, "allow_spot":false,
        "gpu_class":"nvidia-a100-80gb", "gpu_count":1, "gpu_vram_gb":80
    }))
    .unwrap();
    assert!(
        adapter.quote(&spec, "us-west-1").await.is_err(),
        "40GB Lambda listing must not satisfy an80GB GPU request"
    );
}

#[tokio::test]
async fn vast_quote_uses_bearer_auth_and_top_level_offer_filters() {
    use blueprint_remote_providers::providers::vast_ai::VastAiAdapter;
    use wiremock::matchers::{header, path_regex};
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex("^/bundles/?$"))
        .and(header("Authorization", "Bearer synthetic-review-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "offers":[{"id":7,"dph_total":1.0,"reliability2":0.999,
                "gpu_name":"B200","num_gpus":8,"gpu_ram":184320,"rentable":true,"rented":false}]
        })))
        .mount(&server)
        .await;
    let adapter =
        VastAiAdapter::with_endpoint("synthetic-review-key".into(), &server.uri()).unwrap();
    let spec: ResourceSpec = serde_json::from_value(serde_json::json!({
        "cpu":4, "memory_gb":16, "storage_gb":100, "allow_spot":false,
        "gpu_class":"nvidia-b200", "gpu_count":8, "gpu_vram_gb":180
    }))
    .unwrap();
    let quote = adapter
        .quote(&spec, "")
        .await
        .expect("documented Vast request must be quotable");
    assert_eq!(quote.estimated_hourly_cost, Some(1.0));
    let requests = server.received_requests().await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert!(
        body.get("q").is_none(),
        "Vast POST search filters must be top-level"
    );
    assert_eq!(body["gpu_name"]["eq"], "B200");
    assert_eq!(body["num_gpus"]["eq"], 8);
}

#[tokio::test]
async fn cancelled_allocation_keeps_its_provider_retirement_obligation() {
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
    let server = MockServer::start().await;
    let created = Arc::new(AtomicBool::new(false));
    let deleted = Arc::new(AtomicBool::new(false));
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data":{"gpuTypes":[{"id":"NVIDIA B200", "lowestPrice":{
                "uninterruptablePrice":1.0, "stockStatus":"High"}}]}
        })))
        .mount(&server)
        .await;
    let created_by_peer = created.clone();
    Mock::given(method("POST"))
        .and(path("/pods"))
        .respond_with(move |_: &wiremock::Request| {
            created_by_peer.store(true, Ordering::SeqCst);
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"id":"cancelled-pod"}))
                .set_delay(Duration::from_millis(300))
        })
        .mount(&server)
        .await;
    let deleted_by_peer = deleted.clone();
    Mock::given(method("DELETE"))
        .and(path("/pods/cancelled-pod"))
        .respond_with(move |_: &wiremock::Request| {
            deleted_by_peer.store(true, Ordering::SeqCst);
            ResponseTemplate::new(204)
        })
        .mount(&server)
        .await;
    let retired = deleted.clone();
    Mock::given(method("GET"))
        .and(path("/pods/cancelled-pod"))
        .respond_with(move |_: &wiremock::Request| {
            if retired.load(Ordering::SeqCst) {
                ResponseTemplate::new(404)
            } else {
                ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "id":"cancelled-pod", "desiredStatus":"RUNNING", "publicIp":"203.0.113.20",
                    "gpuTypeId":"NVIDIA B200", "portMappings":{"22":12345}
                }))
            }
        })
        .mount(&server)
        .await;
    let store = tempfile::tempdir().unwrap();
    let sdk = Arc::new(provisioner(&server, store.path()));
    let spec: ResourceSpec = serde_json::from_value(serde_json::json!({
        "cpu":4, "memory_gb":16, "storage_gb":100, "allow_spot":false,
        "gpu_class":"nvidia-b200", "gpu_count":8, "gpu_vram_gb":180
    }))
    .unwrap();
    let allocating = sdk.clone();
    let task =
        tokio::spawn(async move { allocating.allocate(&spec, Duration::from_secs(1)).await });
    tokio::time::timeout(Duration::from_secs(3), async {
        while !created.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("provider must receive create before cancelling caller");
    task.abort();
    assert!(matches!(task.await, Err(error) if error.is_cancelled()));
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let state = tokio::fs::read(store.path().join("deployment_state.json")).await;
            let removed = state.ok().and_then(|bytes| serde_json::from_slice::<HashMap<String, DeploymentRecord>>(&bytes).ok())
                .is_some_and(|records| records.is_empty());
            if deleted.load(Ordering::SeqCst) && removed { break; }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }).await.expect("cancelling an allocation must retain cleanup responsibility for the created provider resource");
}
