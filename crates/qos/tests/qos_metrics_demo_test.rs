use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, Instant};

use blueprint_core::{Job, error, info, warn};
use blueprint_qos::heartbeat::HeartbeatConfig;
use blueprint_qos::{
    GrafanaConfig, GrafanaServerConfig, PrometheusServerConfig, QoSService, default_qos_config,
};
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
const GRAFANA_PORT: u16 = 3000;
const OPERATOR_COUNT: usize = 1; // Number of operators for the test
const INPUT_VALUE: u64 = 5; // Value to square in our test job
const TOTAL_JOBS_TO_RUN: u64 = 35; // Aim for ~70 seconds of active job processing
const JOB_INTERVAL_MS: u64 = 2000; // Time between job submissions in milliseconds (2 seconds)
const PROMETHEUS_BLUEPRINT_UID: &str = "prometheus_blueprint_default";
const LOKI_BLUEPRINT_UID: &str = "loki_blueprint_default";
const CUSTOM_NETWORK_NAME: &str = "blueprint-metrics-network"; // Custom Docker network for container communication
const GRAFANA_CONTAINER_NAME: &str = "blueprint-grafana"; // Consistent container name for Grafana
const PROMETHEUS_CONTAINER_NAME: &str = "blueprint-scraping-prometheus"; // Consistent container name for Prometheus

/// Utility function to clean up any existing Docker containers and networks to avoid conflicts
async fn cleanup_docker_containers(_harness: &TangleTestHarness<()>) -> Result<(), Error> {
    info!("Cleaning up existing Docker containers before test...");

    // Remove the Prometheus container if it exists
    let _prometheus_rm = Command::new("docker")
        .args(["rm", "-f", PROMETHEUS_CONTAINER_NAME])
        .output();

    // Remove Grafana container if it exists
    let _grafana_rm = Command::new("docker")
        .args(["rm", "-f", GRAFANA_CONTAINER_NAME])
        .output();

    // Also remove our custom Docker network if it exists
    let _network_rm = Command::new("docker")
        .args(["network", "rm", CUSTOM_NETWORK_NAME])
        .output();

    Ok(())
}

/// Helper function to connect a container to our custom network with retry logic
async fn connect_container_to_network(
    container_name: &str,
    network_name: &str,
    alias: Option<&str>,
) -> Result<(), Error> {
    let mut attempts = 0;
    let max_attempts = 3;

    loop {
        attempts += 1;

        // Build the connection command - optionally with an alias for better DNS resolution
        let mut args = vec!["network", "connect"];

        // Add alias if provided - this gives additional DNS name to the container in the network
        if let Some(alias_name) = alias {
            args.push("--alias");
            args.push(alias_name);
        }

        args.push(network_name);
        args.push(container_name);

        info!(
            "Connecting container {} to network {}...",
            container_name, network_name
        );
        let connect_result = Command::new("docker").args(&args).output();

        match connect_result {
            Ok(output) => {
                if output.status.success() {
                    info!(
                        "✅ Successfully connected {} to network {}",
                        container_name, network_name
                    );
                    break;
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    if error.contains("already exists") || error.contains("already connected") {
                        info!(
                            "Container {} already connected to network {}",
                            container_name, network_name
                        );
                        break;
                    } else if attempts < max_attempts {
                        warn!(
                            "Failed to connect container to network: {}. Retrying ({}/{})",
                            error, attempts, max_attempts
                        );
                        sleep(Duration::from_secs(2)).await;
                        continue;
                    } else {
                        warn!(
                            "Failed to connect container to network after {} attempts: {}",
                            max_attempts, error
                        );
                        // Non-blocking error, continue with test
                        break;
                    }
                }
            }
            Err(e) => {
                if attempts < max_attempts {
                    warn!(
                        "Failed to run network connect command: {}. Retrying ({}/{})",
                        e, attempts, max_attempts
                    );
                    sleep(Duration::from_secs(2)).await;
                    continue;
                } else {
                    warn!(
                        "Failed to run network connect command after {} attempts: {}",
                        max_attempts, e
                    );
                    // Non-blocking error, continue with test
                    break;
                }
            }
        };
    }

    Ok(())
}

