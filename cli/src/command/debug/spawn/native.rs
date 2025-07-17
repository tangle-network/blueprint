use blueprint_manager::blueprint_auth::db::RocksDb;
use blueprint_manager::config::BlueprintManagerContext;
use blueprint_manager::rt::service::Service;
use blueprint_manager::sources::{BlueprintArgs, BlueprintEnvVars};
use std::path::PathBuf;

pub fn setup_native(
    ctx: BlueprintManagerContext,
    service_name: &str,
    binary: PathBuf,
    db: RocksDb,
    env: BlueprintEnvVars,
    args: BlueprintArgs,
) -> color_eyre::Result<Service> {
    let service = Service::new_native(db, ctx.runtime_dir(), service_name, binary, env, args)?;
    Ok(service)
}
