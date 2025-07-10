use blueprint_manager::blueprint_auth::db::RocksDb;
use blueprint_manager::config::BlueprintManagerConfig;
use blueprint_manager::rt::ResourceLimits;
use blueprint_manager::rt::service::Service;
use blueprint_manager::sources::{BlueprintArgs, BlueprintEnvVars};

pub async fn setup_with_tee(
    manager_config: BlueprintManagerConfig,
    service_name: &str,
    image: String,
    db: RocksDb,
    env: BlueprintEnvVars,
    args: BlueprintArgs,
) -> color_eyre::Result<Service> {
    let kube_client = manager_config.kube_client().await?;

    let service = Service::new_tee(
        kube_client,
        ResourceLimits::default(),
        db,
        manager_config.runtime_dir(),
        service_name,
        image,
        env,
        args,
    )?;

    Ok(service)
}
