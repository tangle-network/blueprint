use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::mpsc;
use tracing::{Level, error, info, warn};
use tracing_subscriber::EnvFilter;

// Use the library crate
use blueprint_pricing_engine_simple_lib::{
    cache::PriceCache,
    config::OperatorConfig,
    handlers::handle_blueprint_update,
    service::{
        blockchain::{event::BlockchainEvent, listener::EventListener},
        rpc::server::run_rpc_server,
    },
    signer::OperatorSigner,
};

/// Operator RFQ Pricing Engine Server CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the TOML configuration file.
    #[arg(
        short,
        long,
        value_name = "FILE",
        env = "OPERATOR_CONFIG_PATH",
        default_value = "config/operator.toml"
    )]
    config: PathBuf,

    /// Tangle node WebSocket URL (only used if 'tangle-listener' feature is enabled).
    #[arg(
        long,
        value_name = "URL",
        env = "OPERATOR_NODE_URL",
        default_value = "ws://127.0.0.1:9944"
    )]
    node_url: String,

    /// Log level (e.g., info, debug, trace)
    #[arg(long, value_name = "LEVEL", env = "RUST_LOG", default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging using EnvFilter and RUST_LOG/cli arg
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&cli.log_level))
        .unwrap_or_else(|_| EnvFilter::new("info")); // Fallback

    tracing_subscriber::fmt().with_env_filter(filter).init();

    info!("Starting Operator RFQ Pricing Server (binary)...");
    info!("Using configuration file: {:?}", cli.config);
    if cfg!(feature = "tangle-listener") {
        info!(
            "Tangle listener enabled. Connecting to node: {}",
            cli.node_url
        );
    } else {
        info!("Tangle listener feature is disabled.");
    }

    // 1. Load Configuration using the library function
    // Adjust load_config if it needs the path passed in
    let config = Arc::new(
        engine_lib::config::load_config_from_path(&cli.config).context(format!(
            "Failed to load configuration from {:?}",
            cli.config
        ))?,
    );
    info!("Configuration loaded successfully.");

    // 2. Initialize Cache
    let price_cache = Arc::new(PriceCache::new(&config.database_path).context(format!(
        "Failed to initialize price cache DB at {:?}",
        config.database_path
    ))?);
    info!("Price cache initialized.");

    // 3. Initialize Signer
    let operator_signer =
        Arc::new(OperatorSigner::new(&config).context("Failed to initialize operator signer")?);
    info!(
        "Operator signer initialized. Public Key: {}",
        hex::encode(operator_signer.public_key().to_bytes())
    );

    // -- Blockchain Listener Setup --
    let (listener_handle, event_processor_handle) = {
        // 4. Create Blockchain Event Channel
        let (event_tx, mut event_rx) = mpsc::channel::<BlockchainEvent>(100);

        // 5. Spawn Blockchain Event Processor Task
        let cache_clone = price_cache.clone();
        let config_clone = config.clone();
        let processor_handle = tokio::spawn(async move {
            info!("Starting blockchain event processor task...");
            while let Some(event) = event_rx.recv().await {
                info!("Received blockchain event: {:?}", event);
                let (blueprint_id_bytes, needs_update) = match &event {
                    BlockchainEvent::Registered(e) => (Some(e.blueprint_id.0), true),
                    BlockchainEvent::PriceTargetsUpdated(e) => (Some(e.blueprint_id.0), true),
                    BlockchainEvent::Unregistered(e) => {
                        warn!(
                            "Blueprint unregistered: {:?}. Removing price.",
                            e.blueprint_id
                        );
                        let blueprint_hash_hex = hex::encode(e.blueprint_id.0);
                        match cache_clone.remove_price(&blueprint_hash_hex) {
                            Ok(Some(_)) => info!(
                                "Removed price for unregistered blueprint {}",
                                blueprint_hash_hex
                            ),
                            Ok(None) => info!(
                                "No price found to remove for unregistered blueprint {}",
                                blueprint_hash_hex
                            ),
                            Err(err) => {
                                error!("Failed to remove price for {}: {}", blueprint_hash_hex, err)
                            }
                        }
                        (None, false)
                    }
                    _ => (None, false),
                };

                if needs_update {
                    if let Some(id_bytes) = blueprint_id_bytes {
                        let blueprint_hash_hex = hex::encode(id_bytes);
                        let cache_for_handler = cache_clone.clone();
                        let config_for_handler = config_clone.clone();
                        tokio::spawn(async move {
                            info!(
                                "Spawning handler task for blueprint: {}",
                                blueprint_hash_hex
                            );
                            match handle_blueprint_update(
                                blueprint_hash_hex.clone(),
                                cache_for_handler,
                                config_for_handler,
                            )
                            .await
                            {
                                Ok(_) => info!(
                                    "Successfully processed update for blueprint: {}",
                                    blueprint_hash_hex
                                ),
                                Err(e) => error!(
                                    "Error processing update for blueprint {}: {}",
                                    blueprint_hash_hex, e
                                ),
                            }
                        });
                    } else {
                        error!(
                            "Expected blueprint ID for event {:?}, but not found.",
                            event
                        );
                    }
                }
            }
            info!("Blockchain event channel closed. Processor task exiting.");
        });

        // 6. Initialize and Run Blockchain Listener
        let listener = EventListener::new(cli.node_url.clone(), event_tx)
            .await
            .context(format!(
                "Failed to create blockchain listener for {}",
                cli.node_url
            ))?;

        let handle = tokio::spawn(async move {
            if let Err(e) = listener.run().await {
                error!("Blockchain listener failed: {}", e);
            } else {
                info!("Blockchain listener stopped gracefully.");
            }
        });
        (Some(handle), Some(processor_handle))
    };

    // 7. Initialize and Run RPC Server
    let rpc_handle = tokio::spawn(run_rpc_server(
        config.clone(),
        price_cache.clone(),
        operator_signer.clone(),
    ));

    // 8. Wait for shutdown signal or task completion
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Ctrl-C received, shutting down.");
        }
        // Only select on listener/processor if they exist
        res = async { if let Some(h) = listener_handle { h.await } else { futures::future::pending().await } } => {
            error!("Blockchain listener task exited unexpectedly: {:?}", res);
        }
        res = async { if let Some(h) = event_processor_handle { h.await } else { futures::future::pending().await } } => {
             error!("Event processor task exited unexpectedly: {:?}", res);
        }
         res = rpc_handle => {
            error!("RPC server task exited unexpectedly: {:?}", res);
        }
    }

    info!("Shutdown complete.");
    Ok(())
}
