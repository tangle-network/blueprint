//! Utilities for spinning up an Anvil testnet with the full Tangle EVM stack deployed.
//!
//! These helpers replay the broadcast artifacts generated from
//! `tnt-core/script/v2/LocalTestnet.s.sol` so all tests run against the same contract
//! addresses the SDK expects in production. The broadcast file is bundled with the SDK
//! and can be overridden via `TNT_BROADCAST_PATH` if needed.

use alloy_primitives::{Address, TxKind};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::TransactionRequest;
use anyhow::{Context, Result};
use blueprint_chain_setup_anvil::{AnvilTestnet, snapshot_available, start_empty_anvil_testnet};
use blueprint_client_tangle_evm::{
    ServiceStatus, TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings,
};
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use hex::FromHex;
use serde::Deserialize;
use serde_json::{self, Value, json};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use thiserror::Error;
use tokio::time::{Duration, sleep};
use url::Url;

/// Default blueprint/service IDs baked into the local testnet script.
pub const LOCAL_BLUEPRINT_ID: u64 = 0;
pub const LOCAL_SERVICE_ID: u64 = 0;

const TANGLE_ADDRESS: &str = "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9";
const RESTAKING_ADDRESS: &str = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512";
const STATUS_REGISTRY_ADDRESS: &str = "0xdC64a140Aa3E981100a9BecA4E685f962f0CF6C9";
const DEFAULT_FEE_WEI: u128 = 1;
const DEFAULT_GAS_LIMIT: u64 = 120_000_000;
const OPERATOR1_PRIVATE_KEY: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

/// Errors raised while preparing the deterministic harness.
#[derive(Debug, Error)]
pub enum HarnessError {
    #[error("LocalTestnet broadcast artifact missing at {0}. Set TNT_BROADCAST_PATH to override.")]
    MissingBroadcast(PathBuf),
}

/// Returns `true` if the provided error was caused by missing TNT core artifacts.
#[must_use]
pub fn missing_tnt_core_artifacts(err: &anyhow::Error) -> bool {
    err.chain().any(|cause| {
        cause
            .downcast_ref::<HarnessError>()
            .map_or(false, |harness_err| {
                matches!(harness_err, HarnessError::MissingBroadcast(_))
            })
    })
}

/// Deterministic Tangle EVM harness backed by Anvil.
pub struct TangleEvmHarness {
    pub testnet: AnvilTestnet,
    pub tangle_contract: Address,
    pub restaking_contract: Address,
    pub status_registry_contract: Address,
}

pub type SeededTangleEvmTestnet = TangleEvmHarness;

/// Build the canonical harness configured entirely via env vars.
///
/// Callers should prefer this helper over [`TangleEvmHarness::builder`] so new
/// knobs automatically fan out. Today it honors `BLUEPRINT_ANVIL_LOGS=1` to
/// stream Anvil stdout/stderr and can grow additional env-based settings in the
/// future without touching every test.
#[must_use]
pub fn harness_builder_from_env() -> TangleEvmHarnessBuilder {
    TangleEvmHarness::builder().include_anvil_logs(true)
}

impl fmt::Debug for TangleEvmHarness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TangleEvmHarness")
            .field("http_endpoint", &self.testnet.http_endpoint)
            .field("ws_endpoint", &self.testnet.ws_endpoint)
            .field("tangle_contract", &self.tangle_contract)
            .field("restaking_contract", &self.restaking_contract)
            .field("status_registry_contract", &self.status_registry_contract)
            .finish()
    }
}

impl TangleEvmHarness {
    /// Construct a builder with default settings (headless, seeded from broadcast).
    #[must_use]
    pub fn builder() -> TangleEvmHarnessBuilder {
        TangleEvmHarnessBuilder::default()
    }

    /// Convenience API used by older helpers.
    pub async fn start(seed_from_broadcast: bool) -> Result<Self> {
        Self::builder()
            .seed_from_broadcast(seed_from_broadcast)
            .spawn()
            .await
    }

    /// HTTP endpoint exposed by the underlying Anvil instance.
    #[must_use]
    pub fn http_endpoint(&self) -> &Url {
        &self.testnet.http_endpoint
    }

    /// WS endpoint exposed by the underlying Anvil instance.
    #[must_use]
    pub fn ws_endpoint(&self) -> &Url {
        &self.testnet.ws_endpoint
    }

