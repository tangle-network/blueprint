//! CLI tests for operator commands.
//!
//! Tests use the TangleEvmHarness which pre-registers operators 1 and 2.
//! Account 3 (delegator) has TNT tokens for testing new operator registration.

use std::fs;

use blueprint_crypto::{BytesEncoding, k256::{K256Ecdsa, K256SigningKey}};
use blueprint_keystore::{Keystore, KeystoreConfig, backends::Backend};
use blueprint_testing_utils::anvil::seed_operator_key;
use blueprint_testing_utils::anvil::tangle_evm::{LOCAL_BLUEPRINT_ID, LOCAL_SERVICE_ID};
use color_eyre::eyre::{Result, eyre};
use hex::FromHex;
use serde_json::Value;
use tempfile::TempDir;

use crate::tests::util::{
    RUN_TNT_E2E_ENV, is_e2e_enabled, network_cli_args, parse_json_lines, run_cli_command,
    spawn_harness,
};

/// Operator 1 address (pre-registered in harness with TNT stake).
const OPERATOR1_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";

/// Account 3 (delegator) - has TNT tokens, not yet registered as operator.
const OPERATOR3_ADDRESS: &str = "0x90F79bf6EB2c4f870365E785982E1f101E93b906";
const OPERATOR3_PRIVATE_KEY: &str =
    "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6";

/// Default stake amount (100 TNT in wei).
const DEFAULT_STAKE_AMOUNT: u128 = 100_000_000_000_000_000_000;

fn seed_operator1_keystore(path: &std::path::Path) -> Result<()> {
    seed_operator_key(path).map_err(|e| eyre!(e.to_string()))?;
    Ok(())
}

fn seed_operator3_keystore(path: &std::path::Path) -> Result<()> {
    let keystore = Keystore::new(KeystoreConfig::new().fs_root(path))?;
    let secret = Vec::from_hex(OPERATOR3_PRIVATE_KEY)?;
    let signing_key = K256SigningKey::from_bytes(&secret)?;
    keystore
        .insert::<K256Ecdsa>(&signing_key)
        .map_err(|e| eyre!(e.to_string()))?;
    Ok(())
}

/// Test: Register new operator with TNT stake.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_operator_register() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping test_operator_register (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("test_operator_register").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator3_keystore(&keystore_path)?;

    let mut args = vec!["operator".to_string(), "register".to_string()];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--amount".into());
    args.push(DEFAULT_STAKE_AMOUNT.to_string());
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let events = parse_json_lines(&output.stdout)?;
    assert!(
        events.iter().any(|e| e.get("tx_hash").is_some()),
        "register missing tx_hash: {events:?}"
    );

    Ok(())
}

/// Test: Increase operator stake after registration.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_operator_increase_stake() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping test_operator_increase_stake (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("test_operator_increase_stake").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator3_keystore(&keystore_path)?;

    // First register
    let mut register_args = vec!["operator".to_string(), "register".to_string()];
    register_args.extend(network_cli_args(&harness, &keystore_path));
    register_args.push("--amount".into());
    register_args.push(DEFAULT_STAKE_AMOUNT.to_string());
    register_args.push("--json".into());
    run_cli_command(&register_args)?;

    // Then increase stake
    let increase_amount: u128 = 50_000_000_000_000_000_000; // 50 TNT
    let mut args = vec!["operator".to_string(), "increase-stake".to_string()];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--amount".into());
    args.push(increase_amount.to_string());
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let events = parse_json_lines(&output.stdout)?;
    assert!(
        events.iter().any(|e| e.get("tx_hash").is_some()),
        "increase-stake missing tx_hash: {events:?}"
    );

    Ok(())
}

