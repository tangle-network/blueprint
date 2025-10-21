/// Tangle Protocol Event Handler
///
/// Handles Tangle-specific events including blueprint registration, service lifecycle, and job execution.
use crate::blueprint::native::FilteredBlueprint;
use crate::blueprint::ActiveBlueprints;
use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::protocol::tangle::client::TangleProtocolClient;
use crate::protocol::types::ProtocolEvent;
use crate::sdk::utils::bounded_string_to_string;
use crate::sources::github::GithubBinaryFetcher;
use crate::sources::testing::TestSourceFetcher;
use crate::sources::{BlueprintArgs, BlueprintEnvVars, BlueprintSourceHandler, DynBlueprintSource};
use blueprint_clients::tangle::client::TangleConfig;
use blueprint_clients::tangle::services::{RpcServicesWithBlueprint, TangleServicesClient};
use blueprint_core::{error, info, trace, warn};
use blueprint_runner::config::{BlueprintEnvironment, Protocol};
use std::sync::Arc;
use tangle_subxt::subxt::utils::AccountId32;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::{
    BlueprintSource, NativeFetcher,
};
use tangle_subxt::tangle_testnet_runtime::api::services::events::{
    JobCalled, JobResultSubmitted, PreRegistration, Registered, ServiceInitiated, Unregistered,
};
use tokio::sync::RwLock;
use crate::rt::ResourceLimits;
use crate::rt::service::Status;

const DEFAULT_PROTOCOL: Protocol = Protocol::Tangle;

/// Internal state maintained by the Tangle event handler
#[derive(Default)]
struct TangleHandlerState {
    /// Blueprints the operator is registered to
    operator_blueprints: Vec<RpcServicesWithBlueprint>,
    /// The operator's account ID
    account_id: Option<AccountId32>,
    /// Services client for querying Tangle state
    services_client: Option<Arc<TangleServicesClient<TangleConfig>>>,
}


/// Tangle protocol event handler implementation
pub struct TangleEventHandler {
    state: Arc<RwLock<TangleHandlerState>>,
}

