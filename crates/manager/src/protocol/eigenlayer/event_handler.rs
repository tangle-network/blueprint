/// EigenLayer Protocol Event Handler
///
/// Handles EigenLayer-specific events including task creation, operator registration, and response submission.
///
/// # Multi-AVS Architecture
///
/// - **Multi-AVS Support**: Spawns separate blueprint instances for each registered AVS
/// - **Registration-Based**: Reads AVS registrations from `~/.tangle/eigenlayer_registrations.json`
/// - **Unique Blueprint IDs**: Derives blueprint_id from service_manager address
/// - **Task-Based Events**: Each AVS blueprint processes TaskCreated events from EVM logs
/// - **No Service Registration Flow**: Uses CLI-based registration/deregistration

use crate::blueprint::ActiveBlueprints;
use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::protocol::eigenlayer::client::EigenlayerProtocolClient;
use crate::protocol::eigenlayer::{RegistrationStateManager, RegistrationStatus};
use crate::protocol::types::ProtocolEvent;
use crate::rt::ResourceLimits;
use crate::rt::service::Status;
use crate::sources::{BlueprintArgs, BlueprintEnvVars, BlueprintSourceHandler, DynBlueprintSource};
use crate::sources::testing::TestSourceFetcher;
use blueprint_core::{error, info, warn};
use blueprint_runner::config::{BlueprintEnvironment, Protocol};
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::field::BoundedString;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::BlueprintSource;

/// Helper function to create a BoundedString from any type that can be converted to String
fn new_bounded_string<S: Into<String>>(s: S) -> BoundedString {
    let s = s.into();
    BoundedString(BoundedVec(s.into_bytes()))
}

/// Read the package name from a Cargo.toml file in the given directory
///
/// This ensures we use the actual package name for `cargo build --bin`,
/// rather than deriving it from the directory name which may not match.
///
/// Returns an error if:
/// - Path is not a directory (e.g., it's a binary file)
/// - Cargo.toml doesn't exist
/// - Cargo.toml exists but package name can't be parsed
fn read_package_name_from_cargo_toml(blueprint_path: &std::path::Path) -> Result<String> {
    // If path is a file (binary), we can't read Cargo.toml
    if !blueprint_path.is_dir() {
        return Err(Error::Other(format!(
            "Path is not a directory (binary?): {}",
            blueprint_path.display()
        )));
    }

    let cargo_toml_path = blueprint_path.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        return Err(Error::Other(format!(
            "Cargo.toml not found at: {}",
            cargo_toml_path.display()
        )));
    }

    let contents = std::fs::read_to_string(&cargo_toml_path)
        .map_err(|e| Error::Other(format!("Failed to read Cargo.toml: {}", e)))?;

    // Simple parse - look for [package]\nname = "..."
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with("name") && line.contains('=') {
            if let Some(name_part) = line.split('=').nth(1) {
                let name = name_part
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();
                if !name.is_empty() {
                    return Ok(name);
                }
            }
        }
    }

    Err(Error::Other(
        "Could not find package name in Cargo.toml".to_string(),
    ))
}

/// EigenLayer protocol event handler implementation
///
/// Supports multi-AVS architecture by spawning separate blueprint instances
/// for each registered AVS service.
pub struct EigenlayerEventHandler {
    /// Background service handles for operator-level tasks
    /// These run once per operator and monitor rewards, slashing, etc.
    background_services: Option<BackgroundServices>,
}

/// Background services for operator-level monitoring
struct BackgroundServices {
    #[allow(dead_code)] // Keep handles alive to prevent task cancellation
    rewards_task: tokio::task::JoinHandle<()>,
    #[allow(dead_code)] // Keep handles alive to prevent task cancellation
    slashing_task: tokio::task::JoinHandle<()>,
}

impl EigenlayerEventHandler {
    /// Create a new EigenLayer event handler
    pub fn new() -> Self {
        Self {
            background_services: None,
        }
    }

