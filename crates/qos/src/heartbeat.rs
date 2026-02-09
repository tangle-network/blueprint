use crate::error::{Error, Result};
use alloy_network::EthereumWallet;
use alloy_primitives::{Address, keccak256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::TransactionRequest;
use alloy_sol_types::SolCall;
use blueprint_core::{info, warn};
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tnt_core_bindings::bindings::r#i_operator_status_registry::IOperatorStatusRegistry::submitHeartbeatCall;
use tokio::{sync::Mutex, task::JoinHandle};

const ETH_MESSAGE_PREFIX: &[u8] = b"\x19Ethereum Signed Message:\n32";

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

    pub status_registry_address: Address,
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self {
            interval_secs: 300,
            jitter_percent: 10,
            service_id: 0,
            blueprint_id: 0,
            max_missed_heartbeats: 3,
            status_registry_address: Address::ZERO,
        }
    }
}

/// Status information included in a heartbeat submission to the chain.
///
/// This struct contains essential metadata that identifies the service and its current state,
/// including the current block number, timestamp, service and blueprint identifiers, and
/// optional status information. This data is encoded and signed before submission.
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    fn send_heartbeat(
        &self,
        status: &HeartbeatStatus,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'static>>;
}

/// Bridge trait for providing on-chain metrics to the heartbeat system.
///
/// The `MetricsProvider` trait uses RPITIT which is not dyn-compatible.
/// This trait provides a dyn-compatible bridge for reading on-chain metrics.
pub trait MetricsSource: Send + Sync + 'static {
    /// Read all pending on-chain metrics (non-destructive).
    fn get_custom_metrics(&self) -> Pin<Box<dyn Future<Output = Vec<(String, u64)>> + Send + '_>>;
    /// Clear on-chain metrics after successful submission.
    fn clear_custom_metrics(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}

/// Configuration required to execute blockchain transactions for heartbeats.
#[derive(Clone)]
struct HeartbeatRuntimeConfig {
    http_rpc_endpoint: String,
    keystore_uri: String,
    status_registry_address: Address,
    dry_run: bool,
}

struct HeartbeatTaskContext<C: HeartbeatConsumer + Send + Sync + 'static> {
    config_service_id: u64,
    config_blueprint_id: u64,
    consumer: Arc<C>,
    last_heartbeat_lock: Arc<Mutex<Option<HeartbeatStatus>>>,
    runtime: Arc<HeartbeatRuntimeConfig>,
    instance_service_id: u64,
    instance_blueprint_id: u64,
    metrics_source: Option<Arc<dyn MetricsSource>>,
}

impl<C: HeartbeatConsumer + Send + Sync + 'static> Clone for HeartbeatTaskContext<C> {
    fn clone(&self) -> Self {
        Self {
            config_service_id: self.config_service_id,
            config_blueprint_id: self.config_blueprint_id,
            consumer: Arc::clone(&self.consumer),
            last_heartbeat_lock: Arc::clone(&self.last_heartbeat_lock),
            runtime: Arc::clone(&self.runtime),
            instance_service_id: self.instance_service_id,
            instance_blueprint_id: self.instance_blueprint_id,
            metrics_source: self.metrics_source.clone(),
        }
    }
}

/// Service for sending periodic heartbeats to the Tangle EVM contracts.
#[derive(Clone)]
pub struct HeartbeatService<C: HeartbeatConsumer + Send + Sync + 'static> {
    config: HeartbeatConfig,
    consumer: Arc<C>,
    last_heartbeat: Arc<Mutex<Option<HeartbeatStatus>>>,
    running: Arc<Mutex<bool>>,
    task_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    runtime: Arc<HeartbeatRuntimeConfig>,
    service_id: u64,
    blueprint_id: u64,
    metrics_source: Option<Arc<dyn MetricsSource>>,
}

