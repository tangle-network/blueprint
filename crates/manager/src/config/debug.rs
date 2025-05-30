use crate::config::BlueprintManagerConfig;
use crate::rt::hypervisor::net::NetworkManager;
use crate::rt::service::Service;
use crate::sources::{BlueprintArgs, BlueprintEnvVars};
use blueprint_runner::config::Protocol;
use clap::Subcommand;
use nix::fcntl::OFlag;
use nix::pty::{grantpt, posix_openpt, ptsname_r, unlockpt};
use std::fs;
use std::os::fd::OwnedFd;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tracing::error;

#[derive(Subcommand, Debug, Clone)]
pub enum DebugCommand {
    SpawnService {
        #[arg(default_value_t = 0)]
        id: u32,
        #[arg(default_value = "service")]
        service_name: String,
        #[arg(long, required = true)]
        binary: PathBuf,
        #[arg(long, default_value_t = Protocol::Tangle)]
        protocol: Protocol,
    },
}

impl DebugCommand {
    pub async fn execute(
        self,
        mut config: BlueprintManagerConfig,
    ) -> Result<(), crate::error::Error> {
        let network_candidates = config
            .default_address_pool
            .hosts()
            .filter(|ip| ip.octets()[3] != 0 && ip.octets()[3] != 255)
            .collect();
        let network_manager = NetworkManager::new(network_candidates).await.map_err(|e| {
            error!("Failed to create network manager: {e}");
            e
        })?;

        let tmp = tempfile::tempdir()?;
        config.data_dir = tmp.path().join("data");
        config.cache_dir = tmp.path().join("cache");
        config.runtime_dir = tmp.path().join("runtime");
        config.keystore_uri = tmp.path().join("keystore").to_string_lossy().into();

        match self {
            DebugCommand::SpawnService {
                id,
                service_name,
                binary,
                protocol,
            } => {
                config.verify_directories_exist()?;

                let master = posix_openpt(OFlag::O_RDWR | OFlag::O_NOCTTY)?;
                grantpt(&master)?;
                unlockpt(&master)?;

                let slave_path = ptsname_r(&master)?;

                let owned: OwnedFd = master.into();
                let mut master_file = File::from_std(fs::File::from(owned));
                let mut master_file_clone = master_file.try_clone().await?;

                let pump_out = tokio::task::spawn(async move {
                    tokio::io::copy(&mut master_file, &mut tokio::io::stdout()).await
                });

                let pump_in = tokio::task::spawn(async move {
                    tokio::io::copy(&mut tokio::io::stdin(), &mut master_file_clone).await
                });

                let args = BlueprintArgs::new(&config);
                let env = BlueprintEnvVars {
                    http_rpc_endpoint: "".to_string(),
                    ws_rpc_endpoint: "".to_string(),
                    keystore_uri: config.keystore_uri.clone(),
                    data_dir: config.data_dir.clone(),
                    blueprint_id: 0,
                    service_id: 0,
                    protocol,
                    bootnodes: "".to_string(),
                    registration_mode: false,
                };

                let mut service = Service::new(
                    id,
                    network_manager,
                    config.data_dir,
                    config.keystore_uri,
                    config.cache_dir,
                    &config.runtime_dir,
                    Some(Path::new(&slave_path)),
                    &service_name,
                    binary,
                    env,
                    args,
                )
                .await?;
                let mut is_alive = Box::pin(service.start().await?.unwrap());

                loop {
                    tokio::select! {
                        _ = tokio::signal::ctrl_c() => {
                            break;
                        }
                        // _ = &mut is_alive => {
                        //     warn!("Networking will **NOT** work for this VM");
                        // }
                    }
                }

                pump_out.abort();

                service.shutdown().await?;
            }
        }

        Ok(())
    }
}
