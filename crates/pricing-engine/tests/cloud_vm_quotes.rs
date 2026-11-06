use blueprint_pricing_engine_lib::{PricingFetcher, types::CloudProvider};
use serde_json::{Value, json};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

async fn peer(responses: Vec<Value>) -> (String, tokio::task::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let requests = tokio::spawn(async move {
        let mut requests = Vec::new();
        for body in responses {
            let (mut stream, _) =
                tokio::time::timeout(std::time::Duration::from_secs(5), listener.accept())
                    .await
                    .unwrap()
                    .unwrap();
            let mut input = Vec::new();
            loop {
                let mut buffer = [0; 1024];
                let count = stream.read(&mut buffer).await.unwrap();
                assert!(count > 0);
                input.extend_from_slice(&buffer[..count]);
                if input.windows(4).any(|b| b == b"\r\n\r\n") {
                    break;
                }
            }
            requests.push(String::from_utf8(input).unwrap());
            let body = body.to_string();
            stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).as_bytes()).await.unwrap();
        }
        requests
    });
    (url, requests)
}

fn droplet(price: Value) -> Value {
    json!({"slug":"gpu-test-8","available":true,"regions":["nyc3"],"vcpus":16,"memory":65536,"price_hourly":price})
}

#[tokio::test]
async fn digitalocean_exact_sku_is_fresh_and_uses_caller_token() {
    let (url, requests) = peer(vec![
        json!({"sizes":[droplet(json!(4.0))]}),
        json!({"sizes":[droplet(json!(7.0))]}),
    ])
    .await;
    let fetcher = PricingFetcher::new()
        .unwrap()
        .with_token(CloudProvider::DigitalOcean, "synthetic-token".into())
        .unwrap()
        .with_endpoint(CloudProvider::DigitalOcean, &url)
        .unwrap();
    assert_eq!(
        fetcher
            .quote_instance(CloudProvider::DigitalOcean, "nyc3", "gpu-test-8")
            .await
            .unwrap(),
        Some(4.0)
    );
    assert_eq!(
        fetcher
            .quote_instance(CloudProvider::DigitalOcean, "nyc3", "gpu-test-8")
            .await
            .unwrap(),
        Some(7.0)
    );
    for request in requests.await.unwrap() {
        assert!(request.starts_with("GET /v2/sizes?per_page=200 "));
        assert!(
            request
                .to_ascii_lowercase()
                .contains("authorization: bearer synthetic-token")
        );
        assert!(!request.lines().next().unwrap().contains("synthetic-token"));
    }
}

#[tokio::test]
async fn unknown_unavailable_wrong_region_and_missing_prices_are_unselectable() {
    let mut unavailable = droplet(json!(1));
    unavailable["available"] = json!(false);
    let mut wrong_region = droplet(json!(1));
    wrong_region["regions"] = json!(["ams3"]);
    let mut unknown = droplet(json!(1));
    unknown["slug"] = json!("another-sku");
    let (url, requests) = peer(vec![json!({"sizes":[unavailable,wrong_region,unknown,droplet(Value::Null),droplet(json!(0)),droplet(json!("NaN"))]})]).await;
    let fetcher = PricingFetcher::new()
        .unwrap()
        .with_endpoint(CloudProvider::DigitalOcean, &url)
        .unwrap();
    assert_eq!(
        fetcher
            .quote_instance(CloudProvider::DigitalOcean, "nyc3", "gpu-test-8")
            .await
            .unwrap(),
        None
    );
    requests.await.unwrap();
}

#[tokio::test]
async fn vultr_uses_region_hourly_rate_without_monthly_conversion() {
    let row = json!({"id":"gpu-test-8","locations":["ewr"],"deploy_ondemand":true,"vcpu_count":16,"ram":65536,
        "monthly_cost":1,"hourly_cost":12,"location_cost":{"ewr":{"hourly_cost":14.25}}});
    let mut missing = row.clone();
    missing["location_cost"]["ewr"]["hourly_cost"] = Value::Null;
    let (url, requests) = peer(vec![json!({"plans":[row]}), json!({"plans":[missing]})]).await;
    let fetcher = PricingFetcher::new()
        .unwrap()
        .with_endpoint(CloudProvider::Vultr, &url)
        .unwrap();
    assert_eq!(
        fetcher
            .quote_instance(CloudProvider::Vultr, "ewr", "gpu-test-8")
            .await
            .unwrap(),
        Some(14.25)
    );
    assert_eq!(
        fetcher
            .quote_instance(CloudProvider::Vultr, "ewr", "gpu-test-8")
            .await
            .unwrap(),
        None
    );
    requests.await.unwrap();
}

#[tokio::test]
async fn azure_quotes_only_exact_linux_ondemand_usd_hour() {
    let row = json!({"serviceName":"Virtual Machines","armSkuName":"Standard_Test8","armRegionName":"eastus","currencyCode":"USD","type":"Consumption",
        "unitOfMeasure":"1 Hour","productName":"Virtual Machines Test Series","meterName":"Test8","retailPrice":8.75});
    let mut windows = row.clone();
    windows["productName"] = json!("Virtual Machines Test Series Windows");
    windows["retailPrice"] = json!(0.01);
    let mut spot = row.clone();
    spot["meterName"] = json!("Test8 Spot");
    spot["retailPrice"] = json!(0.02);
    let mut wrong_sku = row.clone();
    wrong_sku["armSkuName"] = json!("Standard_Other8");
    wrong_sku["retailPrice"] = json!(0.03);
    let mut wrong_region = row.clone();
    wrong_region["armRegionName"] = json!("westus");
    wrong_region["retailPrice"] = json!(0.04);
    let (url, requests) = peer(vec![
        json!({"Items":[windows,spot,wrong_sku,wrong_region,row],"NextPageLink":null}),
    ])
    .await;
    let fetcher = PricingFetcher::new()
        .unwrap()
        .with_endpoint(CloudProvider::Azure, &url)
        .unwrap();
    assert_eq!(
        fetcher
            .quote_instance(CloudProvider::Azure, "eastus", "Standard_Test8")
            .await
            .unwrap(),
        Some(8.75)
    );
    assert!(requests.await.unwrap()[0].starts_with("GET /api/retail/prices?currencyCode=USD&"));
}

#[tokio::test]
async fn pagination_cannot_send_credentials_to_another_origin() {
    let (url, requests) = peer(vec![
        json!({"sizes":[],"links":{"pages":{"next":"https://different.example/steal"}}}),
    ])
    .await;
    let fetcher = PricingFetcher::new()
        .unwrap()
        .with_token(CloudProvider::DigitalOcean, "synthetic-token".into())
        .unwrap()
        .with_endpoint(CloudProvider::DigitalOcean, &url)
        .unwrap();
    assert!(
        fetcher
            .quote_instance(CloudProvider::DigitalOcean, "nyc3", "gpu-test-8")
            .await
            .unwrap_err()
            .to_string()
            .contains("origin")
    );
    assert_eq!(requests.await.unwrap().len(), 1);
    assert!(
        PricingFetcher::new()
            .unwrap()
            .with_endpoint(CloudProvider::Vultr, "http://outside.example")
            .is_err()
    );
}

#[tokio::test]
async fn gcp_machine_type_is_not_mispriced_as_one_billing_component() {
    assert_eq!(
        PricingFetcher::new()
            .unwrap()
            .quote_instance(CloudProvider::GCP, "us-central1", "a3-highgpu-8g")
            .await
            .unwrap(),
        None
    );
}
