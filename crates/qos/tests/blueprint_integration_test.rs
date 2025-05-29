use std::{fs, process::Command, sync::Arc, time::Duration};

use blueprint_core::{info, Job};
use blueprint_qos::{
    default_qos_config,
    proto::{GetStatusRequest, GetResourceUsageRequest, GetBlueprintMetricsRequest},
    QoSServiceBuilder,
};
use blueprint_tangle_extra::layers::TangleLayer;
use blueprint_testing_utils::{
    setup_log, Error,
    tangle::{InputValue, OutputValue, TangleTestHarness, blueprint::create_test_blueprint, harness::SetupServicesOpts},
};
use tokio::time::sleep;

mod utils;

// Constants for test timeouts and configuration
const TEST_TIMEOUT_SECS: u64 = 60; // Total test timeout
const HEARTBEAT_TIMEOUT_SECS: u64 = 10; // Timeout waiting for heartbeats
const HEARTBEAT_INTERVAL_SECS: u64 = 2; // Test interval for heartbeats
const MIN_HEARTBEATS: usize = 2; // Minimum expected heartbeats
const OPERATOR_COUNT: usize = 1; // Number of operators for the test
const INPUT_VALUE: u64 = 5; // Value to square in our test job
const QOS_PORT: u16 = 8085; // Port for QoS gRPC service

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

    let harness: TangleTestHarness<utils::TestContext> = TangleTestHarness::setup(temp_dir).await?;

    std::env::set_current_dir(&blueprint_dir).unwrap();

    cleanup_docker_containers()?;

    // Create the heartbeat consumer and test context
    let heartbeat_consumer = Arc::new(utils::MockHeartbeatConsumer::new());
    
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

    // Set up QoS configuration with proper settings
    let mut qos_config = default_qos_config();
    if let Some(heartbeat_config) = &mut qos_config.heartbeat {
        heartbeat_config.interval_secs = HEARTBEAT_INTERVAL_SECS;
        heartbeat_config.service_id = service_id;
        heartbeat_config.blueprint_id = blueprint_id;
    }
    
    if let Some(metrics_config) = &mut qos_config.metrics {
        metrics_config.bind_address = format!("0.0.0.0:{}", QOS_PORT);
        metrics_config.collection_interval_secs = 1;
        metrics_config.service_id = service_id;
        metrics_config.blueprint_id = blueprint_id;
    }
    
    if let Some(prometheus_config) = &mut qos_config.prometheus_server {
        prometheus_config.port = QOS_PORT + 1;
        prometheus_config.use_docker = false; // Use embedded server for testing
    }
    
    // Create the QoSService
    let qos_service = QoSServiceBuilder::new()
        .with_config(qos_config.clone())
        .with_heartbeat_consumer(heartbeat_consumer.clone())
        .build()
        .await
        .map_err(|e| Error::Setup(format!("Failed to build QoS service: {}", e)))?;
    
    let test_ctx = utils::TestContext::new(heartbeat_consumer.clone(), qos_config.clone());

    // Start the test environment and add the square job
    info!("Starting test environment");
    test_env.initialize().await?;
    let layered_job = utils::square.layer(TangleLayer {});
    test_env.add_job(layered_job).await;
    test_env.start(test_ctx).await?;
    
    // Start the QoS service - no explicit start method, service starts when built
    info!("QoS service initialized");
    
    // Submit a job and verify it works
    info!("Submitting test job to square {}", INPUT_VALUE);
    let job = harness
        .submit_job(service_id, utils::XSQUARE_JOB_ID, vec![InputValue::Uint64(INPUT_VALUE)])
        .await
        .map_err(|e| Error::Setup(format!("Failed to submit job: {}", e)))?;
    
    // Wait for job execution and verify results
    let results = harness
        .wait_for_job_execution(service_id, job)
        .await
        .map_err(|e| Error::Setup(format!("Failed to wait for job execution: {}", e)))?;
    
    harness.verify_job(&results, vec![OutputValue::Uint64(INPUT_VALUE * INPUT_VALUE)]);
    info!("Job executed successfully: {} squared = {}", INPUT_VALUE, INPUT_VALUE * INPUT_VALUE);
    
    // Wait for heartbeats to be processed
    info!("Waiting for heartbeats to be processed");
    sleep(Duration::from_secs(HEARTBEAT_TIMEOUT_SECS)).await;

    // Test that heartbeats are being generated
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
    
    // Test the QoS gRPC API
    info!("Testing QoS metrics gRPC API");
    let qos_addr = format!("127.0.0.1:{}", QOS_PORT);
    let mut client = utils::connect_to_qos_metrics(&qos_addr).await
        .map_err(|e| Error::Setup(format!("Failed to connect to QoS metrics service: {}", e)))?;
    
    // Test GetStatus API
    let status_response = client.get_status(GetStatusRequest {
        service_id: service_id,
        blueprint_id: blueprint_id,
    })
        .await
        .map_err(|e| Error::Setup(format!("Failed to get status: {}", e)))?;
    
    let status = status_response.into_inner();
    info!("QoS status: service_id={}, blueprint_id={}, status_code={}", 
          status.service_id, status.blueprint_id, status.status_code);
    
    assert_eq!(status.service_id, service_id, "Service ID mismatch");
    assert_eq!(status.blueprint_id, blueprint_id, "Blueprint ID mismatch");
    
    // Test GetResourceUsage API
    let resource_response = client.get_resource_usage(GetResourceUsageRequest {
        service_id: service_id,
        blueprint_id: blueprint_id,
    })
        .await
        .map_err(|e| Error::Setup(format!("Failed to get resource usage: {}", e)))?;
    
    let resources = resource_response.into_inner();
    info!("QoS resources: CPU usage={}%, Memory usage={} bytes", 
          resources.cpu_usage, resources.memory_usage);
    
    // Test GetBlueprintMetrics API
    let metrics_response = client.get_blueprint_metrics(GetBlueprintMetricsRequest {
        service_id: service_id,
        blueprint_id: blueprint_id,
    })
        .await
        .map_err(|e| Error::Setup(format!("Failed to get blueprint metrics: {}", e)))?;
    
    let metrics = metrics_response.into_inner();
    info!("QoS blueprint metrics: timestamp={}, custom metrics count={}", 
          metrics.timestamp, metrics.custom_metrics.len());
    
    info!("✅ QoS Blueprint integration test completed successfully");
    Ok(())
}
