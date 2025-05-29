#[cfg(test)]
mod tests {
    use blueprint_core::{info, warn};
    use blueprint_qos::{
        QoSServiceBuilder, default_qos_config,
        error::Error as QosError,
        heartbeat::{HeartbeatConsumer, HeartbeatStatus},
        servers::grafana::GrafanaServerConfig,
        servers::loki::LokiServerConfig,
    };
    use blueprint_testing_utils::setup_log;

    use reqwest::Client;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tempfile::TempDir;
    use tokio::time::sleep;

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
        #[allow(dead_code)]
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

    const BASE_PORT: u16 = 9000;

    /// Tests the core QoS service functionality including metrics, heartbeats, and configuration.
    ///
    /// This test verifies:
    /// - QoS service initialization with custom configuration
    /// - Heartbeat mechanism functionality
    /// - Metrics provider initialization and access
    #[tokio::test]
    async fn test_qos_service_functionality() -> Result<(), QosError> {
        setup_log();
        info!("Starting QoS service functionality test");

        // Create a mock heartbeat consumer
        let heartbeat_consumer = Arc::new(MockHeartbeatConsumer::new());

        // Create a custom QoS configuration
        let mut qos_config = default_qos_config();

        // Update the configuration with test values
        info!("Setting up heartbeat configuration");
        if let Some(heartbeat_config) = &mut qos_config.heartbeat {
            heartbeat_config.service_id = 1;
            heartbeat_config.blueprint_id = 2;
            heartbeat_config.interval_secs = 1;
            info!(
                "Heartbeat interval set to {} seconds",
                heartbeat_config.interval_secs
            );
        } else {
            warn!("No heartbeat configuration found in default config");
            // Create heartbeat config if it doesn't exist
            qos_config.heartbeat = Some(blueprint_qos::heartbeat::HeartbeatConfig {
                service_id: 1,
                blueprint_id: 2,
                interval_secs: 1,
                jitter_percent: 10,
                max_missed_heartbeats: 3,
            });
            info!("Created new heartbeat configuration with 1 second interval");
        }

        if let Some(metrics_config) = &mut qos_config.metrics {
            metrics_config.service_id = 1;
            metrics_config.blueprint_id = 2;
            metrics_config.bind_address = format!("127.0.0.1:{}", BASE_PORT);
        }

        // Build the QoS service
        info!("Building QoS service with heartbeat consumer");
        let qos_service_result = QoSServiceBuilder::new()
            .with_config(qos_config.clone())
            .with_heartbeat_consumer(heartbeat_consumer.clone())
            .build()
            .await;

        if let Err(ref e) = qos_service_result {
            warn!("QoS service build failed: {:?}", e);
        } else {
            info!("QoS service built successfully");
        }

        assert!(
            qos_service_result.is_ok(),
            "QoS service build failed: {:?}",
            qos_service_result.err()
        );

        let qos_service = qos_service_result.unwrap();

        // Verify that the metrics provider is available
        let metrics_provider = qos_service.metrics_provider();
        assert!(
            metrics_provider.is_some(),
            "Metrics provider should be available"
        );

        // Wait for some heartbeats to be sent
        info!("Waiting for heartbeats to be sent...");
        sleep(Duration::from_secs(5)).await;

        // Verify that heartbeats were sent
        let heartbeat_count = heartbeat_consumer.heartbeat_count();
        info!("Heartbeat count: {}", heartbeat_count);
        assert!(
            heartbeat_count > 0,
            "At least one heartbeat should have been sent"
        );

        // The QoS service will be dropped at the end of the test
        info!("Test completed");

        Ok(())
    }

