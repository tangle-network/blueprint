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
    init_benchmark_cache, init_job_pricing_config, init_operator_signer, init_pricing_config,
    load_operator_config,
    service::blockchain::event::BlockchainEvent,
    service::rpc::server::run_rpc_server,
    signer::QuoteSigningDomain,
    spawn_event_processor, start_blockchain_listener, wait_for_shutdown,
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

    /// Path to the resource pricing configuration file (service creation quotes).
    #[arg(
        long,
        value_name = "FILE",
        env = "PRICING_CONFIG_PATH",
        default_value = "config/default_pricing.toml"
    )]
    pub pricing_config: PathBuf,

    /// Path to the per-job pricing configuration file (job RFQ quotes).
    /// If not provided, GetJobPrice returns NOT_FOUND for all jobs.
    #[arg(long, value_name = "FILE", env = "JOB_PRICING_CONFIG_PATH")]
    pub job_pricing_config: Option<PathBuf>,

    /// Tangle node WebSocket URL (only used if 'tangle-listener' feature is enabled).
    #[arg(
        long,
        value_name = "URL",
        env = "OPERATOR_HTTP_RPC",
        default_value = "http://127.0.0.1:8545"
    )]
    pub http_rpc_endpoint: String,

    #[arg(
        long,
        value_name = "URL",
        env = "OPERATOR_WS_RPC",
        default_value = "ws://127.0.0.1:8545"
    )]
    pub ws_rpc_endpoint: String,

    #[arg(long, env = "OPERATOR_BLUEPRINT_ID", default_value_t = 0)]
    pub blueprint_id: u64,

    #[arg(long, env = "OPERATOR_SERVICE_ID")]
    pub service_id: Option<u64>,

    #[arg(
        long,
        env = "OPERATOR_TANGLE_CONTRACT",
        default_value = "0x0000000000000000000000000000000000000000"
    )]
    pub tangle_contract: String,

    #[arg(
        long,
        env = "OPERATOR_RESTAKING_CONTRACT",
        default_value = "0x0000000000000000000000000000000000000000"
    )]
    pub restaking_contract: String,

    #[arg(
        long,
        env = "OPERATOR_STATUS_REGISTRY_CONTRACT",
        default_value = "0x0000000000000000000000000000000000000000"
    )]
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

    if tangle_contract == Address::ZERO {
        return Err(PricingError::Config(
            "missing OPERATOR_TANGLE_CONTRACT (required for EIP-712 quote signatures)".to_string(),
        ));
    }

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

    // Initialize pricing configuration
    let pricing_config = init_pricing_config(
        cli.pricing_config
            .to_str()
            .unwrap_or("config/default_pricing.toml"),
    )
    .await?;

    // Initialize per-job pricing configuration (optional)
    let job_pricing_config = match &cli.job_pricing_config {
        Some(path) => Some(init_job_pricing_config(path).await?),
        None => None,
    };

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
