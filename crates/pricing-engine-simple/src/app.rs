//! Application-level functionality for the pricing engine
//!
//! This module contains the high-level application logic that ties together
//! the various components of the pricing engine.

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use crate::{
    cache::PriceCache,
    config::{OperatorConfig, load_config_from_path},
    error::{PricingError, Result},
    handlers::handle_blueprint_update,
    service::blockchain::event::BlockchainEvent,
    service::blockchain::listener::EventListener,
};

/// Initialize the logging system with the specified log level
pub fn init_logging(log_level: &str) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level.to_string()));
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

/// Initialize the price cache
pub async fn init_price_cache(config: &Arc<OperatorConfig>) -> Result<Arc<PriceCache>> {
    let price_cache =
        Arc::new(PriceCache::new(&config.database_path).map_err(|e| {
            PricingError::Cache(format!("Failed to initialize price cache: {}", e))
        })?);
    info!("Price cache initialized");
    Ok(price_cache)
}

/// Process blockchain events and update pricing as needed
pub fn spawn_event_processor(
    mut event_rx: mpsc::Receiver<BlockchainEvent>,
    price_cache: Arc<PriceCache>,
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
                    // Convert ID to a string hash format for the cache
                    let blueprint_hash = id.to_string();
                    info!("Updating pricing for blueprint: {}", blueprint_hash);

                    // Handle the blueprint update
                    if let Err(e) =
                        handle_blueprint_update(blueprint_hash, price_cache.clone(), config.clone())
                            .await
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
