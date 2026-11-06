//! Cloud behavior through the shipped CLI process boundary.
#![cfg(feature = "remote-providers")]

use std::process::{Command, Output};

fn command(args: &[&str]) -> Output {
    let home = tempfile::tempdir().unwrap();
    Command::new(env!("CARGO_BIN_EXE_cargo-tangle"))
        .args(["tangle", "cloud"])
        .args(args)
        .env_clear()
        .env("HOME", home.path())
        .env("XDG_CONFIG_HOME", home.path())
        .env("PATH", "/usr/bin:/bin")
        .current_dir(home.path())
        .output()
        .unwrap()
}

#[test]
fn invalid_duration_and_missing_deployment_fail_without_fabricated_results() {
    let duration = command(&["estimate", "--duration", "0h"]);
    assert!(
        !duration.status.success(),
        "zero duration was priced: {}",
        String::from_utf8_lossy(&duration.stdout)
    );
    assert!(
        duration.stdout.is_empty(),
        "invalid requests must not print estimates"
    );
    let status = command(&["status", "missing-deployment"]);
    assert!(
        !status.status.success(),
        "missing deployment was shown as running"
    );
    assert!(!String::from_utf8_lossy(&status.stdout).contains("54.123.45.67"));
}