impl TangleEventHandler {
    /// Create a new Tangle event handler
    #[must_use] pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(TangleHandlerState::default())),
        }
    }

    /// Extract account ID from environment
    fn get_account_id(env: &BlueprintEnvironment) -> Result<AccountId32> {
        use blueprint_crypto::sp_core::SpSr25519;
        use blueprint_crypto::tangle_pair_signer::TanglePairSigner;
        use blueprint_keystore::backends::Backend;
        use blueprint_keystore::{Keystore, KeystoreConfig};
        use tangle_subxt::subxt::tx::Signer;

        let keystore = Keystore::new(KeystoreConfig::new().fs_root(&env.keystore_uri))?;
        let sr_key_pub = keystore.first_local::<SpSr25519>()?;
        let sr_pair = keystore.get_secret::<SpSr25519>(&sr_key_pub)?;
        let sr_key = TanglePairSigner::new(sr_pair.0);

        Ok(sr_key.account_id().clone())
    }

    /// Process Tangle events and return information about what changed
    fn check_events(
        event: &ProtocolEvent,
        active_blueprints: &mut ActiveBlueprints,
        account_id: &AccountId32,
    ) -> EventCheckResult {
        let tangle_event = match event.as_tangle() {
            Some(evt) => evt,
            None => {
                warn!("Expected Tangle event but got different protocol");
                return EventCheckResult::default();
            }
        };

        let inner = &tangle_event.inner;
        let pre_registration_events = inner.events.find::<PreRegistration>();
        let registered_events = inner.events.find::<Registered>();
        let unregistered_events = inner.events.find::<Unregistered>();
        let service_initiated_events = inner.events.find::<ServiceInitiated>();
        let job_called_events = inner.events.find::<JobCalled>();
        let job_result_submitted_events = inner.events.find::<JobResultSubmitted>();

        let mut result = EventCheckResult::default();

        // Handle pre-registration events
        for evt in pre_registration_events {
            match evt {
                Ok(evt) => {
                    if &evt.operator == account_id {
                        result.blueprint_registrations.push(evt.blueprint_id);
                        info!("Pre-registered event: {evt:?}");
                    }
                }
                Err(err) => {
                    warn!("Error handling pre-registered event: {err:?}");
                }
            }
        }

        // Handle registered events
        for evt in registered_events {
            match evt {
                Ok(evt) => {
                    info!("Registered event: {evt:?}");
                    result.needs_update = true;
                }
                Err(err) => {
                    warn!("Error handling registered event: {err:?}");
                }
            }
        }

        // Handle unregistered events
        for evt in unregistered_events {
            match evt {
                Ok(evt) => {
                    info!("Unregistered event: {evt:?}");
                    if &evt.operator == account_id
                        && active_blueprints.remove(&evt.blueprint_id).is_some()
                    {
                        info!("Removed services for blueprint_id: {}", evt.blueprint_id);
                        result.needs_update = true;
                    }
                }
                Err(err) => {
                    warn!("Error handling unregistered event: {err:?}");
                }
            }
        }

        // Handle service initiated events
        for evt in service_initiated_events {
            match evt {
                Ok(evt) => {
                    info!("Service initiated event: {evt:?}");
                    result.needs_update = true;
                }
                Err(err) => {
                    warn!("Error handling service initiated event: {err:?}");
                }
            }
        }

        // Handle job called events
        for evt in job_called_events {
            match evt {
                Ok(evt) => {
                    info!("Job called event: {evt:?}");
                }
                Err(err) => {
                    warn!("Error handling job called event: {err:?}");
                }
            }
        }

        // Handle job result submitted events
        for evt in job_result_submitted_events {
            match evt {
                Ok(evt) => {
                    info!("Job result submitted event: {evt:?}");
                }
                Err(err) => {
                    warn!("Error handling job result submitted event: {err:?}");
                }
            }
        }

        result
    }

    /// Process a Tangle event and start/stop services as needed
    async fn process_event(
        event: &ProtocolEvent,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
        check_result: EventCheckResult,
        services_client: &TangleServicesClient<TangleConfig>,
        operator_blueprints: &[RpcServicesWithBlueprint],
    ) -> Result<()> {
        let tangle_event = event
            .as_tangle()
            .ok_or_else(|| Error::Other("Expected Tangle event in Tangle handler".to_string()))?;

        info!(
            "Processing Tangle event at block {}",
            tangle_event.block_number
        );

        // Handle new registrations from PreRegistration events
        let mut registration_blueprints = Vec::new();
        if !check_result.blueprint_registrations.is_empty() {
            for blueprint_id in &check_result.blueprint_registrations {
                let blueprint = services_client
                    .get_blueprint_by_id(tangle_event.inner.hash, *blueprint_id)
                    .await?
                    .ok_or_else(|| {
                        Error::Other(
                            "Unable to retrieve blueprint for registration mode".to_string(),
                        )
                    })?;

                let general_blueprint = FilteredBlueprint {
                    blueprint_id: *blueprint_id,
                    services: vec![0], // Dummy service id for registration mode
                    sources: blueprint.sources.0,
                    name: bounded_string_to_string(&blueprint.metadata.name)?,
                    registration_mode: true,
                    protocol: DEFAULT_PROTOCOL,
                };

                registration_blueprints.push(general_blueprint);
            }
        }

        // Combine operator blueprints with registration blueprints
        let mut verified_blueprints = Vec::new();
        for blueprint in operator_blueprints
            .iter()
            .map(|r| FilteredBlueprint {
                blueprint_id: r.blueprint_id,
                services: r.services.iter().map(|s| s.id).collect(),
                sources: r.blueprint.sources.0.clone(),
                name: bounded_string_to_string(&r.blueprint.metadata.name)
                    .unwrap_or_else(|_| "unknown_blueprint_name".to_string()),
                registration_mode: false,
                protocol: DEFAULT_PROTOCOL,
            })
            .chain(registration_blueprints)
        {
            let verified_blueprint = VerifiedBlueprint {
                fetchers: get_fetcher_candidates(&blueprint, ctx)?,
                blueprint,
            };

            verified_blueprints.push(verified_blueprint);
        }

        trace!(
            "OnChain Verified Blueprints: {:?}",
            verified_blueprints
                .iter()
                .map(|r| format!("{r:?}"))
                .collect::<Vec<_>>()
        );

        // Start services if needed
        for blueprint in &mut verified_blueprints {
            blueprint
                .start_services_if_needed(env, ctx, active_blueprints)
                .await?;
        }

        // Clean up services that are no longer on-chain or have died
        let mut to_remove: Vec<(u64, u64)> = Vec::new();

        for (blueprint_id, process_handles) in &mut *active_blueprints {
            for (service_id, process_handle) in process_handles {
                info!(
                    "Checking service for on-chain termination: bid={blueprint_id}//sid={service_id}"
                );

                // Check if service is still on-chain
                for verified_blueprint in &verified_blueprints {
                    let services = &verified_blueprint.blueprint.services;
                    if !services.contains(service_id) {
                        warn!(
                            "Killing service that is no longer on-chain: bid={blueprint_id}//sid={service_id}"
                        );
                        to_remove.push((*blueprint_id, *service_id));
                    }
                }

                // Check if process has died
                if !to_remove.contains(&(*blueprint_id, *service_id))
                    && !matches!(process_handle.status().await, Ok(Status::Running))
                {
                    warn!("Killing service that has died to allow for auto-restart");
                    to_remove.push((*blueprint_id, *service_id));
                }
            }
        }

        // Remove dead/terminated services
        for (blueprint_id, service_id) in to_remove {
            let mut should_delete_blueprint = false;
            if let Some(blueprints) = active_blueprints.get_mut(&blueprint_id) {
                warn!(
                    "Removing service that is no longer active: bid={blueprint_id}//sid={service_id}"
                );

                if let Some(process_handle) = blueprints.remove(&service_id) {
                    warn!("Sending abort signal to service: bid={blueprint_id}//sid={service_id}");
                    process_handle.shutdown().await?;
                }

                if blueprints.is_empty() {
                    should_delete_blueprint = true;
                }
            }

            if should_delete_blueprint {
                active_blueprints.remove(&blueprint_id);
            }
        }

        Ok(())
    }
}

