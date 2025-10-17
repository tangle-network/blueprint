/// EigenLayer Protocol Event Handler
///
/// Handles EigenLayer-specific events including task creation, operator registration, and response submission.
///
/// # Key Architectural Differences from Tangle
///
/// - **No Multi-Instance Management**: EigenLayer blueprints ARE instances (not templates)
/// - **Task-Based Events**: Processes TaskCreated, TaskResponded events from EVM logs
/// - **Single Blueprint Process**: One blueprint binary handles all tasks
/// - **No Service Registration**: No PreRegistration/ServiceInitiated flow

use crate::blueprint::ActiveBlueprints;
use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::protocol::eigenlayer::client::EigenlayerProtocolClient;
use crate::protocol::types::ProtocolEvent;
use crate::rt::ResourceLimits;
use crate::rt::service::Status;
use crate::sources::{BlueprintArgs, BlueprintEnvVars, BlueprintSourceHandler, DynBlueprintSource};
use crate::sources::testing::TestSourceFetcher;
use blueprint_core::{error, info};
use blueprint_runner::config::{BlueprintEnvironment, Protocol};
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::field::BoundedString;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::BlueprintSource;

/// Helper function to create a BoundedString from any type that can be converted to String
fn new_bounded_string<S: Into<String>>(s: S) -> BoundedString {
    let s = s.into();
    BoundedString(BoundedVec(s.into_bytes()))
}

/// EigenLayer protocol event handler implementation
///
/// Unlike Tangle's multi-instance model, EigenLayer blueprints are single-instance.
/// The handler simply ensures the blueprint binary is running and lets it handle tasks internally.
pub struct EigenlayerEventHandler;

impl EigenlayerEventHandler {
    /// Create a new EigenLayer event handler
    pub fn new() -> Self {
        Self
    }

    /// Start the blueprint binary if not already running
    ///
    /// EigenLayer blueprints are single-instance - we just ensure one binary is running.
    async fn ensure_blueprint_running(
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        // For EigenLayer, we use a special blueprint_id of 0 since there's no on-chain registration
        const EIGENLAYER_BLUEPRINT_ID: u64 = 0;
        const EIGENLAYER_SERVICE_ID: u64 = 0;

        // Check if blueprint is already running
        if let Some(services) = active_blueprints.get_mut(&EIGENLAYER_BLUEPRINT_ID) {
            if let Some(handle) = services.get_mut(&EIGENLAYER_SERVICE_ID) {
                // Check if the process is still alive
                match handle.status().await {
                    Ok(Status::Running) => {
                        // Blueprint is running fine
                        return Ok(());
                    }
                    _ => {
                        info!("EigenLayer blueprint process died, will restart");
                    }
                }
            }
        }

        info!("Starting EigenLayer blueprint binary");

        // For EigenLayer, use a test fetcher pointing to the blueprint source directory
        // The directory path should be provided via EIGENLAYER_BLUEPRINT_PATH environment variable
        // This should point to the examples/incredible-squaring-eigenlayer directory
        let blueprint_dir = std::env::var("EIGENLAYER_BLUEPRINT_PATH")
            .unwrap_or_else(|_| {
                // Fall back to examples/incredible-squaring-eigenlayer relative to workspace root
                let workspace_root = std::env::var("CARGO_MANIFEST_DIR")
                    .unwrap_or_else(|_| ".".to_string());
                std::path::PathBuf::from(workspace_root)
                    .join("../../examples/incredible-squaring-eigenlayer")
                    .to_string_lossy()
                    .to_string()
            });

        info!("Using EigenLayer blueprint directory at: {}", blueprint_dir);

        // Create a test fetcher for the blueprint
        // TestSourceFetcher will run `cargo build` in the blueprint directory to produce the binary
        use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::TestFetcher;
        let test_fetcher = TestFetcher {
            cargo_package: new_bounded_string("incredible-squaring-blueprint-eigenlayer"),
            cargo_bin: new_bounded_string("incredible-squaring-blueprint-eigenlayer"),
            base_path: new_bounded_string(blueprint_dir.clone()),
        };

        let mut fetcher: Box<DynBlueprintSource<'static>> = {
            let fetcher = TestSourceFetcher::new(
                test_fetcher.clone(),
                EIGENLAYER_BLUEPRINT_ID,
                "incredible-squaring-blueprint-eigenlayer".to_string(),
            );
            DynBlueprintSource::boxed(fetcher)
        };

        // Create cache directory for the blueprint
        let cache_dir = ctx
            .cache_dir()
            .join(format!("{}-incredible-squaring-blueprint-eigenlayer", EIGENLAYER_BLUEPRINT_ID));

        if let Err(e) = std::fs::create_dir_all(&cache_dir) {
            error!(
                "Failed to create cache directory for EigenLayer blueprint at {}",
                cache_dir.display()
            );
            return Err(e.into());
        }

        // Fetch the blueprint binary
        fetcher.fetch(&cache_dir).await.map_err(|e| {
            error!("Failed to fetch EigenLayer blueprint binary: {e}");
            e
        })?;

