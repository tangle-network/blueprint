#![allow(dead_code, unused_imports)]

use blueprint_qos::{
    GrafanaConfig, GrafanaServerConfig, LokiConfig, QoSConfig, QoSServiceBuilder,
    default_qos_config,
};
use chrono::Utc;
use log::{info, warn};
use reqwest;
use serial_test::serial;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::sleep;

// Correctly declare the utils module at the top level
#[path = "utils.rs"]
mod utils;

use utils::MockHeartbeatConsumer;

// Base port for servers to avoid conflicts in parallel tests
const BASE_PORT: u16 = 4500;

/// Sets up logging for the test.
fn setup_log() {
    // Attemps to init the logger and ignores the error if it's already initialized
    let _ = env_logger::builder().is_test(true).try_init();
}

/// This test verifies that the `QoSService` correctly sends heartbeats
/// when configured with a `HeartbeatConsumer`.
#[tokio::test]
#[serial]
async fn test_qos_service_functionality() {
    setup_log();
    info!("Starting test: test_qos_service_functionality");

    let consumer = std::sync::Arc::new(MockHeartbeatConsumer::new());
    let mut qos_config = QoSConfig::default();

    // Disable server management for this test to isolate heartbeat functionality
    qos_config.grafana_server = None;
    qos_config.loki_server = None;
    qos_config.heartbeat = Some(Default::default());

    let _qos_service = QoSServiceBuilder::new()
        .with_config(qos_config)
        .with_heartbeat_consumer(consumer.clone())
        .build()
        .await
        .expect("QoS service should build successfully");

    // Allow some time for the heartbeat service to send a few heartbeats.
    // Heartbeats are sent every 5 seconds by default.
    info!("Waiting for 15 seconds to receive heartbeats...");
    sleep(Duration::from_secs(15)).await;

    let count = consumer.heartbeat_count();
    info!("Received {} heartbeats.", count);
    assert!(
        count > 0,
        "Should have received at least one heartbeat after 15 seconds"
    );

    info!("Test test_qos_service_functionality finished successfully.");
}

/// This test verifies that the `QoSService` can correctly start and manage
/// Docker containers for Grafana and Loki.
#[tokio::test]
#[serial]
async fn test_grafana_loki_server_management() {
    setup_log();
    info!("Starting test: test_grafana_loki_server_management");

    // Create a temporary directory for Docker volumes
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path_str = temp_dir.path().to_str().unwrap().to_string();
    let consumer = std::sync::Arc::new(MockHeartbeatConsumer::new());

    let mut qos_config = QoSConfig::default();
    qos_config.grafana_server = Some(GrafanaServerConfig {
        port: BASE_PORT,
        container_name: "test-grafana-integration".to_string(),
        data_dir: temp_path_str.clone(),
        ..Default::default()
    });
    qos_config.loki_server = Some(blueprint_qos::LokiServerConfig {
        port: BASE_PORT + 1,
        container_name: "test-loki-integration".to_string(),
        data_dir: temp_path_str,
        ..Default::default()
    });

    info!("Building QoS service to start Grafana and Loki containers...");
    let qos_service = QoSServiceBuilder::new()
        .with_config(qos_config)
        .with_heartbeat_consumer(consumer)
        .build()
        .await
        .expect("QoS service should build and start servers");

    // Wait for a generous amount of time for Docker containers to download and start.
    info!("Waiting 90 seconds for Grafana and Loki to start...");
    sleep(Duration::from_secs(90)).await;

    // Verify servers are up by checking their health/ready endpoints.
    let grafana_health_url = format!("http://localhost:{}/api/health", BASE_PORT);
    let loki_ready_url = format!("http://localhost:{}/ready", BASE_PORT + 1);

    info!("Checking Grafana health at: {}", grafana_health_url);
    let grafana_resp = reqwest::get(&grafana_health_url).await;
    assert!(
        grafana_resp.is_ok() && grafana_resp.unwrap().status().is_success(),
        "Grafana health check should succeed"
    );
    info!("Grafana is up!");

    info!("Checking Loki readiness at: {}", loki_ready_url);
    let loki_resp = reqwest::get(&loki_ready_url).await;
    assert!(
        loki_resp.is_ok() && loki_resp.unwrap().status().is_success(),
        "Loki readiness check should succeed"
    );
    info!("Loki is up!");

    // Dropping the service should trigger the shutdown of the containers.
    info!("Shutting down QoS service and associated containers...");
    drop(qos_service);
    sleep(Duration::from_secs(10)).await; // Give time for shutdown commands to execute.

    // Verify servers are down
    info!("Verifying Grafana is down...");
    let grafana_resp_after_shutdown = reqwest::get(&grafana_health_url).await;
    assert!(
        grafana_resp_after_shutdown.is_err(),
        "Grafana should be down after shutdown"
    );

    info!("Verifying Loki is down...");
    let loki_resp_after_shutdown = reqwest::get(&loki_ready_url).await;
    assert!(
        loki_resp_after_shutdown.is_err(),
        "Loki should be down after shutdown"
    );

    info!("Test test_grafana_loki_server_management finished successfully.");
}
