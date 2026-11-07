//! Recorded Lium nodes and a synthetic native template/rental peer.
use super::*;

async fn lium_native_peer() -> MockServer {
    let peer = MockServer::start().await;
    // https://lium.io/api/public/v1/nodes, captured 2026-11-06T03:55:56Z; SHA25600475806e9408e56046e04ad29f1056876941fd771de28e28c26782cdcb11828.
    // Create/list replies below are synthetic; this is no claim that stock remains available.
    let nodes: Value = serde_json::from_str(include_str!(
        "../fixtures/provider-api/lium-nodes-2026-11-06.json"
    ))
    .unwrap();
    Mock::given(method("GET"))
        .and(path("/public/v1/nodes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(nodes))
        .mount(&peer)
        .await;
    use std::sync::{Arc, Mutex};
    use wiremock::{Request, matchers::path_regex};
    let state = Arc::new(Mutex::new((
        0u64,
        std::collections::BTreeMap::<String, Value>::new(),
        std::collections::BTreeMap::<String, Value>::new(),
        false,
    )));
    Mock::given(path_regex("^/(templates(/[^/]+)?|pods(/[^/]+)?|executors/[^/]+/rent|control/fail-next-rent)$"))
        .respond_with(move |request:&Request| {
            assert_eq!(request.headers.get("x-api-key").unwrap(),"synthetic-local-peer-key");
            let mut state=state.lock().unwrap();
            match (request.method.as_str(),request.url.path()) {
                ("GET","/templates")=>ResponseTemplate::new(200).set_body_json(state.1.values().cloned().collect::<Vec<_>>()),
                ("GET","/pods")=>ResponseTemplate::new(200).set_body_json(state.2.values().cloned().collect::<Vec<_>>()),
                ("POST","/templates")=>{
                    let mut body:Value=request.body_json().unwrap();state.0+=1;
                    let id=format!("00000000-0000-4000-8000-{:012}",state.0);
                    body["id"]=json!(id);state.1.insert(id.clone(),body.clone());ResponseTemplate::new(200).set_body_json(body)
                },
                ("POST","/control/fail-next-rent")=>{state.3=true;ResponseTemplate::new(204)},
                ("POST",_)=>{
                    let key=request.headers.get("idempotency-key").unwrap().to_str().unwrap();
                    assert!(key.len()==64 && key.bytes().all(|b|b.is_ascii_hexdigit()));
                    if state.3 {state.3=false;return ResponseTemplate::new(503).set_body_string("synthetic-provider-secret-body");}
                    let body:Value=request.body_json().unwrap();let template=state.1.get(body["template_id"].as_str().unwrap()).unwrap().clone();
                    state.0+=1;let id=format!("00000000-0000-4000-8000-{:012}",state.0);
                    state.2.insert(id.clone(),json!({"id":id,"template":template,"pod_name":body["pod_name"],"status":"RUNNING",
                        "ssh_keys":[{"name":"synthetic","public_key":body["user_public_key"]}],
                        "gpu_name":"B200","gpu_count":"8","price":48.24,"created_at":"2026-11-06T10:00:00Z",
                        "ssh_connect_cmd":"ssh -p 22345 root@192.0.2.13","termination_hours":body["termination_hours"],
                        "executor":{"location":{"region":"NSW"},"executor_ip_address":"192.0.2.13"}}));
                    ResponseTemplate::new(200).set_body_json(json!({"success":true,"pod_id":id}))
                },
                ("GET",path)=>{
                    let (kind,id)=path.trim_start_matches('/').split_once('/').unwrap();
                    let raw=if kind=="templates" {state.1.get(id)} else {state.2.get(id)};
                    raw.map(|raw|ResponseTemplate::new(200).set_body_json(raw)).unwrap_or_else(||ResponseTemplate::new(404))
                },
                ("DELETE",path)=>{
                    let (kind,id)=path.trim_start_matches('/').split_once('/').unwrap();
                    if kind=="templates" {state.1.remove(id);}else if let Some(pod)=state.2.remove(id){state.1.remove(pod["template"]["id"].as_str().unwrap());}
                    ResponseTemplate::new(204)
                },
                _=>ResponseTemplate::new(404),
            }
        }).mount(&peer).await;
    peer
}

async fn assert_stable_native_retry(
    provisioner: &CloudProvisioner,
    spec: &ResourceSpec,
    peer: &MockServer,
    native_path: &str,
) {
    let mut retry = spec.clone();
    retry.ownership.as_mut().unwrap().namespace = "synthetic-retry".into();
    let quote = provisioner.quotes(&retry).await.unwrap().quotes.remove(0);
    reqwest::Client::new()
        .post(format!("{}/control/fail-next-rent", peer.uri()))
        .header("X-API-Key", "synthetic-local-peer-key")
        .send()
        .await
        .unwrap();
    assert!(
        provisioner
            .create_managed(&retry, &quote.candidate)
            .await
            .is_err()
    );
    let retried = provisioner
        .create_managed(&retry, &quote.candidate)
        .await
        .unwrap();
    let retry_requests = peer.received_requests().await.unwrap();
    let native_retries = retry_requests
        .iter()
        .rev()
        .filter(|request| request.url.path() == native_path)
        .take(2)
        .collect::<Vec<_>>();
    assert_eq!(
        native_retries[0].headers.get("idempotency-key"),
        native_retries[1].headers.get("idempotency-key")
    );
    assert_eq!(
        native_retries[0].body, native_retries[1].body,
        "adopting an orphan template must preserve the exact native rent payload"
    );
    assert_eq!(
        retry_requests
            .iter()
            .filter(|request| request.method == "POST" && request.url.path() == "/templates")
            .count(),
        5,
        "retry must adopt the existing owned template"
    );
    provisioner
        .destroy_managed(
            &CloudProvider::BittensorLium,
            retry.ownership.as_ref().unwrap(),
            Some(&retried.instance.id),
        )
        .await
        .unwrap();
    assert!(
        provisioner
            .list_managed("synthetic-retry")
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn lium_rents_the_recorded_executor_with_the_requested_count_and_lifetime() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = lium_native_peer().await;
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
    spec.ports = vec![8000];
    spec.volume_id = Some("external-volume-id".into());
    spec.volume_mount_path = Some("/mnt".into());
    let instance = provisioner
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
    assert_eq!(body["volume_id"], "external-volume-id");
    assert_eq!(body["initial_port_count"], 2);
    assert!(
        (body["expected_price_per_gpu_hour"].as_f64().unwrap() + 0.01) * 8.0 <= 48.24,
        "the native one-cent tolerance must fit within the uncapped selected quote"
    );
    let template: Value = requests
        .iter()
        .find(|request| request.method == "POST" && request.url.path() == "/templates")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(
        template["docker_image"],
        "example.invalid/synthetic-image:v1"
    );
    assert_eq!(template["internal_ports"], json!([22, 8000]));
    assert_eq!(template["is_private"], true);
    assert_eq!(template["one_time_template"], true);
    let identity = spec.ownership.as_ref().unwrap();
    let status = provisioner
        .status_managed(&CloudProvider::BittensorLium, identity, &instance.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(status.gpu_count, Some(8));
    assert_eq!(status.gpu_vram_gb, Some(180));
    assert_eq!(status.hourly_cost_usd, Some(48.24));
    assert_eq!(
        status.ssh_endpoint.as_deref(),
        Some("ssh://root@192.0.2.13:22345")
    );
    assert!(status.endpoint_url.is_none() && status.worker_url.is_none());
    let mut other = spec.clone();
    other.ownership.as_mut().unwrap().namespace = "synthetic-cell-b".into();
    other.ownership.as_mut().unwrap().lease_id = "lium-other".into();
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
        2,
        "pod and private template must both be recoverable"
    );
    assert!(
        provisioner
            .destroy_managed(
                &CloudProvider::BittensorLium,
                identity,
                Some(&other_resource.instance.id)
            )
            .await
            .is_err()
    );
    provisioner
        .destroy_managed(&CloudProvider::BittensorLium, identity, Some(&instance.id))
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
        2
    );
    provisioner
        .destroy_managed(
            &CloudProvider::BittensorLium,
            other.ownership.as_ref().unwrap(),
            Some(&other_resource.instance.id),
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
    let mut capped = spec.clone();
    capped.max_hourly_cost_usd = Some(48.32);
    capped.ownership.as_mut().unwrap().namespace = "synthetic-capped".into();
    let quote = provisioner.quotes(&capped).await.unwrap().quotes.remove(0);
    let capped_resource = provisioner
        .create_managed(&capped, &quote.candidate)
        .await
        .unwrap();
    let capped_requests = peer.received_requests().await.unwrap();
    let cap_body: Value = capped_requests
        .iter()
        .rev()
        .find(|request| request.method == "POST" && request.url.path() == native_path)
        .unwrap()
        .body_json()
        .unwrap();
    assert!(
        (cap_body["expected_price_per_gpu_hour"].as_f64().unwrap() + 0.01) * 8.0
            <= quote.offer.hourly_cost_usd,
        "a higher caller cap must not replace the selected quote ceiling"
    );
    provisioner
        .destroy_managed(
            &CloudProvider::BittensorLium,
            capped.ownership.as_ref().unwrap(),
            Some(&capped_resource.instance.id),
        )
        .await
        .unwrap();
    assert!(
        provisioner
            .list_managed("synthetic-capped")
            .await
            .unwrap()
            .is_empty()
    );
    let mut interrupted = spec.clone();
    interrupted.ownership.as_mut().unwrap().namespace = "synthetic-interrupted".into();
    reqwest::Client::new()
        .post(format!("{}/control/fail-next-rent", peer.uri()))
        .header("X-API-Key", "synthetic-local-peer-key")
        .send()
        .await
        .unwrap();
    let quote = provisioner
        .quotes(&interrupted)
        .await
        .unwrap()
        .quotes
        .remove(0);
    let failure = provisioner
        .create_managed(&interrupted, &quote.candidate)
        .await
        .unwrap_err();
    assert!(matches!(
        failure,
        blueprint_remote_providers::Error::Provider(
            blueprint_remote_providers::managed::ProviderErrorCode::Indeterminate
        )
    ));
    assert!(
        !failure
            .to_string()
            .contains("synthetic-provider-secret-body")
    );
    let orphaned = provisioner
        .list_managed("synthetic-interrupted")
        .await
        .unwrap();
    assert_eq!(orphaned.len(), 1);
    assert_eq!(
        orphaned[0].kind,
        blueprint_remote_providers::managed::ManagedResourceKind::Template
    );
    provisioner
        .destroy_managed(
            &CloudProvider::BittensorLium,
            interrupted.ownership.as_ref().unwrap(),
            Some("00000000-0000-4000-8000-999999999999"),
        )
        .await
        .unwrap();
    assert!(
        provisioner
            .list_managed("synthetic-interrupted")
            .await
            .unwrap()
            .is_empty(),
        "a missing primary id does not excuse an orphan template"
    );
    assert_stable_native_retry(&provisioner, &spec, &peer, native_path).await;
    let native_keys = peer
        .received_requests()
        .await
        .unwrap()
        .iter()
        .filter(|request| request.url.path() == native_path)
        .map(|request| {
            request
                .headers
                .get("idempotency-key")
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        })
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        native_keys.len(),
        5,
        "equal specifications in different cells require separate native idempotency keys"
    );
}

#[tokio::test]
async fn lium_prices_the_recorded_eight_b300_nodes_through_catalog_aliases() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = lium_native_peer().await;
    let credentials=serde_json::from_value(json!({"provider":"bittensor-lium","api_key":"synthetic-local-peer-key","endpoint":peer.uri()})).unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let mut spec = ResourceSpec::basic();
    spec.gpu_class = Some("nvidia-b300".into());
    spec.gpu_count = Some(8);
    spec.gpu_vram_gb = Some(288);
    spec.allow_spot = true;
    spec.region = Some("LND".into());
    spec.lease_duration_secs = Some(3600);
    spec.validate().unwrap();
    let result = provisioner.quotes(&spec).await.unwrap();
    let offers = result
        .quotes
        .iter()
        .map(|quote| {
            (
                &quote.offer.native_id,
                quote.offer.hourly_cost_usd,
                quote.offer.gpu_count,
                quote.offer.gpu_class.as_str(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        offers,
        vec![
            (
                &"d87df3d2-bfb9-4a96-aff0-edde934a0637".to_owned(),
                51.2,
                8,
                "nvidia-b300"
            ),
            (
                &"558e4c00-1bcc-45c6-84fa-fc4f29502af9".to_owned(),
                64.0,
                8,
                "nvidia-b300"
            )
        ],
        "recorded AC variants use catalog VRAM and keep both native node offers; no present-stock claim"
    );
    spec.region = Some("TX".into());
    let pc = provisioner.quotes(&spec).await.unwrap();
    assert_eq!(
        pc.quotes.len(),
        1,
        "recorded PC variant must match the same B300 class"
    );
    assert_eq!(
        pc.quotes[0].offer.native_id,
        "efebd441-6d29-4582-934c-9dbadd17a018"
    );
    assert_eq!(
        pc.quotes[0].offer.hourly_cost_usd.to_bits(),
        52.0_f64.to_bits()
    );
    assert_eq!(pc.quotes[0].offer.gpu_count, 8);
    assert_eq!(pc.quotes[0].offer.gpu_class, "nvidia-b300");
    spec.allow_spot = false;
    assert!(provisioner.quotes(&spec).await.unwrap().quotes.is_empty());
    assert!(
        peer.received_requests()
            .await
            .unwrap()
            .iter()
            .all(|request| request.method == "GET")
    );
}
