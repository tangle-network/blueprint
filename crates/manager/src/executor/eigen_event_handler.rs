use crate::config::BlueprintManagerContext;
use crate::error::Result;
use crate::rt::ResourceLimits;
use crate::executor::EigenlayerClient;
use crate::sources::{BlueprintArgs, BlueprintEnvVars, DynBlueprintSource};
use crate::sources::testing::TestSourceFetcher;
use crate::sources::BlueprintSourceHandler;
use blueprint_tangle_extra::metadata::types::sources::TestFetcher;
use blueprint_std::borrow::Cow;
use alloy_rpc_types::Log;
use blueprint_runner::config::{BlueprintEnvironment, Protocol};
use blueprint_core::{info, error, trace};
use blueprint_std::fs;
use blueprint_std::path::{Path, PathBuf};
use color_eyre::eyre;
use tokio::signal;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;

const DEFAULT_PROTOCOL: Protocol = Protocol::Eigenlayer;

pub async fn handle_init(
    eigenlayer_client: &EigenlayerClient,
    config: &BlueprintEnvironment,
    ctx: &BlueprintManagerContext
) -> Result<()> {
    info!("Beginning initialization of Blueprint Manager");

    let binary_path = ctx.eigen_blueprint_binary_path().unwrap();
    let (blueprint_id, service_id, service_vm_id, blueprint_name) = get_blueprint_metadata(binary_path);
    // create directories
    let cache_dir = ctx.cache_dir().join(format!(
        "{blueprint_name}-{blueprint_id}",
    ));
    fs::create_dir_all(&cache_dir)?;
    let runtime_dir = ctx.runtime_dir().join(blueprint_id.to_string());
    fs::create_dir_all(&runtime_dir)?;
    let sub_service_str = format!("{blueprint_name}-service-{service_id}");

    // fetch source
    let mut fetcher = get_fetcher(blueprint_id.clone(), blueprint_name);
    if let Err(e) = fetcher.fetch(&cache_dir).await {
        error!("Failed to fetch blueprint from source: {e}");
        return Err(e.into());
    }

    // TODO: Actually configure resource limits
    let limits = ResourceLimits::default();

    // spawn service
    let args = BlueprintArgs::new(ctx);
    let env = BlueprintEnvVars {
        http_rpc_endpoint: config.http_rpc_endpoint.clone(),
        ws_rpc_endpoint: config.ws_rpc_endpoint.clone(),
        #[cfg(feature = "tee")]
        kms_endpoint: config.kms_url.clone(),
        keystore_uri: config.keystore_uri.clone(),
        data_dir: config.data_dir.clone(),
        blueprint_id: blueprint_id.into(),
        service_id: service_id.into(),
        protocol: DEFAULT_PROTOCOL,
        chain: ctx.runtime_chain(),
        bootnodes: String::new(),
        // update via spawn > new_native
        bridge_socket_path: config.bridge_socket_path.clone(),
        registration_mode: false,
    };
    let mut service = fetcher
        .spawn(
            ctx,
            limits,
            config,
            service_vm_id,
            env,
            args,
            &sub_service_str,
            &cache_dir,
            &runtime_dir,
        )
        .await?;

    let service_start_res = service.start().await;
    match service_start_res {
        Ok(Some(is_alive)) => {
            is_alive.await?;
        }
        Ok(None) => {}
        Err(e) => {
            error!("Service did not start successfully, aborting: {e}");
            service.shutdown().await?;
        }
    };

    Ok(())
}

fn get_blueprint_metadata(binary_path: &Path) -> (u64, u64, u32, String) {
    let default_blueprint_id: u64 = 0;
    let default_service_id: u64 = 0;
    let default_service_vm_id: u32 = 0;
    
    let bin_name = binary_path.file_name().unwrap().to_str().unwrap();
    (default_blueprint_id, default_service_id, default_service_vm_id, bin_name.to_string())
}

fn get_fetcher(blueprint_id: u64, binary_name: String) -> Box<DynBlueprintSource<'static>> {
    let fetcher = TestSourceFetcher::new(
        TestFetcher {
            cargo_package: Cow::Borrowed(&binary_name.clone()),
            cargo_bin: Cow::Borrowed(&binary_name.clone()),
            // only case that in development, we are in the root of the repo
            base_path: Cow::Borrowed("./"),
        }.into(),
        blueprint_id,
        binary_name,
    );
    DynBlueprintSource::boxed(fetcher)
}

// TODO(daniel): Implement this
pub(crate) async fn handle_eigen_event(
    event: &Log,
    _blueprint_config: &BlueprintEnvironment,
    _ctx: &BlueprintManagerContext,
) -> Result<()> {
    trace!("Received notification {:?}", event);

    Ok(())
}