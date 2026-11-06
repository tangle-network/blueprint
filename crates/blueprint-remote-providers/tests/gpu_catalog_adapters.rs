use blueprint_remote_providers::providers::{
    BittensorLiumAdapter, PrimeIntellectAdapter, RunPodAdapter, TensorDockAdapter,
    lambda_labs::LambdaLabsInstanceMapper, runpod::RunPodInstanceMapper,
};
use blueprint_remote_providers::{ResourceSpec, infra::traits::CloudProviderAdapter};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_partial_json, method, path},
};

fn gpu(class: &str, count: u32, vram: u32) -> ResourceSpec {
    serde_json::from_value(json!({
        "cpu":4.0,"memory_gb":16.0,"storage_gb":100.0,
        "gpu_class":class,"gpu_count":count,"gpu_vram_gb":vram,"allow_spot":false,
        "ssh_public_key":"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIO5lyvUO/zYHB6a5iuEaYvz6O2/OcPe44RBZI80mIt65 contract-test"
    })).unwrap()
}

#[test]
fn explicit_gpu_class_reaches_provider_skus_without_host_ram_guessing() {
    let spec = gpu("nvidia-b200", 8, 180);
    assert_eq!(
        RunPodInstanceMapper::map(&spec).instance_type,
        "NVIDIA B200"
    );
    assert_eq!(
        LambdaLabsInstanceMapper::map(&spec).instance_type,
        "gpu_8x_b200"
    );
    assert!(
        LambdaLabsInstanceMapper::map(&gpu("nvidia-h100", 3, 80))
            .instance_type
            .is_empty(),
        "a fixed node SKU must never round a three-GPU request up to eight"
    );
}

#[test]
fn unknown_gpu_class_has_no_selectable_price() {
    assert!(
        RunPodInstanceMapper::map(&gpu("nvidia-unknown", 1, 24))
            .estimated_hourly_cost
            .is_none()
    );
}

#[tokio::test]
async fn runpod_quote_and_create_preserve_class_count_and_private_access() {
    let peer = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"data":{"gpuTypes":[{
                "id":"NVIDIA B200","lowestPrice":{"uninterruptablePrice":2.0,"stockStatus":"High"}
            }]}})),
        )
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("POST"))
        .and(path("/pods"))
        .and(body_partial_json(
            json!({"gpuTypeIds":["NVIDIA B200"],"gpuCount":8,"containerDiskInGb":100}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id":"pod-eight"})))
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("GET"))
        .and(path("/pods/pod-eight"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id":"pod-eight","gpuTypeId":"NVIDIA B200","desiredStatus":"RUNNING",
            "publicIp":"203.0.113.8","portMappings":{"22":30222}
        })))
        .mount(&peer)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/pods/pod-eight"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&peer)
        .await;
    let adapter = RunPodAdapter::with_endpoint("account-secret".into(), &peer.uri()).unwrap();
    let mut spec = gpu("nvidia-b200", 8, 180);
    spec.image = Some("owner/ssh-workload:latest".into());
    let quote = adapter.quote(&spec, "").await.unwrap();
    assert_eq!(quote.estimated_hourly_cost, Some(16.0));
    let instance = adapter
        .provision_resources(&quote.instance_type, "", &spec, false)
        .await
        .unwrap();
    assert_eq!(
        instance.status,
        blueprint_remote_providers::InstanceStatus::Starting
    );
    let instance = adapter.get_instance_details(&instance.id).await.unwrap();
    assert_eq!(
        adapter.lease_endpoint(&instance).await.unwrap(),
        "ssh://root@203.0.113.8:30222"
    );
    assert!(
        adapter
            .quote(&gpu("nvidia-unknown", 1, 24), "")
            .await
            .is_err()
    );
    adapter.terminate_instance(&instance.id).await.unwrap();
    let requests = peer.received_requests().await.unwrap();
    let body: Value = requests
        .iter()
        .find(|request| request.url.path() == "/pods")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(body["env"]["PUBLIC_KEY"], spec.ssh_public_key.unwrap());
    assert_eq!(body["imageName"], "owner/ssh-workload:latest");
    assert!(!body.to_string().contains("account-secret"));
    let query: Value = requests
        .iter()
        .find(|request| request.url.path() == "/graphql")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(query["variables"]["input"]["gpuCount"], 8);
}

