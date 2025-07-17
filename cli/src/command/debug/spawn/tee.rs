use blueprint_manager::blueprint_auth::db::RocksDb;
use blueprint_manager::config::BlueprintManagerContext;
use blueprint_manager::rt::ResourceLimits;
use blueprint_manager::rt::service::Service;
use blueprint_manager::sources::{BlueprintArgs, BlueprintEnvVars};

pub async fn setup_with_tee(
    ctx: &BlueprintManagerContext,
    service_name: &str,
    image: String,
    db: RocksDb,
    env: BlueprintEnvVars,
    args: BlueprintArgs,
) -> color_eyre::Result<Service> {
    let service = Service::new_tee(
        ctx,
        ResourceLimits::default(),
        db,
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
