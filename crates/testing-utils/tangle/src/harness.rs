use crate::runner::MockHeartbeatConsumer;
use crate::Error;
use crate::multi_node::{find_open_tcp_bind_port, MultiNodeTestEnv};
use crate::{InputValue, OutputValue, keys::inject_tangle_key};
use blueprint_auth::proxy::DEFAULT_AUTH_PROXY_PORT;
use blueprint_chain_setup::tangle::testnet::SubstrateNode;
use blueprint_chain_setup::tangle::transactions;
use blueprint_chain_setup::tangle::transactions::setup_operators_with_service;
use blueprint_client_tangle::client::TangleClient;
use blueprint_contexts::tangle::TangleClientContext;
use blueprint_core::debug;
use blueprint_core_testing_utils::setup_log;
use blueprint_crypto_tangle_pair_signer::TanglePairSigner;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::crypto::sp_core::{SpEcdsa, SpSr25519};
use blueprint_runner::config::BlueprintEnvironment;
use blueprint_runner::config::ContextConfig;
use blueprint_runner::config::SupportedChains;
use blueprint_runner::error::RunnerError;
use blueprint_runner::tangle::error::TangleError;
use blueprint_std::io;
use blueprint_std::path::{Path, PathBuf};
use tangle_subxt::subxt::utils::AccountId32;
use tangle_subxt::tangle_testnet_runtime::api::assets::events::created::AssetId;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::pricing::PricingQuote;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::{AssetSecurityCommitment, AssetSecurityRequirement};
use blueprint_tangle_extra::serde::new_bounded_string;
use std::marker::PhantomData;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tangle_subxt::tangle_testnet_runtime::api::services::calls::types::register::RegistrationArgs;
use tangle_subxt::tangle_testnet_runtime::api::services::calls::types::request::RequestArgs;
use tangle_subxt::tangle_testnet_runtime::api::services::events::JobCalled;
use tangle_subxt::tangle_testnet_runtime::api::services::{
    calls::types::{call::Job, register::Preferences},
    events::JobResultSubmitted,
};
use tempfile::TempDir;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use blueprint_core::{error, info};
use url::Url;
use blueprint_auth::db::RocksDb;
use blueprint_manager_bridge::server::{Bridge, BridgeHandle};
use blueprint_pricing_engine_lib::{init_benchmark_cache, init_operator_signer, load_pricing_from_toml, OperatorConfig, DEFAULT_CONFIG};
use blueprint_pricing_engine_lib::service::rpc::server::run_rpc_server;

pub const ENDOWED_TEST_NAMES: [&str; 10] = [
    "Alice",
    "Bob",
    "Charlie",
    "Dave",
    "Eve",
    "Ferdinand",
    "Gina",
    "Hank",
    "Ivy",
    "Jack",
];

/// Configuration for the Tangle test harness
#[derive(Clone, Debug)]
pub struct TangleTestConfig {
    pub http_endpoint: Url,
    pub ws_endpoint: Url,
    pub temp_dir: PathBuf,
    pub bridge_socket_path: PathBuf,
}

/// Test harness for Tangle network tests
pub struct TangleTestHarness<Ctx = ()> {
    client: TangleClient,
    pub sr25519_signer: TanglePairSigner<sp_core::sr25519::Pair>,
    pub ecdsa_signer: TanglePairSigner<sp_core::ecdsa::Pair>,
    pub alloy_key: alloy_signer_local::PrivateKeySigner,
    config: TangleTestConfig,
    _phantom: PhantomData<Ctx>,

    // Handles to keep alive
    _temp_dir: tempfile::TempDir,
    _node: SubstrateNode,
    _auth_proxy: JoinHandle<Result<(), Error>>,
    _bridge: BridgeHandle,
}

/// Create a new Tangle test harness
///
/// # Returns
///
/// The [`BlueprintEnvironment`] for the relevant node
///
/// # Errors
///
/// Returns an error if the keystore fails to be created
pub async fn generate_env_from_node_id(
    identity: &str,
    http_endpoint: Url,
    ws_endpoint: Url,
    test_dir: &Path,
) -> Result<BlueprintEnvironment, RunnerError> {
    let node_dir = test_dir.join(identity.to_ascii_lowercase());
    let keystore_path = node_dir.join("keystore");
    tokio::fs::create_dir_all(&keystore_path).await?;
    inject_tangle_key(&keystore_path, &format!("//{identity}"))
        .map_err(|err| RunnerError::Tangle(TangleError::Keystore(err)))?;

    let data_dir = node_dir.join("data");
    tokio::fs::create_dir_all(&data_dir).await?;

    // Create context config
    let context_config = ContextConfig::create_tangle_config(
        http_endpoint,
        ws_endpoint,
        keystore_path.display().to_string(),
        None,
        data_dir,
        None,
        SupportedChains::LocalTestnet,
        0,
        Some(0),
    );

    // Load environment
    let mut env =
        BlueprintEnvironment::load_with_config(context_config).map_err(RunnerError::Config)?;

    // Always set test mode, dont require callers to set env vars
    env.test_mode = true;

    Ok(env)
}

