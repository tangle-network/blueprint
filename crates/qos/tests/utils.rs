use blueprint_core::info;
use blueprint_qos::error::Error as QosError;
use blueprint_qos::heartbeat::{HeartbeatConsumer, HeartbeatStatus};
use blueprint_testing_utils::Error;

use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub const XSQUARE_JOB_ID: u8 = 1;

#[derive(Clone)]
pub struct TestContext {
    pub heartbeat_consumer: Arc<MockHeartbeatConsumer>,
}

/// Handle squaring a number in the integration test
/// This simulates a job handler function but doesn't use the Job trait
pub async fn square_job(ctx: &TestContext, input: u64) -> Result<u64, Error> {
    info!("Square job handler called with input: {}", input);

    // Track a heartbeat for the job execution (processing)
    ctx.heartbeat_consumer
        .send_heartbeat(&HeartbeatStatus {
            block_number: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            service_id: 1,
            blueprint_id: 1,
            status_code: 0, // 0 = processing
            status_message: Some(format!("Processing square job for input: {}", input)),
        })
        .await
        .map_err(|e| Error::Setup(format!("Failed to send heartbeat: {}", e)))?;

    // Calculate result
    let result = input * input;

    // Send completion heartbeat
    ctx.heartbeat_consumer
        .send_heartbeat(&HeartbeatStatus {
            block_number: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            service_id: 1,
            blueprint_id: 1,
            status_code: 1, // 1 = completed
            status_message: Some(format!("Completed square job, result: {}", result)),
        })
        .await
        .map_err(|e| Error::Setup(format!("Failed to send completion heartbeat: {}", e)))?;

    Ok(result)
}

/// Square job handler for QoS integration testing
/// Takes a number and returns its square while tracking heartbeats
/// through the QoS service
/// Mock implementation of the HeartbeatConsumer for testing
#[derive(Clone, Default)]
pub struct MockHeartbeatConsumer {
    pub heartbeats: Arc<Mutex<Vec<HeartbeatStatus>>>,
}

impl MockHeartbeatConsumer {
    pub fn new() -> Self {
        Self {
            heartbeats: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn heartbeat_count(&self) -> usize {
        self.heartbeats.lock().unwrap().len()
    }

    /// Gets a copy of all received heartbeat statuses
    pub fn get_heartbeats(&self) -> Vec<HeartbeatStatus> {
        self.heartbeats.lock().unwrap().clone()
    }
}

impl HeartbeatConsumer for MockHeartbeatConsumer {
    fn send_heartbeat(
        &self,
        status: &HeartbeatStatus,
    ) -> impl std::future::Future<Output = Result<(), QosError>> + Send {
        let status = status.clone();
        let heartbeats = self.heartbeats.clone();

        async move {
            heartbeats.lock().unwrap().push(status);
            Ok(())
        }
    }
}
