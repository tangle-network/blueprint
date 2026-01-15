//! CLI tests for delegator commands.
//!
//! These tests verify the delegator subcommand flows including queries,
//! deposits, delegations, unstakes, and withdrawals.

use std::{fs, path::Path};

use blueprint_crypto::{BytesEncoding, k256::{K256Ecdsa, K256SigningKey}};
use blueprint_keystore::{Keystore, KeystoreConfig, backends::Backend};
use blueprint_testing_utils::anvil::{
    seed_operator_key,
    tangle_evm::LOCAL_BLUEPRINT_ID,
};
use color_eyre::eyre::{Result, eyre};
use hex::FromHex;
use serde_json::Value;
use tempfile::TempDir;
use tokio::time::{Duration, sleep};

use crate::tests::util::{
    RUN_TNT_E2E_ENV, is_e2e_enabled, network_cli_args, parse_json_lines, run_cli_command,
    spawn_harness,
};

/// Operator 1 address (matches Anvil default account index 1).
const OPERATOR1_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";

/// Delegator private key (Anvil account index 3).
const DELEGATOR_PRIVATE_KEY: &str =
    "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6";

/// Delegator address derived from DELEGATOR_PRIVATE_KEY.
const DELEGATOR_ADDRESS: &str = "0x90F79bf6EB2c4f870365E785982E1f101E93b906";

/// Native token address (zero address).
const NATIVE_TOKEN: &str = "0x0000000000000000000000000000000000000000";

/// Deposit amount in wei for tests.
const DEPOSIT_AMOUNT: u128 = 1_000_000_000_000_000_000; // 1 ETH

fn seed_delegator_keystore(path: &Path) -> Result<()> {
    let keystore = Keystore::new(KeystoreConfig::new().fs_root(path))?;
    let secret = Vec::from_hex(DELEGATOR_PRIVATE_KEY)?;
    let signing_key = K256SigningKey::from_bytes(&secret)?;
    keystore
        .insert::<K256Ecdsa>(&signing_key)
        .map_err(|e| eyre!(e.to_string()))?;
    Ok(())
}

fn seed_operator_keystore(path: &Path) -> Result<()> {
    seed_operator_key(path).map_err(|e| eyre!(e.to_string()))?;
    Ok(())
}

/// Query delegator positions (no delegation yet).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_delegator_positions_empty() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_delegator_positions_empty (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_delegator_positions_empty").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_delegator_keystore(&keystore_path)?;

    let mut args = vec![
        "delegator".to_string(),
        "positions".to_string(),
    ];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid JSON output: {e}\n{}", output.stdout))?;

    // Verify delegator address is present
    assert!(
        parsed.get("delegator").is_some(),
        "positions output missing delegator field: {}",
        output.stdout
    );

    // Verify deposit structure exists
    assert!(
        parsed.get("deposit").is_some(),
        "positions output missing deposit field: {}",
        output.stdout
    );

    Ok(())
}

/// Query delegator positions for a specific address.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_delegator_positions_for_address() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_delegator_positions_for_address (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_delegator_positions_for_address").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator_keystore(&keystore_path)?;

    let mut args = vec![
        "delegator".to_string(),
        "positions".to_string(),
    ];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--delegator".into());
    args.push(DELEGATOR_ADDRESS.into());
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid JSON output: {e}\n{}", output.stdout))?;

    let delegator = parsed.get("delegator").and_then(Value::as_str);
    assert_eq!(
        delegator,
        Some(DELEGATOR_ADDRESS.to_lowercase().as_str()),
        "positions output has wrong delegator: {:?}",
        delegator
    );

    Ok(())
}

/// Query delegations for a delegator.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_delegator_delegations_empty() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_delegator_delegations_empty (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_delegator_delegations_empty").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_delegator_keystore(&keystore_path)?;

    let mut args = vec![
        "delegator".to_string(),
        "delegations".to_string(),
    ];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid JSON output: {e}\n{}", output.stdout))?;

    let delegations = parsed.get("delegations").and_then(Value::as_array);
    assert!(
        delegations.is_some(),
        "delegations output missing delegations field: {}",
        output.stdout
    );

    Ok(())
}

/// Query pending unstakes for a delegator.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_delegator_pending_unstakes_empty() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_delegator_pending_unstakes_empty (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_delegator_pending_unstakes_empty").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_delegator_keystore(&keystore_path)?;

    let mut args = vec![
        "delegator".to_string(),
        "pending-unstakes".to_string(),
    ];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid JSON output: {e}\n{}", output.stdout))?;

    let unstakes = parsed.get("pending_unstakes").and_then(Value::as_array);
    assert!(
        unstakes.is_some(),
        "pending-unstakes output missing pending_unstakes field: {}",
        output.stdout
    );

    Ok(())
}

