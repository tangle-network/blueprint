use crate::command::deploy::definition::decode_blueprint_definition;
use crate::command::jobs::{check::wait_for_job_result, submit::submit_job as submit_job_call};
use crate::command::tangle::TangleClientArgs;
use crate::tests::util::{
    RUN_TNT_E2E_ENV, is_e2e_enabled, network_cli_args, run_cli_command, spawn_harness,
};
use alloy_primitives::Bytes;
use assert_cmd::Command;
use blueprint_client_tangle_evm::{TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings};
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::K256SigningKey;
use blueprint_keystore::{Keystore, KeystoreConfig, backends::Backend};
use blueprint_testing_utils::anvil::{
    TangleEvmHarness, insert_default_operator_key, tangle_evm::LOCAL_BLUEPRINT_ID,
    tangle_evm::LOCAL_SERVICE_ID,
};
use color_eyre::eyre::{Result, eyre};
use hex::FromHex;
use serde_json::Value;
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::Stdio;
use tempfile::TempDir;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

const SERVICE_OWNER_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_jobs_list_reports_blueprint_jobs() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_jobs_list_reports_blueprint_jobs (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_jobs_list_reports_blueprint_jobs").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_private_key(&keystore_path, SERVICE_OWNER_PRIVATE_KEY)?;

    let mut args = vec![
        "blueprint".to_string(),
        "jobs".to_string(),
        "list".to_string(),
    ];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--blueprint-id".into());
    args.push(LOCAL_BLUEPRINT_ID.to_string());
    args.push("--json".into());

    let output = Command::cargo_bin("cargo-tangle")
        .map_err(|e| eyre!(e))?
        .env("NO_COLOR", "1")
        .args(args.iter().map(|s| s.as_str()))
        .output()
        .map_err(|e| eyre!("failed to execute cargo-tangle: {e}"))?;

    if !output.status.success() {
        return Err(eyre!(
            "CLI command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let parsed: Value = serde_json::from_str(&stdout)
        .map_err(|e| eyre!("invalid JSON job list output: {e}\n{stdout}"))?;
    let jobs = parsed
        .as_array()
        .ok_or_else(|| eyre!("expected array output from jobs list"))?;
    assert!(
        !jobs.is_empty(),
        "jobs list output should include at least one job: {stdout}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_jobs_list_warns_when_blueprint_hashes_missing() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!(
            "Skipping cli_jobs_list_warns_when_blueprint_hashes_missing (set {RUN_TNT_E2E_ENV}=1)"
        );
        return Ok(());
    }

    let harness = match spawn_harness("cli_jobs_list_warns_when_blueprint_hashes_missing").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let operator_client =
        build_operator_client(&harness, LOCAL_BLUEPRINT_ID, LOCAL_SERVICE_ID).await?;
    let raw_definition = operator_client
        .get_raw_blueprint_definition(LOCAL_BLUEPRINT_ID)
        .await
        .map_err(|e| eyre!(e.to_string()))?;
    let decoded = decode_blueprint_definition(&raw_definition)
        .map_err(|e| eyre!("failed to decode blueprint definition: {e}"))?;
    let missing_sources = decoded
        .sources
        .iter()
        .enumerate()
        .filter(|(_, source)| source.binaries.is_empty())
        .map(|(idx, _)| idx + 1)
        .collect::<Vec<_>>();
    if missing_sources.is_empty() {
        eprintln!(
            "Skipping cli_jobs_list_warns_when_blueprint_hashes_missing: \
             seeded blueprint already has binary hashes"
        );
        return Ok(());
    }

    let keystore_dir = TempDir::new()?;
    let keystore_path = keystore_dir.path().join("keys");
    fs::create_dir_all(&keystore_path)?;
    seed_private_key(&keystore_path, SERVICE_OWNER_PRIVATE_KEY)?;

    let mut args = vec![
        "blueprint".to_string(),
        "jobs".to_string(),
        "list".to_string(),
    ];
    args.extend(network_cli_args(&harness, &keystore_path));
    args.push("--blueprint-id".into());
    args.push(LOCAL_BLUEPRINT_ID.to_string());
    args.push("--json".into());

    let output = run_cli_command(&args)?;
    assert!(
        output
            .stderr
            .contains("source entries without binary hashes"),
        "CLI stderr missing hash warning:\n{}",
        output.stderr
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_jobs_show_reports_call_metadata() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_jobs_show_reports_call_metadata (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_jobs_show_reports_call_metadata").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let owner_keystore = TempDir::new()?;
    let owner_keystore_path = owner_keystore.path().join("keys");
    fs::create_dir_all(&owner_keystore_path)?;
    seed_private_key(&owner_keystore_path, SERVICE_OWNER_PRIVATE_KEY)?;

    let submit_args = TangleClientArgs {
        http_rpc_url: harness.http_endpoint().clone(),
        ws_rpc_url: harness.ws_endpoint().clone(),
        keystore_path: owner_keystore_path.clone(),
        tangle_contract: format!("{:#x}", harness.tangle_contract),
        restaking_contract: format!("{:#x}", harness.restaking_contract),
        status_registry_contract: Some(format!("{:#x}", harness.status_registry_contract)),
    };

    let service_id = LOCAL_SERVICE_ID;
    let blueprint_id = LOCAL_BLUEPRINT_ID;
    let submit_client = submit_args.connect(blueprint_id, Some(service_id)).await?;
    let payload = Bytes::from_static(b"cli-jobs-show");
    let submission = submit_job_call(&submit_client, service_id, 0, payload.clone()).await?;

    sleep(Duration::from_millis(250)).await;

    let mut args = vec![
        "blueprint".to_string(),
        "jobs".to_string(),
        "show".to_string(),
    ];
    args.extend(network_cli_args(&harness, &owner_keystore_path));
    args.push("--blueprint-id".into());
    args.push(blueprint_id.to_string());
    args.push("--service-id".into());
    args.push(service_id.to_string());
    args.push("--call-id".into());
    args.push(submission.call_id.to_string());
    args.push("--json".into());

    let output = Command::cargo_bin("cargo-tangle")
        .map_err(|e| eyre!(e))?
        .env("NO_COLOR", "1")
        .args(args.iter().map(|s| s.as_str()))
        .output()
        .map_err(|e| eyre!("failed to execute cargo-tangle: {e}"))?;

    if !output.status.success() {
        return Err(eyre!(
            "CLI command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let details: Value = serde_json::from_str(&stdout)
        .map_err(|e| eyre!("invalid JSON job details output: {e}\n{stdout}"))?;
    assert_eq!(
        details.get("call_id").and_then(Value::as_u64),
        Some(submission.call_id),
        "job call output missing expected call_id"
    );
    assert_eq!(
        details.get("service_id").and_then(Value::as_u64),
        Some(service_id),
        "job call output missing service id"
    );
    assert_eq!(
        details.get("completed").and_then(Value::as_bool),
        Some(false),
        "job call should not be completed initially"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_jobs_submit_watch_reports_job_result() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping cli_jobs_submit_watch_reports_job_result (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("cli_jobs_submit_watch_reports_job_result").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let owner_keystore = TempDir::new()?;
    let owner_keystore_path = owner_keystore.path().join("keys");
    fs::create_dir_all(&owner_keystore_path)?;
    seed_private_key(&owner_keystore_path, SERVICE_OWNER_PRIVATE_KEY)?;

    let mut args = vec![
        "blueprint".to_string(),
        "jobs".to_string(),
        "submit".to_string(),
    ];
    args.extend(network_cli_args(&harness, &owner_keystore_path));
    args.push("--blueprint-id".into());
    args.push(LOCAL_BLUEPRINT_ID.to_string());
    args.push("--service-id".into());
    args.push(LOCAL_SERVICE_ID.to_string());
    args.push("--job".into());
    args.push("0".into());
    args.push("--payload-hex".into());
    args.push("0x68656c6c6f2d636c69".into());
    args.push("--watch".into());
    args.push("--timeout-secs".into());
    args.push("45".into());
    args.push("--json".into());

    let binary = std::env::var("CARGO_BIN_EXE_cargo-tangle")
        .map_err(|_| eyre!("CARGO_BIN_EXE_cargo-tangle is not set"))?;
    let mut command = std::process::Command::new(&binary);
    command
        .env("NO_COLOR", "1")
        .args(args.iter().map(|s| s.as_str()))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|e| eyre!("failed to spawn cargo-tangle: {e}"))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| eyre!("missing stdout handle from cargo-tangle"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| eyre!("missing stderr handle from cargo-tangle"))?;

    let (stdout_tx, mut stdout_rx) = mpsc::unbounded_channel();
    let stdout_handle = std::thread::spawn(move || -> Result<()> {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line?;
            if stdout_tx.send(line).is_err() {
                break;
            }
        }
        Ok(())
    });
    let stderr_handle = std::thread::spawn(move || -> std::io::Result<String> {
        let mut buffer = String::new();
        let mut reader = BufReader::new(stderr);
        reader.read_to_string(&mut buffer)?;
        Ok(buffer)
    });

    let operator_client =
        build_operator_client(&harness, LOCAL_BLUEPRINT_ID, LOCAL_SERVICE_ID).await?;
    let result_payload = Bytes::from_static(b"cli-job-result");

    let mut observed_lines = Vec::new();
    let mut operator_task = None;
    let mut job_result_seen = false;

    while let Some(line) = stdout_rx.recv().await {
        let trimmed = line.trim().to_string();
        if trimmed.is_empty() {
            continue;
        }
        observed_lines.push(trimmed.clone());
        let parsed_json = serde_json::from_str::<Value>(&trimmed).ok();
        if operator_task.is_none() {
            if let Some(call_id) = parsed_json
                .as_ref()
                .and_then(extract_call_id_from_json)
                .or_else(|| extract_call_id_from_text(&trimmed))
            {
                let client = operator_client.clone();
                let payload = result_payload.clone();
                operator_task = Some(tokio::spawn(async move {
                    sleep(Duration::from_millis(250)).await;
                    client
                        .submit_result(LOCAL_SERVICE_ID, call_id, payload)
                        .await
                        .map_err(|e| eyre!(e.to_string()))
                        .map(|_| ())
                }));
            }
        }

        if matches_json_job_result(parsed_json.as_ref()) || trimmed.contains("Job result ready") {
            job_result_seen = true;
            break;
        }
    }
    drop(stdout_rx);

    if let Some(task) = operator_task {
        task.await??;
    } else {
        return Err(eyre!(
            "CLI output never emitted job_submitted event: {observed_lines:?}"
        ));
    }

    let stdout_result = stdout_handle
        .join()
        .map_err(|_| eyre!("stdout reader panicked"))?;
    stdout_result?;
    let stderr_result = stderr_handle
        .join()
        .map_err(|_| eyre!("stderr reader panicked"))?;
    let stderr_output = stderr_result.map_err(|e| eyre!(format!("failed to read stderr: {e}")))?;

    let status = child
        .wait()
        .map_err(|e| eyre!("failed to wait for cargo-tangle: {e}"))?;
    if !status.success() {
        return Err(eyre!(
            "CLI command failed with status {status:?}: {stderr_output}"
        ));
    }

    if !job_result_seen {
        return Err(eyre!(
            "CLI output missing job result notification: {observed_lines:?}"
        ));
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn jobs_submit_and_watch_result_roundtrip() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping jobs_submit_and_watch_result_roundtrip (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match spawn_harness("jobs_submit_and_watch_result_roundtrip").await? {
        Some(harness) => harness,
        None => return Ok(()),
    };

    let owner_keystore = TempDir::new()?;
    let owner_keystore_path = owner_keystore.path().join("keys");
    fs::create_dir_all(&owner_keystore_path)?;
    seed_private_key(&owner_keystore_path, SERVICE_OWNER_PRIVATE_KEY)?;

    let submit_args = TangleClientArgs {
        http_rpc_url: harness.http_endpoint().clone(),
        ws_rpc_url: harness.ws_endpoint().clone(),
        keystore_path: owner_keystore_path.clone(),
        tangle_contract: format!("{:#x}", harness.tangle_contract),
        restaking_contract: format!("{:#x}", harness.restaking_contract),
        status_registry_contract: Some(format!("{:#x}", harness.status_registry_contract)),
    };

    let service_id = LOCAL_SERVICE_ID;
    let blueprint_id = LOCAL_BLUEPRINT_ID;
    let submit_client = submit_args.connect(blueprint_id, Some(service_id)).await?;

    let payload = Bytes::from_static(b"hello-jobs");
    let submission = submit_job_call(&submit_client, service_id, 0, payload.clone()).await?;

    let watcher = submit_client.clone();
    let watch_task = tokio::spawn(async move {
        wait_for_job_result(
            &watcher,
            service_id,
            submission.call_id,
            Duration::from_secs(45),
        )
        .await
    });

    sleep(Duration::from_millis(250)).await;
    let operator_client = build_operator_client(&harness, blueprint_id, service_id).await?;
    let expected_output = b"result-ok".to_vec();
    operator_client
        .submit_result(
            service_id,
            submission.call_id,
            Bytes::from(expected_output.clone()),
        )
        .await
        .map_err(|e| eyre!(e.to_string()))?;

    let observed = watch_task
        .await
        .map_err(|e| eyre!(format!("watch task failed: {e}")))??;
    assert_eq!(observed, expected_output);

    Ok(())
}

async fn build_operator_client(
    harness: &TangleEvmHarness,
    blueprint_id: u64,
    service_id: u64,
) -> Result<TangleEvmClient> {
    let settings = TangleEvmSettings {
        blueprint_id,
        service_id: Some(service_id),
        tangle_contract: harness.tangle_contract,
        restaking_contract: harness.restaking_contract,
        status_registry_contract: harness.status_registry_contract,
    };
    let config = TangleEvmClientConfig::new(
        harness.http_endpoint().clone(),
        harness.ws_endpoint().clone(),
        "memory://operator-client",
        settings,
    )
    .test_mode(true);

    let keystore = Keystore::new(KeystoreConfig::new().in_memory(true))?;
    insert_default_operator_key(&keystore).map_err(|e| eyre!(e))?;
    TangleEvmClient::with_keystore(config, keystore)
        .await
        .map_err(|e| eyre!(e.to_string()))
}

fn seed_private_key(path: &Path, hex_key: &str) -> Result<()> {
    let keystore = Keystore::new(KeystoreConfig::new().fs_root(path))?;
    let secret = Vec::from_hex(hex_key)?;
    let signing_key = K256SigningKey::from_bytes(&secret)?;
    keystore.insert::<blueprint_crypto::k256::K256Ecdsa>(&signing_key)?;
    Ok(())
}

fn extract_call_id_from_json(value: &Value) -> Option<u64> {
    (value.get("event").and_then(Value::as_str) == Some("job_submitted"))
        .then(|| value.get("call_id").and_then(Value::as_u64))
        .flatten()
}

fn extract_call_id_from_text(line: &str) -> Option<u64> {
    if let Some(idx) = line.find("Call ID:") {
        let suffix = &line[idx + "Call ID:".len()..];
        let digits: String = suffix
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .collect();
        if !digits.is_empty() {
            return digits.parse().ok();
        }
    }
    None
}

fn matches_json_job_result(value: Option<&Value>) -> bool {
    value
        .and_then(|val| val.get("event").and_then(Value::as_str))
        .map_or(false, |event| event == "job_result")
}
