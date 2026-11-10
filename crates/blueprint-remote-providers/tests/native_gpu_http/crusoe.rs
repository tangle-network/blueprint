//! A synthetic Crusoe v1 peer with the live API's shapes; no stock or price claim.
use super::*;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use std::sync::{Arc, Mutex};
use wiremock::{Request, matchers::path_regex};

const PROJECT: &str = "00000000-0000-4000-8000-00000000c0de";
const KEY_ID: &str = "synthetic-key-id";
const SECRET: &str = "c3ludGhldGljLXNlY3JldC0x";

fn uuid(n: u64) -> String {
    format!("00000000-0000-4000-8000-{n:012}")
}

/// Crusoe's published scheme, recomputed independently of the client.
fn assert_signed(request: &Request) {
    let stamp = request.headers.get("x-crusoe-timestamp").unwrap();
    let mut pairs: Vec<_> = request.url.query_pairs().into_owned().collect();
    pairs.sort();
    let query = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(pairs)
        .finish();
    let message = format!(
        "{}\n{query}\n{}\n{}\n",
        request.url.path(),
        request.method,
        stamp.to_str().unwrap()
    );
    let key = aws_lc_rs::hmac::Key::new(
        aws_lc_rs::hmac::HMAC_SHA256,
        &URL_SAFE_NO_PAD.decode(SECRET).unwrap(),
    );
    let signature = URL_SAFE_NO_PAD.encode(aws_lc_rs::hmac::sign(&key, message.as_bytes()));
    assert_eq!(
        request.headers.get("authorization").unwrap(),
        &format!("Bearer 1.0:{KEY_ID}:{signature}")
    );
}

