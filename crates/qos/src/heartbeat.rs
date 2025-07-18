use crate::error::Result;
use blueprint_crypto::sp_core::SpSr25519;
use blueprint_crypto::{hashing, sp_core::SpEcdsa};

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

/// Configuration for the heartbeat service that sends periodic liveness signals to the chain.
///
/// Heartbeats are critical for service reliability monitoring and slashing prevention.
/// They signal that the service is alive and functional, allowing the blockchain to
/// track operator performance and trigger penalties when services fail.
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

/// Status information included in a heartbeat submission to the chain.
///
/// This struct contains essential metadata that identifies the service and its current state,
/// including the current block number, timestamp, service and blueprint identifiers, and
/// optional status information. This data is encoded and signed before submission.
#[derive(Clone, Debug, Encode, Decode)] // Added Encode, Decode
pub struct HeartbeatStatus {
    pub block_number: u64,

    pub timestamp: u64,

    pub service_id: u64,

    pub blueprint_id: u64,

    pub status_code: u32,

    pub status_message: Option<String>,
}

/// Trait for sending heartbeats to the blockchain.
///
/// Implementers of this trait handle the actual submission of heartbeat data
/// to the chain, allowing for different transport mechanisms or chain targets
/// while maintaining a consistent heartbeat protocol.
pub trait HeartbeatConsumer: Send + Sync + 'static {
    /// Sends a heartbeat status update to the blockchain.
    ///
    /// This method handles the actual submission of the heartbeat data to the chain,
    /// which typically involves signing the heartbeat message and submitting it
    /// as an extrinsic to the `Tangle` blockchain.
    fn send_heartbeat(
        &self,
        status: &HeartbeatStatus,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'static>>;
}

/// Service for sending periodic heartbeats to the `Tangle` blockchain.
///
/// This service runs in the background and periodically submits signed `heartbeat`
/// messages to the chain according to the configured interval. Heartbeats provide
/// proof that a service is alive and help prevent slashing due to inactivity.
/// The service includes jitter to prevent thundering herd problems.
#[derive(Clone)]
pub struct HeartbeatService<C: HeartbeatConsumer + Send + Sync + 'static> {
    config: HeartbeatConfig,
    consumer: Arc<C>,
    last_heartbeat: Arc<Mutex<Option<HeartbeatStatus>>>,
    running: Arc<Mutex<bool>>,
    task_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    ws_rpc_endpoint: String,
    keystore_uri: String,
    service_id: u64,
    blueprint_id: u64,
}

struct HeartbeatContextArgs<C: HeartbeatConsumer + Send + Sync + 'static> {
    config_service_id: u64,
    config_blueprint_id: u64,
    consumer: Arc<C>,
    last_heartbeat_lock: Arc<Mutex<Option<HeartbeatStatus>>>,
    ws_rpc_endpoint: String,
    keystore_uri: String,
    instance_service_id: u64,
    instance_blueprint_id: u64,
}

