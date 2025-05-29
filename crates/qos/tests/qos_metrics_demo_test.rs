use blueprint_core::{error, info, warn};
use blueprint_qos::{
    QoSServiceBuilder,
    metrics::MetricsConfig,
    servers::{
        grafana::GrafanaServerConfig, loki::LokiServerConfig, prometheus::PrometheusServerConfig,
    },
};
use blueprint_testing_utils::setup_log;
use prometheus::{IntGauge, Opts, Registry};
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use tokio::signal;
use tokio::time::Duration;

// Add a MockHeartbeatConsumer implementation for this test
use blueprint_qos::heartbeat::{HeartbeatConsumer, HeartbeatStatus};
use std::sync::Mutex;

#[derive(Clone, Debug)]
struct MockHeartbeatConsumer {
    heartbeats: Arc<Mutex<Vec<HeartbeatStatus>>>,
}

impl MockHeartbeatConsumer {
    fn new() -> Self {
        Self {
            heartbeats: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl HeartbeatConsumer for MockHeartbeatConsumer {
    async fn send_heartbeat(
        &self,
        status: &HeartbeatStatus,
    ) -> Result<(), blueprint_qos::error::Error> {
        self.heartbeats.lock().unwrap().push(status.clone());
        Ok(())
    }
}

/// Demonstrates the complete QoS metrics system with Grafana, Loki, and Prometheus.
///
/// Features:
/// - Metrics collection via Prometheus
/// - Dashboard visualization in Grafana
/// - Log collection in Loki
/// - Simulated metrics generation
/// - Automatic dashboard setup
///
/// Note: This test runs until manually terminated with Ctrl+C.
#[tokio::test]
#[ignore] // Ignore by default since this is a demo test that runs until manually stopped - run with: cargo test test_qos_metrics_demo -- --ignored
async fn test_qos_metrics_demo() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging system
    setup_log();
    info!("Starting QoS metrics demonstration");

    // Setup mock heartbeat consumer
    let heartbeat_consumer = Arc::new(MockHeartbeatConsumer::new());

    // Configure server ports
    let grafana_port = 3000;
    let prometheus_port = 9091;
    let loki_port = 3100;

    info!(
        "Server configuration: Grafana({}), Prometheus({}), Loki({})",
        grafana_port, prometheus_port, loki_port
    );

    // Server configurations
    let grafana_config = GrafanaServerConfig {
        port: grafana_port,
        admin_user: "admin".to_string(),
        admin_password: "admin".to_string(),
        allow_anonymous: true,
        anonymous_role: "Admin".to_string(),
        container_name: "blueprint-test-grafana".to_string(),
        ..Default::default()
    };

    let prometheus_config = PrometheusServerConfig {
        port: prometheus_port,
        docker_container_name: "blueprint-test-prometheus".to_string(),
        ..Default::default()
    };

    let loki_config = LokiServerConfig {
        port: loki_port,
        container_name: "blueprint-test-loki".to_string(),
        ..Default::default()
    };

    // Configure metrics collection
    let metrics_config = MetricsConfig {
        service_id: 1001,
        blueprint_id: 2001,
        bind_address: format!("0.0.0.0:{}", prometheus_port),
        ..Default::default()
    };

    // Build QoS service with all components
    info!("Building QoS service with all monitoring components");
    let mut qos_service = QoSServiceBuilder::<MockHeartbeatConsumer>::new()
        .with_heartbeat_consumer(heartbeat_consumer)
        .with_metrics_config(metrics_config)
        .with_grafana_server_config(grafana_config)
        .with_prometheus_server_config(prometheus_config)
        .with_loki_server_config(loki_config)
        .manage_servers(true)
        .build()
        .await?;

    // Wait for servers to fully initialize
    info!("Waiting for servers to initialize (5 seconds)...");
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Log server URLs for user access
    info!("Server URLs:");
    if let Some(url) = qos_service.grafana_server_url() {
        info!("Grafana: {}", url);
    } else {
        warn!("Grafana server not initialized");
    }

    if let Some(url) = qos_service.loki_server_url() {
        info!("Loki: {}", url);
    } else {
        warn!("Loki server not initialized");
    }

    if let Some(url) = qos_service.prometheus_server_url() {
        info!("Prometheus: {}", url);
    } else {
        warn!("Prometheus server not initialized");
    }

    // Display detailed server status
    qos_service.debug_server_status();

    // Use the container name
    if qos_service.prometheus_server_url().is_some() {
        info!("Prometheus: http://blueprint-test-prometheus:9091");
    } else {
        info!("Prometheus server not initialized");
    }

    // Debug server status
    qos_service.debug_server_status();

    // Configure service and blueprint IDs for metrics
    let service_id = 1001;
    let blueprint_id = 2001;

    // Setup metrics registry
    let registry = Registry::new();

    // Helper function to create and register a gauge with labels
    let create_gauge = |name: &str, help: &str| -> Result<IntGauge, Box<dyn std::error::Error>> {
        let opts = Opts::new(name, help)
            .const_label("service_id", service_id.to_string())
            .const_label("blueprint_id", blueprint_id.to_string());

        let gauge = IntGauge::with_opts(opts)?;
        registry.register(Box::new(gauge.clone()))?;

        Ok(gauge)
    };

    // Create simulation metrics
    info!("Setting up simulated metrics");
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

    // Create and set up Grafana dashboard
    info!("Creating monitoring dashboard");
    if let Some(_grafana_url) = qos_service.grafana_server_url() {
        // Just use the built-in dashboard creation method instead
        match qos_service.create_dashboard("prometheus", "loki").await {
            Ok(Some(url)) => info!("Dashboard created successfully: {}", url),
            Ok(None) => info!("Dashboard creation skipped - Grafana client unavailable"),
            Err(e) => error!("Dashboard creation failed: {}", e),
        }
    } else {
        warn!("Cannot create dashboard - Grafana URL unavailable");
    }

    // Start background thread for metrics simulation
    let start_time = Instant::now();
    thread::spawn(move || {
        let mut execution_count = 0;
        let mut error_count = 0;

        loop {
            // Calculate elapsed time
            let seconds = start_time.elapsed().as_secs();
            let elapsed_secs = i64::try_from(seconds).unwrap_or(i64::MAX);

            // Update metrics with simulated values
            cpu_usage.set(elapsed_secs % 100); // CPU: 0-99%
            memory_usage.set(elapsed_secs % 1024); // Memory: 0-1023MB
            job_executions.set(execution_count); // Executions: increasing counter

            // Increment execution counter
            execution_count += 1;

            // Add error every 10 seconds
            if elapsed_secs % 10 == 0 {
                error_count += 1;
                job_errors.set(error_count);
            }

            // Update heartbeat timestamp
            heartbeat.set(elapsed_secs);

            // Wait before next update
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    info!("Metrics simulation active - data is being generated");

    // Display dashboard access information
    if let Some(url) = qos_service.grafana_server_url() {
        info!("Dashboard URL: {}/dashboards", url);
    }

    info!("QoS metrics demonstration running - press Ctrl+C to stop");

    // Wait for termination signal
    match signal::ctrl_c().await {
        Ok(()) => info!("Received shutdown signal, stopping services"),
        Err(e) => error!("Ctrl+C signal handler error: {}", e),
    }

    info!("QoS metrics demonstration completed");
    Ok(())
}
