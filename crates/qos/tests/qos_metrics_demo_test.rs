use opentelemetry::KeyValue;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::process::Command;

use blueprint_core::{Job, error, info, warn};
use blueprint_qos::heartbeat::HeartbeatConfig;
use blueprint_qos::servers::common::DockerManager;
use blueprint_tangle_extra::layers::TangleLayer;

const TEST_GRAFANA_CONTAINER_NAME: &str = "blueprint-grafana";
const TEST_LOKI_CONTAINER_NAME: &str = "blueprint-loki";
const TEST_PROMETHEUS_CONTAINER_NAME: &str = "blueprint-test-prometheus";
// const PROMETHEUS_PORT: u16 = 9090;

use blueprint_qos::{GrafanaServerConfig, PrometheusServerConfig, QoSService, default_qos_config};
use blueprint_testing_utils::tangle::harness::TangleTestHarness;

use blueprint_core::debug;
use blueprint_qos::logging::grafana::CreateDataSourceRequest;
use blueprint_testing_utils::Error as TestRunnerError;
use blueprint_testing_utils::tangle::multi_node::NodeSlot;
use blueprint_testing_utils::tangle::runner::MockHeartbeatConsumer;
use blueprint_testing_utils::{
    setup_log,
    tangle::{
        InputValue, OutputValue, blueprint::create_test_blueprint, harness::SetupServicesOpts,
    },
};
use prometheus::Registry;
use prometheus::{IntGauge, Opts};
use serde_json::json;
use tokio::time::sleep;

mod utils;

// Metrics Constants
const GRAFANA_PORT: u16 = 3001;
const OPERATOR_COUNT: usize = 1;
const INPUT_VALUE: u64 = 5;
const TOTAL_JOBS_TO_RUN: u64 = 10;
const JOB_INTERVAL_MS: u64 = 2000;
const PROMETHEUS_BLUEPRINT_UID: &str = "prometheus_blueprint_default";
const LOKI_BLUEPRINT_UID: &str = "loki_blueprint_default";
const CUSTOM_NETWORK_NAME: &str = "blueprint-metrics-network";

/// Utility function to clean up any existing Docker containers and networks to avoid conflicts
async fn cleanup_docker_containers(
    _harness: &TangleTestHarness<()>,
) -> Result<(), TestRunnerError> {
    info!("Cleaning up existing Docker containers before test...");

    let _grafana_rm = Command::new("docker")
        .args(["rm", "-f", TEST_GRAFANA_CONTAINER_NAME])
        .output()
        .await;

    let _loki_rm = Command::new("docker")
        .args(["rm", "-f", TEST_LOKI_CONTAINER_NAME])
        .output()
        .await;

    let _prometheus_rm = Command::new("docker")
        .args(["rm", "-f", TEST_PROMETHEUS_CONTAINER_NAME])
        .output()
        .await;

    let _network_rm = Command::new("docker")
        .args(["network", "rm", CUSTOM_NETWORK_NAME])
        .output()
        .await;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    Ok(())
}

