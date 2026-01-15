use std::{fs, path::Path, str::FromStr};

use blueprint_crypto::{
    BytesEncoding,
    k256::{K256Ecdsa, K256SigningKey},
};
use blueprint_keystore::{Keystore, KeystoreConfig, backends::Backend};
use blueprint_testing_utils::anvil::{
    TangleEvmHarness, seed_operator_key,
    tangle_evm::{LOCAL_BLUEPRINT_ID, LOCAL_SERVICE_ID},
};
use color_eyre::eyre::{Result, eyre};
use hex::FromHex;
use serde_json::Value;
use tempfile::TempDir;
use tokio::time::{Duration, sleep};

use crate::{
    command::{signer::load_evm_signer, tangle::TangleClientArgs},
    tests::util::{
        RUN_TNT_E2E_ENV, is_e2e_enabled, network_cli_args, parse_json_lines, run_cli_command,
        spawn_harness,
    },
};
use alloy_primitives::{Address, U256};
use blueprint_client_tangle_evm::{MembershipModel, PricingModel};

const OPERATOR1_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
const OPERATOR2_ADDRESS: &str = "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC";
const OPERATOR2_PRIVATE_KEY: &str =
    "5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";
const DEFAULT_EXPOSURE_BPS: u16 = 5_000;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_service_list_reports_default_service() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_service_list_reports_default_service (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_service_list_reports_default_service").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator_keystore(&keystore_path)?;

    let mut args = vec![
        "blueprint".to_string(),
        "service".to_string(),
        "list".to_string(),
    ];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    let stdout = output.stdout;
    let parsed: Value =
        serde_json::from_str(&stdout).map_err(|e| eyre!("invalid JSON output: {e}\n{stdout}"))?;
    let services = parsed
        .as_array()
        .ok_or_else(|| eyre!("expected an array of services in output"))?;
    assert!(
        services.iter().any(|svc| {
            svc.get("service_id").and_then(Value::as_u64) == Some(LOCAL_SERVICE_ID)
                && svc.get("blueprint_id").and_then(Value::as_u64) == Some(LOCAL_BLUEPRINT_ID)
        }),
        "service list output missing default service: {stdout}"
    );

    Ok(())
}

fn seed_operator_keystore(path: &Path) -> Result<()> {
    seed_operator_key(path).map_err(|e| eyre!(e.to_string()))?;
    Ok(())
}

fn seed_specific_operator_key(path: &Path, hex_key: &str) -> Result<()> {
    let keystore = Keystore::new(KeystoreConfig::new().fs_root(path))?;
    let secret = Vec::from_hex(hex_key)?;
    let signing_key = K256SigningKey::from_bytes(&secret)?;
    keystore
        .insert::<K256Ecdsa>(&signing_key)
        .map_err(|e| eyre!(e.to_string()))?;
    Ok(())
}

struct RequestDefaults {
    operators: Vec<String>,
    payment_amount: u128,
}

async fn resolve_request_defaults(
    harness: &TangleEvmHarness,
    keystore_path: &Path,
) -> Result<Option<RequestDefaults>> {
    let client_args = TangleClientArgs {
        http_rpc_url: harness.http_endpoint().clone(),
        ws_rpc_url: harness.ws_endpoint().clone(),
        keystore_path: keystore_path.to_path_buf(),
        tangle_contract: format!("{:#x}", harness.tangle_contract),
        restaking_contract: format!("{:#x}", harness.restaking_contract),
        status_registry_contract: Some(format!("{:#x}", harness.status_registry_contract)),
    };
    let client = client_args.connect(LOCAL_BLUEPRINT_ID, None).await?;
    let (min_operators, pricing_model, subscription_rate, event_rate) = client
        .get_blueprint_config(LOCAL_BLUEPRINT_ID)
        .await
        .map(|config| {
            (
                config.minOperators.max(1),
                match config.pricing {
                    1 => PricingModel::Subscription,
                    2 => PricingModel::EventDriven,
                    _ => PricingModel::PayOnce,
                },
                config.subscriptionRate,
                config.eventRate,
            )
        })
        .unwrap_or((1, PricingModel::PayOnce, U256::ZERO, U256::ZERO));

    let mut operators = vec![OPERATOR1_ADDRESS.to_string(), OPERATOR2_ADDRESS.to_string()];
    let needed = min_operators as usize;
    if needed > operators.len() {
        return Ok(None);
    }
    operators.truncate(needed);
    let payment_amount = match pricing_model {
        PricingModel::Subscription => subscription_rate.to::<u128>(),
        PricingModel::EventDriven => event_rate.to::<u128>(),
        PricingModel::PayOnce => 0,
    };

    Ok(Some(RequestDefaults {
        operators,
        payment_amount,
    }))
}

