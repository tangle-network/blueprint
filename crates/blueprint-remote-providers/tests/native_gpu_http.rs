//! Public SDK requests against HTTP peers; no current-stock or live-rental claim.
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, ResourceSpec, managed::ManagedLeaseIdentity,
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[cfg(unix)]
#[path = "native_gpu_http/bootstrap.rs"]
mod bootstrap;

#[path = "native_gpu_http/lambda_firewall.rs"]
mod lambda_firewall;

#[path = "native_gpu_http/lambda_access.rs"]
mod lambda_access;

#[path = "native_gpu_http/lambda_sku.rs"]
mod lambda_sku;

#[path = "native_gpu_http/lium.rs"]
mod lium;

#[path = "fixtures/measured_recipe.rs"]
mod measured_recipe;
#[path = "native_gpu_http/provenance.rs"]
mod provenance;
#[path = "fixtures/runpod_http.rs"]
mod runpod_fixture;
use runpod_fixture::runpod_peer_state;

async fn runpod_v2_peer() -> MockServer {
    runpod_peer_state().await.0
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
    use std::sync::{Arc, Mutex};
    use wiremock::{Request, matchers::path_regex};
    let state = Arc::new(Mutex::new(std::collections::BTreeMap::<u64, Value>::new()));
    Mock::given(path_regex("^/(bundles/|asks/[0-9]+/|instances/[0-9]+/|v1/instances)$"))
        .respond_with(move |request: &Request| {
            let mut state = state.lock().unwrap();
            match (request.method.as_str(), request.url.path()) {
                ("POST", "/bundles/") => {
                    let body: Value = request.body_json().unwrap();
                    let offers = [(7001,0.4),(7002,0.45)].into_iter()
                        .filter(|(id,_)|body["id"]["eq"].as_u64().is_none_or(|requested|requested==*id))
                        .map(|(id,bid)|json!({"id":id,"gpu_name":"B200","num_gpus":8,"dph_total":bid+0.1,
                            "min_bid":bid,"storage_total_cost":0.0,"reliability2":0.999,"verified":true,"rentable":true,"rented":false,
                            "cpu_cores_effective":128,"cpu_ram":2048000,"disk_space":4096,"geolocation":"synthetic-region"})).collect::<Vec<_>>();
                    ResponseTemplate::new(200).set_body_json(json!({"offers":offers}))
                },
                ("PUT", path) => {
                    let body:Value=request.body_json().unwrap();
                    let id=path.trim_start_matches("/asks/").trim_end_matches('/').parse::<u64>().unwrap()+1000;
                    state.insert(id,json!({"id":id,"label":body["label"],"gpu_name":"B200","num_gpus":8,
                        "actual_status":if id==8002 {"stopped"} else {"running"},"intended_status":"running","is_bid":true,
                        "public_ipaddr":"192.0.2.12","ssh_host":"ssh.synthetic.invalid","ssh_port":22001,
                        "dph_total":body["price"],"geolocation":"synthetic-region","start_date":1793959200}));
                    ResponseTemplate::new(200).set_body_json(json!({"success":true,"new_contract":id}))
                },
                ("GET", "/v1/instances") => {
                    let offset=request.url.query_pairs().find(|(key,_)|key=="after_token").and_then(|(_,value)|value.parse::<usize>().ok()).unwrap_or(0);
                    let rows=state.values().rev().cloned().collect::<Vec<_>>();
                    ResponseTemplate::new(200).set_body_json(json!({"instances":rows.iter().skip(offset).take(1).collect::<Vec<_>>(),
                        "next_token":(offset+1<rows.len()).then(||(offset+1).to_string())}))
                },
                ("GET", path) => {
                    let id=path.trim_start_matches("/instances/").trim_end_matches('/').parse::<u64>().unwrap();
                    state.get(&id).map(|raw|ResponseTemplate::new(200).set_body_json(json!({"instances":raw}))).unwrap_or_else(||ResponseTemplate::new(404))
                },
                ("DELETE", path) => {
                    let id=path.trim_start_matches("/instances/").trim_end_matches('/').parse::<u64>().unwrap();state.remove(&id);ResponseTemplate::new(204)
                },
                _=>ResponseTemplate::new(404),
            }
        }).mount(&peer).await;
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
    let instance = provisioner
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
    let identity = spec.ownership.as_ref().unwrap();
    let mut second = spec.clone();
    second.ownership.as_mut().unwrap().namespace = "synthetic-cell-b".into();
    second.ownership.as_mut().unwrap().lease_id = "vast-second".into();
    let quotes = provisioner.quotes(&second).await.unwrap();
    assert_eq!(
        quotes.quotes.len(),
        4,
        "both native offers and both rental types must survive selection"
    );
    let candidate = quotes
        .quotes
        .iter()
        .find(|quote| quote.offer.native_id == "7002" && quote.offer.spot)
        .unwrap();
    let other = provisioner
        .create_managed(&second, &candidate.candidate)
        .await
        .unwrap();
    assert_eq!(
        other.instance.id, "8002",
        "an explicitly selected second candidate must not rent the first offer"
    );
    let status = provisioner
        .status_managed(
            &CloudProvider::VastAi,
            second.ownership.as_ref().unwrap(),
            "8002",
        )
        .await
        .unwrap()
        .unwrap();
    assert!(status.preempted);
    assert_eq!(status.hourly_cost_usd, Some(0.45));
    assert_eq!(
        status.ssh_endpoint.as_deref(),
        Some("ssh://root@ssh.synthetic.invalid:22001")
    );
    assert!(status.endpoint_url.is_none() && status.worker_url.is_none());
    assert_eq!(
        provisioner
            .list_managed(&identity.namespace)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(
        provisioner
            .destroy_managed(&CloudProvider::VastAi, identity, Some("8002"))
            .await
            .is_err()
    );
    provisioner
        .destroy_managed(&CloudProvider::VastAi, identity, Some(&instance.id))
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
        provisioner
            .list_managed("synthetic-cell-b")
            .await
            .unwrap()
            .len(),
        1
    );
    provisioner
        .destroy_managed(
            &CloudProvider::VastAi,
            second.ownership.as_ref().unwrap(),
            Some("8002"),
        )
        .await
        .unwrap();
    assert!(
        provisioner
            .list_managed("synthetic-cell-b")
            .await
            .unwrap()
            .is_empty()
    );
    let requests = peer.received_requests().await.unwrap();
    let second_launches = requests
        .iter()
        .filter(|request| request.method == "PUT" && request.url.path() == "/asks/7002/")
        .collect::<Vec<_>>();
    assert_eq!(second_launches.len(), 1);
    assert_eq!(
        second_launches[0].body_json::<Value>().unwrap()["price"],
        0.45
    );
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
    use std::sync::{Arc, Mutex};
    use wiremock::{Request, matchers::path_regex};
    let instances = Arc::new(Mutex::new(
        std::collections::BTreeMap::<String, Value>::new(),
    ));
    Mock::given(path_regex("^/(instances(/[^/]+)?|instance-operations/(launch|terminate)|firewall-rulesets/[^/]+)$"))
        .respond_with(move |request: &Request| {
            assert_eq!(request.headers.get("authorization").unwrap(), "Bearer synthetic-local-peer-key");
            let mut state = instances.lock().unwrap();
            match (request.method.as_str(), request.url.path()) {
                ("GET", "/firewall-rulesets/global") => ResponseTemplate::new(200).set_body_json(json!({"data":{
                    "id":"global","rules":[{"protocol":"tcp","port_range":[22,22],"source_network":"192.0.2.0/24"}]}})),
                ("GET", "/firewall-rulesets/existing-app") => ResponseTemplate::new(200).set_body_json(json!({"data":{
                    "id":"existing-app","region":{"name":"us-test-1"},
                    "rules":[{"protocol":"tcp","port_range":[8000,8000],"source_network":"192.0.2.0/24"}]}})),
                ("GET", "/instances") => ResponseTemplate::new(200).set_body_json(json!({"data":state.values().cloned().collect::<Vec<_>>(),"page_token":null})),
                ("POST", "/instance-operations/launch") => {
                    let body: Value = request.body_json().unwrap();
                    let id = format!("synthetic-lambda-{}", state.len() + 1);
                    state.insert(id.clone(), json!({"id":id,"name":body["name"],"tags":body["tags"],
                        "status":"active","ip":"192.0.2.14","created":"2026-11-06T10:00:00Z","ssh_key_names":body["ssh_key_names"],
                        "instance_type":{"name":body["instance_type_name"],"specs":{"gpus":8},"price_cents_per_hour":6400},
                        "region":{"name":body["region_name"]}}));
                    ResponseTemplate::new(200).set_body_json(json!({"data":{"instance_ids":[id]}}))
                },
                ("POST", "/instance-operations/terminate") => {
                    let body: Value = request.body_json().unwrap();
                    for id in body["instance_ids"].as_array().unwrap() { state.remove(id.as_str().unwrap()); }
                    ResponseTemplate::new(200).set_body_json(json!({"data":{}}))
                },
                ("GET", path) => state.get(path.trim_start_matches("/instances/"))
                    .map(|raw| ResponseTemplate::new(200).set_body_json(json!({"data":raw})))
                    .unwrap_or_else(||ResponseTemplate::new(404)),
                _ => ResponseTemplate::new(404),
            }
        }).mount(&peer).await;
    let credentials = serde_json::from_value(json!({"provider":"lambda-labs",
        "api_key":"synthetic-local-peer-key","endpoint":peer.uri(),"ssh_key_name":"synthetic-key","firewall_ruleset_ids":["existing-app"]
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
    spec.source_network = Some("192.0.2.0/24".into());
    spec.ports = vec![8000];
    spec.validate()
        .expect("the integration request must be valid");
    spec.ownership = Some(ManagedLeaseIdentity {
        namespace: "synthetic-cell".into(),
        lease_id: "lambda-eight".into(),
    });
    spec.lease_duration_secs = Some(3600);
    let instance = provisioner
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
    assert_eq!(launch["firewall_rulesets"], json!([{"id":"existing-app"}]));
    assert!(
        !requests.iter().any(|request| request.method != "GET"
            && request.url.path().starts_with("/firewall-rulesets"))
    );
    #[cfg(unix)]
    bootstrap::execute_cloud_init_startup(launch["user_data"].as_str().unwrap());
    let identity = spec.ownership.as_ref().unwrap();
    let status = provisioner
        .status_managed(&CloudProvider::LambdaLabs, identity, &instance.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(status.gpu_count, Some(8));
    assert_eq!(status.gpu_vram_gb, Some(180));
    assert_eq!(status.hourly_cost_usd, Some(64.0));
    assert_eq!(
        status.ssh_endpoint.as_deref(),
        Some("ssh://ubuntu@192.0.2.14:22")
    );
    assert!(status.endpoint_url.is_none() && status.worker_url.is_none());
    let mut second = spec.clone();
    second.ownership.as_mut().unwrap().namespace = "synthetic-cell-b".into();
    second.ownership.as_mut().unwrap().lease_id = "lambda-other".into();
    let quotes = provisioner.quotes(&second).await.unwrap();
    let other = provisioner
        .create_managed(&second, &quotes.quotes[0].candidate)
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
    assert!(
        provisioner
            .destroy_managed(
                &CloudProvider::LambdaLabs,
                identity,
                Some(&other.instance.id)
            )
            .await
            .is_err()
    );
    provisioner
        .destroy_managed(&CloudProvider::LambdaLabs, identity, Some(&instance.id))
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
        provisioner
            .list_managed("synthetic-cell-b")
            .await
            .unwrap()
            .len(),
        1
    );
    provisioner
        .destroy_managed(
            &CloudProvider::LambdaLabs,
            second.ownership.as_ref().unwrap(),
            Some(&other.instance.id),
        )
        .await
        .unwrap();
    assert!(
        provisioner
            .list_managed("synthetic-cell-b")
            .await
            .unwrap()
            .is_empty()
    );
}

#[path = "native_gpu_http/adoption.rs"]
mod adoption;

#[path = "native_gpu_http/leases.rs"]
mod leases;
