use blueprint_manager::config::BlueprintManagerContext;
use blueprint_manager::rt::ResourceLimits;
use blueprint_manager::rt::service::Service;
use blueprint_manager::sources::{BlueprintArgs, BlueprintEnvVars};
use std::path::PathBuf;

pub async fn setup_native(
    ctx: &BlueprintManagerContext,
    limits: ResourceLimits,
    service_name: &str,
    binary: PathBuf,
    env: BlueprintEnvVars,
    args: BlueprintArgs,
) -> color_eyre::Result<Service> {
    let service = Service::new_native(
        ctx,
        limits,
        ctx.runtime_dir(),
        service_name,
        binary,
        env,
        args,
    )
    .await?;
    Ok(service)
}
