//! Every advertised command renders help through the shipped executable for its feature set.
use std::collections::BTreeSet;
use std::process::Command;

const FULL: &str = "attest blueprint delegator dev harness key operator";

fn help(path: &[String]) -> Result<String, String> {
    let output = Command::new(env!("CARGO_BIN_EXE_cargo-tangle"))
        .args(path)
        .arg("--help")
        .output()
        .unwrap();
    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err(format!(
            "{path:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn subcommands(help: &str) -> BTreeSet<String> {
    let listed = help.split_once("Commands:\n").map_or("", |(_, rest)| rest);
    listed
        .lines()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| line.strip_prefix("  "))
        .filter(|line| line.starts_with(|c: char| c.is_ascii_lowercase()))
        .filter_map(|line| line.split_whitespace().next())
        .filter(|name| *name != "help")
        .map(str::to_owned)
        .collect()
}

#[test]
fn every_command_in_the_built_feature_set_renders_help() {
    let mut expected = BTreeSet::new();
    if cfg!(feature = "full") {
        expected.extend(FULL.split(' ').map(str::to_owned));
    }
    if cfg!(feature = "remote-providers") {
        expected.insert("cloud".to_owned());
    }
    let root = help(&[]).unwrap();
    assert_eq!(Ok(root.clone()), help(&["tangle".to_owned()]));
    let roots = subcommands(&root);
    assert_eq!(roots, expected);
    let mut pending: Vec<Vec<String>> = roots.into_iter().map(|root| vec![root]).collect();
    let mut failures = Vec::new();
    while let Some(path) = pending.pop() {
        match help(&path) {
            Ok(text) => {
                for child in subcommands(&text) {
                    pending.push([path.as_slice(), &[child]].concat());
                }
            }
            Err(failure) => failures.push(failure),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[cfg(feature = "remote-providers")]
#[test]
fn an_unknown_cloud_provider_is_rejected() {
    let output = Command::new(env!("CARGO_BIN_EXE_cargo-tangle"))
        .args(["cloud", "configure", "invalid"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid value 'invalid'"));
}
