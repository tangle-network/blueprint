use blueprint_qos::servers::prometheus::{PrometheusServer, PrometheusServerConfig};
use blueprint_qos::servers::grafana::{GrafanaServer, GrafanaServerConfig};
use blueprint_qos::servers::loki::{LokiServer, LokiServerConfig};
use blueprint_qos::servers::ServerManager;
use blueprint_qos::logging::grafana::{Dashboard, Panel, DataSource, GridPos, Target, FieldConfig};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::time::Duration;
use blueprint_core::{info, error};
use blueprint_testing_utils::setup_log;
use prometheus::{Registry, IntGauge, Opts};
use std::thread;
use std::time::Instant;
use reqwest::{Client, header};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;

/// This test demonstrates running all three servers (Grafana, Loki, and Prometheus)
/// without using volume mounts, which can cause permission issues in some environments.
#[tokio::test]
#[ignore] // Ignore by default since this runs indefinitely
async fn test_all_servers() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_log();

    info!("Starting all servers test...");

    // Configure Prometheus server to use embedded mode
    let prometheus_config = PrometheusServerConfig {
        port: 9090,
        host: "0.0.0.0".to_string(),
        use_docker: false, // Use embedded server instead of Docker
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

    // Create the servers directly
    let prometheus_server = Arc::new(PrometheusServer::new(prometheus_config));
    let grafana_server = Arc::new(GrafanaServer::new(grafana_config));
    let loki_server = Arc::new(LokiServer::new(loki_config));
    
    // Start the Prometheus server (embedded mode)
    info!("Starting Prometheus server...");
    if let Err(e) = prometheus_server.start().await {
        error!("Failed to start Prometheus server: {}", e);
        return Err(e.into());
    } else {
        info!("Prometheus server started successfully at {}", prometheus_server.url());
    }
    
    // Start the Grafana server (Docker without volume mounts)
    info!("Starting Grafana server...");
    if let Err(e) = grafana_server.start().await {
        error!("Failed to start Grafana server: {}", e);
        return Err(e.into());
    } else {
        info!("Grafana server started successfully at {}", grafana_server.url());
    }
    
    // Start the Loki server (Docker without volume mounts)
    info!("Starting Loki server...");
    if let Err(e) = loki_server.start().await {
        error!("Failed to start Loki server: {}", e);
        return Err(e.into());
    } else {
        info!("Loki server started successfully at {}", loki_server.url());
    }
    
    // Print server URLs for easy access
    info!("Server URLs:");
    info!("Prometheus: {}", prometheus_server.url());
    info!("Grafana: {}", grafana_server.url());
    info!("Loki: {}", loki_server.url());

    // Wait a bit for servers to fully initialize
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Create a custom function to create a dashboard using Basic Auth
    async fn create_dashboard_with_basic_auth(
        grafana_url: &str, 
        username: &str, 
        password: &str, 
        dashboard: Dashboard, 
        message: &str
    ) -> std::result::Result<String, Box<dyn std::error::Error>> {
        info!("Creating dashboard in Grafana using Basic Auth...");
        
        // Create auth header using base64 encoding
        let auth = format!("{}:{}", username, password);
        let encoded_auth = BASE64_STANDARD.encode(auth);
        
        // Create a custom HTTP client with appropriate headers
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Basic {}", encoded_auth)).unwrap(),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        
        // Create the request body
        let request = serde_json::json!({
            "dashboard": dashboard,
            "folderUid": null,
            "message": message,
            "overwrite": true
        });
        
        // Send the request
        let url = format!("{}/api/dashboards/db", grafana_url);
        let response = client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| Box::<dyn std::error::Error>::from(format!("Failed to create dashboard: {}", e)))?;
            
        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Box::<dyn std::error::Error>::from(format!(
                "Failed to create dashboard: {}",
                error_text
            )));
        }
        
        let dashboard_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Box::<dyn std::error::Error>::from(format!("Failed to parse dashboard response: {}", e)))?;
            
        // Extract the URL from the response
        let url = dashboard_response["url"]
            .as_str()
            .unwrap_or("/dashboards")
            .to_string();
            
        let full_url = format!("{}{}", grafana_url, url);
        info!("Created dashboard: {}", full_url);
        
        Ok(full_url)
    }

    // Wait a bit longer for Grafana to fully initialize
    info!("Waiting for Grafana to fully initialize...");
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Create a custom dashboard that simulates metrics from a running blueprint
    info!("Creating custom dashboard for blueprint metrics...");
    let service_id = 1001;
    let blueprint_id = 2001;
    
    // Create a dashboard for the Blueprint
    let mut dashboard = Dashboard {
        id: None,
        uid: Some(format!("test-blueprint-{}-{}", service_id, blueprint_id)),
        title: format!("Test Blueprint Service {} - {}", service_id, blueprint_id),
        tags: vec!["blueprint".to_string(), "test".to_string(), "tangle".to_string()],
        timezone: "browser".to_string(),
        refresh: Some("5s".to_string()),
        schema_version: 36,
        version: None,
        panels: Vec::new(),
    };

    // Add system metrics panel
    let system_metrics_panel = Panel {
        id: Some(1),
        title: "System Metrics".to_string(),
        panel_type: "timeseries".to_string(),
        datasource: Some(DataSource {
            ds_type: "prometheus".to_string(),
            uid: "prometheus".to_string(), // Default datasource name
        }),
        grid_pos: GridPos {
            x: 0,
            y: 0,
            w: 12,
            h: 8,
        },
        targets: vec![
            Target {
                ref_id: "A".to_string(),
                expr: format!(
                    "test_blueprint_cpu_usage{{service_id=\"{}\",blueprint_id=\"{}\"}}",
                    service_id, blueprint_id
                ),
                datasource: None,
            },
            Target {
                ref_id: "B".to_string(),
                expr: format!(
                    "test_blueprint_memory_usage{{service_id=\"{}\",blueprint_id=\"{}\"}}",
                    service_id, blueprint_id
                ),
                datasource: None,
            },
        ],
        options: HashMap::new(),
        field_config: FieldConfig::default(),
    };

    // Add job metrics panel
    let job_metrics_panel = Panel {
        id: Some(2),
        title: "Job Executions".to_string(),
        panel_type: "timeseries".to_string(),
        datasource: Some(DataSource {
            ds_type: "prometheus".to_string(),
            uid: "prometheus".to_string(),
        }),
        grid_pos: GridPos {
            x: 12,
            y: 0,
            w: 12,
            h: 8,
        },
        targets: vec![
            Target {
                ref_id: "A".to_string(),
                expr: format!(
                    "test_blueprint_job_executions{{service_id=\"{}\",blueprint_id=\"{}\"}}",
                    service_id, blueprint_id
                ),
                datasource: None,
            },
            Target {
                ref_id: "B".to_string(),
                expr: format!(
                    "test_blueprint_job_errors{{service_id=\"{}\",blueprint_id=\"{}\"}}",
                    service_id, blueprint_id
                ),
                datasource: None,
            },
        ],
        options: HashMap::new(),
        field_config: FieldConfig::default(),
    };

    // Add logs panel
    let logs_panel = Panel {
        id: Some(3),
        title: "Logs".to_string(),
        panel_type: "logs".to_string(),
        datasource: Some(DataSource {
            ds_type: "loki".to_string(),
            uid: "loki".to_string(),
        }),
        grid_pos: GridPos {
            x: 0,
            y: 8,
            w: 24,
            h: 8,
        },
        targets: vec![Target {
            ref_id: "A".to_string(),
            expr: format!(
                "{{service=\"test-blueprint\",service_id=\"{}\",blueprint_id=\"{}\"}}",
                service_id, blueprint_id
            ),
            datasource: None,
        }],
        options: HashMap::new(),
        field_config: FieldConfig::default(),
    };

    // Add heartbeat panel
    let heartbeat_panel = Panel {
        id: Some(4),
        title: "Heartbeats".to_string(),
        panel_type: "stat".to_string(),
        datasource: Some(DataSource {
            ds_type: "prometheus".to_string(),
            uid: "prometheus".to_string(),
        }),
        grid_pos: GridPos {
            x: 0,
            y: 16,
            w: 8,
            h: 4,
        },
        targets: vec![Target {
            ref_id: "A".to_string(),
            expr: format!(
                "test_blueprint_last_heartbeat{{service_id=\"{}\",blueprint_id=\"{}\"}}",
                service_id, blueprint_id
            ),
            datasource: None,
        }],
        options: HashMap::new(),
        field_config: FieldConfig::default(),
    };

    // Add all panels to the dashboard
    dashboard.panels.push(system_metrics_panel);
    dashboard.panels.push(job_metrics_panel);
    dashboard.panels.push(logs_panel);
    dashboard.panels.push(heartbeat_panel);

    // Create the dashboard in Grafana using Basic Auth
    match create_dashboard_with_basic_auth(
        &grafana_server.url(),
        "admin",
        "admin",
        dashboard,
        "Test dashboard for blueprint metrics"
    ).await {
        Ok(url) => info!("Dashboard created successfully at {}", url),
        Err(e) => error!("Failed to create dashboard: {}", e),
    }

    // Set up Prometheus metrics for simulation
    info!("Setting up simulated metrics...");
    let registry = Registry::new();
    
    // Helper function to create and register a gauge with labels
    let create_gauge = |name: &str, help: &str| -> Result<IntGauge, Box<dyn std::error::Error>> {
        let opts = Opts::new(name, help)
            .const_label("service_id", service_id.to_string())
            .const_label("blueprint_id", blueprint_id.to_string());
        
        let gauge = IntGauge::with_opts(opts)
            .map_err(|e| Box::<dyn std::error::Error>::from(e))?;
            
        registry.register(Box::new(gauge.clone()))
            .map_err(|e| Box::<dyn std::error::Error>::from(e))?;
            
        Ok(gauge)
    };
    
    // Create all the gauges
    let cpu_usage = create_gauge(
        "test_blueprint_cpu_usage",
        "Simulated CPU usage for test blueprint"
    )?;
    
    let memory_usage = create_gauge(
        "test_blueprint_memory_usage",
        "Simulated memory usage for test blueprint"
    )?;
    
    let job_executions = create_gauge(
        "test_blueprint_job_executions",
        "Simulated job executions for test blueprint"
    )?;
    
    let job_errors = create_gauge(
        "test_blueprint_job_errors",
        "Simulated job errors for test blueprint"
    )?;
    
    let heartbeat = create_gauge(
        "test_blueprint_last_heartbeat",
        "Simulated last heartbeat timestamp for test blueprint"
    )?;

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
    info!("You can view the dashboard at {}/dashboards", grafana_server.url());
    info!("All servers started. Sleeping indefinitely...");
    info!("Press Ctrl+C to stop the test.");
    
    // Sleep for a very long time (effectively indefinitely)
    tokio::time::sleep(Duration::from_secs(3600 * 24)).await; // 24 hours
    
    Ok(())
}
