use std::fs;
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, Instant};

use blueprint_core::{Job, info, warn};
use blueprint_qos::{
    QoSServiceBuilder, default_qos_config,
    metrics::opentelemetry::OpenTelemetryConfig,
    metrics::types::MetricsConfig,
    metrics::{EnhancedMetricsProvider, service},
    servers,
};
use blueprint_tangle_extra::layers::TangleLayer;
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

/// Mock implementation of HeartbeatConsumer for testing
#[derive(Debug, Clone)]
struct MockHeartbeatConsumer {}

impl MockHeartbeatConsumer {
    pub fn new() -> Self {
        Self {}
    }
}

// Implement the HeartbeatConsumer trait
impl blueprint_qos::heartbeat::HeartbeatConsumer for MockHeartbeatConsumer {
    fn send_heartbeat(
        &self,
        _status: &blueprint_qos::heartbeat::HeartbeatStatus,
    ) -> impl std::future::Future<Output = Result<(), blueprint_qos::error::Error>> + Send {
        async { Ok(()) }
    }
}

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
        "blueprint-loki", // also try default names
        "blueprint-grafana",
        "blueprint-prometheus",
        "loki",
        "grafana",
        "prometheus",
        "qos-test",
    ];

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

/// Demonstrates the complete QoS metrics system with Grafana, Loki, and Prometheus.
///
/// Features:
/// - Metrics collection via Prometheus
/// - Dashboard visualization in Grafana
/// - Log collection in Loki
/// - Continuous job execution to generate metrics
/// - Automatic dashboard setup
///
/// This test runs until TOTAL_JOBS_TO_RUN have been completed.
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

    // Clean up existing containers
    cleanup_docker_containers(&harness)?;

    // Create a QoS configuration with integrated server management
    let mut qos_config = default_qos_config();

    // Configure server containers for metrics visualization
    let grafana_server_config = servers::grafana::GrafanaServerConfig {
        port: GRAFANA_PORT,
        container_name: "blueprint-test-grafana".to_string(),
        admin_user: "admin".to_string(),
        admin_password: "admin".to_string(),
        allow_anonymous: true,
        ..Default::default()
    };

    let prometheus_server_config = servers::prometheus::PrometheusServerConfig {
        port: PROMETHEUS_PORT,
        docker_container_name: "blueprint-test-prometheus".to_string(),
        ..Default::default()
    };

    let loki_server_config = servers::loki::LokiServerConfig {
        port: LOKI_PORT,
        container_name: "blueprint-test-loki".to_string(),
        ..Default::default()
    };

    // Enable server management and set configurations
    qos_config.manage_servers = true;
    qos_config.grafana_server = Some(grafana_server_config);
    qos_config.prometheus_server = Some(prometheus_server_config);
    qos_config.loki_server = Some(loki_server_config);

    info!(
        "QoS metrics visualization configured with Grafana on port {}",
        GRAFANA_PORT
    );

    // Set up test service with operators
    info!("Setting up test service with {} operators", OPERATOR_COUNT);
    let setup_services_opts = SetupServicesOpts {
        exit_after_registration: false,
        skip_service_request: false,
        registration_args: vec![Vec::default(); OPERATOR_COUNT].try_into().unwrap(),
        request_args: Vec::default(),
    };

    let (mut test_env, service_id, blueprint_id) = harness
        .setup_services_with_options::<OPERATOR_COUNT>(setup_services_opts)
        .await?;

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

    // Add square job to node handle
    info!("Adding square job to node handle");
    node_handle.add_job(utils::square.layer(TangleLayer)).await;

    // Set up metrics server on dedicated port
    let metrics_addr = format!("127.0.0.1:{}", METRICS_PORT);

    // Start the BlueprintRunner
    info!("Starting BlueprintRunner with node handle");
    node_handle
        .start_runner(())
        .await
        .map_err(|e| Error::Setup(format!("Failed to start runner: {}", e)))?;

    info!(
        "BlueprintRunner started successfully - QoS service and heartbeat service should be running internally"
    );

    // Start a metrics server to expose metrics for viewing
    info!("Starting metrics server on {}", metrics_addr);
    let metrics_addr_clone = metrics_addr.clone();

    // Copy the service_id and blueprint_id for use in the async block
    let metrics_service_id = service_id;
    let metrics_blueprint_id = blueprint_id;

    let _metrics_server_handle = tokio::spawn(async move {
        let metrics_config = MetricsConfig {
            service_id: metrics_service_id,
            blueprint_id: metrics_blueprint_id,
            bind_address: metrics_addr_clone.clone(),
            ..Default::default()
        };

        // Create metrics provider - just for verification
        if let Err(e) =
            EnhancedMetricsProvider::new(metrics_config.clone(), OpenTelemetryConfig::default())
        {
            panic!("Failed to create metrics provider: {}", e);
        }

        info!("Metrics server running at http://{}", metrics_addr_clone);
        // Start metrics server with the config
        if let Err(e) = service::run_metrics_server(metrics_config).await {
            panic!("Metrics server error: {}", e);
        }
    });

    // Create a heartbeat consumer for QoS service
    let heartbeat_consumer = Arc::new(MockHeartbeatConsumer::new());

    // Build the QoS service with server management
    info!("Starting QoS service with server management");
    let qos_service_result = QoSServiceBuilder::new()
        .with_config(qos_config.clone())
        .with_heartbeat_consumer(heartbeat_consumer)
        .manage_servers(true)
        .build()
        .await;

    match qos_service_result {
        Ok(mut qos_service) => {
            // Wait for servers to initialize
            info!("Waiting for metrics servers to initialize...");
            sleep(Duration::from_secs(5)).await;

            // Create the dashboard
            info!("Creating Grafana dashboard for blueprint metrics");
            match qos_service.create_dashboard("prometheus", "loki").await {
                Ok(dashboard_url) => {
                    info!("Dashboard created successfully");
                    info!("Dashboard URL: {:?}", dashboard_url);
                }
                Err(e) => {
                    warn!("Failed to create dashboard: {}", e);
                }
            }

            // Get Grafana URL - this is the only URL we show to the user
            if let Some(url) = qos_service.grafana_server_url() {
                // Only display the Grafana URL as requested
                info!("Grafana dashboard available at: {}", url);
                info!("Login credentials: admin/admin (if required)");
            } else {
                // Fallback to direct URL if server URL isn't available
                info!(
                    "Grafana dashboard available at: http://127.0.0.1:{}",
                    GRAFANA_PORT
                );
                info!("Login credentials: admin/admin (if required)");
            }
        }
        Err(e) => {
            warn!("Failed to start QoS service with server management: {}", e);
            warn!("Falling back to using direct Grafana URL");

            // Provide direct access URL in case of failure
            info!(
                "Grafana dashboard available at: http://127.0.0.1:{}",
                GRAFANA_PORT
            );
            info!("Login credentials: admin/admin (if required)");
        }
    }

    // Setup registry for job metrics
    let registry = Registry::new();
    // Use the actual service_id and blueprint_id from the test setup

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
