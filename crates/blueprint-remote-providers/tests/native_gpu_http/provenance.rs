//! SDK-test inspection only: production CLI never decodes candidate bytes.
use super::*;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use blueprint_remote_providers::managed::ProviderErrorCode;

#[tokio::test]
async fn measured_quotes_project_public_receipts_and_preserve_private_intent() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let (peer, state) =
        runpod_fixture::runpod_peer_state_at(measured_recipe::PRIVATE_ENDPOINT).await;
    state.lock().unwrap().2[1]["price"]["secure"] = json!(3.0);
    let credentials = serde_json::from_value(
        json!({"provider":"runpod","api_key":"synthetic-local-peer-key",
        "endpoint":format!("{}{}",peer.uri(),measured_recipe::PRIVATE_ENDPOINT)}),
    )
    .unwrap();
    let root = tempfile::tempdir().unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials])
        .unwrap()
        .initialize_lease_state_dir(root.path())
        .await
        .unwrap();
    let mut spec = measured_recipe::measured_spec("provenance-cell", "lease");
    let small = provisioner.quotes(&spec).await.unwrap();
    let unselected = spec.arms[0].measurements[0].clone();
    spec.arms[0].measurements.extend(vec![unselected; 256]);
    let original = measured_recipe::recipe_value(&spec);
    let quotes = provisioner.quotes(&spec).await.unwrap();
    assert_eq!(quotes.quotes.len(), 2);
    assert_eq!(
        quotes.quotes[0].candidate.len(),
        small.quotes[0].candidate.len(),
        "private recipe size must not enlarge the public selector"
    );
    assert_ne!(
        quotes.quotes[0].candidate, small.quotes[0].candidate,
        "the complete private input still changes its commitment"
    );
    assert_eq!(quotes.quotes[0].offer.gpu_class, "nvidia-h200");
    assert_eq!(
        quotes.quotes[0].offer.hourly_cost_usd.to_bits(),
        9.0f64.to_bits()
    );
    assert_eq!(
        quotes.quotes[1].offer.hourly_cost_usd.to_bits(),
        6.0f64.to_bits()
    );
    assert_eq!(quotes.quotes[0].output_tokens_per_second, Some(1000.0));
    assert_eq!(quotes.quotes[0].usd_per_million_output_tokens, Some(2.5));
    let receipt = quotes.quotes[0].measurement.as_ref().unwrap();
    assert_eq!(
        serde_json::to_value(receipt).unwrap(),
        json!({
            "weights_manifest_sha256":"aa".repeat(32),"raw_result_sha256":"11".repeat(32),
            "measured_at":"2026-11-07T00:00:00Z","input_tokens":6000,"output_tokens":1000,
            "elapsed_ms":1000,"concurrency":8,"tp":2,"replicas":1
        })
    );
    let output = serde_json::to_string(&quotes).unwrap();
    for quote in &quotes.quotes {
        assert_eq!(quote.offer.pricing_source, peer.uri());
        let decoded = String::from_utf8(URL_SAFE_NO_PAD.decode(&quote.candidate).unwrap()).unwrap();
        let selector: Value = serde_json::from_str(&decoded).unwrap();
        assert!(
            selector["arm"]["measurements"]
                .as_array()
                .unwrap()
                .is_empty()
        );
        for canary in measured_recipe::CANARIES
            .into_iter()
            .chain(["PRIVATE_ENDPOINT_CANARY"])
        {
            assert!(!output.contains(canary) && !decoded.contains(canary));
        }
    }
    assert_eq!(measured_recipe::recipe_value(&spec), original);
    let selected = &quotes.quotes[0];
    let mut changed = spec.clone();
    changed.arms[0].measurements[0]
        .raw_result_uri
        .push_str("changed");
    assert!(matches!(
        provisioner
            .create_managed(&changed, &selected.candidate)
            .await,
        Err(blueprint_remote_providers::Error::Provider(
            ProviderErrorCode::Conflict
        ))
    ));
    // Mutate a fresh public selector only; no legacy authority or fingerprint is forged.
    let mut altered: Value =
        serde_json::from_slice(&URL_SAFE_NO_PAD.decode(&selected.candidate).unwrap()).unwrap();
    altered["arm"]["arm"] = json!("not-an-original-selector");
    let altered = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&altered).unwrap());
    assert!(matches!(
        provisioner.create_managed(&spec, &altered).await,
        Err(blueprint_remote_providers::Error::Provider(
            ProviderErrorCode::InvalidRequest
        ))
    ));
    assert_eq!(state.lock().unwrap().0, 0);
    let resource = provisioner
        .create_managed(&spec, &selected.candidate)
        .await
        .unwrap();
    let expires = resource.expires_at;
    let recovered = provisioner
        .recover_lease(&spec, std::time::Duration::from_secs(3600))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(recovered.resource.expires_at, expires);
    assert_eq!(state.lock().unwrap().0, 1);
    provisioner
        .destroy_managed(
            &CloudProvider::RunPod,
            spec.ownership.as_ref().unwrap(),
            Some(&resource.instance.id),
        )
        .await
        .unwrap();
}
