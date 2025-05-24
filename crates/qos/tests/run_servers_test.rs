use blueprint_qos::{
    QoSServiceBuilder,
    servers::{
        grafana::GrafanaServerConfig,
        loki::LokiServerConfig,
    },
    heartbeat::{HeartbeatConsumer, HeartbeatStatus},
    error::Error as QosError,
    metrics::MetricsConfig,
};
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use tokio::time::Duration;
use blueprint_core::{info, error};
use blueprint_testing_utils::setup_log;
use prometheus::{Registry, IntGauge, Opts};
use prometheus::core::Collector;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

// Create a Prometheus datasource in Grafana using Basic Auth
async fn create_prometheus_datasource(grafana_url: &str, prometheus_url: &str) -> Result<String, Box<dyn std::error::Error>> {
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
    let datasource_json = format!(r#"{{
        "name": "prometheus",
        "type": "prometheus",
        "url": "{}",
        "access": "proxy",
        "basicAuth": false,
        "isDefault": true
    }}"#, prometheus_url);
    
    // Send the request to create the datasource
    let datasource_url = format!("{}/api/datasources", grafana_url);
    let response = client.post(&datasource_url)
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

// Custom dashboard creation function using Basic Auth
async fn create_dashboard_with_basic_auth(grafana_url: &str) -> Result<String, Box<dyn std::error::Error>> {
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
                                    { "color": "green", "value": null },
                                    { "color": "orange", "value": 70 },
                                    { "color": "red", "value": 90 }
                                ]
                            },
                            "max": 100
                        }
                    },
                    "targets": [
                        {
                            "expr": "test_blueprint_cpu_usage{service_id=\"1001\", blueprint_id=\"2001\"}",
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
                                    { "color": "green", "value": null },
                                    { "color": "orange", "value": 700 },
                                    { "color": "red", "value": 900 }
                                ]
                            },
                            "max": 1024
                        }
                    },
                    "targets": [
                        {
                            "expr": "test_blueprint_memory_usage{service_id=\"1001\", blueprint_id=\"2001\"}",
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
                        "h": 8
                    },
                    "targets": [
                        {
                            "expr": "test_blueprint_job_executions{service_id=\"1001\", blueprint_id=\"2001\"}",
                            "refId": "A"
                        }
                    ]
                },
                {
                    "id": 4,
                    "title": "Job Errors",
                    "type": "stat",
                    "gridPos": {
                        "x": 0,
                        "y": 8,
                        "w": 8,
                        "h": 8
                    },
                    "fieldConfig": {
                        "defaults": {
                            "mappings": [],
                            "thresholds": {
                                "mode": "absolute",
                                "steps": [
                                    { "color": "green", "value": null },
                                    { "color": "orange", "value": 5 },
                                    { "color": "red", "value": 10 }
                                ]
                            }
                        }
                    },
                    "targets": [
                        {
                            "expr": "test_blueprint_job_errors{service_id=\"1001\", blueprint_id=\"2001\"}",
                            "refId": "A"
                        }
                    ]
                },
                {
                    "id": 5,
                    "title": "Last Heartbeat",
                    "type": "stat",
                    "gridPos": {
                        "x": 8,
                        "y": 8,
                        "w": 8,
                        "h": 8
                    },
                    "targets": [
                        {
                            "expr": "test_blueprint_last_heartbeat{service_id=\"1001\", blueprint_id=\"2001\"}",
                            "refId": "A"
                        }
                    ]
                }
            ]
        },
        "overwrite": true,
        "message": "Blueprint test dashboard created"
    }"#;
    
    // Send the request to create the dashboard
    let dashboard_url = format!("{}/api/dashboards/db", grafana_url);
    let response = client.post(&dashboard_url)
        .headers(headers)
        .body(dashboard_json.to_string())
        .send()
        .await?;
    
    // Check if the request was successful
    if response.status().is_success() {
        let json = response.json::<serde_json::Value>().await?;
        let url = format!("{}{}", grafana_url, json["url"].as_str().unwrap_or("/"));
        Ok(url)
    } else {
        let status = response.status();
        let body = response.text().await?;
        Err(format!("Failed to create dashboard: Status {}: {}", status, body).into())
    }
}

// Mock HeartbeatConsumer for testing purposes
#[derive(Clone, Debug)]
struct MockHeartbeatConsumer;

impl HeartbeatConsumer for MockHeartbeatConsumer {
    async fn send_heartbeat(&self, _status: &HeartbeatStatus) -> Result<(), QosError> {
        Ok(())
    }
}

