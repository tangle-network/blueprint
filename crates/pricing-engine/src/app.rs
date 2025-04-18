//! Application-level functionality for the pricing engine
//!
//! This module contains the high-level application logic that ties together
//! the various components of the pricing engine.

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use crate::{
    benchmark_cache::BenchmarkCache,
    config::{OperatorConfig, load_config_from_path},
    error::{PricingError, Result},
    handlers::handle_blueprint_update,
    service::blockchain::event::BlockchainEvent,
    service::blockchain::listener::EventListener,
    signer::OperatorSigner,
};

use blueprint_keystore::backends::Backend;

/// Initialize the logging system with the specified log level
pub fn init_logging(log_level: &str) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}

/// Start the blockchain event listener if the feature is enabled
pub async fn start_blockchain_listener(
    node_url: String,
    event_tx: mpsc::Sender<BlockchainEvent>,
) -> Option<tokio::task::JoinHandle<()>> {
    if cfg!(feature = "tangle-listener") {
        info!("Starting blockchain event listener");
        Some(tokio::spawn(async move {
            match EventListener::new(node_url, event_tx).await {
                Ok(listener) => {
                    if let Err(e) = listener.run().await {
                        error!("Blockchain listener error: {}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to create blockchain listener: {}", e);
                }
            }
        }))
    } else {
        info!("Blockchain event listener feature not enabled");
        None
    }
}

/// Load the operator configuration from the specified path
pub async fn load_operator_config(config_path: &PathBuf) -> Result<Arc<OperatorConfig>> {
    let config = load_config_from_path(config_path)
        .map_err(|e| PricingError::Config(format!("Failed to load config: {}", e)))?;
    let config = Arc::new(config);
    info!("Configuration loaded");
    Ok(config)
}

/// Initialize the benchmark cache
pub async fn init_benchmark_cache(config: &Arc<OperatorConfig>) -> Result<Arc<BenchmarkCache>> {
    let benchmark_cache = Arc::new(BenchmarkCache::new(&config.database_path).map_err(|e| {
        PricingError::Cache(format!("Failed to initialize benchmark cache: {}", e))
    })?);
    info!("Benchmark cache initialized");
    Ok(benchmark_cache)
}

/// Initialize the operator signer with a keypair
pub fn init_operator_signer<P: AsRef<std::path::Path>>(
    config: &OperatorConfig,
    keystore_path: P,
) -> Result<Arc<Mutex<OperatorSigner<blueprint_keystore::crypto::k256::K256Ecdsa>>>> {
    use blueprint_crypto::BytesEncoding;
    use blueprint_keystore::crypto::k256::K256Ecdsa;
    use blueprint_keystore::{Keystore, KeystoreConfig};

    info!("Initializing operator signer with K256Ecdsa");

    let keystore_path = keystore_path.as_ref();
    if !keystore_path.exists() {
        info!("Creating keystore directory: {:?}", keystore_path);
        std::fs::create_dir_all(keystore_path)?;
    }

    // Initialize the keystore
    let keystore_config = KeystoreConfig::new().fs_root(keystore_path);
    let keystore = Keystore::new(keystore_config)?;

    // Get or generate the keypair
    let public_key = match keystore.list_local::<K256Ecdsa>()? {
        keys if !keys.is_empty() => {
            info!("Using existing K256Ecdsa operator key");
            keys[0]
        }
        _ => {
            info!("Generating new K256Ecdsa operator key");
            // Generate a new keypair
            keystore.generate::<K256Ecdsa>(None)?
        }
    };

    // Get the secret key
    let keypair = keystore.get_secret::<K256Ecdsa>(&public_key)?;

    // Create a deterministic operator ID from the public key
    let mut operator_id = [0u8; 32];
    let public_bytes = public_key.to_bytes();

    // Copy the public key bytes to the operator ID, or hash them if needed
    if public_bytes.len() >= 32 {
        operator_id.copy_from_slice(&public_bytes[0..32]);
    } else {
        // If public key is shorter than 32 bytes, use a hash
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&public_bytes);
        operator_id.copy_from_slice(&hasher.finalize());
    }

    // Create the operator signer
    let signer = OperatorSigner::new(config, keypair, operator_id)?;
    info!(
        "K256Ecdsa operator signer initialized with public key: {:?}",
        signer.public_key()
    );

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

            // Extract blueprint ID and determine if we need to update pricing
            let (blueprint_id, update_pricing) = match event {
                BlockchainEvent::Registered(e) => (Some(e.blueprint_id), true),
                BlockchainEvent::PriceTargetsUpdated(e) => (Some(e.blueprint_id), true),
                _ => (None, false),
            };

            if update_pricing {
                if let Some(id) = blueprint_id {
                    info!("Updating pricing for blueprint ID: {}", id);

                    // Handle the blueprint update
                    if let Err(e) =
                        handle_blueprint_update(id, benchmark_cache.clone(), config.clone()).await
                    {
                        error!("Failed to update pricing: {}", e);
                    }
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
