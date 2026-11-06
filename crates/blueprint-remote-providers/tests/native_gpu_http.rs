//! Public SDK requests against HTTP peers; no current-stock or live-rental claim.
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, ResourceSpec, managed::ManagedLeaseIdentity,
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

async fn runpod_v2_peer() -> MockServer {
    use std::sync::{Arc, Mutex};
    use wiremock::{Request, matchers::path_regex};
    let peer = MockServer::start().await;
    let state = Arc::new(Mutex::new((
        0u64,
        std::collections::BTreeMap::<String, Value>::new(),
    )));
    Mock::given(path_regex("^/(catalog/gpus|pods(/[^/]+)?)$"))
        .respond_with(move |request: &Request| {
            if request.headers.get("authorization").and_then(|header| header.to_str().ok()) != Some("Bearer synthetic-local-peer-key") {
                return ResponseTemplate::new(401).set_body_string("synthetic-local-peer-key");
            }
            let mut state = state.lock().unwrap();
            match (request.method.as_str(), request.url.path()) {
                ("GET", "/catalog/gpus") => ResponseTemplate::new(200).set_body_json(json!({"gpus":[{
                    "id":"NVIDIA H200","memory":141,"secure":true,"community":false,
                    "maxCount":{"secure":8,"community":0},"price":{"secure":4.5,"community":0},
                    "availability":"HIGH","dataCenters":[{"id":"US-TEST-1","availability":"HIGH"}]
                }]})),
                ("POST", "/pods") => {
                    let Ok(body) = request.body_json::<Value>() else { return ResponseTemplate::new(400); };
                    state.0 += 1;
                    let id = format!("synthetic{}", state.0);
                    let pod = json!({
                        "id":id,"name":body["name"],"gpu":{"id":body["gpu"]["id"],"count":body["gpu"]["count"],"vcpuCount":8,"memory":64},
                        "env":body["env"],"ports":body["ports"],"mounts":body["mounts"],"image":body["image"],
                        "status":"RUNNING","dataCenterId":"US-TEST-1","cost":9.0,
                        "ssh":{"direct":{"host":"192.0.2.10","port":22334,"username":"root"}},
                        "runtime":{"ports":[{"private":8000,"public":null,"type":"http","ip":null}]}
                    });
                    state.1.insert(id, pod.clone());
                    ResponseTemplate::new(201).set_body_json(pod)
                }
                ("GET", "/pods") => {
                    let offset = request.url.query_pairs().find(|(key,_)|key=="cursor")
                        .and_then(|(_,value)| value.parse::<usize>().ok()).unwrap_or(0);
                    let pods = state.1.values().rev().cloned().collect::<Vec<_>>();
                    let page = pods.iter().skip(offset).take(1).cloned().collect::<Vec<_>>();
                    let more = offset + page.len() < pods.len();
                    ResponseTemplate::new(200).set_body_json(json!({"pods":page,
                        "pagination":{"hasNextPage":more,"nextCursor":more.then(||(offset+1).to_string())}}))
                }
                ("GET", path) => match state.1.get(path.trim_start_matches("/pods/")) {
                    Some(pod) => ResponseTemplate::new(200).set_body_json(pod),
                    None => ResponseTemplate::new(404),
                },
                ("DELETE", path) => {
                    state.1.remove(path.trim_start_matches("/pods/"));
                    ResponseTemplate::new(204)
                }
                _ => ResponseTemplate::new(404),
            }
        }).mount(&peer).await;
    peer
}

