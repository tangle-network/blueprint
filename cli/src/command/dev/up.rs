//! `cargo-tangle dev up` — boot a local Anvil devnet, pre-register the seeded operator,
//! and write a `.tangle.toml` so every other cargo-tangle command works without arguments.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::time::{Duration, Instant};


use alloy_network::EthereumWallet;
use alloy_primitives::Address;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types_eth::TransactionRequest;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::SolCall;
use blueprint_client_tangle::contracts::ITangle::addPermittedCallerCall;
use blueprint_testing_utils::anvil::tangle::insert_default_operator_key;
use clap::Args;
use color_eyre::eyre::{Context, Result, bail, eyre};
use tokio::time::sleep;
use url::Url;

use crate::workspace::{Defaults, Network, TangleWorkspace, WORKSPACE_FILE};

// These match the values seeded into the LocalTestnet broadcast/snapshot bundled
// with blueprint-chain-setup-anvil. Keep in sync with
// crates/testing-utils/anvil/src/tangle.rs.
const TANGLE_CONTRACT: &str = "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9";
const RESTAKING_CONTRACT: &str = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512";
const STATUS_REGISTRY_CONTRACT: &str = "0x8f86403A4DE0bb5791fa46B8e795C547942fE4Cf";
const OPERATOR1_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
const SERVICE_OWNER_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const CHAIN_ID: u64 = 31_337;
const DEFAULT_SERVICE_ID: u64 = 0;
const DEFAULT_BLUEPRINT_ID: u64 = 0;

const DEV_DIR: &str = ".tangle/dev";

#[derive(Args, Debug)]
pub struct UpArgs {
    /// Port for the Anvil HTTP/WS RPC.
    #[arg(long, default_value_t = 8545)]
    pub port: u16,
    /// Skip granting `addPermittedCaller` for the seeded operator. You will need
    /// to do this yourself before the operator can submit jobs on its own behalf.
    #[arg(long)]
    pub no_grant_caller: bool,
    /// Stream Anvil stdout/stderr to `.tangle/dev/anvil.log` even if debug is off.
    #[arg(long)]
    pub anvil_logs: bool,
    /// Overwrite an existing `.tangle.toml` in the current directory.
    #[arg(long)]
    pub force: bool,
}