impl<Ctx> TangleTestHarness<Ctx>
where
    Ctx: Clone + Send + Sync + 'static,
{
    /// Create a new `TangleTestHarness`
    ///
    /// # Errors
    ///
    /// TODO
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use blueprint_tangle_testing_utils::TangleTestHarness;
    /// use tempfile::TempDir;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let tmp_dir = TempDir::new()?;
    /// let harness = TangleTestHarness::setup(tmp_dir).await?;
    /// # let _: TangleTestHarness<()> = harness;
    /// # Ok(()) }
    /// ```
    pub async fn setup(test_dir: TempDir) -> Result<Self, Error> {
        setup_log();
        // Start Local Tangle Node
        let node = blueprint_chain_setup::tangle::run(
            blueprint_chain_setup::tangle::NodeConfig::new(false).with_log_target("evm", "trace"),
        )
        .await
        .map_err(|e| Error::Setup(e.to_string()))?;
        let http_endpoint = Url::parse(&format!("http://127.0.0.1:{}", node.ws_port()))?;
        let ws_endpoint = Url::parse(&format!("ws://127.0.0.1:{}", node.ws_port()))?;

        let (auth_proxy_db, auth_proxy_task) =
            run_auth_proxy(test_dir.path().to_path_buf(), DEFAULT_AUTH_PROXY_PORT).await?;

        let auth_proxy = tokio::spawn(auth_proxy_task);

        // Alice idx = 0
        let mut alice_env = generate_env_from_node_id(
            ENDOWED_TEST_NAMES[0],
            http_endpoint.clone(),
            ws_endpoint.clone(),
            test_dir.path(),
        )
        .await?;

        // Setup bridge
        let runtime_dir = test_dir.path().join("runtime");
        tokio::fs::create_dir_all(&runtime_dir).await?;

        let bridge = Bridge::new(runtime_dir, String::from("service"), auth_proxy_db, true);
        let bridge_socket_path = bridge.base_socket_path();

        let (bridge_handle, _alive_rx) = bridge.spawn()?;
        alice_env.bridge_socket_path = Some(bridge_socket_path.clone());

        // Create config
        let config = TangleTestConfig {
            http_endpoint: http_endpoint.clone(),
            ws_endpoint: ws_endpoint.clone(),
            temp_dir: test_dir.path().to_path_buf(),
            bridge_socket_path,
        };

        // Setup signers
        let keystore = alice_env.keystore();
        let sr25519_public = keystore.first_local::<SpSr25519>()?;
        let sr25519_pair = keystore.get_secret::<SpSr25519>(&sr25519_public)?;
        let sr25519_signer = TanglePairSigner::new(sr25519_pair.0);

        let ecdsa_public = keystore.first_local::<SpEcdsa>()?;
        let ecdsa_pair = keystore.get_secret::<SpEcdsa>(&ecdsa_public)?;
        let ecdsa_signer = TanglePairSigner::new(ecdsa_pair.0);
        let alloy_key = ecdsa_signer
            .alloy_key()
            .map_err(|e| Error::Setup(e.to_string()))?;

        let client = alice_env.tangle_client().await?;

        let harness = TangleTestHarness {
            client,
            sr25519_signer,
            ecdsa_signer,
            alloy_key,
            config,
            _phantom: PhantomData,
            _temp_dir: test_dir,
            _node: node,
            _auth_proxy: auth_proxy,
            _bridge: bridge_handle,
        };

        // Deploy MBSM if needed
        harness
            .deploy_mbsm_if_needed()
            .await
            .map_err(|e| Error::Setup(format!("Failed to deploy MBSM: {e}")))?;

        Ok(harness)
    }

    #[must_use]
    pub fn env(&self) -> &BlueprintEnvironment {
        &self.client.config
    }

    #[must_use]
    pub fn config(&self) -> &TangleTestConfig {
        &self.config
    }
}

struct NodeInfo {
    env: BlueprintEnvironment,
    client: TangleClient,
    preferences: Preferences,
    // To keep the server alive
    _pricing_rpc: JoinHandle<()>,
}

