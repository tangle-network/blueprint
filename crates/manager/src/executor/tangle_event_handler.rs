use std::fs;

use crate::config::{BlueprintManagerConfig, BlueprintManagerContext};
use crate::error::{Error, Result};
use crate::blueprint::native::FilteredBlueprint;
use crate::blueprint::ActiveBlueprints;
use crate::sdk::utils::bounded_string_to_string;
use crate::sources::github::GithubBinaryFetcher;
use crate::sources::{BlueprintArgs, BlueprintEnvVars, BlueprintSourceHandler, DynBlueprintSource};
use crate::sources::testing::TestSourceFetcher;
use blueprint_clients::EventsClient;
use blueprint_clients::tangle::client::{TangleClient, TangleConfig, TangleEvent};
use blueprint_clients::tangle::services::{RpcServicesWithBlueprint, TangleServicesClient};
use blueprint_runner::config::Protocol;
use blueprint_runner::config::BlueprintEnvironment;
use blueprint_core::{error, info, trace, warn};
use blueprint_std::fmt::Debug;
use tangle_subxt::subxt::utils::AccountId32;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::{BlueprintSource, NativeFetcher};
use tangle_subxt::tangle_testnet_runtime::api::services::events::{
    JobCalled, JobResultSubmitted, PreRegistration, Registered, ServiceInitiated, Unregistered,
};
use crate::rt::ResourceLimits;
use crate::rt::service::Status;

const DEFAULT_PROTOCOL: Protocol = Protocol::Tangle;


/// * Query to get Vec<RpcServicesWithBlueprint>
/// * For each `RpcServicesWithBlueprint`, fetch the associated blueprint binary (fetch/download)
///   -> If the services field is empty, just emit and log inside the executed binary "that states a new service instance got created by one of these blueprints"
///   -> If the services field is not empty, for each service in RpcServicesWithBlueprint.services, spawn the blueprint binary, using params to set the job type to listen to (in terms of our old language, each spawned service represents a single "`RoleType`")
#[allow(clippy::too_many_arguments)]
pub async fn handle_init(
    tangle_runtime: &TangleClient,
    services_client: &TangleServicesClient<TangleConfig>,
    sub_account_id: &AccountId32,
    active_blueprints: &mut ActiveBlueprints,
    blueprint_env: &BlueprintEnvironment,
    ctx: &BlueprintManagerContext,
) -> Result<Vec<RpcServicesWithBlueprint>> {
    info!("Beginning initialization of Blueprint Manager");

    let Some(init_event) = tangle_runtime.next_event().await else {
        return Err(Error::InitialBlock);
    };

    let maybe_operator_subscribed_blueprints = services_client
        .query_operator_blueprints(init_event.hash, sub_account_id.clone())
        .await;

    let operator_subscribed_blueprints =
        maybe_operator_subscribed_blueprints.unwrap_or_else(|err| {
            warn!(
                "Failed to query operator blueprints: {}, did you register as an operator?",
                err
            );
            Vec::new()
        });

    info!(
        "Received {} initial blueprints this operator is registered to",
        operator_subscribed_blueprints.len()
    );

    // Immediately poll, handling the initial state
    let poll_result =
        check_blueprint_events(&init_event, active_blueprints, sub_account_id);

    handle_tangle_event(
        &init_event,
        &operator_subscribed_blueprints,
        blueprint_env,
        ctx,
        active_blueprints,
        poll_result,
        services_client,
    )
    .await?;

    Ok(operator_subscribed_blueprints)
}

pub struct VerifiedBlueprint {
    pub(crate) fetchers: Vec<Box<DynBlueprintSource<'static>>>,
    pub(crate) blueprint: FilteredBlueprint,
}

impl VerifiedBlueprint {
    #[allow(clippy::cast_possible_truncation)]
    pub async fn start_services_if_needed(
        &mut self,
        blueprint_config: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        let cache_dir = ctx.cache_dir().join(format!(
            "{}-{}",
            self.blueprint.name, self.blueprint.blueprint_id
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
                    index = index,
                    source_type = core::any::type_name_of_val(source),
                    e = e,
                    blueprint_name = blueprint.name,
                    blueprint_id = blueprint.blueprint_id
                );
                continue;
            }

            // TODO(serial): Check preferred sources first
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

                info!("Starting protocol: {sub_service_str}",);

                let id = active_blueprints.len() as u32;

                let runtime_dir = ctx.runtime_dir().join(id.to_string());
                fs::create_dir_all(&runtime_dir)?;

                // TODO: Actually configure resource limits
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

impl Debug for VerifiedBlueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(
            "{}/bid={}/sid(s)={:?}",
            self.blueprint.name, self.blueprint.blueprint_id, self.blueprint.services
        )
        .fmt(f)
    }
}

#[derive(Default, Debug)]
pub struct EventPollResult {
    pub needs_update: bool,
    // A vec of blueprints we have not yet become registered to
    pub blueprint_registrations: Vec<u64>,
}