    async fn assert_seeded_service_active(&self) -> Result<()> {
        let settings = TangleEvmSettings {
            blueprint_id: LOCAL_BLUEPRINT_ID,
            service_id: Some(LOCAL_SERVICE_ID),
            tangle_contract: self.tangle_contract,
            restaking_contract: self.restaking_contract,
            status_registry_contract: self.status_registry_contract,
        };

        let config = TangleEvmClientConfig::new(
            self.http_endpoint().clone(),
            self.ws_endpoint().clone(),
            "memory://",
            settings,
        )
        .test_mode(true);

        let keystore = Keystore::new(KeystoreConfig::new().in_memory(true))?;
        insert_default_operator_key(&keystore)?;
        let client = TangleEvmClient::with_keystore(config, keystore).await?;
        let service = client
            .get_service_info(LOCAL_SERVICE_ID)
            .await
            .context("failed to read seeded service state")?;
        if service.status != ServiceStatus::Active {
            anyhow::bail!(
                "seeded service {LOCAL_SERVICE_ID} not active (status: {:?})",
                service.status
            );
        }
        Ok(())
    }
}

/// Builder for [`TangleEvmHarness`].
pub struct TangleEvmHarnessBuilder {
    include_anvil_logs: bool,
    seed_from_broadcast: bool,
}

impl Default for TangleEvmHarnessBuilder {
    fn default() -> Self {
        Self {
            include_anvil_logs: false,
            seed_from_broadcast: true,
        }
    }
}

impl TangleEvmHarnessBuilder {
    /// Include stdout/stderr from the spawned Anvil container.
    #[must_use]
    pub fn include_anvil_logs(mut self, include: bool) -> Self {
        self.include_anvil_logs = include;
        self
    }

    /// Enable or disable replaying `LocalTestnet.s.sol` artifacts.
    #[must_use]
    pub fn seed_from_broadcast(mut self, seed_from_broadcast: bool) -> Self {
        self.seed_from_broadcast = seed_from_broadcast;
        self
    }

    /// Boot the harness.
    pub async fn spawn(self) -> Result<TangleEvmHarness> {
        let TangleEvmHarnessBuilder {
            include_anvil_logs,
            seed_from_broadcast,
        } = self;

        let snapshot_loaded = snapshot_available();
        let testnet = start_empty_anvil_testnet(include_anvil_logs).await;
        if seed_from_broadcast && !snapshot_loaded {
            let broadcast = load_broadcast_file()?;
            seed_local_state(testnet.http_endpoint.as_str(), &broadcast).await?;
        }

        let harness = TangleEvmHarness {
            testnet,
            tangle_contract: Address::from_str(TANGLE_ADDRESS)?,
            restaking_contract: Address::from_str(RESTAKING_ADDRESS)?,
            status_registry_contract: Address::from_str(STATUS_REGISTRY_ADDRESS)?,
        };

        if seed_from_broadcast {
            if let Err(err) = harness.assert_seeded_service_active().await {
                if snapshot_loaded {
                    blueprint_core::warn!(
                        "Anvil snapshot missing seeded service, replaying broadcast: {err}"
                    );
                    let broadcast = load_broadcast_file()?;
                    seed_local_state(harness.http_endpoint().as_str(), &broadcast).await?;
                    harness.assert_seeded_service_active().await?;
                } else {
                    return Err(err);
                }
            }
        }

        Ok(harness)
    }
}

/// Boot a clean Anvil instance and replay the latest `LocalTestnet.s.sol` broadcast.
pub async fn start_tangle_evm_testnet(include_logs: bool) -> Result<SeededTangleEvmTestnet> {
    TangleEvmHarness::builder()
        .include_anvil_logs(include_logs)
        .seed_from_broadcast(true)
        .spawn()
        .await
}

