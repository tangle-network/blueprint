//! CLI surface tests for the blueprint binary version upgrade flow.
//!
//! These tests focus on argument parsing and validation (the things we can
//! exercise without the on-chain `BlueprintsBinaryVersions` facet wired into
//! the test harness). End-to-end coverage that requires the facet ships
//! once `LocalTestnet.s.sol` registers it; this file is the contract for the
//! CLI's parse-only surface.

use std::fs;

use blueprint_testing_utils::anvil::{seed_operator_key, tangle::LOCAL_BLUEPRINT_ID};
use color_eyre::eyre::{Result, eyre};
use tempfile::TempDir;

use crate::tests::util::{
    RUN_TNT_E2E_ENV, cargo_tangle_cmd, is_e2e_enabled, network_cli_args, run_cli_command,
    spawn_harness,
};

fn seed_keystore(path: &std::path::Path) -> Result<()> {
    seed_operator_key(path).map_err(|e| eyre!(e.to_string()))?;
    Ok(())
}

// ─── pure parse-only tests ────────────────────────────────────────────────────
//
// We exercise `--help` and the misuse paths; clap validates the surface without
// the harness. These tests do NOT need anvil and run in CI by default.

#[test]
fn help_for_publish_version() -> Result<()> {
    let output = cargo_tangle_cmd()?
        .args(["blueprint", "publish-version", "--help"])
        .output()
        .map_err(|e| eyre!("running help: {e}"))?;
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--blueprint-id"));
    assert!(stdout.contains("--binary"));
    assert!(stdout.contains("--binary-uri"));
    assert!(stdout.contains("--pin-to-ipfs"));
    assert!(stdout.contains("--attestation-bundle"));
    Ok(())
}

#[test]
fn help_for_attest_submit() -> Result<()> {
    let output = cargo_tangle_cmd()?
        .args(["attest", "submit", "--help"])
        .output()
        .map_err(|e| eyre!("running help: {e}"))?;
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--blueprint-id"));
    assert!(stdout.contains("--version-id"));
    assert!(stdout.contains("--report"));
    assert!(stdout.contains("--kind"));
    assert!(stdout.contains("--severity"));
    assert!(stdout.contains("--expires-in"));
    Ok(())
}

#[test]
fn help_for_service_set_policy() -> Result<()> {
    let output = cargo_tangle_cmd()?
        .args(["blueprint", "service", "set-policy", "--help"])
        .output()
        .map_err(|e| eyre!("running help: {e}"))?;
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--service-id"));
    assert!(stdout.contains("--policy"));
    // Each variant of UpgradePolicyArg should be exposed.
    assert!(stdout.contains("AUTO"));
    assert!(stdout.contains("APPROVE"));
    assert!(stdout.contains("MANUAL"));
    Ok(())
}

#[test]
fn invalid_policy_value_is_rejected() -> Result<()> {
    let output = cargo_tangle_cmd()?
        .args([
            "blueprint",
            "service",
            "set-policy",
            "--service-id",
            "1",
            "--policy",
            "FAST",
        ])
        .output()
        .map_err(|e| eyre!("running invalid policy: {e}"))?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("FAST") || stderr.contains("invalid value"),
        "unexpected stderr: {stderr}"
    );
    Ok(())
}

#[test]
fn invalid_severity_value_is_rejected() -> Result<()> {
    let output = cargo_tangle_cmd()?
        .args([
            "attest",
            "submit",
            "--blueprint-id",
            "1",
            "--version-id",
            "0",
            "--report",
            "ipfs://bafy...",
            "--kind",
            "AUDIT",
            "--severity",
            "catastrophic",
        ])
        .output()
        .map_err(|e| eyre!("running invalid severity: {e}"))?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("catastrophic") || stderr.contains("invalid value"),
        "unexpected stderr: {stderr}"
    );
    Ok(())
}

