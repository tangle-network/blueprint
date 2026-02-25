#![allow(dead_code, clippy::unused_async)]

use std::sync::Arc;

use anyhow::{Context, Result};
use blueprint_qos::error::Error as QosError;
use blueprint_qos::heartbeat::{HeartbeatConsumer, HeartbeatStatus};
use blueprint_qos::proto::qos_metrics_client::QosMetricsClient;
use blueprint_tangle_extra::extract::{TangleArg, TangleResult};
use tokio::sync::Mutex;
use tonic::transport::Channel;

/// Square job ID exercised in the QoS integration tests.
pub const XSQUARE_JOB_ID: u8 = 0;

/// Minimal job handler used by the integration tests.
pub async fn square(TangleArg((value,)): TangleArg<(u64,)>) -> TangleResult<u64> {
    TangleResult(value * value)
}

/// Mock heartbeat consumer that records every heartbeat locally.
#[derive(Clone, Default)]
pub struct MockHeartbeatConsumer {
    heartbeats: Arc<Mutex<Vec<HeartbeatStatus>>>,
}

impl MockHeartbeatConsumer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub async fn heartbeat_count(&self) -> usize {
        self.heartbeats.lock().await.len()
    }

    #[must_use]
    pub async fn latest_heartbeat(&self) -> Option<HeartbeatStatus> {
        self.heartbeats.lock().await.last().cloned()
    }
}

impl HeartbeatConsumer for MockHeartbeatConsumer {
    fn send_heartbeat(
        &self,
        status: &HeartbeatStatus,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), QosError>> + Send>> {
        let status = status.clone();
        let heartbeats = Arc::clone(&self.heartbeats);

        Box::pin(async move {
            heartbeats.lock().await.push(status);
            Ok(())
        })
    }
}

/// Connects to the QoS metrics gRPC service exposed by the tests.
pub async fn connect_to_qos_metrics(addr: &str) -> Result<QosMetricsClient<Channel>> {
    let endpoint = format!("http://{addr}")
        .parse::<tonic::transport::Endpoint>()
        .context("invalid QoS metrics endpoint")?;

    let channel = endpoint
        .connect()
        .await
        .context("failed to connect to QoS metrics endpoint")?;
    Ok(QosMetricsClient::new(channel))
}
