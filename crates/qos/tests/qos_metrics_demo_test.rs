use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use blueprint_core::{error, info};
use blueprint_qos::{
    QoSServiceBuilder,
    error::Error as QosError,
    heartbeat::{HeartbeatConsumer, HeartbeatStatus},
    metrics::MetricsConfig,
    servers::{
        grafana::GrafanaServerConfig, loki::LokiServerConfig, prometheus::PrometheusServerConfig,
    },
};
use blueprint_testing_utils::setup_log;
use prometheus::{IntGauge, Opts, Registry};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use tokio::time::Duration;
use tokio::signal;

/// Create a Prometheus datasource in Grafana using Basic Auth
async fn create_prometheus_datasource(
    grafana_url: &str,
    prometheus_url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Create a reqwest client
    let client = reqwest::Client::new();

    // Set up Basic Auth headers with admin:admin credentials
    let auth_string = format!("{}:{}", "admin", "admin");
    let encoded_auth = BASE64.encode(auth_string);
    let auth_header = format!("Basic {}", encoded_auth);

    // Set up headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_header)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Create datasource JSON
    let datasource_json = format!(
        r#"{{
        "name": "prometheus",
        "type": "prometheus",
        "url": "{}",
        "access": "proxy",
        "basicAuth": false,
        "isDefault": true
    }}"#,
        prometheus_url
    );

    // Send the request to create the datasource
    let datasource_url = format!("{}/api/datasources", grafana_url);
    let response = client
        .post(&datasource_url)
        .headers(headers)
        .body(datasource_json)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        Ok("prometheus".to_string())
    } else {
        let status = response.status();
        let body = response.text().await?;

        // If the datasource already exists, that's fine
        if body.contains("data source with the same name already exists") {
            Ok("prometheus".to_string())
        } else {
            Err(format!("Failed to create datasource: Status {}: {}", status, body).into())
        }
    }
}

/// Create a simple dashboard in Grafana with CPU and Memory metrics
async fn create_dashboard_with_basic_auth(
    grafana_url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Create a reqwest client
    let client = reqwest::Client::new();

    // Set up Basic Auth headers with admin:admin credentials
    let auth_string = format!("{}:{}", "admin", "admin");
    let encoded_auth = BASE64.encode(auth_string);
    let auth_header = format!("Basic {}", encoded_auth);

    // Set up headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_header)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Create a simple dashboard JSON
    let dashboard_json = r#"{
        "dashboard": {
            "id": null,
            "uid": "blueprint-test-dashboard",
            "title": "Blueprint Test Dashboard",
            "tags": ["blueprint", "test"],
            "timezone": "browser",
            "schemaVersion": 16,
            "version": 0,
            "refresh": "10s",
            "panels": [
                {
                    "id": 1,
                    "title": "CPU Usage",
                    "type": "gauge",
                    "gridPos": {
                        "x": 0,
                        "y": 0,
                        "w": 8,
                        "h": 8
                    },
                    "fieldConfig": {
                        "defaults": {
                            "mappings": [],
                            "thresholds": {
                                "mode": "absolute",
                                "steps": [
                                    {
                                        "color": "green",
                                        "value": null
                                    },
                                    {
                                        "color": "orange",
                                        "value": 70
                                    },
                                    {
                                        "color": "red",
                                        "value": 85
                                    }
                                ]
                            },
                            "min": 0,
                            "max": 100,
                            "unit": "percent"
                        }
                    },
                    "targets": [
                        {
                            "expr": "test_blueprint_cpu_usage",
                            "refId": "A"
                        }
                    ]
                },
                {
                    "id": 2,
                    "title": "Memory Usage",
                    "type": "gauge",
                    "gridPos": {
                        "x": 8,
                        "y": 0,
                        "w": 8,
                        "h": 8
                    },
                    "fieldConfig": {
                        "defaults": {
                            "mappings": [],
                            "thresholds": {
                                "mode": "absolute",
                                "steps": [
                                    {
                                        "color": "green",
                                        "value": null
                                    },
                                    {
                                        "color": "orange",
                                        "value": 768
                                    },
                                    {
                                        "color": "red",
                                        "value": 896
                                    }
                                ]
                            },
                            "min": 0,
                            "max": 1024,
                            "unit": "MB"
                        }
                    },
                    "targets": [
                        {
                            "expr": "test_blueprint_memory_usage",
                            "refId": "A"
                        }
                    ]
                },
                {
                    "id": 3,
                    "title": "Job Executions",
                    "type": "stat",
                    "gridPos": {
                        "x": 16,
                        "y": 0,
                        "w": 8,
                        "h": 4
                    },
                    "targets": [
                        {
                            "expr": "test_blueprint_job_executions",
                            "refId": "A"
                        }
                    ]
                },
                {
                    "id": 4,
                    "title": "Job Errors",
                    "type": "stat",
                    "gridPos": {
                        "x": 16,
                        "y": 4,
                        "w": 8,
                        "h": 4
                    },
                    "fieldConfig": {
                        "defaults": {
                            "color": {
                                "mode": "thresholds"
                            },
                            "thresholds": {
                                "mode": "absolute",
                                "steps": [
                                    {
                                        "color": "green",
                                        "value": null
                                    },
                                    {
                                        "color": "orange",
                                        "value": 1
                                    },
                                    {
                                        "color": "red",
                                        "value": 5
                                    }
                                ]
                            }
                        }
                    },
                    "targets": [
                        {
                            "expr": "test_blueprint_job_errors",
                            "refId": "A"
                        }
                    ]
                }
            ]
        },
        "folderId": 0,
        "overwrite": true
    }"#;

    // Send the request to create the dashboard
    let dashboard_url = format!("{}/api/dashboards/db", grafana_url);
    let response = client
        .post(&dashboard_url)
        .headers(headers)
        .body(dashboard_json)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let url = json["url"].as_str().unwrap_or("");
        Ok(format!("{}{}", grafana_url, url))
    } else {
        let status = response.status();
        let body = response.text().await?;
        Err(format!("Failed to create dashboard: Status {}: {}", status, body).into())
    }
}