pub async fn execute(args: UpArgs) -> Result<()> {
    if !args.force && Path::new(WORKSPACE_FILE).exists() {
        bail!(
            "{WORKSPACE_FILE} already exists. Use --force to overwrite, or `cargo-tangle dev down` to tear down an existing devnet first."
        );
    }

    ensure_anvil_on_path()?;

    let dev_dir = PathBuf::from(DEV_DIR);
    fs::create_dir_all(&dev_dir).with_context(|| format!("creating {}", dev_dir.display()))?;
    let anvil_pid_file = dev_dir.join("anvil.pid");
    let anvil_log_path = dev_dir.join("anvil.log");
    let keystore_dir = dev_dir.join("keystore");

    // Refuse to clobber a running devnet.
    if let Some(pid) = read_pid(&anvil_pid_file) {
        if pid_alive(pid) {
            bail!(
                "anvil already running (PID {pid}) from a previous `dev up`. Run `cargo-tangle dev down` first."
            );
        }
    }

    let snapshot = locate_snapshot()?;
    println!("→ Anvil snapshot: {}", snapshot.display());

    // 1. Launch anvil detached. We don't use testcontainers here so the chain
    //    survives past the lifetime of this process.
    let anvil_log = fs::File::create(&anvil_log_path)
        .with_context(|| format!("creating {}", anvil_log_path.display()))?;
    let anvil_err = anvil_log
        .try_clone()
        .with_context(|| "duplicating anvil log file")?;

    let mut cmd = Command::new("anvil");
    cmd.arg("--load-state")
        .arg(&snapshot)
        .args(["--host", "127.0.0.1"])
        .args(["--port", &args.port.to_string()])
        .args(["--base-fee", "0"])
        .args(["--gas-price", "0"])
        .args(["--gas-limit", "100000000"])
        .args(["--hardfork", "cancun"])
        .stdin(Stdio::null())
        .stdout(Stdio::from(anvil_log))
        .stderr(Stdio::from(anvil_err));

    let anvil = cmd.spawn().with_context(|| "launching anvil")?;
    let anvil_pid = anvil.id();
    fs::write(&anvil_pid_file, anvil_pid.to_string())
        .with_context(|| format!("writing {}", anvil_pid_file.display()))?;

    // Detach — let the child survive our exit. We only needed it to get the PID.
    std::mem::forget(anvil);

    let http_rpc_url = Url::parse(&format!("http://127.0.0.1:{}", args.port))
        .expect("literal URL is valid");
    let ws_rpc_url =
        Url::parse(&format!("ws://127.0.0.1:{}", args.port)).expect("literal URL is valid");

    // 2. Wait for the chain to respond with a block number past the seeded state.
    let seeded_block = wait_for_seeded_chain(&http_rpc_url).await.map_err(|e| {
        // If readiness fails, best-effort kill what we just spawned.
        let _ = kill_pid(anvil_pid);
        let _ = fs::remove_file(&anvil_pid_file);
        e
    })?;
    println!("✓ Anvil ready at {http_rpc_url} (block {seeded_block})");

    // 3. Seed the keystore with the well-known operator1 key.
    fs::create_dir_all(&keystore_dir)
        .with_context(|| format!("creating {}", keystore_dir.display()))?;
    let keystore = blueprint_keystore::Keystore::new(
        blueprint_keystore::KeystoreConfig::new().fs_root(&keystore_dir),
    )
    .map_err(|e| eyre!(e.to_string()))?;
    insert_default_operator_key(&keystore).map_err(|e| eyre!(e.to_string()))?;
    println!("✓ Operator keystore at {}", keystore_dir.display());

    let tangle_contract = Address::from_str(TANGLE_CONTRACT).expect("literal address");
    let restaking_contract = Address::from_str(RESTAKING_CONTRACT).expect("literal address");
    let status_registry_contract =
        Address::from_str(STATUS_REGISTRY_CONTRACT).expect("literal address");
    let operator1 = Address::from_str(OPERATOR1_ADDRESS).expect("literal address");

    // 4. Grant addPermittedCaller so the operator can submit its own jobs.
    if !args.no_grant_caller {
        grant_permitted_caller(&http_rpc_url, tangle_contract, DEFAULT_SERVICE_ID, operator1)
            .await
            .with_context(|| "granting addPermittedCaller")?;
        println!("✓ Permitted caller granted for service {DEFAULT_SERVICE_ID} -> {operator1}");
    } else {
        println!("  Skipped addPermittedCaller (--no-grant-caller).");
    }

    // 5. Write .tangle.toml in the current working directory.
    let mut networks = HashMap::new();
    networks.insert(
        "local".to_string(),
        Network {
            http_rpc_url: http_rpc_url.clone(),
            ws_rpc_url: ws_rpc_url.clone(),
            tangle_contract,
            restaking_contract,
            status_registry_contract: Some(status_registry_contract),
            chain_id: Some(CHAIN_ID),
        },
    );
    let ws = TangleWorkspace {
        source: PathBuf::from(WORKSPACE_FILE),
        active: "local".to_string(),
        networks,
        defaults: Defaults {
            keystore_path: Some(keystore_dir.clone()),
            blueprint_id: Some(DEFAULT_BLUEPRINT_ID),
            service_id: Some(DEFAULT_SERVICE_ID),
        },
    };
    ws.write()
        .with_context(|| format!("writing {}", WORKSPACE_FILE))?;
    println!("✓ Workspace written to {WORKSPACE_FILE}");

    println!();
    println!("Devnet is up.");
    println!("  anvil PID        {anvil_pid}  (log: {})", anvil_log_path.display());
    println!("  http_rpc_url     {http_rpc_url}");
    println!("  ws_rpc_url       {ws_rpc_url}");
    println!("  chain_id         {CHAIN_ID}");
    println!("  tangle_contract  {tangle_contract}");
    println!("  keystore         {}", keystore_dir.display());
    println!();
    println!("Try: cargo-tangle blueprint jobs submit --job 0 --payload-hex 000000000000000000000000000000000000000000000000000000000000002a --watch");
    println!("Stop with: cargo-tangle dev down");
    Ok(())
}

