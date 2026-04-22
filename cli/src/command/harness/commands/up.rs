use crate::command::harness::UpArgs;
use crate::command::harness::config::HarnessConfig;
use crate::command::harness::orchestrator::Orchestrator;
use color_eyre::eyre::{Result, eyre};
use std::path::PathBuf;

pub async fn run(args: UpArgs) -> Result<()> {
    let mut config = if let Some(compose) = &args.compose {
        // Compose mode: merge harness.toml from multiple blueprint directories
        let paths: Vec<PathBuf> = compose
            .split(',')
            .map(|s| {
                let s = s.trim();
                if let Some(rest) = s.strip_prefix("~/") {
                    let home = std::env::var("HOME").unwrap_or_default();
                    PathBuf::from(home).join(rest)
                } else {
                    PathBuf::from(s)
                }
            })
            .collect();
        HarnessConfig::compose(&paths)?
    } else {
        HarnessConfig::load(args.config.as_deref())?
    };

    config.apply_chain_overrides(&args.chain);
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
