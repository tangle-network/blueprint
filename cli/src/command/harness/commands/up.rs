use crate::command::harness::UpArgs;
use crate::command::harness::config::HarnessConfig;
use crate::command::harness::orchestrator::Orchestrator;
use color_eyre::eyre::{Result, eyre};

pub async fn run(args: UpArgs) -> Result<()> {
    let mut config = HarnessConfig::load(args.config.as_deref())?;
    config.filter(args.only.as_deref());

    if args.include_anvil_logs {
        config.chain.include_anvil_logs = true;
    }

    if config.blueprints.is_empty() {
        return Err(eyre!(
            "no blueprints configured (or all filtered out by --only)"
        ));
    }

    let mut orchestrator = Orchestrator::bootstrap(&config).await?;
    orchestrator.spawn_blueprints(&config).await?;
    orchestrator.run_until_shutdown().await?;

    Ok(())
}
