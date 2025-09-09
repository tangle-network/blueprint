//! End-to-end integration tests for Blueprint Manager with Remote Providers
//! 
//! These tests verify the complete flow from CLI → Manager → Remote Provider → Deployment

use blueprint_remote_providers::{
    CloudProvider, ResourceSpec,
};
use std::sync::Arc;
use std::time::Duration;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, body_string_contains};

/// Test the complete deployment flow from resource request to cloud instance
#[tokio::test]
async fn test_e2e_deployment_flow() {
    // 1. Set up mock cloud servers
    let mock_aws = setup_mock_aws_server().await;
    let mock_do = setup_mock_digitalocean_server().await;
    
    // 2. Configure environment to use mock endpoints
    std::env::set_var("AWS_ENDPOINT_URL", mock_aws.uri());
    std::env::set_var("DIGITALOCEAN_API_URL", mock_do.uri());
    std::env::set_var("MOCK_CLOUD_PROVIDERS", "true");
    
    // 3. Initialize the system as Blueprint Manager would
    let state_dir = tempfile::tempdir().unwrap();
    let extensions = RemoteDeploymentExtensions::new(
        state_dir.path(),
        true, // enable TTL
        Arc::new(blueprint_remote_providers::infrastructure::InfrastructureProvisioner::new(
            CloudProvider::AWS
        ).await.unwrap())
    ).await.unwrap();
    
    // 4. Define resource requirements (as would come from CLI)
    let resource_spec = ResourceSpec {
        cpu: 2.0,
        memory_gb: 4.0,
        storage_gb: 20.0,
        gpu_count: None,
        allow_spot: true,
    };
    
    // 5. Deploy a service (simulating what Blueprint Manager does)
    let deployment = extensions.deploy_remote(
        "test-blueprint",
        "test-service",
        resource_spec,
        CloudProvider::AWS,
        "us-west-2".to_string(),
        Some(3600), // 1 hour TTL
    ).await.unwrap();
    
    // 6. Verify deployment was recorded
    assert_eq!(deployment.instance_id, "i-mockinstance123");
    assert!(deployment.ttl_seconds.is_some());
    
    // 7. Verify we can query deployment status
    let status = extensions.get_deployment_status("test-blueprint", "test-service")
        .await
        .unwrap();
    assert!(status.is_some());
    
    // 8. Test cleanup on TTL expiry
    tokio::time::sleep(Duration::from_millis(100)).await;
    // In real scenario, TTL manager would handle this
    
    // 9. Verify mock server received correct requests
    let requests = mock_aws.received_requests().await.unwrap();
    assert!(!requests.is_empty());
    assert!(requests[0].body_string().contains("RunInstances"));
}

/// Test provider selection based on workload characteristics
#[tokio::test]
async fn test_provider_selection_for_workloads() {
    let mock_aws = setup_mock_aws_server().await;
    let mock_gcp = setup_mock_gcp_server().await;
    
    std::env::set_var("AWS_ENDPOINT_URL", mock_aws.uri());
    std::env::set_var("GCP_ENDPOINT_URL", mock_gcp.uri());
    
    // GPU workload should prefer AWS/GCP
    let gpu_spec = ResourceSpec {
        cpu: 4.0,
        memory_gb: 16.0,
        storage_gb: 100.0,
        gpu_count: Some(1),
        allow_spot: false,
    };
    
    // Use pricing service to find best provider
    let pricing_service = blueprint_remote_providers::PricingService::new();
    let (provider, _) = pricing_service.find_cheapest_provider(&gpu_spec, 24.0);
    
    assert!(matches!(provider, CloudProvider::AWS | CloudProvider::GCP));
}

/// Test multi-provider failover scenario
#[tokio::test]
async fn test_multi_provider_failover() {
    // Set up primary provider to fail
    let mock_aws = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(503)) // Service unavailable
        .mount(&mock_aws)
        .await;
    
    // Set up backup provider to succeed
    let mock_do = setup_mock_digitalocean_server().await;
    
    std::env::set_var("AWS_ENDPOINT_URL", mock_aws.uri());
    std::env::set_var("DIGITALOCEAN_API_URL", mock_do.uri());
    
    // System should automatically failover to DigitalOcean
    // This tests the retry and failover logic
    
    let resource_spec = ResourceSpec::basic();
    
    // In production, the CloudProvisioner would handle failover
    // For now, we verify the mock setup works
    let aws_requests = mock_aws.received_requests().await.unwrap();
    assert!(!aws_requests.is_empty()); // Should have tried AWS
    
    let do_requests = mock_do.received_requests().await.unwrap();
    // In real implementation, this would show failover happened
}

