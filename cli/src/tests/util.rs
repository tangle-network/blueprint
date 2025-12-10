use assert_cmd::Command;
use blueprint_testing_utils::anvil::{TangleEvmHarness, missing_tnt_core_artifacts};
use color_eyre::eyre::{Result, eyre};
use serde_json::Value;
use std::{borrow::Cow, env, path::Path};

use alloy_provider::{Provider, ProviderBuilder};
use serde_json::json;

/// Environment variable that enables end-to-end CLI tests.
pub const RUN_TNT_E2E_ENV: &str = "RUN_TNT_E2E";

/// Returns `true` when end-to-end tests should run.
pub fn is_e2e_enabled() -> bool {
    match env::var(RUN_TNT_E2E_ENV) {
        Ok(value) => {
            let trimmed = value.trim();
            !trimmed.is_empty() && trimmed != "0"
        }
        Err(_) => false,
    }
}

/// Build the standard network arguments expected by cargo-tangle CLI tests.
pub fn network_cli_args(harness: &TangleEvmHarness, keystore_path: &Path) -> Vec<String> {
    vec![
        "--http-rpc-url".into(),
        harness.http_endpoint().as_str().to_string(),
        "--ws-rpc-url".into(),
        harness.ws_endpoint().as_str().to_string(),
        "--keystore-path".into(),
        keystore_path.to_string_lossy().to_string(),
        "--tangle-contract".into(),
        format!("{:#x}", harness.tangle_contract),
        "--restaking-contract".into(),
        format!("{:#x}", harness.restaking_contract),
        "--status-registry-contract".into(),
        format!("{:#x}", harness.status_registry_contract),
    ]
}

/// Execute `cargo-tangle` with the provided arguments, capturing stdout/stderr.
pub fn run_cli_command(args: &[String]) -> Result<CliCommandOutput> {
    let output = Command::cargo_bin("cargo-tangle")
        .map_err(|e| eyre!(e))?
        .env("NO_COLOR", "1")
        .args(args.iter().map(|s| s.as_str()))
        .output()
        .map_err(|e| eyre!("failed to execute cargo-tangle: {e}"))?;

    if !output.status.success() {
        return Err(eyre!(
            "CLI command failed (status {:?}): {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(CliCommandOutput {
        stdout: String::from_utf8(output.stdout)?,
        stderr: String::from_utf8(output.stderr)?,
    })
}

/// Spawn the default Anvil harness or skip the test when artifacts are missing.
pub async fn spawn_harness(test_name: &str) -> Result<Option<TangleEvmHarness>> {
    match TangleEvmHarness::builder()
        .include_anvil_logs(false)
        .spawn()
        .await
    {
        Ok(harness) => Ok(Some(harness)),
        Err(err) => {
            if missing_tnt_core_artifacts(&err) {
                eprintln!("Skipping {test_name}: {err}");
                Ok(None)
            } else {
                Err(eyre!(err))
            }
        }
    }
}

/// Parse newline-delimited JSON output from the CLI.
pub fn parse_json_lines(stdout: &str) -> Result<Vec<Value>> {
    stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).map_err(|e| eyre!("invalid JSON line {line}: {e}")))
        .collect()
}

/// Output produced by `run_cli_command`.
pub struct CliCommandOutput {
    pub stdout: String,
    pub stderr: String,
}

/// Advance the Anvil chain time by the provided number of seconds and mine a block.
pub async fn advance_chain_time(harness: &TangleEvmHarness, seconds: u64) -> Result<()> {
    let provider = ProviderBuilder::new()
        .connect(harness.http_endpoint().as_str())
        .await
        .map_err(|e| eyre!(format!("failed to connect to anvil: {e}")))?;

    provider
        .raw_request::<_, Value>(Cow::Borrowed("evm_increaseTime"), json!([seconds]))
        .await
        .map_err(|e| eyre!(format!("failed to advance time: {e}")))?;

    provider
        .raw_request::<_, Value>(Cow::Borrowed("anvil_mine"), json!(["0x1"]))
        .await
        .map_err(|e| eyre!(format!("failed to mine block: {e}")))?;

    Ok(())
}
