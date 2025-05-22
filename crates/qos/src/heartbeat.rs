use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use blueprint_core::{info, warn};
use rand::Rng;

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

    /// Handle to the background task that sends heartbeats
    task_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl<C> HeartbeatService<C> {
    /// Create a new heartbeat service
    pub fn new(config: HeartbeatConfig, consumer: Arc<C>) -> Self {
        Self {
            config,
            consumer,
            last_heartbeat: Arc::new(Mutex::new(None)),
            running: Arc::new(Mutex::new(false)),
            task_handle: Arc::new(Mutex::new(None)),
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
    async fn send_heartbeat(&self) -> Result<()>
    where
        C: HeartbeatConsumer,
    {
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

    /// Start sending heartbeats at the configured interval
    ///
    /// # Errors
    /// Returns an error if the service is already running
    pub async fn start_heartbeat(&self) -> Result<()>
    where
        C: HeartbeatConsumer,
    {
        let mut running = self.running.lock().await;
        if *running {
            return Err(crate::error::Error::Other(
                "Heartbeat service is already running".to_string(),
            ));
        }

        *running = true;
        drop(running);

        let service = self.clone();
        let interval_secs = self.config.interval_secs;
        let jitter_percent = self.config.jitter_percent;

        let handle = tokio::spawn(async move {
            loop {
                if let Err(e) = service.send_heartbeat().await {
                    warn!("Failed to send heartbeat: {}", e);
                }
                let sleep_duration = if jitter_percent > 0 {
                    let interval_ms = interval_secs * 1000;
                    let max_jitter_ms = (interval_ms * u64::from(jitter_percent)) / 100;
                    
                    let mut rng = rand::thread_rng();
                    let jitter_ms = if max_jitter_ms > 0 {
                        #[allow(clippy::cast_possible_wrap)]
                        let max_jitter_ms_i64 = max_jitter_ms as i64;
                        rng.gen_range(-max_jitter_ms_i64..max_jitter_ms_i64)
                    } else {
                        0
                    };
                    
                    #[allow(clippy::cast_possible_wrap)]
                    let final_ms = interval_ms as i64 + jitter_ms;
                    let final_ms = final_ms.max(100);
                    
                    #[allow(clippy::cast_sign_loss)]
                    Duration::from_millis(final_ms as u64)
                } else {
                    Duration::from_secs(interval_secs)
                };

                tokio::time::sleep(sleep_duration).await;

                if !*service.running.lock().await {
                    break;
                }
            }
        });

        *self.task_handle.lock().await = Some(handle);

        Ok(())
    }

    /// Stop sending heartbeats
    ///
    /// # Errors
    /// Returns an error if the service is not running
    pub async fn stop_heartbeat(&self) -> Result<()>
    where
        C: HeartbeatConsumer,
    {
        let mut running = self.running.lock().await;
        if !*running {
            return Err(crate::error::Error::Other(
                "Heartbeat service is not running".to_string(),
            ));
        }

        *running = false;
        drop(running);

        let mut handle = self.task_handle.lock().await;
        if let Some(h) = handle.take() {
            h.abort();
        }

        Ok(())
    }
}

impl<C> Clone for HeartbeatService<C> {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            consumer: self.consumer.clone(),
            last_heartbeat: self.last_heartbeat.clone(),
            running: self.running.clone(),
            task_handle: self.task_handle.clone(),
        }
    }
}

impl<C> Drop for HeartbeatService<C> {
    fn drop(&mut self) {
        if let Ok(mut handle) = self.task_handle.try_lock() {
            if let Some(h) = handle.take() {
                h.abort();
            }
        }
    }
}
