#![allow(dead_code, clippy::unused_async)]

use blueprint_qos::error::Error as QosError;
use blueprint_qos::heartbeat::{HeartbeatConsumer, HeartbeatStatus};
use blueprint_qos::proto::qos_metrics_client::QosMetricsClient;
use blueprint_std::pin::Pin;
use blueprint_std::sync::{Arc, Mutex};
use blueprint_tangle_extra::extract::{TangleArg, TangleResult};
use blueprint_testing_utils::Error;
use tonic::transport::Channel;

// Square job ID
pub const XSQUARE_JOB_ID: u8 = 0;

/// A copy of the `square` function from the `incredible-squaring` crate used for testing
pub async fn square(TangleArg(x): TangleArg<u64>) -> TangleResult<u64> {
    let result = x * x;

    // The result is then converted into a `JobResult` to be sent back to the caller.
    TangleResult(result)
}

/// Mock implementation of the `HeartbeatConsumer` for testing
#[derive(Clone, Default)]
pub struct MockHeartbeatConsumer {
    pub heartbeats: Arc<Mutex<Vec<HeartbeatStatus>>>,
}

impl MockHeartbeatConsumer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            heartbeats: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns the number of heartbeats received
    ///
    /// # Panics
    ///
    /// Panics if the heartbeats mutex is poisoned
    #[must_use]
    pub fn heartbeat_count(&self) -> usize {
        self.heartbeats.lock().unwrap().len()
    }

    /// Gets a copy of all received heartbeat statuses
    ///
    /// # Panics
    ///
    /// Panics if the heartbeats mutex is poisoned
    #[must_use]
    pub fn get_heartbeats(&self) -> Vec<HeartbeatStatus> {
        self.heartbeats.lock().unwrap().clone()
    }
}

impl HeartbeatConsumer for MockHeartbeatConsumer {
    fn send_heartbeat(
        &self,
        status: &HeartbeatStatus,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), QosError>> + Send>> {
        let status = status.clone();
        let heartbeats = self.heartbeats.clone();

        Box::pin(async move {
            heartbeats.lock().unwrap().push(status);
            Ok(())
        })
    }
}

/// Connect to the `QoS` metrics gRPC service
///
/// # Errors
///
/// Returns an error if connection to the gRPC service fails
pub async fn connect_to_qos_metrics(addr: &str) -> Result<QosMetricsClient<Channel>, Error> {
    let endpoint = tonic::transport::Endpoint::new(format!("http://{}", addr))
        .map_err(|e| Error::Setup(format!("Failed to create endpoint: {}", e)))?;
    let channel = endpoint
        .connect()
        .await
        .map_err(|e| Error::Setup(format!("Failed to connect to endpoint: {}", e)))?;
    Ok(QosMetricsClient::new(channel))
}