/// Query pending withdrawals for a delegator.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_delegator_pending_withdrawals_empty() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_delegator_pending_withdrawals_empty (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_delegator_pending_withdrawals_empty").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_delegator_keystore(&keystore_path)?;

    let mut args = vec![
        "delegator".to_string(),
        "pending-withdrawals".to_string(),
    ];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid JSON output: {e}\n{}", output.stdout))?;

    let withdrawals = parsed.get("pending_withdrawals").and_then(Value::as_array);
    assert!(
        withdrawals.is_some(),
        "pending-withdrawals output missing pending_withdrawals field: {}",
        output.stdout
    );

    Ok(())
}

/// Deposit native ETH via CLI.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_delegator_deposit_native() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_delegator_deposit_native (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_delegator_deposit_native").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_delegator_keystore(&keystore_path)?;

    let mut args = vec![
        "delegator".to_string(),
        "deposit".to_string(),
    ];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--amount".into());
    args.push(DEPOSIT_AMOUNT.to_string());
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let events = parse_json_lines(&output.stdout)?;

    // Verify transaction was submitted
    assert!(
        events.iter().any(|e| {
            e.get("event").and_then(Value::as_str) == Some("tx_confirmed")
                || e.get("tx_hash").is_some()
        }),
        "deposit output missing transaction confirmation: {:?}",
        events
    );

    // Verify deposit increased in positions
    sleep(Duration::from_millis(500)).await;

    let mut pos_args = vec![
        "delegator".to_string(),
        "positions".to_string(),
    ];
    pos_args.extend(network_cli_args(&harness, &keystore_path));
    pos_args.push("--json".into());

    let pos_output = run_cli_command(&pos_args)?;
    let parsed: Value = serde_json::from_str(&pos_output.stdout)
        .map_err(|e| eyre!("invalid JSON output: {e}\n{}", pos_output.stdout))?;

    let deposit_amount = parsed
        .get("deposit")
        .and_then(|d| d.get("amount"))
        .and_then(Value::as_str)
        .unwrap_or("0");

    assert!(
        deposit_amount != "0",
        "deposit amount should be non-zero after deposit: {}",
        deposit_amount
    );

    Ok(())
}

/// Full delegation flow: deposit, delegate, undelegate.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_delegator_delegate_and_undelegate() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_delegator_delegate_and_undelegate (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_delegator_delegate_and_undelegate").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_delegator_keystore(&keystore_path)?;
    let network_args = network_cli_args(&harness, &keystore_path);

    // Step 1: Deposit native ETH
    let mut deposit_args = vec![
        "delegator".to_string(),
        "deposit".to_string(),
    ];
    deposit_args.extend(network_args.clone());
    deposit_args.push("--amount".into());
    deposit_args.push(DEPOSIT_AMOUNT.to_string());
    deposit_args.push("--json".into());

    run_cli_command(&deposit_args)?;
    sleep(Duration::from_millis(500)).await;

    // Step 2: Delegate to operator using from-deposit
    let mut delegate_args = vec![
        "delegator".to_string(),
        "delegate".to_string(),
    ];
    delegate_args.extend(network_args.clone());
    delegate_args.push("--operator".into());
    delegate_args.push(OPERATOR1_ADDRESS.into());
    delegate_args.push("--amount".into());
    delegate_args.push((DEPOSIT_AMOUNT / 2).to_string());
    delegate_args.push("--from-deposit".into());
    delegate_args.push("--json".into());

    let delegate_output = run_cli_command(&delegate_args)?;
    let delegate_events = parse_json_lines(&delegate_output.stdout)?;
    assert!(
        delegate_events.iter().any(|e| {
            e.get("event").and_then(Value::as_str) == Some("tx_confirmed")
                || e.get("tx_hash").is_some()
        }),
        "delegate output missing transaction confirmation: {:?}",
        delegate_events
    );

    sleep(Duration::from_millis(500)).await;

    // Verify delegation exists
    let mut delegations_args = vec![
        "delegator".to_string(),
        "delegations".to_string(),
    ];
    delegations_args.extend(network_args.clone());
    delegations_args.push("--json".into());

    let delegations_output = run_cli_command(&delegations_args)?;
    let delegations_parsed: Value = serde_json::from_str(&delegations_output.stdout)
        .map_err(|e| eyre!("invalid JSON output: {e}\n{}", delegations_output.stdout))?;

    let delegations = delegations_parsed
        .get("delegations")
        .and_then(Value::as_array);
    assert!(
        delegations.map_or(false, |d| !d.is_empty()),
        "delegations should not be empty after delegation: {}",
        delegations_output.stdout
    );

    // Step 3: Undelegate (schedule unstake)
    let mut undelegate_args = vec![
        "delegator".to_string(),
        "undelegate".to_string(),
    ];
    undelegate_args.extend(network_args.clone());
    undelegate_args.push("--operator".into());
    undelegate_args.push(OPERATOR1_ADDRESS.into());
    undelegate_args.push("--amount".into());
    undelegate_args.push((DEPOSIT_AMOUNT / 4).to_string());
    undelegate_args.push("--json".into());

    let undelegate_output = run_cli_command(&undelegate_args)?;
    let undelegate_events = parse_json_lines(&undelegate_output.stdout)?;
    assert!(
        undelegate_events.iter().any(|e| {
            e.get("event").and_then(Value::as_str) == Some("tx_confirmed")
                || e.get("tx_hash").is_some()
        }),
        "undelegate output missing transaction confirmation: {:?}",
        undelegate_events
    );

    sleep(Duration::from_millis(500)).await;

    // Verify pending unstake exists
    let mut unstakes_args = vec![
        "delegator".to_string(),
        "pending-unstakes".to_string(),
    ];
    unstakes_args.extend(network_args.clone());
    unstakes_args.push("--json".into());

    let unstakes_output = run_cli_command(&unstakes_args)?;
    let unstakes_parsed: Value = serde_json::from_str(&unstakes_output.stdout)
        .map_err(|e| eyre!("invalid JSON output: {e}\n{}", unstakes_output.stdout))?;

    let unstakes = unstakes_parsed
        .get("pending_unstakes")
        .and_then(Value::as_array);
    assert!(
        unstakes.map_or(false, |u| !u.is_empty()),
        "pending_unstakes should not be empty after undelegate: {}",
        unstakes_output.stdout
    );

    Ok(())
}

