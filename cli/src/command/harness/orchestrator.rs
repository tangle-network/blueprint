use crate::command::harness::config::HarnessConfig;
use crate::command::run::tangle::run_blueprint;
use crate::command::tangle::{DevnetStack, SpawnMethod, run_opts_from_stack};
use blueprint_runner::tangle::config::TangleProtocolSettings;
use color_eyre::eyre::{Result, eyre};
use tokio::task::JoinHandle;

pub struct Orchestrator {
    stack: DevnetStack,
    handles: Vec<BlueprintTask>,
}

pub struct BlueprintTask {
    pub name: String,
    pub handle: JoinHandle<Result<()>>,
}

impl Orchestrator {
    /// Bring up the devnet stack (anvil + contracts + keystore).
    pub async fn bootstrap(config: &HarnessConfig) -> Result<Self> {
        if !config.chain.anvil {
            return Err(eyre!(
                "remote RPC mode not yet supported — set chain.anvil = true"
            ));
        }

        println!("Starting local Tangle devnet (anvil + contracts)...");
        let stack = DevnetStack::spawn(config.chain.include_anvil_logs).await?;
        println!("  HTTP RPC:  {}", stack.http_rpc_url());
        println!("  WS RPC:    {}", stack.ws_rpc_url());
        println!("  Tangle:    {}", stack.tangle_contract());
        println!();

        Ok(Self {
            stack,
            handles: Vec::new(),
        })
    }

    /// Spawn blueprint-manager for each configured blueprint.
    /// Each blueprint runs as an independent tokio task against the shared devnet stack.
    pub async fn spawn_blueprints(&mut self, config: &HarnessConfig) -> Result<()> {
        // Inject all blueprint env vars up front so child processes spawned by
        // blueprint-manager inherit them. Conflicting keys across blueprints are
        // unsupported in this MVP — last-write wins.
        for bp in &config.blueprints {
            for (k, v) in &bp.env {
                // Safety: env mutation happens before any tasks are spawned.
                unsafe { std::env::set_var(k, v) };
            }
        }

        for (idx, bp) in config.blueprints.iter().enumerate() {
            println!(
                "[{}] Starting blueprint-manager for '{}' at {}",
                idx + 1,
                bp.name,
                bp.path.display()
            );

            // For the MVP every blueprint reuses the harness default service id.
            // A future revision will mint a service per blueprint via on-chain RFQ.
            let settings = TangleProtocolSettings {
                blueprint_id: 0,
                service_id: Some(self.stack.default_service_id()),
                tangle_contract: self.stack.tangle_contract(),
                restaking_contract: self.stack.restaking_contract(),
                status_registry_contract: self.stack.status_registry_contract(),
            };

            let mut run_opts =
                run_opts_from_stack(&self.stack, &settings, false, SpawnMethod::Native);
            run_opts.shutdown_after = None;

            let name = bp.name.clone();
            let handle = tokio::spawn(async move {
                let result = run_blueprint(run_opts).await;
                if let Err(e) = &result {
                    eprintln!("[{name}] blueprint-manager exited with error: {e}");
                }
                result
            });

            self.handles.push(BlueprintTask {
                name: bp.name.clone(),
                handle,
            });
        }

        Ok(())
    }

    /// Wait for SIGTERM/SIGINT, then shut down all blueprints gracefully.
    pub async fn run_until_shutdown(self) -> Result<()> {
        println!();
        println!("Harness up. {} blueprint(s) running.", self.handles.len());
        println!("Press Ctrl+C to stop.");
        println!();

        tokio::signal::ctrl_c()
            .await
            .map_err(|e| eyre!("failed to listen for Ctrl-C: {e}"))?;

        println!();
        println!("Shutdown signal received, stopping blueprints...");

        for task in &self.handles {
            task.handle.abort();
        }

        for task in self.handles {
            let _ = tokio::time::timeout(std::time::Duration::from_secs(5), task.handle).await;
        }

        self.stack.shutdown().await;

        println!("Harness stopped.");
        Ok(())
    }
}