impl Default for TangleEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl TangleEventHandler {
    /// Initialize the handler with the protocol client
    pub async fn initialize(
        &mut self,
        client: &TangleProtocolClient,
        env: &BlueprintEnvironment,
        _ctx: &BlueprintManagerContext,
        _active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        info!("Initializing Tangle protocol handler");

        // Get the services client from the Tangle client
        let services_client = Arc::new(client.tangle_client().services_client().clone());

        // Get the account ID from the keystore
        let account_id = Self::get_account_id(env)?;

        // Store in state for future use
        {
            let mut state = self.state.write().await;
            state.services_client = Some(services_client.clone());
            state.account_id = Some(account_id.clone());
        }

        info!(
            "Tangle protocol handler initialized for operator: {}",
            account_id
        );

        Ok(())
    }

    /// Handle a Tangle protocol event
    pub async fn handle_event(
        &mut self,
        event: &ProtocolEvent,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        // Get state
        let (services_client, account_id, mut operator_blueprints) = {
            let state = self.state.read().await;
            let services_client = state
                .services_client
                .as_ref()
                .ok_or_else(|| Error::Other("Handler not initialized".to_string()))?
                .clone();
            let account_id = state
                .account_id
                .as_ref()
                .ok_or_else(|| Error::Other("Handler not initialized".to_string()))?
                .clone();
            let operator_blueprints = state.operator_blueprints.clone();
            (services_client, account_id, operator_blueprints)
        };

        // Check what happened in this event
        let check_result = Self::check_events(event, active_blueprints, &account_id);

        // If we need to update blueprints, query the latest state
        if check_result.needs_update || operator_blueprints.is_empty() {
            let tangle_event = event.as_tangle().ok_or_else(|| {
                Error::Other("Expected Tangle event in Tangle handler".to_string())
            })?;

            operator_blueprints = services_client
                .query_operator_blueprints(tangle_event.inner.hash, account_id.clone())
                .await
                .unwrap_or_else(|err| {
                    warn!("Failed to query operator blueprints: {err}");
                    Vec::new()
                });

            // Update state with new blueprints
            {
                let mut state = self.state.write().await;
                state.operator_blueprints = operator_blueprints.clone();
            }
        }

        // Process the event
        Self::process_event(
            event,
            env,
            ctx,
            active_blueprints,
            check_result,
            &services_client,
            &operator_blueprints,
        )
        .await
    }
}

/// Result of checking Tangle events
#[derive(Default, Debug)]
struct EventCheckResult {
    needs_update: bool,
    blueprint_registrations: Vec<u64>,
}

/// A verified blueprint ready to be instantiated
struct VerifiedBlueprint {
    fetchers: Vec<Box<DynBlueprintSource<'static>>>,
    blueprint: FilteredBlueprint,
}