    /// Ensure all registered AVS blueprints are running
    ///
    /// Reads AVS registrations from the state file and spawns/monitors each one.
    async fn ensure_all_registered_avs_running(
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        // Load AVS registrations from state file
        let state_manager = RegistrationStateManager::load()
            .map_err(|e| Error::Other(format!("Failed to load AVS registrations: {}", e)))?;

        // Get all active registrations
        let active_registrations: Vec<_> = state_manager
            .registrations()
            .registrations
            .values()
            .filter(|reg| reg.status == RegistrationStatus::Active)
            .collect();

        if active_registrations.is_empty() {
            info!("No active AVS registrations found");
            return Ok(());
        }

        info!(
            "Found {} active AVS registration(s)",
            active_registrations.len()
        );

        // For each active registration, ensure the AVS blueprint is running
        for registration in active_registrations {
            let blueprint_id = registration.blueprint_id();
            let service_id = 0u64; // EigenLayer uses 1:1 mapping (one blueprint per AVS)

            // Check if already running
            if let Some(services) = active_blueprints.get_mut(&blueprint_id) {
                if let Some(handle) = services.get_mut(&service_id) {
                    match handle.status().await {
                        Ok(Status::Running) => {
                            info!(
                                "AVS {} (blueprint_id={}) is already running",
                                registration.config.service_manager, blueprint_id
                            );
                            continue; // Already running, skip
                        }
                        _ => {
                            info!(
                                "AVS {} (blueprint_id={}) process died, will restart",
                                registration.config.service_manager, blueprint_id
                            );
                        }
                    }
                }
            }

            // Spawn the AVS blueprint
            info!(
                "Starting AVS blueprint for service_manager {} (blueprint_id={})",
                registration.config.service_manager, blueprint_id
            );
            Self::spawn_avs_blueprint(env, ctx, active_blueprints, registration).await?;
        }

        Ok(())
    }

    /// Spawn a single AVS blueprint instance
    async fn spawn_avs_blueprint(
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
        registration: &blueprint_eigenlayer_extra::registration::AvsRegistration,
    ) -> Result<()> {
        let blueprint_id = registration.blueprint_id();
        let service_id = 0u64;

        info!(
            "Spawning AVS blueprint from path: {:?}",
            registration.config.blueprint_path
        );

        // Use the blueprint path from the registration config
        let blueprint_dir = registration
            .config
            .blueprint_path
            .to_string_lossy()
            .to_string();

        info!("Using AVS blueprint directory at: {}", blueprint_dir);

        // Read blueprint package name from Cargo.toml (not directory name)
        // This ensures binary name matches what `cargo build --bin` expects
        let blueprint_name = read_package_name_from_cargo_toml(&registration.config.blueprint_path)
            .unwrap_or_else(|_| {
                // Fallback to directory name if Cargo.toml can't be read
                warn!("Could not read package name from Cargo.toml, using directory name");
                registration
                    .config
                    .blueprint_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("eigenlayer-blueprint")
                    .to_string()
            });

        // Create a test fetcher for the blueprint
        // TestSourceFetcher will run `cargo build` in the blueprint directory to produce the binary
        use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::TestFetcher;
        let test_fetcher = TestFetcher {
            cargo_package: new_bounded_string(&blueprint_name),
            cargo_bin: new_bounded_string(&blueprint_name),
            base_path: new_bounded_string(blueprint_dir.clone()),
        };

        let mut fetcher: Box<DynBlueprintSource<'static>> = {
            let fetcher =
                TestSourceFetcher::new(test_fetcher.clone(), blueprint_id, blueprint_name.clone());
            DynBlueprintSource::boxed(fetcher)
        };

        // Create cache directory for the blueprint
        let cache_dir = ctx
            .cache_dir()
            .join(format!("{}-{}", blueprint_id, blueprint_name));

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

