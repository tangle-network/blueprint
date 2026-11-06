#![cfg(all(feature = "remote-providers", unix, not(feature = "vm-sandbox")))]

use blueprint_manager::{
    config::{BlueprintManagerConfig, BlueprintManagerContext},
    rt::{
        ResourceLimits,
        service::{Service, Status},
    },
    sources::{BlueprintArgs, BlueprintEnvVars},
};
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, infra::traits::CloudProviderAdapter, providers::RunPodAdapter,
};
use blueprint_runner::config::{BlueprintEnvironment, Protocol};
use serde_json::json;
use std::{
    collections::HashMap,
    ffi::OsString,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_partial_json, method, path},
};

struct ChildBoundary {
    old_path: OsString,
    pid_file: PathBuf,
    remote_path_file: PathBuf,
}
impl Drop for ChildBoundary {
    fn drop(&mut self) {
        unsafe {
            std::env::set_var("PATH", &self.old_path);
        }
        stop_workload(&self.pid_file);
        if let Ok(path) = std::fs::read_to_string(&self.remote_path_file) {
            if let Some(directory) = Path::new(path.trim()).parent() {
                if directory.starts_with("/tmp")
                    && directory.file_name().is_some_and(|name| {
                        name.to_string_lossy().starts_with("blueprint-gpu-lease-")
                    })
                {
                    let _ = std::fs::remove_dir_all(directory);
                }
            }
        }
    }
}
fn stop_workload(pid_file: &Path) {
    if let Ok(pid) = std::fs::read_to_string(pid_file) {
        if pid.trim().parse::<u32>().is_ok() {
            let _ = std::process::Command::new("/bin/kill")
                .arg(pid.trim())
                .status();
        }
        let _ = std::fs::remove_file(pid_file);
    }
}
fn executable(path: &Path, source: &str) {
    std::fs::write(path, source).unwrap();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).unwrap();
}

