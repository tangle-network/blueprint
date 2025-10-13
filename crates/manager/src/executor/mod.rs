use crate::blueprint::ActiveBlueprints;
use crate::config::{AuthProxyOpts, BlueprintManagerContext};
use crate::error::Error;
use crate::error::Result;
#[cfg(feature = "vm-sandbox")]
use crate::rt::hypervisor::net;
use crate::sdk::entry::SendFuture;
use blueprint_auth::db::RocksDb;
use blueprint_clients::EventsClient;
#[cfg(feature = "tangle")]
use blueprint_clients::tangle::client::TangleClient;
#[cfg(feature = "eigenlayer")]
use blueprint_clients::eigenlayer::client::EigenlayerClient;
use blueprint_core::{error, info, warn};
use blueprint_crypto::sp_core::{SpEcdsa, SpSr25519};
use blueprint_crypto::tangle_pair_signer::TanglePairSigner;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_runner::config::BlueprintEnvironment;
use color_eyre::Report;
use color_eyre::eyre::OptionExt;
use sp_core::{ecdsa, sr25519};
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use tangle_subxt::subxt::tx::Signer;
use tangle_subxt::subxt::utils::AccountId32;
use tokio::task::JoinHandle;

#[cfg(feature = "tangle")]
pub(crate) mod tangle_event_handler;
#[cfg(feature = "eigenlayer")]
pub(crate) mod eigen_event_handler;

pub struct BlueprintManagerHandle {
    shutdown_call: Option<tokio::sync::oneshot::Sender<()>>,
    start_tx: Option<tokio::sync::oneshot::Sender<()>>,
    running_task: JoinHandle<color_eyre::Result<()>>,
    span: tracing::Span,
    #[cfg(feature = "tangle")]
    sr25519_id: TanglePairSigner<sr25519::Pair>,
    ecdsa_id: TanglePairSigner<ecdsa::Pair>,
    keystore_uri: String,
}

impl BlueprintManagerHandle {
    /// Send a start signal to the blueprint manager
    ///
    /// # Errors
    ///
    /// * If the start signal fails to send
    /// * If the start signal has already been sent
    pub fn start(&mut self) -> color_eyre::Result<()> {
        let _span = self.span.enter();
        match self.start_tx.take() {
            Some(tx) => match tx.send(()) {
                Ok(()) => {
                    info!("Start signal sent to Blueprint Manager");
                    Ok(())
                }
                Err(()) => Err(Report::msg(
                    "Failed to send start signal to Blueprint Manager",
                )),
            },
            None => Err(Report::msg("Blueprint Manager Already Started")),
        }
    }

    /// Returns the SR25519 keypair for this blueprint manager
    #[cfg(feature = "tangle")]
    #[must_use]
    pub fn sr25519_id(&self) -> &TanglePairSigner<sr25519::Pair> {
        &self.sr25519_id
    }

    /// Returns the ECDSA keypair for this blueprint manager
    #[must_use]
    pub fn ecdsa_id(&self) -> &TanglePairSigner<ecdsa::Pair> {
        &self.ecdsa_id
    }

    /// Shutdown the blueprint manager
    ///
    /// # Errors
    ///
    /// * If the shutdown signal fails to send
    /// * If the shutdown signal has already been sent
    pub fn shutdown(&mut self) -> color_eyre::Result<()> {
        self.shutdown_call
            .take()
            .map(|tx| tx.send(()))
            .ok_or_eyre("Shutdown already called")?
            .map_err(|()| Report::msg("Failed to send shutdown signal to Blueprint Manager"))
    }

    /// Returns the keystore URI for this blueprint manager
    #[must_use]
    pub fn keystore_uri(&self) -> &str {
        &self.keystore_uri
    }

    #[must_use]
    pub fn span(&self) -> &tracing::Span {
        &self.span
    }
}

/// Add default behavior for unintentional dropping of the `BlueprintManagerHandle`
/// This will ensure that the `BlueprintManagerHandle` is executed even if the handle
/// is dropped, which is similar behavior to the tokio `SpawnHandle`
impl Drop for BlueprintManagerHandle {
    fn drop(&mut self) {
        let _ = self.start();
    }
}

/// Implement the Future trait for the `BlueprintManagerHandle` to allow
/// for the handle to be awaited on
impl Future for BlueprintManagerHandle {
    type Output = color_eyre::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Start the blueprint manager if it has not been started
        let this = self.get_mut();
        if this.start_tx.is_some() {
            if let Err(err) = this.start() {
                return Poll::Ready(Err(err));
            }
        }

        let result = futures::ready!(Pin::new(&mut this.running_task).poll(cx));

        match result {
            Ok(res) => Poll::Ready(res),
            Err(err) => Poll::Ready(Err(Report::msg(format!(
                "Blueprint Manager Closed Unexpectedly (JoinError): {err:?}"
            )))),
        }
    }
}

