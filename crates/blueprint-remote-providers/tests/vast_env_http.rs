//! Public request against a synthetic native peer; no live secret or stock claim.
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, ResourceSpec, managed::ManagedLeaseIdentity,
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn vast_preserves_the_callers_native_environment_map() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/instances"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"instances":[],"next_token":null})),
        )
        .mount(&peer)
        .await;
    Mock::given(method("POST")).and(path("/bundles/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"offers":[{
            "id":7001,"gpu_name":"B200","num_gpus":8,"dph_total":48.0,
            "reliability2":0.999,"verified":true,"rentable":true,"rented":false,
            "cpu_cores_effective":128,"cpu_ram":2048000,"disk_space":4096,"geolocation":"synthetic-region"
        }]}))).mount(&peer).await;
    Mock::given(method("PUT"))
        .and(path("/asks/7001/"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success":true,"new_contract":8001})),
        )
        .mount(&peer)
        .await;
    let credentials = serde_json::from_value(
        json!({"provider":"vast-ai","api_key":"synthetic-provider-key","endpoint":peer.uri()}),
    )
    .unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let mut spec = ResourceSpec::basic();
    spec.gpu_class = Some("nvidia-b200".into());
    spec.gpu_count = Some(8);
    spec.image = Some("example.invalid/workload:v1".into());
    spec.ownership = Some(ManagedLeaseIdentity {
        namespace: "synthetic-cell".into(),
        lease_id: "native-env".into(),
    });
    spec.lease_duration_secs = Some(3600);
    spec.ports = vec![8000];
    spec.env.insert(
        "CALLER_CONFIG".into(),
        "literal spaces = $token; 'quote'".into(),
    );
    spec.command = Some(vec!["--listen".into(), "0.0.0.0:8000".into()]);
    spec.validate().unwrap();
    provisioner
        .provision_with_requirements(CloudProvider::VastAi, &spec, "synthetic-region", false)
        .await
        .unwrap();
    let requests = peer.received_requests().await.unwrap();
    let rent: Value = requests
        .iter()
        .find(|request| request.method == "PUT")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(
        rent["env"],
        json!({"CALLER_CONFIG":"literal spaces = $token; 'quote'","-p 8000:8000":"1"})
    );
    assert_eq!(rent["args"], json!(["--listen", "0.0.0.0:8000"]));
    assert!(!rent.to_string().contains("synthetic-provider-key"));
}