    /// Tests the Grafana and Loki server management functionality.
    ///
    /// This test verifies:
    /// - Server initialization and startup
    /// - Server URL accessibility
    /// - Health check endpoints
    /// - Dashboard creation
    ///
    /// Note: This test requires Docker to be running on the host machine.
    #[tokio::test]
    async fn test_grafana_loki_server_management() -> Result<(), QosError> {
        setup_log();
        info!("Starting Grafana and Loki server management test");

        // Create a temporary directory for server data
        let temp_dir = TempDir::new()
            .map_err(|e| QosError::Other(format!("Failed to create temp dir: {}", e)))?;
        let temp_dir_path = temp_dir.path().to_string_lossy().to_string();
        info!(
            "Using temporary directory for Docker volumes: {}",
            temp_dir_path
        );

        // Create a mock heartbeat consumer
        let heartbeat_consumer = Arc::new(MockHeartbeatConsumer::new());

        // Create a custom QoS configuration
        let mut qos_config = default_qos_config();

        // Configure Grafana server
        let grafana_port = BASE_PORT + 1;
        let grafana_server_config = GrafanaServerConfig {
            port: grafana_port,
            container_name: "test-grafana".to_string(),
            data_dir: format!("{}/grafana", temp_dir_path),
            ..Default::default() // Use default values for other fields
        };

        // Create Loki server configuration
        let loki_server_config = LokiServerConfig {
            port: BASE_PORT + 2,
            data_dir: temp_dir.path().join("loki").to_string_lossy().to_string(),
            container_name: "test-loki-server".to_string(),
        };

        // Enable server management
        qos_config.manage_servers = true;
        qos_config.grafana_server = Some(grafana_server_config);
        qos_config.loki_server = Some(loki_server_config);

        // Log the configuration
        info!("Grafana server config: {:?}", qos_config.grafana_server);
        info!("Loki server config: {:?}", qos_config.loki_server);
        info!("manage_servers flag: {}", qos_config.manage_servers);

        // Build the QoS service
        info!("Building QoS service with server management enabled");
        let qos_service_result = QoSServiceBuilder::new()
            .with_config(qos_config.clone())
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

        // Check if server URLs are available through public methods
        info!(
            "Grafana server URL available: {}",
            qos_service.grafana_server_url().is_some()
        );
        info!(
            "Loki server URL available: {}",
            qos_service.loki_server_url().is_some()
        );

        // Log server status
        qos_service.debug_server_status();

        // Verify that the Grafana server URL is available
        let grafana_url = qos_service.grafana_server_url();
        assert!(
            grafana_url.is_some(),
            "Grafana server URL should be available"
        );
        info!("Grafana server URL: {}", grafana_url.as_ref().unwrap());

        // Verify that the Loki server URL is available
        let loki_url = qos_service.loki_server_url();
        assert!(loki_url.is_some(), "Loki server URL should be available");
        info!("Loki server URL: {}", loki_url.as_ref().unwrap());

        // Wait for servers to be fully initialized
        info!("Waiting for servers to be fully initialized");
        sleep(Duration::from_secs(3)).await;

        // Verify that the Grafana server is accessible
        let client = Client::new();
        let grafana_health_url = format!("{}/api/health", grafana_url.as_ref().unwrap());
        let grafana_response = client.get(&grafana_health_url).send().await;
        assert!(
            grafana_response.is_ok(),
            "Failed to connect to Grafana server"
        );

        let grafana_status = grafana_response.unwrap().status();
        assert!(
            grafana_status.is_success(),
            "Grafana server health check failed with status: {}",
            grafana_status
        );
        info!("Grafana server health check passed");

        // Verify that the Loki server is accessible
        let loki_ready_url = format!("{}/ready", loki_url.as_ref().unwrap());
        let loki_response = client.get(&loki_ready_url).send().await;
        assert!(loki_response.is_ok(), "Failed to connect to Loki server");

        let loki_status = loki_response.unwrap().status();
        assert!(
            loki_status.is_success(),
            "Loki server ready check failed with status: {}",
            loki_status
        );
        info!("Loki server ready check passed");

        // Create a Grafana dashboard
        let dashboard_result = qos_service.create_dashboard("prometheus", "loki").await;
        assert!(
            dashboard_result.is_ok(),
            "Dashboard creation failed: {:?}",
            dashboard_result.err()
        );
        info!("Dashboard creation successful: {:?}", dashboard_result);

        // QoS service will be dropped at the end of the test, stopping the servers
        info!("Test completed successfully, cleaning up servers");

        Ok(())
    }
}