async fn submit_service_request(
    network_args: &[String],
    defaults: &RequestDefaults,
) -> Result<u64> {
    let mut request_args = vec![
        "blueprint".to_string(),
        "service".to_string(),
        "request".to_string(),
    ];
    request_args.extend(network_args.iter().cloned());
    request_args.push("--blueprint-id".into());
    request_args.push(LOCAL_BLUEPRINT_ID.to_string());
    for operator in &defaults.operators {
        request_args.push("--operator".into());
        request_args.push(operator.clone());
    }
    for _ in 0..defaults.operators.len() {
        request_args.push("--operator-exposure-bps".into());
        request_args.push(DEFAULT_EXPOSURE_BPS.to_string());
    }
    if defaults.payment_amount > 0 {
        request_args.push("--payment-amount".into());
        request_args.push(defaults.payment_amount.to_string());
    }
    request_args.push("--json".into());

    let output = run_cli_command(&request_args)?;
    let events = parse_json_lines(&output.stdout)?;
    events
        .iter()
        .find_map(|event| {
            (event.get("event").and_then(Value::as_str) == Some("service_request_id"))
                .then(|| event.get("request_id").and_then(Value::as_u64))
                .flatten()
        })
        .ok_or_else(|| eyre!("service request output missing request id: {events:?}"))
}