/// Run the blueprint manager with the given configuration
///
/// # Arguments
///
/// * `blueprint_manager_config` - The configuration for the blueprint manager
/// * `keystore` - The keystore to use for the blueprint manager
/// * `env` - The environment configuration for the blueprint
/// * `shutdown_cmd` - The shutdown command for the blueprint manager
///
/// # Returns
///
/// * A handle to the running blueprint manager
///
/// # Errors
///
/// * If the blueprint manager fails to start
///
/// # Panics
///
/// * If the SR25519 or ECDSA keypair cannot be found
#[allow(clippy::used_underscore_binding)]
pub async fn run_blueprint_manager_with_keystore<F: SendFuture<'static, ()>>(
    #[allow(unused_mut)] mut ctx: BlueprintManagerContext,
    keystore: Keystore,
    env: BlueprintEnvironment,
    shutdown_cmd: F,
) -> color_eyre::Result<BlueprintManagerHandle> {
    let logger_id = if let Some(custom_id) = &ctx.instance_id {
        custom_id.as_str()
    } else {
        "Local"
    };

    let span = tracing::info_span!("Blueprint-Manager", id = logger_id);

    let _span = span.enter();
    info!("Starting blueprint manager ... waiting for start signal ...");

    // Create the auth proxy task
    let (db, auth_proxy_task) =
        run_auth_proxy(ctx.data_dir().to_path_buf(), ctx.auth_proxy_opts.clone()).await?;
    ctx.set_db(db).await;

    let ecdsa_key_pub = keystore.first_local::<SpEcdsa>()?;
    let ecdsa_pair = keystore.get_secret::<SpEcdsa>(&ecdsa_key_pub)?;
    let ecdsa_key = TanglePairSigner::new(ecdsa_pair.0);

    #[cfg(feature = "tangle")]
    let tangle_key = {
        let sr_key_pub = keystore.first_local::<SpSr25519>()?;
        let sr_pair = keystore.get_secret::<SpSr25519>(&sr_key_pub)?;
        TanglePairSigner::new(sr_pair.0)
    };

    #[cfg(feature = "tangle")]
    let sub_account_id = tangle_key.account_id().clone();
    #[cfg(feature = "tangle")]
    let mut active_blueprints = HashMap::new();

    let keystore_uri = env.keystore_uri.clone();
    #[cfg(feature = "vm-sandbox")]
    let network_interface = ctx.vm.network_interface.clone();

    #[cfg(feature = "tangle")]
    let manager_tangle_task = async move {
        let tangle_client = TangleClient::with_keystore(env.clone(), keystore).await?;
        let services_client = tangle_client.services_client();

        // With the basics setup, we must now implement the main logic of the Blueprint Manager
        // Handle initialization logic
        // NOTE: The node running this code should be registered as an operator for the blueprints, otherwise, this
        // code will fail
        let mut operator_subscribed_blueprints = tangle_event_handler::handle_init(
            &tangle_client,
            services_client,
            &sub_account_id,
            &mut active_blueprints,
            &env,
            &ctx,
        )
        .await?;

        // Now, run the main event loop
        // Listen to FinalityNotifications and poll for new/deleted services that correspond to the blueprints above
        while let Some(event) = tangle_client.next_event().await {
            let result = tangle_event_handler::check_blueprint_events(
                &event,
                &mut active_blueprints,
                &sub_account_id.clone(),
            );

            if result.needs_update {
                operator_subscribed_blueprints = services_client
                    .query_operator_blueprints(event.hash, sub_account_id.clone())
                    .await?;
            }

            tangle_event_handler::handle_tangle_event(
                &event,
                &operator_subscribed_blueprints,
                &env,
                &ctx,
                &mut active_blueprints,
                result,
                services_client,
            )
            .await?;
        }

        Err::<(), _>(Error::ClientDied)
    };

    #[cfg(feature = "eigenlayer")]
    let manager_eigenlayer_task = async move {
        let eigenlayer_client = EigenlayerClient::new(env.clone());
        eigen_event_handler::handle_init(
            &eigenlayer_client,
            &env,
            &ctx,
        )
        .await?;

        while let Some(event) = eigenlayer_client.next_event().await {
            eigen_event_handler::handle_eigen_event(
                &event,
                &env,
                &ctx,
            )
            .await?;
        }

        Err::<(), _>(Error::ClientDied)
    };

    let (tx_stop, rx_stop) = tokio::sync::oneshot::channel::<()>();

    let shutdown_task = async move {
        tokio::select! {
            _res0 = shutdown_cmd => {
                info!("Shutdown-1 command received, closing application");
            },

            _res1 = rx_stop => {
                info!("Manual shutdown signal received, closing application");
            }
        }
    };

    let (start_tx, start_rx) = tokio::sync::oneshot::channel::<()>();
    let combined_task = async move {
        start_rx
            .await
            .map_err(|_err| Report::msg("Failed to receive start signal"))?;
    
        #[cfg(feature = "tangle")]
        tokio::select! {
            res0 = manager_tangle_task => {
                Err(Report::msg(format!("Blueprint Manager Closed Unexpectedly: {res0:?}")))
            },
            res1 = auth_proxy_task => {
                Err(Report::msg(format!("Auth Proxy Closed Unexpectedly: {res1:?}")))
            },
            () = shutdown_task => {
                #[cfg(feature = "vm-sandbox")]
                if let Err(e) = net::nftables::cleanup_firewall(&network_interface) {
                    error!("Failed to cleanup nftables rules: {e}");
                }
                Ok(())
            }
        }
    
        #[cfg(feature = "eigenlayer")]
        tokio::select! {
            res0 = manager_eigenlayer_task => {
                Err(Report::msg(format!("Blueprint Manager Closed Unexpectedly: {res0:?}")))
            },
            res1 = auth_proxy_task => {
                Err(Report::msg(format!("Auth Proxy Closed Unexpectedly: {res1:?}")))
            },
            () = shutdown_task => {
                #[cfg(feature = "vm-sandbox")]
                if let Err(e) = net::nftables::cleanup_firewall(&network_interface) {
                    error!("Failed to cleanup nftables rules: {e}");
                }
                Ok(())
            }
        }

        #[cfg(all(not(feature = "tangle"), not(feature = "eigenlayer")))]
        tokio::select! {
            res1 = auth_proxy_task => {
                Err(Report::msg(format!("Auth Proxy Closed Unexpectedly: {res1:?}")))
            },
            () = shutdown_task => {
                #[cfg(feature = "vm-sandbox")]
                if let Err(e) = net::nftables::cleanup_firewall(&network_interface) {
                    error!("Failed to cleanup nftables rules: {e}");
                }
                Ok(())
            }
        }
    };

    drop(_span);
    let handle = tokio::spawn(combined_task);

    let handle = BlueprintManagerHandle {
        start_tx: Some(start_tx),
        shutdown_call: Some(tx_stop),
        running_task: handle,
        span,
        #[cfg(feature = "tangle")]
        sr25519_id: tangle_key,
        ecdsa_id: ecdsa_key,
        keystore_uri,
    };

    Ok(handle)
}