#[derive(Debug, Clone)]
pub struct SetupServicesOpts<const N: usize> {
    /// Whether to exit after registration
    pub exit_after_registration: bool,
    /// Whether to skip automatic service request
    pub skip_service_request: bool,
    /// Registration parameters for each node
    pub registration_args: [RegistrationArgs; N],
    /// Request parameters for the service
    pub request_args: RequestArgs,
}

impl<const N: usize> Default for SetupServicesOpts<N> {
    fn default() -> Self {
        Self {
            exit_after_registration: false,
            skip_service_request: false,
            registration_args: vec![RegistrationArgs::default(); N].try_into().unwrap(),
            request_args: RequestArgs::default(),
        }
    }
}

impl<Ctx> TangleTestHarness<Ctx>
where
    Ctx: Clone + Send + Sync + 'static,
{
    async fn get_all_node_info<const N: usize>(&self) -> Result<Vec<NodeInfo>, RunnerError> {
        let mut nodes = vec![];

        for name in &ENDOWED_TEST_NAMES[..N] {
            let mut env = generate_env_from_node_id(
                name,
                self.config.http_endpoint.clone(),
                self.config.ws_endpoint.clone(),
                &self.config.temp_dir,
            )
            .await?;
            env.bridge_socket_path = Some(self.config.bridge_socket_path.clone());

            let client = env
                .tangle_client()
                .await
                .map_err(|err| RunnerError::Other(err.into()))?;

            let keystore = env.keystore();
            let ecdsa_public = keystore
                .first_local::<SpEcdsa>()
                .map_err(|err| RunnerError::Tangle(TangleError::Keystore(err)))?;

            let rpc_port = find_open_tcp_bind_port();
            let rpc_address = format!("127.0.0.1:{rpc_port}");
            info!("Binding node {name} to {rpc_address}");

            let operator_config = OperatorConfig {
                keystore_path: self.config.temp_dir.join(name),
                rpc_port,
                ..Default::default()
            };

            let benchmark_cache = init_benchmark_cache(&operator_config)
                .await
                .map_err(|e| RunnerError::Other(e.into()))?;

            let pricing_config =
                load_pricing_from_toml(DEFAULT_CONFIG).map_err(|e| RunnerError::Other(e.into()))?;

            let operator_signer =
                init_operator_signer(&operator_config, &operator_config.keystore_path)
                    .map_err(|e| RunnerError::Other(e.into()))?;

            let pricing_rpc = tokio::spawn(async move {
                if let Err(e) = run_rpc_server(
                    Arc::new(operator_config),
                    benchmark_cache,
                    Arc::new(Mutex::new(pricing_config)),
                    operator_signer,
                )
                .await
                {
                    blueprint_core::error!("gRPC server error: {}", e);
                }
            });

            let preferences = Preferences {
                key: blueprint_runner::tangle::config::decompress_pubkey(&ecdsa_public.0.0)
                    .unwrap(),
                rpc_address: new_bounded_string(rpc_address),
            };

            nodes.push(NodeInfo {
                env,
                client,
                preferences,
                _pricing_rpc: pricing_rpc,
            });
        }

        Ok(nodes)
    }

    /// Gets a reference to the Tangle client
    #[must_use]
    pub fn client(&self) -> &TangleClient {
        &self.client
    }

    /// Deploys MBSM if not already deployed
    async fn deploy_mbsm_if_needed(&self) -> Result<(), Error> {
        let latest_revision = transactions::get_latest_mbsm_revision(&self.client)
            .await
            .map_err(|e| Error::Setup(e.to_string()))?;

        if let Some((rev, addr)) = latest_revision {
            debug!("MBSM is deployed at revision #{rev} at address {addr}");
            return Ok(());
        }

        debug!("MBSM is not deployed");

        let bytecode = tnt_core_bytecode::bytecode::MASTER_BLUEPRINT_SERVICE_MANAGER;
        transactions::deploy_new_mbsm_revision(
            self.config.ws_endpoint.as_str(),
            &self.client,
            &self.sr25519_signer,
            self.alloy_key.clone(),
            bytecode,
            alloy_primitives::address!("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"), // TODO: User-defined address
        )
        .await
        .map_err(|e| Error::Setup(e.to_string()))?;

        Ok(())
    }

    /// Creates deploy options for a blueprint
    ///
    /// # Errors
    ///
    /// See [`read_cargo_toml_file()`]
    ///
    /// [`read_cargo_toml_file()`]: blueprint_core_testing_utils::read_cargo_toml_file
    pub fn create_deploy_opts(
        &self,
        manifest_path: PathBuf,
    ) -> io::Result<blueprint_chain_setup::tangle::deploy::Opts> {
        Ok(blueprint_chain_setup::tangle::deploy::Opts {
            pkg_name: Some(self.get_blueprint_name(&manifest_path)?),
            http_rpc_url: self.config.http_endpoint.to_string(),
            ws_rpc_url: self.config.ws_endpoint.to_string(),
            manifest_path,
            signer: Some(self.sr25519_signer.clone()),
            signer_evm: Some(self.alloy_key.clone()),
        })
    }

    #[allow(clippy::unused_self)]
    fn get_blueprint_name(&self, manifest_path: &std::path::Path) -> io::Result<String> {
        let manifest = blueprint_core_testing_utils::read_cargo_toml_file(manifest_path)?;
        Ok(manifest.package.unwrap().name)
    }

    /// Deploys a blueprint from the current directory and returns its ID
    ///
    /// # Errors
    ///
    /// See [`deploy_to_tangle()`]
    ///
    /// [`deploy_to_tangle()`]: blueprint_chain_setup::tangle::deploy::deploy_to_tangle
    pub async fn deploy_blueprint(&self) -> Result<u64, Error> {
        let manifest_path = std::env::current_dir()?.join("Cargo.toml");
        let opts = self.create_deploy_opts(manifest_path)?;
        let blueprint_id = blueprint_chain_setup::tangle::deploy::deploy_to_tangle(opts)
            .await
            .map_err(|e| Error::Setup(e.to_string()))?;
        Ok(blueprint_id)
    }

    /// Sets up a complete service environment with initialized event handlers
    ///
    /// # Returns
    /// A tuple of the test environment, the service ID, and the blueprint ID i.e., (`test_env`, `service_id`, `blueprint_id`)
    ///
    /// # Note
    /// The Service ID will always be 0 if automatic registration is disabled, as there is not yet a service to have an ID
    ///
    /// # Errors
    ///
    /// * See [`Self::setup_services`], [`Self::deploy_blueprint()`] and [`MultiNodeTestEnv::new()`]
    pub async fn setup_services_with_options<const N: usize>(
        &self,
        SetupServicesOpts {
            exit_after_registration,
            skip_service_request,
            registration_args,
            request_args,
        }: SetupServicesOpts<N>,
    ) -> Result<(MultiNodeTestEnv<Ctx, MockHeartbeatConsumer>, u64, u64), Error> {
        const { assert!(N > 0, "Must have at least 1 initial node") };

        // Deploy blueprint
        let blueprint_id = self.deploy_blueprint().await?;

        let nodes = self.get_all_node_info::<N>().await?;

        // Setup operator and get service
        let service_id = if exit_after_registration {
            0
        } else {
            let mut all_clients = Vec::new();
            let mut all_signers = Vec::new();
            let mut all_preferences = Vec::new();

            for node in nodes {
                let keystore = node.env.keystore();
                let sr25519_public = keystore
                    .first_local::<SpSr25519>()
                    .map_err(|err| RunnerError::Tangle(TangleError::Keystore(err)))?;
                let sr25519_pair = keystore
                    .get_secret::<SpSr25519>(&sr25519_public)
                    .map_err(|err| RunnerError::Tangle(TangleError::Keystore(err)))?;
                let sr25519_signer = TanglePairSigner::new(sr25519_pair.0);
                all_clients.push(node.client);
                all_signers.push(sr25519_signer);
                all_preferences.push(node.preferences);
            }

            setup_operators_with_service(
                &all_clients[..N],
                &all_signers[..N],
                blueprint_id,
                &all_preferences,
                &registration_args,
                request_args.clone(),
                exit_after_registration,
                skip_service_request,
            )
            .await
            .map_err(|e| Error::Setup(e.to_string()))?
        };

        // Create and initialize the new multi-node environment
        let executor = MultiNodeTestEnv::new::<N>(self.config.clone());

        Ok((executor, service_id, blueprint_id))
    }

    /// Sets up a complete service environment with initialized event handlers
    ///
    /// # Returns
    /// A tuple of the test environment, the service ID, and the blueprint ID i.e., (`test_env`, `service_id`, `blueprint_id`)
    ///
    /// # Note
    /// The Service ID will always be 0 if automatic registration is disabled, as there is not yet a service to have an ID
    ///
    /// # Errors
    ///
    /// * See [`Self::deploy_blueprint()`] and [`MultiNodeTestEnv::new()`]
    pub async fn setup_services<const N: usize>(
        &self,
        exit_after_registration: bool,
    ) -> Result<(MultiNodeTestEnv<Ctx, MockHeartbeatConsumer>, u64, u64), Error> {
        const { assert!(N > 0, "Must have at least 1 initial node") };

        self.setup_services_with_options::<N>(SetupServicesOpts {
            exit_after_registration,
            skip_service_request: false,
            registration_args: vec![RegistrationArgs::default(); N].try_into().unwrap(),
            request_args: RequestArgs::default(),
        })
        .await
    }

    /// Requests a service with a given blueprint using pricing quotes.
    ///
    /// # Arguments
    /// * `blueprint_id` - The ID of the blueprint to request
    /// * `request_args` - The arguments for the request
    /// * `quotes` - The pricing quotes for the service
    /// * `quote_signatures` - The signatures for the pricing quotes
    /// * `security_commitments` - The security commitments for the service
    /// * `optional_assets` - Optional asset security requirements (defaults to Custom(0) at 50%-80% if not provided)
    ///
    /// # Errors
    ///
    /// Returns an error if the transaction fails
    #[allow(clippy::too_many_arguments)]
    pub async fn request_service_with_quotes(
        &self,
        blueprint_id: u64,
        request_args: RequestArgs,
        operators: Vec<AccountId32>,
        quotes: Vec<PricingQuote>,
        quote_signatures: Vec<sp_core::ecdsa::Signature>,
        security_commitments: Vec<AssetSecurityCommitment<AssetId>>,
        optional_assets: Option<Vec<AssetSecurityRequirement<AssetId>>>,
    ) -> Result<u64, Error> {
        transactions::request_service_with_quotes(
            &self.client,
            &self.sr25519_signer,
            blueprint_id,
            request_args,
            operators,
            quotes,
            quote_signatures,
            security_commitments,
            optional_assets,
        )
        .await
        .map_err(|e| Error::Setup(e.to_string()))
    }

    /// Submits a job to be executed
    ///
    /// # Arguments
    /// * `service_id` - The ID of the service to submit the job to
    /// * `job_id` - The ID of the job to submit
    /// * `inputs` - The input values for the job
    ///
    /// # Returns
    /// The submitted job if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the transaction fails
    pub async fn submit_job(
        &self,
        service_id: u64,
        job_id: u8,
        inputs: Vec<InputValue>,
    ) -> Result<JobCalled, Error> {
        let job = transactions::submit_job(
            &self.client,
            &self.sr25519_signer,
            service_id,
            Job::from(job_id),
            inputs,
            0, // TODO: Should this take a call ID? or leave it up to the caller to verify?
        )
        .await
        .map_err(|e| Error::Setup(e.to_string()))?;

        Ok(job)
    }

    /// Executes a previously submitted job and waits for completion
    ///
    /// # Arguments
    /// * `service_id` - The ID of the service the job was submitted to
    /// * `job` - The submitted job to execute
    ///
    /// # Returns
    /// The job results if execution was successful
    ///
    /// # Errors
    ///
    /// Returns an error if no job result is found.
    pub async fn wait_for_job_execution(
        &self,
        service_id: u64,
        job: JobCalled,
    ) -> Result<JobResultSubmitted, Error> {
        let results = transactions::wait_for_completion_of_tangle_job(
            &self.client,
            service_id,
            job.call_id,
            1,
        )
        .await
        .map_err(|e| Error::Setup(e.to_string()))?;

        Ok(results)
    }

    /// Verifies that job results match expected outputs
    ///
    /// # Arguments
    /// * `results` - The actual job results
    /// * `expected` - The expected output values
    ///
    /// # Returns
    /// The verified results if they match expectations
    ///
    /// # Panics
    ///
    /// If the results don't match the expected outputs
    pub fn verify_job(&self, results: &JobResultSubmitted, expected: impl AsRef<[OutputValue]>) {
        assert_eq!(
            results.result.len(),
            expected.as_ref().len(),
            "Number of outputs doesn't match expected"
        );

        for (result, expected) in results.result.iter().zip(expected.as_ref().iter()) {
            assert_eq!(result, expected);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_harness_setup() {
        let test_dir = TempDir::new().unwrap();
        let harness = Box::pin(TangleTestHarness::<()>::setup(test_dir)).await;
        assert!(harness.is_ok(), "Harness setup should succeed");

        let harness = harness.unwrap();
        assert!(
            harness.client().now().await.is_some(),
            "Client should be connected to live chain"
        );
    }

    #[tokio::test]
    async fn test_deploy_mbsm() {
        let test_dir = TempDir::new().unwrap();
        let harness = Box::pin(TangleTestHarness::<()>::setup(test_dir))
            .await
            .unwrap();

        // MBSM should be deployed during setup
        let latest_revision = transactions::get_latest_mbsm_revision(harness.client())
            .await
            .unwrap();
        assert!(latest_revision.is_some(), "MBSM should be deployed");
    }
}
