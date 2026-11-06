//! A synthetic own-SKU listing proves flexible counts, not current Lambda stock.
use super::*;

#[tokio::test]
async fn lambda_accepts_an_own_three_gpu_sku_with_known_gpu_description() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = MockServer::start().await;
    Mock::given(method("GET")).and(path("/instance-types"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{
            "synthetic_own_3x_b200":{"instance_type":{"name":"synthetic_own_3x_b200","gpu_description":"NVIDIA B200","price_cents_per_hour":2400,
                "specs":{"gpus":3,"memory_gib":384,"vcpus":12,"storage_gib":1024}},
                "regions_with_capacity_available":[{"name":"us-test-1"}]}
        }}))).mount(&peer).await;
    Mock::given(method("GET"))
        .and(path("/firewall-rulesets/global"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"data":{"id":"global","rules":[
                {"protocol":"tcp","port_range":[22,22],"source_network":"0.0.0.0/0"}
            ]}}),
        ))
        .mount(&peer)
        .await;
    Mock::given(method("GET"))
        .and(path("/instances"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[]})))
        .mount(&peer)
        .await;
    use std::sync::{Arc, Mutex};
    use wiremock::Request;
    let native = Arc::new(Mutex::new(Value::Null));
    let launched = native.clone();
    Mock::given(method("POST")).and(path("/instance-operations/launch"))
        .respond_with(move |request:&Request|{
            let body:Value=request.body_json().unwrap();
            *launched.lock().unwrap()=json!({"id":"synthetic-three","tags":body["tags"],"status":"active","ip":"192.0.2.40",
                "region":{"name":"us-test-1"},"instance_type":{"name":body["instance_type_name"],"gpu_description":"NVIDIA B200","price_cents_per_hour":2400,"specs":{"gpus":3}}});
            ResponseTemplate::new(200).set_body_json(json!({"data":{"instance_ids":["synthetic-three"]}}))
        }).mount(&peer).await;
    Mock::given(method("GET"))
        .and(path("/instances/synthetic-three"))
        .respond_with(move |_: &Request| {
            ResponseTemplate::new(200).set_body_json(json!({"data":native.lock().unwrap().clone()}))
        })
        .mount(&peer)
        .await;
    let credentials=serde_json::from_value(json!({"provider":"lambda-labs","api_key":"synthetic-local-peer-key","endpoint":peer.uri(),"ssh_key_name":"synthetic-key"})).unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let mut spec = ResourceSpec::basic();
    spec.gpu_class = Some("nvidia-b200".into());
    spec.gpu_count = Some(3);
    spec.gpu_vram_gb = Some(180);
    spec.ownership = Some(ManagedLeaseIdentity {
        namespace: "synthetic-cell".into(),
        lease_id: "own-three".into(),
    });
    spec.lease_duration_secs = Some(3600);
    spec.validate().unwrap();
    let instance = provisioner
        .provision_with_requirements(CloudProvider::LambdaLabs, &spec, "us-test-1", false)
        .await
        .unwrap();
    let requests = peer.received_requests().await.unwrap();
    let launch: Value = requests
        .iter()
        .find(|request| request.method == "POST")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(launch["instance_type_name"], "synthetic_own_3x_b200");
    assert_eq!(launch["quantity"], 1);
    let status = provisioner
        .status_managed(
            &CloudProvider::LambdaLabs,
            spec.ownership.as_ref().unwrap(),
            &instance.id,
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(status.gpu_class.as_deref(), Some("nvidia-b200"));
    assert_eq!(status.gpu_count, Some(3));
    assert_eq!(status.gpu_vram_gb, Some(180));
}
