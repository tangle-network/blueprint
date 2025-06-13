use crate::error::Result;
use blueprint_crypto::sp_core::SpSr25519;
use blueprint_crypto::{hashing, sp_core::SpEcdsa};

use blueprint_client_tangle::client::TangleClient;
use blueprint_crypto::tangle_pair_signer::TanglePairSigner;
use blueprint_keystore::backends::Backend;
use sp_core::{Pair, ecdsa::Signature as SpEcdsaSignature};

use blueprint_core::{info, warn};
use parity_scale_codec::{Decode, Encode};
use rand::Rng;
use std::{
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tangle_subxt::tangle_testnet_runtime::api as tangle_api;
use tokio::{sync::Mutex, task::JoinHandle};

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
#[derive(Clone, Debug, Encode, Decode)] // Added Encode, Decode
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
    ) -> Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'static>>;
}

/// Service for sending heartbeats to the chain
#[derive(Clone)]
pub struct HeartbeatService<C: HeartbeatConsumer + Send + Sync + 'static> {
    // C (HeartbeatConsumer) is now a trait object
    config: HeartbeatConfig,
    consumer: Arc<C>,
    last_heartbeat: Arc<Mutex<Option<HeartbeatStatus>>>,
    running: Arc<Mutex<bool>>,
    task_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    http_rpc_endpoint: String,
    ws_rpc_endpoint: String,
    keystore_uri: String,
    service_id: u64,
    blueprint_id: u64,
}