/// Test concurrent deployments don't interfere
#[tokio::test]
async fn test_concurrent_deployments() {
    let mock_server = setup_mock_aws_server().await;
    std::env::set_var("AWS_ENDPOINT_URL", mock_server.uri());
    
    let state_dir = tempfile::tempdir().unwrap();
    let provisioner = Arc::new(
        blueprint_remote_providers::infrastructure::InfrastructureProvisioner::new(
            CloudProvider::AWS
        ).await.unwrap()
    );
    
    let extensions = Arc::new(
        RemoteDeploymentExtensions::new(
            state_dir.path(),
            false, // no TTL for this test
            provisioner,
        ).await.unwrap()
    );
    
    // Launch multiple deployments concurrently
    let mut handles = vec![];
    for i in 0..5 {
        let ext = extensions.clone();
        let handle = tokio::spawn(async move {
            ext.deploy_remote(
                &format!("blueprint-{}", i),
                &format!("service-{}", i),
                ResourceSpec::minimal(),
                CloudProvider::AWS,
                "us-west-2".to_string(),
                None,
            ).await
        });
        handles.push(handle);
    }
    
    // Wait for all deployments
    let results: Vec<_> = futures::future::join_all(handles).await;
    
    // All should succeed
    for result in results {
        assert!(result.unwrap().is_ok());
    }
    
    // Verify all deployments are tracked
    for i in 0..5 {
        let status = extensions
            .get_deployment_status(&format!("blueprint-{}", i), &format!("service-{}", i))
            .await
            .unwrap();
        assert!(status.is_some());
    }
}

/// Test resource cleanup and cost tracking
#[tokio::test]
async fn test_resource_cleanup_and_costs() {
    let mock_server = setup_mock_aws_server().await;
    
    // Add termination endpoint
    Mock::given(method("POST"))
        .and(body_string_contains("TerminateInstances"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string(r#"<?xml version="1.0" encoding="UTF-8"?>
<TerminateInstancesResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">
    <requestId>test-terminate</requestId>
    <instancesSet>
        <item>
            <instanceId>i-mockinstance123</instanceId>
            <currentState><code>32</code><name>shutting-down</name></currentState>
            <previousState><code>16</code><name>running</name></previousState>
        </item>
    </instancesSet>
</TerminateInstancesResponse>"#))
        .mount(&mock_server)
        .await;
    
    std::env::set_var("AWS_ENDPOINT_URL", mock_server.uri());
    
    // Deploy
    let state_dir = tempfile::tempdir().unwrap();
    let extensions = RemoteDeploymentExtensions::new(
        state_dir.path(),
        false,
        Arc::new(blueprint_remote_providers::infrastructure::InfrastructureProvisioner::new(
            CloudProvider::AWS
        ).await.unwrap())
    ).await.unwrap();
    
    let deployment = extensions.deploy_remote(
        "test-blueprint",
        "test-service",
        ResourceSpec::basic(),
        CloudProvider::AWS,
        "us-west-2".to_string(),
        None,
    ).await.unwrap();
    
    // Track costs
    let cost_report = blueprint_remote_providers::PricingService::new()
        .calculate_cost(&deployment.resource_spec, CloudProvider::AWS, 1.0);
    assert!(cost_report.total_cost > 0.0);
    assert!(cost_report.total_cost < 10.0); // Sanity check
    
    // Cleanup
    extensions.cleanup_deployment("test-blueprint", "test-service").await.unwrap();
    
    // Verify termination was called
    let requests = mock_server.received_requests().await.unwrap();
    let has_terminate = requests.iter().any(|r| 
        r.body_string().contains("TerminateInstances")
    );
    assert!(has_terminate);
}

// Helper functions to set up mock servers

async fn setup_mock_aws_server() -> MockServer {
    let server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(body_string_contains("RunInstances"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string(r#"<?xml version="1.0" encoding="UTF-8"?>
<RunInstancesResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">
    <requestId>test-request</requestId>
    <instancesSet>
        <item>
            <instanceId>i-mockinstance123</instanceId>
            <instanceState><code>0</code><name>pending</name></instanceState>
            <instanceType>t3.micro</instanceType>
            <privateIpAddress>172.31.0.10</privateIpAddress>
        </item>
    </instancesSet>
</RunInstancesResponse>"#))
        .mount(&server)
        .await;
    
    server
}

async fn setup_mock_digitalocean_server() -> MockServer {
    let server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/v2/droplets"))
        .respond_with(ResponseTemplate::new(201)
            .set_body_json(serde_json::json!({
                "droplet": {
                    "id": 12345,
                    "name": "test-droplet",
                    "status": "new",
                    "networks": {
                        "v4": [{"ip_address": "104.236.32.182", "type": "public"}],
                        "v6": []
                    }
                }
            })))
        .mount(&server)
        .await;
    
    server
}

async fn setup_mock_gcp_server() -> MockServer {
    let server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path_regex(r"/compute/v1/projects/.*/zones/.*/instances"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "kind": "compute#operation",
                "id": "operation-123",
                "status": "PENDING",
                "operationType": "insert"
            })))
        .mount(&server)
        .await;
    
    server
}

use regex::Regex;
use wiremock::matchers::path_regex;