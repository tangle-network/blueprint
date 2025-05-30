use crate::blueprint::native::FilteredBlueprint;
use crate::config::BlueprintManagerConfig;
use blueprint_runner::config::{BlueprintEnvironment, Protocol, SupportedChains};
use std::path::{Path, PathBuf};

pub mod github;
pub mod testing;

#[auto_impl::auto_impl(Box)]
#[dynosaur::dynosaur(pub(crate) DynBlueprintSource)]
pub trait BlueprintSourceHandler: Send + Sync {
    fn fetch(
        &mut self,
        cache_dir: &Path,
    ) -> impl Future<Output = crate::error::Result<PathBuf>> + Send;
    fn blueprint_id(&self) -> u64;
    fn name(&self) -> String;
}

unsafe impl Send for DynBlueprintSource<'_> {}
unsafe impl Sync for DynBlueprintSource<'_> {}

pub struct BlueprintArgs {
    pub test_mode: bool,
    pub pretty: bool,
    pub verbose: u8,
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
        }
    }

    #[must_use]
    pub fn encode(&self) -> Vec<String> {
        let mut arguments = vec![];
        arguments.push("run".to_string());

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

        arguments
    }
}

pub struct BlueprintEnvVars {
    pub http_rpc_endpoint: String,
    pub ws_rpc_endpoint: String,
    pub keystore_uri: String,
    pub data_dir: PathBuf,
    pub blueprint_id: u64,
    pub service_id: u64,
    pub protocol: Protocol,
    pub bootnodes: String,
    pub registration_mode: bool,
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
        let base_data_dir = &manager_config.data_dir;
        let data_dir = base_data_dir.join(format!("blueprint-{blueprint_id}-{sub_service_str}"));

        let bootnodes = env
            .bootnodes
            .iter()
            .fold(String::new(), |acc, bootnode| format!("{acc} {bootnode}"));

        BlueprintEnvVars {
            http_rpc_endpoint: env.http_rpc_endpoint.to_string(),
            ws_rpc_endpoint: env.ws_rpc_endpoint.to_string(),
            keystore_uri: env.keystore_uri.to_string(),
            data_dir,
            blueprint_id,
            service_id,
            protocol: blueprint.protocol,
            bootnodes,
            registration_mode: blueprint.registration_mode,
        }
    }

    #[must_use]
    pub fn encode(&self) -> Vec<(String, String)> {
        let chain = match self.http_rpc_endpoint.as_str() {
            url if url.contains("127.0.0.1") || url.contains("localhost") => {
                SupportedChains::LocalTestnet
            }
            _ => SupportedChains::Testnet,
        };

        // Add required env vars for all child processes/blueprints
        let env_vars = vec![
            (
                "HTTP_RPC_URL".to_string(),
                self.http_rpc_endpoint.to_string(),
            ),
            ("WS_RPC_URL".to_string(), self.ws_rpc_endpoint.clone()),
            ("KEYSTORE_URI".to_string(), self.keystore_uri.clone()),
            ("BLUEPRINT_ID".to_string(), self.blueprint_id.to_string()),
            ("SERVICE_ID".to_string(), self.service_id.to_string()),
            ("PROTOCOL".to_string(), self.protocol.to_string()),
            ("CHAIN".to_string(), chain.to_string()),
            ("BOOTNODES".to_string(), self.bootnodes.clone()),
        ];

        env_vars
    }
}
