use crate::Error;
use crate::env::{EigenlayerTestEnvironment, setup_eigenlayer_test_environment};
use alloy_primitives::Address;
use alloy_provider::RootProvider;
use blueprint_auth::db::RocksDb;
use blueprint_chain_setup::anvil::keys::{ANVIL_PRIVATE_KEYS, inject_anvil_key};
use blueprint_chain_setup::anvil::{Container, start_empty_anvil_testnet};
use blueprint_core::{error, info};
use blueprint_evm_extra::util::get_provider_http;
use blueprint_manager_bridge::server::{Bridge, BridgeHandle};
use blueprint_runner::config::{BlueprintEnvironment, ContextConfig, SupportedChains};
use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
use std::future::Future;
use std::marker::PhantomData;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::task::JoinHandle;
use url::Url;

/// Configuration for the Eigenlayer test harness
#[derive(Default)]
pub struct EigenlayerTestConfig {
    pub http_endpoint: Option<Url>,
    pub ws_endpoint: Option<Url>,
    pub eigenlayer_contract_addresses: Option<EigenlayerProtocolSettings>,
}

/// Test harness for Eigenlayer network tests
pub struct EigenlayerTestHarness<Ctx> {
    env: BlueprintEnvironment,
    config: EigenlayerTestConfig,
    pub http_endpoint: Url,
    pub ws_endpoint: Url,
    pub accounts: Vec<Address>,
    pub eigenlayer_contract_addresses: EigenlayerProtocolSettings,
    _temp_dir: TempDir,
    _container: Container,
    _phantom: PhantomData<Ctx>,
    _auth_proxy: JoinHandle<Result<(), Error>>,
    _bridge: BridgeHandle,
}

impl EigenlayerTestHarness<()> {
    /// Create a new `EigenlayerTestHarness`
    ///
    /// NOTE: The resulting harness will have a context of `()`. This is not valid for jobs that require
    ///       a context. See [`Self::setup_with_context()`] and [`Self::set_context()`].
    ///
    /// # Errors
    ///
    /// * See [`Self::setup_with_context()`]
    pub async fn setup(test_dir: TempDir) -> Result<Self, Error> {
        Self::setup_with_context(test_dir, ()).await
    }
}

impl<Ctx> EigenlayerTestHarness<Ctx>
where
    Ctx: Clone + Send + Sync + 'static,
{
    /// Create a new `EigenlayerTestHarness` with a predefined context
    ///
    /// NOTE: If your context type depends on [`Self::env()`], see [`Self::setup()`]
    ///
    /// # Errors
    ///
    /// * TODO
    pub async fn setup_with_context(test_dir: TempDir, _context: Ctx) -> Result<Self, Error> {
        // Start local Anvil testnet
        let testnet = start_empty_anvil_testnet(true).await;

        // Setup Eigenlayer test environment
        let EigenlayerTestEnvironment {
            accounts,
            http_endpoint,
            ws_endpoint,
            eigenlayer_contract_addresses,
        } = setup_eigenlayer_test_environment(testnet.http_endpoint, testnet.ws_endpoint).await;

        // Setup temporary testing keystore
        let keystore_path = test_dir.path().join("keystore");
        inject_anvil_key(&keystore_path, ANVIL_PRIVATE_KEYS[0])?;

        let data_dir = test_dir.path().join("data");
        tokio::fs::create_dir_all(&data_dir).await?;

        // Setup auth proxy
        const DEFAULT_AUTH_PROXY_PORT: u16 = 50051;
        let (auth_proxy_db, auth_proxy_task) =
            run_auth_proxy(test_dir.path().to_path_buf(), DEFAULT_AUTH_PROXY_PORT).await?;

        let auth_proxy = tokio::spawn(auth_proxy_task);

        // Setup bridge
        let runtime_dir = test_dir.path().join("runtime");
        tokio::fs::create_dir_all(&runtime_dir).await?;

        let bridge = Bridge::new(runtime_dir, String::from("service"), auth_proxy_db, true);
        let bridge_socket_path = bridge.base_socket_path();

        let (bridge_handle, _alive_rx) = bridge.spawn()?;

        // Create context config
        let context_config = ContextConfig::create_eigenlayer_config(
            Url::parse(&http_endpoint)?,
            Url::parse(&ws_endpoint)?,
            keystore_path.to_string_lossy().into_owned(),
            None,
            data_dir,
            None,
            SupportedChains::LocalTestnet,
            eigenlayer_contract_addresses,
        );

        // Load environment with bridge configuration
        let mut env = BlueprintEnvironment::load_with_config(context_config)
            .map_err(|e| Error::Setup(e.to_string()))?;

        env.bridge_socket_path = Some(bridge_socket_path);
        env.test_mode = true;

        // Create config
        let config = EigenlayerTestConfig {
            http_endpoint: Some(Url::parse(&http_endpoint)?),
            ws_endpoint: Some(Url::parse(&ws_endpoint)?),
            eigenlayer_contract_addresses: Some(eigenlayer_contract_addresses),
        };

        Ok(Self {
            env,
            config,
            http_endpoint: Url::parse(&http_endpoint)?,
            ws_endpoint: Url::parse(&ws_endpoint)?,
            accounts,
            eigenlayer_contract_addresses,
            _temp_dir: test_dir,
            _container: testnet.container,
            _phantom: core::marker::PhantomData,
            _auth_proxy: auth_proxy,
            _bridge: bridge_handle,
        })
    }

    #[must_use]
    #[allow(clippy::used_underscore_binding)]
    pub fn set_context<Ctx2: Clone + Send + Sync + 'static>(
        self,
        _context: Ctx2,
    ) -> EigenlayerTestHarness<Ctx2> {
        EigenlayerTestHarness {
            env: self.env,
            config: self.config,
            http_endpoint: self.http_endpoint,
            ws_endpoint: self.ws_endpoint,
            accounts: self.accounts,
            eigenlayer_contract_addresses: self.eigenlayer_contract_addresses,
            _temp_dir: self._temp_dir,
            _container: self._container,
            _phantom: PhantomData::<Ctx2>,
            _auth_proxy: self._auth_proxy,
            _bridge: self._bridge,
        }
    }

    #[must_use]
    pub fn env(&self) -> &BlueprintEnvironment {
        &self.env
    }
}