fn ensure_anvil_on_path() -> Result<()> {
    let status = Command::new("anvil")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match status {
        Ok(s) if s.success() => Ok(()),
        _ => Err(eyre!(
            "`anvil` not found or not runnable on PATH. Install Foundry: `curl -L https://foundry.paradigm.xyz | bash && foundryup`"
        )),
    }
}

fn locate_snapshot() -> Result<PathBuf> {
    // Re-exported by the chain-setup crate so callers don't need to touch paths.
    use blueprint_chain_setup::anvil::snapshot::default_snapshot_path;
    let path = default_snapshot_path();
    if !path.is_file() {
        bail!(
            "Anvil snapshot not found at {}. Rebuild the blueprint-sdk workspace or ensure the crate's snapshots/ directory is populated.",
            path.display()
        );
    }
    Ok(path)
}

async fn wait_for_seeded_chain(http_rpc_url: &Url) -> Result<u64> {
    let deadline = Instant::now() + Duration::from_secs(15);
    let mut last_err: Option<color_eyre::eyre::Report> = None;
    while Instant::now() < deadline {
        match try_block_number(http_rpc_url).await {
            Ok(block) if block >= 200 => return Ok(block),
            Ok(block) => {
                last_err = Some(eyre!(
                    "anvil responded but seeded state looks incomplete (block {block} < 200)"
                ));
            }
            Err(e) => last_err = Some(e),
        }
        sleep(Duration::from_millis(250)).await;
    }
    Err(last_err.unwrap_or_else(|| eyre!("anvil did not become ready in 15s")))
}

async fn try_block_number(http_rpc_url: &Url) -> Result<u64> {
    let provider = ProviderBuilder::new()
        .connect(http_rpc_url.as_str())
        .await
        .with_context(|| format!("connecting to {http_rpc_url}"))?;
    let block = provider
        .get_block_number()
        .await
        .with_context(|| "fetching block number")?;
    Ok(block)
}

async fn grant_permitted_caller(
    http_rpc_url: &Url,
    tangle: Address,
    service_id: u64,
    caller: Address,
) -> Result<()> {
    let signer = PrivateKeySigner::from_str(SERVICE_OWNER_PRIVATE_KEY)
        .with_context(|| "decoding service-owner key")?;
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(http_rpc_url.as_str())
        .await
        .with_context(|| "connecting with service-owner wallet")?;

    let call = addPermittedCallerCall {
        serviceId: service_id,
        caller,
    };
    let tx = TransactionRequest::default()
        .to(tangle)
        .input(call.abi_encode().into());

    let pending = provider
        .send_transaction(tx)
        .await
        .with_context(|| "sending addPermittedCaller tx")?;
    let receipt = pending
        .get_receipt()
        .await
        .with_context(|| "awaiting addPermittedCaller receipt")?;
    if !receipt.status() {
        bail!("addPermittedCaller reverted on-chain");
    }
    Ok(())
}

fn read_pid(path: &Path) -> Option<u32> {
    fs::read_to_string(path).ok().and_then(|s| s.trim().parse().ok())
}

fn pid_alive(pid: u32) -> bool {
    use nix::sys::signal::kill;
    use nix::unistd::Pid;
    // signal None is a liveness check (ESRCH iff the process is gone).
    kill(Pid::from_raw(pid as i32), None).is_ok()
}

fn kill_pid(pid: u32) -> Result<()> {
    use nix::errno::Errno;
    use nix::sys::signal::{Signal, kill};
    use nix::unistd::Pid;
    match kill(Pid::from_raw(pid as i32), Signal::SIGTERM) {
        Ok(()) | Err(Errno::ESRCH) => Ok(()),
        Err(e) => Err(eyre!("SIGTERM to {pid} failed: {e}")),
    }
}