impl<C: HeartbeatConsumer + Send + Sync + 'static> HeartbeatService<C> {
    async fn do_send_heartbeat(
        config_service_id: u64,
        config_blueprint_id: u64,
        consumer: Arc<C>,
        last_heartbeat_lock: Arc<Mutex<Option<HeartbeatStatus>>>,
        ws_rpc_endpoint: String,
        keystore_uri: String,
        service_id: u64,
        blueprint_id: u64,
    ) -> Result<()> {
        // --- Part 1: Local heartbeat via consumer ---
        let status = HeartbeatStatus {
            block_number: 0, // For local consumer, block_number might not be critical or fetched from chain here
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| crate::error::Error::Other(format!("System time error: {}", e)))?
                .as_secs(),
            service_id: config_service_id, // Use passed parameter
            blueprint_id: config_blueprint_id, // Use passed parameter
            status_code: 0, // Assuming 0 is OK
            status_message: None,
        };

        consumer.send_heartbeat(&status).await?;
        *last_heartbeat_lock.lock().await = Some(status.clone());

        // --- Part 2: On-chain heartbeat ---
        info!(
            service_id = config_service_id,
            blueprint_id = config_blueprint_id,
            "Attempting to send heartbeat to chain..."
        );

        let client = TangleClient::new(
            ws_rpc_endpoint.clone(), // Use passed parameter
            keystore_uri.clone(),    // Use passed parameter
            blueprint_id,            // Use passed parameter (direct service_id for TangleClient)
            service_id,              // Use passed parameter (direct blueprint_id for TangleClient)
        )
        .await
        .map_err(|e| crate::error::Error::Other(format!("Failed to create TangleClient: {}", e)))?;

        let keystore_config = blueprint_keystore::KeystoreConfig::new().fs_root(keystore_uri.clone());
        let keystore = blueprint_keystore::Keystore::new(keystore_config).map_err(|e| {
            crate::error::Error::Other(format!("Failed to initialize keystore: {}", e))
        })?;

        let operator_ecdsa_public_key = keystore.first_local::<SpEcdsa>().map_err(|e| {
            crate::error::Error::Other(format!(
                "Failed to query ECDSA public key from keystore: {}",
                e
            ))
        })?;

        let operator_ecdsa_secret = keystore
            .get_secret::<SpEcdsa>(&operator_ecdsa_public_key)
            .map_err(|e| {
                crate::error::Error::Other(format!("Failed to get ECDSA secret key: {}", e))
            })?;

        let submitter_sr25519_public_key = keystore.first_local::<SpSr25519>().map_err(|e| {
            crate::error::Error::Other(format!(
                "Failed to query SR25519 public key for submission: {}",
                e
            ))
        })?;

        let submitter_sr25519_secret = keystore
            .get_secret::<SpSr25519>(&submitter_sr25519_public_key)
            .map_err(|e| {
                crate::error::Error::Other(format!(
                    "Failed to get SR25519 secret key for submission: {}",
                    e
                ))
            })?;

        let submitter_signer = TanglePairSigner::new(submitter_sr25519_secret.0);
        let metrics_data_bytes = status.encode();
        let service_id_for_payload = status.service_id;
        let blueprint_id_for_payload = status.blueprint_id;
        let mut message_to_sign = service_id_for_payload.to_le_bytes().to_vec();
        message_to_sign.extend_from_slice(&blueprint_id_for_payload.to_le_bytes());
        message_to_sign.extend_from_slice(&metrics_data_bytes);
        let message_hash = hashing::keccak_256(&message_to_sign);
        let signature: SpEcdsaSignature = operator_ecdsa_secret.sign(&message_hash);

        let heartbeat_call = tangle_api::tx().services().heartbeat(
            service_id_for_payload,
            blueprint_id_for_payload,
            metrics_data_bytes.clone(),
            signature.0,
        );

        let extrinsic_result = client
            .tx()
            .sign_and_submit_then_watch_default(&heartbeat_call, &submitter_signer)
            .await
            .map_err(|e| {
                crate::error::Error::Other(format!("Failed to submit heartbeat extrinsic: {}", e))
            })?;

        let _events = extrinsic_result
            .wait_for_finalized_success()
            .await
            .map_err(|e| {
                crate::error::Error::Other(format!("Heartbeat extrinsic failed to finalize: {}", e))
            })?;

        info!(
            service_id = config_service_id,
            blueprint_id = config_blueprint_id,
            "Successfully sent heartbeat to chain and it finalized."
        );

        Ok(())
    }

    /// Create a new heartbeat service
    pub fn new(
        config: HeartbeatConfig,
        consumer: Arc<C>,
        http_rpc_endpoint: String,
        ws_rpc_endpoint: String,
        keystore_uri: String,
        service_id: u64,
        blueprint_id: u64,
    ) -> Self {
        Self {
            config,
            consumer,
            last_heartbeat: Arc::new(Mutex::new(None)),
            running: Arc::new(Mutex::new(false)),
            task_handle: Arc::new(Mutex::new(None)),
            http_rpc_endpoint,
            ws_rpc_endpoint,
            keystore_uri,
            service_id,
            blueprint_id,
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



    /// Start sending heartbeats at the configured interval
    ///
    /// # Errors
    /// Returns an error if the service is already running
    pub async fn start_heartbeat(&self) -> Result<()> {
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
                                let cfg_service_id = service.config.service_id;
                let cfg_blueprint_id = service.config.blueprint_id;
                let consumer_clone = Arc::clone(&service.consumer);
                let last_heartbeat_clone = Arc::clone(&service.last_heartbeat);
                let ws_endpoint_clone = service.ws_rpc_endpoint.clone();
                let keystore_clone = service.keystore_uri.clone();
                let service_id_val = service.service_id; // direct field
                let blueprint_id_val = service.blueprint_id; // direct field

                if let Err(e) = HeartbeatService::do_send_heartbeat(
                    cfg_service_id,
                    cfg_blueprint_id,
                    consumer_clone,
                    last_heartbeat_clone,
                    ws_endpoint_clone,
                    keystore_clone,
                    service_id_val,
                    blueprint_id_val,
                ).await {
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
    pub async fn stop_heartbeat(&self) -> Result<()> {
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

impl<C: HeartbeatConsumer + Send + Sync + 'static> Drop for HeartbeatService<C> {
    fn drop(&mut self) {
        if let Ok(mut handle) = self.task_handle.try_lock() {
            if let Some(h) = handle.take() {
                h.abort();
            }
        }
    }
}