#[tokio::test]
#[serial_test::serial(env)]
async fn manager_from_binary_allocates_once_uploads_keys_and_args_then_retires() {
    let temporary = tempfile::tempdir().unwrap();
    let tools = temporary.path().join("tools");
    std::fs::create_dir(&tools).unwrap();
    let trace = temporary.path().join("ssh-argv.jsonl");
    let pid_file = temporary.path().join("pid");
    let remote_path_file = temporary.path().join("remote-keystore-path");
    let arguments_file = temporary.path().join("actual-argv");
    let credential_file = temporary.path().join("actual-key");
    let permissions_file = temporary.path().join("actual-key-mode");
    let sentinel = temporary.path().join("must-not-exist");
    let payload = format!("space;$(touch {}) 'quoted'", sentinel.display());
    executable(
        &tools.join("ssh"),
        &format!(
            r#"#!/usr/bin/python3
import json, os, subprocess, sys
argv = sys.argv[1:]
with open({trace:?}, 'a') as log: log.write(json.dumps(argv) + '\n')
key = argv[argv.index('-i') + 1]
assert os.stat(key).st_mode & 0o777 == 0o600
assert 'StrictHostKeyChecking=accept-new' in argv
assert '-p' in argv and argv[argv.index('-p') + 1] == '30222'
script = sys.stdin.read()
result = subprocess.run(['/bin/sh'], input=script, text=True, stdout=subprocess.PIPE)
if 'nohup ./workload' in script and result.returncode == 0:
    with open({pid_file:?}, 'w') as pid: pid.write(result.stdout.strip())
sys.stdout.write(result.stdout)
sys.exit(result.returncode)
"#
        ),
    );
    executable(
        &tools.join("scp"),
        &format!(
            r#"#!/usr/bin/python3
import json, shutil, sys
argv = sys.argv[1:]
with open({trace:?}, 'a') as log: log.write(json.dumps(argv) + '\n')
assert 'StrictHostKeyChecking=yes' in argv
assert '-P' in argv and argv[argv.index('-P') + 1] == '30222'
shutil.copyfile(argv[-2], argv[-1].split(':', 1)[1])
"#
        ),
    );
    let old_path = std::env::var_os("PATH").unwrap();
    let _boundary = ChildBoundary {
        old_path: old_path.clone(),
        pid_file: pid_file.clone(),
        remote_path_file: remote_path_file.clone(),
    };
    unsafe {
        std::env::set_var(
            "PATH",
            format!("{}:{}", tools.display(), old_path.to_string_lossy()),
        );
    }

    let peer = MockServer::start().await;
    let retired = Arc::new(AtomicBool::new(false));
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"data":{"gpuTypes":[{
                "id":"NVIDIA B200","lowestPrice":{"uninterruptablePrice":2.0,"stockStatus":"High"}
            }]}})),
        )
        .mount(&peer)
        .await;
    Mock::given(method("POST"))
        .and(path("/pods"))
        .and(body_partial_json(
            json!({"gpuTypeIds":["NVIDIA B200"],"gpuCount":8}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id":"manager-node"})))
        .expect(1)
        .mount(&peer)
        .await;
    let retired_read = retired.clone();
    Mock::given(method("GET"))
        .and(path("/pods/manager-node"))
        .respond_with(move |_: &wiremock::Request| {
            if retired_read.load(Ordering::SeqCst) {
                ResponseTemplate::new(404)
            } else {
                ResponseTemplate::new(200).set_body_json(json!({
                    "id":"manager-node","gpuTypeId":"NVIDIA B200","desiredStatus":"RUNNING",
                    "publicIp":"127.0.0.1","portMappings":{"22":30222}
                }))
            }
        })
        .mount(&peer)
        .await;
    let retired_write = retired.clone();
    let pid_for_delete = pid_file.clone();
    Mock::given(method("DELETE"))
        .and(path("/pods/manager-node"))
        .respond_with(move |_: &wiremock::Request| {
            stop_workload(&pid_for_delete);
            retired_write.store(true, Ordering::SeqCst);
            ResponseTemplate::new(204)
        })
        .expect(1)
        .mount(&peer)
        .await;

    let mut providers: HashMap<CloudProvider, Box<dyn CloudProviderAdapter>> = HashMap::new();
    providers.insert(
        CloudProvider::RunPod,
        Box::new(RunPodAdapter::with_endpoint("account-test-only".into(), &peer.uri()).unwrap()),
    );
    let provisioner = Arc::new(
        CloudProvisioner::with_providers(providers)
            .with_lease_store(temporary.path().join("leases")),
    );
    let mut config = BlueprintManagerConfig::default();
    config.paths.data_dir = temporary.path().join("data");
    config.paths.cache_dir = temporary.path().join("cache");
    config.paths.runtime_dir = temporary.path().join("runtime");
    config.paths.keystore_uri = temporary
        .path()
        .join("keystore")
        .to_string_lossy()
        .into_owned();
    config.remote_deployment_opts.enable_remote_deployments = true;
    config.remote_deployment_opts.preferred_provider = Some("runpod".into());
    config.remote_deployment_opts.max_hourly_cost = 20.0;
    std::fs::create_dir(&config.paths.keystore_uri).unwrap();
    std::fs::write(
        Path::new(&config.paths.keystore_uri).join("operator.fixture"),
        b"synthetic-operator-key",
    )
    .unwrap();
    let keystore = config.paths.keystore_uri.clone();
    let context = BlueprintManagerContext::new(config)
        .await
        .unwrap()
        .with_cloud_provisioner(provisioner);
    let binary = temporary.path().join("blueprint-binary");
    executable(
        &binary,
        &format!(
            r#"#!/bin/sh
printf '%s\n' "$@" > '{arguments_file}'
printf '%s\n' "$KEYSTORE_URI" > '{remote_path_file}'
cat "$KEYSTORE_URI/operator.fixture" > '{credential_file}'
stat -c '%a' "$KEYSTORE_URI/operator.fixture" > '{permissions_file}'
while :; do sleep 1; done
"#,
            arguments_file = arguments_file.display(),
            remote_path_file = remote_path_file.display(),
            credential_file = credential_file.display(),
            permissions_file = permissions_file.display()
        ),
    );
    let limits = ResourceLimits {
        gpu_count: Some(8),
        gpu_class: Some("nvidia-b200".into()),
        gpu_min_vram_gb: Some(180),
        ..Default::default()
    };
    let environment = BlueprintEnvVars {
        http_rpc_endpoint: "http://127.0.0.1:8545".parse().unwrap(),
        ws_rpc_endpoint: "ws://127.0.0.1:8545".parse().unwrap(),
        #[cfg(feature = "tee")]
        kms_endpoint: "http://127.0.0.1:9000".parse().unwrap(),
        keystore_uri: keystore,
        data_dir: temporary.path().join("original-data"),
        blueprint_id: 1,
        service_id: 1,
        protocol: Protocol::Tangle,
        chain: None,
        bootnodes: String::new(),
        registration_mode: false,
        registration_capture_only: false,
        bridge_socket_path: None,
        tangle_contract: None,
        staking_contract: None,
        status_registry_contract: None,
    };
    let arguments = BlueprintArgs {
        test_mode: true,
        pretty: false,
        verbose: 0,
        dry_run: false,
        extra_args: vec![("--payload".into(), payload.clone())],
    };
    let mut service = Service::from_binary(
        &context,
        limits,
        &BlueprintEnvironment::default(),
        1,
        environment,
        arguments,
        &binary,
        "gpu-manager-contract",
        temporary.path(),
        &temporary.path().join("runtime"),
    )
    .await
    .unwrap();
    assert_eq!(
        std::fs::read(&credential_file).unwrap(),
        b"synthetic-operator-key"
    );
    assert_eq!(
        std::fs::read_to_string(&permissions_file).unwrap().trim(),
        "600"
    );
    assert_eq!(
        std::fs::read_to_string(&arguments_file)
            .unwrap()
            .lines()
            .collect::<Vec<_>>(),
        vec!["run", "--test-mode", "--payload", &payload]
    );
    assert!(!sentinel.exists(), "arguments must remain literal");
    let recorded = std::fs::read_to_string(&trace).unwrap();
    assert!(!recorded.contains("synthetic-operator-key"));
    assert!(!recorded.contains("account-test-only"));
    assert!(!recorded.contains(&payload));
    assert!(recorded.contains("UserKnownHostsFile="));
    service.start().await.unwrap();
    assert!(matches!(service.status().await.unwrap(), Status::Running));
    stop_workload(&pid_file);
    assert!(
        matches!(service.status().await.unwrap(), Status::Error),
        "dead workload must not be reported running"
    );
    service.shutdown().await.unwrap();
    assert!(retired.load(Ordering::SeqCst));
}
