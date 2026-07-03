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
/// Anvil image for the current `localtestnet-state.json` snapshot. The dump was
/// produced by anvil 1.7.x and includes full block/receipt history, which the
/// event-sourced client paths (`eth_getLogs`) require the container to serve.
pub const ANVIL_TAG: &str = "v1.7.1";
/// Anvil image for the legacy environments: `data/state.json` is a pre-1.7
/// dump newer anvil refuses to parse, and the empty testnet keeps this image
/// because v1.7.1's server-side gas fill under-provisions contract-creation
/// txs sent via `eth_sendTransaction` from filler-less alloy providers
/// (observed: EigenLayer stack deploys OOG at the filled limit).
pub const ANVIL_TAG_LEGACY: &str = "nightly-5b7e4cb3c882b28f3c32ba580de27ce7381f415a";

pub struct AnvilTestnet {
    pub container: Container,
    pub http_endpoint: Url,
    pub ws_endpoint: Url,
    pub temp_dir: TempDir,
}

/// Start an Anvil container for testing with contract state loaded.
/// Includes retry logic for transient Docker errors (e.g., image pull failures).
#[allow(clippy::missing_panics_doc)] // TODO(serial): Return errors, not panics
pub async fn start_anvil_container(state_json: Option<&str>, include_logs: bool) -> AnvilTestnet {
    // No hardfork override: the current snapshot was dumped under the image's
    // default hardfork. Forcing an older fork onto a newer-fork dump makes
    // gas estimation and execution disagree (observed as spurious OutOfGas).
    start_anvil_container_with_tag(ANVIL_TAG, None, state_json, include_logs).await
}

/// Start an Anvil container from a specific foundry image tag. The tag (and
/// hardfork override, when set) must match the supplied state dump — dump
/// formats are not forward compatible across anvil majors, so both travel
/// with the state artifact.
#[allow(clippy::missing_panics_doc)] // TODO(serial): Return errors, not panics
pub async fn start_anvil_container_with_tag(
    anvil_tag: &str,
    hardfork: Option<&str>,
    state_json: Option<&str>,
    include_logs: bool,
) -> AnvilTestnet {
    const MAX_RETRIES: u32 = 3;
    const RETRY_DELAY_MS: u64 = 2000;

    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let state_path = temp_dir.path().join("state.json");

    if let Some(json) = state_json {
        fs::write(&state_path, json).expect("Failed to write state file");
    }

    let mut cmd: Vec<String> = vec!["--host".into(), "0.0.0.0".into()];
    if state_json.is_some() {
        cmd.extend(["--load-state".into(), "/state.json".into()]);
    }
    cmd.extend([
        "--base-fee".into(),
        "0".into(),
        "--gas-price".into(),
        "0".into(),
        "--gas-limit".into(),
        "100000000".into(),
        "--code-size-limit".into(),
        "100000".into(),
    ]);
    if let Some(fork) = hardfork {
        cmd.extend(["--hardfork".into(), fork.into()]);
    }

    let mut last_error = String::new();
    for attempt in 1..=MAX_RETRIES {
        let image = GenericImage::new(ANVIL_IMAGE, anvil_tag)
            .with_wait_for(WaitFor::message_on_stdout("Listening on"))
            .with_exposed_port(8545.tcp())
            .with_entrypoint("anvil");
        let result = if state_json.is_some() {
            image
                .with_mount(testcontainers::core::Mount::bind_mount(
                    state_path.to_str().unwrap(),
                    "/state.json",
                ))
                .with_cmd(cmd.clone())
                .start()
                .await
        } else {
            image.with_cmd(cmd.clone()).start().await
        };

        match result {
            Ok(container) => {
                if include_logs {
                    let reader = container.stdout(true);
                    tokio::task::spawn(async move {
                        let mut reader = reader;
                        let mut buffer = String::new();
                        while reader.read_line(&mut buffer).await.unwrap_or(0) > 0 {
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
                println!("Anvil HTTP endpoint: {http_endpoint}");
                let ws_endpoint = format!("ws://localhost:{port}").parse().unwrap();
                println!("Anvil WS endpoint: {ws_endpoint}");

                return AnvilTestnet {
                    container,
                    http_endpoint,
                    ws_endpoint,
                    temp_dir,
                };
            }
            Err(e) => {
                last_error = format!("{e}");
                if attempt < MAX_RETRIES {
                    error!(
                        "Anvil container start attempt {attempt}/{MAX_RETRIES} failed: {e}. Retrying in {RETRY_DELAY_MS}ms..."
                    );
                    tokio::time::sleep(std::time::Duration::from_millis(RETRY_DELAY_MS)).await;
                }
            }
        }
    }

    panic!(
        "Failed to start Anvil container after {MAX_RETRIES} attempts. Last error: {last_error}"
    );
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
    // data/state.json is a legacy-format dump; newer anvil refuses to parse it,
    // and it was built under cancun rules.
    start_anvil_container_with_tag(
        ANVIL_TAG_LEGACY,
        Some("cancun"),
        Some(get_default_state_json()),
        include_logs,
    )
    .await
}

/// Starts an empty Anvil container (funded default accounts, no seeded state).
///
/// Callers that need the seeded Tangle localtestnet go through
/// `blueprint_anvil_testing_utils::TangleHarness`, which loads the snapshot
/// explicitly. Booting the seeded chain here made fresh-deployment flows
/// (e.g. the EigenLayer stack in client/manager tests) run on a dirty chain
/// for no benefit.
///
/// Runs the legacy image: fresh-deployment flows built on filler-less alloy
/// providers rely on server-side gas fill, which v1.7.1 under-provisions for
/// creation txs (see [`ANVIL_TAG_LEGACY`]).
///
/// # Arguments
/// * `include_logs` - If true, testnet output will be printed to the console.
pub async fn start_empty_anvil_testnet(include_logs: bool) -> AnvilTestnet {
    start_anvil_container_with_tag(ANVIL_TAG_LEGACY, Some("cancun"), None, include_logs).await
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
pub async fn get_receipt<P, D>(
    call: CallBuilder<P, D, Ethereum>,
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