async fn crusoe_peer() -> MockServer {
    let peer = MockServer::start().await;
    let vm = |id: String, template: Value, location: &str| {
        json!({"id":id,"name":"other","type":"a100-80gb.1x","location":location,"state":"STATE_RUNNING",
            "created_at":"2026-11-10T00:00:00Z","instance_template_id":template,
            "network_interfaces":[{"ips":[{"private_ipv4":{"address":"10.0.0.2"},"public_ipv4":{"address":"192.0.2.21"}}]}]})
    };
    // A foreign template and two foreign VMs, one with no template; list must skip all three.
    let state = Arc::new(Mutex::new((
        3u64,
        std::collections::BTreeMap::from([(
            uuid(1),
            json!({"id":uuid(1),"name":"foreign",
            "startup_script":"#!/bin/bash\necho unrelated\n"}),
        )]),
        std::collections::BTreeMap::from([
            (uuid(2), vm(uuid(2), json!(uuid(1)), "us-test1-a")),
            (uuid(3), vm(uuid(3), Value::Null, "us-test1-a")),
        ]),
        std::collections::BTreeMap::<String, Value>::new(),
    )));
    let compute = format!("/projects/{PROJECT}/compute");
    Mock::given(path_regex("^/(locations|capacities|projects/.+)$"))
        .respond_with(move |request: &Request| {
            assert_signed(request);
            let mut state = state.lock().unwrap();
            let (ref mut next, ref mut templates, ref mut vms, ref mut operations) = *state;
            let path = request.url.path();
            let rest = path.strip_prefix(compute.as_str()).unwrap_or(path);
            // Each accepted change answers IN_PROGRESS; its operation reads SUCCEEDED.
            let accept = |next: &mut u64, operations: &mut std::collections::BTreeMap<String, Value>, result: Value| {
                *next += 1;
                let id = uuid(*next);
                operations.insert(id.clone(), json!({"operation_id":id,"state":"SUCCEEDED","result":result}));
                ResponseTemplate::new(200).set_body_json(json!({"operation":{"operation_id":id,"state":"IN_PROGRESS"}}))
            };
            match (request.method.as_str(), rest) {
                ("GET", "/locations") => ResponseTemplate::new(200).set_body_json(json!({"items":["us-test1-a","eu-test1-a"]})),
                ("GET", "/capacities") => ResponseTemplate::new(200).set_body_json(json!({"items":[
                    {"type":"a100-80gb.1x","location":"us-test1-a","quantity":1},
                    {"type":"a100-80gb.1x","location":"eu-test1-a","quantity":0},
                    {"type":"a100-80gb.1x","location":"us-unlisted1-a","quantity":1},
                    {"type":"a100-80gb.2x","location":"us-test1-a","quantity":1},
                    {"type":"mi300x-192gb-ib.8x","location":"us-test1-a","quantity":1}]})),
                ("GET", "/vms/types") => ResponseTemplate::new(200).set_body_json(json!({"items":[
                    {"product_name":"a100-80gb.1x","gpu_type":"A100-PCIe-80GB","num_gpu":1,"cpu_cores":12,"memory_gb":120,"disk_gb":0},
                    {"product_name":"a100-80gb.2x","gpu_type":"A100-PCIe-80GB","num_gpu":2,"cpu_cores":24,"memory_gb":240,"disk_gb":0},
                    {"product_name":"l40s-48gb.1x","gpu_type":"L40S-48GB","num_gpu":1,"cpu_cores":8,"memory_gb":147,"disk_gb":0},
                    {"product_name":"mi300x-192gb-ib.8x","gpu_type":"MI300X-192GB","num_gpu":8,"cpu_cores":160,"memory_gb":1800,"disk_gb":0}]})),
                ("GET", "/instance-templates") => ResponseTemplate::new(200).set_body_json(json!({"items":templates.values().collect::<Vec<_>>()})),
                ("POST", "/instance-templates") => {
                    let body: Value = request.body_json().unwrap();
                    *next += 1;
                    let id = uuid(*next);
                    let template = json!({"id":id,"name":body["template_name"],"type":body["type"],"image_name":body["image_name"],
                        "ssh_public_key":body["ssh_public_key"],"startup_script":body["startup_script"]});
                    templates.insert(id, template.clone());
                    ResponseTemplate::new(200).set_body_json(template)
                }
                ("POST", "/vms/bulk-instances") => {
                    let body: Value = request.body_json().unwrap();
                    let template = &templates[body["instance_template_id"].as_str().unwrap()];
                    assert_eq!(body["count"], 1);
                    *next += 1;
                    let mut created = vm(uuid(*next), body["instance_template_id"].clone(), body["locations"][0]["location"].as_str().unwrap());
                    created["type"] = template["type"].clone();
                    created["name"] = json!(format!("{}-1", body["name_prefix"].as_str().unwrap()));
                    vms.insert(uuid(*next), created.clone());
                    accept(next, operations, json!([created]))
                }
                // One VM per page, so every listing follows `next_page_token`.
                ("GET", "/vms/instances") => {
                    let page: usize = request.url.query_pairs().find(|(key, _)| key == "next_token")
                        .map_or(0, |(_, token)| token.parse().unwrap());
                    let token = if page + 1 < vms.len() { (page + 1).to_string() } else { String::new() };
                    ResponseTemplate::new(200).set_body_json(json!({"items":vms.values().skip(page).take(1).collect::<Vec<_>>(),"next_page_token":token}))
                }
                ("GET", rest) if rest.starts_with("/vms/instances/operations/") =>
                    ResponseTemplate::new(200).set_body_json(&operations[rest.rsplit('/').next().unwrap()]),
                ("GET", rest) => {
                    let id = rest.rsplit('/').next().unwrap();
                    let raw = if rest.starts_with("/instance-templates/") { templates.get(id) } else { vms.get(id) };
                    raw.map_or_else(|| ResponseTemplate::new(404), |raw| ResponseTemplate::new(200).set_body_json(raw))
                }
                ("DELETE", rest) if rest.starts_with("/instance-templates/") => {
                    assert!(!vms.values().any(|vm| vm["instance_template_id"] == rest.rsplit('/').next().unwrap()));
                    templates.remove(rest.rsplit('/').next().unwrap());
                    ResponseTemplate::new(200)
                }
                ("DELETE", rest) => match vms.remove(rest.rsplit('/').next().unwrap()) {
                    Some(_) => accept(next, operations, Value::Null),
                    None => ResponseTemplate::new(404),
                },
                _ => ResponseTemplate::new(404),
            }
        })
        .mount(&peer)
        .await;
    peer
}

