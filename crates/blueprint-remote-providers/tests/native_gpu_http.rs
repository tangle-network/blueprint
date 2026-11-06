//! Public SDK requests against synthetic provider HTTP peers; no stock or live-rental claim.
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, ResourceSpec, managed::ManagedLeaseIdentity,
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn runpod_receives_the_requested_gpu_shape_and_volume() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/pods"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id":"synthetic-pod"})))
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("GET"))
        .and(path("/pods/synthetic-pod"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id":"synthetic-pod", "desiredStatus":"RUNNING", "publicIp":"192.0.2.10",
            "gpuTypeId":"NVIDIA H200", "gpuCount":2, "dataCenterId":"US-TEST-1"
        })))
        .mount(&peer)
        .await;
    let credentials = serde_json::from_value(json!({
        "provider":"runpod", "api_key":"synthetic-local-peer-key", "endpoint":peer.uri()
    }))
    .unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let mut spec = ResourceSpec::basic();
    spec.ownership = Some(ManagedLeaseIdentity {
        namespace: "synthetic-cell".into(),
        lease_id: "synthetic-lease".into(),
    });
    spec.lease_duration_secs = Some(3600);
    spec.gpu_class = Some("nvidia-h200".into());
    spec.gpu_count = Some(2);
    spec.gpu_vram_gb = Some(140);
    spec.volume_gb = Some(37.0);
    spec.image = Some("example.invalid/workload@sha256:synthetic".into());
    let result = provisioner
        .provision_with_requirements(CloudProvider::RunPod, &spec, "US-TEST-1", false)
        .await
        .unwrap();
    assert_eq!(result.id, "synthetic-pod");
    let requests = peer.received_requests().await.unwrap();
    let launch: Value = requests
        .iter()
        .find(|request| request.method == "POST")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(
        json!({
            "gpuTypeIds":launch["gpuTypeIds"], "gpuCount":launch["gpuCount"],
            "volumeInGb":launch["volumeInGb"], "imageName":launch["imageName"]
        }),
        json!({
            "gpuTypeIds":["NVIDIA H200"], "gpuCount":2, "volumeInGb":37,
            "imageName":"example.invalid/workload@sha256:synthetic"
        }),
        "provider launch must preserve the caller's class, count, volume and workload image"
    );
}
