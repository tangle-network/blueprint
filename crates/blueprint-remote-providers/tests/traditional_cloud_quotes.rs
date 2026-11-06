use blueprint_remote_providers::providers::{
    azure::adapter::AzureAdapter, digitalocean::adapter::DigitalOceanAdapter,
    gcp::adapter::GcpAdapter, vultr::adapter::VultrAdapter,
};
use blueprint_remote_providers::{
    ResourceSpec, infra::traits::CloudProviderAdapter, infra::types::InstanceStatus,
};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_partial_json, header, method, path},
};

fn resources() -> ResourceSpec {
    let mut spec = ResourceSpec::basic();
    spec.ssh_public_key = Some("ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIO5lyvUO/zYHB6a5iuEaYvz6O2/OcPe44RBZI80mIt65 integration".into());
    spec
}

#[tokio::test]
async fn digitalocean_quotes_native_size_and_returns_created_id_before_readiness() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/sizes"))
        .and(header("authorization", "Bearer synthetic-token"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"sizes":[{"slug":"s-2vcpu-4gb","available":true,
            "regions":["nyc3"],"vcpus":2,"memory":4096,"price_hourly":0.05}]})),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v2/droplets"))
        .and(body_partial_json(
            json!({"region":"nyc3","size":"s-2vcpu-4gb","ssh_keys":[]}),
        ))
        .respond_with(
            ResponseTemplate::new(202).set_body_json(json!({"droplet":{"id":123,"status":"new"}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let adapter =
        DigitalOceanAdapter::with_endpoint("synthetic-token".into(), "nyc3".into(), &server.uri())
            .unwrap();
    let quote = adapter.quote(&resources(), "nyc3").await.unwrap();
    assert_eq!(quote.instance_type, "s-2vcpu-4gb");
    assert_eq!(quote.estimated_hourly_cost, Some(0.05));
    let instance = tokio::time::timeout(
        std::time::Duration::from_secs(1),
        adapter.provision_resources(&quote.instance_type, "nyc3", &resources(), false),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(instance.id, "123");
    assert!(matches!(instance.status, InstanceStatus::Starting));
    let requests = server.received_requests().await.unwrap();
    let created: serde_json::Value = serde_json::from_slice(&requests[1].body).unwrap();
    assert!(
        created["user_data"]
            .as_str()
            .unwrap()
            .contains(resources().ssh_public_key.as_ref().unwrap())
    );
    assert!(!created.to_string().contains("synthetic-token"));
}

#[tokio::test]
async fn vultr_quotes_hourly_total_and_does_not_poll_after_creation() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/v2/plans"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"plans":[{"id":"vc2-2c-4gb","locations":["ewr"],
            "deploy_ondemand":true,"vcpu_count":2,"ram":4096,"hourly_cost":0.1,"monthly_cost":1}]}))).expect(1).mount(&server).await;
    Mock::given(method("POST"))
        .and(path("/v2/instances"))
        .and(body_partial_json(
            json!({"region":"ewr","plan":"vc2-2c-4gb"}),
        ))
        .respond_with(
            ResponseTemplate::new(202).set_body_json(json!({"instance":{"id":"created-vultr"}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let adapter = VultrAdapter::with_endpoint("synthetic-token".into(), &server.uri()).unwrap();
    let quote = adapter.quote(&resources(), "ewr").await.unwrap();
    assert_eq!(quote.estimated_hourly_cost, Some(0.1));
    let instance = tokio::time::timeout(
        std::time::Duration::from_secs(1),
        adapter.provision_resources(&quote.instance_type, "ewr", &resources(), false),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(instance.id, "created-vultr");
    assert!(matches!(instance.status, InstanceStatus::Starting));
    let requests = server.received_requests().await.unwrap();
    let created: serde_json::Value = serde_json::from_slice(&requests[1].body).unwrap();
    use base64::Engine;
    let init = base64::engine::general_purpose::STANDARD
        .decode(created["user_data"].as_str().unwrap())
        .unwrap();
    assert!(
        String::from_utf8(init)
            .unwrap()
            .contains(resources().ssh_public_key.as_ref().unwrap())
    );
}

#[tokio::test]
async fn azure_quotes_without_sending_account_token_and_retains_existing_network() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/retail/prices"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"Items":[{"armSkuName":"Standard_B2ms","armRegionName":"eastus",
            "serviceName":"Virtual Machines","currencyCode":"USD","type":"Consumption","unitOfMeasure":"1 Hour","productName":"Virtual Machines BS Series",
            "meterName":"B2ms","retailPrice":0.25}],"NextPageLink":null}))).expect(1).mount(&server).await;
    Mock::given(method("PUT")).and(body_partial_json(json!({"properties":{"hardwareProfile":{"vmSize":"Standard_B2ms"},
        "networkProfile":{"networkInterfaces":[{"id":"/subscriptions/test/resourceGroups/rg/providers/Microsoft.Network/networkInterfaces/existing",
            "properties":{"primary":true,"deleteOption":"Detach"}}]}}})))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({"properties":{"provisioningState":"Creating"}}))).expect(1).mount(&server).await;
    let adapter = AzureAdapter::with_endpoint("test".into(),"rg".into(),"synthetic-account-token".into(),&server.uri()).unwrap()
        .with_network_interface("/subscriptions/test/resourceGroups/rg/providers/Microsoft.Network/networkInterfaces/existing".into());
    let quote = adapter.quote(&resources(), "eastus").await.unwrap();
    assert_eq!(quote.estimated_hourly_cost, Some(0.25));
    let instance = adapter
        .provision_resources(&quote.instance_type, "eastus", &resources(), false)
        .await
        .unwrap();
    assert!(instance.id.starts_with(
        "/subscriptions/test/resourceGroups/rg/providers/Microsoft.Compute/virtualMachines/"
    ));
    assert!(matches!(instance.status, InstanceStatus::Starting));
    let requests = server.received_requests().await.unwrap();
    assert!(!requests[0].headers.contains_key("authorization"));
    assert!(requests[1].headers.contains_key("authorization"));
    assert_eq!(
        requests.len(),
        2,
        "network resources must not be created before the VM id is tracked"
    );
}

