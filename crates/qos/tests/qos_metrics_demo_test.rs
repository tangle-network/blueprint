use std::fs;
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, Instant};

use blueprint_core::{Job, info, warn};
use blueprint_qos::heartbeat::HeartbeatConfig;
use blueprint_qos::{PrometheusServerConfig, QoSService, default_qos_config};
use blueprint_tangle_extra::layers::TangleLayer;
use blueprint_testing_utils::tangle::runner::MockHeartbeatConsumer;
use blueprint_testing_utils::{
    Error, setup_log,
    tangle::multi_node::NodeSlot,
    tangle::{
        InputValue, OutputValue, TangleTestHarness, blueprint::create_test_blueprint,
        harness::SetupServicesOpts,
    },
};
use prometheus::Registry;
use prometheus::{IntGauge, Opts};
use tokio::time::sleep;

mod utils;

// Port constants for the metrics servers
const METRICS_PORT: u16 = 8085;
const GRAFANA_PORT: u16 = 3000;
const PROMETHEUS_PORT: u16 = 9091;
const LOKI_PORT: u16 = 3100;
const OPERATOR_COUNT: usize = 1; // Number of operators for the test
const INPUT_VALUE: u64 = 5; // Value to square in our test job
const TOTAL_JOBS_TO_RUN: u64 = 50; // Number of jobs to run before test completes
const JOB_INTERVAL_MS: u64 = 2000; // Time between job submissions in milliseconds

