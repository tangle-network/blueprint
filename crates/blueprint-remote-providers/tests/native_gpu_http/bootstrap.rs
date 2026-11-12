//! Execute the cloud-init process payload against child-process peers.
use serde_json::{Value, json};

use std::os::unix::fs::PermissionsExt;
use std::process::Command;

pub(super) fn execute_cloud_init_startup(document: &str) {
    let temp = tempfile::tempdir().unwrap();
    let document: Value = serde_yaml::from_str(document).unwrap();
    let rebase = |path: &str| temp.path().join(path.trim_start_matches('/'));
    for file in document["write_files"].as_array().unwrap() {
        let path = rebase(file["path"].as_str().unwrap());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, file["content"].as_str().unwrap()).unwrap();
        let mode = u32::from_str_radix(file["permissions"].as_str().unwrap(), 8).unwrap();
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).unwrap();
        assert_eq!(file["owner"], "root:root");
    }
    let startup: Vec<&str> = document["runcmd"][0]
        .as_array()
        .unwrap()
        .iter()
        .map(|arg| arg.as_str().unwrap())
        .collect();
    launch(&temp, &startup);
}

/// The same payload as a bash startup script: run it with its roots under a temp dir, then launch.
pub(super) fn execute_startup_script(script: &str) {
    let temp = tempfile::tempdir().unwrap();
    let (body, start) = script.trim_end().rsplit_once('\n').unwrap();
    let root = temp.path().display().to_string();
    let body = body
        .replace(" /etc/", &format!(" {root}/etc/"))
        .replace(" /usr/local/", &format!(" {root}/usr/local/"));
    let output = Command::new("bash")
        .args(["-c", &body])
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "startup script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    for (path, mode) in [
        ("etc/tangle-gpu/workload.env", 0o600),
        ("etc/tangle-gpu/workload.json", 0o600),
        ("usr/local/lib/tangle-gpu-start.py", 0o700),
    ] {
        let metadata = std::fs::metadata(temp.path().join(path)).unwrap();
        assert_eq!(metadata.permissions().mode() & 0o777, mode, "{path}");
    }
    launch(&temp, &start.split(' ').collect::<Vec<_>>());
}

fn launch(temp: &tempfile::TempDir, startup: &[&str]) {
    let rebase = |path: &str| temp.path().join(path.trim_start_matches('/'));
    let config_path = rebase(startup[2]);
    let mut config: Value = serde_json::from_slice(&std::fs::read(&config_path).unwrap()).unwrap();
    config["env_file"] = json!(rebase(config["env_file"].as_str().unwrap()));
    std::fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();
    let bin = temp.path().join("bin");
    std::fs::create_dir(&bin).unwrap();
    let docker = r#"#!/usr/bin/env python3
import json, os, pathlib, stat, sys
arguments = sys.argv[1:]
path = pathlib.Path(arguments[arguments.index('--env-file') + 1])
receipt = {'arguments': arguments,
           'secret_loaded': 'MODEL_KEY=synthetic-bootstrap-secret\n' in path.read_text(),
           'env_file_mode': stat.S_IMODE(path.stat().st_mode)}
pathlib.Path(os.environ['BOOTSTRAP_RECEIPT']).write_text(json.dumps(receipt))
"#;
    for (name, source) in [("docker", docker), ("systemctl", "#!/bin/sh\nexit 0\n")] {
        let path = bin.join(name);
        std::fs::write(&path, source).unwrap();
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let receipt_path = temp.path().join("receipt.json");
    let output = Command::new(startup[0])
        .arg(rebase(startup[1]))
        .arg(config_path)
        .env_clear()
        .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
        .env("BOOTSTRAP_RECEIPT", &receipt_path)
        .output()
        .unwrap();
    for stream in [&output.stdout, &output.stderr] {
        assert!(!String::from_utf8_lossy(stream).contains("synthetic-bootstrap-secret"));
    }
    assert!(
        output.status.success(),
        "startup child failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&std::fs::read(receipt_path).unwrap()).unwrap();
    assert_eq!(receipt["secret_loaded"], true);
    assert_eq!(receipt["env_file_mode"], 0o600);
    let arguments = receipt["arguments"].as_array().unwrap();
    assert!(!arguments.iter().any(|arg| {
        arg.as_str()
            .is_some_and(|arg| arg.contains("synthetic-bootstrap-secret"))
    }));
    let image = arguments
        .iter()
        .position(|arg| arg == "example.invalid/workload:v1")
        .unwrap();
    assert_eq!(arguments[image - 1], "--");
    assert!(
        arguments
            .windows(2)
            .any(|pair| pair == [json!("--publish"), json!("8000:8000")])
    );
}