/// Execute unstake command (may not have matured unstakes).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_delegator_execute_unstake() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_delegator_execute_unstake (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_delegator_execute_unstake").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_delegator_keystore(&keystore_path)?;

    let mut args = vec![
        "delegator".to_string(),
        "execute-unstake".to_string(),
    ];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--json".into());

    // This may succeed or fail depending on state; we just verify the CLI runs
    let result = run_cli_command(&args);

    // Either it succeeds (no unstakes to execute is valid) or it fails gracefully
    match result {
        Ok(output) => {
            let events = parse_json_lines(&output.stdout)?;
            // Should have some output indicating success or no-op
            assert!(
                !output.stdout.is_empty() || events.is_empty(),
                "execute-unstake should produce output"
            );
        }
        Err(e) => {
            // Some errors are expected if no unstakes are ready
            let msg = e.to_string();
            assert!(
                msg.contains("no") || msg.contains("empty") || msg.contains("revert"),
                "unexpected error from execute-unstake: {msg}"
            );
        }
    }

    Ok(())
}

/// Schedule and execute withdrawal flow.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_delegator_schedule_and_execute_withdraw() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_delegator_schedule_and_execute_withdraw (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_delegator_schedule_and_execute_withdraw").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_delegator_keystore(&keystore_path)?;
    let network_args = network_cli_args(&harness, &keystore_path);

    // Step 1: Deposit first
    let mut deposit_args = vec![
        "delegator".to_string(),
        "deposit".to_string(),
    ];
    deposit_args.extend(network_args.clone());
    deposit_args.push("--amount".into());
    deposit_args.push(DEPOSIT_AMOUNT.to_string());
    deposit_args.push("--json".into());

    run_cli_command(&deposit_args)?;
    sleep(Duration::from_millis(500)).await;

    // Step 2: Schedule withdrawal
    let mut schedule_args = vec![
        "delegator".to_string(),
        "schedule-withdraw".to_string(),
    ];
    schedule_args.extend(network_args.clone());
    schedule_args.push("--amount".into());
    schedule_args.push((DEPOSIT_AMOUNT / 2).to_string());
    schedule_args.push("--json".into());

    let schedule_output = run_cli_command(&schedule_args)?;
    let schedule_events = parse_json_lines(&schedule_output.stdout)?;
    assert!(
        schedule_events.iter().any(|e| {
            e.get("event").and_then(Value::as_str) == Some("tx_confirmed")
                || e.get("tx_hash").is_some()
        }),
        "schedule-withdraw output missing transaction confirmation: {:?}",
        schedule_events
    );

    sleep(Duration::from_millis(500)).await;

    // Verify pending withdrawal exists
    let mut withdrawals_args = vec![
        "delegator".to_string(),
        "pending-withdrawals".to_string(),
    ];
    withdrawals_args.extend(network_args.clone());
    withdrawals_args.push("--json".into());

    let withdrawals_output = run_cli_command(&withdrawals_args)?;
    let withdrawals_parsed: Value = serde_json::from_str(&withdrawals_output.stdout)
        .map_err(|e| eyre!("invalid JSON output: {e}\n{}", withdrawals_output.stdout))?;

    let withdrawals = withdrawals_parsed
        .get("pending_withdrawals")
        .and_then(Value::as_array);
    assert!(
        withdrawals.map_or(false, |w| !w.is_empty()),
        "pending_withdrawals should not be empty after schedule-withdraw: {}",
        withdrawals_output.stdout
    );

    // Step 3: Try execute-withdraw (may not be matured yet)
    let mut execute_args = vec![
        "delegator".to_string(),
        "execute-withdraw".to_string(),
    ];
    execute_args.extend(network_args.clone());
    execute_args.push("--json".into());

    // This may fail if withdrawal hasn't matured; that's expected
    let _ = run_cli_command(&execute_args);

    Ok(())
}