pub(crate) fn check_blueprint_events(
    event: &TangleEvent,
    active_blueprints: &mut ActiveBlueprints,
    account_id: &AccountId32,
) -> EventPollResult {
    let pre_registration_events = event.events.find::<PreRegistration>();
    let registered_events = event.events.find::<Registered>();
    let unregistered_events = event.events.find::<Unregistered>();
    let service_initiated_events = event.events.find::<ServiceInitiated>();
    let job_called_events = event.events.find::<JobCalled>();
    let job_result_submitted_events = event.events.find::<JobResultSubmitted>();

    let mut result = EventPollResult::default();

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
                    info!("Removed services for blueprint_id: {}", evt.blueprint_id,);

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

#[allow(clippy::too_many_arguments)]
pub(crate) async fn handle_tangle_event(
    event: &TangleEvent,
    blueprints: &[RpcServicesWithBlueprint],
    blueprint_config: &BlueprintEnvironment,
    ctx: &BlueprintManagerContext,
    active_blueprints: &mut ActiveBlueprints,
    poll_result: EventPollResult,
    client: &TangleServicesClient<TangleConfig>,
) -> Result<()> {
    info!("Received notification {}", event.number);

    let mut registration_blueprints = vec![];
    // First, check to see if we need to register any new services invoked by the PreRegistration event
    if !poll_result.blueprint_registrations.is_empty() {
        for blueprint_id in &poll_result.blueprint_registrations {
            let blueprint = client
                .get_blueprint_by_id(event.hash, *blueprint_id)
                .await?
                .ok_or_else(|| {
                    Error::Other(String::from(
                        "Unable to retrieve blueprint for registration mode",
                    ))
                })?;

            let general_blueprint = FilteredBlueprint {
                blueprint_id: *blueprint_id,
                services: vec![0], // Add a dummy service id for now, since it does not matter for registration mode
                sources: blueprint.sources.0,
                name: bounded_string_to_string(&blueprint.metadata.name)?,
                registration_mode: true,
                protocol: DEFAULT_PROTOCOL,
            };

            registration_blueprints.push(general_blueprint);
        }
    }

    let mut verified_blueprints = vec![];

    for blueprint in blueprints
        .iter()
        .map(|r| FilteredBlueprint {
            blueprint_id: r.blueprint_id,
            services: r.services.iter().map(|r| r.id).collect(),
            sources: r.blueprint.sources.0.clone(),
            name: bounded_string_to_string(&r.blueprint.metadata.name)
                .unwrap_or("unknown_blueprint_name".to_string()),
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

    // Step 3: Check to see if we need to start any new services
    for blueprint in &mut verified_blueprints {
        blueprint
            .start_services_if_needed(blueprint_config, ctx, active_blueprints)
            .await?;
    }

    // Check to see if local is running services that are not on-chain
    let mut to_remove: Vec<(u64, u64)> = vec![];

    // Loop through every (blueprint_id, service_id) running. See if the service is still on-chain. If not, kill it and add it to to_remove
    for (blueprint_id, process_handles) in &mut *active_blueprints {
        for (service_id, process_handle) in process_handles {
            info!(
                "Checking service for on-chain termination: bid={blueprint_id}//sid={service_id}"
            );

            // Since the below "verified blueprints" were freshly obtained from an on-chain source,
            // we compare all these fresh values to see if we're running a service locally that is no longer on-chain
            for verified_blueprints in &verified_blueprints {
                let services = &verified_blueprints.blueprint.services;
                if !services.contains(service_id) {
                    warn!(
                        "Killing service that is no longer on-chain: bid={blueprint_id}//sid={service_id}"
                    );
                    to_remove.push((*blueprint_id, *service_id));
                }
            }

            // Check to see if any process handles have died
            if !to_remove.contains(&(*blueprint_id, *service_id))
                && !matches!(process_handle.status().await, Ok(Status::Running))
            {
                // By removing any killed processes, we will auto-restart them on the next finality notification if required
                warn!("Killing service that has died to allow for auto-restart");
                to_remove.push((*blueprint_id, *service_id));
            }
        }
    }

    for (blueprint_id, service_id) in to_remove {
        let mut should_delete_blueprint = false;
        if let Some(blueprints) = active_blueprints.get_mut(&blueprint_id) {
            warn!(
                "Removing service that is no longer active on-chain or killed: bid={blueprint_id}//sid={service_id}"
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

fn get_fetcher_candidates(
    blueprint: &FilteredBlueprint,
    manager_opts: &BlueprintManagerConfig,
) -> Result<Vec<Box<DynBlueprintSource<'static>>>> {
    let mut test_fetcher_idx = None;
    let mut fetcher_candidates: Vec<Box<DynBlueprintSource<'static>>> = vec![];

    for (source_idx, blueprint_source) in blueprint.sources.iter().enumerate() {
        match &blueprint_source {
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
                        manager_opts.allow_unchecked_attestations,
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
                // TODO: demote to TRACE once proven to work
                warn!("Using testing fetcher");
                // if !manager_opts.test_mode {
                //     warn!("Ignoring testing fetcher as we are not in test mode");
                //     continue;
                // }

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

    // A bunch of sanity checks to enforce structure

    // Ensure that we have at least one fetcher
    if fetcher_candidates.is_empty() {
        return Err(Error::NoFetchers);
    }

    // Ensure that we have a test fetcher if we are in test mode
    if manager_opts.test_mode && test_fetcher_idx.is_none() {
        return Err(Error::NoTestFetcher);
    }

    // Ensure that we have only one fetcher if we are in test mode
    if manager_opts.test_mode {
        fetcher_candidates =
            vec![fetcher_candidates.remove(test_fetcher_idx.expect("Should exist"))];
    }

    Ok(fetcher_candidates)
}
