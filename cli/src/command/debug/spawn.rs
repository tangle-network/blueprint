use crate::command::deploy::mbsm::deploy_mbsm_if_needed;
use crate::command::deploy::tangle::deploy_tangle;
use crate::command::register::register;
use blueprint_chain_setup::tangle::testnet::SubstrateNode;
use blueprint_clients::tangle::client::TangleClient;
use blueprint_crypto::sp_core::{SpEcdsa, SpSr25519};
use blueprint_crypto::tangle_pair_signer::TanglePairSigner;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_manager::config::{AuthProxyOpts, BlueprintManagerConfig};
use blueprint_manager::executor::run_auth_proxy;
use blueprint_manager::rt::hypervisor::ServiceVmConfig;
use blueprint_manager::rt::hypervisor::net::NetworkManager;
use blueprint_manager::rt::service::Service;
use blueprint_manager::sources::{BlueprintArgs, BlueprintEnvVars};
use blueprint_runner::config::{BlueprintEnvironment, Protocol};
use nix::sys::termios;
use nix::sys::termios::{InputFlags, LocalFlags, SetArg};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::{fs, thread};
use tracing::{error, info, warn};
use url::Url;

/// Spawns a Tangle testnet and virtual machine for the given blueprint
///
/// # Errors
///
/// * Unable to spawn/connect to the Tangle node
///
/// See also:
///
/// * [`deploy_mbsm_if_needed()`]
/// * [`NetworkManager::new()`]
/// * [`run_auth_proxy()`]
/// * [`register()`]
/// * [`Service::new()`]
/// * [`Service::start()`]
#[allow(clippy::too_many_arguments, clippy::missing_panics_doc)]
pub async fn execute(
    http_rpc_url: Option<Url>,
    ws_rpc_url: Option<Url>,
    manifest_path: PathBuf,
    package: Option<String>,
    id: u32,
    service_name: String,
    binary: PathBuf,
    protocol: Protocol,
    _verify_network_connection: bool,
) -> color_eyre::Result<()> {
    let mut manager_config = BlueprintManagerConfig::default();

    let tmp = tempfile::tempdir()?;
    manager_config.data_dir = tmp.path().join("data");
    manager_config.cache_dir = tmp.path().join("cache");
    manager_config.runtime_dir = tmp.path().join("runtime");
    manager_config.keystore_uri = tmp.path().join("keystore").to_string_lossy().into();

    manager_config.verify_directories_exist()?;
    let network_candidates = manager_config
        .default_address_pool
        .hosts()
        .filter(|ip| ip.octets()[3] != 0 && ip.octets()[3] != 255)
        .collect();
    let network_manager = NetworkManager::new(network_candidates).await.map_err(|e| {
        error!("Failed to create network manager: {e}");
        e
    })?;

    blueprint_testing_utils::tangle::keys::inject_tangle_key(
        &manager_config.keystore_uri,
        "//Alice",
    )?;

    let (_node, http, ws) = setup_tangle_node(tmp.path(), http_rpc_url, ws_rpc_url).await?;
    Box::pin(deploy_tangle(
        http.to_string(),
        ws.to_string(),
        package,
        false,
        Some(PathBuf::from(&manager_config.keystore_uri)),
        manifest_path,
    ))
    .await?;
    register(ws.to_string(), 0, manager_config.keystore_uri.clone(), "").await?;

    let (db, auth_proxy_task) =
        run_auth_proxy(manager_config.data_dir.clone(), AuthProxyOpts::default()).await?;

    let args = BlueprintArgs::new(&manager_config);
    let env = BlueprintEnvVars {
        http_rpc_endpoint: http,
        ws_rpc_endpoint: ws,
        keystore_uri: manager_config.keystore_uri.clone(),
        data_dir: manager_config.data_dir.clone(),
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
        db,
        manager_config.data_dir,
        manager_config.keystore_uri,
        manager_config.cache_dir,
        &manager_config.runtime_dir,
        &service_name,
        binary,
        env,
        args,
    )
    .await?;

    // TODO: Check is_alive
    let _is_alive = Box::pin(service.start().await?.unwrap());

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
        _ = auth_proxy_task => {
            warn!("Auth proxy shutdown");
        },
    }

    stdin_to_pty.join().unwrap();
    pty_to_stdout.join().unwrap();

    service.shutdown().await?;

    Ok(())
}

fn set_raw_mode(fd: &fs::File) -> io::Result<()> {
    let mut termios = termios::tcgetattr(fd)?;

    termios.input_flags &= !(InputFlags::ICRNL | InputFlags::IXON);
    termios.local_flags &= !(LocalFlags::ICANON | LocalFlags::ECHO | LocalFlags::ISIG);

    termios::tcsetattr(fd, SetArg::TCSANOW, &termios)?;

    Ok(())
}

async fn setup_tangle_node(
    tmp_path: &Path,
    mut http_rpc_url: Option<Url>,
    mut ws_rpc_url: Option<Url>,
) -> color_eyre::Result<(Option<SubstrateNode>, Url, Url)> {
    let mut node = None;
    if http_rpc_url.is_none() || ws_rpc_url.is_none() {
        info!("Spawning Tangle node");

        match blueprint_chain_setup::tangle::run(blueprint_chain_setup::tangle::NodeConfig::new(
            false,
        ))
        .await
        {
            Ok(tangle) => {
                let http = format!("http://127.0.0.1:{}", tangle.ws_port());
                let ws = format!("ws://127.0.0.1:{}", tangle.ws_port());

                info!("Tangle node running on {http} / {ws}");

                http_rpc_url = Some(Url::parse(&http)?);
                ws_rpc_url = Some(Url::parse(&ws)?);

                node = Some(tangle);
            }
            Err(e) => {
                error!("Failed to setup local Tangle node: {e}");
                return Err(e.into());
            }
        }
    }

    let http = http_rpc_url.unwrap();
    let ws = ws_rpc_url.unwrap();

    let alice_keystore_config = KeystoreConfig::new().fs_root(tmp_path.join("keystore"));
    let alice_keystore = Keystore::new(alice_keystore_config)?;

    let alice_sr25519_public = alice_keystore.first_local::<SpSr25519>()?;
    let alice_sr25519_pair = alice_keystore.get_secret::<SpSr25519>(&alice_sr25519_public)?;
    let alice_sr25519_signer = TanglePairSigner::new(alice_sr25519_pair.0);

    let alice_ecdsa_public = alice_keystore.first_local::<SpEcdsa>()?;
    let alice_ecdsa_pair = alice_keystore.get_secret::<SpEcdsa>(&alice_ecdsa_public)?;
    let alice_ecdsa_signer = TanglePairSigner::new(alice_ecdsa_pair.0);
    let alice_alloy_key = alice_ecdsa_signer
        .alloy_key()
        .map_err(|e| color_eyre::Report::msg(format!("Failed to get Alice's Alloy key: {}", e)))?;

    let mut env = BlueprintEnvironment::default();
    env.http_rpc_endpoint = http.clone();
    env.ws_rpc_endpoint = ws.clone();
    let alice_client = TangleClient::with_keystore(env, alice_keystore).await?;

    deploy_mbsm_if_needed(
        ws.clone(),
        &alice_client,
        &alice_sr25519_signer,
        alice_alloy_key.clone(),
    )
    .await?;

    Ok((node, http, ws))
}
