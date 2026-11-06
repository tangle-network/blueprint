//! Unknown VM estimates through the shipped CLI process boundary.
#![cfg(feature = "remote-providers")]
use std::process::Command;

#[test]
fn a_missing_live_price_is_a_failed_estimate_without_an_invented_charge() {
    let home = tempfile::tempdir().unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_cargo-tangle"))
        .args([
            "tangle",
            "cloud",
            "estimate",
            "--provider",
            "aws",
            "--duration",
            "1h",
        ])
        .env_clear()
        .env("HOME", home.path())
        .env("XDG_CONFIG_HOME", home.path())
        .env("PATH", "/usr/bin:/bin")
        .current_dir(home.path())
        .output()
        .unwrap();
    assert!(
        !result.status.success(),
        "an unquoted AWS VM was priced from a handwritten fallback"
    );
    assert!(
        result.stdout.is_empty(),
        "failed estimates must not print invented prices"
    );
}