async fn approve_service_request_cli(network_args: &[String], request_id: u64) -> Result<()> {
    let mut approve_args = vec![
        "blueprint".to_string(),
        "service".to_string(),
        "approve".to_string(),
    ];
    approve_args.extend(network_args.iter().cloned());
    approve_args.push("--request-id".into());
    approve_args.push(request_id.to_string());
    approve_args.push("--restaking-percent".into());
    approve_args.push("0".into());
    approve_args.push("--json".into());
    run_cli_command(&approve_args)?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_service_request_list_and_show_roundtrip() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_service_request_list_and_show_roundtrip (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_service_request_list_and_show_roundtrip").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator_keystore(&keystore_path)?;
    let network_args = network_cli_args(&harness, &keystore_path);

    let defaults = match resolve_request_defaults(&harness, &keystore_path).await? {
        Some(defaults) => defaults,
        None => {
            eprintln!(
                "Skipping cli_service_request_list_and_show_roundtrip: \
                 not enough operators for request"
            );
            return Ok(());
        }
    };

    let mut request_args = vec![
        "blueprint".to_string(),
        "service".to_string(),
        "request".to_string(),
    ];
    request_args.extend(network_args.clone());
    request_args.push("--blueprint-id".into());
    request_args.push(LOCAL_BLUEPRINT_ID.to_string());
    for operator in &defaults.operators {
        request_args.push("--operator".into());
        request_args.push(operator.clone());
    }
    for _ in 0..defaults.operators.len() {
        request_args.push("--operator-exposure-bps".into());
        request_args.push(DEFAULT_EXPOSURE_BPS.to_string());
    }
    if defaults.payment_amount > 0 {
        request_args.push("--payment-amount".into());
        request_args.push(defaults.payment_amount.to_string());
    }
    request_args.push("--json".into());

    let request_output = run_cli_command(&request_args)?;
    let events = parse_json_lines(&request_output.stdout)?;
    let request_id = events
        .iter()
        .find_map(|event| {
            (event.get("event").and_then(Value::as_str) == Some("service_request_id"))
                .then(|| event.get("request_id").and_then(Value::as_u64))
                .flatten()
        })
        .ok_or_else(|| eyre!("service request output missing request id: {events:?}"))?;

    sleep(Duration::from_millis(500)).await;

    let mut list_args = vec![
        "blueprint".to_string(),
        "service".to_string(),
        "requests".to_string(),
    ];
    list_args.extend(network_args.clone());
    list_args.push("--json".into());
    let list_output = run_cli_command(&list_args)?;
    let parsed_list: Value = serde_json::from_str(&list_output.stdout)
        .map_err(|e| eyre!("invalid request list JSON: {e}\n{}", list_output.stdout))?;
    let requests = parsed_list
        .as_array()
        .ok_or_else(|| eyre!("expected request list array output"))?;
    assert!(
        requests.iter().any(|entry| {
            entry
                .get("request_id")
                .and_then(Value::as_u64)
                .map_or(false, |value| value == request_id)
        }),
        "service requests output missing new request: {}",
        list_output.stdout
    );

    let mut show_args = vec![
        "blueprint".to_string(),
        "service".to_string(),
        "show".to_string(),
    ];
    show_args.extend(network_args);
    show_args.push("--request-id".into());
    show_args.push(request_id.to_string());
    let show_output = run_cli_command(&show_args)?;
    assert!(
        show_output
            .stdout
            .contains(&format!("Request ID: {request_id}")),
        "service show output missing request id:\n{}",
        show_output.stdout
    );
    assert!(
        show_output
            .stdout
            .contains(&format!("Blueprint ID: {LOCAL_BLUEPRINT_ID}")),
        "service show output missing blueprint id:\n{}",
        show_output.stdout
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_service_approve_creates_new_service() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_service_approve_creates_new_service (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_service_approve_creates_new_service").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator_keystore(&keystore_path)?;
    let network_args = network_cli_args(&harness, &keystore_path);
    let defaults = match resolve_request_defaults(&harness, &keystore_path).await? {
        Some(defaults) => defaults,
        None => {
            eprintln!(
                "Skipping cli_service_approve_creates_new_service: \
                 not enough operators for request"
            );
            return Ok(());
        }
    };

    let client_args = TangleClientArgs {
        http_rpc_url: harness.http_endpoint().clone(),
        ws_rpc_url: harness.ws_endpoint().clone(),
        keystore_path: keystore_path.clone(),
        tangle_contract: format!("{:#x}", harness.tangle_contract),
        restaking_contract: format!("{:#x}", harness.restaking_contract),
        status_registry_contract: Some(format!("{:#x}", harness.status_registry_contract)),
    };
    let admin_client = client_args.connect(LOCAL_BLUEPRINT_ID, None).await?;
    let before = admin_client.service_count().await?;

    let request_id = submit_service_request(&network_args, &defaults).await?;
    approve_service_request_cli(&network_args, request_id).await?;

    sleep(Duration::from_millis(500)).await;
    let after = admin_client.service_count().await?;
    assert_eq!(
        after,
        before + 1,
        "service count did not increase after approval"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_service_reject_marks_request_rejected() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_service_reject_marks_request_rejected (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_service_reject_marks_request_rejected").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator_keystore(&keystore_path)?;
    let network_args = network_cli_args(&harness, &keystore_path);
    let defaults = match resolve_request_defaults(&harness, &keystore_path).await? {
        Some(defaults) => defaults,
        None => {
            eprintln!(
                "Skipping cli_service_reject_marks_request_rejected: \
                 not enough operators for request"
            );
            return Ok(());
        }
    };

    let client_args = TangleClientArgs {
        http_rpc_url: harness.http_endpoint().clone(),
        ws_rpc_url: harness.ws_endpoint().clone(),
        keystore_path: keystore_path.clone(),
        tangle_contract: format!("{:#x}", harness.tangle_contract),
        restaking_contract: format!("{:#x}", harness.restaking_contract),
        status_registry_contract: Some(format!("{:#x}", harness.status_registry_contract)),
    };
    let admin_client = client_args.connect(LOCAL_BLUEPRINT_ID, None).await?;

    let request_id = submit_service_request(&network_args, &defaults).await?;
    sleep(Duration::from_millis(500)).await;

    let mut reject_args = vec![
        "blueprint".to_string(),
        "service".to_string(),
        "reject".to_string(),
    ];
    reject_args.extend(network_args.clone());
    reject_args.push("--request-id".into());
    reject_args.push(request_id.to_string());
    reject_args.push("--json".into());
    run_cli_command(&reject_args)?;

    sleep(Duration::from_millis(500)).await;
    let info = admin_client
        .get_service_request_info(request_id)
        .await
        .map_err(|e| eyre!(e.to_string()))?;
    assert!(info.rejected, "service request was not marked rejected");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_service_join_and_leave_dynamic_service() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_service_join_and_leave_dynamic_service (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_service_join_and_leave_dynamic_service").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_specific_operator_key(&keystore_path, OPERATOR2_PRIVATE_KEY)?;
    let network_args = network_cli_args(&harness, &keystore_path);

    let client_args = TangleClientArgs {
        http_rpc_url: harness.http_endpoint().clone(),
        ws_rpc_url: harness.ws_endpoint().clone(),
        keystore_path: keystore_path.clone(),
        tangle_contract: format!("{:#x}", harness.tangle_contract),
        restaking_contract: format!("{:#x}", harness.restaking_contract),
        status_registry_contract: Some(format!("{:#x}", harness.status_registry_contract)),
    };
    let register_client = client_args.connect(LOCAL_BLUEPRINT_ID, None).await?;
    let service_info = register_client
        .get_service_info(LOCAL_SERVICE_ID)
        .await
        .map_err(|e| eyre!(e.to_string()))?;
    if service_info.membership != MembershipModel::Dynamic {
        eprintln!("Skipping cli_service_join_and_leave_dynamic_service: service is not dynamic");
        return Ok(());
    }
    register_client
        .register_operator(LOCAL_BLUEPRINT_ID, "http://operator2.local:8545", None)
        .await
        .map_err(|e| eyre!(e.to_string()))?;

    let signer = load_evm_signer(&keystore_path)?;

    let mut join_args = vec![
        "blueprint".to_string(),
        "service".to_string(),
        "join".to_string(),
    ];
    join_args.extend(network_args.clone());
    join_args.push("--service-id".into());
    join_args.push(LOCAL_SERVICE_ID.to_string());
    join_args.push("--exposure-bps".into());
    join_args.push("6000".into());
    join_args.push("--json".into());
    let join_output = run_cli_command(&join_args)?;
    let join_events = parse_json_lines(&join_output.stdout)?;
    assert!(
        join_events
            .iter()
            .any(|event| event.get("event").and_then(Value::as_str) == Some("service_joined")),
        "service join output missing event: {join_events:?}"
    );

    sleep(Duration::from_millis(500)).await;
    let admin_client = client_args
        .connect(LOCAL_BLUEPRINT_ID, Some(LOCAL_SERVICE_ID))
        .await?;
    let operators = admin_client
        .get_service_operators(LOCAL_SERVICE_ID)
        .await
        .map_err(|e| eyre!(e.to_string()))?;
    assert!(
        operators.contains(&signer.operator_address),
        "operator join did not add operator to service"
    );

    // Use the legacy immediate leave path (no exit queue functionality)
    let mut leave_args = vec![
        "blueprint".to_string(),
        "service".to_string(),
        "leave".to_string(),
    ];
    leave_args.extend(network_args.clone());
    leave_args.push("--service-id".into());
    leave_args.push(LOCAL_SERVICE_ID.to_string());
    leave_args.push("--json".into());
    let leave_output = run_cli_command(&leave_args)?;
    let leave_events = parse_json_lines(&leave_output.stdout)?;
    assert!(
        leave_events
            .iter()
            .any(|event| event.get("event").and_then(Value::as_str) == Some("service_left")),
        "service leave output missing event: {leave_events:?}"
    );

    sleep(Duration::from_millis(250)).await;
    let remaining = admin_client
        .get_service_operators(LOCAL_SERVICE_ID)
        .await
        .map_err(|e| eyre!(e.to_string()))?;
    assert!(
        !remaining.contains(&signer.operator_address),
        "operator leave did not remove operator from service"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_operator_status_reports_json_snapshot() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_operator_status_reports_json_snapshot (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_operator_status_reports_json_snapshot").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_operator_keystore(&keystore_path)?;
    let network_args = network_cli_args(&harness, &keystore_path);

    let client_args = TangleClientArgs {
        http_rpc_url: harness.http_endpoint().clone(),
        ws_rpc_url: harness.ws_endpoint().clone(),
        keystore_path: keystore_path.clone(),
        tangle_contract: format!("{:#x}", harness.tangle_contract),
        restaking_contract: format!("{:#x}", harness.restaking_contract),
        status_registry_contract: Some(format!("{:#x}", harness.status_registry_contract)),
    };
    let client = client_args
        .connect(LOCAL_BLUEPRINT_ID, Some(LOCAL_SERVICE_ID))
        .await?;
    let operator_address =
        Address::from_str(OPERATOR1_ADDRESS).map_err(|_| eyre!("invalid operator address"))?;
    let expected_status = client
        .operator_status(LOCAL_SERVICE_ID, operator_address)
        .await
        .map_err(|e| eyre!(e.to_string()))?;

    let mut status_args = vec!["operator".to_string(), "status".to_string()];
    status_args.extend(network_args);
    status_args.push("--blueprint-id".into());
    status_args.push(LOCAL_BLUEPRINT_ID.to_string());
    status_args.push("--service-id".into());
    status_args.push(LOCAL_SERVICE_ID.to_string());
    status_args.push("--operator".into());
    status_args.push(OPERATOR1_ADDRESS.into());
    status_args.push("--json".into());

    let output = run_cli_command(&status_args)?;
    let parsed: Value = serde_json::from_str(&output.stdout)
        .map_err(|e| eyre!("invalid operator status JSON: {e}\n{}", output.stdout))?;
    let expected_operator = format!("{:#x}", expected_status.operator);
    assert_eq!(
        parsed.get("service_id").and_then(Value::as_u64),
        Some(LOCAL_SERVICE_ID),
        "operator status output missing service id"
    );
    assert_eq!(
        parsed.get("operator").and_then(Value::as_str),
        Some(expected_operator.as_str()),
        "operator status output missing operator address"
    );
    assert_eq!(
        parsed.get("status_code").and_then(Value::as_u64),
        Some(u64::from(expected_status.status_code)),
        "operator status output missing status code"
    );
    assert_eq!(
        parsed.get("last_heartbeat").and_then(Value::as_u64),
        Some(expected_status.last_heartbeat),
        "operator status output missing last heartbeat"
    );
    assert_eq!(
        parsed.get("online").and_then(Value::as_bool),
        Some(expected_status.online),
        "operator status output missing online flag"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_blueprint_register_registers_operator() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_blueprint_register_registers_operator (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_blueprint_register_registers_operator").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_specific_operator_key(&keystore_path, OPERATOR2_PRIVATE_KEY)?;

    let signer = load_evm_signer(&keystore_path)?;
    let client_args = TangleClientArgs {
        http_rpc_url: harness.http_endpoint().clone(),
        ws_rpc_url: harness.ws_endpoint().clone(),
        keystore_path: keystore_path.clone(),
        tangle_contract: format!("{:#x}", harness.tangle_contract),
        restaking_contract: format!("{:#x}", harness.restaking_contract),
        status_registry_contract: Some(format!("{:#x}", harness.status_registry_contract)),
    };
    let admin_client = client_args
        .connect(LOCAL_BLUEPRINT_ID, Some(LOCAL_SERVICE_ID))
        .await?;
    if admin_client
        .is_operator_registered(LOCAL_BLUEPRINT_ID, signer.operator_address)
        .await
        .map_err(|e| eyre!(e.to_string()))?
    {
        eprintln!(
            "Skipping cli_blueprint_register_registers_operator: operator already registered"
        );
        return Ok(());
    }

    let mut args = vec!["blueprint".to_string(), "register".to_string()];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--blueprint-id".into());
    args.push(LOCAL_BLUEPRINT_ID.to_string());
    args.push("--rpc-endpoint".into());
    args.push("https://rpc.example.com".into());

    let output = run_cli_command(&args)?;
    assert!(
        output.stdout.contains("Operator ready"),
        "register output missing success message:\n{}",
        output.stdout
    );

    admin_client
        .get_operator_metadata(LOCAL_BLUEPRINT_ID, signer.operator_address)
        .await
        .map_err(|e| eyre!(e.to_string()))?;

    Ok(())
}