impl<C: HeartbeatConsumer + Send + Sync + 'static> HeartbeatService<C> {
    async fn do_send_heartbeat(args: HeartbeatContextArgs<C>) -> Result<()> {
        let HeartbeatContextArgs {
            config_service_id,
            config_blueprint_id,
            consumer,
            last_heartbeat_lock,
            ws_rpc_endpoint,
            keystore_uri,
            instance_service_id,
            instance_blueprint_id,
        } = args;
        let status = HeartbeatStatus {
            block_number: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| crate::error::Error::Other(format!("System time error: {}", e)))?
                .as_secs(),
            service_id: config_service_id,
            blueprint_id: config_blueprint_id,
            status_code: 0,
            status_message: None,
        };

        consumer.send_heartbeat(&status).await?;
        *last_heartbeat_lock.lock().await = Some(status.clone());

        info!(
            service_id = config_service_id,
            blueprint_id = config_blueprint_id,
            instance_service_id = instance_service_id,
            instance_blueprint_id = instance_blueprint_id,
            "Attempting to send heartbeat to chain..."
        );

        let client = tangle_subxt::subxt::OnlineClient::from_insecure_url(ws_rpc_endpoint.clone())
            .await
            .unwrap();

        let keystore_config =
            blueprint_keystore::KeystoreConfig::new().fs_root(keystore_uri.clone());
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

    /// Creates a new heartbeat service with the specified configuration and consumer.
    ///
    /// # Parameters
    /// * `config` - Configuration settings for heartbeat intervals, jitter, and thresholds
    /// * `consumer` - The component responsible for sending heartbeats to the chain
    /// * `ws_rpc_endpoint` - WebSocket RPC endpoint for blockchain communication
    /// * `keystore_uri` - URI of the keystore containing signing credentials
    /// * `service_id` - Unique identifier of the service on the blockchain
    /// * `blueprint_id` - Identifier of the blueprint the service is running
    pub fn new(
        config: HeartbeatConfig,
        consumer: Arc<C>,
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
            ws_rpc_endpoint,
            keystore_uri,
            service_id,
            blueprint_id,
        }
    }

    /// Returns the most recent heartbeat status sent to the chain, if available.
    ///
    /// This information can be used to verify when the last successful heartbeat
    /// was sent and what status information was included.
    #[must_use]
    pub async fn last_heartbeat(&self) -> Option<HeartbeatStatus> {
        self.last_heartbeat.lock().await.clone()
    }

    /// Checks if the heartbeat service is currently active and sending heartbeats.
    ///
    /// Returns `true` if the service is running and sending heartbeats at the configured
    /// interval, `false` otherwise.
    #[must_use]
    pub async fn is_running(&self) -> bool {
        *self.running.lock().await
    }

    /// Starts the heartbeat service, which will periodically send heartbeats to the chain.
    ///
    /// This method launches a background task that sends heartbeats according to the
    /// configured interval with some jitter to prevent synchronized heartbeats across
    /// the network. Each heartbeat includes current service status and is cryptographically
    /// signed to verify authenticity.
    ///
    /// # Errors
    /// Returns an error if the service is already running
    pub async fn start_heartbeat(&self) -> Result<()> {
        let initial_jitter_percent = self.config.jitter_percent;
        let initial_interval_ms = self.config.interval_secs * 1000;
        let initial_jitter = if initial_jitter_percent > 0 {
            let max_jitter = (initial_interval_ms * u64::from(initial_jitter_percent)) / 100;
            rand::thread_rng().gen_range(0..=max_jitter)
        } else {
            0
        };
        tokio::time::sleep(Duration::from_millis(initial_jitter)).await;

        let config_service_id = self.config.service_id;
        let config_blueprint_id = self.config.blueprint_id;
        let consumer = Arc::clone(&self.consumer);
        let last_heartbeat_lock = Arc::clone(&self.last_heartbeat);
        let ws_rpc_endpoint = self.ws_rpc_endpoint.clone();
        let keystore_uri = self.keystore_uri.clone();
        let service_id = self.service_id;
        let blueprint_id = self.blueprint_id;
        let interval_ms = self.config.interval_secs * 1000;
        let jitter_percent_val = self.config.jitter_percent;
        let running_status = Arc::clone(&self.running);

        let handle = tokio::spawn(async move {
            let base_interval = Duration::from_millis(interval_ms);
            loop {
                if !*running_status.lock().await {
                    info!("Heartbeat service stopping as requested.");
                    break;
                }

                let context_args = HeartbeatContextArgs {
                    config_service_id,
                    config_blueprint_id,
                    consumer: Arc::clone(&consumer),
                    last_heartbeat_lock: Arc::clone(&last_heartbeat_lock),
                    ws_rpc_endpoint: ws_rpc_endpoint.clone(),
                    keystore_uri: keystore_uri.clone(),
                    instance_service_id: service_id,
                    instance_blueprint_id: blueprint_id,
                };
                if let Err(e) = HeartbeatService::do_send_heartbeat(context_args).await {
                    warn!("Failed to send heartbeat: {}", e);
                }

                let sleep_duration = if jitter_percent_val > 0 {
                    let max_jitter = (interval_ms * u64::from(jitter_percent_val)) / 100;
                    let current_jitter = rand::thread_rng().gen_range(0..=max_jitter);
                    base_interval + Duration::from_millis(current_jitter)
                } else {
                    base_interval
                };

                tokio::time::sleep(sleep_duration).await;
            }
        });

        *self.task_handle.lock().await = Some(handle);

        Ok(())
    }

    /// Stops the heartbeat service and terminates the background heartbeat task.
    ///
    /// This will prevent further heartbeats from being sent to the chain. Services
    /// should call this method during graceful shutdown to avoid resource leaks.
    /// Note that stopping heartbeats may eventually trigger slashing if the service
    /// remains inactive beyond the threshold period defined on-chain.
    ///
    /// # Errors
    /// Returns an error if the service is not currently running
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