impl VerifiedBlueprint {
    async fn start_services_if_needed(
        &mut self,
        blueprint_config: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        let cache_dir = ctx.cache_dir().join(format!(
            "{}-{}",
            self.blueprint.blueprint_id, self.blueprint.name
        ));
        if let Err(e) = std::fs::create_dir_all(&cache_dir) {
            error!(
                "Failed to create cache directory for blueprint at {} (name: {}, id: {})",
                cache_dir.display(),
                self.blueprint.name,
                self.blueprint.blueprint_id
            );
            return Err(e.into());
        }

        for (index, source) in self.fetchers.iter_mut().enumerate() {
            let blueprint = &self.blueprint;
            let blueprint_id = source.blueprint_id();

            if active_blueprints.contains_key(&blueprint_id) {
                return Ok(());
            }

            if let Err(e) = source.fetch(&cache_dir).await {
                error!(
                    "Failed to fetch blueprint from source at index #{index}[{source_type}]: {e} (blueprint: {blueprint_name}, id: {blueprint_id}). attempting next source",
                    source_type = core::any::type_name_of_val(source),
                    blueprint_name = blueprint.name,
                );
                continue;
            }

            let service_str = source.name();
            for service_id in &blueprint.services {
                let sub_service_str = format!("{service_str}-{service_id}");

                let args = BlueprintArgs::new(ctx);
                let env = BlueprintEnvVars::new(
                    blueprint_config,
                    ctx,
                    blueprint_id,
                    *service_id,
                    blueprint,
                    &sub_service_str,
                );

                info!("Starting protocol: {sub_service_str}");

                let id = active_blueprints.len() as u32;
                let runtime_dir = ctx.runtime_dir().join(id.to_string());
                std::fs::create_dir_all(&runtime_dir)?;

                let limits = ResourceLimits::default();

                let mut service = source
                    .spawn(
                        ctx,
                        limits,
                        blueprint_config,
                        id,
                        env,
                        args,
                        &sub_service_str,
                        &cache_dir,
                        &runtime_dir,
                    )
                    .await?;

                let service_start_res = service.start().await;
                match service_start_res {
                    Ok(Some(is_alive)) => {
                        is_alive.await?;

                        active_blueprints
                            .entry(blueprint_id)
                            .or_default()
                            .insert(*service_id, service);
                    }
                    Ok(None) => {}
                    Err(e) => {
                        error!("Service did not start successfully, aborting: {e}");
                        service.shutdown().await?;
                    }
                }
            }

            break;
        }

        Ok(())
    }
}

impl std::fmt::Debug for VerifiedBlueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(
            "{}/bid={}/sid(s)={:?}",
            self.blueprint.name, self.blueprint.blueprint_id, self.blueprint.services
        )
        .fmt(f)
    }
}

/// Get fetcher candidates for a blueprint based on its sources
fn get_fetcher_candidates(
    blueprint: &FilteredBlueprint,
    ctx: &BlueprintManagerContext,
) -> Result<Vec<Box<DynBlueprintSource<'static>>>> {
    let mut test_fetcher_idx = None;
    let mut fetcher_candidates: Vec<Box<DynBlueprintSource<'static>>> = Vec::new();

    for (source_idx, blueprint_source) in blueprint.sources.iter().enumerate() {
        match blueprint_source {
            BlueprintSource::Wasm { .. } => {
                warn!("WASM blueprints are not supported yet");
                return Err(Error::UnsupportedBlueprint);
            }

            BlueprintSource::Native(native) => match native {
                NativeFetcher::Github(gh) => {
                    let fetcher = GithubBinaryFetcher::new(
                        gh.clone(),
                        blueprint.blueprint_id,
                        blueprint.name.clone(),
                        ctx.allow_unchecked_attestations,
                    );
                    fetcher_candidates.push(DynBlueprintSource::boxed(fetcher));
                }
                NativeFetcher::IPFS(_) => {
                    warn!("IPFS Native sources are not supported yet");
                    return Err(Error::UnsupportedBlueprint);
                }
            },

            #[cfg(feature = "containers")]
            BlueprintSource::Container(container) => {
                let fetcher = crate::sources::container::ContainerSource::new(
                    container.clone(),
                    blueprint.blueprint_id,
                    blueprint.name.clone(),
                );
                fetcher_candidates.push(DynBlueprintSource::boxed(fetcher));
            }

            #[cfg(not(feature = "containers"))]
            BlueprintSource::Container(_) => {
                return Err(Error::UnsupportedBlueprint);
            }

            BlueprintSource::Testing(test) => {
                warn!("Using testing fetcher");

                let fetcher = TestSourceFetcher::new(
                    test.clone(),
                    blueprint.blueprint_id,
                    blueprint.name.clone(),
                );

                test_fetcher_idx = Some(source_idx);
                fetcher_candidates.push(DynBlueprintSource::boxed(fetcher));
            }
        }
    }

    // Sanity checks
    if fetcher_candidates.is_empty() {
        return Err(Error::NoFetchers);
    }

    if ctx.test_mode && test_fetcher_idx.is_none() {
        return Err(Error::NoTestFetcher);
    }

    if ctx.test_mode {
        fetcher_candidates =
            vec![fetcher_candidates.remove(test_fetcher_idx.expect("Should exist"))];
    }

    Ok(fetcher_candidates)
}
