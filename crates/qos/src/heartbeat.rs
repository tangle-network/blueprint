use rand::{Rng, thread_rng};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::sync::oneshot::{self, Receiver};
use tokio::time;
use tracing::{error, info};

use crate::error::Result;
// use blueprint_runner::{BackgroundService, error::RunnerError};

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
    pub async fn last_heartbeat(&self) -> Option<HeartbeatStatus> {
        self.last_heartbeat.lock().await.clone()
    }

    /// Check if the service is running
    pub async fn is_running(&self) -> bool {
        *self.running.lock().await
    }

    /// Send a heartbeat to the chain
    async fn send_heartbeat(&self) -> Result<()> {
        let status = HeartbeatStatus {
            block_number: 0,
            timestamp: chrono::Utc::now().timestamp() as u64,
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

// impl<C> BackgroundService for HeartbeatService<C>
// where
//     C: HeartbeatConsumer,
// {
//     fn start(
//         &self,
//     ) -> impl std::future::Future<
//         Output = std::result::Result<Receiver<std::result::Result<(), RunnerError>>, RunnerError>,
//     > + Send {
//         let config = self.config.clone();
//         let running = self.running.clone();
//         let service = self.clone();

//         async move {
//             let (tx, rx) = oneshot::channel();

//             *running.lock().await = true;

//             tokio::spawn(async move {
//                 let mut interval = time::interval(Duration::from_secs(config.interval_secs));

//                 loop {
//                     interval.tick().await;

//                     if config.jitter_percent > 0 {
//                         let jitter = thread_rng().gen_range(0..=config.jitter_percent) as u64;
//                         let jitter_ms = (config.interval_secs * jitter * 10) / 1000;
//                         if jitter_ms > 0 {
//                             time::sleep(Duration::from_millis(jitter_ms)).await;
//                         }
//                     }

//                     if !*running.lock().await {
//                         break;
//                     }

//                     if let Err(e) = service.send_heartbeat().await {
//                         error!("Failed to send heartbeat: {}", e);
//                     }
//                 }

//                 let _ = tx.send(Ok(()));
//             });

//             Ok(rx)
//         }
//     }
// }

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

// /// Implementation of HeartbeatConsumer for Tangle
// #[cfg(feature = "tangle")]
// pub mod tangle {
//     use super::*;
//     use blueprint_clients::tangle::client::TangleClient;
//     use blueprint_crypto::tangle_pair_signer::TanglePairSigner;

//     /// Tangle heartbeat consumer
//     pub struct TangleHeartbeatConsumer<S> {
//         client: TangleClient,

//         signer: S,
//     }

//     impl<S> TangleHeartbeatConsumer<S> {
//         /// Create a new Tangle heartbeat consumer
//         pub fn new(client: TangleClient, signer: S) -> Self {
//             Self { client, signer }
//         }
//     }

//     impl<S> HeartbeatConsumer for TangleHeartbeatConsumer<S>
//     where
//         S: Send + Sync + Clone + 'static,
//     {
//         async fn send_heartbeat(&self, status: &HeartbeatStatus) -> Result<()> {
//             info!(
//                 service_id = status.service_id,
//                 blueprint_id = status.blueprint_id,
//                 "Sending heartbeat to Tangle chain"
//             );

//             Ok(())
//         }
//     }
// }
