//! SSH/Docker process contract only: no live host authentication or GPU visibility claim.
#![cfg(unix)]
use blueprint_remote_providers::ResourceSpec;
use blueprint_remote_providers::deployment::ssh::{
    ContainerRuntime, DeploymentConfig, SshConnection, SshDeploymentClient,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;

#[path = "fixtures/ssh_process.rs"]
mod ssh_process;
use ssh_process::SECRET;

#[tokio::test]
async fn ssh_worker() -> anyhow::Result<()> {
    let Ok(mode) = std::env::var("SSH_PROCESS_MODE") else {
        return Ok(());
    };
    tracing_subscriber::fmt()
        .with_max_level(tracing_subscriber::filter::LevelFilter::TRACE)
        .with_writer(std::io::stderr)
        .try_init()
        .ok();
    let connection = SshConnection {
        host: "gpu-peer.invalid".into(),
        user: "root".into(),
        ..Default::default()
    };
    let client = SshDeploymentClient::new(
        connection,
        ContainerRuntime::Docker,
        DeploymentConfig::default(),
    )
    .await;
    if mode == "untrusted" {
        assert!(
            client.is_err(),
            "unknown SSH identity must fail before any credential upload"
        );
        return Ok(());
    }
    let client = client?;
    let root = std::path::PathBuf::from(std::env::var_os("SSH_PROCESS_ROOT").unwrap());
    let secure = blueprint_remote_providers::deployment::secure_ssh::SecureSshClient::new(
        blueprint_remote_providers::deployment::secure_ssh::SecureSshConnection::new(
            "gpu-peer.invalid".into(),
            "root".into(),
        )?,
    );
    assert!(
        secure
            .write_protected_file(root.join("kept").to_str().unwrap(), SECRET.as_bytes())
            .await
            .is_err()
    );
    if mode == "failed-transfer" {
        let partial = root.join("partial");
        assert!(
            secure
                .write_protected_file(partial.to_str().unwrap(), SECRET.as_bytes())
                .await
                .is_err()
        );
        assert!(
            !partial.exists(),
            "failed transfer retained its owned credential staging file"
        );
        return Ok(());
    }
    let spec = ResourceSpec {
        gpu_count: Some(2),
        gpu_class: Some("nvidia-h200".into()),
        gpu_vram_gb: Some(140),
        ports: vec![8000, 12345],
        endpoint_port: Some(8000),
        ..ResourceSpec::basic()
    };
    let env = HashMap::from([("SECRET_TOKEN".into(), SECRET.into())]);
    client
        .deploy_blueprint("example.invalid/workload:v1", &spec, env.clone())
        .await?;
    let binary = b"#!/bin/sh\nexit 0\n";
    client
        .deploy_github_binary(
            "https://example.invalid/blueprint",
            &format!("{:x}", Sha256::digest(binary)),
            "blueprint",
            &spec,
            &env,
        )
        .await?;
    Ok(())
}

#[test]
fn gpu_ports_and_credentials_cross_processes_without_secret_argv_or_logs() -> anyhow::Result<()> {
    for mode in ["trusted", "untrusted", "failed-transfer"] {
        let temp = tempfile::tempdir()?;
        let root = temp.path();
        let output =
            ssh_process::command(root, &std::env::current_exe()?, "ssh_worker", mode)?.output()?;
        for stream in [&output.stdout, &output.stderr] {
            assert!(
                !String::from_utf8_lossy(stream).contains(SECRET),
                "workload secret reached logging"
            );
        }
        assert!(
            output.status.success(),
            "process proof failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let argv = std::fs::read_to_string(root.join("ssh-argv.jsonl"))?;
        assert!(!argv.contains(SECRET), "workload secret reached SSH argv");
        assert_eq!(std::fs::read_to_string(root.join("kept"))?, "preexisting");
        if mode != "trusted" {
            assert!(!root.join("docker.json").exists());
            continue;
        }
        let receipt: Value = serde_json::from_slice(&std::fs::read(root.join("docker.json"))?)?;
        assert_eq!(receipt["loaded"], true);
        assert_eq!(receipt["mode"], 0o600);
        let args = receipt["arguments"].as_array().unwrap();
        assert!(
            args.windows(2)
                .any(|pair| pair == [Value::from("--gpus"), Value::from("2")])
        );
        for port in [8000, 12345] {
            assert!(args.contains(&Value::from(format!("127.0.0.1:{port}:{port}"))));
        }
        assert!(!args.contains(&Value::from("127.0.0.1:9615:9615")));
        assert!(!receipt.to_string().contains(SECRET));
        let native = root.join("opt/blueprint/config/blueprint.env");
        assert!(std::fs::read_to_string(&native)?.contains(SECRET));
        assert_eq!(
            std::fs::metadata(native)?.permissions().mode() & 0o777,
            0o600
        );
    }
    Ok(())
}
