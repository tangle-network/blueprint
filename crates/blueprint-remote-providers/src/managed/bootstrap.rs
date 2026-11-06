//! Cloud-init payload shared by providers that run a container on a GPU VM.
use super::ProviderErrorCode;
use crate::{Error, Result};
use std::collections::BTreeMap;

/// Generate native cloud-init; the VM image must provide Python, Docker and its GPU runtime.
pub fn gpu_container_cloud_init(
    image: &str,
    env: &BTreeMap<String, String>,
    ports: &[u16],
    mounts: &[(String, String)],
    command: Option<&[String]>,
) -> Result<String> {
    let invalid = || Error::Provider(ProviderErrorCode::InvalidRequest);
    if image.is_empty()
        || image.starts_with('-')
        || image.chars().any(|c| c.is_whitespace() || c.is_control())
        || ports.contains(&0)
        || mounts.iter().any(|(source, target)| {
            [source, target].iter().any(|path| {
                !path.starts_with('/')
                    || path.chars().any(|c| c.is_control() || c == ',' || c == '"')
            })
        })
        || command.is_some_and(|args| args.iter().any(|arg| arg.contains('\0')))
    {
        return Err(invalid());
    }
    let mut environment = String::new();
    for (key, value) in env {
        if key.is_empty()
            || !key.bytes().enumerate().all(|(index, byte)| {
                byte == b'_' || byte.is_ascii_alphabetic() || (index > 0 && byte.is_ascii_digit())
            })
            || value.contains(['\0', '\n', '\r'])
        {
            return Err(invalid());
        }
        environment.push_str(key);
        environment.push('=');
        environment.push_str(value);
        environment.push('\n');
    }
    let mut ports = ports.to_vec();
    ports.sort_unstable();
    ports.dedup();
    let configuration = serde_json::json!({
        "image":image,"ports":ports,"mounts":mounts,"command":command,"env_file":"/etc/tangle-gpu/workload.env"
    });
    let launch = r#"import json, subprocess, sys
with open(sys.argv[1], encoding='utf-8') as handle:
    workload = json.load(handle)
arguments = ['docker', 'run', '--detach', '--restart=unless-stopped', '--gpus=all', '--name=tangle-gpu-workload', '--env-file', workload['env_file']]
for port in workload['ports']:
    arguments.extend(['--publish', str(port) + ':' + str(port)])
for source, target in workload['mounts']:
    arguments.extend(['--mount', 'type=bind,src=' + source + ',dst=' + target])
arguments.extend(['--', workload['image']])
if workload.get('command'):
    arguments.extend(workload['command'])
subprocess.run(['systemctl', 'enable', '--now', 'docker'], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
subprocess.run(arguments, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
"#;
    let document = serde_json::json!({"write_files":[
        {"path":"/etc/tangle-gpu/workload.env","owner":"root:root","permissions":"0600","content":environment},
        {"path":"/etc/tangle-gpu/workload.json","owner":"root:root","permissions":"0600","content":configuration.to_string()},
        {"path":"/usr/local/lib/tangle-gpu-start.py","owner":"root:root","permissions":"0700","content":launch}
    ],"runcmd":[["python3","/usr/local/lib/tangle-gpu-start.py","/etc/tangle-gpu/workload.json"]]});
    serde_yaml::to_string(&document)
        .map(|yaml| format!("#cloud-config\n{yaml}"))
        .map_err(|_| invalid())
}

/// Forward workload configuration only when its runtime delivery is representable.
pub fn workload_environment(spec: &crate::ResourceSpec) -> Result<BTreeMap<String, String>> {
    if spec.manifest.is_some() {
        return Err(Error::Provider(ProviderErrorCode::Unsupported));
    }
    Ok(spec.env.clone())
}
