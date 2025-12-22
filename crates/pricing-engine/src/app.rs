use blueprint_client_tangle_evm::{TangleEvmClient, TangleEvmClientConfig};
use blueprint_core::{error, info};
use blueprint_crypto::k256::K256Ecdsa;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

use crate::{
    OperatorSigner,
    benchmark_cache::BenchmarkCache,
    config::{OperatorConfig, load_config_from_path},
    error::{PricingError, Result},
    handlers::handle_blueprint_update,
    service::blockchain::{event::BlockchainEvent, evm_listener::EvmEventListener},
};

use blueprint_keystore::{Keystore, KeystoreConfig};

use blueprint_keystore::backends::Backend;

/// Start the blockchain event listener if the feature is enabled
pub async fn start_blockchain_listener(
    evm_config: TangleEvmClientConfig,
    event_tx: mpsc::Sender<BlockchainEvent>,
) -> Option<tokio::task::JoinHandle<()>> {
    match TangleEvmClient::new(evm_config).await {
        Ok(client) => {
            let listener = EvmEventListener::new(Arc::new(client), event_tx);
            Some(tokio::spawn(async move {
                if let Err(e) = listener.run().await {
                    error!("Blockchain listener error: {e}");
                }
            }))
        }
        Err(e) => {
            error!("Failed to start blockchain listener: {}", e);
            None
        }
    }
}

/// Load the operator configuration from the specified path
pub async fn load_operator_config(config_path: &PathBuf) -> Result<Arc<OperatorConfig>> {
    let config = load_config_from_path(config_path)
        .map_err(|e| PricingError::Config(format!("Failed to load config: {e}")))?;
    let config = Arc::new(config);
    info!("Configuration loaded");
    Ok(config)
}

/// Initialize the benchmark cache
pub async fn init_benchmark_cache(config: &Arc<OperatorConfig>) -> Result<Arc<BenchmarkCache>> {
    let benchmark_cache =
        Arc::new(BenchmarkCache::new(&config.database_path).map_err(|e| {
            PricingError::Cache(format!("Failed to initialize benchmark cache: {e}"))
        })?);
    info!("Benchmark cache initialized");
    Ok(benchmark_cache)
}

/// Initialize the operator signer with a keypair
pub fn init_operator_signer<P: AsRef<std::path::Path>>(
    config: &OperatorConfig,
    keystore_path: P,
    domain: crate::signer::QuoteSigningDomain,
) -> Result<Arc<Mutex<OperatorSigner>>> {
    info!("Initializing operator signer with ECDSA");

    let keystore_path = keystore_path.as_ref();
    if !keystore_path.exists() {
        info!("Creating keystore directory: {:?}", keystore_path);
        std::fs::create_dir_all(keystore_path)?;
    }

    let keystore_config = KeystoreConfig::new().fs_root(keystore_path);
    let keystore = Keystore::new(keystore_config)?;

    let ecdsa_public_key = match keystore.list_local::<K256Ecdsa>()? {
        keys if !keys.is_empty() => {
            info!("Using existing ECDSA operator key");
            keys[0]
        }
        _ => {
            info!("Generating new ECDSA operator key");
            // Generate a new keypair
            keystore.generate::<K256Ecdsa>(None)?
        }
    };
    let ecdsa_keypair = keystore.get_secret::<K256Ecdsa>(&ecdsa_public_key)?;
    let signer = OperatorSigner::new(config, ecdsa_keypair, domain)?;

    Ok(Arc::new(Mutex::new(signer)))
}

/// Process blockchain events and update pricing as needed
pub fn spawn_event_processor(
    mut event_rx: mpsc::Receiver<BlockchainEvent>,
    benchmark_cache: Arc<BenchmarkCache>,
    config: Arc<OperatorConfig>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        info!("Starting blockchain event processor");
        while let Some(event) = event_rx.recv().await {
            info!("Received blockchain event: {:?}", event);

            if let BlockchainEvent::ServiceActivated { blueprint_id, .. } = event {
                info!("Updating pricing for blueprint ID: {}", blueprint_id);
                if let Err(e) =
                    handle_blueprint_update(blueprint_id, benchmark_cache.clone(), config.clone())
                        .await
                {
                    error!("Failed to update pricing: {}", e);
                }
            }
        }
        info!("Blockchain event processor stopped");
    })
}

/// Wait for a shutdown signal (Ctrl+C)
pub async fn wait_for_shutdown() {
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            info!("Received shutdown signal");
        }
        Err(e) => {
            error!("Failed to listen for shutdown signal: {}", e);
        }
    }
}

/// Clean up resources and shut down the application
pub async fn cleanup(listener_handle: Option<tokio::task::JoinHandle<()>>) {
    info!("Shutting down Tangle Cloud Pricing Engine");
    if let Some(handle) = listener_handle {
        handle.abort();
        match handle.await {
            Ok(_) => info!("Blockchain listener stopped"),
            Err(e) if e.is_cancelled() => info!("Blockchain listener cancelled"),
            Err(e) => error!("Error stopping blockchain listener: {}", e),
        }
    }
}