        // Create runtime directory
        let id = active_blueprints.len() as u32;
        let runtime_dir = ctx.runtime_dir().join(id.to_string());
        std::fs::create_dir_all(&runtime_dir)?;

        // Prepare environment variables and arguments
        let service_str = fetcher.name();
        let mut args = BlueprintArgs::new(ctx);

        // Add EigenLayer contract addresses from protocol settings
        if let blueprint_runner::config::ProtocolSettings::Eigenlayer(eigenlayer_settings) = &env.protocol_settings {
            let eigenlayer_args = vec![
                ("--allocation-manager".to_string(), format!("{:?}", eigenlayer_settings.allocation_manager_address)),
                ("--registry-coordinator".to_string(), format!("{:?}", eigenlayer_settings.registry_coordinator_address)),
                ("--operator-state-retriever".to_string(), format!("{:?}", eigenlayer_settings.operator_state_retriever_address)),
                ("--delegation-manager".to_string(), format!("{:?}", eigenlayer_settings.delegation_manager_address)),
                ("--strategy-manager".to_string(), format!("{:?}", eigenlayer_settings.strategy_manager_address)),
                ("--service-manager".to_string(), format!("{:?}", eigenlayer_settings.service_manager_address)),
                ("--stake-registry".to_string(), format!("{:?}", eigenlayer_settings.stake_registry_address)),
                ("--avs-directory".to_string(), format!("{:?}", eigenlayer_settings.avs_directory_address)),
                ("--rewards-coordinator".to_string(), format!("{:?}", eigenlayer_settings.rewards_coordinator_address)),
                ("--permission-controller".to_string(), format!("{:?}", eigenlayer_settings.permission_controller_address)),
                ("--strategy".to_string(), format!("{:?}", eigenlayer_settings.strategy_address)),
            ];

            args.extra_args = eigenlayer_args;
        }

        // For EigenLayer, we don't have multiple services, so we pass minimal info
        let blueprint_env = BlueprintEnvVars::new(
            env,
            ctx,
            EIGENLAYER_BLUEPRINT_ID,
            EIGENLAYER_SERVICE_ID,
            &crate::blueprint::native::FilteredBlueprint {
                blueprint_id: EIGENLAYER_BLUEPRINT_ID,
                services: vec![EIGENLAYER_SERVICE_ID],
                sources: vec![BlueprintSource::Testing(test_fetcher.clone())],
                name: "incredible-squaring-blueprint-eigenlayer".to_string(),
                registration_mode: false,
                protocol: Protocol::Eigenlayer,
            },
            &service_str,
        );

        info!("Spawning EigenLayer blueprint process: {service_str}");

        // Configure resource limits
        let limits = ResourceLimits::default();

        // Spawn the blueprint process
        let mut service = fetcher
            .spawn(
                ctx,
                limits,
                env,
                id,
                blueprint_env,
                args,
                &service_str,
                &cache_dir,
                &runtime_dir,
            )
            .await?;

        // Start the service
        let service_start_res = service.start().await;
        match service_start_res {
            Ok(Some(is_alive)) => {
                info!("EigenLayer blueprint started, waiting for health check");
                is_alive.await?;
                info!("EigenLayer blueprint is alive and healthy");

                // Add to active blueprints
                active_blueprints
                    .entry(EIGENLAYER_BLUEPRINT_ID)
                    .or_default()
                    .insert(EIGENLAYER_SERVICE_ID, service);
            }
            Ok(None) => {
                info!("EigenLayer blueprint started (no health check)");
                active_blueprints
                    .entry(EIGENLAYER_BLUEPRINT_ID)
                    .or_default()
                    .insert(EIGENLAYER_SERVICE_ID, service);
            }
            Err(e) => {
                error!("EigenLayer blueprint did not start successfully: {e}");
                service.shutdown().await?;
                return Err(e);
            }
        }

        Ok(())
    }
}

impl Default for EigenlayerEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EigenlayerEventHandler {
    /// Initialize the handler with the protocol client
    pub async fn initialize(
        &mut self,
        _client: &EigenlayerProtocolClient,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        info!("Initializing EigenLayer protocol handler");

        // Start the blueprint binary
        Self::ensure_blueprint_running(env, ctx, active_blueprints).await?;

        info!("EigenLayer protocol handler initialized");
        Ok(())
    }

    /// Handle an EigenLayer protocol event
    pub async fn handle_event(
        &mut self,
        event: &ProtocolEvent,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        let eigenlayer_event = event.as_eigenlayer().ok_or_else(|| {
            Error::Other("Expected EigenLayer event in EigenLayer handler".to_string())
        })?;

        info!(
            "Processing EigenLayer event at block {} with {} logs",
            eigenlayer_event.block_number,
            eigenlayer_event.logs.len()
        );

        // Ensure the blueprint is still running
        Self::ensure_blueprint_running(env, ctx, active_blueprints).await?;

        // The blueprint binary itself will process the events via its job handlers
        // (e.g., initialize_bls_task job listening for NewTaskCreated events)
        //
        // We don't need to explicitly route events here like Tangle does,
        // because the blueprint's jobs are already watching for their specific events.
        //
        // Our only responsibility is to keep the blueprint binary alive.

        Ok(())
    }
}
