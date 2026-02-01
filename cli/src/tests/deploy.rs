use std::fs;

use blueprint_testing_utils::anvil::{
    TangleHarness, missing_tnt_core_artifacts,
    tangle::{LOCAL_BLUEPRINT_ID, LOCAL_SERVICE_ID},
};
use color_eyre::eyre::{Result, eyre};
use tempfile::TempDir;

use crate::tests::util::{RUN_TNT_E2E_ENV, cargo_tangle_cmd, is_e2e_enabled};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn deploys_blueprint_to_devnet() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping deploys_blueprint_to_devnet (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }

    let harness = match TangleHarness::builder()
        .include_anvil_logs(false)
        .spawn()
        .await
    {
        Ok(harness) => harness,
        Err(err) => {
            if missing_tnt_core_artifacts(&err) {
                eprintln!("Skipping deploys_blueprint_to_devnet: {err}");
                return Ok(());
            }
            return Err(eyre!(err));
        }
    };

    let settings_dir = TempDir::new()?;
    let settings_path = settings_dir.path().join("settings.env");
    let settings_contents = format!(
        "BLUEPRINT_ID={}\nSERVICE_ID={}\nTANGLE_CONTRACT={:#x}\nRESTAKING_CONTRACT={:#x}\nSTATUS_REGISTRY_CONTRACT={:#x}\n",
        LOCAL_BLUEPRINT_ID,
        LOCAL_SERVICE_ID,
        harness.tangle_contract,
        harness.restaking_contract,
        harness.status_registry_contract,
    );
    fs::write(&settings_path, settings_contents)?;

    // Release the harness before running the deploy command so the CLI owns any Devnet stacks.
    drop(harness);

    let output = cargo_tangle_cmd()?
        .env("NO_COLOR", "1")
        .current_dir(settings_dir.path())
        .args([
            "blueprint",
            "deploy",
            "tangle",
            "--network",
            "devnet",
            "--exit-after-seconds",
            "15",
        ])
        .output()
        .map_err(|e| eyre!("failed to execute cargo-tangle: {e}"))?;

    if !output.status.success() {
        return Err(eyre!(
            "deployment command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8(output.stdout)?;
    assert!(
        stdout.contains("Deploying blueprint to local Anvil devnet"),
        "deploy output missing warmup log:\n{stdout}"
    );
    let summary_line = stdout
        .lines()
        .rev()
        .find(|line| line.contains("Deployment complete"))
        .ok_or_else(|| eyre!("missing deployment summary in output:\n{stdout}"))?;
    assert!(
        summary_line.contains(&format!("blueprint={LOCAL_BLUEPRINT_ID}")),
        "unexpected blueprint id in summary: {summary_line}"
    );
    let service_value = summary_line
        .split("service=")
        .nth(1)
        .and_then(|chunk| chunk.split_whitespace().next())
        .ok_or_else(|| eyre!("summary missing service id: {summary_line}"))?;
    let observed_service: u64 = service_value
        .parse()
        .map_err(|_| eyre!("invalid service id {service_value}"))?;
    assert_eq!(
        observed_service, LOCAL_SERVICE_ID,
        "deployment summary reported unexpected service id"
    );

    Ok(())
}
