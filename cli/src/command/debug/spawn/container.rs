use blueprint_manager::config::BlueprintManagerContext;
use blueprint_manager::rt::ResourceLimits;
use blueprint_manager::rt::service::Service;
use blueprint_manager::sources::{BlueprintArgs, BlueprintEnvVars};

pub async fn setup_with_container(
    ctx: &BlueprintManagerContext,
    limits: ResourceLimits,
    service_name: &str,
    image: String,
    env: BlueprintEnvVars,
    args: BlueprintArgs,
) -> color_eyre::Result<Service> {
    let service = Service::new_container(
        ctx,
        limits,
        ctx.runtime_dir(),
        service_name,
        image,
        env,
        args,
        true,
    )
    .await?;

    Ok(service)
}