async fn seed_local_state(rpc_url: &str, broadcast: &BroadcastFile) -> Result<()> {
    let provider = ProviderBuilder::new()
        .connect(rpc_url)
        .await
        .context("failed to connect to anvil")?;
    let mut impersonated = HashSet::new();
    let mut account_nonces = HashMap::new();
    let mut funded_accounts = HashSet::new();

    for (idx, tx) in broadcast.transactions.iter().enumerate() {
        let message = format!(
            "Seeding broadcast tx {}/{}: {}::{:?}",
            idx + 1,
            broadcast.transactions.len(),
            tx.contract_name.as_deref().unwrap_or("Unknown"),
            tx.function
        );
        blueprint_core::info!("{message}");
        println!("{message}");
        if tx.transaction_type != "CREATE" && tx.transaction_type != "CALL" {
            continue;
        }

        let from = tx
            .transaction
            .get("from")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("transaction missing from field"))?;
        let from = Address::from_str(from)?;
        let mut request: TransactionRequest = serde_json::from_value(tx.transaction.clone())
            .context("failed to deserialize broadcast transaction")?;
        request.from = Some(from);
        inject_fee_fields(&mut request);
        println!("Request: {:?}", request);

        let target_nonce = if let Some(nonce) = request.nonce {
            nonce
        } else {
            fetch_account_nonce(&provider, from).await?
        };
        if funded_accounts.insert(from) {
            fund_account(&provider, from).await?;
        }
        if impersonated.insert(from) {
            impersonate_account(&provider, from).await?;
        }

        let mut current_nonce = if let Some(nonce) = account_nonces.get(&from) {
            *nonce
        } else {
            let chain_nonce = fetch_account_nonce(&provider, from).await?;
            account_nonces.insert(from, chain_nonce);
            chain_nonce
        };
        if target_nonce > current_nonce {
            bump_account_nonce(&provider, from, current_nonce, target_nonce).await?;
            current_nonce = target_nonce;
            account_nonces.insert(from, current_nonce);
        } else if target_nonce < current_nonce {
            set_account_nonce(&provider, from, target_nonce).await?;
            account_nonces.insert(from, target_nonce);
            println!("Reset nonce for {from:?} -> {target_nonce}");
        }
        request.nonce = Some(target_nonce);
        let tx_hash: String = provider
            .raw_request(Cow::Borrowed("eth_sendTransaction"), json!([request]))
            .await
            .with_context(|| {
                format!(
                    "failed to send transaction {}::{:?}",
                    tx.contract_name.as_deref().unwrap_or("Unknown"),
                    tx.function
                )
            })?;
        println!(
            "Sent tx {}::{:?}: {tx_hash}",
            tx.contract_name.as_deref().unwrap_or("Unknown"),
            tx.function
        );
        let tx_info: Value = provider
            .raw_request(Cow::Borrowed("eth_getTransactionByHash"), json!([tx_hash]))
            .await
            .unwrap_or(Value::Null);
        println!("Tx info: {tx_info}");
        let _ = mine_blocks(&provider, 1).await;
        let receipt = wait_for_receipt(&provider, &tx_hash).await?;
        println!(
            "Confirmed tx {}::{:?}",
            tx.contract_name.as_deref().unwrap_or("Unknown"),
            tx.function
        );
        account_nonces.insert(from, target_nonce.saturating_add(1));
        let status = receipt
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if status != "0x1" {
            anyhow::bail!(
                "transaction seeding failed for {}::{:?}: {:?}",
                tx.contract_name.as_deref().unwrap_or("Unknown"),
                tx.function,
                receipt
            );
        }
    }

    for address in impersonated {
        let _ = provider
            .raw_request::<_, Value>(
                Cow::Borrowed("anvil_stopImpersonatingAccount"),
                json!([format!("{:#x}", address)]),
            )
            .await;
    }

    verify_contract(&provider, TANGLE_ADDRESS).await?;
    verify_contract(&provider, RESTAKING_ADDRESS).await?;

    Ok(())
}

fn inject_fee_fields(request: &mut TransactionRequest) {
    let uses_1559 = request.max_fee_per_gas.is_some() || request.max_priority_fee_per_gas.is_some();
    if uses_1559 {
        if request.max_fee_per_gas.is_none() {
            request.max_fee_per_gas = Some(DEFAULT_FEE_WEI);
        }
        if request.max_priority_fee_per_gas.is_none() {
            request.max_priority_fee_per_gas = request.max_fee_per_gas;
        }
        if request.max_priority_fee_per_gas.is_none() {
            request.max_priority_fee_per_gas = Some(DEFAULT_FEE_WEI);
        }
    } else if request.gas_price.is_none() {
        request.gas_price = Some(DEFAULT_FEE_WEI);
    }
    if request.gas.is_none() {
        request.gas = Some(DEFAULT_GAS_LIMIT);
    }
}

async fn fetch_account_nonce(provider: &impl Provider, address: Address) -> Result<u64> {
    let nonce_hex: String = provider
        .raw_request(
            Cow::Borrowed("eth_getTransactionCount"),
            json!([format!("{:#x}", address), "latest"]),
        )
        .await
        .context("failed to fetch nonce")?;
    Ok(u64::from_str_radix(nonce_hex.trim_start_matches("0x"), 16).unwrap_or_default())
}

async fn impersonate_account(provider: &impl Provider, address: Address) -> Result<()> {
    provider
        .raw_request::<_, Value>(
            Cow::Borrowed("anvil_impersonateAccount"),
            json!([format!("{:#x}", address)]),
        )
        .await
        .context("failed to impersonate account")?;
    println!("Impersonating account {address:?} complete");
    Ok(())
}