#[test]
fn help_for_ship_advertises_ci_flags() -> Result<()> {
    // The ship wizard is the single entrypoint we promote to CI users. Its
    // --help MUST surface every flag the GitHub Actions composite passes,
    // otherwise the action breaks at runtime with "unrecognized argument".
    let output = cargo_tangle_cmd()?
        .args(["blueprint", "ship", "--help"])
        .output()
        .map_err(|e| eyre!("running ship help: {e}"))?;
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for flag in [
        "--yes",
        "--no-build",
        "--binary",
        "--binary-uri",
        "--pin-ipfs",
        "--attestation-bundle",
        "--attestation-hash",
        "--promote",
        "--no-promote",
        "--policy-services",
        "--dry-run",
        "--blueprint-id",
        "--json",
    ] {
        assert!(
            stdout.contains(flag),
            "ship --help missing {flag}: {stdout}"
        );
    }
    Ok(())
}

#[test]
fn ship_rejects_mutually_exclusive_attestation_flags() -> Result<()> {
    // --attestation-bundle and --attestation-hash both feed the on-chain
    // `attestationHash` slot; allowing both would let an operator silently
    // ship a sigstore bundle that disagrees with the hash. Clap must reject.
    let dir = TempDir::new()?;
    let bundle = dir.path().join("bundle.json");
    fs::write(&bundle, b"x").unwrap();
    let output = cargo_tangle_cmd()?
        .args([
            "blueprint",
            "ship",
            "--yes",
            "--blueprint-id",
            "1",
            "--binary",
            bundle.to_string_lossy().as_ref(),
            "--attestation-bundle",
            bundle.to_string_lossy().as_ref(),
            "--attestation-hash",
            "0x0000000000000000000000000000000000000000000000000000000000000000",
        ])
        .output()
        .map_err(|e| eyre!("running ship with conflicting attestation flags: {e}"))?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cannot be used")
            || stderr.contains("conflicts")
            || stderr.contains("--attestation"),
        "expected mutual-exclusion error, got: {stderr}"
    );
    Ok(())
}

#[test]
fn ship_rejects_promote_with_no_promote() -> Result<()> {
    // Belt-and-suspenders against operator footgun: the wizard treats these
    // as conflicting flags so we never land in an ambiguous state.
    let output = cargo_tangle_cmd()?
        .args(["blueprint", "ship", "--yes", "--promote", "--no-promote"])
        .output()
        .map_err(|e| eyre!("running ship with both promote flags: {e}"))?;
    assert!(!output.status.success());
    Ok(())
}

#[test]
fn help_for_service_upgrades_lists_manager_url() -> Result<()> {
    // upgrades / upgrade-local / upgrade-whitelist all go through the
    // manager's local RPC. The --manager-url flag must be discoverable.
    let output = cargo_tangle_cmd()?
        .args(["blueprint", "service", "upgrades", "--help"])
        .output()
        .map_err(|e| eyre!("running upgrades help: {e}"))?;
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--service-id"));
    assert!(stdout.contains("--manager-url"));
    Ok(())
}

#[test]
fn help_for_upgrade_skip_requires_reason() -> Result<()> {
    let output = cargo_tangle_cmd()?
        .args(["blueprint", "service", "upgrade-skip", "--help"])
        .output()
        .map_err(|e| eyre!("running upgrade-skip help: {e}"))?;
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--reason"));
    assert!(stdout.contains("--version-id"));
    Ok(())
}

#[test]
fn upgrade_skip_without_reason_is_rejected() -> Result<()> {
    // The `reason` field lands in the manager's audit log; making it
    // optional would defeat the entire purpose of recording skips.
    let output = cargo_tangle_cmd()?
        .args([
            "blueprint",
            "service",
            "upgrade-skip",
            "--service-id",
            "1",
            "--version-id",
            "2",
        ])
        .output()
        .map_err(|e| eyre!("running upgrade-skip without reason: {e}"))?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--reason"),
        "expected reason error: {stderr}"
    );
    Ok(())
}