#[tokio::test]
async fn prime_uses_a_live_offer_and_sends_eight_gpus_and_the_lease_key() {
    let peer = MockServer::start().await;
    Mock::given(method("GET")).and(path("/availability/gpus"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"totalCount":1,"items":[{
            "cloudId":"eight-b200-node","gpuType":"B200_180GB","gpuCount":8,"gpuMemory":180,
            "socket":"SXM","provider":"runpod","stockStatus":"Available","security":"secure_cloud",
            "prices":{"onDemand":16.0,"currency":"USD"},
            "disk":{"defaultCount":100},"vcpu":{"defaultCount":4},"memory":{"defaultCount":16}
        }]}))).expect(1).mount(&peer).await;
    Mock::given(method("POST"))
        .and(path("/ssh_keys/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id":"lease-key"})))
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("POST"))
        .and(path("/pods/"))
        .and(body_partial_json(
            json!({"pod":{"gpuType":"B200_180GB","gpuCount":8,"sshKeyId":"lease-key"}}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id":"prime-eight"})))
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("GET"))
        .and(path("/pods/prime-eight"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"id":"prime-eight","status":"ACTIVE",
            "gpuName":"B200_180GB","ip":"203.0.113.9","sshConnection":"root@203.0.113.9 -p 2200"})),
        )
        .mount(&peer)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/pods/prime-eight"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&peer)
        .await;
    let adapter = PrimeIntellectAdapter::with_endpoint("secret".into(), &peer.uri()).unwrap();
    let spec = gpu("nvidia-b200", 8, 180);
    let quote = adapter.quote(&spec, "").await.unwrap();
    assert_eq!(quote.estimated_hourly_cost, Some(16.0));
    let instance = adapter
        .provision_resources(&quote.instance_type, "", &spec, false)
        .await
        .unwrap();
    assert_eq!(
        instance.status,
        blueprint_remote_providers::InstanceStatus::Starting
    );
    let instance = adapter.get_instance_details(&instance.id).await.unwrap();
    assert_eq!(
        adapter.lease_endpoint(&instance).await.unwrap(),
        "ssh://root@203.0.113.9:2200"
    );
    adapter.terminate_instance(&instance.id).await.unwrap();
}

#[tokio::test]
async fn lium_rent_by_spec_preserves_eight_gpus_and_vram() {
    let peer = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/executors/rent-by-spec"))
        .and(body_partial_json(
            json!({"gpu_type":"B200","gpu_count":8,"min_vram_gb":180,"dry_run":true,"termination_hours":26}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"price_per_hour":12.0})))
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("POST"))
        .and(path("/executors/rent-by-spec"))
        .and(body_partial_json(
            json!({"gpu_type":"B200","gpu_count":8,"min_vram_gb":180,"dry_run":false,"termination_hours":26}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"pod_id":"lium-eight"})))
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("GET"))
        .and(path("/pods/lium-eight"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"id":"lium-eight","status":"RUNNING",
            "executor":{"machine_name":"B200","executor_ip_address":"203.0.113.10"},
            "ssh_connect_cmd":"ssh root@203.0.113.10 -p 2201"}),
        ))
        .mount(&peer)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/pods/lium-eight"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&peer)
        .await;
    let adapter = BittensorLiumAdapter::with_endpoint("secret".into(), &peer.uri()).unwrap();
    let mut spec = gpu("nvidia-b200", 8, 180);
    spec.lease_duration_secs = Some(90_001);
    let quote = adapter.quote(&spec, "").await.unwrap();
    let instance = adapter
        .provision_resources(&quote.instance_type, "", &spec, false)
        .await
        .unwrap();
    assert_eq!(
        instance.status,
        blueprint_remote_providers::InstanceStatus::Starting
    );
    let instance = adapter.get_instance_details(&instance.id).await.unwrap();
    assert_eq!(
        adapter.lease_endpoint(&instance).await.unwrap(),
        "ssh://root@203.0.113.10:2201"
    );
    adapter.terminate_instance(&instance.id).await.unwrap();
    let requests = peer.received_requests().await.unwrap();
    for request in requests
        .iter()
        .filter(|request| request.url.path() == "/executors/rent-by-spec")
    {
        let body: Value = request.body_json().unwrap();
        assert_eq!(body["termination_hours"], 26);
    }
}

