use clap::Parser;
use std::path::PathBuf;
use tokio::sync::mpsc;
use tracing::info;

// Import functions from the library
use blueprint_pricing_engine_simple_lib::{
    cleanup, error::Result, init_logging, init_price_cache, load_operator_config,
    service::blockchain::event::BlockchainEvent, spawn_event_processor, start_blockchain_listener,
    wait_for_shutdown,
};

/// Operator RFQ Pricing Engine Server CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the TOML configuration file.
    #[arg(
        short,
        long,
        value_name = "FILE",
        env = "OPERATOR_CONFIG_PATH",
        default_value = "config/operator.toml"
    )]
    pub config: PathBuf,

    /// Tangle node WebSocket URL (only used if 'tangle-listener' feature is enabled).
    #[arg(
        long,
        value_name = "URL",
        env = "OPERATOR_NODE_URL",
        default_value = "ws://127.0.0.1:9944"
    )]
    pub node_url: String,

    /// Log level (e.g., info, debug, trace)
    #[arg(long, value_name = "LEVEL", env = "RUST_LOG", default_value = "info")]
    pub log_level: String,
}

/// Run the pricing engine application
pub async fn run_app(cli: Cli) -> Result<()> {
    // Initialize logging
    init_logging(&cli.log_level);

    info!("Starting Tangle Cloud Pricing Engine");

    // Create a channel for blockchain events
    let (event_tx, event_rx) = mpsc::channel::<BlockchainEvent>(100);

    // Start blockchain event listener if the feature is enabled
    let listener_handle = start_blockchain_listener(cli.node_url.clone(), event_tx).await;

    // Load configuration
    let config = load_operator_config(&cli.config).await?;

    // Initialize price cache
    let price_cache = init_price_cache(&config).await?;

    // Skip operator signer initialization for now
    // @human-review: The operator signer initialization is commented out due to type inference issues.
    // A proper implementation would require a concrete KeyType implementation and a valid keypair.
    info!("Skipping operator signer initialization");

    // Process blockchain events
    let _event_processor = spawn_event_processor(event_rx, price_cache, config);

    // Wait for shutdown signal
    wait_for_shutdown().await;

    // Cleanup and shutdown
    cleanup(listener_handle).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Run the application
    run_app(cli).await
}
