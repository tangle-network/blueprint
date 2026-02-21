use alloy_primitives::Address;
use alloy_provider::{Provider, ProviderBuilder};
use blueprint_client_tangle::{TangleClientConfig, TangleSettings};
use blueprint_core::info;
use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::sync::mpsc;
use url::Url;

// Import functions from the library
use blueprint_pricing_engine_lib::{
    cleanup,
    error::{PricingError, Result},
    handle_blueprint_update, init_benchmark_cache, init_job_pricing_config, init_operator_signer,
    init_pricing_config, init_subscription_pricing_config, load_operator_config,
    service::blockchain::event::BlockchainEvent,
    service::rpc::server::run_rpc_server,
    signer::QuoteSigningDomain,
    spawn_event_processor, start_blockchain_listener, wait_for_shutdown,
};

/// Operator RFQ Pricing Engine Server CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the TOML operator configuration file.
    #[arg(short, long, value_name = "FILE", env = "OPERATOR_CONFIG_PATH")]
    pub config: PathBuf,

    /// Path to the resource + subscription pricing configuration file.
    #[arg(long, value_name = "FILE", env = "PRICING_CONFIG_PATH")]
    pub pricing_config: PathBuf,

    /// Path to the per-job pricing configuration file (job RFQ quotes).
    #[arg(long, value_name = "FILE", env = "JOB_PRICING_CONFIG_PATH")]
    pub job_pricing_config: PathBuf,

    /// HTTP RPC endpoint for the EVM chain.
    #[arg(long, value_name = "URL", env = "OPERATOR_HTTP_RPC")]
    pub http_rpc_endpoint: String,

    /// WebSocket RPC endpoint for the EVM chain.
    #[arg(long, value_name = "URL", env = "OPERATOR_WS_RPC")]
    pub ws_rpc_endpoint: String,

    #[arg(long, env = "OPERATOR_BLUEPRINT_ID")]
    pub blueprint_id: u64,

    #[arg(long, env = "OPERATOR_SERVICE_ID")]
    pub service_id: Option<u64>,

    #[arg(long, env = "OPERATOR_TANGLE_CONTRACT")]
    pub tangle_contract: String,

    #[arg(long, env = "OPERATOR_RESTAKING_CONTRACT")]
    pub restaking_contract: String,

    #[arg(long, env = "OPERATOR_STATUS_REGISTRY_CONTRACT")]
    pub status_registry_contract: String,

    /// Log level (e.g., info, debug, trace)
    #[arg(long, value_name = "LEVEL", env = "RUST_LOG", default_value = "info")]
    pub log_level: String,
}

/// Run the pricing engine application
pub async fn run_app(cli: Cli) -> Result<()> {
    info!("Starting Tangle Cloud Pricing Engine");

    // Load configuration (already returns Arc<OperatorConfig>)
    let config = load_operator_config(&cli.config).await?;

    let tangle_contract = parse_address(&cli.tangle_contract)?;
    let restaking_contract = parse_address(&cli.restaking_contract)?;
    let status_registry_contract = parse_address(&cli.status_registry_contract)?;

    let evm_settings = TangleSettings {
        blueprint_id: cli.blueprint_id,
        service_id: cli.service_id,
        tangle_contract,
        restaking_contract,
        status_registry_contract,
    };

    let http_rpc_endpoint = Url::parse(&cli.http_rpc_endpoint).map_err(|e| {
        PricingError::Config(format!(
            "invalid HTTP RPC endpoint {}: {e}",
            cli.http_rpc_endpoint
        ))
    })?;
    let ws_rpc_endpoint = Url::parse(&cli.ws_rpc_endpoint).map_err(|e| {
        PricingError::Config(format!(
            "invalid WS RPC endpoint {}: {e}",
            cli.ws_rpc_endpoint
        ))
    })?;

    let evm_config = TangleClientConfig::new(
        http_rpc_endpoint,
        ws_rpc_endpoint,
        config.keystore_path.to_string_lossy().to_string(),
        evm_settings,
    );

    let provider = ProviderBuilder::new()
        .connect(cli.http_rpc_endpoint.as_str())
        .await
        .map_err(|e| PricingError::Config(format!("failed to connect HTTP RPC: {e}")))?;
    let chain_id = provider
        .get_chain_id()
        .await
        .map_err(|e| PricingError::Config(format!("failed to read chain id: {e}")))?;

    // Create a channel for blockchain events
    let (event_tx, event_rx) = mpsc::channel::<BlockchainEvent>(100);

    // Start blockchain event listener
    let listener_handle = start_blockchain_listener(evm_config, event_tx).await;

    // Initialize benchmark cache
    let benchmark_cache = init_benchmark_cache(&config).await?;

    // Seed benchmark profile on startup if none exists for the configured blueprint
    match benchmark_cache.get_profile(cli.blueprint_id) {
        Ok(Some(_)) => {
            info!(
                "Benchmark profile already exists for blueprint {}",
                cli.blueprint_id
            );
        }
        _ => {
            info!(
                "No benchmark profile for blueprint {}, running initial benchmark...",
                cli.blueprint_id
            );
            if let Err(e) =
                handle_blueprint_update(cli.blueprint_id, benchmark_cache.clone(), config.clone())
                    .await
            {
                blueprint_core::error!(
                    "Initial benchmark failed for blueprint {}: {e}",
                    cli.blueprint_id
                );
            }
        }
    }

    // Initialize pricing configuration
    let pricing_config_path = cli.pricing_config.to_str().ok_or_else(|| {
        PricingError::Config("pricing config path is not valid UTF-8".to_string())
    })?;
    let pricing_config = init_pricing_config(pricing_config_path).await?;

    // Initialize per-job pricing configuration
    let job_pricing_config = init_job_pricing_config(&cli.job_pricing_config).await?;

    // Initialize subscription pricing from the same pricing config file.
    let subscription_config = init_subscription_pricing_config(pricing_config_path)?;

    // Initialize operator signer
    let operator_signer = init_operator_signer(
        &config,
        &config.keystore_path,
        QuoteSigningDomain {
            chain_id,
            verifying_contract: tangle_contract,
        },
    )?;
    info!("Operator signer initialized successfully");

    // Process blockchain events
    let _event_processor = spawn_event_processor(event_rx, benchmark_cache.clone(), config.clone());

    // Start the gRPC server
    let server_handle = tokio::spawn(async move {
        if let Err(e) = run_rpc_server(
            config,
            benchmark_cache,
            pricing_config,
            job_pricing_config,
            subscription_config,
            operator_signer,
        )
        .await
        {
            blueprint_core::error!("gRPC server error: {}", e);
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

fn parse_address(input: &str) -> Result<Address> {
    Address::from_str(input)
        .map_err(|e| PricingError::Config(format!("invalid address {}: {e}", input)))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Run the application
    run_app(cli).await
}
