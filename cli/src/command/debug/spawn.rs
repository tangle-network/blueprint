mod container;
mod native;
#[cfg(feature = "vm-debug")]
mod vm;

use crate::command::deploy::mbsm::deploy_mbsm_if_needed;
use crate::command::deploy::tangle::deploy_tangle;
use crate::command::register::register;
use blueprint_chain_setup::tangle::testnet::SubstrateNode;
use blueprint_clients::tangle::client::TangleClient;
use blueprint_core::{error, info};
use blueprint_crypto::sp_core::{SpEcdsa, SpSr25519};
use blueprint_crypto::tangle_pair_signer::TanglePairSigner;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_manager::config::{AuthProxyOpts, BlueprintManagerConfig, BlueprintManagerContext};
use blueprint_manager::executor::run_auth_proxy;
use blueprint_manager::rt::ResourceLimits;
use blueprint_manager::sources::{BlueprintArgs, BlueprintEnvVars};
use blueprint_runner::config::{BlueprintEnvironment, Protocol, SupportedChains};
use clap::ValueEnum;
use std::future;
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use tokio::task::{JoinError, JoinHandle};
use url::Url;

async fn setup_tangle_node(
    tmp_path: &Path,
    package: Option<String>,
    manifest_path: &Path,
    keystore_uri: String,
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

    Box::pin(deploy_tangle(
        http.to_string(),
        ws.to_string(),
        package,
        false,
        Some(PathBuf::from(keystore_uri.clone())),
        manifest_path.to_path_buf(),
    ))
    .await?;
    register(ws.to_string(), 0, keystore_uri, "").await?;

    Ok((node, http, ws))
}

pub struct PtyIo {
    pub stdin_to_pty: JoinHandle<io::Result<()>>,
    pub pty_to_stdout: JoinHandle<io::Result<()>>,
}

#[derive(ValueEnum, Debug, Copy, Clone)]
pub enum ServiceSpawnMethod {
    Native,
    #[cfg(feature = "vm-debug")]
    Vm,
    Container,
}

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
#[allow(
    clippy::too_many_arguments,
    clippy::missing_panics_doc,
    clippy::large_futures
)]
pub async fn execute(
    http_rpc_url: Option<Url>,
    ws_rpc_url: Option<Url>,
    manifest_path: PathBuf,
    package: Option<String>,
    #[allow(unused_variables)] id: u32,
    service_name: String,
    binary: Option<PathBuf>,
    image: Option<String>,
    protocol: Protocol,
    method: ServiceSpawnMethod,
    #[cfg(feature = "vm-debug")] _verify_network_connection: bool,
) -> color_eyre::Result<()> {
    let mut manager_config = BlueprintManagerConfig::default();

    let tmp = tempfile::tempdir()?;
    manager_config.paths.data_dir = tmp.path().join("data");
    manager_config.paths.cache_dir = tmp.path().join("cache");
    manager_config.paths.runtime_dir = tmp.path().join("runtime");
    manager_config.paths.keystore_uri = tmp.path().join("keystore").to_string_lossy().into();

    let ctx = BlueprintManagerContext::new(manager_config).await?;

    blueprint_testing_utils::tangle::keys::inject_tangle_key(ctx.keystore_uri(), "//Alice")?;

    let (_node, http, ws) = setup_tangle_node(
        tmp.path(),
        package,
        &manifest_path,
        ctx.keystore_uri().to_string(),
        http_rpc_url,
        ws_rpc_url,
    )
    .await?;

    let (db, auth_proxy_task) =
        run_auth_proxy(ctx.data_dir().to_path_buf(), AuthProxyOpts::default()).await?;
    ctx.set_db(db).await;

    let args = BlueprintArgs::new(&ctx);
    let env = BlueprintEnvVars {
        http_rpc_endpoint: http,
        ws_rpc_endpoint: ws,
        // TODO
        kms_endpoint: "https://127.0.0.1:19821".parse().unwrap(),
        keystore_uri: ctx.keystore_uri().to_string(),
        data_dir: ctx.data_dir().to_path_buf(),
        blueprint_id: 0,
        service_id: 0,
        protocol,
        chain: Some(SupportedChains::LocalTestnet),
        bootnodes: String::new(),
        registration_mode: false,
        // Set later
        bridge_socket_path: None,
    };

    // TODO: Allow setting resource limits on the CLI?
    let limits = ResourceLimits::default();

    let (mut service, pty_io) = match method {
        ServiceSpawnMethod::Native => {
            let service =
                native::setup_native(&ctx, limits, &service_name, binary.unwrap(), env, args)
                    .await?;
            (service, None)
        }
        #[cfg(feature = "vm-debug")]
        ServiceSpawnMethod::Vm => {
            let (service, pty) =
                vm::setup_with_vm(&ctx, limits, &service_name, id, binary.unwrap(), env, args)
                    .await?;

            (service, Some(pty))
        }
        ServiceSpawnMethod::Container => {
            let service = container::setup_with_container(
                &ctx,
                limits,
                &service_name,
                image.unwrap(),
                env,
                args,
            )
            .await?;
            (service, None)
        }
    };

    // TODO: Check is_alive
    let _is_alive = Box::pin(service.start().await?.unwrap());

    let stdin_task: Pin<Box<dyn Future<Output = Result<io::Result<()>, JoinError>>>>;
    let stdout_task: Pin<Box<dyn Future<Output = Result<io::Result<()>, JoinError>>>>;
    if let Some(PtyIo {
        stdin_to_pty,
        pty_to_stdout,
    }) = pty_io
    {
        stdin_task = Box::pin(stdin_to_pty);
        stdout_task = Box::pin(pty_to_stdout);
    } else {
        stdin_task = Box::pin(future::pending());
        stdout_task = Box::pin(future::pending());
    }

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = auth_proxy_task => {
            error!("Auth proxy shutdown");
        },
        res = stdin_task => {
            error!("stdin task died: {res:?}");
        },
        res = stdout_task => {
            error!("stdout task died: {res:?}");
        },
    }

    let shutdown_res = service.shutdown().await;

    #[cfg(feature = "vm-debug")]
    if method == ServiceSpawnMethod::Vm {
        vm::vm_shutdown(&ctx.vm.network_interface).await?;
    }

    shutdown_res.map_err(Into::into)
}
