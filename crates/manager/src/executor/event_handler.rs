use std::fs;
use crate::config::{BlueprintManagerConfig, BlueprintManagerContext};
use crate::error::{Error, Result};
use crate::blueprint::native::FilteredBlueprint;
use crate::blueprint::ActiveBlueprints;
use crate::sdk::utils::bounded_string_to_string;
use crate::sources::github::GithubBinaryFetcher;
use crate::sources::{BlueprintArgs, BlueprintEnvVars, BlueprintSourceHandler, DynBlueprintSource};
use crate::sources::testing::TestSourceFetcher;
use blueprint_clients::tangle::client::{TangleConfig, TangleEvent};
use blueprint_clients::tangle::services::{RpcServicesWithBlueprint, TangleServicesClient};
use blueprint_runner::config::Protocol;
use blueprint_runner::config::BlueprintEnvironment;
use blueprint_core::{error, info, trace, warn};
use blueprint_std::fmt::Debug;
use tangle_subxt::subxt::utils::AccountId32;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::{BlueprintSource, NativeFetcher};
use tangle_subxt::tangle_testnet_runtime::api::services::events::{
    JobCalled, JobResultSubmitted, PreRegistration, Registered, ServiceInitiated, ServiceTerminated, Unregistered,
};
use crate::rt::ResourceLimits;
use crate::rt::service::Status;
#[cfg(feature = "remote-providers")]
use crate::rt::Service;

const DEFAULT_PROTOCOL: Protocol = Protocol::Tangle;

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

                // Check if remote deployment is enabled and should be used
                #[cfg(feature = "remote-providers")]
                let mut service = {
                    if ctx.config.remote_deployment_opts.enable_remote_deployments {
                        info!("Remote deployments enabled, checking deployment strategy...");

                        // Try remote deployment first if enabled
                        match try_remote_deployment(
                            ctx,
                            blueprint_id,
                            *service_id,
                            limits.clone(),
                            &sub_service_str,
                        )
                        .await
                        {
                            Ok(remote_service) => {
                                info!("Successfully deployed service remotely");
                                remote_service
                            }
                            Err(e) => {
                                warn!("Remote deployment failed: {}, falling back to local", e);
                                // Fall back to local deployment
                                source
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
                                    .await?
                            }
                        }
                    } else {
                        // Local deployment
                        source
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
                            .await?
                    }
                };

                #[cfg(not(feature = "remote-providers"))]
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
    #[cfg(feature = "remote-providers")]
    pub service_initiated: Vec<ServiceInitiated>,
    #[cfg(feature = "remote-providers")]
    pub service_terminated: Vec<ServiceTerminated>,
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
    let service_terminated_events = event.events.find::<ServiceTerminated>();
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
                info!(
                    "Available event fields - blueprint_id: {}, service_id: {}, request_id: {}, owner: {:?}",
                    evt.blueprint_id, evt.service_id, evt.request_id, evt.owner
                );
                result.needs_update = true;

                #[cfg(feature = "remote-providers")]
                {
                    // Store event data for remote provider handling
                    result.service_initiated.push(evt);
                }
            }
            Err(err) => {
                warn!("Error handling service initiated event: {err:?}");
            }
        }
    }

    // Handle service terminated events
    for evt in service_terminated_events {
        match evt {
            Ok(evt) => {
                info!("Service terminated event: {evt:?}");
                info!(
                    "Service terminated - blueprint_id: {}, service_id: {}, owner: {:?}",
                    evt.blueprint_id, evt.service_id, evt.owner
                );
                result.needs_update = true;

                #[cfg(feature = "remote-providers")]
                {
                    // Store event data for remote provider handling
                    result.service_terminated.push(evt);
                }
            }
            Err(err) => {
                warn!("Error handling service terminated event: {err:?}");
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

/// Try to deploy a service remotely using cloud providers
#[cfg(feature = "remote-providers")]
async fn try_remote_deployment(
    ctx: &BlueprintManagerContext,
    blueprint_id: u64,
    service_id: u64,
    limits: ResourceLimits,
    service_name: &str,
) -> Result<Service> {
    use blueprint_remote_providers::{
        auto_deployment::AutoDeploymentManager, resources::ResourceSpec,
    };

    info!("Attempting remote deployment for service: {}", service_name);

    // Convert ResourceLimits to ResourceSpec - use actual CPU count from limits
    let resource_spec = ResourceSpec {
        cpu: limits.cpu_count.map(|c| c as f64).unwrap_or(2.0), // Use actual CPU count or default to 2
        memory_gb: (limits.memory_size / (1024 * 1024 * 1024)) as f64,
        storage_gb: (limits.storage_space / (1024 * 1024 * 1024)) as f64,
        gpu_count: limits.gpu_count.map(|c| c as u32),
        network_bandwidth_mbps: limits.network_bandwidth.map(|b| b as f64),
    };

    // Load credentials if provided
    let credentials_path = ctx
        .config
        .remote_deployment_opts
        .cloud_credentials_path
        .as_ref()
        .ok_or_else(|| Error::Other("Cloud credentials path not configured".into()))?;

    // Create auto-deployment manager
    let mut manager = AutoDeploymentManager::new();
    manager.load_credentials_from_file(credentials_path)?;

    // Set max cost limit
    manager.set_max_hourly_cost(ctx.config.remote_deployment_opts.max_hourly_cost);

    // Deploy to cheapest provider
    let deployment_config = manager
        .auto_deploy_service(blueprint_id, service_id, resource_spec, None)
        .await
        .map_err(|e| Error::Other(format!("Remote deployment failed: {}", e)))?;

    info!(
        "Successfully deployed to {} in region {} (instance: {})",
        deployment_config.provider, deployment_config.region, deployment_config.instance_id
    );

    // Create DeploymentTracker with proper path configuration
    let tracker_path = ctx.data_dir().join("remote_deployments");
    let tracker = std::sync::Arc::new(
        blueprint_remote_providers::deployment::tracker::DeploymentTracker::new(&tracker_path)
            .await?,
    );

    // Create the remote service instance
    let remote_instance = crate::rt::remote::RemoteServiceInstance::new(deployment_config, tracker);

    // Create runtime directory for this service
    let runtime_dir = ctx.runtime_dir().join(format!("remote-{}", service_id));
    std::fs::create_dir_all(&runtime_dir)?;

    // Return the Service wrapped around the remote instance
    Service::new_remote(ctx, runtime_dir, service_name, remote_instance).await
}
