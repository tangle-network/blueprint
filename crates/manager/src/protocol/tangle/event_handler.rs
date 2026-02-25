use std::sync::Arc;

use alloy_sol_types::sol;
use async_trait::async_trait;
use blueprint_client_tangle::contracts::ITangle;
use blueprint_core::info;
use blueprint_runner::config::{BlueprintEnvironment, Protocol};
use tokio::fs::create_dir_all;

use crate::blueprint::ActiveBlueprints;
use crate::blueprint::native::FilteredBlueprint;
use crate::config::BlueprintManagerContext;
use crate::config::SourceType;
use crate::error::{Error, Result};
use crate::protocol::tangle::client::TangleProtocolClient;
use crate::protocol::tangle::metadata::OnChainMetadataProvider;
use crate::protocol::types::ProtocolEvent;
use crate::rt::ResourceLimits;
use crate::sources::github::GithubBinaryFetcher;
use crate::sources::remote::RemoteBinaryFetcher;
use crate::sources::testing::TestSourceFetcher;
use crate::sources::types::BlueprintSource;
use crate::sources::{BlueprintArgs, BlueprintEnvVars, BlueprintSourceHandler, DynBlueprintSource};

sol! {
    #[allow(missing_docs)]
    event OperatorPreRegistered(uint64 indexed blueprintId, address indexed operator);
}

/// Reserved service identifier used for preregistration-mode launches.
pub const REGISTRATION_SERVICE_ID: u64 = 0;

/// Blueprint metadata describing how to launch a service.
#[derive(Debug, Clone)]
pub struct BlueprintMetadata {
    pub blueprint_id: u64,
    pub service_id: u64,
    pub name: String,
    pub sources: Vec<BlueprintSource>,
    pub registration_mode: bool,
    pub registration_capture_only: bool,
}

#[async_trait]
pub trait BlueprintMetadataProvider: Send + Sync {
    async fn resolve_service(
        &self,
        client: &TangleProtocolClient,
        service_id: u64,
    ) -> Result<Option<BlueprintMetadata>>;

    async fn resolve_registration(
        &self,
        client: &TangleProtocolClient,
        blueprint_id: u64,
    ) -> Result<Option<BlueprintMetadata>>;
}

/// Handles Tangle events and translates them into blueprint lifecycle actions.
pub struct TangleEventHandler {
    metadata: Arc<dyn BlueprintMetadataProvider>,
}

impl TangleEventHandler {
    #[must_use]
    pub fn new() -> Self {
        Self {
            metadata: Arc::new(OnChainMetadataProvider::new()),
        }
    }

    #[must_use]
    pub fn with_metadata_provider(metadata: Arc<dyn BlueprintMetadataProvider>) -> Self {
        Self { metadata }
    }

    /// Initialize the handler by syncing the client with the latest observed block.
    ///
    /// # Errors
    ///
    /// Returns an error if the client fails to provide initialization data.
    pub async fn initialize(
        &self,
        client: &mut TangleProtocolClient,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        if env.registration_mode() {
            let settings = env
                .protocol_settings
                .tangle()
                .map_err(|e| Error::Other(e.to_string()))?;
            let blueprint_id = settings.blueprint_id;
            if let Some(mut metadata) = self
                .metadata
                .resolve_registration(client, blueprint_id)
                .await?
            {
                metadata.registration_capture_only = env.registration_capture_only();
                self.ensure_service_running(metadata, env, ctx, active_blueprints)
                    .await?;
            } else {
                info!(
                    blueprint_id,
                    "Registration-mode launch skipped; metadata unavailable"
                );
            }
            return Ok(());
        }

        if let Some(evt) = client.client().latest_event().await {
            info!("Tangle client initialized at block {}", evt.block_number);
            // Process historical events (e.g. ServiceActivated) that were emitted
            // before the Manager started — catches up on pre-seeded state.
            use crate::protocol::types::TangleProtocolEvent;
            let proto_event = ProtocolEvent::Tangle(TangleProtocolEvent {
                block_number: evt.block_number,
                block_hash: evt.block_hash,
                timestamp: evt.timestamp,
                logs: evt.logs.clone(),
                inner: evt,
            });
            self.handle_event(client, &proto_event, env, ctx, active_blueprints)
                .await?;
        }

        // Fallback: if no services were discovered via events (e.g. Anvil
        // state-only snapshot with no log/receipt data), enumerate services
        // directly from on-chain contract state.
        if active_blueprints.is_empty() {
            let operator = client.client().account();
            let service_count = client.client().service_count().await.unwrap_or(0);
            if service_count > 0 {
                info!(
                    service_count,
                    %operator,
                    "No services found via events; scanning contract state"
                );
            }
            for service_id in 0..service_count {
                match client
                    .client()
                    .is_service_operator(service_id, operator)
                    .await
                {
                    Ok(true) => {
                        info!(
                            service_id,
                            "Found active service for operator via contract state"
                        );
                        match self.metadata.resolve_service(client, service_id).await {
                            Ok(Some(metadata)) => {
                                if let Err(e) = self
                                    .ensure_service_running(metadata, env, ctx, active_blueprints)
                                    .await
                                {
                                    info!(service_id, error = %e, "Failed to start service from contract state");
                                }
                            }
                            Ok(None) => {
                                info!(service_id, "Service metadata unavailable");
                            }
                            Err(e) => {
                                info!(service_id, error = %e, "Failed to resolve service metadata");
                            }
                        }
                    }
                    Ok(false) => {}
                    Err(e) => {
                        info!(service_id, error = %e, "Failed to check operator membership");
                    }
                }
            }
        }

        Ok(())
    }