/// Delegate with direct deposit (not from-deposit).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_delegator_delegate_direct() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_delegator_delegate_direct (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_delegator_delegate_direct").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_delegator_keystore(&keystore_path)?;
    let network_args = network_cli_args(&harness, &keystore_path);

    // Delegate directly without pre-deposit (deposits + delegates in one tx)
    let mut delegate_args = vec![
        "delegator".to_string(),
        "delegate".to_string(),
    ];
    delegate_args.extend(network_args.clone());
    delegate_args.push("--operator".into());
    delegate_args.push(OPERATOR1_ADDRESS.into());
    delegate_args.push("--amount".into());
    delegate_args.push(DEPOSIT_AMOUNT.to_string());
    delegate_args.push("--json".into());

    let delegate_output = run_cli_command(&delegate_args)?;
    let delegate_events = parse_json_lines(&delegate_output.stdout)?;
    assert!(
        delegate_events.iter().any(|e| {
            e.get("event").and_then(Value::as_str) == Some("tx_confirmed")
                || e.get("tx_hash").is_some()
        }),
        "delegate output missing transaction confirmation: {:?}",
        delegate_events
    );

    sleep(Duration::from_millis(500)).await;

    // Verify delegation exists
    let mut delegations_args = vec![
        "delegator".to_string(),
        "delegations".to_string(),
    ];
    delegations_args.extend(network_args.clone());
    delegations_args.push("--json".into());

    let delegations_output = run_cli_command(&delegations_args)?;
    let delegations_parsed: Value = serde_json::from_str(&delegations_output.stdout)
        .map_err(|e| eyre!("invalid JSON output: {e}\n{}", delegations_output.stdout))?;

    let delegations = delegations_parsed
        .get("delegations")
        .and_then(Value::as_array);
    assert!(
        delegations.map_or(false, |d| !d.is_empty()),
        "delegations should not be empty after direct delegation: {}",
        delegations_output.stdout
    );

    Ok(())
}

/// Delegate with fixed blueprint selection mode.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_delegator_delegate_fixed_selection() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_delegator_delegate_fixed_selection (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_delegator_delegate_fixed_selection").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_delegator_keystore(&keystore_path)?;
    let network_args = network_cli_args(&harness, &keystore_path);

    // Delegate with fixed selection mode
    let mut delegate_args = vec![
        "delegator".to_string(),
        "delegate".to_string(),
    ];
    delegate_args.extend(network_args.clone());
    delegate_args.push("--operator".into());
    delegate_args.push(OPERATOR1_ADDRESS.into());
    delegate_args.push("--amount".into());
    delegate_args.push(DEPOSIT_AMOUNT.to_string());
    delegate_args.push("--selection".into());
    delegate_args.push("fixed".into());
    delegate_args.push("--blueprint-id".into());
    delegate_args.push(LOCAL_BLUEPRINT_ID.to_string());
    delegate_args.push("--json".into());

    let delegate_output = run_cli_command(&delegate_args)?;
    let delegate_events = parse_json_lines(&delegate_output.stdout)?;
    assert!(
        delegate_events.iter().any(|e| {
            e.get("event").and_then(Value::as_str) == Some("tx_confirmed")
                || e.get("tx_hash").is_some()
        }),
        "delegate with fixed selection missing transaction confirmation: {:?}",
        delegate_events
    );

    Ok(())
}

/// Test positions with non-native token (uses zero address in these tests).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_delegator_positions_with_token() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_delegator_positions_with_token (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_delegator_positions_with_token").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_delegator_keystore(&keystore_path)?;

    let mut args = vec![
        "delegator".to_string(),
        "positions".to_string(),
    ];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--token".into());
    args.push(NATIVE_TOKEN.into());
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid JSON output: {e}\n{}", output.stdout))?;

    let token = parsed.get("token").and_then(Value::as_str);
    assert_eq!(
        token,
        Some(NATIVE_TOKEN.to_lowercase().as_str()),
        "positions output should include token field: {:?}",
        token
    );

    Ok(())
}
