//! Public SDK requests against HTTP peers; no current-stock or live-rental claim.
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

#[tokio::test]
async fn lambda_cannot_turn_three_requested_gpus_into_an_eight_gpu_launch() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/instance-types"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{
            "gpu_8x_b200_sxm6":{"instance_type":{"name":"gpu_8x_b200_sxm6",
                "gpu_description":"NVIDIA B200", "price_cents_per_hour":6400,
                "specs":{"gpus":8,"memory_gib":2048,"vcpus":128,"storage_gib":4096}},
                "regions_with_capacity_available":[{"name":"us-test-1"}]}
        }})))
        .mount(&peer)
        .await;
    Mock::given(method("POST"))
        .and(path("/instance-operations/launch"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":{"instance_ids":["synthetic-lambda"]}})),
        )
        .mount(&peer)
        .await;
    Mock::given(method("GET"))
        .and(path("/instances/synthetic-lambda"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{
            "id":"synthetic-lambda","status":"active","ip":"192.0.2.11",
            "instance_type":{"name":"gpu_8x_h100_sxm"},"region":{"name":"us-test-1"}
        }})))
        .mount(&peer)
        .await;
    let credentials = serde_json::from_value(json!({"provider":"lambda-labs",
        "api_key":"synthetic-local-peer-key","endpoint":peer.uri(),"ssh_key_name":"synthetic-key"
    }))
    .unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let mut spec = ResourceSpec::basic();
    spec.gpu_class = Some("nvidia-b200".into());
    spec.gpu_count = Some(3);
    spec.gpu_vram_gb = Some(180);
    spec.ownership = Some(ManagedLeaseIdentity {
        namespace: "synthetic-cell".into(),
        lease_id: "lambda-three".into(),
    });
    spec.lease_duration_secs = Some(3600);
    let result = provisioner
        .provision_with_requirements(CloudProvider::LambdaLabs, &spec, "us-test-1", false)
        .await;
    assert!(
        matches!(
            result,
            Err(blueprint_remote_providers::Error::Provider(
                blueprint_remote_providers::managed::ProviderErrorCode::NoStock
            ))
        ),
        "a native listing with only eight-GPU SKUs cannot satisfy a three-GPU request: {result:?}"
    );
    assert!(
        !peer
            .received_requests()
            .await
            .unwrap()
            .iter()
            .any(|request| request.method == "POST"),
        "an unsupported GPU shape must not launch a different count"
    );
}

#[tokio::test]
async fn vast_search_and_bid_preserve_native_query_fields_and_requested_class() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/bundles/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"offers":[{
            "id":7001,"gpu_name":"B200","num_gpus":8,"dph_total":0.5,
            "min_bid":0.4,"reliability":0.999,"reliability2":0.999,
            "verified":true,"verification":"verified","rentable":true,"rented":false,
            "gpu_ram":180000,"geolocation":"synthetic-region"
        }]})))
        .mount(&peer)
        .await;
    Mock::given(method("PUT"))
        .and(path("/asks/7001/"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success":true,"new_contract":8001})),
        )
        .mount(&peer)
        .await;
    Mock::given(method("GET"))
        .and(path("/instances/"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"instances":[{
                "id":8001,"gpu_name":"B200","num_gpus":8,"actual_status":"running",
                "public_ipaddr":"192.0.2.12","geolocation":"synthetic-region"
            }]})),
        )
        .mount(&peer)
        .await;
    let credentials = serde_json::from_value(json!({"provider":"vast-ai",
        "api_key":"synthetic-local-peer-key","endpoint":peer.uri()
    }))
    .unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let mut spec = ResourceSpec::basic();
    spec.gpu_class = Some("nvidia-b200".into());
    spec.gpu_count = Some(8);
    spec.gpu_vram_gb = Some(180);
    spec.allow_spot = true;
    spec.ownership = Some(ManagedLeaseIdentity {
        namespace: "synthetic-cell".into(),
        lease_id: "vast-bid".into(),
    });
    spec.lease_duration_secs = Some(3600);
    spec.image = Some("example.invalid/synthetic-image:v1".into());
    provisioner
        .provision_with_requirements(CloudProvider::VastAi, &spec, "synthetic-region", false)
        .await
        .unwrap();
    let requests = peer.received_requests().await.unwrap();
    let search: Value = requests
        .iter()
        .find(|r| r.method == "POST")
        .unwrap()
        .body_json()
        .unwrap();
    let rent: Value = requests
        .iter()
        .find(|r| r.method == "PUT")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(
        json!({"nested_query":search.get("q"),"gpu_name":search["gpu_name"],
        "num_gpus":search["num_gpus"],"type":search["type"],"bid":rent["price"],"image":rent["image"]}),
        json!({"nested_query":null,"gpu_name":{"eq":"B200"},"num_gpus":{"eq":8},
            "type":"bid","bid":0.4,"image":"example.invalid/synthetic-image:v1"}),
        "synthetic native peer prices are test inputs, never observed stock or a public price table"
    );
}