        // Add AVS-specific contract addresses from registration config
        let avs_config = &registration.config;
        let eigenlayer_args = vec![
            (
                "--allocation-manager".to_string(),
                format!(
                    "{:?}",
                    avs_config
                        .allocation_manager
                        .unwrap_or(avs_config.strategy_manager)
                ),
            ),
            (
                "--registry-coordinator".to_string(),
                format!("{:?}", avs_config.registry_coordinator),
            ),
            (
                "--operator-state-retriever".to_string(),
                format!("{:?}", avs_config.operator_state_retriever),
            ),
            (
                "--delegation-manager".to_string(),
                format!("{:?}", avs_config.delegation_manager),
            ),
            (
                "--strategy-manager".to_string(),
                format!("{:?}", avs_config.strategy_manager),
            ),
            (
                "--service-manager".to_string(),
                format!("{:?}", avs_config.service_manager),
            ),
            (
                "--stake-registry".to_string(),
                format!("{:?}", avs_config.stake_registry),
            ),
            (
                "--avs-directory".to_string(),
                format!("{:?}", avs_config.avs_directory),
            ),
            (
                "--rewards-coordinator".to_string(),
                format!("{:?}", avs_config.rewards_coordinator),
            ),
            (
                "--permission-controller".to_string(),
                format!(
                    "{:?}",
                    avs_config
                        .permission_controller
                        .unwrap_or(avs_config.delegation_manager)
                ),
            ),
            (
                "--strategy".to_string(),
                format!("{:?}", avs_config.strategy_address),
            ),
        ];

        args.extra_args = eigenlayer_args;

        // Create blueprint environment for this AVS
        let blueprint_env = BlueprintEnvVars::new(
            env,
            ctx,
            blueprint_id,
            service_id,
            &crate::blueprint::native::FilteredBlueprint {
                blueprint_id,
                services: vec![service_id],
                sources: vec![BlueprintSource::Testing(test_fetcher.clone())],
                name: blueprint_name.clone(),
                registration_mode: false,
                protocol: Protocol::Eigenlayer,
            },
            &service_str,
        );

        info!("Spawning AVS blueprint process: {service_str}");

        // Configure resource limits
        let limits = ResourceLimits::default();

        // Fetch the binary
        let binary_path = fetcher.fetch(&cache_dir).await?;

        // Spawn the blueprint process using the runtime target from registration config
        let mut service = match registration.config.runtime_target {
            blueprint_eigenlayer_extra::RuntimeTarget::Native => {
                info!("Using Native runtime (no sandbox) - testing only!");
                crate::rt::service::Service::new_native(
                    ctx,
                    limits,
                    &runtime_dir,
                    &service_str,
                    &binary_path,
                    blueprint_env,
                    args,
                )
                .await?
            }
            blueprint_eigenlayer_extra::RuntimeTarget::Hypervisor => {
                #[cfg(feature = "vm-sandbox")]
                {
                    info!("Using Hypervisor runtime (cloud-hypervisor VM)");
                    crate::rt::service::Service::new_vm(
                        ctx,
                        limits,
                        crate::rt::hypervisor::ServiceVmConfig {
                            _id: id,
                            ..Default::default()
                        },
                        &env.data_dir,
                        &env.keystore_uri,
                        &cache_dir,
                        &runtime_dir,
                        &service_str,
                        &binary_path,
                        blueprint_env.clone(),
                        args.clone(),
                    )
                    .await?
                }
                #[cfg(not(feature = "vm-sandbox"))]
                {
                    error!("Hypervisor runtime requested but vm-sandbox feature not enabled");
                    return Err(Error::Other(
                        "Hypervisor runtime not available. Recompile with --features vm-sandbox or use --runtime native for testing.".into()
                    ));
                }
            }
            blueprint_eigenlayer_extra::RuntimeTarget::Container => {
                #[cfg(feature = "containers")]
                {
                    let container_image =
                        registration.config.container_image.clone().ok_or_else(|| {
                            Error::Other(
                                "Container runtime requires container_image in config".into(),
                            )
                        })?;

                    info!("Using Container runtime with image: {}", container_image);
                    crate::rt::service::Service::new_container(
                        ctx,
                        limits,
                        &runtime_dir,
                        &service_str,
                        container_image,
                        blueprint_env.clone(),
                        args.clone(),
                        false, // debug mode
                    )
                    .await?
                }
                #[cfg(not(feature = "containers"))]
                {
                    error!("Container runtime requested but containers feature not enabled");
                    return Err(Error::Other(
                        "Container runtime not available. Recompile with --features containers or use 'native' for testing.".into()
                    ));
                }
            }
        };

