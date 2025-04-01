//! Tangle Cloud Pricing Engine Service
//!
//! A standalone service for the Tangle Cloud Pricing Engine.

use blueprint_crypto::sp_core::{SpSr25519, SpSr25519Public};
use blueprint_keystore::backends::Backend;
use blueprint_runner::config::BlueprintEnvironment;
use clap::Parser;
use sp_core::crypto::Ss58Codec;
use std::{net::SocketAddr, process::exit};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

// Import from the local crate instead of an external one
use blueprint_pricing_engine::{
    Service, ServiceConfig,
    error::{Error, Result},
    models::{PricingModel, PricingModelType},
    types::{Price, TimePeriod},
};

#[derive(Debug, Parser)]
#[command(
    name = "pricing-service",
    about = "Tangle Cloud Pricing Engine Service",
    version
)]
struct Cli {
    /// WebSocket URL of the Tangle node to connect to
    #[arg(long, default_value = "ws://127.0.0.1:9944")]
    node_url: Option<String>,

    /// JSON-RPC server listen address
    #[arg(long, default_value = "127.0.0.1:9955")]
    rpc_addr: String,

    /// Path to the keystore for signing transactions
    #[arg(long)]
    keystore_path: Option<String>,

    /// Operator name
    #[arg(long, default_value = "Tangle Cloud Operator")]
    operator_name: String,

    /// Operator description
    #[arg(long)]
    operator_description: Option<String>,

    /// Operator public key (on-chain identity)
    #[arg(long)]
    operator_public_key: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Parse command line arguments
    let cli = Cli::parse();

    let env = BlueprintEnvironment::load().map_err(|e| {
        error!("Failed to load environment: {}", e);
        Error::Other(format!("Failed to load environment: {}", e))
    })?;

    let keystore = env.keystore();
    let public_key = keystore.first_local::<SpSr25519>().map_err(|e| {
        error!("Failed to get signing key: {}", e);
        Error::Other(format!("Failed to get signing key: {}", e))
    })?;
    let signing_key = keystore.get_secret::<SpSr25519>(&public_key).map_err(|e| {
        error!("Failed to get signing key: {}", e);
        Error::Other(format!("Failed to get signing key: {}", e))
    })?;

    info!("Starting Tangle Cloud Pricing Engine Service");
    info!("Operator name: {}", cli.operator_name);
    info!("RPC address: {}", cli.rpc_addr);
    info!("Node URL: {:?}", cli.node_url);
    info!("Keystore path: {:?}", cli.keystore_path);

    // Parse the RPC address
    let rpc_addr: SocketAddr = cli.rpc_addr.parse().map_err(|e| {
        error!("Failed to parse RPC address: {}", e);
        Error::Other(format!("Invalid RPC address: {}", e))
    })?;

    // Create some example pricing models
    let models = create_example_pricing_models();

    // Create operator key - in a real implementation, this would be loaded from the keystore
    let operator_public_key_str = cli.operator_public_key.unwrap_or_else(|| {
        // Use a placeholder key if none provided
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string()
    });

    let operator_public_key_bytes =
        sp_core::sr25519::Public::from_ss58check(&operator_public_key_str).map_err(|e| {
            error!("Failed to parse operator public key: {}", e);
            Error::Other(format!("Failed to parse operator public key: {}", e))
        })?;

    let operator_public_key = SpSr25519Public(operator_public_key_bytes);

    // Create the service configuration
    let config = ServiceConfig::<SpSr25519> {
        rpc_addr,
        node_url: cli.node_url,
        keystore_path: cli.keystore_path,
        operator_name: cli.operator_name,
        operator_description: cli.operator_description,
        operator_public_key,
        supported_blueprints: models.iter().map(|m| m.blueprint_id.clone()).collect(),
        network_handle: None, // No network handle for this example
    };

    // Create and start the service
    let mut service = Service::new(config, models, signing_key);
    if let Err(e) = service.start().await {
        error!("Failed to start service: {}", e);
        exit(1);
    }

    info!("Service stopped");
    Ok(())
}

/// Create example pricing models for demonstration
fn create_example_pricing_models() -> Vec<PricingModel> {
    // Define pricing periods and token
    let hour = TimePeriod::Hour;
    let month = TimePeriod::Month;
    let token = "TNT".to_string();

    vec![
        // Basic compute model
        PricingModel {
            model_type: PricingModelType::Fixed,
            name: "Basic Compute".to_string(),
            description: Some("Low-cost compute resources".to_string()),
            blueprint_id: "compute.basic".to_string(), // Blueprint ID instead of category
            base_price: Some(Price {
                value: 1_000_000_000_000_000_000,
                token: token.clone(),
            }),
            resource_pricing: Vec::new(),
            billing_period: Some(hour),
        },
        // Premium compute model
        PricingModel {
            model_type: PricingModelType::Fixed,
            name: "Premium Compute".to_string(),
            description: Some("High-performance compute resources".to_string()),
            blueprint_id: "compute.premium".to_string(), // Blueprint ID instead of category
            base_price: Some(Price {
                value: 2_500_000_000_000_000_000,
                token: token.clone(),
            }),
            resource_pricing: Vec::new(),
            billing_period: Some(hour),
        },
        // Basic storage model
        PricingModel {
            model_type: PricingModelType::Fixed,
            name: "Basic Storage".to_string(),
            description: Some("Standard storage solution".to_string()),
            blueprint_id: "storage.basic".to_string(), // Blueprint ID instead of category
            base_price: Some(Price {
                value: 5_000_000_000_000_000_000,
                token: token.clone(),
            }),
            resource_pricing: Vec::new(),
            billing_period: Some(month),
        },
        // Premium storage model
        PricingModel {
            model_type: PricingModelType::Fixed,
            name: "Premium Storage".to_string(),
            description: Some("High-speed SSD storage".to_string()),
            blueprint_id: "storage.premium".to_string(), // Blueprint ID instead of category
            base_price: Some(Price {
                value: 15_000_000_000_000_000_000,
                token: token.clone(),
            }),
            resource_pricing: Vec::new(),
            billing_period: Some(month),
        },
    ]
}