#[tokio::test]
async fn lium_rents_the_recorded_executor_with_the_requested_count_and_lifetime() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = MockServer::start().await;
    // https://lium.io/api/public/v1/nodes, captured 2026-11-06T03:55:56Z; SHA25600475806e9408e56046e04ad29f1056876941fd771de28e28c26782cdcb11828.
    // Create/list replies below are synthetic; this is no claim that stock remains available.
    let nodes: Value = serde_json::from_str(include_str!(
        "fixtures/provider-api/lium-nodes-2026-11-06.json"
    ))
    .unwrap();
    Mock::given(method("GET"))
        .and(path("/public/v1/nodes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(nodes))
        .mount(&peer)
        .await;
    Mock::given(method("POST"))
        .and(path("/rentals"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"rental_id":"synthetic-lium"})),
        )
        .mount(&peer)
        .await;
    Mock::given(method("GET"))
        .and(path("/rentals/synthetic-lium"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "rental_id":"synthetic-lium","status":"active","public_ip":"192.0.2.13",
            "gpu_type":"B200","region":"NSW"
        })))
        .mount(&peer)
        .await;
    let credentials = serde_json::from_value(json!({"provider":"bittensor-lium",
        "api_key":"synthetic-local-peer-key","endpoint":peer.uri()
    }))
    .unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let mut spec = ResourceSpec::basic();
    spec.gpu_class = Some("nvidia-b200".into());
    spec.gpu_count = Some(8);
    spec.gpu_vram_gb = Some(180);
    spec.ownership = Some(ManagedLeaseIdentity {
        namespace: "synthetic-cell".into(),
        lease_id: "lium-eight".into(),
    });
    spec.lease_duration_secs = Some(3600);
    spec.ssh_public_key = Some(
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMo32AY0vFK7g5FBIBcyPdxaxSEM5rEc0kEzMVveA9b+".into(),
    );
    spec.image = Some("example.invalid/synthetic-image:v1".into());
    provisioner
        .provision_with_requirements(CloudProvider::BittensorLium, &spec, "NSW", false)
        .await
        .unwrap();
    let requests = peer.received_requests().await.unwrap();
    let native_path = "/executors/b5864202-5da2-4707-886b-4805fecb4e3c/rent";
    let launch = requests
        .iter()
        .find(|r| r.method == "POST" && r.url.path() == native_path)
        .unwrap_or_else(|| {
            panic!(
                "expected the recorded native executor route; observed {:?}",
                requests.iter().map(|r| r.url.path()).collect::<Vec<_>>()
            )
        });
    let body: Value = launch.body_json().unwrap();
    assert_eq!(body["gpu_count"], 8);
    assert_eq!(body["termination_hours"], 1);
    assert!(body["template_id"].as_str().is_some());
    assert!(body.get("hotkey").is_none());
    assert!(body.get("coldkey").is_none());
}
