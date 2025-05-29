use blueprint_core::info;
use blueprint_qos::{
    default_qos_config,
    error::Error as QosError,
    heartbeat::{HeartbeatConfig, HeartbeatConsumer, HeartbeatStatus},
    unified_service::QoSServiceBuilder,
};
use blueprint_testing_utils::setup_log;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone, Debug)]
struct MockHeartbeatConsumer {
    heartbeat_count: Arc<tokio::sync::Mutex<usize>>,
    last_status: Arc<tokio::sync::Mutex<Option<HeartbeatStatus>>>,
}

impl MockHeartbeatConsumer {
    fn new() -> Self {
        Self {
            heartbeat_count: Arc::new(tokio::sync::Mutex::new(0)),
            last_status: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }

    async fn heartbeat_count(&self) -> usize {
        *self.heartbeat_count.lock().await
    }
}

impl HeartbeatConsumer for MockHeartbeatConsumer {
    async fn send_heartbeat(&self, status: &HeartbeatStatus) -> std::result::Result<(), QosError> {
        let mut count = self.heartbeat_count.lock().await;
        *count += 1;

        let mut last = self.last_status.lock().await;
        *last = Some(status.clone());

        info!(
            "Heartbeat sent via consumer, count: {}, status: {:?}",
            *count, status
        );
        Ok(())
    }
}

const SERVICE_ID: u64 = 1; // Dummy service ID
const BLUEPRINT_ID: u64 = 1; // Dummy blueprint ID
const HEARTBEAT_TIMEOUT_SECS: u64 = 15;

#[tokio::test]
async fn test_qos_heartbeat_functionality() -> Result<(), QosError> {
    setup_log();
    info!("Starting QoS heartbeat functionality test");

    let heartbeat_consumer = Arc::new(MockHeartbeatConsumer::new());

    let mut config = default_qos_config();
    config.heartbeat = Some(HeartbeatConfig {
        interval_secs: 1, // Fast heartbeats for testing
        jitter_percent: 5,
        service_id: SERVICE_ID,
        blueprint_id: BLUEPRINT_ID,
        max_missed_heartbeats: 3,
    });

    let qos_service = QoSServiceBuilder::new()
        .with_config(config)
        .with_heartbeat_consumer(heartbeat_consumer.clone())
        .build()
        .await?;

    info!("QoS service initialized with heartbeat functionality");
    let qos_service = Arc::new(qos_service);

    if let Some(heartbeat_service) = &qos_service.heartbeat_service() {
        info!("Starting heartbeat service...");
        heartbeat_service.start_heartbeat().await?
    } else {
        panic!("Heartbeat service not found in QoS service");
    }

    info!("Waiting for heartbeats to be sent...");
    let mut heartbeats_received = false;

    for attempt in 1..=HEARTBEAT_TIMEOUT_SECS {
        sleep(Duration::from_secs(1)).await;

        let heartbeat_count = heartbeat_consumer.heartbeat_count().await;
        if heartbeat_count > 0 {
            info!(
                "Success: Received {} heartbeat(s) after {} seconds",
                heartbeat_count, attempt
            );
            heartbeats_received = true;
            break;
        }

        info!("Waiting for heartbeats... ({} seconds elapsed)", attempt);
    }

    assert!(
        heartbeats_received,
        "No heartbeats were received - heartbeat service failed to function"
    );

    info!("QoS heartbeat functionality test completed successfully");
    Ok(())
}