    /// Process a protocol event and reconcile running services.
    ///
    /// # Errors
    ///
    /// Returns an error if metadata resolution fails or service management
    /// encounters an unrecoverable issue.
    pub async fn handle_event(
        &self,
        client: &TangleProtocolClient,
        event: &ProtocolEvent,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        let tangle_evt = event
            .as_tangle()
            .ok_or_else(|| Error::Other("Expected Tangle event in handler".to_string()))?;

        for log in &tangle_evt.logs {
            if let Ok(evt) = log.log_decode::<ITangle::ServiceActivated>() {
                let service_id = evt.inner.serviceId;
                if let Some(metadata) = self.metadata.resolve_service(client, service_id).await? {
                    self.ensure_service_running(metadata, env, ctx, active_blueprints)
                        .await?;
                } else {
                    info!(
                        service_id,
                        "ServiceActivated observed but metadata unavailable"
                    );
                }
            } else if let Ok(evt) = log.log_decode::<ITangle::ServiceTerminated>() {
                let service_id = evt.inner.serviceId;
                if let Ok(service) = client.client().get_service(service_id).await {
                    self.stop_service(service.blueprintId, service_id, active_blueprints)
                        .await?;
                }
            } else if let Ok(evt) = log.log_decode::<OperatorPreRegistered>() {
                let blueprint_id = evt.inner.data.blueprintId;
                if let Some(metadata) = self
                    .metadata
                    .resolve_registration(client, blueprint_id)
                    .await?
                {
                    self.ensure_service_running(metadata, env, ctx, active_blueprints)
                        .await?;
                } else {
                    info!(
                        blueprint_id,
                        "OperatorPreRegistered observed but blueprint metadata unavailable"
                    );
                }
            }
        }

        Ok(())
    }

