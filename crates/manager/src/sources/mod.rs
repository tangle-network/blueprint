use crate::blueprint::native::FilteredBlueprint;
use crate::config::{BlueprintManagerConfig, BlueprintManagerContext};
use crate::rt::ResourceLimits;
use crate::rt::service::Service;
use alloy_primitives::Address;
use blueprint_runner::config::{BlueprintEnvironment, Protocol, SupportedChains};
use std::path::{Path, PathBuf};
use url::Url;

#[cfg(feature = "containers")]
pub mod container;
pub mod github;
pub mod remote;
pub mod testing;
pub mod types;

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

#[derive(Clone)]
pub struct BlueprintArgs {
    pub test_mode: bool,
    pub pretty: bool,
    pub verbose: u8,
    pub dry_run: bool,
    /// Protocol-specific extra arguments (e.g., EigenLayer contract addresses)
    pub extra_args: Vec<(String, String)>,
}

impl BlueprintArgs {
    #[must_use]
    pub fn new(manager_config: &BlueprintManagerConfig) -> Self {
        if manager_config.test_mode {
            blueprint_core::warn!("Test mode is enabled");
        }

        // TODO: Add support for keystore password
        // if let Some(keystore_password) = &env.keystore_password {
        //     arguments.push(format!("--keystore-password={}", keystore_password));
        // }

        Self {
            test_mode: manager_config.test_mode,
            pretty: manager_config.pretty,
            verbose: manager_config.verbose,
            dry_run: false,
            extra_args: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
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

        if self.dry_run {
            arguments.push("--dry-run".to_string());
        }

        // Add protocol-specific extra arguments (e.g., EigenLayer contract addresses)
        for (key, value) in &self.extra_args {
            arguments.push(key.clone());
            arguments.push(value.clone());
        }

        arguments
    }
}

#[derive(Clone)]
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
    pub registration_capture_only: bool,
    pub bridge_socket_path: Option<PathBuf>,
    /// Tangle EVM contract addresses (only set for Tangle protocol)
    pub tangle_contract: Option<Address>,
    pub restaking_contract: Option<Address>,
    pub status_registry_contract: Option<Address>,
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

        // Extract contract addresses from protocol settings if using Tangle
        let (tangle_contract, restaking_contract, status_registry_contract) =
            if let Ok(settings) = env.protocol_settings.tangle() {
                (
                    Some(settings.tangle_contract),
                    Some(settings.restaking_contract),
                    Some(settings.status_registry_contract),
                )
            } else {
                (None, None, None)
            };

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
            registration_capture_only: blueprint.registration_capture_only,
            bridge_socket_path: env.bridge_socket_path.clone(),
            tangle_contract,
            restaking_contract,
            status_registry_contract,
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
            registration_capture_only,
            bridge_socket_path,
            tangle_contract,
            restaking_contract,
            status_registry_contract,
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

        // Add Tangle contract addresses if present
        if let Some(addr) = tangle_contract {
            env_vars.push(("TANGLE_CONTRACT".to_string(), format!("{addr}")));
        }
        if let Some(addr) = restaking_contract {
            env_vars.push(("RESTAKING_CONTRACT".to_string(), format!("{addr}")));
        }
        if let Some(addr) = status_registry_contract {
            env_vars.push(("STATUS_REGISTRY_CONTRACT".to_string(), format!("{addr}")));
        }

        if let Some(bridge_socket_path) = bridge_socket_path {
            env_vars.push((
                "BRIDGE_SOCKET_PATH".to_string(),
                bridge_socket_path.display().to_string(),
            ));
        }

        if *registration_mode {
            let registration_path = data_dir.join("registration_inputs.bin");
            env_vars.push((
                "REGISTRATION_MODE_ON".to_string(),
                registration_mode.to_string(),
            ));
            env_vars.push((
                "REGISTRATION_OUTPUT_PATH".to_string(),
                registration_path.display().to_string(),
            ));
            if *registration_capture_only {
                env_vars.push((
                    "REGISTRATION_CAPTURE_ONLY".to_string(),
                    registration_capture_only.to_string(),
                ));
            }
        }

        env_vars
    }
}