#[test]
fn publish_version_requires_binary_uri_or_pin() -> Result<()> {
    // Create a temp binary so the file path check passes; the failure must be
    // about the missing URI, not the missing file.
    let dir = TempDir::new()?;
    let binary = dir.path().join("artifact.bin");
    fs::write(&binary, b"placeholder").unwrap();
    let keystore = dir.path().join("keystore");
    fs::create_dir_all(&keystore)?;
    seed_keystore(&keystore)?;

    let output = cargo_tangle_cmd()?
        .args([
            "blueprint",
            "publish-version",
            "--http-rpc-url",
            "http://127.0.0.1:65535", // bogus — we never get to the RPC call
            "--ws-rpc-url",
            "ws://127.0.0.1:65535",
            "--keystore-path",
            keystore.to_string_lossy().as_ref(),
            "--tangle-contract",
            "0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9",
            "--staking-contract",
            "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0",
            "--status-registry-contract",
            "0x5f3f1dBD7B74C6B46e8c44f98792A1dAf8d69154",
            "--blueprint-id",
            "1",
            "--binary",
            binary.to_string_lossy().as_ref(),
        ])
        .output()
        .map_err(|e| eyre!("running publish without uri: {e}"))?;
    assert!(!output.status.success());
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.contains("--binary-uri") || combined.contains("--pin-to-ipfs"),
        "unexpected output: {combined}"
    );
    Ok(())
}

// ─── e2e tests (gated on RUN_TNT_E2E) ─────────────────────────────────────────
//
// The harness wires up the Tangle diamond but does NOT yet register the
// `BlueprintsBinaryVersions` / `BlueprintsBinaryAttestations` facets (those
// land via the upcoming LocalTestnet update). When that update ships these
// tests can be flipped from "expect-fail" to "expect-pass" by deleting the
// `expected_fail` guard.

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn list_versions_returns_empty_or_skips() -> Result<()> {
    if !is_e2e_enabled() {
        eprintln!("Skipping list_versions_returns_empty_or_skips (set {RUN_TNT_E2E_ENV}=1)");
        return Ok(());
    }
    let harness = match spawn_harness("list_versions_returns_empty_or_skips").await? {
        Some(h) => h,
        None => return Ok(()),
    };
    let dir = TempDir::new()?;
    let keystore = dir.path().join("keystore");
    fs::create_dir_all(&keystore)?;
    seed_keystore(&keystore)?;

    let mut args = vec![
        "blueprint".to_string(),
        "list-versions".to_string(),
        "--blueprint-id".to_string(),
        LOCAL_BLUEPRINT_ID.to_string(),
        "--json".to_string(),
    ];
    args.extend(network_cli_args(&harness, &keystore));

    // The facet is not yet registered on the harness; the call MUST fail with
    // a "function selector not found" / revert. We accept either: success
    // with `versions: []` (facet registered) OR a clean Err (facet missing).
    match run_cli_command(&args) {
        Ok(out) => {
            // Facet present → empty version list expected for LOCAL_BLUEPRINT_ID.
            let parsed: serde_json::Value = serde_json::from_str(&out.stdout)?;
            let versions = parsed
                .get("versions")
                .and_then(|v| v.as_array())
                .ok_or_else(|| eyre!("expected `versions` array: {}", out.stdout))?;
            // If the facet IS deployed, no versions exist for the default blueprint.
            assert!(
                versions.is_empty(),
                "default harness blueprint should not have published versions yet"
            );
        }
        Err(e) => {
            let msg = e.to_string().to_lowercase();
            // Allow "selector" / "revert" / "decode" markers — the facet is
            // simply not wired into LocalTestnet yet.
            assert!(
                msg.contains("selector")
                    || msg.contains("revert")
                    || msg.contains("decode")
                    || msg.contains("execution reverted")
                    || msg.contains("call failed"),
                "unexpected error shape: {msg}"
            );
            eprintln!(
                "BlueprintsBinaryVersions facet not yet registered on harness — error tolerated."
            );
        }
    }
    Ok(())
}
