use clap::Parser;
use std::path::PathBuf;
use tokio::sync::mpsc;
use tracing::info;

// Import functions from the library
use blueprint_pricing_engine_simple_lib::{
    cleanup, error::Result, init_benchmark_cache, init_logging, init_operator_signer,
    init_pricing_config, load_operator_config, service::blockchain::event::BlockchainEvent,
    service::rpc::server::run_rpc_server, spawn_event_processor, start_blockchain_listener,
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

    /// Path to the pricing configuration file.
    #[arg(
        long,
        value_name = "FILE",
        env = "PRICING_CONFIG_PATH",
        default_value = "config/default_pricing.toml"
    )]
    pub pricing_config: PathBuf,

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

    // Load configuration (already returns Arc<OperatorConfig>)
    let config = load_operator_config(&cli.config).await?;

    // Initialize benchmark cache
    let benchmark_cache = init_benchmark_cache(&config).await?;

    // Initialize pricing configuration
    let pricing_config = init_pricing_config(
        cli.pricing_config
            .to_str()
            .unwrap_or("config/default_pricing.toml"),
    )
    .await?;

    // Initialize operator signer
    let operator_signer = init_operator_signer(&config, &config.keystore_path)?;
    info!("Operator signer initialized successfully");

    // Process blockchain events
    let _event_processor = spawn_event_processor(event_rx, benchmark_cache.clone(), config.clone());

    // Start the gRPC server
    let server_handle = tokio::spawn(async move {
        if let Err(e) =
            run_rpc_server(config, benchmark_cache, pricing_config, operator_signer).await
        {
            tracing::error!("gRPC server error: {}", e);
        }
    });

    // Wait for shutdown signal
    wait_for_shutdown().await;

    // Cleanup and shutdown
    cleanup(listener_handle).await;

    // Abort the server
    server_handle.abort();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Run the application
    run_app(cli).await
}
