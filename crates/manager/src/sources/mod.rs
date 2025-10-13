use crate::blueprint::native::FilteredBlueprint;
use crate::config::{BlueprintManagerConfig, BlueprintManagerContext};
use crate::rt::ResourceLimits;
use crate::rt::service::Service;
use blueprint_runner::config::{BlueprintEnvironment, Protocol, SupportedChains};
use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
use std::path::{Path, PathBuf};
use url::Url;

#[cfg(feature = "containers")]
pub mod container;
pub mod github;
pub mod testing;

#[auto_impl::auto_impl(Box)]
#[dynosaur::dynosaur(pub(crate) DynBlueprintSource)]
pub trait BlueprintSourceHandler: Send + Sync {
    fn fetch(
        &mut self,
        cache_dir: &Path,
    ) -> impl Future<Output = crate::error::Result<PathBuf>> + Send;
    #[allow(clippy::too_many_arguments)]
    fn spawn(
        &mut self,
        ctx: &BlueprintManagerContext,
        limits: ResourceLimits,
        blueprint_config: &BlueprintEnvironment,
        id: u32,
        env: BlueprintEnvVars,
        args: BlueprintArgs,
        sub_service_str: &str,
        cache_dir: &Path,
        runtime_dir: &Path,
    ) -> impl Future<Output = crate::error::Result<Service>> + Send;
    fn blueprint_id(&self) -> u64;
    fn name(&self) -> String;
}

unsafe impl Send for DynBlueprintSource<'_> {}
unsafe impl Sync for DynBlueprintSource<'_> {}

pub struct BlueprintArgs {
    pub test_mode: bool,
    pub pretty: bool,
    pub verbose: u8,

    // Eigenlayer config

    /// The address of the slasher contract
    pub slasher_address: Option<String>,
    /// The address of the pause registry contract
    pub pause_registry_address: Option<String>,
    /// The address of the allocation manager contract
    pub allocation_manager_address: Option<String>,
    /// The address of the registry coordinator contract
    pub registry_coordinator_address: Option<String>,
    /// The address of the operator state retriever contract
    pub operator_state_retriever_address: Option<String>,
    /// The address of the operator registry contract
    pub delegation_manager_address: Option<String>,
    /// The address of the Service Manager contract
    pub service_manager_address: Option<String>,
    /// The address of the Stake Registry contract
    pub stake_registry_address: Option<String>,
    /// The address of the strategy manager contract
    pub strategy_manager_address: Option<String>,
    /// The address of the avs registry contract
    pub avs_directory_address: Option<String>,
    /// The address of the rewards coordinator contract
    pub rewards_coordinator_address: Option<String>,
    /// The address of the permission controller contract
    pub permission_controller_address: Option<String>,
    /// The address of the strategy contract
    pub strategy_address: Option<String>,

    // TODO(daniel): Implement Keystore Password
    // #[cfg(feature = "eigenlayer")]
    // /// The keystore password to reveal config
    // pub keystore_password: Option<String>,
}

impl BlueprintArgs {
    #[must_use]
    pub fn new(manager_config: &BlueprintManagerConfig) -> Self {
        if manager_config.test_mode {
            blueprint_core::warn!("Test mode is enabled");
        }

        // TODO(daniel): Add support for keystore password
        // if let Some(keystore_password) = &env.keystore_password {
        //     arguments.push(format!("--keystore-password={}", keystore_password));
        // }

        let default_contract_address = EigenlayerProtocolSettings::default();

        Self {
            test_mode: manager_config.test_mode,
            pretty: manager_config.pretty,
            verbose: manager_config.verbose,
            slasher_address: Some(default_contract_address.slasher_address.to_string()),
            pause_registry_address: Some(default_contract_address.pause_registry_address.to_string()),
            allocation_manager_address: Some(default_contract_address.allocation_manager_address.to_string()),
            registry_coordinator_address: Some(default_contract_address.registry_coordinator_address.to_string()),
            operator_state_retriever_address: Some(default_contract_address.operator_state_retriever_address.to_string()),
            delegation_manager_address: Some(default_contract_address.delegation_manager_address.to_string()),
            service_manager_address: Some(default_contract_address.service_manager_address.to_string()),
            stake_registry_address: Some(default_contract_address.stake_registry_address.to_string()),
            strategy_manager_address: Some(default_contract_address.strategy_manager_address.to_string()),
            avs_directory_address: Some(default_contract_address.avs_directory_address.to_string()),
            rewards_coordinator_address: Some(default_contract_address.rewards_coordinator_address.to_string()),
            permission_controller_address: Some(default_contract_address.permission_controller_address.to_string()),
            strategy_address: Some(default_contract_address.strategy_address.to_string())
        }
    }

