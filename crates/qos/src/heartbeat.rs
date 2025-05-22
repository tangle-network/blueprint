use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tracing::info;

use crate::error::Result;

/// Configuration for the heartbeat service
#[derive(Clone, Debug)]
pub struct HeartbeatConfig {
    pub interval_secs: u64,

    pub jitter_percent: u8,

    pub service_id: u64,

    pub blueprint_id: u64,

    pub max_missed_heartbeats: u32,
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self {
            interval_secs: 300,
            jitter_percent: 10,
            service_id: 0,
            blueprint_id: 0,
            max_missed_heartbeats: 3,
        }
    }
}

/// Status information included in a heartbeat
#[derive(Clone, Debug)]
pub struct HeartbeatStatus {
    pub block_number: u64,

    pub timestamp: u64,

    pub service_id: u64,

    pub blueprint_id: u64,

    pub status_code: u32,

    pub status_message: Option<String>,
}

/// Trait for sending heartbeats to the chain
pub trait HeartbeatConsumer: Send + Sync + 'static {
    /// Send a heartbeat to the chain
    fn send_heartbeat(
        &self,
        status: &HeartbeatStatus,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// Service for sending heartbeats to the chain
pub struct HeartbeatService<C> {
    config: HeartbeatConfig,

    consumer: Arc<C>,

    last_heartbeat: Arc<Mutex<Option<HeartbeatStatus>>>,

    running: Arc<Mutex<bool>>,
}

impl<C> HeartbeatService<C>
where
    C: HeartbeatConsumer,
{
    /// Create a new heartbeat service
    pub fn new(config: HeartbeatConfig, consumer: Arc<C>) -> Self {
        Self {
            config,
            consumer,
            last_heartbeat: Arc::new(Mutex::new(None)),
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Get the last heartbeat status
    #[must_use]
    pub async fn last_heartbeat(&self) -> Option<HeartbeatStatus> {
        self.last_heartbeat.lock().await.clone()
    }

    /// Check if the service is running
    #[must_use]
    pub async fn is_running(&self) -> bool {
        *self.running.lock().await
    }

    /// Send a heartbeat to the chain
    ///
    /// # Errors
    /// Returns an error if the heartbeat cannot be sent to the consumer
    #[allow(dead_code)]
    async fn send_heartbeat(&self) -> Result<()> {
        let status = HeartbeatStatus {
            block_number: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            service_id: self.config.service_id,
            blueprint_id: self.config.blueprint_id,
            status_code: 0,
            status_message: None,
        };

        self.consumer.send_heartbeat(&status).await?;

        *self.last_heartbeat.lock().await = Some(status);

        // Log the heartbeat
        info!(
            service_id = self.config.service_id,
            blueprint_id = self.config.blueprint_id,
            "Sent heartbeat to chain"
        );

        Ok(())
    }
}

impl<C> Clone for HeartbeatService<C>
where
    C: HeartbeatConsumer,
{
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            consumer: self.consumer.clone(),
            last_heartbeat: self.last_heartbeat.clone(),
            running: self.running.clone(),
        }
    }
}