/// Run the blueprint manager with the given configuration
///
/// # Arguments
///
/// * `blueprint_manager_config` - The configuration for the blueprint manager
/// * `env` - The environment configuration for the blueprint
/// * `shutdown_cmd` - The shutdown command for the blueprint manager
///
/// # Returns
///
/// * A handle to the running blueprint manager
///
/// # Errors
///
/// * If the blueprint manager fails to start
///
/// # Panics
///
/// * If the SR25519 or ECDSA keypair cannot be found
#[allow(clippy::used_underscore_binding)]
pub async fn run_blueprint_manager<F: SendFuture<'static, ()>>(
    ctx: BlueprintManagerContext,
    env: BlueprintEnvironment,
    shutdown_cmd: F,
) -> color_eyre::Result<BlueprintManagerHandle> {
    run_blueprint_manager_with_keystore(
        ctx,
        Keystore::new(KeystoreConfig::new().fs_root(&env.keystore_uri))?,
        env,
        shutdown_cmd,
    )
    .await
}

/// Runs the authentication proxy server.
///
/// This function sets up and runs an authenticated proxy server that listens on the configured host and port.
/// It creates necessary directories for the proxy's database and then starts the server.
///
/// # Arguments
///
/// * `data_dir` - The path to the data directory where the proxy's database will be stored.
/// * `auth_proxy_opts` - Configuration options for the authentication proxy, including host and port.
///
/// # Errors
///
/// This function will return an error if:
/// - It fails to create the necessary directories for the database.
/// - It fails to bind to the specified host and port.
/// - The Axum server encounters an error during operation.
pub async fn run_auth_proxy(
    data_dir: PathBuf,
    auth_proxy_opts: AuthProxyOpts,
) -> Result<(RocksDb, impl Future<Output = Result<()>>)> {
    let db_path = data_dir.join("private").join("auth-proxy").join("db");
    tokio::fs::create_dir_all(&db_path).await?;

    let proxy = blueprint_auth::proxy::AuthenticatedProxy::new(&db_path)?;
    let db = proxy.db();

    let router = proxy.router();

    Ok((db, async move {
        let listener = tokio::net::TcpListener::bind((
            auth_proxy_opts.auth_proxy_host,
            auth_proxy_opts.auth_proxy_port,
        ))
        .await?;
        info!(
            "Auth proxy listening on {}:{}",
            auth_proxy_opts.auth_proxy_host, auth_proxy_opts.auth_proxy_port
        );
        let result = axum::serve(listener, router).await;
        if let Err(err) = result {
            error!("Auth proxy error: {err}");
        }

        Ok(())
    }))
}
