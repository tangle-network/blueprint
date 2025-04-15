use crate::config::BlueprintManagerConfig;
use crate::gadget::native::FilteredBlueprint;
use blueprint_runner::config::{BlueprintEnvironment, SupportedChains};
use tokio::sync::mpsc::UnboundedReceiver;

pub mod binary;
pub mod container;
pub mod github;
pub mod testing;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Running,
    Finished,
    Error,
}

pub struct ProcessHandle {
    status: UnboundedReceiver<Status>,
    cached_status: Status,
    abort_handle: tokio::sync::oneshot::Sender<()>,
}

impl ProcessHandle {
    #[must_use]
    pub fn new(
        mut status: UnboundedReceiver<Status>,
        abort_handle: tokio::sync::oneshot::Sender<()>,
    ) -> Self {
        let cached_status = status.try_recv().ok().unwrap_or(Status::Running);
        Self {
            status,
            cached_status,
            abort_handle,
        }
    }

    pub fn status(&mut self) -> Status {
        self.status.try_recv().ok().unwrap_or(self.cached_status)
    }

    pub async fn wait_for_status_change(&mut self) -> Option<Status> {
        self.status.recv().await
    }

    #[must_use]
    pub fn abort(self) -> bool {
        self.abort_handle.send(()).is_ok()
    }
}

#[auto_impl::auto_impl(Box)]
#[dynosaur::dynosaur(pub(crate) DynBlueprintSource)]
pub trait BlueprintSourceHandler: Send + Sync {
    fn fetch(&mut self) -> impl Future<Output = crate::error::Result<()>> + Send;
    fn spawn(
        &mut self,
        env: &BlueprintEnvironment,
        service: &str,
        args: Vec<String>,
        env_vars: Vec<(String, String)>,
    ) -> impl Future<Output = crate::error::Result<ProcessHandle>> + Send;
    fn blueprint_id(&self) -> u64;
    fn name(&self) -> String;
}

unsafe impl Send for DynBlueprintSource<'_> {}
unsafe impl Sync for DynBlueprintSource<'_> {}

#[must_use]
pub fn process_arguments_and_env(
    env: &BlueprintEnvironment,
    manager_config: &BlueprintManagerConfig,
    blueprint_id: u64,
    service_id: u64,
    blueprint: &FilteredBlueprint,
    sub_service_str: &str,
) -> (Vec<String>, Vec<(String, String)>) {
    let mut arguments = vec![];
    arguments.push("run".to_string());

    if manager_config.test_mode {
        arguments.push("--test-mode".to_string());
    }

    if manager_config.pretty {
        arguments.push("--pretty".to_string());
    }

    if manager_config.test_mode {
        blueprint_core::warn!("Test mode is enabled");
    }

    // TODO: Add support for keystore password
    // if let Some(keystore_password) = &env.keystore_password {
    //     arguments.push(format!("--keystore-password={}", keystore_password));
    // }

    // Uses occurrences of clap short -v
    if manager_config.verbose > 0 {
        arguments.push(format!("-{}", "v".repeat(manager_config.verbose as usize)));
    }

    let chain = match env.http_rpc_endpoint.as_str() {
        url if url.contains("127.0.0.1") || url.contains("localhost") => {
            SupportedChains::LocalTestnet
        }
        _ => SupportedChains::Testnet,
    };

    let bootnodes = env
        .bootnodes
        .iter()
        .fold(String::new(), |acc, bootnode| format!("{acc} {bootnode}"));

    // Add required env vars for all child processes/gadgets
    let mut env_vars = vec![
        (
            "HTTP_RPC_URL".to_string(),
            env.http_rpc_endpoint.to_string(),
        ),
        ("WS_RPC_URL".to_string(), env.ws_rpc_endpoint.to_string()),
        ("KEYSTORE_URI".to_string(), env.keystore_uri.clone()),
        ("BLUEPRINT_ID".to_string(), format!("{}", blueprint_id)),
        ("SERVICE_ID".to_string(), format!("{}", service_id)),
        ("PROTOCOL".to_string(), blueprint.protocol.to_string()),
        ("CHAIN".to_string(), chain.to_string()),
        ("BOOTNODES".to_string(), bootnodes),
    ];

    let base_data_dir = &manager_config.data_dir;
    let data_dir = base_data_dir.join(format!("blueprint-{blueprint_id}-{sub_service_str}"));
    env_vars.push((
        "DATA_DIR".to_string(),
        data_dir.to_string_lossy().into_owned(),
    ));

    // Ensure our child process inherits the current processes' environment vars
    env_vars.extend(std::env::vars());

    if blueprint.registration_mode {
        env_vars.push(("REGISTRATION_MODE_ON".to_string(), "true".to_string()));
    }

    (arguments, env_vars)
}
