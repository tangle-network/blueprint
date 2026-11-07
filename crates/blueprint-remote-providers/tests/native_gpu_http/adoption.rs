//! Independent provider HTTP probes; all resources and credentials are synthetic.
use super::*;
use blueprint_remote_providers::managed::ProviderErrorCode;

async fn runpod_adoption_probe(field: &str, changed: Value, reject: bool, cap: Option<f64>) {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = runpod_v2_peer().await;
    let credentials = serde_json::from_value(json!({
        "provider":"runpod", "api_key":"synthetic-local-peer-key", "endpoint":peer.uri()
    }))
    .unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let mut spec = ResourceSpec::basic();
    spec.ownership = Some(ManagedLeaseIdentity {
        namespace: format!("review-{}", field.replace('/', "-")),
        lease_id: "adoption-proof".into(),
    });
    spec.lease_duration_secs = Some(3600);
    spec.gpu_class = Some("nvidia-h200".into());
    spec.gpu_count = Some(2);
    spec.gpu_vram_gb = Some(140);
    spec.image = Some("example.invalid/review:v1".into());
    spec.region = Some("US-TEST-1".into());
    spec.max_hourly_cost_usd = cap;
    let quote = provisioner.quotes(&spec).await.unwrap().quotes.remove(0);
    let original = provisioner
        .create_managed(&spec, &quote.candidate)
        .await
        .unwrap();
    let mut reported: Value = reqwest::Client::new()
        .get(format!("{}/pods/{}", peer.uri(), original.instance.id))
        .bearer_auth("synthetic-local-peer-key")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    *reported.pointer_mut(field).unwrap() = changed;
    Mock::given(method("GET"))
        .and(path("/pods"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "pods":[reported],"pagination":{"hasNextPage":false,"nextCursor":null}
        })))
        .with_priority(1)
        .mount(&peer)
        .await;
    let result = provisioner.create_managed(&spec, &quote.candidate).await;
    let requests = peer.received_requests().await.unwrap();
    assert_eq!(
        requests
            .iter()
            .filter(|request| request.method == "POST" && request.url.path() == "/pods")
            .count(),
        1,
        "an adoption conflict must never rent another VM"
    );
    if reject {
        assert!(
            result.is_err(),
            "changed native assignment {field} was adopted: {result:?}"
        );
    } else {
        assert_eq!(result.unwrap().instance.id, original.instance.id);
    }
}

#[tokio::test]
async fn review_adoption_preserves_the_assignment_and_both_price_ceilings() {
    for (field, changed, reject, cap) in [
        ("/gpu/id", json!("NVIDIA GeForce RTX 3090"), true, Some(9.0)),
        ("/gpu/count", json!(1), true, Some(9.0)),
        ("/dataCenterId", json!("US-OTHER"), true, Some(9.0)),
        ("/cost", json!(9.01), true, Some(9.0)),
        ("/cost", json!(9.01), true, Some(90.0)),
        ("/cost", json!(9.01), true, None),
        ("/cost", json!(9.0), false, Some(90.0)),
        ("/cost", json!(9.0), false, None),
    ] {
        runpod_adoption_probe(field, changed, reject, cap).await;
    }
}

async fn transport_probe(status: u16, expected: ProviderErrorCode) {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    for provider in ["runpod", "lambda-labs", "vast-ai", "bittensor-lium"] {
        let peer = MockServer::start().await;
        let destination = MockServer::start().await;
        Mock::given(wiremock::matchers::any())
            .respond_with(
                ResponseTemplate::new(status)
                    .insert_header("Location", destination.uri())
                    .set_body_string("synthetic-adversarial-provider-key"),
            )
            .mount(&peer)
            .await;
        let credential = serde_json::from_value(json!({
            "provider":provider,"api_key":"synthetic-adversarial-provider-key","endpoint":peer.uri()
        }))
        .unwrap();
        let provisioner = CloudProvisioner::from_credentials(vec![credential]).unwrap();
        let mut spec = ResourceSpec::basic();
        spec.gpu_class = Some("nvidia-h200".into());
        spec.gpu_count = Some(2);
        spec.gpu_vram_gb = Some(140);
        spec.lease_duration_secs = Some(3600);
        let quotes = provisioner.quotes(&spec).await.unwrap();
        assert!(quotes.quotes.is_empty());
        assert_eq!(quotes.errors.len(), 1);
        assert_eq!(
            quotes.errors[0].code, expected,
            "wrong closed failure for {provider}"
        );
        assert!(
            !serde_json::to_string(&quotes)
                .unwrap()
                .contains("synthetic-adversarial-provider-key")
        );
        let requests = peer.received_requests().await.unwrap();
        assert_eq!(requests.len(), 1, "{provider} must reach the HTTP boundary");
        assert!(requests[0].headers.values().any(|value| {
            value
                .to_str()
                .is_ok_and(|value| value.contains("synthetic-adversarial-provider-key"))
        }));
        assert!(
            destination.received_requests().await.unwrap().is_empty(),
            "credentials must never follow a provider redirect"
        );
    }
}

#[tokio::test]
async fn review_redirects_never_forward_the_four_provider_credentials() {
    transport_probe(302, ProviderErrorCode::ProviderError).await;
}

#[tokio::test]
async fn review_authentication_errors_never_reflect_provider_body_secrets() {
    transport_probe(401, ProviderErrorCode::AuthRejected).await;
}