#[tokio::test]
async fn tensordock_live_host_price_and_form_create_keep_count_eight() {
    let peer = MockServer::start().await;
    Mock::given(method("GET")).and(path("/client/deploy/hostnodes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"success":true,"hostnodes":{"host-eight":{
            "status":{"online":true,"listed":true},"location":{"id":"location"},"networking":{"ports":[2202]},
            "specs":{"gpu":{"h100-sxm5-80gb":{"amount":8,"price":1.0,"vram":80}},
                "cpu":{"amount":32,"price":0.01},"ram":{"amount":128,"price":0.001},
                "storage":{"amount":1000,"price":0.0001}}
        }}}))).expect(1).mount(&peer).await;
    Mock::given(method("POST"))
        .and(path("/client/deploy/single"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"success":true,"server":"tensor-eight"})),
        )
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("POST"))
        .and(path("/client/get/single"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"success":true,"virtualmachine":{
                "status":"Running","hostname":"203.0.113.11","location":"location",
                "specs":{"gpu":{"type":"h100-sxm5-80gb","amount":8}},"port_forwards":{"2202":"22"}
            }}),
        ))
        .mount(&peer)
        .await;
    Mock::given(method("POST"))
        .and(path("/client/delete/single"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"success":true})))
        .expect(1)
        .mount(&peer)
        .await;
    let adapter =
        TensorDockAdapter::with_endpoint("key".into(), "token".into(), &peer.uri()).unwrap();
    let spec = gpu("nvidia-h100", 8, 80);
    let quote = adapter.quote(&spec, "").await.unwrap();
    assert!((quote.estimated_hourly_cost.unwrap() - 8.066).abs() < 0.00001);
    let instance = adapter
        .provision_resources(&quote.instance_type, "", &spec, false)
        .await
        .unwrap();
    assert_eq!(
        instance.status,
        blueprint_remote_providers::InstanceStatus::Starting
    );
    let instance = adapter.get_instance_details(&instance.id).await.unwrap();
    assert_eq!(
        adapter.lease_endpoint(&instance).await.unwrap(),
        "ssh://user@203.0.113.11:2202"
    );
    adapter.terminate_instance(&instance.id).await.unwrap();
    let requests = peer.received_requests().await.unwrap();
    let request = requests
        .iter()
        .find(|request| request.url.path() == "/client/deploy/single")
        .unwrap();
    let form: std::collections::HashMap<_, _> = url::form_urlencoded::parse(&request.body)
        .into_owned()
        .collect();
    assert_eq!(form["gpu_count"], "8");
    assert_eq!(form["gpu_model"], "h100-sxm5-80gb");
    assert_eq!(form["ssh_key"], spec.ssh_public_key.unwrap());
}

#[cfg(feature = "kubernetes")]
#[tokio::test]
#[serial_test::serial]
async fn coreweave_class_and_gpu_count_reach_the_kubernetes_http_request() {
    use blueprint_remote_providers::{
        core::deployment_target::DeploymentTarget, providers::CoreWeaveAdapter,
    };
    let peer = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/apis/apps/v1/namespaces/lease-test/deployments"))
        .and(body_partial_json(json!({"spec":{"template":{"spec":{
            "nodeSelector":{"gpu.nvidia.com/class":"B200"},
            "containers":[{"name":"blueprint","image":"owner/workload:latest",
                "resources":{"limits":{"nvidia.com/gpu":"8"},"requests":{"nvidia.com/gpu":"8"}}}]
        }}}})))
        .respond_with(ResponseTemplate::new(201).set_body_json(
            json!({"apiVersion":"apps/v1","kind":"Deployment",
            "metadata":{"name":"blueprint"},"spec":{"selector":{"matchLabels":{"app":"blueprint"}},
                "template":{"metadata":{"labels":{"app":"blueprint"}},"spec":{"containers":[]}}}}),
        ))
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("POST"))
        .and(path("/api/v1/namespaces/lease-test/services"))
        .respond_with(
            ResponseTemplate::new(201).set_body_json(json!({"apiVersion":"v1","kind":"Service",
            "metadata":{"name":"blueprint"},"spec":{"ports":[{"port":8080}]}})),
        )
        .expect(1)
        .mount(&peer)
        .await;
    let directory = tempfile::tempdir().unwrap();
    let config = directory.path().join("config");
    std::fs::write(&config,format!("apiVersion: v1\nkind: Config\ncurrent-context: peer\nclusters:\n- name: peer\n  cluster:\n    server: {}\ncontexts:\n- name: peer\n  context:\n    cluster: peer\n    user: peer\nusers:\n- name: peer\n  user: {{}}\n",peer.uri())).unwrap();
    struct RestoreKubeconfig(Option<std::ffi::OsString>);
    impl Drop for RestoreKubeconfig {
        fn drop(&mut self) {
            match &self.0 {
                Some(value) => unsafe { std::env::set_var("KUBECONFIG", value) },
                None => unsafe { std::env::remove_var("KUBECONFIG") },
            }
        }
    }
    let _restore = RestoreKubeconfig(std::env::var_os("KUBECONFIG"));
    // Process configuration is restored on errors and panics as well as success.
    unsafe { std::env::set_var("KUBECONFIG", &config) };
    let adapter = CoreWeaveAdapter::with_credentials("account-token".into(), "lease-test".into());
    let result = adapter
        .deploy_blueprint_with_target(
            &DeploymentTarget::GenericKubernetes {
                context: None,
                namespace: "lease-test".into(),
            },
            "owner/workload:latest",
            &gpu("nvidia-b200", 8, 180),
            Default::default(),
        )
        .await;
    result.unwrap();
}

