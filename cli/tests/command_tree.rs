//! Every advertised command renders help through the shipped executable for its feature set.
use std::collections::BTreeSet;
use std::process::Command;

const FULL: &str = "attest blueprint delegator dev harness key operator";

fn help(path: &[String]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_cargo-tangle"))
        .args(path)
        .arg("--help")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{path:?} --help failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
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
    let root = help(&[]);
    assert_eq!(root, help(&["tangle".to_owned()]), "cargo's tangle prefix");
    let roots = subcommands(&root);
    assert_eq!(roots, expected);
    let mut pending: Vec<Vec<String>> = roots.into_iter().map(|root| vec![root]).collect();
    let mut visited = 1;
    while let Some(path) = pending.pop() {
        visited += 1;
        assert!(visited < 300, "unexpected recursive command graph");
        for child in subcommands(&help(&path)) {
            pending.push([path.as_slice(), &[child]].concat());
        }
    }
}
