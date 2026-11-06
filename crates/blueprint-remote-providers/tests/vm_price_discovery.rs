#![allow(clippy::float_cmp)]
//! VM selection through the exported discovery boundary, without provider effects.
use blueprint_remote_providers::CloudProvider;
use blueprint_remote_providers::monitoring::discovery::MachineTypeDiscovery;
use blueprint_remote_providers::pricing::PricingFetcher;

#[tokio::test]
async fn a_provider_without_a_live_quote_cannot_win_vm_selection() {
    let mut discovery = MachineTypeDiscovery::new();
    if let Ok(machines) = discovery
        .discover_machine_types(&CloudProvider::AWS, "us-east-1", &PricingFetcher::default())
        .await
    {
        assert!(
            machines
                .iter()
                .all(|machine| machine.hourly_price.is_none() && machine.spot_price.is_none()),
            "discovery presented handwritten fallback prices as provider quotes"
        );
    }
    assert!(
        discovery
            .find_best_match(&CloudProvider::AWS, 2, 1.0, false, Some(10.0))
            .is_none(),
        "an unpriced VM remained selectable with a caller spending cap"
    );
    assert!(
        discovery
            .find_best_match(&CloudProvider::AWS, 2, 1.0, false, None)
            .is_none(),
        "removing a spending cap made an unknown price selectable"
    );
}

use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use wiremock::{
    Mock, MockServer, Request, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn provider_refresh_replaces_prices_and_never_retains_a_missing_quote() {
    let server = MockServer::start().await;
    let calls = Arc::new(AtomicUsize::new(0));
    let observed = calls.clone();
    Mock::given(method("GET")).and(path("/v2/sizes"))
        .respond_with(move |_:&Request| {
            let price=match observed.fetch_add(1,Ordering::SeqCst) {0=>json!(1.0),1=>json!(2.0),_=>serde_json::Value::Null};
            ResponseTemplate::new(200).set_body_json(json!({"sizes":[{"slug":"s-2vcpu-4gb",
                "available":true,"regions":["nyc3"],"vcpus":2,"memory":4096,"price_hourly":price}]}))
        }).mount(&server).await;
    let pricing = PricingFetcher::new()
        .unwrap()
        .with_digitalocean_token("synthetic-rate-token".into())
        .unwrap()
        .with_endpoint(CloudProvider::DigitalOcean, &server.uri())
        .unwrap();
    let mut discovery = MachineTypeDiscovery::new();
    for expected in [Some(1.0), Some(2.0), None] {
        discovery
            .discover_machine_types(&CloudProvider::DigitalOcean, "nyc3", &pricing)
            .await
            .unwrap();
        let selected =
            discovery.find_best_match(&CloudProvider::DigitalOcean, 2, 4.0, false, Some(3.0));
        assert_eq!(selected.and_then(|machine| machine.hourly_price), expected);
    }
    assert_eq!(calls.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn automatic_selection_quotes_the_sku_it_will_create_and_has_no_default_provider_price() {
    use blueprint_remote_providers::{
        ResourceSpec,
        infra::auto::{AutoDeploymentManager, EnabledProvider},
    };
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/v2/sizes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"sizes":[
            {"slug":"s-1vcpu-1gb","available":true,"regions":["nyc3"],"vcpus":1,"memory":1024,"price_hourly":0.001},
            {"slug":"s-2vcpu-4gb","available":true,"regions":["nyc3"],"vcpus":2,"memory":4096,"price_hourly":1.25}
        ]}))).mount(&server).await;
    let pricing = PricingFetcher::new()
        .unwrap()
        .with_digitalocean_token("synthetic-rate-token".into())
        .unwrap()
        .with_endpoint(CloudProvider::DigitalOcean, &server.uri())
        .unwrap();
    let mut manager = AutoDeploymentManager::new().with_pricing_fetcher(pricing);
    manager.set_max_hourly_cost(2.0);
    assert!(
        manager
            .find_cheapest_provider(&ResourceSpec::basic())
            .await
            .is_err(),
        "no configured providers invented an AWS allocation and price"
    );
    manager
        .configure_providers(vec![EnabledProvider {
            provider: CloudProvider::DigitalOcean,
            region: "nyc3".into(),
            credentials_env: Default::default(),
            enabled: true,
            priority: 1,
        }])
        .await;
    let (provider, region, price) = manager
        .find_cheapest_provider(&ResourceSpec::basic())
        .await
        .unwrap();
    assert_eq!(provider, CloudProvider::DigitalOcean);
    assert_eq!(region, "nyc3");
    assert_eq!(
        price, 1.25,
        "selection quoted a cheap SKU that it would not provision"
    );
}