/// Demonstrates the complete QoS metrics system with Grafana, Loki, and Prometheus.
#[tokio::test]
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::large_futures
)]
async fn test_qos_metrics_demo() -> Result<(), TestRunnerError> {
    setup_log();
    info!(
        "Starting QoS metrics demo with {} job runs",
        TOTAL_JOBS_TO_RUN
    );

    info!("Creating test blueprint with QoS integration");
    let (temp_dir, blueprint_dir) = create_test_blueprint();

    let harness: TangleTestHarness<()> = TangleTestHarness::setup(temp_dir).await?;
    let ws_rpc_url = harness.env().ws_rpc_endpoint.clone();
    let keystore_uri = harness.env().keystore_uri.clone();
    std::env::set_current_dir(&blueprint_dir).unwrap();
    cleanup_docker_containers(&harness).await?;

    info!("Setting up test service with {} operators", OPERATOR_COUNT);
    let setup_services_opts = SetupServicesOpts {
        exit_after_registration: false,
        ..Default::default()
    };
    let (mut test_env, service_id, blueprint_id) = harness
        .setup_services_with_options::<OPERATOR_COUNT>(setup_services_opts)
        .await?;
    info!("Test environment initialized, submitting jobs to generate metrics...");

    info!("Blueprint includes QoS integration");

    info!("Initializing test environment");
    test_env.initialize().await?;

    let operator_index = 0;
    info!("Using operator index {} for testing", operator_index);

    let node_handle = {
        let nodes = test_env.nodes.read().await;
        match &nodes[operator_index] {
            NodeSlot::Occupied(node) => node.clone(),
            NodeSlot::Empty => {
                return Err(TestRunnerError::Setup(format!(
                    "Node {} is not initialized",
                    operator_index
                )));
            }
        }
    };

    info!("Adding square job to node handle");
    node_handle.add_job(utils::square.layer(TangleLayer)).await;

    let mut qos_config = default_qos_config();

    if let Some(metrics_conf) = qos_config.metrics.as_mut() {
        if let Some(prom_server_conf) = metrics_conf.prometheus_server.as_mut() {
            prom_server_conf.port = 9091;
            prom_server_conf.host = "0.0.0.0".to_string();
            prom_server_conf.use_docker = false;
        } else {
            metrics_conf.prometheus_server = Some(PrometheusServerConfig {
                port: 9091,
                host: "0.0.0.0".to_string(),
                use_docker: false,
                docker_image: "prom/prometheus:latest".to_string(),
                docker_container_name: "blueprint-embedded-prometheus".to_string(),
                config_path: None,
                data_path: None,
            });
        }
    }

    qos_config.heartbeat = Some(HeartbeatConfig {
        interval_secs: 5,
        jitter_percent: 5,
        service_id,
        blueprint_id: 1,
        max_missed_heartbeats: 3,
    });

    qos_config.grafana_server = Some(GrafanaServerConfig {
        port: GRAFANA_PORT,
        ..Default::default()
    });

    let prometheus_config_path =
        "/home/tjemmmic/webb/blueprint/crates/qos/tests/config/prometheus.yml";
    qos_config.prometheus_server = Some(PrometheusServerConfig {
        use_docker: true,
        docker_container_name: "blueprint-prometheus".to_string(),
        config_path: Some(prometheus_config_path.to_string()),
        ..Default::default()
    });

    let prometheus_datasource_url = "http://blueprint-prometheus:9090".to_string();

    qos_config.grafana = Some(blueprint_qos::logging::grafana::GrafanaConfig {
        url: format!("http://localhost:{}", GRAFANA_PORT),
        api_key: String::new(),
        org_id: None,
        admin_user: Some("admin".to_string()),
        admin_password: Some("admin".to_string()),
        folder: None,
        loki_config: None,
        prometheus_datasource_url: Some(prometheus_datasource_url),
    });

    let docker = DockerManager::new().unwrap();
    info!(
        "Ensuring custom Docker network '{}' exists...",
        CUSTOM_NETWORK_NAME
    );
    match docker.create_network(CUSTOM_NETWORK_NAME).await {
        Ok(()) => info!("Docker network '{}' created.", CUSTOM_NETWORK_NAME),
        Err(e) => {
            if !e.to_string().contains("already exists") {
                return Err(TestRunnerError::Setup(format!(
                    "Failed to create docker network: {}",
                    e
                )));
            }
            info!("Docker network '{}' already exists.", CUSTOM_NETWORK_NAME);
        }
    }

    qos_config.docker_network = Some(CUSTOM_NETWORK_NAME.to_string());
    qos_config.manage_servers = true;

    qos_config.loki = Some(blueprint_qos::logging::loki::LokiConfig {
        url: "http://blueprint-loki:3100".to_string(),
        username: Some("test-tenant".to_string()),
        ..Default::default()
    });

    let qos_service = QoSService::new(
        qos_config.clone(),
        Arc::new(MockHeartbeatConsumer::new()),
        ws_rpc_url.to_string(),
        keystore_uri,
    )
    .await
    .unwrap();

    info!("Creating QoS service...");
    let qos_service_arc = Arc::new(qos_service);

    let nodes = test_env.nodes.read().await;
    for (i, node_slot) in nodes.iter().enumerate() {
        match node_slot {
            NodeSlot::Occupied(node_handle) => {
                info!("Setting QoS service for node handle {}...", i);
                node_handle.set_qos_service(qos_service_arc.clone()).await;
            }
            NodeSlot::Empty => {
                warn!("Node slot {} is empty, skipping QoS service setup.", i);
            }
        }
    }
    info!("QoS service set on all active node handles.");

    qos_service_arc.debug_server_status();

    let prom_config = qos_config
        .prometheus_server
        .as_ref()
        .expect("Prometheus server config should be set");
    let prometheus_host_for_grafana = if prom_config.use_docker {
        prom_config.docker_container_name.clone()
    } else {
        "host.docker.internal".to_string()
    };
    let prometheus_datasource_url = format!(
        "http://{}:{}",
        prometheus_host_for_grafana, prom_config.port
    );

    if let Some(url) = qos_service_arc.grafana_server_url() {
        info!("Grafana is running at: {}", url);
    } else {
        warn!("Grafana URL not available, but continuing");
    }

    if let Some(grafana_client) = qos_service_arc.grafana_client() {
        // Create the Loki datasource
        info!(
            "Attempting to create Loki datasource: UID {}",
            LOKI_BLUEPRINT_UID
        );
        let loki_ds_request = CreateDataSourceRequest {
            name: "Blueprint Loki (Test)".to_string(),
            ds_type: "loki".to_string(),
            url: "http://localhost:3100".to_string(), // Assuming Loki runs on localhost for the test harness
            access: "proxy".to_string(),
            uid: Some(LOKI_BLUEPRINT_UID.to_string()),
            is_default: Some(false),
            json_data: Some(serde_json::json!({"maxLines": 1000})),
        };
        match grafana_client
            .create_or_update_datasource(loki_ds_request)
            .await
        {
            Ok(response) => {
                info!(
                    "Successfully created/updated Loki datasource '{}' (UID: {}) via test. Name from response: '{}', ID: {:?}",
                    "Blueprint Loki (Test)",
                    LOKI_BLUEPRINT_UID,
                    response.datasource.name,
                    response.datasource.id
                );
                debug!("Full Loki datasource creation response: {:?}", response);
            }
            Err(e) => error!(
                "Failed to create/update Loki datasource (UID: {}) via test: {}",
                LOKI_BLUEPRINT_UID, e
            ),
        }

        // Explicitly create the Prometheus datasource
        info!(
            "Attempting to create Prometheus datasource: UID {}",
            PROMETHEUS_BLUEPRINT_UID
        );
        let prometheus_ds_request = CreateDataSourceRequest {
            name: "Blueprint Prometheus (Test)".to_string(),
            ds_type: "prometheus".to_string(),
            url: prometheus_datasource_url.clone(), // This should be http://host.docker.internal:9090 for Grafana container to reach host Prometheus
            access: "proxy".to_string(),
            uid: Some(PROMETHEUS_BLUEPRINT_UID.to_string()),
            is_default: Some(false),
            json_data: Some(json!({
                "httpMethod": "GET",
                "timeInterval": "15s",
            })),
        };

        match grafana_client
            .create_or_update_datasource(prometheus_ds_request)
            .await
        {
            Ok(response) => {
                info!(
                    "Successfully created/updated Prometheus datasource '{}' (UID: {}) via test. Name from response: '{}', ID: {:?}",
                    "Blueprint Prometheus (Test)",
                    PROMETHEUS_BLUEPRINT_UID,
                    response.datasource.name,
                    response.datasource.id
                );
                debug!(
                    "Full Prometheus datasource creation response: {:?}",
                    response
                );

                // Perform a health check on the newly created Prometheus datasource
                info!(
                    "Performing health check for Prometheus datasource UID: {}",
                    PROMETHEUS_BLUEPRINT_UID
                );
                match grafana_client
                    .check_datasource_health(PROMETHEUS_BLUEPRINT_UID)
                    .await
                {
                    Ok(health_response) => {
                        if health_response.status == "OK" {
                            info!(
                                "Prometheus datasource '{}' (UID: {}) health check successful: Status: {}, Message: '{}'",
                                "Blueprint Prometheus (Test)",
                                PROMETHEUS_BLUEPRINT_UID,
                                health_response.status,
                                health_response.message
                            );
                        } else {
                            warn!(
                                "Prometheus datasource '{}' (UID: {}) is not healthy: Status: {}, Message: '{}'. This might cause dashboard issues.",
                                "Blueprint Prometheus (Test)",
                                PROMETHEUS_BLUEPRINT_UID,
                                health_response.status,
                                health_response.message
                            );
                        }
                    }
                    Err(e) => {
                        warn!(
                            "Failed to check Prometheus datasource health for UID {} in test: {}. Dashboard panels might fail.",
                            PROMETHEUS_BLUEPRINT_UID, e
                        );
                    }
                }
            }
            Err(e) => {
                error!(
                    "Failed to create/update Prometheus datasource (UID: {}) via test: {}",
                    PROMETHEUS_BLUEPRINT_UID, e
                );
            }
        }

        // Create the dashboard, assuming datasources are now set up
        info!("Creating dashboard with pre-configured datasources using direct method...");
        match grafana_client
            .create_blueprint_dashboard(
                service_id,
                blueprint_id,
                PROMETHEUS_BLUEPRINT_UID,
                LOKI_BLUEPRINT_UID,
            )
            .await
        {
            Ok(dashboard_url) => {
                info!("Successfully created Grafana dashboard: {}", dashboard_url);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await; // Allow time for dashboard to settle if needed
                info!("Waited 5 seconds after dashboard creation.");
            }
            Err(e) => warn!(
                "Failed to create Grafana dashboard: {:?}. Proceeding without it.",
                e
            ),
        }
    } else {
        warn!(
            "No Grafana client available; skipping Loki & Prometheus datasource creation and dashboard creation."
        );
    }
    let otel_job_counter = qos_service_arc
        .provider()
        .expect("Metrics provider should be configured and available in QoSService")
        .get_otel_job_executions_counter();
    info!("Successfully fetched otel_job_counter from QoSService");

    // Set the QoSService on the NodeHandle (this moves qos_service)
    node_handle.set_qos_service(qos_service_arc.clone()).await;
    info!("QoS Service has been set on NodeHandle");

    // Start the BlueprintRunner
    info!("Starting BlueprintRunner with node handle");
    node_handle
        .start_runner(())
        .await
        .map_err(|e| TestRunnerError::Setup(format!("Failed to start runner: {}", e)))?;

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
            .map_err(|e| TestRunnerError::Setup(format!("Failed to submit job: {}", e)))?;

        // Wait for job execution completion
        let result = harness
            .wait_for_job_execution(service_id, call)
            .await
            .map_err(|e| {
                TestRunnerError::Setup(format!("Failed to wait for job execution: {}", e))
            })?;

        // Verify job result
        harness.verify_job(
            &result,
            vec![OutputValue::Uint64(INPUT_VALUE * INPUT_VALUE)],
        );

        // Record metrics
        jobs_completed += 1;
        job_executions.set(jobs_completed as i64);
        job_success.set(jobs_completed as i64);
        let latency = i64::try_from(job_start.elapsed().as_millis()).unwrap();
        job_latency.set(latency);

        info!(
            "Job #{} completed: {} squared = {} (took {} ms)",
            jobs_completed,
            INPUT_VALUE,
            INPUT_VALUE * INPUT_VALUE,
            latency
        );

        // Increment the OTel counter directly
        info!(
            "OTel: Incrementing otel_job_executions_counter directly for job_id: {}, execution_time_ms: {}",
            utils::XSQUARE_JOB_ID,
            latency as f64
        );
        otel_job_counter.add(
            1,
            &[
                KeyValue::new("service_id", service_id.to_string()),
                KeyValue::new("blueprint_id", blueprint_id.to_string()),
            ],
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

    info!("Servers will remain running for 30 more seconds for metrics viewing");
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    cleanup_docker_containers(&harness).await?;

    Ok(())
}
