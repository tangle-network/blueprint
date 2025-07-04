use crate::error::Error;
use crate::state::{AnvilState, get_default_state_json};
use alloy_contract::{CallBuilder, CallDecoder};
use alloy_provider::Provider;
use alloy_provider::network::Ethereum;
use alloy_rpc_types_eth::TransactionReceipt;
use blueprint_core::{error, info};
use std::fs;
use tempfile::TempDir;
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{ExecCommand, IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};
use tokio::io::AsyncBufReadExt;
use url::Url;

pub type Container = ContainerAsync<GenericImage>;

pub const ANVIL_IMAGE: &str = "ghcr.io/foundry-rs/foundry";
pub const ANVIL_TAG: &str = "nightly-5b7e4cb3c882b28f3c32ba580de27ce7381f415a";

pub struct AnvilTestnet {
    pub container: Container,
    pub http_endpoint: Url,
    pub ws_endpoint: Url,
    pub temp_dir: TempDir,
}

/// Start an Anvil container for testing with contract state loaded.
#[allow(clippy::missing_panics_doc)] // TODO(serial): Return errors, not panics
pub async fn start_anvil_container(state_json: Option<&str>, include_logs: bool) -> AnvilTestnet {
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let container = if let Some(state_json) = state_json {
        // Create a temporary directory and write the state file
        let state_path = temp_dir.path().join("state.json");
        fs::write(&state_path, state_json).expect("Failed to write state file");

        GenericImage::new(ANVIL_IMAGE, ANVIL_TAG)
            .with_wait_for(WaitFor::message_on_stdout("Listening on"))
            .with_exposed_port(8545.tcp())
            .with_entrypoint("anvil")
            .with_mount(testcontainers::core::Mount::bind_mount(
                state_path.to_str().unwrap(),
                "/state.json",
            ))
            .with_cmd([
                "--host",
                "0.0.0.0",
                "--load-state",
                "/state.json",
                "--base-fee",
                "0",
                "--gas-price",
                "0",
                "--code-size-limit",
                "50000",
                "--hardfork",
                "shanghai",
            ])
            .start()
            .await
            .expect("Error starting anvil container")
    } else {
        GenericImage::new(ANVIL_IMAGE, ANVIL_TAG)
            .with_wait_for(WaitFor::message_on_stdout("Listening on"))
            .with_exposed_port(8545.tcp())
            .with_entrypoint("anvil")
            .with_cmd([
                "--host",
                "0.0.0.0",
                "--base-fee",
                "0",
                "--gas-price",
                "0",
                "--code-size-limit",
                "50000",
                "--hardfork",
                "shanghai",
            ])
            .start()
            .await
            .expect("Error starting anvil container")
    };

    if include_logs {
        let reader = container.stdout(true);
        tokio::task::spawn(async move {
            let mut reader = reader;
            let mut buffer = String::new();
            while reader.read_line(&mut buffer).await.unwrap() > 0 {
                info!("{:?}", buffer);
                buffer.clear();
            }
        });
    }

    mine_anvil_blocks(&container, 200).await;

    let port = container
        .ports()
        .await
        .unwrap()
        .map_to_host_port_ipv4(8545)
        .unwrap();

    let http_endpoint = format!("http://localhost:{port}").parse().unwrap();
    println!("Anvil HTTP endpoint: {}", http_endpoint);
    let ws_endpoint = format!("ws://localhost:{port}").parse().unwrap();
    println!("Anvil WS endpoint: {}", ws_endpoint);

    AnvilTestnet {
        container,
        http_endpoint,
        ws_endpoint,
        temp_dir,
    }
}

/// Mine Anvil blocks.
#[allow(clippy::missing_panics_doc)] // TODO(serial): Return errors, not panics
pub async fn mine_anvil_blocks(container: &Container, n: u32) {
    let _output = container
        .exec(ExecCommand::new([
            "cast",
            "rpc",
            "anvil_mine",
            n.to_string().as_str(),
        ]))
        .await
        .expect("Failed to mine anvil blocks");
}

/// Starts an Anvil container for testing with the default state.
///
/// # Arguments
/// * `include_logs` - If true, testnet output will be printed to the console.
pub async fn start_default_anvil_testnet(include_logs: bool) -> AnvilTestnet {
    start_anvil_container(Some(get_default_state_json()), include_logs).await
}

/// Starts an Anvil container for testing with the default state.
///
/// # Arguments
/// * `include_logs` - If true, testnet output will be printed to the console.
pub async fn start_empty_anvil_testnet(include_logs: bool) -> AnvilTestnet {
    start_anvil_container(None, include_logs).await
}

/// Starts an Anvil container for testing with custom state.
///
/// # Arguments
/// * `state` - The state to load into Anvil.
/// * `include_logs` - If true, testnet output will be printed to the console.
#[allow(clippy::missing_panics_doc)] // TODO(serial): Return errors, not panics
pub async fn start_anvil_testnet_with_state(
    state: &AnvilState,
    include_logs: bool,
) -> AnvilTestnet {
    let state_json = serde_json::to_string(state).expect("Failed to serialize state");
    start_anvil_container(Some(&state_json), include_logs).await
}

#[allow(clippy::missing_errors_doc)] // TODO: should this even be public?
pub async fn get_receipt<T, P, D>(
    call: CallBuilder<T, P, D, Ethereum>,
) -> Result<TransactionReceipt, Error>
where
    P: Provider<Ethereum>,
    D: CallDecoder,
{
    let pending_tx = match call.send().await {
        Ok(tx) => tx,
        Err(e) => {
            error!("Failed to send transaction: {:?}", e);
            return Err(e.into());
        }
    };

    let receipt = match pending_tx.get_receipt().await {
        Ok(receipt) => receipt,
        Err(e) => {
            error!("Failed to get transaction receipt: {:?}", e);
            return Err(e.into());
        }
    };

    Ok(receipt)
}
