#![cfg(feature = "remote-providers")]
use blueprint_sdk::remote::infra::traits::CloudProviderAdapter;
use blueprint_sdk::remote::providers::{PrimeIntellectAdapter, RunPodAdapter};
use blueprint_sdk::remote::{CloudProvider, CloudProvisioner, ResourceSpec};
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn sdk_exports_compare_providers_hands_over_access_and_expires() {
    let runpod = MockServer::start().await;
    let prime = MockServer::start().await;
    let retired = Arc::new(AtomicBool::new(false));
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"data":{"gpuTypes":[{
                "id":"NVIDIA B200","lowestPrice":{"uninterruptablePrice":2.0,"stockStatus":"High"}
            }]}})),
        )
        .mount(&runpod)
        .await;
    Mock::given(method("POST"))
        .and(path("/pods"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id":"sdk-eight"})))
        .expect(1)
        .mount(&runpod)
        .await;
    let is_retired = retired.clone();
    Mock::given(method("GET"))
        .and(path("/pods/sdk-eight"))
        .respond_with(move |_: &wiremock::Request| {
            if is_retired.load(Ordering::SeqCst) {
                ResponseTemplate::new(404)
            } else {
                ResponseTemplate::new(200).set_body_json(json!({
                    "id":"sdk-eight","gpuTypeId":"NVIDIA B200","desiredStatus":"RUNNING",
                    "publicIp":"203.0.113.8","portMappings":{"22":30222}
                }))
            }
        })
        .mount(&runpod)
        .await;
    let is_retired = retired.clone();
    Mock::given(method("DELETE"))
        .and(path("/pods/sdk-eight"))
        .respond_with(move |_: &wiremock::Request| {
            is_retired.store(true, Ordering::SeqCst);
            ResponseTemplate::new(204)
        })
        .expect(1)
        .mount(&runpod)
        .await;
    Mock::given(method("GET"))
        .and(path("/availability/gpus"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"items":[{
            "cloudId":"expensive-eight","gpuType":"B200_180GB","gpuCount":8,"gpuMemory":180,
            "socket":"SXM","provider":"runpod","stockStatus":"Available","security":"secure_cloud",
            "prices":{"onDemand":24.0,"currency":"USD"},
            "disk":{"defaultCount":100},"vcpu":{"defaultCount":4},"memory":{"defaultCount":16}
        }]})))
        .mount(&prime)
        .await;
    let store = tempfile::tempdir().unwrap();
    let providers = HashMap::from([
        (
            CloudProvider::RunPod,
            Box::new(RunPodAdapter::with_endpoint("account-secret".into(), &runpod.uri()).unwrap())
                as Box<dyn CloudProviderAdapter>,
        ),
        (
            CloudProvider::PrimeIntellect,
            Box::new(
                PrimeIntellectAdapter::with_endpoint("other-secret".into(), &prime.uri()).unwrap(),
            ) as Box<dyn CloudProviderAdapter>,
        ),
    ]);
    let cloud = CloudProvisioner::with_providers(providers).with_lease_store(store.path());
    let spec = ResourceSpec {
        gpu_class: Some("nvidia-b200".into()),
        gpu_count: Some(8),
        gpu_vram_gb: Some(180),
        image: Some("owner/ssh-workload:latest".into()),
        max_hourly_cost_usd: Some(20.0),
        ..ResourceSpec::recommended()
    };
    let quotes = cloud.quotes(&spec).await.unwrap();
    assert_eq!(quotes.len(), 1);
    assert_eq!(quotes[0].provider, CloudProvider::RunPod);
    let lease = cloud.allocate(&spec, Duration::from_secs(2)).await.unwrap();
    assert_eq!(lease.provider, CloudProvider::RunPod);
    assert_eq!(lease.hourly_cost_usd, 16.0);
    assert_eq!(lease.endpoint, "ssh://root@203.0.113.8:30222");
    assert!(
        lease
            .access
            .private_key()
            .unwrap()
            .contains("OPENSSH PRIVATE KEY")
    );
    assert!(!format!("{:?}", lease.access).contains("OPENSSH PRIVATE KEY"));
    let body: serde_json::Value = runpod
        .received_requests()
        .await
        .unwrap()
        .into_iter()
        .find(|request| request.method == "POST" && request.url.path() == "/pods")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(body["gpuCount"], 8);
    assert_eq!(body["env"]["PUBLIC_KEY"], lease.access.public_key);
    assert!(!body.to_string().contains("account-secret"));
    assert!(
        !prime
            .received_requests()
            .await
            .unwrap()
            .iter()
            .any(|request| request.method == "POST")
    );
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let records: serde_json::Value = serde_json::from_slice(
                &tokio::fs::read(store.path().join("deployment_state.json"))
                    .await
                    .unwrap(),
            )
            .unwrap();
            if retired.load(Ordering::SeqCst) && records.as_object().unwrap().is_empty() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    assert!(!lease.is_active().await.unwrap());
    lease.terminate().await.unwrap();
}
