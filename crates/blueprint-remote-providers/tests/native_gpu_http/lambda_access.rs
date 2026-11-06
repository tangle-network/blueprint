//! A selected existing key must actually provide the requested lease access.
use super::*;

fn public_key(byte: u8) -> String {
    use base64::Engine;
    let mut blob = Vec::new();
    for field in [b"ssh-ed25519".as_slice(), &[byte; 32]] {
        blob.extend((field.len() as u32).to_be_bytes());
        blob.extend(field);
    }
    format!(
        "ssh-ed25519 {}",
        base64::engine::general_purpose::STANDARD.encode(blob)
    )
}

async fn lambda_key_peer(registered: Option<String>) -> MockServer {
    let peer = MockServer::start().await;
    Mock::given(method("GET")).and(path("/instance-types"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{
            "gpu_8x_b200_sxm6":{"instance_type":{"name":"gpu_8x_b200_sxm6","gpu_description":"NVIDIA B200","price_cents_per_hour":6400,
                "specs":{"gpus":8,"memory_gib":2048,"vcpus":128,"storage_gib":4096}},"regions_with_capacity_available":[{"name":"us-test-1"}]}
        }}))).mount(&peer).await;
    Mock::given(method("GET")).and(path("/firewall-rulesets/global"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{"rules":[{"protocol":"tcp","port_range":[22,22],"source_network":"0.0.0.0/0"}]}}))).mount(&peer).await;
    Mock::given(method("GET"))
        .and(path("/instances"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[]})))
        .mount(&peer)
        .await;
    Mock::given(method("GET")).and(path("/ssh-keys"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":registered.map(|key|vec![json!({"id":"synthetic-key-id","name":"synthetic-key","public_key":key})]).unwrap_or_default()}))).mount(&peer).await;
    let native = std::sync::Arc::new(std::sync::Mutex::new(Value::Null));
    let created = native.clone();
    Mock::given(method("POST")).and(path("/instance-operations/launch"))
        .respond_with(move |request:&wiremock::Request| {
            let body:Value=request.body_json().unwrap();
            *created.lock().unwrap()=json!({"id":"synthetic-instance","status":"booting","tags":body["tags"],"ssh_key_names":body["ssh_key_names"],"region":{"name":body["region_name"]},"instance_type":{"name":body["instance_type_name"],"specs":{"gpus":8},"price_cents_per_hour":6400}});
            ResponseTemplate::new(200).set_body_json(json!({"data":{"instance_ids":["synthetic-instance"]}}))
        }).mount(&peer).await;
    Mock::given(method("GET"))
        .and(path("/instances/synthetic-instance"))
        .respond_with(move |_: &wiremock::Request| {
            ResponseTemplate::new(200).set_body_json(json!({"data":native.lock().unwrap().clone()}))
        })
        .mount(&peer)
        .await;
    peer
}

#[tokio::test]
async fn lambda_verifies_requested_public_key_before_launching() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let desired = public_key(17);
    for (registered, accepted) in [
        (Some(public_key(34)), false),
        (None, false),
        (Some("ssh-ed25519 not-base64".into()), false),
        (Some("ssh-ed25519 Zm9v".into()), false),
        (Some(format!("{desired} provider-comment")), true),
    ] {
        let peer = lambda_key_peer(registered).await;
        let credentials=serde_json::from_value(json!({"provider":"lambda-labs","api_key":"synthetic-local-peer-key","endpoint":peer.uri(),"ssh_key_name":"synthetic-key"})).unwrap();
        let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
        let mut spec = ResourceSpec::basic();
        spec.gpu_class = Some("nvidia-b200".into());
        spec.gpu_count = Some(8);
        spec.ownership = Some(ManagedLeaseIdentity {
            namespace: "synthetic-cell".into(),
            lease_id: "key-bound-lease".into(),
        });
        spec.lease_duration_secs = Some(3600);
        spec.ssh_public_key = Some(format!("{desired} caller-comment"));
        spec.validate().unwrap();
        let result = provisioner
            .provision_with_requirements(CloudProvider::LambdaLabs, &spec, "us-test-1", false)
            .await;
        if accepted {
            assert!(
                result.is_ok(),
                "a matching public key with a different comment must work: {result:?}"
            );
        } else {
            assert!(
                matches!(
                    result,
                    Err(blueprint_remote_providers::Error::Provider(
                        blueprint_remote_providers::managed::ProviderErrorCode::Unsupported
                    ))
                ),
                "an unrelated, missing or malformed key cannot provide the requested access: {result:?}"
            );
        }
        let requests = peer.received_requests().await.unwrap();
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.method == "POST"
                    && request.url.path() == "/instance-operations/launch")
                .count(),
            usize::from(accepted)
        );
        assert!(
            !requests
                .iter()
                .any(|request| request.method != "GET" && request.url.path() == "/ssh-keys")
        );
    }
}