#[tokio::test]
async fn gcp_is_unpriced_but_explicit_inserts_return_a_persistable_resource_name() {
    let server = MockServer::start().await;
    Mock::given(method("POST")).and(path("/compute/v1/projects/test/zones/us-central1-a/instances"))
        .and(body_partial_json(json!({"machineType":"zones/us-central1-a/machineTypes/e2-standard-2","metadata":{"items":[
            {"key":"ssh-keys","value":format!("blueprint:{}",resources().ssh_public_key.unwrap())},
            {"key":"enable-oslogin","value":"FALSE"},
            {"key":"startup-script","value":"#!/bin/sh\nset -eu\napt-get update\napt-get install -y docker.io\n"}
        ]}})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"name":"insert-operation","status":"PENDING"}))).expect(1).mount(&server).await;
    let adapter =
        GcpAdapter::with_endpoint("test".into(), "synthetic-token".into(), &server.uri()).unwrap();
    assert!(adapter.quote(&resources(), "us-central1").await.is_err());
    let instance = adapter
        .provision_resources("e2-standard-2", "us-central1", &resources(), false)
        .await
        .unwrap();
    assert!(
        instance
            .id
            .starts_with("projects/test/zones/us-central1-a/instances/")
    );
    assert!(matches!(instance.status, InstanceStatus::Starting));
}

#[cfg(feature = "aws")]
#[tokio::test]
async fn aws_signed_query_quotes_catalog_sku_with_exact_region_filters() {
    use blueprint_remote_providers::providers::aws::AwsAdapter;
    let server = MockServer::start().await;
    let document = json!({"product":{"productFamily":"Compute Instance","attributes":{"instanceType":"p5.48xlarge","regionCode":"us-east-1",
        "operatingSystem":"Linux","tenancy":"Shared","preInstalledSw":"NA","capacitystatus":"Used"}},
        "terms":{"OnDemand":{"term":{"priceDimensions":{"dimension":{"unit":"Hrs","beginRange":"0","pricePerUnit":{"USD":"12.50"}}}}}}});
    Mock::given(method("POST"))
        .and(path("/"))
        .and(header("x-amz-target", "AWSPriceListService.GetProducts"))
        .and(body_partial_json(
            json!({"ServiceCode":"AmazonEC2","Filters":[
                {"Field":"instanceType","Type":"TERM_MATCH","Value":"p5.48xlarge"},
                {"Field":"regionCode","Type":"TERM_MATCH","Value":"us-east-1"},
                {"Field":"operatingSystem","Type":"TERM_MATCH","Value":"Linux"},
                {"Field":"tenancy","Type":"TERM_MATCH","Value":"Shared"},
                {"Field":"preInstalledSw","Type":"TERM_MATCH","Value":"NA"},
                {"Field":"capacitystatus","Type":"TERM_MATCH","Value":"Used"}
            ]}),
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"PriceList":[document.to_string()]})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .credentials_provider(aws_sdk_ec2::config::Credentials::new(
            "synthetic-access",
            "synthetic-secret",
            None,
            None,
            "http-peer",
        ))
        .region(aws_config::Region::new("us-east-1"))
        .endpoint_url(server.uri())
        .load()
        .await;
    let adapter = AwsAdapter::with_config(&config).unwrap();
    let mut spec = resources();
    spec.gpu_class = Some("nvidia-h100".into());
    spec.gpu_count = Some(8);
    spec.gpu_vram_gb = Some(80);
    let quote = adapter.quote(&spec, "us-east-1").await.unwrap();
    assert_eq!(quote.instance_type, "p5.48xlarge");
    assert_eq!(quote.estimated_hourly_cost, Some(12.5));
    let requests = server.received_requests().await.unwrap();
    assert!(
        requests[0].headers["authorization"]
            .to_str()
            .unwrap()
            .starts_with("AWS4-HMAC-SHA256 Credential=synthetic-access/")
    );
}
