//! Deploy native blueprints through the SDK's priced, expiring GPU leases.

use crate::{
    config::BlueprintManagerContext,
    error::{Error, Result},
    rt::{ResourceLimits, remote::RemoteServiceInstance, service::Service},
    sources::{BlueprintArgs, BlueprintEnvVars},
};
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, Lease, ResourceSpec,
    deployment::secure_ssh::{SecureSshClient, SecureSshConnection},
};
use std::{path::Path, sync::Arc, time::Duration};

#[derive(Debug, Clone)]
pub struct RemoteDeploymentPolicy {
    pub max_hourly_cost: Option<f32>,
    pub prefer_spot: bool,
    pub auto_terminate_hours: Option<u32>,
    pub tee_required: bool,
    pub tee_backend: Option<String>,
}

impl Default for RemoteDeploymentPolicy {
    fn default() -> Self {
        Self {
            max_hourly_cost: None,
            prefer_spot: false,
            auto_terminate_hours: Some(24),
            tee_required: std::env::var("BLUEPRINT_REMOTE_TEE_REQUIRED").is_ok_and(|value| {
                matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes"
                )
            }),
            tee_backend: std::env::var("TEE_BACKEND").ok(),
        }
    }
}

pub struct RemoteDeploymentService {
    policy: RemoteDeploymentPolicy,
}

