//! Integration tests for DigitalOcean Functions
//!
//! These tests verify the DigitalOcean executor behavior through public API.

#![cfg(feature = "digitalocean")]

use blueprint_faas::digitalocean::DigitalOceanExecutor;
use blueprint_faas::FaasExecutor;

#[tokio::test]
#[ignore = "Requires DigitalOcean API token and creates real resources"]
async fn test_end_to_end_deployment() {
    let token = match std::env::var("DIGITALOCEAN_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            println!("Skipping E2E test: DIGITALOCEAN_TOKEN not set");
            return;
        }
    };

    let executor = DigitalOceanExecutor::new(token, "nyc1")
        .await
        .expect("Failed to create executor");

    // Create a simple test binary
    let test_binary = b"#!/bin/sh\necho 'test'";

    let config = blueprint_faas::FaasConfig {
        memory_mb: 512,
        timeout_secs: 60,
        ..Default::default()
    };

    // Deploy
    let deployment = executor
        .deploy_job(999, test_binary, &config)
        .await
        .expect("Failed to deploy");

    assert_eq!(deployment.job_id, 999);
    assert!(!deployment.endpoint.is_empty());

    // Health check
    let healthy = executor.health_check(999).await.expect("Health check failed");
    assert!(healthy);

    // Clean up
    executor
        .undeploy_job(999)
        .await
        .expect("Failed to undeploy");
}