#[tokio::test]
async fn runpod_receives_the_requested_gpu_shape_and_volume() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = runpod_v2_peer().await;
    let credentials = serde_json::from_value(json!({
        "provider":"runpod", "api_key":"synthetic-local-peer-key", "endpoint":peer.uri()
    }))
    .unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let mut spec = ResourceSpec::basic();
    let identity = ManagedLeaseIdentity {
        namespace: "synthetic-cell-a".into(),
        lease_id: "lease-a".into(),
    };
    spec.ownership = Some(identity.clone());
    spec.lease_duration_secs = Some(3600);
    spec.gpu_class = Some("nvidia-h200".into());
    spec.gpu_count = Some(2);
    spec.gpu_vram_gb = Some(140);
    spec.volume_gb = Some(37.0);
    spec.image = Some("example.invalid/workload:v1".into());
    spec.endpoint_port = Some(8000);
    spec.ports = vec![8000];
    spec.validate()
        .expect("the integration request must be valid");
    let instance = provisioner
        .provision_with_requirements(CloudProvider::RunPod, &spec, "US-TEST-1", false)
        .await
        .unwrap();
    let requests = peer.received_requests().await.unwrap();
    let launch: Value = requests
        .iter()
        .find(|r| r.method == "POST")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(
        json!({"gpu":launch["gpu"]["id"],"count":launch["gpu"]["count"],
        "volume":launch["mounts"]["persistent"]["size"],"image":launch["image"]}),
        json!({"gpu":"NVIDIA H200","count":2,"volume":37,"image":"example.invalid/workload:v1"})
    );
    assert_eq!(launch["ports"], json!(["8000/http"]));
    assert!(!launch.to_string().contains("synthetic-local-peer-key"));
    let resource = provisioner
        .status_managed(&CloudProvider::RunPod, &identity, &instance.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(resource.gpu_class.as_deref(), Some("nvidia-h200"));
    assert_eq!(resource.gpu_count, Some(2));
    assert_eq!(resource.gpu_vram_gb, Some(141));
    assert_eq!(
        resource.hourly_cost_usd.unwrap().to_bits(),
        9.0f64.to_bits()
    );
    assert_eq!(
        resource.endpoint_url.as_deref(),
        Some("https://synthetic1-8000.proxy.runpod.net")
    );
    assert!(
        resource.worker_url.is_none(),
        "a serving endpoint is not an advertised worker control URL"
    );
    assert_eq!(
        resource.ssh_endpoint.as_deref(),
        Some("ssh://root@192.0.2.10:22334")
    );
    let foreign = ManagedLeaseIdentity {
        namespace: "synthetic-cell-b".into(),
        lease_id: "lease-b".into(),
    };
    let mut other = spec.clone();
    other.ownership = Some(foreign.clone());
    let quote = provisioner.quotes(&other).await.unwrap().quotes.remove(0);
    let other_resource = provisioner
        .create_managed(&other, &quote.candidate)
        .await
        .unwrap();
    assert_eq!(
        provisioner
            .list_managed(&identity.namespace)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        provisioner
            .list_managed(&foreign.namespace)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(
        provisioner
            .status_managed(
                &CloudProvider::RunPod,
                &identity,
                &other_resource.instance.id
            )
            .await
            .is_err()
    );
    assert!(
        provisioner
            .destroy_managed(
                &CloudProvider::RunPod,
                &identity,
                Some(&other_resource.instance.id)
            )
            .await
            .is_err()
    );
    provisioner
        .destroy_managed(&CloudProvider::RunPod, &identity, Some(&instance.id))
        .await
        .unwrap();
    assert!(
        provisioner
            .list_managed(&identity.namespace)
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        provisioner.list_managed(&foreign.namespace).await.unwrap()[0]
            .instance
            .id,
        other_resource.instance.id
    );
    provisioner
        .destroy_managed(&CloudProvider::RunPod, &foreign, None)
        .await
        .unwrap();
    assert!(
        provisioner
            .list_managed(&foreign.namespace)
            .await
            .unwrap()
            .is_empty()
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

#[tokio::test]
async fn lambda_native_sku_launch_includes_the_custom_workload_bootstrap() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/instance-types"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{
            "gpu_8x_b200_sxm6":{"instance_type":{"name":"gpu_8x_b200_sxm6",
                "gpu_description":"NVIDIA B200","price_cents_per_hour":6400,
                "specs":{"gpus":8,"memory_gib":2048,"vcpus":128,"storage_gib":4096}},
                "regions_with_capacity_available":[{"name":"us-test-1"}]}
        }})))
        .mount(&peer)
        .await;
    Mock::given(method("GET"))
        .and(path("/instances"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[]})))
        .mount(&peer)
        .await;
    Mock::given(method("POST"))
        .and(path("/instance-operations/launch"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":{"instance_ids":["synthetic-lambda-eight"]}})),
        )
        .mount(&peer)
        .await;
    Mock::given(method("GET")).and(path("/instances/synthetic-lambda-eight"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{
            "id":"synthetic-lambda-eight","status":"active","ip":"192.0.2.14",
            "instance_type":{"name":"gpu_8x_b200_sxm6","specs":{"gpus":8}},"region":{"name":"us-test-1"}
        }}))).mount(&peer).await;
    let credentials = serde_json::from_value(json!({"provider":"lambda-labs",
        "api_key":"synthetic-local-peer-key","endpoint":peer.uri(),"ssh_key_name":"synthetic-key"
    }))
    .unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let mut spec = ResourceSpec::basic();
    spec.gpu_class = Some("nvidia-b200".into());
    spec.gpu_count = Some(8);
    spec.gpu_vram_gb = Some(180);
    spec.image = Some("example.invalid/workload:v1".into());
    spec.env
        .insert("MODEL_KEY".into(), "synthetic-bootstrap-secret".into());
    spec.endpoint_port = Some(8000);
    spec.ports = vec![8000];
    spec.validate()
        .expect("the integration request must be valid");
    spec.ownership = Some(ManagedLeaseIdentity {
        namespace: "synthetic-cell".into(),
        lease_id: "lambda-eight".into(),
    });
    spec.lease_duration_secs = Some(3600);
    provisioner
        .provision_with_requirements(CloudProvider::LambdaLabs, &spec, "us-test-1", false)
        .await
        .unwrap();
    let requests = peer.received_requests().await.unwrap();
    let launch: Value = requests
        .iter()
        .find(|r| r.method == "POST" && r.url.path() == "/instance-operations/launch")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(
        json!({"sku":launch["instance_type_name"],"user_data":launch["user_data"].as_str().is_some(),
        "tags":launch["tags"].as_array().is_some_and(|tags|tags.len()>=6)}),
        json!({"sku":"gpu_8x_b200_sxm6","user_data":true,"tags":true}),
        "a native VM rental must carry the requested container and recovery identity"
    );
    #[cfg(unix)]
    execute_cloud_init_startup(launch["user_data"].as_str().unwrap());
}

