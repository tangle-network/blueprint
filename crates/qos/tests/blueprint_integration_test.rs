#[cfg(test)]
mod tests {
    use blueprint_qos::{
        QoSServiceBuilder, default_qos_config,
        error::Error as QosError,
        heartbeat::{HeartbeatConsumer, HeartbeatStatus},
    };
    use prometheus::core::Number;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
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

            println!("Heartbeat sent, count: {}, status: {:?}", *count, status);
            Ok(())
        }
    }

    const BASE_PORT: u16 = 9000;

    #[tokio::test]
    async fn test_qos_service_functionality() -> Result<(), QosError> {
        // Create a mock heartbeat consumer
        let heartbeat_consumer = Arc::new(MockHeartbeatConsumer::new());

        // Create a custom QoS configuration
        let mut qos_config = default_qos_config();

        // Update the configuration with test values
        println!("Setting up heartbeat configuration");
        if let Some(heartbeat_config) = &mut qos_config.heartbeat {
            heartbeat_config.service_id = 1;
            heartbeat_config.blueprint_id = 2;
            heartbeat_config.interval_secs = 1;
            println!(
                "Heartbeat interval set to {} seconds",
                heartbeat_config.interval_secs
            );
        } else {
            println!("WARNING: No heartbeat configuration found in default config");
            // Create heartbeat config if it doesn't exist
            qos_config.heartbeat = Some(blueprint_qos::heartbeat::HeartbeatConfig {
                service_id: 1,
                blueprint_id: 2,
                interval_secs: 1,
                jitter_percent: 10,
                max_missed_heartbeats: 3,
            });
            println!("Created new heartbeat configuration with 1 second interval");
        }

        if let Some(metrics_config) = &mut qos_config.metrics {
            metrics_config.service_id = 1;
            metrics_config.blueprint_id = 2;
            metrics_config.bind_address = format!("127.0.0.1:{}", BASE_PORT);
        }

        // Build the QoS service
        println!("Building QoS service with heartbeat consumer...");
        let qos_service_result = QoSServiceBuilder::new()
            .with_config(qos_config.clone())
            .with_heartbeat_consumer(heartbeat_consumer.clone())
            .build()
            .await;

        if let Err(ref e) = qos_service_result {
            println!("QoS service build failed: {:?}", e);
        } else {
            println!("QoS service built successfully");
        }

        assert!(
            qos_service_result.is_ok(),
            "QoS service build failed: {:?}",
            qos_service_result.err()
        );

        let mut qos_service = qos_service_result.unwrap();

        // Verify that the metrics provider is available
        assert!(
            qos_service.metrics_provider().is_some(),
            "Metrics provider should be available"
        );

        // Record some metrics
        qos_service.record_job_execution(1, 0.5);
        qos_service.record_job_execution(2, 1.2);
        qos_service.record_job_error(3, "test_error");

        // Allow more time for heartbeats to be sent and metrics to be collected
        println!("Waiting for heartbeats to be sent (this may take a few seconds)...");

        // TODO: Finish heartbeat portion

        println!("Waiting for metrics service to initialize...");
        sleep(Duration::from_secs(2)).await;

        let heartbeat_count = heartbeat_consumer.heartbeat_count();

        assert!(
            heartbeat_count > 0,
            "Heartbeat count should be greater than 0"
        );

        // Verify that the dashboard URL is available if Grafana is configured
        if qos_config.grafana.is_some() {
            // Create a dashboard if it doesn't exist yet
            if qos_service.dashboard_url().is_none() {
                let dashboard_result = qos_service.create_dashboard("prometheus", "loki").await;
                println!("Dashboard creation result: {:?}", dashboard_result);
            }

            // Check if we have a dashboard URL
            if let Some(url) = qos_service.dashboard_url() {
                println!("Grafana dashboard URL: {}", url);
            }
        }

        for i in 4..10 {
            qos_service.record_job_execution(i, i.into_f64() / 10.0);
        }

        // Allow time for the metrics to be collected
        sleep(Duration::from_secs(2)).await;

        Ok(())
    }
}