#[tokio::test]
#[ignore]
async fn test_run_all_servers() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    info!("Starting QoS servers test...");

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

    // Configure Prometheus server
    // Note: We need to use the full path since it's not directly exported from servers
    let prometheus_config = blueprint_qos::servers::prometheus::PrometheusServerConfig {
        port: 9090,
        host: "0.0.0.0".to_string(),
        use_docker: false, // Use embedded server instead of Docker to avoid container initialization issues
        docker_image: "prom/prometheus:latest".to_string(),
        docker_container_name: "blueprint-test-prometheus".to_string(),
        config_path: None,
        data_path: None,
    };

    // Set up metrics configuration with service_id and blueprint_id for dashboard
    let metrics_config = MetricsConfig {
        service_id: 1001,
        blueprint_id: 2001,
        ..Default::default()
    };

    // Create the QoS service with all servers enabled
    let consumer = Arc::new(MockHeartbeatConsumer);
    let mut qos_service = QoSServiceBuilder::new()
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
    
    // Create Prometheus datasource and custom dashboard using Basic Auth
    if let Some(grafana_url) = qos_service.grafana_server_url() {
        // First create the Prometheus datasource
        info!("Creating Prometheus datasource in Grafana...");
        let prometheus_url = "http://localhost:9090";
        match create_prometheus_datasource(&grafana_url, prometheus_url).await {
            Ok(datasource_name) => {
                info!("Prometheus datasource '{}' created successfully", datasource_name);
                
                // Then create the dashboard
                info!("Creating custom dashboard in Grafana...");
                match create_dashboard_with_basic_auth(&grafana_url).await {
                    Ok(url) => info!("Dashboard created successfully at {}", url),
                    Err(e) => error!("Failed to create dashboard: {}", e),
                }
            },
            Err(e) => error!("Failed to create Prometheus datasource: {}", e),
        }
    } else {
        info!("No dashboard created (Grafana server not available)");
    }

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

    // Debug server status
    qos_service.debug_server_status();
    
    // Set up Prometheus metrics for simulation
    info!("Setting up simulated metrics...");
    let registry = Registry::new();
    
    // Helper function to create and register a gauge with labels
    let create_gauge = |name: &str, help: &str| -> Result<IntGauge, Box<dyn std::error::Error>> {
        let service_id = 1001;
        let blueprint_id = 2001;
        
        let opts = Opts::new(name, help)
            .const_label("service_id", service_id.to_string())
            .const_label("blueprint_id", blueprint_id.to_string());
        
        let gauge = IntGauge::with_opts(opts)?;
        registry.register(Box::new(gauge.clone()))?;
        
        Ok(gauge)
    };
    
    // Create all the gauges
    let cpu_usage = match create_gauge(
        "test_blueprint_cpu_usage",
        "Simulated CPU usage for test blueprint"
    ) {
        Ok(gauge) => gauge,
        Err(e) => {
            error!("Failed to create CPU usage gauge: {}", e);
            return Err(e);
        }
    };
    
    let memory_usage = match create_gauge(
        "test_blueprint_memory_usage",
        "Simulated memory usage for test blueprint"
    ) {
        Ok(gauge) => gauge,
        Err(e) => {
            error!("Failed to create memory usage gauge: {}", e);
            return Err(e);
        }
    };
    
    let job_executions = match create_gauge(
        "test_blueprint_job_executions",
        "Simulated job executions for test blueprint"
    ) {
        Ok(gauge) => gauge,
        Err(e) => {
            error!("Failed to create job executions gauge: {}", e);
            return Err(e);
        }
    };
    
    let job_errors = match create_gauge(
        "test_blueprint_job_errors",
        "Simulated job errors for test blueprint"
    ) {
        Ok(gauge) => gauge,
        Err(e) => {
            error!("Failed to create job errors gauge: {}", e);
            return Err(e);
        }
    };
    
    let heartbeat = match create_gauge(
        "test_blueprint_last_heartbeat",
        "Simulated last heartbeat timestamp for test blueprint"
    ) {
        Ok(gauge) => gauge,
        Err(e) => {
            error!("Failed to create heartbeat gauge: {}", e);
            return Err(e);
        }
    };

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
    
    // Provide instructions for viewing the dashboard
    if let Some(url) = qos_service.grafana_server_url() {
        info!("You can view the dashboard at {}/dashboards", url);
    }
    
    // Sleep indefinitely to keep the servers running
    info!("All servers started. Sleeping indefinitely...");
    info!("Press Ctrl+C to stop the test.");
    
    // Sleep for a very long time (effectively indefinitely)
    tokio::time::sleep(Duration::from_secs(3600 * 24)).await; // 24 hours
    
    Ok(())
}