async fn set_account_nonce(provider: &impl Provider, address: Address, nonce: u64) -> Result<()> {
    provider
        .raw_request::<_, Value>(
            Cow::Borrowed("anvil_setNonce"),
            json!([format!("{:#x}", address), format!("0x{nonce:x}")]),
        )
        .await
        .context("failed to override account nonce")?;
    Ok(())
}

async fn bump_account_nonce(
    provider: &impl Provider,
    address: Address,
    start_nonce: u64,
    target_nonce: u64,
) -> Result<()> {
    for nonce in start_nonce..target_nonce {
        println!("Bumping nonce {nonce} for {address:?}");
        let mut filler = TransactionRequest::default();
        filler.from = Some(address);
        filler.to = Some(TxKind::Call(address));
        filler.gas = Some(21_000);
        filler.gas_price = Some(DEFAULT_FEE_WEI);
        filler.nonce = Some(nonce);
        let tx_hash: String = provider
            .raw_request(Cow::Borrowed("eth_sendTransaction"), json!([filler]))
            .await
            .context("failed to send nonce bump transaction")?;
        let _ = mine_blocks(provider, 1).await;
        let _ = wait_for_receipt(provider, &tx_hash).await?;
    }
    Ok(())
}

async fn mine_blocks(provider: &impl Provider, blocks: u64) -> Result<()> {
    provider
        .raw_request::<_, Value>(
            Cow::Borrowed("anvil_mine"),
            json!([format!("0x{blocks:x}")]),
        )
        .await
        .context("failed to mine blocks")?;
    Ok(())
}

async fn wait_for_receipt<P>(provider: &P, hash: &str) -> Result<Value>
where
    P: Provider,
{
    loop {
        let receipt: Value = provider
            .raw_request(Cow::Borrowed("eth_getTransactionReceipt"), json!([hash]))
            .await
            .context("failed to fetch receipt")?;
        if !receipt.is_null() {
            return Ok(receipt);
        }
        sleep(Duration::from_millis(25)).await;
    }
}

async fn verify_contract(provider: &impl Provider, addr: &str) -> Result<()> {
    let code: String = provider
        .raw_request(Cow::Borrowed("eth_getCode"), json!([addr, "latest"]))
        .await
        .context("failed to fetch contract code")?;
    if code == "0x" {
        anyhow::bail!("contract {} not deployed in seeded state", addr);
    }
    Ok(())
}

fn load_broadcast_file() -> Result<BroadcastFile> {
    let path = broadcast_artifact_path()?;
    let data =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let parsed = serde_json::from_str(&data).context("failed to parse broadcast json")?;
    Ok(parsed)
}

fn broadcast_artifact_path() -> Result<PathBuf> {
    if let Some(path) = env_broadcast_path() {
        return Ok(path);
    }

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../chain-setup/anvil/snapshots/localtestnet-broadcast.json");
    if path.exists() {
        Ok(path)
    } else {
        Err(HarnessError::MissingBroadcast(path).into())
    }
}

fn env_broadcast_path() -> Option<PathBuf> {
    let env_value = env::var_os("TNT_BROADCAST_PATH")?;
    let path = PathBuf::from(env_value);
    if path.exists() {
        Some(path)
    } else {
        eprintln!("warning: TNT_BROADCAST_PATH={} does not exist", path.display());
        None
    }
}

async fn fund_account(provider: &impl Provider, address: Address) -> Result<()> {
    provider
        .raw_request::<_, Value>(
            Cow::Borrowed("anvil_setBalance"),
            json!([
                format!("{:#x}", address),
                "0x3635c9adc5dea0000000000" // 1e9 ETH
            ]),
        )
        .await
        .context("failed to fund impersonated account")?;
    if env::var_os("BLUEPRINT_SEED_TRACE").is_some() {
        println!("Funded account {address:?}");
    }
    Ok(())
}

pub fn insert_default_operator_key(keystore: &Keystore) -> Result<()> {
    let secret = Vec::from_hex(OPERATOR1_PRIVATE_KEY)?;
    let signing_key = K256SigningKey::from_bytes(&secret)?;
    keystore.insert::<K256Ecdsa>(&signing_key)?;
    Ok(())
}

#[derive(Deserialize)]
struct BroadcastFile {
    transactions: Vec<BroadcastTransaction>,
}

#[derive(Deserialize)]
struct BroadcastTransaction {
    #[serde(rename = "transactionType")]
    transaction_type: String,
    #[serde(rename = "contractName", default)]
    contract_name: Option<String>,
    #[serde(default)]
    function: Option<String>,
    transaction: Value,
}