    async fn ensure_service_running(
        &self,
        metadata: BlueprintMetadata,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        if active_blueprints
            .get(&metadata.blueprint_id)
            .and_then(|services| services.get(&metadata.service_id))
            .is_some()
        {
            return Ok(());
        }

        if metadata.sources.is_empty() {
            return Err(Error::NoFetchers);
        }

        let filtered = FilteredBlueprint {
            blueprint_id: metadata.blueprint_id,
            services: vec![metadata.service_id],
            sources: metadata.sources.clone(),
            name: metadata.name.clone(),
            registration_mode: metadata.registration_mode,
            registration_capture_only: metadata.registration_capture_only,
            protocol: Protocol::Tangle,
        };

        let service_label = format!("svc-{}-{}", metadata.blueprint_id, metadata.service_id);
        let runtime_dir = ctx.runtime_dir().join(&service_label);
        create_dir_all(&runtime_dir).await?;
        let cache_dir = ctx.cache_dir().join(&service_label);
        create_dir_all(&cache_dir).await?;

        let mut last_err: Option<Error> = None;

        for source in &metadata.sources {
            let mut handler = build_source_handler(
                source,
                metadata.blueprint_id,
                metadata.name.clone(),
                ctx.allow_unchecked_attestations,
            );
            let env_vars = BlueprintEnvVars::new(
                env,
                ctx,
                metadata.blueprint_id,
                metadata.service_id,
                &filtered,
                &metadata.name,
            );
            let args = BlueprintArgs::new(ctx).with_dry_run(env.dry_run);
            let limits = ResourceLimits::default();
            let service_idx = metadata.service_id.try_into().unwrap_or(u32::MAX);

            match handler
                .spawn(
                    ctx,
                    limits,
                    env,
                    service_idx,
                    env_vars,
                    args,
                    &service_label,
                    &cache_dir,
                    &runtime_dir,
                )
                .await
            {
                Ok(mut service) => {
                    if let Some(health) = service.start().await? {
                        if let Err(e) = health.await {
                            last_err = Some(e);
                            continue;
                        }
                    }

                    active_blueprints
                        .entry(metadata.blueprint_id)
                        .or_default()
                        .insert(metadata.service_id, service);
                    info!(
                        "Started Tangle blueprint {} service {}",
                        metadata.blueprint_id, metadata.service_id
                    );
                    return Ok(());
                }
                Err(e) => last_err = Some(e),
            }
        }

        // Fallback: when preferred_source is Native and all on-chain sources failed,
        // try building from local cargo workspace if BLUEPRINT_CARGO_BIN is set.
        if ctx.preferred_source == SourceType::Native {
            if let Ok(cargo_bin) = std::env::var("BLUEPRINT_CARGO_BIN") {
                let base_path = std::env::current_dir()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| ".".to_string());
                info!(
                    cargo_bin = %cargo_bin,
                    base_path = %base_path,
                    "On-chain sources failed; trying local cargo binary fallback"
                );
                let test_source = BlueprintSource::Testing(crate::sources::types::TestFetcher {
                    cargo_package: cargo_bin.clone(),
                    cargo_bin,
                    base_path,
                });
                let mut handler = build_source_handler(
                    &test_source,
                    metadata.blueprint_id,
                    metadata.name.clone(),
                    ctx.allow_unchecked_attestations,
                );
                let env_vars = BlueprintEnvVars::new(
                    env,
                    ctx,
                    metadata.blueprint_id,
                    metadata.service_id,
                    &filtered,
                    &metadata.name,
                );
                let args = BlueprintArgs::new(ctx).with_dry_run(env.dry_run);
                let limits = ResourceLimits::default();
                let service_idx = metadata.service_id.try_into().unwrap_or(u32::MAX);

                match handler
                    .spawn(
                        ctx,
                        limits,
                        env,
                        service_idx,
                        env_vars,
                        args,
                        &service_label,
                        &cache_dir,
                        &runtime_dir,
                    )
                    .await
                {
                    Ok(mut service) => {
                        if let Some(health) = service.start().await? {
                            if let Err(e) = health.await {
                                last_err = Some(e);
                            } else {
                                active_blueprints
                                    .entry(metadata.blueprint_id)
                                    .or_default()
                                    .insert(metadata.service_id, service);
                                info!(
                                    "Started Tangle blueprint {} service {} via local cargo fallback",
                                    metadata.blueprint_id, metadata.service_id
                                );
                                return Ok(());
                            }
                        } else {
                            active_blueprints
                                .entry(metadata.blueprint_id)
                                .or_default()
                                .insert(metadata.service_id, service);
                            info!(
                                "Started Tangle blueprint {} service {} via local cargo fallback",
                                metadata.blueprint_id, metadata.service_id
                            );
                            return Ok(());
                        }
                    }
                    Err(e) => last_err = Some(e),
                }
            }
        }

        Err(last_err.unwrap_or(Error::NoFetchers))
    }

    async fn stop_service(
        &self,
        blueprint_id: u64,
        service_id: u64,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        if let Some(services) = active_blueprints.get_mut(&blueprint_id) {
            if let Some(service) = services.remove(&service_id) {
                info!(
                    "Stopping Tangle blueprint {} service {}",
                    blueprint_id, service_id
                );
                service.shutdown().await?;
            }
            if services.is_empty() {
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

fn build_source_handler(
    source: &BlueprintSource,
    blueprint_id: u64,
    blueprint_name: String,
    allow_unchecked_attestations: bool,
) -> Box<DynBlueprintSource<'static>> {
    match source {
        BlueprintSource::Testing(fetcher) => DynBlueprintSource::boxed(TestSourceFetcher::new(
            fetcher.clone(),
            blueprint_id,
            blueprint_name,
        )),
        BlueprintSource::Github(fetcher) => DynBlueprintSource::boxed(GithubBinaryFetcher::new(
            fetcher.clone(),
            blueprint_id,
            blueprint_name,
            allow_unchecked_attestations,
        )),
        BlueprintSource::Container(fetcher) => {
            DynBlueprintSource::boxed(crate::sources::container::ContainerSource::new(
                fetcher.clone(),
                blueprint_id,
                blueprint_name,
            ))
        }
        BlueprintSource::Remote(fetcher) => DynBlueprintSource::boxed(RemoteBinaryFetcher::new(
            fetcher.clone(),
            blueprint_id,
            blueprint_name,
        )),
    }
}