        // Start the service
        let service_start_res = service.start().await;
        match service_start_res {
            Ok(Some(is_alive)) => {
                info!(
                    "AVS blueprint {} (blueprint_id={}) started, waiting for health check",
                    registration.config.service_manager, blueprint_id
                );
                is_alive.await?;
                info!(
                    "AVS blueprint {} (blueprint_id={}) is alive and healthy",
                    registration.config.service_manager, blueprint_id
                );

                // Add to active blueprints
                active_blueprints
                    .entry(blueprint_id)
                    .or_default()
                    .insert(service_id, service);
            }
            Ok(None) => {
                info!(
                    "AVS blueprint {} (blueprint_id={}) started (no health check)",
                    registration.config.service_manager, blueprint_id
                );
                active_blueprints
                    .entry(blueprint_id)
                    .or_default()
                    .insert(service_id, service);
            }
            Err(e) => {
                error!(
                    "AVS blueprint {} (blueprint_id={}) did not start successfully: {e}",
                    registration.config.service_manager, blueprint_id
                );
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
    ///
    /// Loads AVS registrations and spawns blueprint instances for each active AVS.
    pub async fn initialize(
        &mut self,
        _client: &EigenlayerProtocolClient,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        info!("Initializing EigenLayer protocol handler");

        // Start operator-level background services (rewards, slashing monitoring)
        if self.background_services.is_none() {
            info!("Starting operator-level background services");
            self.background_services = Some(Self::spawn_background_services(env.clone())?);
        }

        // Start all registered AVS blueprints
        Self::ensure_all_registered_avs_running(env, ctx, active_blueprints).await?;

        info!("EigenLayer protocol handler initialized");
        Ok(())
    }

    /// Spawn background services for operator-level monitoring
    ///
    /// These services run continuously and monitor:
    /// - Rewards accumulation and claiming
    /// - Slashing events
    fn spawn_background_services(env: BlueprintEnvironment) -> Result<BackgroundServices> {
        use blueprint_eigenlayer_extra::{RewardsManager, SlashingMonitor};

        // Spawn rewards monitoring task
        let env_clone = env.clone();
        let rewards_task = tokio::spawn(async move {
            let rewards_manager = RewardsManager::new(env_clone);

            loop {
                // Check for claimable rewards every hour
                match rewards_manager.get_claimable_rewards().await {
                    Ok(amount) => {
                        if amount > alloy_primitives::U256::ZERO {
                            info!("Claimable rewards available: {}", amount);
                            // TODO: Auto-claim based on threshold configuration
                        }
                    }
                    Err(e) => {
                        error!("Failed to check claimable rewards: {}", e);
                    }
                }

                tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
            }
        });

        // Spawn slashing monitoring task
        let env_clone = env.clone();
        let slashing_task = tokio::spawn(async move {
            let slashing_monitor = SlashingMonitor::new(env_clone);

            loop {
                // Check for slashing events every 5 minutes
                match slashing_monitor.is_operator_slashed().await {
                    Ok(is_slashed) => {
                        if is_slashed {
                            error!("CRITICAL: Operator has been slashed!");
                            // TODO: Trigger alert/notification system
                        }
                    }
                    Err(e) => {
                        error!("Failed to check slashing status: {}", e);
                    }
                }

                tokio::time::sleep(std::time::Duration::from_secs(300)).await;
            }
        });

        Ok(BackgroundServices {
            rewards_task,
            slashing_task,
        })
    }

    /// Handle an EigenLayer protocol event
    ///
    /// Ensures all registered AVS blueprints are running. The blueprint binaries
    /// themselves process events via their job handlers.
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

        // Ensure all registered AVS blueprints are still running
        Self::ensure_all_registered_avs_running(env, ctx, active_blueprints).await?;

        // The blueprint binaries themselves process the events via their job handlers
        // (e.g., initialize_bls_task job listening for NewTaskCreated events)
        //
        // We don't need to explicitly route events here like Tangle does,
        // because each blueprint's jobs are already watching for their specific AVS events.
        //
        // Our responsibility is to keep all registered AVS blueprints alive.

        Ok(())
    }
}