fn provisioner(peer: &MockServer) -> CloudProvisioner {
    let credentials = serde_json::from_value(json!({"provider":"crusoe","access_key_id":KEY_ID,
        "secret_key":SECRET,"project_id":PROJECT,"hourly_usd":"a100-80gb.1x=2.06,l40s-48gb.1x=1.24",
        "endpoint":peer.uri()}))
    .unwrap();
    CloudProvisioner::from_credentials(vec![credentials]).unwrap()
}

#[tokio::test]
async fn crusoe_lease_is_recovered_and_reaped_from_provider_state_alone() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = crusoe_peer().await;
    let mut spec = ResourceSpec::basic();
    spec.gpu_class = Some("nvidia-a100-80gb".into());
    spec.gpu_count = Some(1);
    spec.gpu_vram_gb = Some(80);
    spec.image = Some("example.invalid/workload:v1".into());
    spec.env
        .insert("MODEL_KEY".into(), "synthetic-bootstrap-secret".into());
    spec.ports = vec![8000];
    spec.endpoint_port = Some(8000);
    spec.lease_duration_secs = Some(3600);
    spec.ssh_public_key = Some(
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMo32AY0vFK7g5FBIBcyPdxaxSEM5rEc0kEzMVveA9b+".into(),
    );
    spec.ownership = Some(ManagedLeaseIdentity {
        namespace: "synthetic-cell".into(),
        lease_id: "crusoe-one".into(),
    });
    let first = provisioner(&peer);
    let book = first.quotes(&spec).await.unwrap();
    assert!(book.errors.is_empty());
    let offers: Vec<_> = book
        .quotes
        .iter()
        .map(|quote| {
            let offer = &quote.offer;
            (
                offer.instance_type.as_str(),
                offer.region.as_str(),
                offer.hourly_cost_usd,
                offer.pricing_source.as_str(),
            )
        })
        .collect();
    // Only the priced, stocked, listed and exactly sized type is offered.
    assert_eq!(offers, [("a100-80gb.1x", "us-test1-a", 2.06, "configured")]);
    let created = first
        .create_managed(&spec, &book.quotes[0].candidate)
        .await
        .unwrap();
    let requests = peer.received_requests().await.unwrap();
    let template: Value = requests
        .iter()
        .find(|request| {
            request.method == "POST" && request.url.path().ends_with("/instance-templates")
        })
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(template["image_name"], "ubuntu22.04-nvidia-pcie-docker");
    let script = template["startup_script"].as_str().unwrap();
    assert!(script.lines().nth(1).unwrap().starts_with("# bpg1:{"));
    assert!(!script.contains("synthetic-bootstrap-secret"));
    #[cfg(unix)]
    bootstrap::execute_startup_script(script);

    // A fresh provisioner holds no local state: the provider alone names the lease.
    let second = provisioner(&peer);
    let identity = spec.ownership.as_ref().unwrap();
    let listed = second.list_managed("synthetic-cell").await.unwrap();
    assert_eq!(
        listed.len(),
        2,
        "the VM and its template, never the foreign objects"
    );
    for resource in &listed {
        assert!(resource.held_by(identity));
        assert_eq!(resource.request_fingerprint, created.request_fingerprint);
        assert_eq!(resource.expires_at, created.expires_at);
    }
    let status = second
        .status_managed(&CloudProvider::Crusoe, identity, &created.instance.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        (
            status.gpu_class.as_deref(),
            status.gpu_count,
            status.gpu_vram_gb,
            status.hourly_cost_usd
        ),
        (Some("nvidia-a100-80gb"), Some(1), Some(80), Some(2.06))
    );
    assert_eq!(
        status.ssh_endpoint.as_deref(),
        Some("ssh://ubuntu@192.0.2.21:22")
    );
    second
        .destroy_managed(&CloudProvider::Crusoe, identity, None)
        .await
        .unwrap();
    assert!(
        second
            .list_managed("synthetic-cell")
            .await
            .unwrap()
            .is_empty()
    );
    let deletes: Vec<_> = peer
        .received_requests()
        .await
        .unwrap()
        .into_iter()
        .filter(|request| request.method == "DELETE")
        .map(|request| request.url.path().contains("/instance-templates/"))
        .collect();
    assert_eq!(
        deletes,
        [false, true],
        "the VM goes before the template that names its lease"
    );
}
