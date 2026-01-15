use assert_cmd::Command;
use blueprint_testing_utils::anvil::{TangleEvmHarness, missing_tnt_core_artifacts};
use color_eyre::eyre::{Result, eyre};
use serde_json::Value;
use std::{
    env,
    path::{Path, PathBuf},
    sync::OnceLock,
};

/// Environment variable that enables end-to-end CLI tests.
pub const RUN_TNT_E2E_ENV: &str = "RUN_TNT_E2E";

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

pub fn cargo_tangle_bin() -> Result<PathBuf> {
    static BIN_PATH: OnceLock<PathBuf> = OnceLock::new();

    if let Some(path) = BIN_PATH.get() {
        return Ok(path.clone());
    }

    if let Ok(path) = env::var("CARGO_BIN_EXE_cargo-tangle") {
        let path = PathBuf::from(path);
        if path.is_file() {
            let _ = BIN_PATH.set(path.clone());
            return Ok(path);
        }
    }

    let target_dir = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace_root().join("target"));
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let exe_name = format!("cargo-tangle{}", env::consts::EXE_SUFFIX);
    let bin_path = target_dir.join(&profile).join(exe_name);
    let force_rebuild = match env::var(RUN_TNT_E2E_ENV) {
        Ok(value) => {
            let trimmed = value.trim();
            !trimmed.is_empty() && trimmed != "0"
        }
        Err(_) => false,
    };

    if force_rebuild || !bin_path.is_file() {
        let mut build = std::process::Command::new("cargo");
        build
            .arg("build")
            .arg("--bin")
            .arg("cargo-tangle")
            .arg("--package")
            .arg("cargo-tangle")
            .current_dir(workspace_root());
        if profile == "release" {
            build.arg("--release");
        }
        if let Some(dir) = env::var_os("CARGO_TARGET_DIR") {
            build.env("CARGO_TARGET_DIR", dir);
        }
        let status = build
            .status()
            .map_err(|e| eyre!("failed to build cargo-tangle: {e}"))?;
        if !status.success() {
            return Err(eyre!("cargo-tangle build failed with status {status}"));
        }
    }

    if !bin_path.is_file() {
        return Err(eyre!(
            "cargo-tangle binary not found at {}",
            bin_path.display()
        ));
    }

    let _ = BIN_PATH.set(bin_path.clone());
    Ok(bin_path)
}

pub fn cargo_tangle_cmd() -> Result<Command> {
    let bin = cargo_tangle_bin()?;
    Ok(Command::new(bin))
}

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
    let output = cargo_tangle_cmd()?
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