#[tokio::test]
async fn lambda_quotes_and_creates_the_exact_eight_gpu_node_in_a_stocked_region() {
    use blueprint_remote_providers::providers::LambdaLabsAdapter;
    let peer = MockServer::start().await;
    Mock::given(method("GET")).and(path("/instance-types"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{"gpu_8x_b200":{
            "instance_type":{"name":"gpu_8x_b200","gpu_description":"NVIDIA B200 180GB",
                "price_cents_per_hour":1600,"specs":{"gpus":8,"vcpus":128,"memory_gib":1024,"storage_gib":2000}},
            "regions_with_capacity_available":[{"name":"eu-central-1"}]
        }}}))).expect(2).mount(&peer).await;
    Mock::given(method("POST"))
        .and(path("/ssh-keys"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{"id":"key-id"}})))
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("POST"))
        .and(path("/instance-operations/launch"))
        .and(body_partial_json(
            json!({"instance_type_name":"gpu_8x_b200","quantity":1,"region_name":"eu-central-1"}),
        ))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":{"instance_ids":["lambda-eight"]}})),
        )
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("GET"))
        .and(path("/instances/lambda-eight"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"data":{"id":"lambda-eight",
            "status":"active","ip":"203.0.113.12","region":{"name":"eu-central-1"},
            "instance_type":{"name":"gpu_8x_b200"}}})),
        )
        .expect(1)
        .mount(&peer)
        .await;
    Mock::given(method("POST"))
        .and(path("/instance-operations/terminate"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":{"terminated_instances":["lambda-eight"]}})),
        )
        .expect(1)
        .mount(&peer)
        .await;
    let adapter = LambdaLabsAdapter::with_endpoint("secret".into(), &peer.uri()).unwrap();
    let spec = gpu("nvidia-b200", 8, 180);
    let quote = adapter.quote(&spec, "").await.unwrap();
    assert_eq!(quote.estimated_hourly_cost, Some(16.0));
    let created = adapter
        .provision_resources(&quote.instance_type, "", &spec, false)
        .await
        .unwrap();
    assert_eq!(
        created.status,
        blueprint_remote_providers::InstanceStatus::Starting
    );
    let instance = adapter.get_instance_details(&created.id).await.unwrap();
    assert_eq!(
        adapter.lease_endpoint(&instance).await.unwrap(),
        "ssh://ubuntu@203.0.113.12:22"
    );
    adapter.terminate_instance(&created.id).await.unwrap();
}

#[tokio::test]
async fn runpod_compares_qualifying_classes_and_can_quote_eight_b300_gpus() {
    let peer = MockServer::start().await;
    Mock::given(method("POST")).and(path("/graphql"))
        .and(body_partial_json(json!({"variables":{"input":{"gpuCount":1}}})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{"gpuTypes":[
            {"id":"NVIDIA L4","lowestPrice":{"uninterruptablePrice":1.0,"stockStatus":"High"}},
            {"id":"NVIDIA GeForce RTX 4090","lowestPrice":{"uninterruptablePrice":0.4,"stockStatus":"High"}},
            {"id":"NVIDIA GeForce RTX 3090","lowestPrice":{"uninterruptablePrice":0.1,"stockStatus":"Unavailable"}},
            {"id":"unknown-gpu","lowestPrice":{"uninterruptablePrice":0.01,"stockStatus":"High"}}
        ]}}))).expect(1).mount(&peer).await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .and(body_partial_json(
            json!({"variables":{"filter":{"id":"NVIDIA B300 SXM6 AC"},"input":{"gpuCount":8}}}),
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"data":{"gpuTypes":[
                {"id":"NVIDIA B300 SXM6 AC","lowestPrice":{"uninterruptablePrice":3.0,"stockStatus":"High"}}
            ]}})),
        )
        .expect(1)
        .mount(&peer)
        .await;
    let adapter = RunPodAdapter::with_endpoint("secret".into(), &peer.uri()).unwrap();
    let mut any_gpu = gpu("nvidia-l4", 1, 24);
    any_gpu.gpu_class = None;
    let quote = adapter.quote(&any_gpu, "").await.unwrap();
    assert_eq!(quote.instance_type, "NVIDIA GeForce RTX 4090");
    assert_eq!(quote.estimated_hourly_cost, Some(0.4));
    let b300 = adapter
        .quote(&gpu("nvidia-b300", 8, 262), "")
        .await
        .unwrap();
    assert_eq!(b300.instance_type, "NVIDIA B300 SXM6 AC");
    assert_eq!(b300.estimated_hourly_cost, Some(24.0));
}