impl<Ctx> EigenlayerTestHarness<Ctx> {
    /// Gets a provider for the HTTP endpoint
    #[must_use]
    pub fn provider(&self) -> RootProvider {
        get_provider_http(self.http_endpoint.as_str())
    }

    /// Gets the list of accounts
    #[must_use]
    pub fn accounts(&self) -> &[Address] {
        &self.accounts
    }

    /// Gets the owner account (first account)
    #[must_use]
    pub fn owner_account(&self) -> Address {
        self.accounts[0]
    }

    /// Gets the aggregator account (ninth account)
    #[must_use]
    pub fn aggregator_account(&self) -> Address {
        self.accounts[9]
    }

    /// Gets the task generator account (fourth account)
    #[must_use]
    pub fn task_generator_account(&self) -> Address {
        self.accounts[4]
    }
}

/// Runs the authentication proxy server.
///
/// This function sets up and runs an authenticated proxy server that listens on the configured host and port.
/// It creates necessary directories for the proxy's database and then starts the server.
///
/// # Arguments
///
/// * `data_dir` - The path to the data directory where the proxy's database will be stored.
/// * `auth_proxy_port` - The port on which the proxy server will listen.
///
/// # Errors
///
/// This function will return an error if:
/// - It fails to create the necessary directories for the database.
/// - It fails to bind to the specified host and port.
/// - The Axum server encounters an error during operation.
async fn run_auth_proxy(
    data_dir: PathBuf,
    auth_proxy_port: u16,
) -> Result<(RocksDb, impl Future<Output = Result<(), Error>>), Error> {
    let db_path = data_dir.join("private").join("auth-proxy").join("db");
    tokio::fs::create_dir_all(&db_path).await?;

    let proxy = blueprint_auth::proxy::AuthenticatedProxy::new(&db_path)?;
    let db = proxy.db();

    let task = async move {
        let router = proxy.router();
        let listener =
            tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, auth_proxy_port)).await?;
        info!(
            "Auth proxy listening on {}:{}",
            Ipv4Addr::LOCALHOST,
            auth_proxy_port
        );
        let result = axum::serve(listener, router).await;
        if let Err(err) = result {
            error!("Auth proxy error: {err}");
        }

        Ok(())
    };

    Ok((db, task))
}
