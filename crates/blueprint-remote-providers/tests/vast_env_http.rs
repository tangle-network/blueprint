//! Public request against a synthetic native peer; no live secret or stock claim.
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, ResourceSpec, managed::ManagedLeaseIdentity,
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

async fn native_peer() -> MockServer {
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
            "id":7001,"gpu_name":"B200","num_gpus":8,"dph_total":150.0,"min_bid":129.0,"storage_total_cost":1.0,
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
    Mock::given(method("POST"))
        .and(path("/instances/8001/ssh/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"success":true})))
        .mount(&peer)
        .await;
    peer
}

#[tokio::test]
async fn vast_preserves_the_callers_native_environment_map() {
    let peer = native_peer().await;
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

#[tokio::test]
async fn vast_accepts_native_bids_above_128_within_the_callers_cap() {
    let peer = native_peer().await;
    let credentials = serde_json::from_value(
        json!({"provider":"vast-ai","api_key":"synthetic-provider-key","endpoint":peer.uri()}),
    )
    .unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let mut spec = ResourceSpec::basic();
    spec.gpu_class = Some("nvidia-b200".into());
    spec.gpu_count = Some(8);
    spec.allow_spot = true;
    spec.max_hourly_cost_usd = Some(140.0);
    let book = provisioner.quotes(&spec).await.unwrap();
    assert_eq!(
        book.quotes.len(),
        1,
        "synthetic native bid129 plus storage1 is selectable under cap140; no live stock claim"
    );
    assert!(book.quotes[0].offer.spot);
    assert_eq!(book.quotes[0].offer.hourly_cost_usd, 130.0);
    spec.max_hourly_cost_usd = Some(129.0);
    assert!(provisioner.quotes(&spec).await.unwrap().quotes.is_empty());
    spec.max_hourly_cost_usd = Some(140.0);
    spec.allow_spot = false;
    assert!(provisioner.quotes(&spec).await.unwrap().quotes.is_empty());
}

#[cfg(unix)]
#[tokio::test]
async fn vast_attaches_the_requested_instance_key_and_preserves_explicit_workload_argv() {
    let peer = native_peer().await;
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
        lease_id: "native-ssh".into(),
    });
    spec.lease_duration_secs = Some(3600);
    let key = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIBERERERERERERERERERERERERERERERERERERERERER synthetic-public-key";
    spec.ssh_public_key = Some(key.into());
    let literal = "spaces $token; 'quote'";
    spec.command = Some(vec!["/usr/bin/printf".into(), "%s".into(), literal.into()]);
    provisioner
        .provision_with_requirements(CloudProvider::VastAi, &spec, "synthetic-region", false)
        .await
        .unwrap();
    let requests = peer.received_requests().await.unwrap();
    let rent: Value = requests
        .iter()
        .find(|r| r.method == "PUT")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(rent["runtype"], "ssh_direc ssh_proxy");
    assert!(
        rent.get("args").is_none(),
        "SSH startup must carry the explicit workload without mixing incompatible launch modes"
    );
    let attach: Value = requests
        .iter()
        .find(|r| r.method == "POST" && r.url.path() == "/instances/8001/ssh/")
        .expect("caller key must reach the instance-scoped API")
        .body_json()
        .unwrap();
    assert_eq!(attach, json!({"ssh_key":key}));
    assert!(!requests.iter().any(|r| r.url.path() == "/ssh/"));
    let output = std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg(rent["onstart"].as_str().unwrap())
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, literal.as_bytes());
    assert!(output.stderr.is_empty());
    spec.command = None;
    let result = provisioner
        .provision_with_requirements(CloudProvider::VastAi, &spec, "synthetic-region", false)
        .await;
    assert!(matches!(
        result,
        Err(blueprint_remote_providers::Error::Provider(
            blueprint_remote_providers::managed::ProviderErrorCode::Unsupported
        ))
    ));
    assert_eq!(
        peer.received_requests()
            .await
            .unwrap()
            .iter()
            .filter(|r| r.method == "PUT")
            .count(),
        1
    );
}