/// Mock HeartbeatConsumer for testing purposes
#[derive(Clone, Debug)]
struct MockHeartbeatConsumer;

impl HeartbeatConsumer for MockHeartbeatConsumer {
    async fn send_heartbeat(&self, _status: &HeartbeatStatus) -> Result<(), QosError> {
        Ok(())
    }
}

/// This test demonstrates running all three servers (Grafana, Loki, and Prometheus)
/// and showcases the metrics collection, visualization, and monitoring capabilities.
/// 
/// It runs a complete QoS metrics setup with:
/// - Prometheus server for metrics collection
/// - Grafana server for visualization
/// - Loki server for logs
/// - Simulated metrics data generation
/// - Dashboard creation and setup
/// 
/// This test is designed to be run manually to demo the QoS features.
/// It will wait for a Ctrl+C signal to terminate.
#[tokio::test]
#[ignore] // Ignore by default since this is a demo that runs until manually stopped
async fn test_qos_metrics_demo() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_log();

    info!("Starting QoS metrics demonstration...");

    // Configure Prometheus server to use Docker mode for full API support
    let prometheus_config = PrometheusServerConfig {
        port: 9091,
        host: "0.0.0.0".to_string(),
        use_docker: true,
        docker_image: "prom/prometheus:latest".to_string(),
        docker_container_name: "blueprint-test-prometheus".to_string(),
        config_path: None,
        data_path: None,
    };

    // Configure Grafana server
    let grafana_config = GrafanaServerConfig {
        port: 3000,
        admin_user: "admin".to_string(),
        admin_password: "admin".to_string(),
        allow_anonymous: true,
        anonymous_role: "Admin".to_string(),
        data_dir: "/var/lib/grafana".to_string(),
        container_name: "blueprint-test-grafana".to_string(),
    };

    // Configure Loki server
    let loki_config = LokiServerConfig {
        port: 3100,
        data_dir: "/var/lib/loki".to_string(),
        container_name: "blueprint-test-loki".to_string(),
    };

    // Configure metrics
    let metrics_config = MetricsConfig::default();

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
    info!("Waiting for servers to initialize (5 seconds)...");
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

    // For Docker-based Prometheus, use the container name
    if qos_service.prometheus_server_url().is_some() {
        info!("Prometheus: http://blueprint-test-prometheus:9091");
    } else {
        info!("Prometheus server not initialized");
    }

    // Debug server status
    qos_service.debug_server_status();

    // Set up service and blueprint IDs for metrics
    let service_id = 1001;
    let blueprint_id = 2001;

    // Create a Prometheus registry and register test metrics
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

    // Create dashboard using either the built-in method or our custom function
    info!("Creating dashboard...");
    if let Some(grafana_url) = qos_service.grafana_server_url() {
        let dashboard_result = create_dashboard_with_basic_auth(&grafana_url).await;
        match dashboard_result {
            Ok(url) => info!("Dashboard created successfully. View at: {}", url),
            Err(e) => error!("Failed to create dashboard manually: {}", e),
        }
    }

    // Start a background task to update metrics
    let start_time = Instant::now();
    thread::spawn(move || {
        let mut execution_count = 0;
        let mut error_count = 0;

        loop {
            // Update metrics with simulated values
            let seconds = start_time.elapsed().as_secs();
            let elapsed_secs = i64::try_from(seconds).unwrap_or(i64::MAX);

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

    info!("QoS metrics demonstration is now running.");
    info!("Press Ctrl+C to stop the demonstration.");

    // Wait for Ctrl+C signal
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Received Ctrl+C, shutting down...");
        }
        Err(e) => {
            error!("Failed to listen for Ctrl+C: {}", e);
        }
    }

    info!("Demonstration completed, servers will be stopped.");
    Ok(())
}
