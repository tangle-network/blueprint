use crate::config::BlueprintManagerConfig;
use crate::rt::hypervisor::ServiceVmConfig;
use crate::rt::hypervisor::net::NetworkManager;
use crate::rt::service::Service;
use crate::sources::{BlueprintArgs, BlueprintEnvVars};
use blueprint_runner::config::Protocol;
use clap::Subcommand;
use nix::sys::termios;
use nix::sys::termios::{InputFlags, LocalFlags, SetArg};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::{fs, thread};
use tracing::{error, info, warn};
use url::Url;

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
        #[arg(long, default_value_t = true)]
        verify_network_connection: bool,
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
                verify_network_connection,
            } => {
                config.verify_directories_exist()?;
                blueprint_tangle_testing_utils::keys::inject_tangle_key(
                    &config.keystore_uri,
                    "//Alice",
                )
                .map_err(|e| crate::error::Error::Other(e.to_string()))?;

                info!("Spawning Tangle node");
                let node = match blueprint_chain_setup::tangle::run(
                    blueprint_chain_setup::tangle::NodeConfig::new(false),
                )
                .await
                {
                    Ok(node) => node,
                    Err(e) => {
                        error!("Failed to setup local Tangle node: {e}");
                        return Err(crate::error::Error::Other(e.to_string()));
                    }
                };

                let http = format!("http://127.0.0.1:{}", node.ws_port());
                let ws = format!("ws://127.0.0.1:{}", node.ws_port());

                info!("Tangle node running on {http} / {ws}");

                let http_rpc_endpoint = Url::parse(&http).unwrap();
                let ws_rpc_endpoint = Url::parse(&ws).unwrap();

                let args = BlueprintArgs::new(&config);
                let env = BlueprintEnvVars {
                    http_rpc_endpoint,
                    ws_rpc_endpoint,
                    keystore_uri: config.keystore_uri.clone(),
                    data_dir: config.data_dir.clone(),
                    blueprint_id: 0,
                    service_id: 0,
                    protocol,
                    bootnodes: String::new(),
                    registration_mode: false,
                };

                let mut service = Service::new(
                    ServiceVmConfig {
                        id,
                        pty: true,
                        ..Default::default()
                    },
                    network_manager,
                    config.data_dir,
                    config.keystore_uri,
                    config.cache_dir,
                    &config.runtime_dir,
                    &service_name,
                    binary,
                    env,
                    args,
                )
                .await?;
                let mut is_alive = Box::pin(service.start().await?.unwrap());

                let pty = service.hypervisor().pty().await?.unwrap();
                info!("VM serial output to: {}", pty.display());

                let pty = fs::OpenOptions::new().read(true).write(true).open(pty)?;

                set_raw_mode(&pty)?;

                let mut pty_reader = pty.try_clone()?;
                let mut pty_writer = pty;

                let stdin_to_pty = thread::spawn(move || {
                    let mut stdin = std::io::stdin();
                    let mut buffer = [0u8; 1024];
                    loop {
                        match stdin.read(&mut buffer) {
                            Ok(0) | Err(_) => break,
                            Ok(n) => {
                                if pty_writer.write_all(&buffer[..n]).is_err() {
                                    break;
                                }
                            }
                        }
                    }
                });

                let pty_to_stdout = thread::spawn(move || {
                    let mut stdout = std::io::stdout();
                    let mut buffer = [0u8; 1024];
                    loop {
                        match pty_reader.read(&mut buffer) {
                            Ok(0) | Err(_) => break,
                            Ok(n) => {
                                if stdout.write_all(&buffer[..n]).is_err() {
                                    break;
                                }
                                stdout.flush().ok();
                            }
                        }
                    }
                });

                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {}
                    _ = &mut is_alive => {
                        warn!("Networking will **NOT** work for this VM");
                        if verify_network_connection {
                            service.shutdown().await?;
                            return Err(crate::error::Error::Other(String::from("Bridge failed to connect")));
                        }
                    }
                }

                stdin_to_pty.join().unwrap();
                pty_to_stdout.join().unwrap();

                service.shutdown().await?;
            }
        }

        Ok(())
    }
}

fn set_raw_mode(fd: &fs::File) -> io::Result<()> {
    let mut termios = termios::tcgetattr(fd)?;

    termios.input_flags &= !(InputFlags::ICRNL | InputFlags::IXON);
    termios.local_flags &= !(LocalFlags::ICANON | LocalFlags::ECHO | LocalFlags::ISIG);

    termios::tcsetattr(fd, SetArg::TCSANOW, &termios)?;

    Ok(())
}
