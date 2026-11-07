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

const SECRET: &str = "synthetic-ssh-workload-secret";

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
    let spec = ResourceSpec {
        gpu_count: Some(2),
        gpu_class: Some("nvidia-h200".into()),
        gpu_vram_gb: Some(140),
        ports: vec![12345],
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
    for mode in ["trusted", "untrusted"] {
        let temp = tempfile::tempdir()?;
        let root = temp.path();
        let bin = root.join("bin");
        std::fs::create_dir(&bin)?;
        std::fs::create_dir_all(root.join(".ssh"))?;
        std::fs::create_dir_all(root.join("etc/systemd/system"))?;
        assert!(
            std::process::Command::new("ssh-keygen")
                .args(["-q", "-t", "ed25519", "-N", "", "-f"])
                .arg(root.join("host"))
                .status()?
                .success()
        );
        std::fs::write(
            root.join(".ssh/known_hosts"),
            format!(
                "gpu-peer.invalid {}",
                std::fs::read_to_string(root.join("host.pub"))?
            ),
        )?;
        let ssh = r#"#!/usr/bin/env python3
import json,os,pathlib,subprocess,sys
root=pathlib.Path(os.environ['SSH_PROCESS_ROOT'])
args=sys.argv[1:]
with (root/'ssh-argv.jsonl').open('a') as f: f.write(json.dumps(args)+'\n')
trusted='StrictHostKeyChecking=yes' in args and not any('/dev/null' in x for x in args)
trusted=trusted and (root/'.ssh/known_hosts').read_text().startswith('gpu-peer.invalid ssh-ed25519 ')
if os.environ['SSH_PROCESS_MODE']=='untrusted' or not trusted: sys.exit(255)
command=args[-1].replace('/opt/',str(root)+'/opt/').replace('/etc/systemd/',str(root)+'/etc/systemd/')
payload=sys.stdin.buffer.read() if command=='sh -se' or command.startswith("sh -c ") else None
if command=='sh -se': payload=payload.replace(b'/opt/',(str(root)+'/opt/').encode()).replace(b'/etc/systemd/',(str(root)+'/etc/systemd/').encode())
sys.exit(subprocess.run(['/bin/sh','-c',command],input=payload).returncode)
"#;
        let docker = r#"#!/usr/bin/env python3
import json,os,pathlib,stat,sys
root=pathlib.Path(os.environ['SSH_PROCESS_ROOT']);args=sys.argv[1:]
if args[0]=='create':
    path=pathlib.Path(args[args.index('--env-file')+1])
    receipt={'arguments':args,'mode':stat.S_IMODE(path.stat().st_mode),'loaded':os.environ['SSH_PROCESS_SECRET'] in path.read_text()}
    (root/'docker.json').write_text(json.dumps(receipt));print('a'*64)
elif args[0]=='inspect': print(json.dumps([{'State':{'Status':'running'},'NetworkSettings':{'Ports':{}}}]))
elif args[0]=='--version': print('Docker process peer')
"#;
        let curl = r#"#!/usr/bin/env python3
import pathlib,sys
pathlib.Path(sys.argv[sys.argv.index('-o')+1]).write_bytes(b'#!/bin/sh\nexit 0\n')
"#;
        let sudo = "#!/bin/sh\n[ \"$1\" != -n ] || shift\ncase \"$1\" in useradd|chown) exit 0;; esac\nexec \"$@\"\n";
        for (name, source) in [
            ("ssh", ssh),
            ("docker", docker),
            ("curl", curl),
            ("sudo", sudo),
            ("systemctl", "#!/bin/sh\nexit 0\n"),
            ("sleep", "#!/bin/sh\nexit 0\n"),
        ] {
            let path = bin.join(name);
            std::fs::write(&path, source)?;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))?;
        }
        let output = std::process::Command::new(std::env::current_exe()?)
            .args(["--exact", "ssh_worker", "--nocapture", "--test-threads=1"])
            .env_clear()
            .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
            .env("HOME", root)
            .env("SSH_PROCESS_ROOT", root)
            .env("SSH_PROCESS_MODE", mode)
            .env("SSH_PROCESS_SECRET", SECRET)
            .output()?;
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
        if mode == "untrusted" {
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