    #[must_use]
    pub fn encode(&self, run: bool) -> Vec<String> {
        let mut arguments = vec![];
        if run {
            arguments.push("run".to_string());
        }

        if self.test_mode {
            arguments.push("--test-mode".to_string());
        }

        if self.pretty {
            arguments.push("--pretty".to_string());
        }

        // Uses occurrences of clap short -v
        if self.verbose > 0 {
            arguments.push(format!("-{}", "v".repeat(self.verbose as usize)));
        }

        // Eigenlayer config
        if let Some(slasher_addr) = &self.slasher_address {
            arguments.push(format!("--slasher-address={}", slasher_addr));
        }
        if let Some(pause_registry_addr) = &self.pause_registry_address {
            arguments.push(format!("--pause-registry-address={}", pause_registry_addr));
        }
        if let Some(allocation_manager_addr) = &self.allocation_manager_address {
            arguments.push(format!("--allocation-manager-address={}", allocation_manager_addr));
        }
        if let Some(registry_coordinator_addr) = &self.registry_coordinator_address {
            arguments.push(format!("--registry-coordinator-address={}", registry_coordinator_addr));
        }
        if let Some(operator_state_retriever_addr) = &self.operator_state_retriever_address {
            arguments.push(format!("--operator-state-retriever-address={}", operator_state_retriever_addr));
        }
        if let Some(delegation_manager_addr) = &self.delegation_manager_address {
            arguments.push(format!("--delegation-manager-address={}", delegation_manager_addr));
        }
        if let Some(service_manager_addr) = &self.service_manager_address {
            arguments.push(format!("--service-manager-address={}", service_manager_addr));
        }
        if let Some(stake_registry_addr) = &self.stake_registry_address {
            arguments.push(format!("--stake-registry-address={}", stake_registry_addr));
        }
        if let Some(strategy_manager_addr) = &self.strategy_manager_address {
            arguments.push(format!("--strategy-manager-address={}", strategy_manager_addr));
        }
        if let Some(avs_directory_addr) = &self.avs_directory_address {
            arguments.push(format!("--avs-directory-address={}", avs_directory_addr));
        }
        if let Some(rewards_coordinator_addr) = &self.rewards_coordinator_address {
            arguments.push(format!("--rewards-coordinator-address={}", rewards_coordinator_addr));
        }
        if let Some(permission_controller_addr) = &self.permission_controller_address {
            arguments.push(format!("--permission-controller-address={}", permission_controller_addr));
        }
        if let Some(strategy_addr) = &self.strategy_address {
            arguments.push(format!("--strategy-address={}", strategy_addr));
        }

        arguments
    }
}

pub struct BlueprintEnvVars {
    pub http_rpc_endpoint: Url,
    pub ws_rpc_endpoint: Url,
    #[cfg(feature = "tee")]
    pub kms_endpoint: Url,
    pub keystore_uri: String,
    pub data_dir: PathBuf,
    pub blueprint_id: u64,
    pub service_id: u64,
    pub protocol: Protocol,
    pub chain: Option<SupportedChains>,
    pub bootnodes: String,
    pub registration_mode: bool,
    pub bridge_socket_path: Option<PathBuf>,
}

impl BlueprintEnvVars {
    #[must_use]
    pub fn new(
        env: &BlueprintEnvironment,
        manager_config: &BlueprintManagerConfig,
        blueprint_id: u64,
        service_id: u64,
        blueprint: &FilteredBlueprint,
        sub_service_str: &str,
    ) -> BlueprintEnvVars {
        let data_dir = manager_config
            .data_dir()
            .join(format!("blueprint-{blueprint_id}-{sub_service_str}"));

        let bootnodes = env
            .bootnodes
            .iter()
            .fold(String::new(), |acc, bootnode| format!("{acc} {bootnode}"));

        BlueprintEnvVars {
            http_rpc_endpoint: env.http_rpc_endpoint.clone(),
            ws_rpc_endpoint: env.ws_rpc_endpoint.clone(),
            #[cfg(feature = "tee")]
            kms_endpoint: env.kms_url.clone(),
            keystore_uri: env.keystore_uri.to_string(),
            data_dir,
            blueprint_id,
            service_id,
            protocol: blueprint.protocol,
            chain: None,
            bootnodes,
            registration_mode: blueprint.registration_mode,
            bridge_socket_path: env.bridge_socket_path.clone(),
        }
    }

    #[must_use]
    pub fn encode(&self) -> Vec<(String, String)> {
        let BlueprintEnvVars {
            http_rpc_endpoint,
            ws_rpc_endpoint,
            #[cfg(feature = "tee")]
            kms_endpoint,
            keystore_uri,
            data_dir,
            blueprint_id,
            service_id,
            protocol,
            chain,
            bootnodes,
            registration_mode,
            bridge_socket_path,
        } = self;

        let chain = chain.unwrap_or_else(|| match http_rpc_endpoint.as_str() {
            url if url.contains("127.0.0.1") || url.contains("localhost") => {
                SupportedChains::LocalTestnet
            }
            _ => SupportedChains::Testnet,
        });

        // Add required env vars for all child processes/blueprints
        let mut env_vars = vec![
            ("HTTP_RPC_URL".to_string(), http_rpc_endpoint.to_string()),
            ("WS_RPC_URL".to_string(), ws_rpc_endpoint.to_string()),
            #[cfg(feature = "tee")]
            ("KMS_URL".to_string(), kms_endpoint.to_string()),
            ("KEYSTORE_URI".to_string(), keystore_uri.clone()),
            ("DATA_DIR".to_string(), data_dir.display().to_string()),
            ("BLUEPRINT_ID".to_string(), blueprint_id.to_string()),
            ("SERVICE_ID".to_string(), service_id.to_string()),
            ("PROTOCOL".to_string(), protocol.to_string()),
            ("CHAIN".to_string(), chain.to_string()),
            ("BOOTNODES".to_string(), bootnodes.clone()),
        ];

        if let Some(bridge_socket_path) = bridge_socket_path {
            env_vars.push((
                "BRIDGE_SOCKET_PATH".to_string(),
                bridge_socket_path.display().to_string(),
            ));
        }

        if *registration_mode {
            env_vars.push((
                "REGISTRATION_MODE_ON".to_string(),
                registration_mode.to_string(),
            ));
        }

        env_vars
    }
}
