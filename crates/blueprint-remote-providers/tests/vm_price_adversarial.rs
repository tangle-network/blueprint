//! Candidate-local unknown prices must not hide other live provider quotes.
use blueprint_remote_providers::{
    CloudProvider, ResourceSpec,
    infra::auto::{AutoDeploymentManager, EnabledProvider},
    pricing::PricingFetcher,
};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

fn provider(provider: CloudProvider, region: &str) -> EnabledProvider {
    EnabledProvider {
        provider,
        region: region.into(),
        credentials_env: Default::default(),
        enabled: true,
        priority: 1,
    }
}

#[tokio::test]
async fn an_unpriced_candidate_cannot_discard_another_providers_live_cpu_quote() {
    let peer = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/sizes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"sizes":[{
            "slug":"s-2vcpu-4gb","available":true,"regions":["nyc3"],
            "vcpus":2,"memory":4096,"price_hourly":0.25
        }]})))
        .mount(&peer)
        .await;
    let fetcher = PricingFetcher::new()
        .unwrap()
        .with_digitalocean_token("synthetic-review-token".into())
        .unwrap()
        .with_endpoint(CloudProvider::DigitalOcean, &peer.uri())
        .unwrap();
    let mut manager = AutoDeploymentManager::new().with_pricing_fetcher(fetcher);
    manager.set_max_hourly_cost(1.0);
    manager
        .configure_providers(vec![
            provider(CloudProvider::DigitalOcean, "nyc3"),
            provider(CloudProvider::Hetzner, "fsn1"),
        ])
        .await;
    let result = manager.find_cheapest_provider(&ResourceSpec::basic()).await;
    assert_eq!(peer.received_requests().await.unwrap().len(), 1);
    let (selected, region, price) = result.expect(
        "an unsupported price source must exclude that candidate, not discard an already observed live quote",
    );
    assert_eq!(selected, CloudProvider::DigitalOcean);
    assert_eq!(region, "nyc3");
    assert_eq!(price, 0.25);
}

#[tokio::test]
async fn rejected_credentials_still_abort_comparison_without_reflecting_the_response() {
    let peer = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/sizes"))
        .respond_with(ResponseTemplate::new(403).set_body_string("synthetic-review-token"))
        .mount(&peer)
        .await;
    let fetcher = PricingFetcher::new()
        .unwrap()
        .with_digitalocean_token("synthetic-review-token".into())
        .unwrap()
        .with_endpoint(CloudProvider::DigitalOcean, &peer.uri())
        .unwrap();
    let manager = AutoDeploymentManager::new().with_pricing_fetcher(fetcher);
    manager
        .configure_providers(vec![provider(CloudProvider::DigitalOcean, "nyc3")])
        .await;
    let error = manager
        .find_cheapest_provider(&ResourceSpec::basic())
        .await
        .unwrap_err();
    assert!(error.to_string().contains("rejected"));
    assert!(!error.to_string().contains("synthetic-review-token"));
    assert_eq!(peer.received_requests().await.unwrap().len(), 1);
}