#[cfg(unix)]
fn execute_cloud_init_startup(document: &str) {
    use std::os::unix::fs::PermissionsExt;
    use std::process::Command;
    let temp = tempfile::tempdir().unwrap();
    let document: Value = serde_yaml::from_str(document).unwrap();
    let rebase = |path: &str| temp.path().join(path.trim_start_matches('/'));
    for file in document["write_files"].as_array().unwrap() {
        let path = rebase(file["path"].as_str().unwrap());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, file["content"].as_str().unwrap()).unwrap();
        let mode = u32::from_str_radix(file["permissions"].as_str().unwrap(), 8).unwrap();
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).unwrap();
        assert_eq!(file["owner"], "root:root");
    }
    let startup = document["runcmd"][0].as_array().unwrap();
    let config_path = rebase(startup[2].as_str().unwrap());
    let mut config: Value = serde_json::from_slice(&std::fs::read(&config_path).unwrap()).unwrap();
    config["env_file"] = json!(rebase(config["env_file"].as_str().unwrap()));
    std::fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();
    let bin = temp.path().join("bin");
    std::fs::create_dir(&bin).unwrap();
    let docker = r#"#!/usr/bin/env python3
import json, os, pathlib, stat, sys
arguments = sys.argv[1:]
path = pathlib.Path(arguments[arguments.index('--env-file') + 1])
receipt = {'arguments': arguments,
           'secret_loaded': 'MODEL_KEY=synthetic-bootstrap-secret\n' in path.read_text(),
           'env_file_mode': stat.S_IMODE(path.stat().st_mode)}
pathlib.Path(os.environ['BOOTSTRAP_RECEIPT']).write_text(json.dumps(receipt))
"#;
    for (name, source) in [("docker", docker), ("systemctl", "#!/bin/sh\nexit 0\n")] {
        let path = bin.join(name);
        std::fs::write(&path, source).unwrap();
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let receipt_path = temp.path().join("receipt.json");
    let output = Command::new(startup[0].as_str().unwrap())
        .arg(rebase(startup[1].as_str().unwrap()))
        .arg(config_path)
        .env_clear()
        .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
        .env("BOOTSTRAP_RECEIPT", &receipt_path)
        .output()
        .unwrap();
    for stream in [&output.stdout, &output.stderr] {
        assert!(!String::from_utf8_lossy(stream).contains("synthetic-bootstrap-secret"));
    }
    assert!(
        output.status.success(),
        "startup child failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&std::fs::read(receipt_path).unwrap()).unwrap();
    assert_eq!(receipt["secret_loaded"], true);
    assert_eq!(receipt["env_file_mode"], 0o600);
    let arguments = receipt["arguments"].as_array().unwrap();
    assert!(!arguments.iter().any(|arg| {
        arg.as_str()
            .is_some_and(|arg| arg.contains("synthetic-bootstrap-secret"))
    }));
    let image = arguments
        .iter()
        .position(|arg| arg == "example.invalid/workload:v1")
        .unwrap();
    assert_eq!(arguments[image - 1], "--");
    assert!(
        arguments
            .windows(2)
            .any(|pair| pair == [json!("--publish"), json!("8000:8000")])
    );
}
