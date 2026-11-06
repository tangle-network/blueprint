//! Provider rules are a union: an unrelated global rule can widen VM ingress.
use super::*;

#[tokio::test]
async fn lambda_refuses_global_rules_outside_requested_ingress_fence() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = MockServer::start().await;
    Mock::given(method("GET")).and(path("/instance-types"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{
            "gpu_8x_b200_sxm6":{"instance_type":{"name":"gpu_8x_b200_sxm6","price_cents_per_hour":6400,
                "specs":{"gpus":8,"memory_gib":2048,"vcpus":128,"storage_gib":4096}},
                "regions_with_capacity_available":[{"name":"us-test-1"}]}
        }}))).mount(&peer).await;
    Mock::given(method("GET"))
        .and(path("/firewall-rulesets/global"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"data":{"id":"global","rules":[
                {"protocol":"tcp","port_range":[22,22],"source_network":"192.0.2.0/24"},
                {"protocol":"tcp","port_range":[8000,8000],"source_network":"192.0.2.0/24"},
                {"protocol":"tcp","port_range":[9000,9000],"source_network":"0.0.0.0/0"}
            ]}}),
        ))
        .mount(&peer)
        .await;
    let credentials = serde_json::from_value(
        json!({"provider":"lambda-labs","api_key":"synthetic-provider-key","endpoint":peer.uri()}),
    )
    .unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let mut spec = ResourceSpec::basic();
    spec.gpu_class = Some("nvidia-b200".into());
    spec.gpu_count = Some(8);
    spec.source_network = Some("192.0.2.0/24".into());
    spec.ports = vec![8000];
    spec.validate().unwrap();
    let result = provisioner.quotes(&spec).await.unwrap();
    assert!(
        result.quotes.is_empty(),
        "a narrower requested ingress fence cannot erase a broader global rule on another VM port"
    );
    assert_eq!(
        result.errors[0].code,
        blueprint_remote_providers::managed::ProviderErrorCode::Unsupported
    );
    assert!(
        peer.received_requests()
            .await
            .unwrap()
            .iter()
            .all(|request| request.method == "GET")
    );
}
