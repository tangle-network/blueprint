//! Shared native RunPod HTTP peer; all resources are process-local fixtures.
use serde_json::{Value, json};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub(crate) type RunpodPeerState =
    std::sync::Arc<std::sync::Mutex<(u64, std::collections::BTreeMap<String, Value>, Vec<Value>)>>;

pub(crate) async fn runpod_peer_state() -> (MockServer, RunpodPeerState) {
    runpod_peer_state_at("").await
}

/// Prefixes are fixed test literals; the response body is shared with native tests.
pub(crate) async fn runpod_peer_state_at(prefix: &'static str) -> (MockServer, RunpodPeerState) {
    use std::sync::{Arc, Mutex};
    use wiremock::{Request, matchers::path_regex};
    let peer = MockServer::start().await;
    let state = Arc::new(Mutex::new((
        0u64,
        std::collections::BTreeMap::<String, Value>::new(),
        ["NVIDIA H200", "NVIDIA B200"]
            .map(|sku| {
                json!({
                    "id":sku,"secure":true,"community":false,
                    "maxCount":{"secure":8,"community":0},"price":{"secure":4.5,"community":0},
                    "availability":"HIGH","dataCenters":[{"id":"US-TEST-1","availability":"HIGH"}]
                })
            })
            .to_vec(),
    )));
    let handle = state.clone();
    Mock::given(path_regex(format!("^{prefix}/(catalog/gpus|pods(/[^/]+)?)$")))
        .respond_with(move |request: &Request| {
            if request.headers.get("authorization").and_then(|header| header.to_str().ok()) != Some("Bearer synthetic-local-peer-key") {
                return ResponseTemplate::new(401).set_body_string("synthetic-local-peer-key");
            }
            let mut state = state.lock().unwrap();
            let path = request.url.path().strip_prefix(prefix).unwrap_or("");
            match (request.method.as_str(), path) {
                ("GET", "/catalog/gpus") => ResponseTemplate::new(200).set_body_json(json!({"gpus":state.2})),
                ("POST", "/pods") => {
                    let Ok(body) = request.body_json::<Value>() else { return ResponseTemplate::new(400); };
                    state.0 += 1;
                    let id = format!("synthetic{}", state.0);
                    let price=state.2.iter().find(|offer|offer["id"]==body["gpu"]["id"]).unwrap()["price"]["secure"].as_f64().unwrap()*body["gpu"]["count"].as_f64().unwrap();
                    let pod = json!({
                        "id":id,"name":body["name"],"gpu":{"id":body["gpu"]["id"],"count":body["gpu"]["count"],"vcpuCount":8,"memory":64},
                        "env":body["env"],"ports":body["ports"],"mounts":body["mounts"],"image":body["image"],
                        "status":"RUNNING","dataCenterId":"US-TEST-1","cost":price,
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
    (peer, handle)
}
