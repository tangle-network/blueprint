use std::fs;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

use blueprint_core::info;
use blueprint_qos::default_qos_config;
use blueprint_testing_utils::Error;
use blueprint_testing_utils::setup_log;
use blueprint_testing_utils::tangle::blueprint::create_test_blueprint;
use blueprint_testing_utils::tangle::harness::SetupServicesOpts;
use blueprint_testing_utils::tangle::{InputValue, OutputValue, TangleTestHarness};
use tempfile::tempdir;
use tokio::time::sleep;

mod utils;

// Constants for test timeouts and configuration
const TEST_TIMEOUT_SECS: u64 = 60; // Total test timeout
const HEARTBEAT_TIMEOUT_SECS: u64 = 10; // Timeout waiting for heartbeats
const HEARTBEAT_INTERVAL_SECS: u64 = 2; // Test interval for heartbeats
const MIN_HEARTBEATS: usize = 2; // Minimum expected heartbeats
const OPERATOR_COUNT: usize = 1; // Number of operators for the test
const INPUT_VALUE: u64 = 5; // Value to square in our test job

/// Utility function to clean up any existing Docker containers to avoid conflicts
fn cleanup_docker_containers() -> Result<(), Error> {
    let containers = ["loki", "grafana", "prometheus", "qos-test"];
    for container_name in &containers {
        info!("Cleaning up container: {}", container_name);
        let output = Command::new("docker")
            .args(["rm", "-f", container_name])
            .output()
            .map_err(|e| Error::Setup(format!("Failed to run docker command: {}", e)))?;

        if !output.status.success() {
            info!("Container {} might not exist, continuing", container_name);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_qos_integration() -> Result<(), Error> {
    setup_log();
    info!("Starting QoS Blueprint integration test");

    info!("Creating test blueprint with QoS integration");
    let (temp_dir, blueprint_dir) = create_test_blueprint();

    let harness: TangleTestHarness<()> = TangleTestHarness::setup(temp_dir).await?;

    std::env::set_current_dir(&blueprint_dir).unwrap();

    cleanup_docker_containers()?;

    let heartbeat_consumer = Arc::new(utils::MockHeartbeatConsumer::new());
    let test_ctx = utils::TestContext {
        heartbeat_consumer: heartbeat_consumer.clone(),
    };

    info!("Setting up test service with {} operators", OPERATOR_COUNT);
    let setup_services_opts = SetupServicesOpts {
        exit_after_registration: false,
        skip_service_request: false,
        registration_args: vec![Default::default(); OPERATOR_COUNT].try_into().unwrap(),
        request_args: Default::default(),
    };

    let (mut test_env, service_id, blueprint_id) = harness
        .setup_services_with_options::<OPERATOR_COUNT>(setup_services_opts)
        .await?;

    let main_rs_content = fs::read_to_string(blueprint_dir.join("src/main.rs"))
        .map_err(|e| Error::Setup(format!("Failed to read main.rs: {}", e)))?;

    assert!(
        main_rs_content.contains("blueprint_qos"),
        "Blueprint should include QoS imports"
    );
    info!("Blueprint includes QoS integration");

    let mut qos_config = default_qos_config();
    if let Some(heartbeat_config) = &mut qos_config.heartbeat {
        heartbeat_config.interval_secs = HEARTBEAT_INTERVAL_SECS;
        heartbeat_config.service_id = service_id;
        heartbeat_config.blueprint_id = blueprint_id;
    }

    // Start the test environment
    info!("Starting test environment");
    test_env.initialize().await?;

    // TODO: Submit and verify job

    info!("Waiting for heartbeats to be processed");
    sleep(Duration::from_secs(HEARTBEAT_TIMEOUT_SECS)).await;

    let heartbeat_count = heartbeat_consumer.heartbeat_count();
    info!(
        "Received {} heartbeats (minimum expected: {})",
        heartbeat_count, MIN_HEARTBEATS
    );
    assert!(
        heartbeat_count >= MIN_HEARTBEATS,
        "Expected at least {} heartbeats, got {}",
        MIN_HEARTBEATS,
        heartbeat_count
    );

    if heartbeat_count > 0 {
        info!(
            "Last heartbeat status: {:?}",
            heartbeat_consumer.get_heartbeats().last()
        );
    }

    info!("✅ QoS Blueprint integration test completed successfully");
    Ok(())
}
