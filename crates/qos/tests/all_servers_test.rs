use async_trait::async_trait;
use blueprint_core::{error, info};
use blueprint_qos::{
    QoSServiceBuilder,
    error::Result as QoSResult,
    heartbeat::{HeartbeatConsumer, HeartbeatStatus},
    metrics::MetricsConfig,
    servers::grafana::GrafanaServerConfig,
    servers::loki::LokiServerConfig,
    servers::prometheus::PrometheusServerConfig,
};
use blueprint_testing_utils::setup_log;
use prometheus::{IntGauge, Opts, Registry};
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use tokio::time::Duration;

/// This test demonstrates running all three servers (Grafana, Loki, and Prometheus)
/// without using volume mounts, which can cause permission issues in some environments.
/// It uses the QoSServiceBuilder to create and manage the servers and create a dashboard.
#[tokio::test]
#[ignore] // Ignore by default since this runs indefinitely
async fn test_all_servers() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_log();

    info!("Starting all servers test...");

    // Configure Prometheus server to use Docker mode for full API support
    let prometheus_config = PrometheusServerConfig {
        port: 9091, // Use a different port to avoid conflicts
        host: "0.0.0.0".to_string(),
        use_docker: true, // Use Docker for full Prometheus API support
        docker_image: "prom/prometheus:latest".to_string(),
        docker_container_name: "blueprint-test-prometheus".to_string(),
        config_path: None,
        data_path: None,
    };

    // Configure Grafana server without volume mounts
    let grafana_config = GrafanaServerConfig {
        port: 3000,
        admin_user: "admin".to_string(),
        admin_password: "admin".to_string(),
        allow_anonymous: true,
        anonymous_role: "Admin".to_string(),
        data_dir: "/var/lib/grafana".to_string(), // This won't be used as we won't mount volumes
        container_name: "blueprint-test-grafana".to_string(),
    };

    // Configure Loki server without volume mounts
    let loki_config = LokiServerConfig {
        port: 3100,
        data_dir: "/var/lib/loki".to_string(), // This won't be used as we won't mount volumes
        container_name: "blueprint-test-loki".to_string(),
    };

    // Configure metrics
    let metrics_config = MetricsConfig::default();

    // Create a mock heartbeat consumer (required by QoSService)
    struct MockHeartbeatConsumer;

    #[async_trait::async_trait]
    impl HeartbeatConsumer for MockHeartbeatConsumer {
        async fn send_heartbeat(&self, _status: &HeartbeatStatus) -> QoSResult<()> {
            // Do nothing in the test
            Ok(())
        }
    }

    // Create the QoS service with all servers enabled
    let consumer = Arc::new(MockHeartbeatConsumer);
    let qos_service = QoSServiceBuilder::new()
        .with_heartbeat_consumer(consumer)
        .with_grafana_server_config(grafana_config)
        .with_loki_server_config(loki_config)
        .with_prometheus_server_config(prometheus_config)
        .with_metrics_config(metrics_config)
        .manage_servers(true)
        .build()
        .await?;

    // Wait a bit for servers to fully initialize
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Print server URLs for easy access
    info!("Server URLs:");
    if let Some(url) = qos_service.grafana_server_url() {
        info!("Grafana: {}", url);
    } else {
        info!("Grafana server not initialized");
    }
    if let Some(url) = qos_service.loki_server_url() {
        info!("Loki: {}", url);
    } else {
        info!("Loki server not initialized");
    }
    if let Some(url) = qos_service.prometheus_server_url() {
        info!("Prometheus: {}", url);
    } else {
        info!("Prometheus server not initialized");
    }

    // Create a Prometheus registry and register test metrics
    let registry = Registry::new();

    // Set up service and blueprint IDs for metrics
    let service_id = 1001;
    let blueprint_id = 2001;

    // Helper function to create and register a gauge with labels
    let create_gauge = |name: &str, help: &str| -> Result<IntGauge, Box<dyn std::error::Error>> {
        let opts = Opts::new(name, help)
            .const_label("service_id", service_id.to_string())
            .const_label("blueprint_id", blueprint_id.to_string());

        let gauge = IntGauge::with_opts(opts).map_err(|e| Box::<dyn std::error::Error>::from(e))?;

        registry
            .register(Box::new(gauge.clone()))
            .map_err(|e| Box::<dyn std::error::Error>::from(e))?;

        Ok(gauge)
    };

    // Create all the gauges for simulation
    info!("Setting up simulated metrics...");
    let cpu_usage = create_gauge(
        "test_blueprint_cpu_usage",
        "Simulated CPU usage for test blueprint",
    )?;

    let memory_usage = create_gauge(
        "test_blueprint_memory_usage",
        "Simulated memory usage for test blueprint",
    )?;

    let job_executions = create_gauge(
        "test_blueprint_job_executions",
        "Simulated job executions for test blueprint",
    )?;

    let job_errors = create_gauge(
        "test_blueprint_job_errors",
        "Simulated job errors for test blueprint",
    )?;

    let heartbeat = create_gauge(
        "test_blueprint_last_heartbeat",
        "Simulated last heartbeat timestamp for test blueprint",
    )?;

    // Create a dashboard using the QoS service's built-in functionality
    info!("Creating dashboard using QoS service...");
    match qos_service
        .create_dashboard("Test Blueprint Dashboard")
        .await
    {
        Ok(_) => info!("Dashboard created successfully using QoS service"),
        Err(e) => error!("Failed to create dashboard: {}", e),
    }

    // Start a background task to update metrics
    let start_time = Instant::now();
    thread::spawn(move || {
        let mut execution_count = 0;
        let mut error_count = 0;

        loop {
            // Update metrics with simulated values
            let elapsed_secs = start_time.elapsed().as_secs() as i64;

            // Simulate CPU usage (0-100%)
            let cpu_value = elapsed_secs % 100;
            cpu_usage.set(cpu_value);

            // Simulate memory usage (0-1024MB)
            let memory_value = elapsed_secs % 1024;
            memory_usage.set(memory_value);

            // Simulate job executions (increasing counter)
            execution_count += 1;
            job_executions.set(execution_count);

            // Simulate occasional errors
            if elapsed_secs % 10 == 0 {
                error_count += 1;
                job_errors.set(error_count);
            }

            // Update heartbeat timestamp
            heartbeat.set(elapsed_secs);

            // Sleep for a bit before updating again
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    info!("Simulated metrics are now being generated");

    // Print information about viewing the dashboard
    if let Some(url) = qos_service.grafana_server_url() {
        info!("You can view the dashboard at {}/dashboards", url);
    }

    info!("All servers started. Sleeping indefinitely...");
    info!("Press Ctrl+C to stop the test.");

    // Sleep for a very long time (effectively indefinitely)
    tokio::time::sleep(Duration::from_secs(3600 * 24)).await; // 24 hours

    Ok(())
}
