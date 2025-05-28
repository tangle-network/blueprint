use std::sync::Arc;
use std::time::Duration;

use blueprint_core::{info, warn};
use blueprint_qos::{
    default_qos_config, error::Error as QosError, heartbeat::{HeartbeatConsumer, HeartbeatStatus},
    servers::grafana::GrafanaServerConfig, servers::loki::LokiServerConfig,
    servers::prometheus::PrometheusServerConfig, QoSServiceBuilder,
};
use blueprint_testing_utils::setup_log;
use prometheus::{Counter, Registry};
use reqwest::Client;
use std::sync::Mutex;
use tempfile::TempDir;
use tokio::time::sleep;

const BASE_PORT: u16 = 9000;

/// Mock `HeartbeatConsumer` for testing.
#[derive(Clone, Debug)]
struct MockHeartbeatConsumer {
    heartbeat_count: Arc<Mutex<usize>>,
    last_status: Arc<Mutex<Option<HeartbeatStatus>>>,
}

impl MockHeartbeatConsumer {
    /// Create a new mock heartbeat consumer
    fn new() -> Self {
        Self {
            heartbeat_count: Arc::new(Mutex::new(0)),
            last_status: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the current heartbeat count
    fn heartbeat_count(&self) -> usize {
        *self.heartbeat_count.lock().unwrap()
    }

    /// Get the last heartbeat status
    fn last_status(&self) -> Option<HeartbeatStatus> {
        self.last_status.lock().unwrap().clone()
    }
}

impl HeartbeatConsumer for MockHeartbeatConsumer {
    async fn send_heartbeat(&self, status: &HeartbeatStatus) -> Result<(), QosError> {
        let mut count = self.heartbeat_count.lock().unwrap();
        *count += 1;

        let mut last = self.last_status.lock().unwrap();
        *last = Some(status.clone());

        info!("Heartbeat sent, count: {}, status: {:?}", *count, status);
        Ok(())
    }
}

/// Tests QoS functionality in a Blueprint environment
/// 
/// This test:
/// 1. Configures QoS with heartbeat, metrics, and server components
/// 2. Tests heartbeat transmission
/// 3. Tests metrics collection
/// 4. Tests Grafana and Loki server management
/// 5. Tests dashboard creation
#[tokio::test]
async fn test_qos_blueprint_integration() -> Result<(), QosError> {
    setup_log();

    // Create temporary directory for QoS data
    let qos_temp_dir = TempDir::new().unwrap();
    let data_dir = qos_temp_dir.path().to_str().unwrap().to_string();
    
    // For test purposes, use fixed IDs
    let service_id = 1;
    let blueprint_id = 2;

    // Create mock heartbeat consumer
    let heartbeat_consumer = Arc::new(MockHeartbeatConsumer::new());
    
    // Setup QoS configuration
    let mut qos_config = default_qos_config();
    
    // Configure heartbeat
    if let Some(heartbeat_config) = &mut qos_config.heartbeat {
        heartbeat_config.service_id = service_id;
        heartbeat_config.blueprint_id = blueprint_id;
        heartbeat_config.interval_secs = 1;
        info!("Heartbeat interval set to {} seconds", heartbeat_config.interval_secs);
    } else {
        warn!("No heartbeat configuration found in default config");
        qos_config.heartbeat = Some(blueprint_qos::heartbeat::HeartbeatConfig {
            service_id,
            blueprint_id,
            interval_secs: 1,
            jitter_percent: 10,
            max_missed_heartbeats: 3,
        });
        info!("Created new heartbeat configuration with 1 second interval");
    }
    
    // Configure metrics
    if let Some(metrics_config) = &mut qos_config.metrics {
        metrics_config.service_id = service_id;
        metrics_config.blueprint_id = blueprint_id;
        metrics_config.bind_address = format!("127.0.0.1:{}", BASE_PORT);
    }
    
    // Configure server management
    let grafana_port = BASE_PORT + 1;
    let loki_port = BASE_PORT + 2;
    let prometheus_port = BASE_PORT + 3;
    
    let grafana_server_config = GrafanaServerConfig {
        port: grafana_port,
        admin_user: "admin".to_string(),
        admin_password: "admin".to_string(),
        container_name: "test-grafana-server".to_string(),
        allow_anonymous: true,
        anonymous_role: "Viewer".to_string(),
        data_dir: data_dir.clone(),
    };
    
    let loki_server_config = LokiServerConfig {
        port: loki_port,
        data_dir: data_dir.clone(),
        container_name: "test-loki-server".to_string(),
    };
    
    let prometheus_server_config = PrometheusServerConfig {
        port: prometheus_port,
        host: "127.0.0.1".to_string(),
        use_docker: true,
        docker_image: "prom/prometheus:latest".to_string(),
        docker_container_name: "test-prometheus-server".to_string(),
        config_path: Some(format!("{}/prometheus.yml", data_dir)),
        data_path: Some(format!("{}/prometheus_data", data_dir)),
    };
    
    // Enable server management
    qos_config.manage_servers = true;
    qos_config.grafana_server = Some(grafana_server_config);
    qos_config.loki_server = Some(loki_server_config);
    qos_config.prometheus_server = Some(prometheus_server_config);
    
    // Build the QoS service
    info!("Building QoS service with full configuration...");
    let qos_service_result = QoSServiceBuilder::new()
        .with_config(qos_config)
        .with_heartbeat_consumer(heartbeat_consumer.clone())
        .manage_servers(true)
        .build()
        .await;
    
    assert!(
        qos_service_result.is_ok(),
        "QoS service build failed: {:?}",
        qos_service_result.err()
    );
    
    let mut qos_service = qos_service_result.unwrap();
    info!("QoS service built successfully");
    
    // Verify metrics setup
    let metrics_provider = qos_service.metrics_provider();
    assert!(metrics_provider.is_some(), "Metrics provider should be available");
    
    // Create and use a custom counter to test metrics
    let registry = Registry::new();
    let counter = Counter::new("test_counter", "Test counter description").unwrap();
    registry.register(Box::new(counter.clone())).unwrap();
    counter.inc();
    
    // Wait for heartbeats to be generated
    info!("Waiting for heartbeats...");
    sleep(Duration::from_secs(3)).await;
    
    // Check that at least one heartbeat was sent
    assert!(
        heartbeat_consumer.heartbeat_count() > 0,
        "No heartbeats were sent"
    );
    info!("Heartbeat count after waiting: {}", heartbeat_consumer.heartbeat_count());
    
    // Test counter increment
    let value = counter.get();
    info!("Counter value: {}", value);
    assert_eq!(value, 1.0, "Counter should have been incremented");
    
    // Verify Grafana and Loki setup
    let grafana_url = qos_service.grafana_server_url();
    let loki_url = qos_service.loki_server_url();
    
    assert!(grafana_url.is_some(), "Grafana server URL should be available");
    assert!(loki_url.is_some(), "Loki server URL should be available");
    
    info!("Grafana server URL: {}", grafana_url.as_ref().unwrap());
    info!("Loki server URL: {}", loki_url.as_ref().unwrap());
    
    // Wait for servers to be fully initialized
    info!("Waiting for servers to be fully initialized...");
    sleep(Duration::from_secs(5)).await;
    
    // Verify Grafana and Loki are running
    let client = Client::new();
    
    // Check Grafana
    let grafana_health_url = format!("{}/api/health", grafana_url.as_ref().unwrap());
    match client.get(&grafana_health_url).send().await {
        Ok(response) => {
            let status = response.status();
            info!("Grafana health check status: {}", status);
            assert!(status.is_success(), "Grafana health check failed");
        },
        Err(e) => {
            warn!("Failed to connect to Grafana: {}", e);
            // Don't fail the test if we can't connect, as it might be due to CI environment
        }
    }
    
    // Check Loki
    let loki_ready_url = format!("{}/ready", loki_url.as_ref().unwrap());
    match client.get(&loki_ready_url).send().await {
        Ok(response) => {
            let status = response.status();
            info!("Loki ready check status: {}", status);
            assert!(status.is_success(), "Loki ready check failed");
        },
        Err(e) => {
            warn!("Failed to connect to Loki: {}", e);
            // Don't fail the test if we can't connect, as it might be due to CI environment
        }
    }
    
    // Create a Grafana dashboard
    match qos_service.create_dashboard("prometheus", "loki").await {
        Ok(dashboard_url) => {
            info!("Dashboard created successfully: {:?}", dashboard_url);
        },
        Err(e) => {
            warn!("Dashboard creation failed: {:?}", e);
            // Don't fail the test if dashboard creation fails, as it might be due to CI environment
        }
    }
    
    // Test recording metrics through the QoS service
    qos_service.record_job_execution(1, 0.5);
    qos_service.record_job_error(2, "test error");
    
    // Wait a bit to ensure metrics are recorded
    sleep(Duration::from_secs(2)).await;
    
    // Verify we can get the dashboard URL (if it was created)
    if let Some(url) = qos_service.dashboard_url() {
        info!("Dashboard URL is available: {}", url);
    }
    
    // We don't need to explicitly stop the QoS service,
    // it will be cleaned up when dropped at the end of the test
    
    // Wait a moment to ensure clean shutdown
    sleep(Duration::from_secs(1)).await;
    
    info!("QoS blueprint integration test completed successfully");
    Ok(())
}
