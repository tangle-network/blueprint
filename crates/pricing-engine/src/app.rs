use blueprint_core::{error, info};
use blueprint_crypto::{
    sp_core::{SpEcdsa, SpSr25519},
    tangle_pair_signer::TanglePairSigner,
};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

use crate::{
    OperatorSigner,
    benchmark_cache::BenchmarkCache,
    config::{OperatorConfig, load_config_from_path},
    error::{PricingError, Result},
    handlers::handle_blueprint_update,
    service::blockchain::{event::BlockchainEvent, listener::EventListener},
};
use tangle_subxt::subxt::tx::Signer;

use blueprint_keystore::{Keystore, KeystoreConfig};

use blueprint_keystore::backends::Backend;

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
                        error!("Blockchain listener error: {e}");
                    }
                }
                Err(e) => {
                    error!("Failed to create blockchain listener: {e}");
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
) -> Result<Arc<Mutex<OperatorSigner<SpEcdsa>>>> {
    info!("Initializing operator signer with ECDSA");

    let keystore_path = keystore_path.as_ref();
    if !keystore_path.exists() {
        info!("Creating keystore directory: {:?}", keystore_path);
        std::fs::create_dir_all(keystore_path)?;
    }

    // Initialize the keystore
    let keystore_config = KeystoreConfig::new().fs_root(keystore_path);
    let keystore = Keystore::new(keystore_config)?;

    let ecdsa_public_key = match keystore.list_local::<SpEcdsa>()? {
        keys if !keys.is_empty() => {
            info!("Using existing ECDSA operator key");
            keys[0]
        }
        _ => {
            info!("Generating new ECDSA operator key");
            // Generate a new keypair
            keystore.generate::<SpEcdsa>(None)?
        }
    };
    let ecdsa_keypair = keystore.get_secret::<SpEcdsa>(&ecdsa_public_key)?;

    let sr25519_public_key = match keystore.list_local::<SpSr25519>()? {
        keys if !keys.is_empty() => {
            info!("Using existing SR25519 operator key");
            keys[0]
        }
        _ => {
            info!("Generating new SR25519 operator key");
            // Generate a new keypair
            keystore.generate::<SpSr25519>(None)?
        }
    };
    let sr25519_keypair = keystore.get_secret::<SpSr25519>(&sr25519_public_key)?;
    let signer = TanglePairSigner::new(sr25519_keypair.0);
    let operator_id = signer.account_id();

    let signer = OperatorSigner::new(config, ecdsa_keypair, operator_id)?;

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

            let (blueprint_id, update_pricing) = match event {
                BlockchainEvent::Registered(e) => (Some(e.blueprint_id), true),
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