impl<C: HeartbeatConsumer + Send + Sync + 'static> HeartbeatService<C> {
    async fn do_send_heartbeat(args: HeartbeatTaskContext<C>) -> Result<()> {
        let HeartbeatTaskContext {
            config_service_id,
            config_blueprint_id,
            consumer,
            last_heartbeat_lock,
            runtime,
            instance_service_id,
            instance_blueprint_id,
            metrics_source,
        } = args;

        let status = HeartbeatStatus {
            block_number: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| Error::Other(format!("System time error: {e}")))?
                .as_secs(),
            service_id: config_service_id,
            blueprint_id: config_blueprint_id,
            status_code: 0,
            status_message: None,
        };

        consumer.send_heartbeat(&status).await?;
        *last_heartbeat_lock.lock().await = Some(status.clone());

        if runtime.dry_run {
            info!(
                service_id = config_service_id,
                blueprint_id = config_blueprint_id,
                "Dry run enabled; skipping on-chain heartbeat submission"
            );
            return Ok(());
        }

        info!(
            service_id = config_service_id,
            blueprint_id = config_blueprint_id,
            instance_service_id = instance_service_id,
            instance_blueprint_id = instance_blueprint_id,
            "Sending heartbeat to Tangle EVM status registry..."
        );

        let keystore = Keystore::new(KeystoreConfig::new().fs_root(runtime.keystore_uri.clone()))
            .map_err(|e| Error::Other(format!("Failed to initialize keystore: {e}")))?;

        let operator_ecdsa_public_key = keystore.first_local::<K256Ecdsa>().map_err(|e| {
            Error::Other(format!(
                "Failed to query operator ECDSA public key from keystore: {e}"
            ))
        })?;

        let operator_ecdsa_secret = keystore
            .get_secret::<K256Ecdsa>(&operator_ecdsa_public_key)
            .map_err(|e| Error::Other(format!("Failed to get operator ECDSA secret key: {e}")))?;

        let mut signing_key = operator_ecdsa_secret.clone();
        let custom_metrics = if let Some(ref source) = metrics_source {
            source.get_custom_metrics().await
        } else {
            vec![]
        };
        let metrics_bytes = if custom_metrics.is_empty() {
            vec![]
        } else {
            crate::metrics::abi::encode_metric_pairs(&custom_metrics)
        };
        let status_code = u8::try_from(status.status_code).unwrap_or(u8::MAX);
        let signature = sign_heartbeat_payload(
            &mut signing_key,
            status.service_id,
            status.blueprint_id,
            status_code,
            &metrics_bytes,
        )?;

        let local_signer = operator_ecdsa_secret
            .alloy_key()
            .map_err(|e| Error::Other(format!("Failed to prepare wallet signer: {e}")))?;
        let wallet = EthereumWallet::from(local_signer);

        let provider = ProviderBuilder::new()
            .wallet(wallet.clone())
            .connect(runtime.http_rpc_endpoint.as_str())
            .await
            .map_err(|e| Error::Other(format!("Failed to connect to RPC endpoint: {e}")))?;

        let heartbeat_call = submitHeartbeatCall {
            serviceId: status.service_id,
            blueprintId: status.blueprint_id,
            statusCode: status_code,
            metrics: metrics_bytes.into(),
            signature: signature.into(),
        };

        let calldata = heartbeat_call.abi_encode();

        let tx_request = TransactionRequest::default()
            .to(runtime.status_registry_address)
            .input(calldata.into());

        let pending_tx = provider
            .send_transaction(tx_request)
            .await
            .map_err(|e| Error::Other(format!("Failed to submit heartbeat transaction: {e}")))?;

        let receipt = pending_tx
            .get_receipt()
            .await
            .map_err(|e| Error::Other(format!("Failed to finalize heartbeat transaction: {e}")))?;

        if receipt.status() {
            info!(
                service_id = config_service_id,
                blueprint_id = config_blueprint_id,
                tx = %receipt.transaction_hash,
                "Heartbeat transaction finalized successfully"
            );
            // Clear metrics only after confirmed on-chain submission
            if let Some(ref source) = metrics_source {
                source.clear_custom_metrics().await;
            }
        } else {
            warn!(
                service_id = config_service_id,
                blueprint_id = config_blueprint_id,
                tx = %receipt.transaction_hash,
                "Heartbeat transaction reverted and may need a retry"
            );
        }

        Ok(())
    }

    /// Creates a new heartbeat service with the specified configuration and consumer.
    pub fn new(
        config: HeartbeatConfig,
        consumer: Arc<C>,
        http_rpc_endpoint: String,
        keystore_uri: String,
        status_registry_address: Address,
        dry_run: bool,
        service_id: u64,
        blueprint_id: u64,
    ) -> Result<Self> {
        Self::with_metrics_source(
            config,
            consumer,
            http_rpc_endpoint,
            keystore_uri,
            status_registry_address,
            dry_run,
            service_id,
            blueprint_id,
            None,
        )
    }

    /// Creates a new heartbeat service with metrics source for on-chain metric submission.
    pub fn with_metrics_source(
        config: HeartbeatConfig,
        consumer: Arc<C>,
        http_rpc_endpoint: String,
        keystore_uri: String,
        status_registry_address: Address,
        dry_run: bool,
        service_id: u64,
        blueprint_id: u64,
        metrics_source: Option<Arc<dyn MetricsSource>>,
    ) -> Result<Self> {
        if http_rpc_endpoint.is_empty() {
            return Err(Error::Other(
                "HTTP RPC endpoint is required for heartbeat service".to_string(),
            ));
        }

        if status_registry_address.is_zero() {
            return Err(Error::Other(
                "Status registry contract address must be configured for heartbeats".to_string(),
            ));
        }

        Ok(Self {
            config,
            consumer,
            last_heartbeat: Arc::new(Mutex::new(None)),
            running: Arc::new(Mutex::new(false)),
            task_handle: Arc::new(Mutex::new(None)),
            runtime: Arc::new(HeartbeatRuntimeConfig {
                http_rpc_endpoint,
                keystore_uri,
                status_registry_address,
                dry_run,
            }),
            service_id,
            blueprint_id,
            metrics_source,
        })
    }

    /// Returns the most recent heartbeat status sent to the chain, if available.
    #[must_use]
    pub async fn last_heartbeat(&self) -> Option<HeartbeatStatus> {
        self.last_heartbeat.lock().await.clone()
    }

    /// Checks if the heartbeat service is currently active and sending heartbeats.
    #[must_use]
    pub async fn is_running(&self) -> bool {
        *self.running.lock().await
    }

    /// Starts the heartbeat service, which will periodically send heartbeats to the chain.
    pub async fn start_heartbeat(&self) -> Result<()> {
        {
            let mut running = self.running.lock().await;
            if *running {
                return Err(Error::Heartbeat("Heartbeat service already running".into()));
            }
            *running = true;
        }

        let initial_jitter_percent = self.config.jitter_percent;
        let initial_interval_ms = self.config.interval_secs * 1000;
        let initial_jitter = if initial_jitter_percent > 0 {
            let max_jitter = (initial_interval_ms * u64::from(initial_jitter_percent)) / 100;
            rand::thread_rng().gen_range(0..=max_jitter)
        } else {
            0
        };
        tokio::time::sleep(Duration::from_millis(initial_jitter)).await;

        let context = HeartbeatTaskContext {
            config_service_id: self.config.service_id,
            config_blueprint_id: self.config.blueprint_id,
            consumer: Arc::clone(&self.consumer),
            last_heartbeat_lock: Arc::clone(&self.last_heartbeat),
            runtime: Arc::clone(&self.runtime),
            instance_service_id: self.service_id,
            instance_blueprint_id: self.blueprint_id,
            metrics_source: self.metrics_source.clone(),
        };

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

                if let Err(e) = HeartbeatService::do_send_heartbeat(context.clone()).await {
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
    pub async fn stop_heartbeat(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        if !*running {
            return Err(Error::Other("Heartbeat service is not running".to_string()));
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

/// Sign heartbeat payload: keccak256(abi.encodePacked(serviceId, blueprintId, statusCode, metrics))
/// with Ethereum signed message prefix. Must match OperatorStatusRegistry.sol verification.
fn sign_heartbeat_payload(
    signing_key: &mut K256SigningKey,
    service_id: u64,
    blueprint_id: u64,
    status_code: u8,
    metrics: &[u8],
) -> Result<Vec<u8>> {
    let mut payload = Vec::with_capacity(17 + metrics.len());
    payload.extend_from_slice(&service_id.to_be_bytes());
    payload.extend_from_slice(&blueprint_id.to_be_bytes());
    payload.push(status_code);
    payload.extend_from_slice(metrics);

    let message_hash = keccak256(&payload);

    let mut prefixed = Vec::with_capacity(ETH_MESSAGE_PREFIX.len() + message_hash.len());
    prefixed.extend_from_slice(ETH_MESSAGE_PREFIX);
    prefixed.extend_from_slice(message_hash.as_slice());

    let prefixed_hash = keccak256(&prefixed);
    let mut digest = [0u8; 32];
    digest.copy_from_slice(prefixed_hash.as_slice());

    let (signature, recovery_id) = signing_key
        .0
        .sign_prehash_recoverable(&digest)
        .map_err(|e| Error::Other(format!("Failed to sign heartbeat payload: {e}")))?;

    let mut signature_bytes = Vec::with_capacity(65);
    signature_bytes.extend_from_slice(&signature.to_bytes());
    signature_bytes.push(recovery_id.to_byte() + 27);
    Ok(signature_bytes)
}
