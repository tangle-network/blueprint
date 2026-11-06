#![allow(clippy::float_cmp)]
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
                if let Some(end) = input.windows(4).position(|part| part == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&input[..end]);
                    let length = headers
                        .lines()
                        .filter_map(|line| line.split_once(':'))
                        .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
                        .and_then(|(_, value)| value.trim().parse::<usize>().ok())
                        .unwrap_or(0);
                    if input.len() >= end + 4 + length {
                        break;
                    }
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
        .with_digitalocean_token("synthetic-token".into())
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
async fn vultr_monthly_cap_is_not_mistaken_for_a_complete_hourly_quote() {
    let row =
        json!({"id":"vc2-2c-4gb","locations":["ewr"],"vcpu_count":2,"ram":4096,"monthly_cost":10});
    let (url, requests) = peer(vec![json!({"plans":[row]})]).await;
    let fetcher = PricingFetcher::new()
        .unwrap()
        .with_vultr_token("synthetic-vultr-token".into())
        .unwrap()
        .with_endpoint(CloudProvider::Vultr, &url)
        .unwrap();
    assert_eq!(
        fetcher
            .quote_instance(CloudProvider::Vultr, "ewr", "vc2-2c-4gb")
            .await
            .unwrap(),
        None
    );
    let requests = requests.await.unwrap();
    assert!(
        requests[0]
            .to_ascii_lowercase()
            .contains("authorization: bearer synthetic-vultr-token")
    );
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
        .with_digitalocean_token("synthetic-token".into())
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

#[cfg(feature = "aws")]
#[tokio::test]
async fn aws_query_signs_exact_sku_requests_and_reads_fresh_whole_node_rates() {
    fn product(price: &str) -> String {
        json!({"product":{"attributes":{"instanceType":"m6i.xlarge","regionCode":"us-east-1",
        "operatingSystem":"Linux","tenancy":"Shared","preInstalledSw":"NA","capacitystatus":"Used",
        "vcpu":"4","memory":"16 GiB"}},"terms":{"OnDemand":{"term":{"priceDimensions":{
            "rate":{"unit":"Hrs","beginRange":"0","endRange":"Inf","pricePerUnit":{"USD":price}}
        }}}}})
        .to_string()
    }
    let (url, requests) = peer(vec![
        json!({"PriceList":[product("0.19")]}),
        json!({"PriceList":[product("0.27")]}),
    ])
    .await;
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("us-east-1"))
        .credentials_provider(aws_sdk_pricing::config::Credentials::new(
            "synthetic-access",
            "synthetic-secret",
            None,
            None,
            "http-peer",
        ))
        .endpoint_url(&url)
        .load()
        .await;
    let pricing = PricingFetcher::new()
        .unwrap()
        .with_aws_config(&config)
        .unwrap();
    for expected in [0.19, 0.27] {
        assert_eq!(
            pricing
                .quote_instance(CloudProvider::AWS, "us-east-1", "m6i.xlarge")
                .await
                .unwrap(),
            Some(expected)
        );
    }
    for request in requests.await.unwrap() {
        let (headers, body) = request.split_once("\r\n\r\n").unwrap();
        assert!(
            headers
                .to_ascii_lowercase()
                .contains("authorization: aws4-hmac-sha256")
        );
        assert!(
            headers
                .to_ascii_lowercase()
                .contains("x-amz-target: awspricelistservice.getproducts")
        );
        let body: Value = serde_json::from_str(body).unwrap();
        assert_eq!(body["ServiceCode"], "AmazonEC2");
        let filters = body["Filters"].as_array().unwrap();
        assert!(
            filters
                .iter()
                .any(|filter| filter["Field"] == "instanceType" && filter["Value"] == "m6i.xlarge")
        );
        assert!(
            filters
                .iter()
                .any(|filter| filter["Field"] == "regionCode" && filter["Value"] == "us-east-1")
        );
    }
}

#[tokio::test]
async fn rejected_pricing_credentials_are_errors_without_reflected_secret_bodies() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let responder = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut request = [0; 4096];
        assert!(stream.read(&mut request).await.unwrap() > 0);
        let body = "synthetic-account-token";
        stream.write_all(format!("HTTP/1.1 401 Unauthorized\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
    });
    let pricing = PricingFetcher::new()
        .unwrap()
        .with_digitalocean_token("synthetic-account-token".into())
        .unwrap()
        .with_endpoint(CloudProvider::DigitalOcean, &url)
        .unwrap();
    let error = pricing
        .quote_instance(CloudProvider::DigitalOcean, "nyc3", "s-2vcpu-4gb")
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        blueprint_pricing_engine_lib::PricingError::AuthRejected
    ));
    assert!(!error.to_string().contains("synthetic-account-token"));
    assert!(!format!("{pricing:?}").contains("synthetic-account-token"));
    responder.await.unwrap();
}
