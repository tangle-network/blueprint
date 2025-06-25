use blueprint_core::{error, info};
use blueprint_manager::blueprint_auth::db::RocksDb;
use blueprint_manager::config::BlueprintManagerConfig;
use blueprint_manager::rt::hypervisor::net::NetworkManager;
use blueprint_manager::rt::hypervisor::{ServiceVmConfig, net};
use blueprint_manager::rt::service::Service;
use blueprint_manager::sources::{BlueprintArgs, BlueprintEnvVars};
use nix::sys::termios;
use nix::sys::termios::{InputFlags, LocalFlags, SetArg};
use std::path::PathBuf;
use std::{fs, io};
use tokio::io::AsyncWriteExt;
use tokio::task::JoinHandle;

async fn setup_with_vm(
    manager_config: &mut BlueprintManagerConfig,
    service_name: &str,
    id: u32,
    binary: PathBuf,
    db: RocksDb,
    env: BlueprintEnvVars,
    args: BlueprintArgs,
) -> color_eyre::Result<(Service, VmPtyIo)> {
    manager_config.verify_network_interface()?;

    let network_candidates = manager_config
        .default_address_pool
        .hosts()
        .filter(|ip| ip.octets()[3] != 0 && ip.octets()[3] != 255)
        .collect();
    let network_manager = NetworkManager::new(network_candidates).await.map_err(|e| {
        error!("Failed to create network manager: {e}");
        e
    })?;

    let service = Service::new(
        ServiceVmConfig {
            id,
            pty: true,
            ..Default::default()
        },
        network_manager,
        manager_config.network_interface.clone().unwrap(),
        db,
        &manager_config.data_dir,
        &manager_config.keystore_uri,
        &manager_config.cache_dir,
        &manager_config.runtime_dir,
        service_name,
        binary,
        env,
        args,
    )
    .await?;

    let pty = service
        .hypervisor()
        .expect("is hypervisor service")
        .pty()
        .await?
        .unwrap();
    info!("VM serial output to: {}", pty.display());

    let pty = fs::OpenOptions::new().read(true).write(true).open(pty)?;

    set_raw_mode(&pty)?;

    let pty_reader = tokio::fs::File::from_std(pty.try_clone()?);
    let pty_writer = tokio::fs::File::from_std(pty);

    let stdin_to_pty = tokio::spawn(async move {
        let mut stdin = tokio::io::stdin();
        let mut writer = pty_writer;
        tokio::io::copy(&mut stdin, &mut writer).await?;
        writer.flush().await?;
        Ok(())
    });

    let pty_to_stdout = tokio::spawn(async move {
        let mut reader = pty_reader;
        let mut stdout = tokio::io::stdout();
        tokio::io::copy(&mut reader, &mut stdout).await?;
        stdout.flush().await?;
        Ok(())
    });

    let io_handles = VmPtyIo {
        stdin_to_pty,
        pty_to_stdout,
    };

    Ok((service, io_handles))
}

fn set_raw_mode(fd: &fs::File) -> io::Result<()> {
    let mut termios = termios::tcgetattr(fd)?;

    termios.input_flags &= !(InputFlags::ICRNL | InputFlags::IXON);
    termios.local_flags &= !(LocalFlags::ICANON | LocalFlags::ECHO | LocalFlags::ISIG);

    termios::tcsetattr(fd, SetArg::TCSANOW, &termios)?;

    Ok(())
}

async fn vm_shutdown(network_interface: &str) -> Result<()> {
    if let Err(e) = net::nftables::cleanup_firewall(network_interface) {
        error!("Failed to cleanup nftables rules: {e}");
    }

    Ok(())
}