/// Utility function to clean up any existing Docker containers to avoid conflicts
fn cleanup_docker_containers(_harness: &TangleTestHarness<()>) -> Result<(), Error> {
    // Check existing containers first (for debugging)
    info!("Current Docker containers before cleanup:");
    let list_output = Command::new("docker")
        .args(["ps", "-a"])
        .output()
        .map_err(|e| Error::Setup(format!("Failed to run docker ps command: {}", e)))?;

    if list_output.status.success() {
        let container_list = String::from_utf8_lossy(&list_output.stdout);
        info!("Docker containers: {}", container_list);
    }

    // Docker container names used in this test
    let containers = [
        "blueprint-test-loki",
        "blueprint-test-grafana",
        "blueprint-test-prometheus",
        "blueprint-loki",
        "blueprint-grafana",
        "blueprint-prometheus",
        "loki",
        "grafana",
        "prometheus",
        "qos-test",
    ];

    for container_name in &containers {
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

/// Demonstrates the complete QoS metrics system with Grafana, Loki, and Prometheus.
#[tokio::test]
#[ignore] // Ignore by default since this is a long-running demo test - run with: cargo test test_qos_metrics_demo -- --ignored
async fn test_qos_metrics_demo() -> Result<(), Error> {
    setup_log();
    info!(
        "Starting QoS metrics demonstration with {} job runs",
        TOTAL_JOBS_TO_RUN
    );

    // Create a test blueprint with QoS integration
    info!("Creating test blueprint with QoS integration");
    let (temp_dir, blueprint_dir) = create_test_blueprint();

    let harness: TangleTestHarness<()> = TangleTestHarness::setup(temp_dir).await?;
    std::env::set_current_dir(&blueprint_dir).unwrap();
    cleanup_docker_containers(&harness)?;

    info!("Setting up test service with {} operators", OPERATOR_COUNT);
    let setup_services_opts = SetupServicesOpts {
        exit_after_registration: false,
        ..Default::default()
    };
    let (mut test_env, service_id, blueprint_id) = harness
        .setup_services_with_options::<OPERATOR_COUNT>(setup_services_opts)
        .await?;
    info!("Test environment initialized, submitting jobs to generate metrics...");

    // Verify that the blueprint includes QoS imports
    let main_rs_content = fs::read_to_string(blueprint_dir.join("src/main.rs"))
        .map_err(|e| Error::Setup(format!("Failed to read main.rs: {}", e)))?;

    assert!(
        main_rs_content.contains("blueprint_qos"),
        "Blueprint should include QoS imports"
    );
    info!("Blueprint includes QoS integration");

    // Initialize test environment
    info!("Initializing test environment");
    test_env.initialize().await?;

    // Get node handle for operator
    let operator_index = 0;
    info!("Using operator index {} for testing", operator_index);

    let node_handle = {
        let nodes = test_env.nodes.read().await;
        match &nodes[operator_index] {
            NodeSlot::Occupied(node) => node.clone(),
            NodeSlot::Empty => {
                return Err(Error::Setup(format!(
                    "Node {} is not initialized",
                    operator_index
                )));
            }
        }
    };

    info!("Adding square job to node handle");
    node_handle.add_job(utils::square.layer(TangleLayer)).await;

    let mut qos_config = default_qos_config();

    qos_config.heartbeat = Some(HeartbeatConfig {
        interval_secs: 5,
        jitter_percent: 5,
        service_id,
        blueprint_id: 1,
        max_missed_heartbeats: 3,
    });
    qos_config.prometheus_server = Some(PrometheusServerConfig {
        host: "0.0.0.0".to_string(),
        port: 9091,
        use_docker: false,
        ..Default::default()
    });
    qos_config.manage_servers = true;
    qos_config.loki = Some(blueprint_qos::logging::loki::LokiConfig {
        url: "http://localhost:3100".to_string(),
        username: Some("test-tenant".to_string()),
        ..Default::default()
    });
    qos_config.grafana = Some(blueprint_qos::logging::grafana::GrafanaConfig {
        url: "http://localhost:3000".to_string(),
        api_key: "test-api-key".to_string(),
        folder: Some("TestDashboards".to_string()),
        ..Default::default()
    });
    let qos_service = QoSService::new(qos_config.clone(), Arc::new(MockHeartbeatConsumer::new()))
        .await
        .unwrap();
    node_handle.set_qos_service(qos_service).await;

    // Start the BlueprintRunner
    info!("Starting BlueprintRunner with node handle");
    node_handle
        .start_runner(())
        .await
        .map_err(|e| Error::Setup(format!("Failed to start runner: {}", e)))?;

    info!(
        "BlueprintRunner started successfully - QoS service and heartbeat service should be running internally"
    );

    info!("Grafana dashboard available at: http://127.0.0.1:3000");
    info!("Login credentials: admin/admin (if required)");

    // Setup registry for job metrics
    let registry = Registry::new();

    // Helper function to create and register a gauge with labels using the actual service_id and blueprint_id
    let create_gauge = |name: &str, help: &str| {
        let opts = Opts::new(name, help)
            .const_label("service_id", service_id.to_string())
            .const_label("blueprint_id", blueprint_id.to_string());

        match IntGauge::with_opts(opts) {
            Ok(gauge) => {
                if let Err(e) = registry.register(Box::new(gauge.clone())) {
                    warn!("Failed to register gauge {}: {}", name, e);
                }
                gauge
            }
            Err(e) => {
                panic!("Failed to create gauge {}: {}", name, e);
            }
        }
    };

    // Create job execution metrics
    info!("Setting up job metrics");
    let job_executions = create_gauge(
        "test_blueprint_job_executions",
        "Job executions for test blueprint",
    );

    let job_success = create_gauge("test_blueprint_job_success", "Successful job executions");

    let job_latency = create_gauge(
        "test_blueprint_job_latency_ms",
        "Job execution latency in milliseconds",
    );

    // Starting the continuous job execution loop
    info!(
        "Starting continuous job execution loop ({} jobs)",
        TOTAL_JOBS_TO_RUN
    );
    info!(
        "Access Grafana dashboard at: http://127.0.0.1:{}",
        GRAFANA_PORT
    );

    // Run jobs in a loop to generate continuous metrics
    let mut jobs_completed = 0;
    let start_time = Instant::now();

    while jobs_completed < TOTAL_JOBS_TO_RUN {
        let job_start = Instant::now();

        // Submit a job to square a number using the actual service_id from setup
        info!(
            "Submitting job #{} to square {}",
            jobs_completed + 1,
            INPUT_VALUE
        );
        let call = harness
            .submit_job(
                service_id,
                utils::XSQUARE_JOB_ID,
                vec![InputValue::Uint64(INPUT_VALUE)],
            )
            .await
            .map_err(|e| Error::Setup(format!("Failed to submit job: {}", e)))?;

        // Wait for job execution completion
        let result = harness
            .wait_for_job_execution(service_id, call)
            .await
            .map_err(|e| Error::Setup(format!("Failed to wait for job execution: {}", e)))?;

        // Verify job result
        harness.verify_job(
            &result,
            vec![OutputValue::Uint64(INPUT_VALUE * INPUT_VALUE)],
        );

        // Record metrics
        jobs_completed += 1;
        job_executions.set(jobs_completed as i64);
        job_success.set(jobs_completed as i64);
        let latency = job_start.elapsed().as_millis() as i64;
        job_latency.set(latency);

        info!(
            "Job #{} completed: {} squared = {} (took {} ms)",
            jobs_completed,
            INPUT_VALUE,
            INPUT_VALUE * INPUT_VALUE,
            latency
        );

        // Wait between job submissions
        sleep(Duration::from_millis(JOB_INTERVAL_MS)).await;
    }

    // Show final statistics
    let total_time = start_time.elapsed();
    info!("QoS metrics demonstration completed");
    info!("Jobs completed: {}", jobs_completed);
    info!("Total time: {:.2} seconds", total_time.as_secs_f64());
    info!(
        "Average job time: {:.2} ms",
        total_time.as_millis() as f64 / jobs_completed as f64
    );

    // Keep server running for a few seconds to allow viewing metrics
    info!("Servers will remain running for 10 more seconds for metrics viewing");
    sleep(Duration::from_secs(10)).await;

    Ok(())
}