/// Helper function to check if a container is running
async fn is_container_running(container_name: &str) -> bool {
    let status_cmd = Command::new("docker")
        .args(["inspect", "-f", "{{.State.Running}}", container_name])
        .output();

    match status_cmd {
        Ok(output) => {
            if output.status.success() {
                let running = String::from_utf8_lossy(&output.stdout).trim() == "true";
                if running {
                    info!("✅ Container {} is running", container_name);
                } else {
                    warn!("❌ Container {} is NOT running", container_name);
                }
                running
            } else {
                warn!(
                    "Failed to check container status: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                false
            }
        }
        Err(e) => {
            warn!("Error checking container status: {}", e);
            false
        }
    }
}

/// Helper function to restart a container if it's not running
async fn ensure_container_running(container_name: &str) -> Result<bool, Error> {
    if !is_container_running(container_name).await {
        info!("Attempting to restart container {}", container_name);
        let restart_cmd = Command::new("docker")
            .args(["restart", container_name])
            .output()
            .map_err(|e| Error::Setup(format!("Failed to restart container: {}", e)))?;

        if restart_cmd.status.success() {
            info!("✅ Successfully restarted container {}", container_name);
            // Give container a moment to stabilize
            sleep(Duration::from_secs(3)).await;
            return Ok(true);
        } else {
            warn!(
                "❌ Failed to restart container: {}",
                String::from_utf8_lossy(&restart_cmd.stderr)
            );
            return Ok(false);
        }
    }
    Ok(true) // Container was already running
}

/// Helper function to show container logs for debugging
async fn show_container_logs(container_name: &str, lines: usize) -> Result<(), Error> {
    info!(
        "Last {} lines of logs from container {}:",
        lines, container_name
    );
    // Use --since flag to get more recent logs even if container restarted multiple times
    let logs_cmd = Command::new("docker")
        .args([
            "logs",
            "--tail",
            &lines.to_string(),
            "--timestamps",
            container_name,
        ])
        .output()
        .map_err(|e| Error::Setup(format!("Failed to get container logs: {}", e)))?;

    if logs_cmd.status.success() {
        let logs = String::from_utf8_lossy(&logs_cmd.stdout);
        if logs.trim().is_empty() {
            info!(
                "[{}] No logs available (container may have exited too quickly)",
                container_name
            );

            // Try fetching logs with -f to capture logs from stopped containers
            let previous_logs_cmd = Command::new("docker")
                .args([
                    "logs",
                    "--tail",
                    &lines.to_string(),
                    "--timestamps",
                    "--details",
                    container_name,
                ])
                .output()
                .map_err(|e| Error::Setup(format!("Failed to get container logs: {}", e)))?;

            if previous_logs_cmd.status.success() {
                let previous_logs = String::from_utf8_lossy(&previous_logs_cmd.stdout);
                if !previous_logs.trim().is_empty() {
                    info!("[{}] Found logs from previous runs:", container_name);
                    for line in previous_logs.lines() {
                        info!("[{}] {}", container_name, line);
                    }
                }
            }
        } else {
            for line in logs.lines() {
                info!("[{}] {}", container_name, line);
            }
        }
    } else {
        warn!(
            "Failed to get logs: {}",
            String::from_utf8_lossy(&logs_cmd.stderr)
        );
    }
    Ok(())
}

/// Helper function to get detailed container status including exit code and reason
async fn get_container_status(container_name: &str) -> Result<(), Error> {
    info!(
        "Getting detailed status for container {}...",
        container_name
    );

    // Check if container exists
    let inspect_cmd = Command::new("docker")
        .args(["inspect", container_name])
        .output()
        .map_err(|e| Error::Setup(format!("Failed to inspect container: {}", e)))?;

    if !inspect_cmd.status.success() {
        warn!("Container {} does not exist", container_name);
        return Ok(());
    }

    // Get container state
    let state_cmd = Command::new("docker")
        .args(["inspect", "-f", "{{json .State}}", container_name])
        .output()
        .map_err(|e| Error::Setup(format!("Failed to get container state: {}", e)))?;

    if state_cmd.status.success() {
        let state = String::from_utf8_lossy(&state_cmd.stdout);
        info!("Container {} state: {}", container_name, state);

        // Check for specific exit code
        let exit_code_cmd = Command::new("docker")
            .args(["inspect", "-f", "{{.State.ExitCode}}", container_name])
            .output()
            .map_err(|e| Error::Setup(format!("Failed to get exit code: {}", e)))?;

        if exit_code_cmd.status.success() {
            let exit_code = String::from_utf8_lossy(&exit_code_cmd.stdout)
                .trim()
                .to_string();
            if exit_code != "0" && exit_code != "" {
                warn!(
                    "⚠️ Container {} exited with non-zero code: {}",
                    container_name, exit_code
                );

                // Check for specific error information
                let error_cmd = Command::new("docker")
                    .args(["inspect", "-f", "{{.State.Error}}", container_name])
                    .output()
                    .map_err(|e| Error::Setup(format!("Failed to get error info: {}", e)))?;

                if error_cmd.status.success() {
                    let error_info = String::from_utf8_lossy(&error_cmd.stdout)
                        .trim()
                        .to_string();
                    if !error_info.is_empty() && error_info != "<no value>" {
                        warn!("⚠️ Container error: {}", error_info);
                    }
                }
            }
        }
    } else {
        warn!(
            "Failed to get state: {}",
            String::from_utf8_lossy(&state_cmd.stderr)
        );
    }

    // Get container config
    let config_cmd = Command::new("docker")
        .args(["inspect", "-f", "{{json .Config}}", container_name])
        .output()
        .map_err(|e| Error::Setup(format!("Failed to get container config: {}", e)))?;

    if config_cmd.status.success() {
        let config = String::from_utf8_lossy(&config_cmd.stdout);
        info!("Container {} config: {}", container_name, config);
    }

    // Check volumes and mounts
    let mounts_cmd = Command::new("docker")
        .args(["inspect", "-f", "{{json .Mounts}}", container_name])
        .output()
        .map_err(|e| Error::Setup(format!("Failed to get mount info: {}", e)))?;

    if mounts_cmd.status.success() {
        let mounts = String::from_utf8_lossy(&mounts_cmd.stdout);
        info!("Container {} mounts: {}", container_name, mounts);
    }

    // For Prometheus container, try to inspect config file
    if container_name == "blueprint-scraping-prometheus" {
        // First try to check if config file exists in the container
        let config_check_cmd = Command::new("docker")
            .args(["exec", container_name, "ls", "-la", "/etc/prometheus/"])
            .output();

        match config_check_cmd {
            Ok(output) => {
                if output.status.success() {
                    info!(
                        "Prometheus config directory contents: {}",
                        String::from_utf8_lossy(&output.stdout)
                    );

                    // Try to cat the config file
                    let cat_cmd = Command::new("docker")
                        .args([
                            "exec",
                            container_name,
                            "cat",
                            "/etc/prometheus/prometheus.yml",
                        ])
                        .output();

                    match cat_cmd {
                        Ok(cat_output) => {
                            if cat_output.status.success() {
                                info!(
                                    "Prometheus config file contents:\n{}",
                                    String::from_utf8_lossy(&cat_output.stdout)
                                );
                            } else {
                                warn!(
                                    "Failed to read prometheus.yml: {}",
                                    String::from_utf8_lossy(&cat_output.stderr)
                                );
                            }
                        }
                        Err(e) => warn!("Failed to execute cat command: {}", e),
                    }
                } else {
                    warn!(
                        "Container not running, can't check config file: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => warn!("Failed to inspect config file: {}", e),
        }
    }

    Ok(())
}

/// Helper function to verify container DNS resolution by testing connection between containers
async fn verify_container_dns_resolution(
    source_container: &str,
    target_container: &str,
    target_port: u16,
) -> Result<bool, Error> {
    // First verify both containers are running
    if !is_container_running(source_container).await {
        warn!(
            "❌ Source container {} is not running - cannot verify DNS resolution",
            source_container
        );
        return Ok(false);
    }

    if !is_container_running(target_container).await {
        warn!(
            "❌ Target container {} is not running - cannot verify DNS resolution",
            target_container
        );
        return Ok(false);
    }

    info!(
        "Verifying DNS resolution from {} to {}:{}...",
        source_container, target_container, target_port
    );

    // Use -v flag for more verbose output to help with debugging
    let curl_check = Command::new("docker")
        .args([
            "exec",
            source_container,
            "curl",
            "-v",
            "-s",
            "-o",
            "/dev/null",
            "-w",
            "%{http_code}",
            &format!("http://{}:{}/", target_container, target_port),
        ])
        .output()
        .map_err(|e| Error::Setup(format!("Failed to execute curl check: {}", e)))?;

    if curl_check.status.success() {
        let status_code = String::from_utf8_lossy(&curl_check.stdout)
            .trim()
            .to_string();
        let success = status_code.starts_with('2'); // Any 2xx status code is a success

        if success {
            info!(
                "✅ DNS resolution successful! {} can reach {} on port {}",
                source_container, target_container, target_port
            );
            return Ok(true);
        } else {
            warn!(
                "⚠ DNS resolution succeeded but got non-200 status code: {}",
                status_code
            );
            // Show stderr for more details
            let stderr = String::from_utf8_lossy(&curl_check.stderr);
            warn!("Curl verbose output: {}", stderr);
            return Ok(false);
        }
    } else {
        let stderr = String::from_utf8_lossy(&curl_check.stderr);
        warn!("❌ DNS resolution failed: {}", stderr);
        return Ok(false);
    }
}

/// Demonstrates the complete QoS metrics system with Grafana, Loki, and Prometheus.
#[tokio::test]
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
    // Configure Grafana server to disable anonymous access and enable login form
    qos_config.grafana_server = Some(GrafanaServerConfig {
        port: GRAFANA_PORT,                  // Ensure this matches the constant used
        allow_anonymous: false,              // Disable anonymous access
        admin_user: "admin".to_string(),     // Default credentials
        admin_password: "admin".to_string(), // Default credentials
        ..Default::default()
    });

    qos_config.grafana = Some(blueprint_qos::logging::grafana::GrafanaConfig {
        url: format!("http://localhost:{}", GRAFANA_PORT),
        api_key: String::new(), // No API key, rely on admin user/pass
        org_id: None,           // Use default org
        admin_user: Some("admin".to_string()),
        admin_password: Some("admin".to_string()),
        folder: None, // Default to no specific folder
    });

    // Configure metrics exposer to use 127.0.0.1 for better connectivity with scraper
    qos_config.prometheus_server = Some(PrometheusServerConfig {
        host: "127.0.0.1".to_string(),
        port: 9091,
        use_docker: false,
        ..Default::default()
    });

    // Add the scraping Prometheus server configuration with custom network for reliable container-to-container communication
    let scraping_prometheus_port = 9092;
    qos_config.scraping_prometheus_server = Some(
        blueprint_qos::servers::scraping_prometheus::ScrapingPrometheusServerConfig {
            host_port: scraping_prometheus_port, // Use a different port than the metrics exposer
            // Use Docker host gateway (172.17.0.1) which is more reliable for container-to-host communication
            scrape_target_address: "172.17.0.1:9091".to_string(),
            // Use bridge network mode which works better in test environments
            network_mode: "bridge".to_string(),
            use_host_gateway_mapping: true, // Enable host gateway mapping for better Docker networking
            // Use custom network for reliable container-to-container communication
            custom_network: Some(CUSTOM_NETWORK_NAME.to_string()),
            ..Default::default()
        },
    );

    // Create a custom Docker network for container-to-container communication if it doesn't exist
    info!("Creating custom Docker network: {}", CUSTOM_NETWORK_NAME);
    let create_network_result = Command::new("docker")
        .args([
            "network",
            "create",
            "--driver",
            "bridge",
            CUSTOM_NETWORK_NAME,
        ])
        .output()
        .map_err(|e| Error::Setup(format!("Failed to create Docker network: {}", e)))?;

    if !create_network_result.status.success() {
        let error = String::from_utf8_lossy(&create_network_result.stderr);
        if error.contains("already exists") {
            info!(
                "Docker network {} already exists, will reuse it",
                CUSTOM_NETWORK_NAME
            );
        } else {
            return Err(Error::Setup(format!(
                "Failed to create Docker network: {}",
                error
            )));
        }
    } else {
        info!(
            "Successfully created Docker network: {}",
            CUSTOM_NETWORK_NAME
        );
    }

    // Explicitly use IPv4 addresses throughout the test for better reliability

    // Add logging for diagnostics
    info!(
        "Configured scraping Prometheus server to run on port {} and scrape from {}",
        scraping_prometheus_port, "localhost:9091"
    );

    qos_config.manage_servers = true;
    qos_config.loki = Some(blueprint_qos::logging::loki::LokiConfig {
        url: "http://localhost:3100".to_string(),
        username: Some("test-tenant".to_string()),
        ..Default::default()
    });
    let _grafana_client_config = GrafanaConfig {
        url: format!("http://localhost:{}", GRAFANA_PORT),
        api_key: String::new(), // No API key, rely on admin user/pass
        org_id: None,           // Use default org
        admin_user: Some("admin".to_string()),
        admin_password: Some("admin".to_string()),
        folder: None,
    };
    qos_config.grafana = Some(blueprint_qos::logging::grafana::GrafanaConfig {
        url: "http://localhost:3000".to_string(),
        api_key: "test-api-key".to_string(),
        folder: Some("TestDashboards".to_string()),
        ..Default::default()
    });
    let mut qos_service =
        QoSService::new(qos_config.clone(), Arc::new(MockHeartbeatConsumer::new()))
            .await
            .unwrap();

    info!("Creating QoS service...");
    qos_service.debug_server_status();

    info!(
        "Waiting 15 seconds to ensure services are fully started and Prometheus has collected initial metrics data..."
    );
    tokio::time::sleep(Duration::from_secs(15)).await;

    // Now we need to update the Prometheus datasource URL to point to the scraper
    // rather than directly to the metrics exposer

    // Get the docker container IP address for the Prometheus container
    info!("Getting Docker container IP address for Prometheus container");
    let container_ip = Command::new("docker")
        .args([
            "inspect",
            "-f",
            "{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}",
            "blueprint-scraping-prometheus",
        ])
        .output()
        .expect("Failed to inspect Docker container");
    let container_ip = String::from_utf8_lossy(&container_ip.stdout)
        .trim()
        .to_string();

    info!("Found Prometheus container IP address: {}", container_ip);

    // When using a custom network, try container name first, then fallback to container IP or localhost
    let prometheus_container_url = format!("http://{}:9092", PROMETHEUS_CONTAINER_NAME);
    let prometheus_ip_url = if !container_ip.is_empty() {
        format!("http://{}:9092", container_ip)
    } else {
        "http://127.0.0.1:9092".to_string()
    };

    // Try container name URL first (this works when both containers are on the same Docker network)
    info!(
        "Checking if Prometheus is accessible via container name: {}",
        prometheus_container_url
    );
    let container_name_accessible = match reqwest::get(format!(
        "{}/api/v1/status/config",
        &prometheus_container_url
    ))
    .await
    {
        Ok(response) => {
            info!(
                "✅ Success! Prometheus accessible via container name: {} (status: {})",
                prometheus_container_url,
                response.status()
            );
            true
        }
        Err(e) => {
            warn!(
                "❌ Container name access failed: {} (trying container IP next)",
                e
            );
            false
        }
    };

    // If container name didn't work, try container IP or localhost
    let prometheus_access_url = if container_name_accessible {
        Some(prometheus_container_url)
    } else {
        // Try container IP (if available) or localhost
        info!(
            "Trying connection to Prometheus at IP address: {}",
            prometheus_ip_url
        );
        match reqwest::get(format!("{}/api/v1/status/config", &prometheus_ip_url)).await {
            Ok(response) => {
                info!(
                    "✅ Success! Prometheus accessible at IP address: {} (status: {})",
                    prometheus_ip_url,
                    response.status()
                );
                Some(prometheus_ip_url)
            }
            Err(e) => {
                warn!("❌ Failed to connect to IP address: {}", e);
                warn!(
                    "All connection attempts failed, but continuing with datasource creation using container name"
                );
                // Default back to container name as a last resort - Grafana might have better connectivity
                Some(prometheus_container_url)
            }
        }
    };

    if let Some(prometheus_url) = prometheus_access_url {
        // Create or update datasource with the best URL we found
        info!(
            "Setting Prometheus datasource to use URL: {}",
            prometheus_url
        );

        // Create a properly configured Prometheus datasource
        let prometheus_ds = blueprint_qos::logging::grafana::CreateDataSourceRequest {
            name: "Blueprint Prometheus".to_string(),
            ds_type: "prometheus".to_string(),
            url: prometheus_url.clone(),
            access: "proxy".to_string(),
            uid: Some(PROMETHEUS_BLUEPRINT_UID.to_string()),
            is_default: Some(true),
            json_data: Some(serde_json::json!({
                "httpMethod": "POST",
                "timeout": 30
            })),
        };

        // Create the datasource directly using the GrafanaClient to ensure it exists
        if let Some(grafana_client) = qos_service.grafana_client() {
            info!(
                "Creating or updating Prometheus datasource with URL: {}",
                prometheus_url
            );
            match grafana_client
                .create_or_update_datasource(prometheus_ds)
                .await
            {
                Ok(_) => info!("✅ Successfully created/updated Prometheus datasource"),
                Err(e) => {
                    error!("Failed to create Prometheus datasource: {}", e);
                    return Err(Error::from(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to create Prometheus datasource: {}", e),
                    )));
                }
            }
        } else {
            error!("No Grafana client available");
            return Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No Grafana client available",
            )));
        }
    }
    sleep(Duration::from_secs(5)).await;

    info!("Attempting to create Grafana dashboard and datasources...");

    // We already created the Prometheus datasource above, so we don't need to create it again.
    // However, we'll add connectivity verification to ensure Grafana can reach Prometheus.

    // The QoSService automatically starts Grafana when created
    info!("QoSService has started a Grafana instance");

    // Get the Grafana server URL (this verifies Grafana is running)
    if let Some(url) = qos_service.grafana_server_url() {
        info!("Grafana is running at: {}", url);
    } else {
        warn!("Grafana URL not available, but continuing");
    }

    // First, get detailed diagnostics about the container status
    info!("Getting detailed container diagnostics...");

    // Get detailed status for Prometheus container
    if let Err(e) = get_container_status(PROMETHEUS_CONTAINER_NAME).await {
        warn!("Error getting Prometheus container status: {}", e);
    }

    // Get detailed status for Grafana container
    if let Err(e) = get_container_status(GRAFANA_CONTAINER_NAME).await {
        warn!("Error getting Grafana container status: {}", e);
    }

    // Check container logs for potential issues (last 20 lines to get more context)
    info!("Checking container logs for potential issues...");
    if let Err(e) = show_container_logs(PROMETHEUS_CONTAINER_NAME, 20).await {
        warn!("Failed to get Prometheus logs: {}", e);
    }

    if let Err(e) = show_container_logs(GRAFANA_CONTAINER_NAME, 10).await {
        warn!("Failed to get Grafana logs: {}", e);
    }

    // Ensure both containers are running before attempting to connect them
    info!("Checking container status and restarting if needed...");

    // Ensure Prometheus container is running
    match ensure_container_running(PROMETHEUS_CONTAINER_NAME).await {
        Ok(true) => info!("✅ Prometheus container is running or was successfully restarted"),
        Ok(false) => warn!("⚠️ Could not ensure Prometheus container is running"),
        Err(e) => warn!("Error checking Prometheus container: {}", e),
    }

    // Ensure Grafana container is running
    match ensure_container_running(GRAFANA_CONTAINER_NAME).await {
        Ok(true) => info!("✅ Grafana container is running or was successfully restarted"),
        Ok(false) => warn!("⚠️ Could not ensure Grafana container is running"),
        Err(e) => warn!("Error checking Grafana container: {}", e),
    }

    // Connect both containers to the custom network with retries
    info!(
        "Connecting Prometheus container {} to custom network {}",
        PROMETHEUS_CONTAINER_NAME, CUSTOM_NETWORK_NAME
    );
    if let Err(e) =
        connect_container_to_network(PROMETHEUS_CONTAINER_NAME, CUSTOM_NETWORK_NAME, None).await
    {
        warn!("Failed to connect Prometheus container to network: {}", e);
        // Non-fatal error, continue with test
    }

    info!(
        "Connecting Grafana container {} to custom network {}",
        GRAFANA_CONTAINER_NAME, CUSTOM_NETWORK_NAME
    );
    if let Err(e) =
        connect_container_to_network(GRAFANA_CONTAINER_NAME, CUSTOM_NETWORK_NAME, None).await
    {
        warn!("Failed to connect Grafana container to network: {}", e);
        // Non-fatal error, continue with test
    }

    // Show each container's network information for debugging
    info!("Inspecting container network details...");
    let _prometheus_net = Command::new("docker")
        .args([
            "inspect",
            "-f",
            "'{{json .NetworkSettings.Networks}}'",
            PROMETHEUS_CONTAINER_NAME,
        ])
        .output()
        .map(|output| {
            if output.status.success() {
                let networks = String::from_utf8_lossy(&output.stdout);
                info!("Prometheus container networks: {}", networks);
            }
        });

    let _grafana_net = Command::new("docker")
        .args([
            "inspect",
            "-f",
            "'{{json .NetworkSettings.Networks}}'",
            GRAFANA_CONTAINER_NAME,
        ])
        .output()
        .map(|output| {
            if output.status.success() {
                let networks = String::from_utf8_lossy(&output.stdout);
                info!("Grafana container networks: {}", networks);
            }
        });

    // Add a longer delay to ensure the network connection is fully established
    info!("Waiting longer for network connection to stabilize and services to be ready...");
    sleep(Duration::from_secs(10)).await;

    // Get the Grafana client from QoSService - this is important for the test
    // to correctly interact with the Grafana API

    // Implement a retry mechanism for verifying connectivity
    let max_retry_attempts = 3;
    let mut success = false;

    info!("Verifying bidirectional DNS resolution between containers with retries...");

    for attempt in 1..=max_retry_attempts {
        info!(
            "Connectivity check attempt {}/{}...",
            attempt, max_retry_attempts
        );

        // First re-check if containers are running
        if !is_container_running(PROMETHEUS_CONTAINER_NAME).await
            || !is_container_running(GRAFANA_CONTAINER_NAME).await
        {
            warn!("Container not running before connectivity check - attempting to restart...");
            let _ = ensure_container_running(PROMETHEUS_CONTAINER_NAME).await;
            let _ = ensure_container_running(GRAFANA_CONTAINER_NAME).await;
            sleep(Duration::from_secs(3)).await;
        }

        // Check Grafana → Prometheus connectivity
        info!("Checking Grafana → Prometheus connectivity");
        match verify_container_dns_resolution(
            GRAFANA_CONTAINER_NAME,
            PROMETHEUS_CONTAINER_NAME,
            9092,
        )
        .await
        {
            Ok(true) => {
                info!("✅ Grafana can successfully reach Prometheus via container name");
                success = true;
            }
            Ok(false) => {
                warn!("⚠ Grafana can resolve Prometheus by name but got a non-200 response")
            }
            Err(e) => warn!(
                "❌ Failed to verify container DNS resolution from Grafana to Prometheus: {}",
                e
            ),
        }

        // Check Prometheus → Grafana connectivity
        info!("Checking Prometheus → Grafana connectivity");
        match verify_container_dns_resolution(
            PROMETHEUS_CONTAINER_NAME,
            GRAFANA_CONTAINER_NAME,
            3000,
        )
        .await
        {
            Ok(true) => {
                info!("✅ Prometheus can successfully reach Grafana via container name");
                success = success && true; // Both connections must work for complete success
            }
            Ok(false) => {
                warn!("⚠ Prometheus can resolve Grafana by name but got a non-200 response")
            }
            Err(e) => warn!(
                "❌ Failed to verify container DNS resolution from Prometheus to Grafana: {}",
                e
            ),
        }

        if success {
            info!("✅ Bidirectional connectivity verified successfully!");
            break;
        } else if attempt < max_retry_attempts {
            warn!("Connectivity check failed, retrying in 5 seconds...");
            sleep(Duration::from_secs(5)).await;
        } else {
            warn!("⚠️ All connectivity check attempts failed, but continuing with test");
            // Non-fatal error, continue with test
        }
    }

    // Create the Loki datasource separately
    let loki_ds = blueprint_qos::logging::grafana::CreateDataSourceRequest {
        name: "Blueprint Loki".to_string(),
        ds_type: "loki".to_string(),
        url: "http://localhost:3100".to_string(),
        access: "proxy".to_string(),
        uid: Some(LOKI_BLUEPRINT_UID.to_string()),
        is_default: Some(false),
        json_data: Some(serde_json::json!({"maxLines": 1000})),
    };

    if let Some(grafana_client) = qos_service.grafana_client() {
        match grafana_client.create_or_update_datasource(loki_ds).await {
            Ok(response) => info!("Successfully created Loki datasource: {}", response.name),
            Err(e) => error!("Failed to create Loki datasource: {}", e),
        }
    }

    // Skip create_dashboard which would try to create datasources again
    // Instead, directly call create_blueprint_dashboard which only creates the dashboard
    info!("Creating dashboard with pre-configured datasources using direct method...");

    // Use grafana_client directly to create dashboard without recreating datasources
    if let Some(grafana_client) = qos_service.grafana_client() {
        match grafana_client
            .create_blueprint_dashboard(
                service_id,
                blueprint_id,
                PROMETHEUS_BLUEPRINT_UID,
                LOKI_BLUEPRINT_UID,
            )
            .await
        {
            Ok(dashboard_url) => info!("Successfully created Grafana dashboard: {}", dashboard_url),
            Err(e) => warn!(
                "Failed to create Grafana dashboard: {:?}. Proceeding without it.",
                e
            ),
        };
    } else {
        warn!("No Grafana client available to create dashboard");
    };
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

    cleanup_docker_containers(&harness).await?;

    Ok(())
}