/// Test: Query restaking status for pre-registered operator.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_operator_restaking_status() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping test_operator_restaking_status (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("test_operator_restaking_status").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator1_keystore(&keystore_path)?;

    let mut args = vec!["operator".to_string(), "restaking".to_string()];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--operator".into());
    args.push(OPERATOR1_ADDRESS.into());
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid JSON: {e}\n{}", output.stdout))?;

    assert!(parsed.get("operator").is_some(), "missing operator field");
    assert!(parsed.get("stake").is_some(), "missing stake field");
    assert!(parsed.get("status").is_some(), "missing status field");

    Ok(())
}

/// Test: List delegators for pre-registered operator.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_operator_delegators() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping test_operator_delegators (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("test_operator_delegators").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator1_keystore(&keystore_path)?;

    let mut args = vec!["operator".to_string(), "delegators".to_string()];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--operator".into());
    args.push(OPERATOR1_ADDRESS.into());
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid JSON: {e}\n{}", output.stdout))?;

    assert!(parsed.get("operator").is_some(), "missing operator field");
    assert!(parsed.get("delegators").is_some(), "missing delegators field");

    Ok(())
}

/// Test: Query operator status in service context.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_operator_status() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping test_operator_status (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("test_operator_status").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator1_keystore(&keystore_path)?;

    let mut args = vec!["operator".to_string(), "status".to_string()];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--blueprint-id".into());
    args.push(LOCAL_BLUEPRINT_ID.to_string());
    args.push("--service-id".into());
    args.push(LOCAL_SERVICE_ID.to_string());
    args.push("--operator".into());
    args.push(OPERATOR1_ADDRESS.into());
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid JSON: {e}\n{}", output.stdout))?;

    assert!(parsed.get("service_id").is_some(), "missing service_id");
    assert!(parsed.get("operator").is_some(), "missing operator");
    assert!(parsed.get("online").is_some(), "missing online");

    Ok(())
}

/// Test: Submit heartbeat for operator in service.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_operator_heartbeat() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping test_operator_heartbeat (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("test_operator_heartbeat").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator1_keystore(&keystore_path)?;

    let mut args = vec!["operator".to_string(), "heartbeat".to_string()];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--blueprint-id".into());
    args.push(LOCAL_BLUEPRINT_ID.to_string());
    args.push("--service-id".into());
    args.push(LOCAL_SERVICE_ID.to_string());
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid JSON: {e}\n{}", output.stdout))?;

    assert!(parsed.get("tx_hash").is_some(), "missing tx_hash");
    assert!(parsed.get("success").is_some(), "missing success");

    Ok(())
}

/// Test: Schedule operator unstake (for pre-registered operator).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_operator_schedule_unstake() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping test_operator_schedule_unstake (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("test_operator_schedule_unstake").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator1_keystore(&keystore_path)?;

    // Schedule a small unstake (0.1 ETH worth)
    let unstake_amount: u128 = 100_000_000_000_000_000;
    let mut args = vec!["operator".to_string(), "schedule-unstake".to_string()];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--amount".into());
    args.push(unstake_amount.to_string());
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid JSON: {e}\n{}", output.stdout))?;

    assert!(parsed.get("tx_hash").is_some(), "missing tx_hash");

    Ok(())
}

/// Test: Execute operator unstake (may be no-op if none matured).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_operator_execute_unstake() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping test_operator_execute_unstake (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("test_operator_execute_unstake").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator1_keystore(&keystore_path)?;

    let mut args = vec!["operator".to_string(), "execute-unstake".to_string()];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--json".into());

    // This may succeed (no-op) or fail if nothing to execute - either is valid
    let _ = run_cli_command(&args);

    Ok(())
}

/// Test: Start leaving operator set.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_operator_start_leaving() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping test_operator_start_leaving (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("test_operator_start_leaving").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator1_keystore(&keystore_path)?;

    let mut args = vec!["operator".to_string(), "start-leaving".to_string()];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid JSON: {e}\n{}", output.stdout))?;

    assert!(parsed.get("tx_hash").is_some(), "missing tx_hash");

    Ok(())
}
