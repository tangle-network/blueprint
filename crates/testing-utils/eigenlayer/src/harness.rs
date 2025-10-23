use crate::Error;
use crate::env::setup_eigenlayer_test_environment;
use alloy_primitives::address;
use alloy_primitives::{Address, U256};
use alloy_provider::Provider;
use blueprint_auth::db::RocksDb;
use blueprint_chain_setup::anvil::keys::inject_anvil_key;
use blueprint_chain_setup::anvil::{AnvilTestnet, Container};
use blueprint_core::{error, info};
use blueprint_evm_extra::util::get_provider_http;
use blueprint_manager_bridge::server::{Bridge, BridgeHandle};
use blueprint_runner::config::{BlueprintEnvironment, ContextConfig, SupportedChains};
use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
use eigenlayer_contract_deployer::core::{
    DelegationManagerConfig, DeployedCoreContracts, DeploymentConfigData, EigenPodManagerConfig,
    RewardsCoordinatorConfig, StrategyFactoryConfig, StrategyManagerConfig,
};
use std::future::Future;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::task::JoinHandle;
use url::Url;

/// Test harness for Eigenlayer network tests
pub struct EigenlayerTestHarness {
    env: BlueprintEnvironment,
    _temp_dir: TempDir,
    _container: Container,
    _auth_proxy: JoinHandle<Result<(), Error>>,
    _bridge: BridgeHandle,
}

impl EigenlayerTestHarness {
    ///
    /// Create a new `EigenlayerTestHarness`
    ///
    /// NOTE: The resulting harness will have a context of `()`. This is not valid for jobs that require
    ///       a context. See [`Self::setup_with_context()`] and [`Self::set_context()`].
    ///
    /// # Errors
    ///
    /// * See [`EigenlayerTestHarness::setup_with_context()`]
    pub async fn setup(
        owner_private_key: &str,
        test_dir: TempDir,
        testnet: AnvilTestnet,
        eigenlayer_protocol_settings: Option<EigenlayerProtocolSettings>,
    ) -> Result<Self, Error> {
        Self::setup_with_context(
            owner_private_key,
            test_dir,
            testnet,
            eigenlayer_protocol_settings,
        )
        .await
    }
}

impl EigenlayerTestHarness {
    /// Create a new `EigenlayerTestHarness` with a predefined context
    ///
    /// NOTE: If your context type depends on [`Self::env()`], see [`Self::setup()`]
    ///
    /// * Params
    /// - `owner_private_key`: The private key of the owner account
    /// - `test_dir`: The directory to store the test data
    /// - `testnet`: The Anvil testnet
    /// - `eigenlayer_protocol_settings`: The Eigenlayer protocol settings.
    ///     - Option<T>: When you set up empty Anvil testnet and re-deploy smart contracts your-self
    ///     - None: When you use the default Eigenlayer test environment
    ///
    /// # Errors
    ///
    /// * See [`crate::Error`]
    pub async fn setup_with_context(
        owner_private_key: &str,
        test_dir: TempDir,
        testnet: AnvilTestnet,
        eigenlayer_protocol_settings: Option<EigenlayerProtocolSettings>,
    ) -> Result<Self, Error> {
        // Setup temporary testing keystore
        let keystore_path = test_dir.path().join("keystore");
        inject_anvil_key(&keystore_path, owner_private_key)?;

        let eigenlayer_protocol_settings = match eigenlayer_protocol_settings {
            Some(settings) => settings,
            None => setup_eigenlayer_test_environment(testnet.http_endpoint.clone()).await,
        };

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
            testnet.http_endpoint.clone(),
            testnet.ws_endpoint.clone(),
            keystore_path.to_string_lossy().into_owned(),
            None,
            data_dir,
            None,
            SupportedChains::LocalTestnet,
            eigenlayer_protocol_settings,
        );

        // Load environment with bridge configuration
        let mut env = BlueprintEnvironment::load_with_config(context_config)
            .map_err(|e| Error::Setup(e.to_string()))?;

        env.bridge_socket_path = Some(bridge_socket_path);
        env.test_mode = true;

        Ok(Self {
            env,
            _temp_dir: test_dir,
            _container: testnet.container,
            _auth_proxy: auth_proxy,
            _bridge: bridge_handle,
        })
    }

    #[must_use]
    pub fn env(&self) -> &BlueprintEnvironment {
        &self.env
    }
}

/// Gets the accounts from the HTTP endpoint
///
/// # Panics
///
/// * See [`Provider::get_accounts()`]
#[must_use]
pub async fn get_accounts(http_endpoint: Url) -> Vec<Address> {
    let provider = get_provider_http(http_endpoint.clone());
    provider.get_accounts().await.unwrap()
}

/// Gets the owner account (first account)
#[must_use]
pub fn get_owner_account(accounts: &[Address]) -> Address {
    accounts[0]
}

/// Gets the aggregator account (ninth account)
#[must_use]
pub fn get_aggregator_account(accounts: &[Address]) -> Address {
    accounts[9]
}

/// Gets the task generator account (fourth account)
#[must_use]
pub fn get_task_generator_account(accounts: &[Address]) -> Address {
    accounts[4]
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

/// Deploy core Eigenlayer core contract
///
/// # Arguments
///
/// * `owner_private_key`: The private key of the owner account
/// * `owner_account`: The owner account
/// * `testnet`: The Anvil testnet
///
/// # Returns
///
/// * `Result<DeployedCoreContracts, Error>`: The deployed core contracts
///     - `Ok(DeployedCoreContracts)`: The deployed core contracts
///     - `Err(Error)`: The error
///
/// # Errors
/// See [`eigenlayer_contract_deployer::core::deploy_core_contracts`]
///
/// # Panics
/// See [`eigenlayer_contract_deployer::core::deploy_core_contracts`]
pub async fn deploy_eigenlayer_core_contracts(
    http_endpoint: &str,
    owner_private_key: &str,
    owner_account: Address,
) -> Result<DeployedCoreContracts, Error> {
    let core_config = DeploymentConfigData {
        strategy_manager: StrategyManagerConfig {
            init_paused_status: U256::from(0),
            init_withdrawal_delay_blocks: 1u32,
        },
        delegation_manager: DelegationManagerConfig {
            init_paused_status: U256::from(0),
            withdrawal_delay_blocks: 0u32,
        },
        eigen_pod_manager: EigenPodManagerConfig {
            init_paused_status: U256::from(0),
        },
        rewards_coordinator: RewardsCoordinatorConfig {
            init_paused_status: U256::from(0),
            max_rewards_duration: 864_000u32,
            max_retroactive_length: 432_000u32,
            max_future_length: 86_400u32,
            genesis_rewards_timestamp: 1_672_531_200_u32,
            updater: owner_account,
            activation_delay: 0u32,
            calculation_interval_seconds: 86_400u32,
            global_operator_commission_bips: 1_000u16,
        },
        strategy_factory: StrategyFactoryConfig {
            init_paused_status: U256::from(0),
        },
    };

    Ok(eigenlayer_contract_deployer::core::deploy_core_contracts(
        http_endpoint,
        owner_private_key,
        owner_account,
        core_config,
        Some(address!("00000000219ab540356cBB839Cbe05303d7705Fa")),
        Some(1_564_000),
    )
    .await
    .unwrap())
}
