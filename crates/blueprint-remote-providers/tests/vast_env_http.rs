//! Public request against a synthetic native peer; no live secret or stock claim.
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, ResourceSpec, managed::ManagedLeaseIdentity,
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

async fn native_peer(uncertain_attach: bool) -> MockServer {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = MockServer::start().await;
    let native = std::sync::Arc::new(std::sync::Mutex::new(Value::Null));
    let listed = native.clone();
    Mock::given(method("GET"))
        .and(path("/v1/instances"))
        .respond_with(move |_: &wiremock::Request| {
            let instance = listed.lock().unwrap();
            let instances = if instance.is_null() {
                vec![]
            } else {
                vec![instance.clone()]
            };
            ResponseTemplate::new(200)
                .set_body_json(json!({"instances":instances,"next_token":null}))
        })
        .mount(&peer)
        .await;
    Mock::given(method("POST")).and(path("/bundles/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"offers":[{
            "id":7001,"gpu_name":"B200","num_gpus":8,"dph_total":150.0,"min_bid":129.0,"storage_total_cost":1.0,
            "reliability2":0.999,"verified":true,"rentable":true,"rented":false,
            "cpu_cores_effective":128,"cpu_ram":2048000,"disk_space":4096,"geolocation":"synthetic-region"
        }]}))).mount(&peer).await;
    let created = native.clone();
    Mock::given(method("PUT")).and(path("/asks/7001/"))
        .respond_with(move |request: &wiremock::Request| {
            let body:Value=request.body_json().unwrap();
            *created.lock().unwrap()=json!({"id":8001,"label":body["label"],"image_runtype":body["runtype"],"actual_status":"running","intended_status":"running","gpu_name":"B200","num_gpus":8,"dph_total":150.0,"geolocation":"synthetic-region"});
            ResponseTemplate::new(200).set_body_json(json!({"success":true,"new_contract":8001}))
        }).mount(&peer).await;
    Mock::given(method("GET"))
        .and(path("/instances/8001/"))
        .respond_with(move |_: &wiremock::Request| {
            ResponseTemplate::new(200)
                .set_body_json(json!({"instances":native.lock().unwrap().clone()}))
        })
        .mount(&peer)
        .await;
    let keys = std::sync::Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
    let uncertain = std::sync::atomic::AtomicBool::new(uncertain_attach);
    Mock::given(path("/instances/8001/ssh/"))
        .respond_with(move |request: &wiremock::Request| {
            let mut keys = keys.lock().unwrap();
            if request.method == "POST" {
                let body: Value = request.body_json().unwrap();
                let key = body["ssh_key"].as_str().unwrap().to_owned();
                assert!(
                    !keys.contains(&key),
                    "completion must verify an existing assignment before retrying mutation"
                );
                keys.push(key);
                if uncertain.swap(false, std::sync::atomic::Ordering::SeqCst) {
                    return ResponseTemplate::new(503)
                        .set_body_string("synthetic-provider-secret-body");
                }
                ResponseTemplate::new(200).set_body_json(json!({"success":true}))
            } else {
                assert_eq!(request.method, "GET");
                let encoded = serde_json::to_string(
                    &keys
                        .iter()
                        .map(|key| json!({"id":1,"name":"synthetic","public_key":key}))
                        .collect::<Vec<_>>(),
                )
                .unwrap();
                ResponseTemplate::new(200).set_body_json(json!({"success":true,"ssh_keys":encoded}))
            }
        })
        .mount(&peer)
        .await;
    peer
}

#[tokio::test]
async fn vast_preserves_the_callers_native_environment_map() {
    let peer = native_peer(false).await;
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
    let peer = native_peer(false).await;
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
    assert_eq!(
        book.quotes[0].offer.hourly_cost_usd.to_bits(),
        130.0_f64.to_bits()
    );
    spec.max_hourly_cost_usd = Some(129.0);
    assert!(provisioner.quotes(&spec).await.unwrap().quotes.is_empty());
    spec.max_hourly_cost_usd = Some(140.0);
    spec.allow_spot = false;
    assert!(provisioner.quotes(&spec).await.unwrap().quotes.is_empty());
}

#[cfg(unix)]
#[tokio::test]
async fn vast_attaches_the_requested_instance_key_and_preserves_explicit_workload_argv() {
    let peer = native_peer(true).await;
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
    spec.ssh_public_key = Some(format!(" \t{key}\r\n"));
    let literal = "spaces $token; 'quote'";
    spec.command = Some(vec!["/usr/bin/printf".into(), "%s".into(), literal.into()]);
    let first = provisioner
        .provision_with_requirements(CloudProvider::VastAi, &spec, "synthetic-region", false)
        .await;
    assert!(
        matches!(
            first,
            Err(blueprint_remote_providers::Error::Provider(
                blueprint_remote_providers::managed::ProviderErrorCode::Indeterminate
            ))
        ),
        "an uncertain attachment must not grant ready access: {first:?}"
    );
    assert_eq!(
        provisioner
            .list_managed("synthetic-cell")
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(
        provisioner
            .list_managed("other-cell")
            .await
            .unwrap()
            .is_empty()
    );
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
    assert_eq!(attach, json!({"ssh_key":key.rsplit_once(' ').unwrap().0}));
    assert!(!requests.iter().any(|r| r.url.path() == "/ssh/"));
    let output = std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg(rent["onstart"].as_str().unwrap())
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, literal.as_bytes());
    assert!(output.stderr.is_empty());
    provisioner
        .provision_with_requirements(CloudProvider::VastAi, &spec, "synthetic-region", false)
        .await
        .unwrap();
    assert_eq!(
        peer.received_requests()
            .await
            .unwrap()
            .iter()
            .filter(|r| r.method == "POST" && r.url.path() == "/instances/8001/ssh/")
            .count(),
        1
    );
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