impl RemoteDeploymentService {
    pub async fn new(policy: RemoteDeploymentPolicy) -> Result<Self> {
        if policy.auto_terminate_hours.is_none_or(|hours| hours == 0) {
            return Err(Error::Other(
                "Remote deployments require a positive lease lifetime".into(),
            ));
        }
        Ok(Self { policy })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn deploy_service(
        &self,
        ctx: &BlueprintManagerContext,
        service_name: &str,
        binary_path: &Path,
        mut env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
        limits: ResourceLimits,
        _blueprint_id: Option<u64>,
    ) -> Result<Service> {
        let provider = ctx
            .remote_deployment_opts
            .preferred_provider
            .as_deref()
            .map(|name| {
                CloudProvider::from_catalog_key(name)
                    .ok_or_else(|| Error::Other("Unknown preferred cloud provider".into()))
            })
            .transpose()?;
        let spec = ResourceSpec {
            provider,
            cpu: limits.cpu_count.map_or(2.0, f32::from),
            memory_gb: limits.memory_size as f32 / (1024.0 * 1024.0 * 1024.0),
            storage_gb: limits.storage_space as f32 / (1024.0 * 1024.0 * 1024.0),
            gpu_class: limits.gpu_class,
            gpu_count: limits.gpu_count.map(u32::from),
            gpu_vram_gb: limits.gpu_min_vram_gb,
            tee_required: self.policy.tee_required,
            allow_spot: self.policy.prefer_spot,
            max_hourly_cost_usd: self.policy.max_hourly_cost.map(f64::from),
            ..Default::default()
        };
        spec.validate().map_err(Error::Other)?;
        if !binary_path.is_file() {
            return Err(Error::Other("Blueprint binary does not exist".into()));
        }
        let keystore = Path::new(&env_vars.keystore_uri)
            .canonicalize()
            .map_err(|_| Error::Other("Configured operator keystore is unavailable".into()))?;
        if !keystore.is_dir() {
            return Err(Error::Other("Operator keystore must be a directory".into()));
        }
        let ttl = Duration::from_secs(u64::from(self.policy.auto_terminate_hours.unwrap()) * 3600);
        let provisioner = match ctx.cloud_provisioner() {
            Some(provisioner) => provisioner.clone(),
            None => Arc::new(
                CloudProvisioner::new()
                    .await?
                    .with_lease_store(ctx.data_dir().join("gpu-leases")),
            ),
        };
        let lease = provisioner.allocate(&spec, ttl).await?;
        env_vars.data_dir = Path::new("/tmp").join(format!("blueprint-{}", lease.id));
        env_vars.keystore_uri = env_vars
            .data_dir
            .join("keystore")
            .to_string_lossy()
            .into_owned();
        env_vars.bridge_socket_path = None;
        let environment = env_vars.encode();
        let (ssh, pid, access) = match deploy_binary(
            &lease,
            binary_path,
            &keystore,
            &environment,
            &arguments.encode(true),
        )
        .await
        {
            Ok(process) => process,
            Err(error) => {
                lease.terminate().await.map_err(|cleanup| {
                    Error::Other(format!("{error}; lease cleanup failed: {cleanup}"))
                })?;
                return Err(error);
            }
        };
        let remote = RemoteServiceInstance::from_lease(lease, spec).with_process(ssh, pid, access);
        Service::new_remote(
            ctx,
            ctx.data_dir().join("runtime").join(service_name),
            service_name,
            remote,
        )
        .await
    }
}

async fn deploy_binary(
    lease: &Lease,
    binary: &Path,
    keystore: &Path,
    environment: &[(String, String)],
    arguments: &[String],
) -> Result<(SecureSshClient, u32, tempfile::TempDir)> {
    let endpoint = url::Url::parse(&lease.endpoint)
        .map_err(|_| Error::Other("Invalid lease SSH endpoint".into()))?;
    if endpoint.scheme() != "ssh" {
        return Err(Error::Other("Lease has no SSH endpoint".into()));
    }
    let private_key = lease
        .access
        .private_key()
        .ok_or_else(|| Error::Other("Lease did not transfer a private access key".into()))?;
    let temporary = tempfile::tempdir()?;
    let key_file = tempfile::NamedTempFile::new_in(temporary.path())?;
    tokio::fs::write(key_file.path(), private_key).await?;
    let (_file, key_path) = key_file.keep().map_err(|error| error.error)?;
    let known_hosts = temporary.path().join("known_hosts");
    tokio::fs::write(&known_hosts, "").await?;
    let connection = SecureSshConnection::new(
        endpoint
            .host_str()
            .ok_or_else(|| Error::Other("Lease SSH host missing".into()))?
            .into(),
        if endpoint.username().is_empty() {
            "root"
        } else {
            endpoint.username()
        }
        .into(),
    )?
    .with_port(endpoint.port().unwrap_or(22))?
    .with_key_path(&key_path)?
    .with_known_hosts(&known_hosts)?;
    let client = SecureSshClient::new(connection);
    let directory = format!("/tmp/blueprint-{}", lease.id);
    client
        .run_script(&format!(
            "set -eu\numask 077\nmkdir -p {}\n",
            shell_quote(&directory)
        ))
        .await?;
    let remote_binary = format!("{directory}/workload");
    client.copy_files(binary, &remote_binary).await?;
    let mut script = zeroize::Zeroizing::new(format!(
        "set -eu\numask 077\ncd {}\nchmod 700 workload\nmkdir -p keystore\n",
        shell_quote(&directory)
    ));
    use base64::Engine;
    for entry in walkdir::WalkDir::new(keystore) {
        let entry = entry.map_err(|_| Error::Other("Unable to read operator keystore".into()))?;
        if entry.file_type().is_symlink() {
            return Err(Error::Other(
                "Operator keystore cannot contain symlinks".into(),
            ));
        }
        let relative = entry
            .path()
            .strip_prefix(keystore)
            .map_err(|_| Error::Other("Invalid keystore path".into()))?;
        let remote_path = format!("{directory}/keystore/{}", relative.display());
        if entry.file_type().is_dir() {
            script.push_str(&format!(
                "mkdir -p {}\nchmod 700 {}\n",
                shell_quote(&remote_path),
                shell_quote(&remote_path)
            ));
        } else if entry.file_type().is_file() {
            let bytes = zeroize::Zeroizing::new(tokio::fs::read(entry.path()).await?);
            let encoded =
                zeroize::Zeroizing::new(base64::engine::general_purpose::STANDARD.encode(&bytes));
            script.push_str(&format!(
                "printf '%s' {} | base64 -d > {}\nchmod 600 {}\n",
                shell_quote(&encoded),
                shell_quote(&remote_path),
                shell_quote(&remote_path)
            ));
        } else {
            return Err(Error::Other("Invalid operator keystore entry".into()));
        }
    }
    for (key, value) in environment {
        if key.is_empty()
            || !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            || value.contains('\0')
        {
            return Err(Error::Other("Invalid workload environment".into()));
        }
        script.push_str(&format!("export {key}={}\n", shell_quote(value)));
    }
    if arguments.iter().any(|arg| arg.contains('\0')) {
        return Err(Error::Other("Invalid workload argument".into()));
    }
    let args = arguments
        .iter()
        .map(|arg| shell_quote(arg))
        .collect::<Vec<_>>()
        .join(" ");
    script.push_str(&format!("nohup ./workload {args} >workload.log 2>&1 </dev/null &\npid=$!\nsleep 1\nkill -0 \"$pid\"\nprintf '%s\\n' \"$pid\"\n"));
    let pid = client
        .run_script(&script)
        .await?
        .trim()
        .parse::<u32>()
        .ok()
        .filter(|pid| *pid > 1)
        .ok_or_else(|| Error::Other("Remote workload did not report a running process".into()))?;
    Ok((client, pid, temporary))
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
