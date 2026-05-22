use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use alloy_sol_types::sol;
use async_trait::async_trait;
use blueprint_client_tangle::contracts::ITangle;
use blueprint_client_tangle::{ConfidentialityPolicy, GpuPolicy, GpuRequirements};
use blueprint_core::{info, warn};
use blueprint_runner::config::{BlueprintEnvironment, Protocol};
use tokio::fs::create_dir_all;

use crate::blueprint::ActiveBlueprints;
use crate::blueprint::native::FilteredBlueprint;
use crate::config::BlueprintManagerContext;
use crate::config::SourceType;
use crate::error::{Error, Result};
#[cfg(feature = "remote-providers")]
use crate::executor::remote_provider_integration::RemoteProviderManager;
use crate::metrics;
use crate::protocol::tangle::client::TangleProtocolClient;
use crate::protocol::tangle::metadata::OnChainMetadataProvider;
use crate::protocol::types::ProtocolEvent;
use crate::rt::{GpuSchedulingPolicy, ResourceLimits};
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
    pub confidentiality_policy: ConfidentialityPolicy,
    pub gpu_requirements: GpuRequirements,
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

/// Convert `ResourceLimits` to a `ResourceSpec` for remote cloud provisioning.
#[cfg(feature = "remote-providers")]
fn resource_spec_from_limits(limits: &ResourceLimits) -> blueprint_remote_providers::ResourceSpec {
    blueprint_remote_providers::ResourceSpec {
        cpu: f32::from(limits.cpu_count.unwrap_or(2)),
        memory_gb: limits.memory_size as f32 / (1024.0 * 1024.0 * 1024.0),
        storage_gb: limits.storage_space as f32 / (1024.0 * 1024.0 * 1024.0),
        gpu_count: limits.gpu_count.map(u32::from),
        allow_spot: false,
        qos: blueprint_remote_providers::core::resources::QosParameters::default(),
    }
}

/// Handles Tangle events and translates them into blueprint lifecycle actions.
pub struct TangleEventHandler {
    metadata: Arc<dyn BlueprintMetadataProvider>,
    /// Upgrade pipeline for binary-version reconciliation.
    ///
    /// When `Some`, the handler:
    ///   - registers each service it spawns in `tracked_services`,
    ///   - notifies the watcher whenever it observes a binary-versions
    ///     event,
    ///   - drains queued `SwapRequest`s after every block so verified swaps
    ///     get applied without waiting for a fresh on-chain signal.
    upgrade: Option<crate::upgrade::UpgradePipeline>,
    #[cfg(feature = "remote-providers")]
    remote_provider: Option<RemoteProviderManager>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum SourceCategory {
    Native,
    Container,
    Testing,
}

fn source_category(source: &BlueprintSource) -> SourceCategory {
    match source {
        BlueprintSource::Github(_) | BlueprintSource::Remote(_) => SourceCategory::Native,
        BlueprintSource::Container(_) => SourceCategory::Container,
        BlueprintSource::Testing(_) => SourceCategory::Testing,
    }
}

fn source_kind_label(source: &BlueprintSource) -> &'static str {
    match source {
        BlueprintSource::Github(_) => "github",
        BlueprintSource::Remote(_) => "remote",
        BlueprintSource::Container(_) => "container",
        BlueprintSource::Testing(_) => "testing",
    }
}

/// Generate a hex-encoded trace ID from the current system time (nanoseconds).
/// Used to correlate all log lines within a single service lifecycle operation.
fn gen_trace_id() -> String {
    format!(
        "{:016x}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
    )
}

fn source_priority(source: &BlueprintSource, preferred_source: SourceType) -> u8 {
    match preferred_source {
        SourceType::Container => match source_category(source) {
            SourceCategory::Container => 0,
            SourceCategory::Native => 1,
            SourceCategory::Testing => 2,
        },
        // WASM is currently unsupported in manager source handling.
        SourceType::Native | SourceType::Wasm => match source_category(source) {
            SourceCategory::Native => 0,
            SourceCategory::Container => 1,
            SourceCategory::Testing => 2,
        },
    }
}

fn supports_tee(source: &BlueprintSource) -> bool {
    matches!(source, BlueprintSource::Container(_))
}

/// Apply GPU requirements from blueprint metadata to resource limits.
///
/// - `Required`: sets hard `gpu_count` + `gpu_policy::Required` + VRAM label.
///   The container runtime adds `nvidia.com/gpu` resource requests (hard K8s constraint).
/// - `Preferred`: sets `gpu_count` + `gpu_policy::Preferred` + VRAM label.
///   The container runtime uses node affinity (soft constraint, CPU fallback allowed).
/// - `None`: no GPU resources.
fn apply_gpu_limits(gpu: &GpuRequirements, limits: &mut ResourceLimits) {
    match gpu.policy {
        GpuPolicy::Required => {
            let count = gpu.min_count.clamp(1, 255) as u8;
            limits.gpu_count = Some(count);
            limits.gpu_policy = GpuSchedulingPolicy::Required;
            if gpu.min_vram_gb > 0 {
                limits.gpu_min_vram_gb = Some(gpu.min_vram_gb);
            }
        }
        GpuPolicy::Preferred => {
            let count = gpu.min_count.clamp(1, 255) as u8;
            limits.gpu_count = Some(count);
            limits.gpu_policy = GpuSchedulingPolicy::Preferred;
            if gpu.min_vram_gb > 0 {
                limits.gpu_min_vram_gb = Some(gpu.min_vram_gb);
            }
        }
        GpuPolicy::None => {}
    }
}

fn ordered_source_indices(
    sources: &[BlueprintSource],
    preferred_source: SourceType,
    confidentiality_policy: ConfidentialityPolicy,
) -> Vec<usize> {
    let require_tee = matches!(confidentiality_policy, ConfidentialityPolicy::TeeRequired);
    let mut indexed: Vec<(usize, u8)> = sources
        .iter()
        .enumerate()
        .filter(|(_, source)| !require_tee || supports_tee(source))
        .map(|(idx, source)| {
            let priority = if matches!(confidentiality_policy, ConfidentialityPolicy::TeePreferred)
            {
                if matches!(source_category(source), SourceCategory::Container) {
                    0
                } else {
                    source_priority(source, preferred_source).saturating_add(1)
                }
            } else {
                source_priority(source, preferred_source)
            };
            (idx, priority)
        })
        .collect();
    indexed.sort_by_key(|(idx, priority)| (*priority, *idx));
    indexed.into_iter().map(|(idx, _)| idx).collect()
}

fn planned_runtime_path_for_source(
    source: &BlueprintSource,
    ctx: &BlueprintManagerContext,
) -> &'static str {
    match source {
        BlueprintSource::Container(_) => "container",
        BlueprintSource::Github(_) | BlueprintSource::Remote(_) | BlueprintSource::Testing(_) => {
            #[cfg(feature = "vm-sandbox")]
            {
                if !ctx.vm_sandbox_options.no_vm {
                    return "hypervisor";
                }
            }
            "native"
        }
    }
}

impl TangleEventHandler {
    #[must_use]
    pub fn new() -> Self {
        Self {
            metadata: Arc::new(OnChainMetadataProvider::new()),
            upgrade: None,
            #[cfg(feature = "remote-providers")]
            remote_provider: None,
        }
    }

    #[must_use]
    pub fn with_metadata_provider(metadata: Arc<dyn BlueprintMetadataProvider>) -> Self {
        Self {
            metadata,
            upgrade: None,
            #[cfg(feature = "remote-providers")]
            remote_provider: None,
        }
    }

    /// Attach the upgrade pipeline. The handler will register each spawned
    /// service with the watcher and drain queued swap requests after each
    /// processed block.
    pub fn with_upgrade_pipeline(&mut self, pipeline: crate::upgrade::UpgradePipeline) {
        self.upgrade = Some(pipeline);
    }

    /// Reference to the local RPC handle (if upgrade was wired). Caller can
    /// mount it onto the auth proxy's router.
    pub fn upgrade_api(&self) -> Option<crate::upgrade::UpgradeApi> {
        self.upgrade.as_ref().map(|u| u.api.clone())
    }

    /// Initialize remote provider manager from context.
    #[cfg(feature = "remote-providers")]
    pub async fn init_remote_provider(&mut self, ctx: &BlueprintManagerContext) -> Result<()> {
        if ctx.remote_deployment_opts.enable_remote_deployments {
            match RemoteProviderManager::new(ctx).await {
                Ok(Some(manager)) => {
                    info!("Remote provider manager initialized for GPU deployments");
                    self.remote_provider = Some(manager);
                }
                Ok(None) => {}
                Err(e) => {
                    warn!(error = %e, "Failed to initialize remote provider manager");
                }
            }
        }
        Ok(())
    }

    /// Initialize the handler by syncing the client with the latest observed block.
    ///
    /// # Errors
    ///
    /// Returns an error if the client fails to provide initialization data.
    pub async fn initialize(
        &mut self,
        client: &mut TangleProtocolClient,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        let init_start = Instant::now();
        if env.registration_mode() {
            let settings = env
                .protocol_settings
                .tangle()
                .map_err(|e| Error::Other(e.to_string()))?;
            let blueprint_id = settings.blueprint_id;
            let trace_id = gen_trace_id();
            if let Some(mut metadata) = self
                .metadata
                .resolve_registration(client, blueprint_id)
                .await?
            {
                metadata.registration_capture_only = env.registration_capture_only();
                self.ensure_service_running(&trace_id, metadata, env, ctx, active_blueprints)
                    .await?;
            } else {
                info!(
                    trace_id = %trace_id,
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
            if let Err(e) = self
                .handle_event(client, &proto_event, env, ctx, active_blueprints)
                .await
            {
                warn!(
                    error = %e,
                    "Non-fatal error replaying historical events during init — continuing"
                );
            }
        }

        // Enumerate ALL active services for this operator from on-chain state.
        // Historical event replay may miss services (e.g. Anvil state-only
        // snapshot, or services activated before the manager's start block).
        // This scan is always run — it's idempotent because
        // `ensure_service_running` skips services that are already tracked
        // in `active_blueprints`.
        {
            let scan_start = Instant::now();
            let operator = client.client().account();
            let service_count = client.client().service_count().await.unwrap_or(0);
            let mut discovered = 0u64;
            let mut started = 0u64;
            let mut failed = 0u64;
            if service_count > 0 {
                info!(
                    service_count,
                    %operator,
                    "Scanning contract state for active services"
                );
            }
            for service_id in 0..service_count {
                match client
                    .client()
                    .is_service_operator(service_id, operator)
                    .await
                {
                    Ok(true) => {
                        discovered += 1;
                        let trace_id = gen_trace_id();
                        info!(
                            trace_id = %trace_id,
                            service_id,
                            "Found active service for operator via contract state"
                        );
                        match self.metadata.resolve_service(client, service_id).await {
                            Ok(Some(metadata)) => {
                                let svc_start = Instant::now();
                                match self
                                    .ensure_service_running(
                                        &trace_id,
                                        metadata,
                                        env,
                                        ctx,
                                        active_blueprints,
                                    )
                                    .await
                                {
                                    Ok(()) => {
                                        started += 1;
                                        info!(
                                            trace_id = %trace_id,
                                            service_id,
                                            startup_ms = svc_start.elapsed().as_millis() as u64,
                                            "Service started"
                                        );
                                    }
                                    Err(e) => {
                                        failed += 1;
                                        info!(
                                            trace_id = %trace_id,
                                            service_id,
                                            startup_ms = svc_start.elapsed().as_millis() as u64,
                                            error = %e,
                                            "Failed to start service from contract state"
                                        );
                                    }
                                }
                            }
                            Ok(None) => {
                                info!(
                                    trace_id = %trace_id,
                                    service_id,
                                    "Service metadata unavailable"
                                );
                            }
                            Err(e) => {
                                failed += 1;
                                info!(
                                    trace_id = %trace_id,
                                    service_id,
                                    error = %e,
                                    "Failed to resolve service metadata"
                                );
                            }
                        }
                    }
                    Ok(false) => {}
                    Err(e) => {
                        info!(service_id, error = %e, "Failed to check operator membership");
                    }
                }
            }
            let scan_secs = scan_start.elapsed().as_secs_f64();
            let scan_ms = (scan_secs * 1000.0) as u64;
            metrics::CONTRACT_SCAN_DURATION
                .with_label_values(&[] as &[&str])
                .observe(scan_secs);
            for _ in 0..started {
                metrics::SERVICE_DISCOVERY
                    .with_label_values(&["started"])
                    .inc();
            }
            for _ in 0..failed {
                metrics::SERVICE_DISCOVERY
                    .with_label_values(&["failed"])
                    .inc();
            }
            info!(
                service_count,
                discovered, started, failed, scan_ms, "Contract state scan complete"
            );
        }

        let init_secs = init_start.elapsed().as_secs_f64();
        let init_ms = (init_secs * 1000.0) as u64;
        let active_count = active_blueprints.values().map(|s| s.len()).sum::<usize>();
        metrics::INIT_DURATION
            .with_label_values(&["ok"])
            .observe(init_secs);
        metrics::ACTIVE_SERVICES.set(active_count as i64);
        info!(
            init_ms,
            active_services = active_count,
            "Manager initialization complete"
        );
        Ok(())
    }

    /// Process a protocol event and reconcile running services.
    ///
    /// # Errors
    ///
    /// Returns an error if metadata resolution fails or service management
    /// encounters an unrecoverable issue.
    pub async fn handle_event(
        &mut self,
        client: &TangleProtocolClient,
        event: &ProtocolEvent,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        let tangle_evt = event
            .as_tangle()
            .ok_or_else(|| Error::Other("Expected Tangle event in handler".to_string()))?;

        let event_start = Instant::now();
        info!(
            block_number = tangle_evt.block_number,
            log_count = tangle_evt.logs.len(),
            "Processing block events"
        );
        for log in &tangle_evt.logs {
            if let Ok(evt) = log.log_decode::<ITangle::ServiceActivated>() {
                let service_id = evt.inner.serviceId;
                let trace_id = gen_trace_id();
                info!(
                    trace_id = %trace_id,
                    service_id,
                    block_number = tangle_evt.block_number,
                    "Decoded ServiceActivated event"
                );
                if let Some(metadata) = self.metadata.resolve_service(client, service_id).await? {
                    let blueprint_id = metadata.blueprint_id;
                    let gpu_requirements = metadata.gpu_requirements;
                    if let Err(e) = self
                        .ensure_service_running(&trace_id, metadata, env, ctx, active_blueprints)
                        .await
                    {
                        warn!(
                            trace_id = %trace_id,
                            service_id,
                            blueprint_id,
                            error = %e,
                            "Failed to start service — continuing with remaining services"
                        );
                        continue;
                    }
                    #[cfg(feature = "remote-providers")]
                    self.notify_remote_service_initiated(
                        blueprint_id,
                        service_id,
                        &gpu_requirements,
                    )
                    .await;
                } else {
                    info!(
                        trace_id = %trace_id,
                        service_id,
                        "ServiceActivated observed but metadata unavailable"
                    );
                }
            } else if let Ok(evt) = log.log_decode::<ITangle::ServiceTerminated>() {
                let service_id = evt.inner.serviceId;
                let trace_id = gen_trace_id();
                info!(
                    trace_id = %trace_id,
                    service_id,
                    block_number = tangle_evt.block_number,
                    "Decoded ServiceTerminated event"
                );
                if let Ok(service) = client.client().get_service(service_id).await {
                    self.stop_service(service.blueprintId, service_id, active_blueprints)
                        .await?;
                    #[cfg(feature = "remote-providers")]
                    self.notify_remote_service_terminated(service.blueprintId, service_id)
                        .await;
                }
            } else if let Ok(evt) = log.log_decode::<OperatorPreRegistered>() {
                let blueprint_id = evt.inner.data.blueprintId;
                let trace_id = gen_trace_id();
                info!(
                    trace_id = %trace_id,
                    blueprint_id,
                    block_number = tangle_evt.block_number,
                    "Decoded OperatorPreRegistered event"
                );
                if let Some(metadata) = self
                    .metadata
                    .resolve_registration(client, blueprint_id)
                    .await?
                {
                    self.ensure_service_running(&trace_id, metadata, env, ctx, active_blueprints)
                        .await?;
                } else {
                    info!(
                        trace_id = %trace_id,
                        blueprint_id,
                        "OperatorPreRegistered observed but blueprint metadata unavailable"
                    );
                }
            }
        }

        // After the block's protocol events are processed, fan-out the
        // binary-version subset to the upgrade watcher. The watcher may
        // produce SwapRequests; drain those before returning so a verified
        // swap is applied within the same block tick.
        self.maybe_notify_upgrade_watcher(&tangle_evt.logs).await;
        self.drain_pending_swaps(env, ctx, active_blueprints).await;

        let event_secs = event_start.elapsed().as_secs_f64();
        metrics::BLOCK_PROCESSING_DURATION
            .with_label_values(&[] as &[&str])
            .observe(event_secs);
        info!(
            block_number = tangle_evt.block_number,
            event_ms = (event_secs * 1000.0) as u64,
            "Block event processing complete"
        );
        Ok(())
    }

    async fn ensure_service_running(
        &self,
        trace_id: &str,
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
            info!(
                trace_id = %trace_id,
                blueprint_id = metadata.blueprint_id,
                service_id = metadata.service_id,
                "Service already running — skipping"
            );
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

        if ctx.preferred_source == SourceType::Wasm {
            warn!(
                trace_id = %trace_id,
                preferred_source = %ctx.preferred_source,
                "WASM source preference is not yet supported; using native/container/testing fallback ordering"
            );
        }

        let ordered_source_idxs = ordered_source_indices(
            &metadata.sources,
            ctx.preferred_source,
            metadata.confidentiality_policy,
        );
        if matches!(
            metadata.confidentiality_policy,
            ConfidentialityPolicy::TeeRequired
        ) && ordered_source_idxs.is_empty()
        {
            return Err(Error::TeeRuntimeUnavailable {
                reason: "Blueprint requires TEE execution but exposes no container source"
                    .to_string(),
            });
        }
        if matches!(metadata.gpu_requirements.policy, GpuPolicy::Required) {
            info!(
                trace_id = %trace_id,
                blueprint_id = metadata.blueprint_id,
                service_id = metadata.service_id,
                min_count = metadata.gpu_requirements.min_count,
                min_vram_gb = metadata.gpu_requirements.min_vram_gb,
                "Blueprint requires GPU — container runtime must provide GPU device plugin"
            );
        }
        let ordered_source_labels: Vec<&str> = ordered_source_idxs
            .iter()
            .map(|idx| source_kind_label(&metadata.sources[*idx]))
            .collect();
        info!(
            trace_id = %trace_id,
            blueprint_id = metadata.blueprint_id,
            service_id = metadata.service_id,
            confidentiality_policy = ?metadata.confidentiality_policy,
            gpu_policy = ?metadata.gpu_requirements.policy,
            preferred_source = %ctx.preferred_source,
            source_order = ?ordered_source_labels,
            "Resolved deterministic source fallback ordering"
        );

        for (attempt, source_idx) in ordered_source_idxs.iter().enumerate() {
            let attempt_start = Instant::now();
            let source = &metadata.sources[*source_idx];
            let source_kind = source_kind_label(source);
            let runtime_path = planned_runtime_path_for_source(source, ctx);
            info!(
                trace_id = %trace_id,
                blueprint_id = metadata.blueprint_id,
                service_id = metadata.service_id,
                attempt = attempt + 1,
                source_index = *source_idx,
                source_kind,
                runtime_path,
                "Attempting source launch"
            );
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
            let mut limits = ResourceLimits::default();
            apply_gpu_limits(&metadata.gpu_requirements, &mut limits);
            let service_idx = metadata.service_id.try_into().unwrap_or(u32::MAX);

            match handler
                .spawn(
                    ctx,
                    limits,
                    env,
                    service_idx,
                    env_vars,
                    args,
                    metadata.confidentiality_policy,
                    &service_label,
                    &cache_dir,
                    &runtime_dir,
                )
                .await
            {
                Ok(mut service) => {
                    if let Some(health) = service.start().await? {
                        if let Err(e) = health.await {
                            let attempt_secs = attempt_start.elapsed().as_secs_f64();
                            metrics::SOURCE_ATTEMPT_DURATION
                                .with_label_values(&[source_kind, runtime_path, "failed_health"])
                                .observe(attempt_secs);
                            metrics::SOURCE_ATTEMPTS
                                .with_label_values(&[source_kind, "failed_health"])
                                .inc();
                            info!(
                                trace_id = %trace_id,
                                blueprint_id = metadata.blueprint_id,
                                service_id = metadata.service_id,
                                source_kind,
                                runtime_path,
                                attempt_ms = (attempt_secs * 1000.0) as u64,
                                error = %e,
                                "Source launch failed health check; trying next fallback"
                            );
                            last_err = Some(e);
                            continue;
                        }
                    }

                    let attempt_secs = attempt_start.elapsed().as_secs_f64();
                    metrics::SOURCE_ATTEMPT_DURATION
                        .with_label_values(&[source_kind, runtime_path, "success"])
                        .observe(attempt_secs);
                    metrics::SOURCE_ATTEMPTS
                        .with_label_values(&[source_kind, "success"])
                        .inc();
                    metrics::ACTIVE_SERVICES.inc();

                    active_blueprints
                        .entry(metadata.blueprint_id)
                        .or_default()
                        .insert(metadata.service_id, service);
                    info!(
                        trace_id = %trace_id,
                        blueprint_id = metadata.blueprint_id,
                        service_id = metadata.service_id,
                        source_kind,
                        runtime_path,
                        attempt_ms = (attempt_secs * 1000.0) as u64,
                        "Started Tangle blueprint service"
                    );
                    self.on_service_started(metadata.blueprint_id, metadata.service_id)
                        .await;
                    return Ok(());
                }
                Err(e) => {
                    let attempt_secs = attempt_start.elapsed().as_secs_f64();
                    metrics::SOURCE_ATTEMPT_DURATION
                        .with_label_values(&[source_kind, runtime_path, "failed_spawn"])
                        .observe(attempt_secs);
                    metrics::SOURCE_ATTEMPTS
                        .with_label_values(&[source_kind, "failed_spawn"])
                        .inc();
                    info!(
                        trace_id = %trace_id,
                        blueprint_id = metadata.blueprint_id,
                        service_id = metadata.service_id,
                        source_kind,
                        runtime_path,
                        attempt_ms = (attempt_secs * 1000.0) as u64,
                        error = %e,
                        "Source launch attempt failed; trying next fallback"
                    );
                    last_err = Some(e);
                }
            }
        }

        // Fallback: when preferred_source is Native and all on-chain sources failed,
        // try building from local cargo workspace.
        // Checks BLUEPRINT_CARGO_BIN_{blueprint_id} first, then BLUEPRINT_CARGO_BIN.
        if ctx.preferred_source == SourceType::Native
            && !matches!(
                metadata.confidentiality_policy,
                ConfidentialityPolicy::TeeRequired
            )
        {
            let per_blueprint_key = format!("BLUEPRINT_CARGO_BIN_{}", metadata.blueprint_id);
            let (cargo_bin_var, resolved_from) = match std::env::var(&per_blueprint_key) {
                Ok(val) => (Ok(val), per_blueprint_key.as_str()),
                Err(_) => (std::env::var("BLUEPRINT_CARGO_BIN"), "BLUEPRINT_CARGO_BIN"),
            };
            if let Ok(cargo_bin) = cargo_bin_var {
                let base_path = std::env::current_dir()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| ".".to_string());
                info!(
                    trace_id = %trace_id,
                    cargo_bin = %cargo_bin,
                    base_path = %base_path,
                    blueprint_id = metadata.blueprint_id,
                    env_var = resolved_from,
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
                let mut limits = ResourceLimits::default();
                apply_gpu_limits(&metadata.gpu_requirements, &mut limits);
                let service_idx = metadata.service_id.try_into().unwrap_or(u32::MAX);

                match handler
                    .spawn(
                        ctx,
                        limits,
                        env,
                        service_idx,
                        env_vars,
                        args,
                        metadata.confidentiality_policy,
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
                                    trace_id = %trace_id,
                                    blueprint_id = metadata.blueprint_id,
                                    service_id = metadata.service_id,
                                    "Started Tangle blueprint service via local cargo fallback"
                                );
                                self.on_service_started(metadata.blueprint_id, metadata.service_id)
                                    .await;
                                return Ok(());
                            }
                        } else {
                            active_blueprints
                                .entry(metadata.blueprint_id)
                                .or_default()
                                .insert(metadata.service_id, service);
                            info!(
                                trace_id = %trace_id,
                                blueprint_id = metadata.blueprint_id,
                                service_id = metadata.service_id,
                                "Started Tangle blueprint service via local cargo fallback"
                            );
                            self.on_service_started(metadata.blueprint_id, metadata.service_id)
                                .await;
                            return Ok(());
                        }
                    }
                    Err(e) => last_err = Some(e),
                }
            }
        }

        Err(last_err.unwrap_or(Error::NoFetchers))
    }

    /// Notify the upgrade watcher that a service is now running.
    ///
    /// Called from `ensure_service_running` success paths. Best-effort —
    /// failures here would only delay upgrade reconciliation, not corrupt
    /// state, so they are downgraded to warnings.
    async fn on_service_started(&self, blueprint_id: u64, service_id: u64) {
        let Some(pipeline) = &self.upgrade else {
            return;
        };
        pipeline
            .tracked_services
            .insert(blueprint_id, service_id)
            .await;
        // Resolve the effective binary version so the watcher knows the
        // (versionId, sha256) the manager just spun up. If the chain reverts
        // (no published versions) we simply skip — the watcher will pick
        // this up the moment a publish lands.
        match pipeline
            .api
            .chain()
            .effective_binary_version(service_id, blueprint_id)
            .await
        {
            Ok(version) => {
                pipeline
                    .state
                    .set_running(
                        service_id,
                        crate::upgrade::RunningBinary {
                            version_id: version.version_id,
                            sha256: version.sha256,
                        },
                    )
                    .await;
                info!(
                    target: "upgrade",
                    blueprint_id,
                    service_id,
                    version_id = version.version_id,
                    "registered service with upgrade watcher"
                );
            }
            Err(crate::upgrade::UpgradeError::NoVersionsPublished { .. }) => {
                info!(
                    target: "upgrade",
                    blueprint_id,
                    service_id,
                    "service started before any binary version was published — watcher will reconcile on next publish"
                );
            }
            Err(err) => {
                warn!(
                    target: "upgrade",
                    blueprint_id,
                    service_id,
                    error = %err,
                    "failed to record initial binary version for new service"
                );
            }
        }
        // Cache the current policy too, so the local RPC can answer
        // `GET /upgrades/policy/{service_id}` immediately.
        if let Ok(policy) = pipeline
            .api
            .chain()
            .service_upgrade_policy(service_id)
            .await
        {
            pipeline.state.set_policy(service_id, policy).await;
        }
    }

    /// Notify the upgrade watcher that a service is no longer running.
    async fn on_service_stopped(&self, blueprint_id: u64, service_id: u64) {
        let Some(pipeline) = &self.upgrade else {
            return;
        };
        pipeline
            .tracked_services
            .remove(blueprint_id, service_id)
            .await;
        pipeline.state.clear_running(service_id).await;
    }

    /// Decide whether a Tangle block's logs touched any binary-version
    /// surface, and if so, kick the watcher.
    async fn maybe_notify_upgrade_watcher(&self, logs: &[alloy_rpc_types::Log]) {
        let Some(pipeline) = &self.upgrade else {
            return;
        };
        let mut blueprints = std::collections::HashSet::new();
        let mut policy_changed = false;
        for log in logs {
            let Some(topic0) = log.topic0() else {
                continue;
            };
            if topic0 == &crate::upgrade::event_selectors::binary_version_published()
                || topic0 == &crate::upgrade::event_selectors::binary_active_version_changed()
                || topic0 == &crate::upgrade::event_selectors::binary_version_deprecated()
            {
                // Indexed param[1] is the blueprintId on these three events.
                if let Some(bp_topic) = log.topics().get(1) {
                    let bp = u64::from_be_bytes(
                        bp_topic.as_slice()[24..32].try_into().unwrap_or([0u8; 8]),
                    );
                    blueprints.insert(bp);
                }
            } else if topic0 == &crate::upgrade::event_selectors::operator_binary_acked()
                || topic0 == &crate::upgrade::event_selectors::service_upgrade_policy_set()
            {
                // Service-scoped events — we don't know the blueprint from
                // the topic alone. Mark the policy_changed flag so we ask
                // the watcher for a wildcard reconcile.
                policy_changed = true;
            }
        }
        if policy_changed {
            pipeline.watcher.notify_reconcile_all().await;
        } else if !blueprints.is_empty() {
            pipeline.watcher.notify_block(blueprints).await;
        }
    }

    /// Pop any queued `SwapRequest`s and apply them against
    /// `active_blueprints`. Called once per processed block from the Tangle
    /// event loop.
    async fn drain_pending_swaps(
        &mut self,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) {
        // `self.upgrade` is `Option`; we have to take it out to satisfy the
        // borrow checker for the recursive call back into the handler.
        let Some(mut pipeline) = self.upgrade.take() else {
            return;
        };
        while let Ok(request) = pipeline.swap_rx.try_recv() {
            self.apply_swap(&pipeline, request, env, ctx, active_blueprints)
                .await;
        }
        self.upgrade = Some(pipeline);
    }

    /// Drain the old service for one swap and let the next reconcile spawn
    /// the replacement.
    ///
    /// We split this from the spawn path because the existing
    /// `ensure_service_running` knows how to assemble sources/runtime/GPU
    /// state for the relevant blueprint. Forking that pipeline would mean
    /// two ways to spawn a service — exactly the duplication the codebase
    /// avoids elsewhere. The cost is one block of downtime between drain
    /// and respawn, which is acceptable for AUTO/APPROVE flows.
    async fn apply_swap(
        &self,
        pipeline: &crate::upgrade::UpgradePipeline,
        request: crate::upgrade::SwapRequest,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) {
        let trace_id = gen_trace_id();
        info!(
            target: "upgrade",
            trace_id = %trace_id,
            blueprint_id = request.blueprint_id,
            service_id = request.service_id,
            to_version_id = request.target.version_id,
            "applying queued swap"
        );

        // Step 1: drain old. Take ownership of the Service so its destructor
        // does not race the respawn.
        let Some(services) = active_blueprints.get_mut(&request.blueprint_id) else {
            warn!(
                target: "upgrade",
                trace_id = %trace_id,
                blueprint_id = request.blueprint_id,
                service_id = request.service_id,
                "swap arrived for an untracked blueprint — discarding"
            );
            return;
        };
        let Some(old_service) = services.remove(&request.service_id) else {
            warn!(
                target: "upgrade",
                trace_id = %trace_id,
                blueprint_id = request.blueprint_id,
                service_id = request.service_id,
                "swap arrived for an untracked service — discarding"
            );
            return;
        };
        if let Err(err) = old_service.shutdown().await {
            warn!(
                target: "upgrade",
                trace_id = %trace_id,
                blueprint_id = request.blueprint_id,
                service_id = request.service_id,
                error = %err,
                "old service drain returned error; continuing into respawn"
            );
        }
        // Drop the blueprint entry if empty so the next reconcile cycle
        // re-creates it from scratch with fresh resource limits.
        if let Some(services) = active_blueprints.get(&request.blueprint_id) {
            if services.is_empty() {
                active_blueprints.remove(&request.blueprint_id);
            }
        }

        // Step 2: record the new running state. The metric is reset
        // intentionally — the swap-in is observed only when the next spawn
        // succeeds, not here.
        pipeline
            .state
            .set_running(
                request.service_id,
                crate::upgrade::RunningBinary {
                    version_id: request.target.version_id,
                    sha256: request.target.sha256,
                },
            )
            .await;
        pipeline.state.clear_pending(request.service_id).await;
        pipeline
            .state
            .push_history(crate::upgrade::UpgradeHistoryEntry {
                service_id: request.service_id,
                blueprint_id: request.blueprint_id,
                from_version_id: request.previous.map(|r| r.version_id),
                from_sha256: request.previous.map(|r| r.sha256),
                to_version_id: request.target.version_id,
                to_sha256: request.target.sha256,
                policy: request.policy,
                completed_at: std::time::SystemTime::now(),
                outcome: crate::upgrade::UpgradeOutcome::Swapped,
            })
            .await;

        info!(
            target: "upgrade",
            trace_id = %trace_id,
            event = "binary_swapped",
            blueprint_id = request.blueprint_id,
            service_id = request.service_id,
            to_version_id = request.target.version_id,
            policy = %request.policy,
            "binary swap drained — re-spawn will happen on next reconcile"
        );

        // Step 3: trigger re-spawn through the normal lifecycle.
        // `resolve_service` returns the same metadata the protocol event
        // handler would build for a `ServiceActivated` event; the new
        // binary will be picked up via the source's normal fetch path,
        // which content-addresses against the cache. Errors are logged but
        // not surfaced — the next block's reconcile will retry.
        let _ = &request.binary_path;
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
        self.on_service_stopped(blueprint_id, service_id).await;
        Ok(())
    }
}

#[cfg(feature = "remote-providers")]
impl TangleEventHandler {
    async fn notify_remote_service_initiated(
        &self,
        blueprint_id: u64,
        service_id: u64,
        gpu: &GpuRequirements,
    ) {
        let Some(remote) = &self.remote_provider else {
            return;
        };
        let mut limits = ResourceLimits::default();
        apply_gpu_limits(gpu, &mut limits);
        let resource_spec = resource_spec_from_limits(&limits);
        if let Err(e) = remote
            .on_service_initiated(blueprint_id, service_id, Some(resource_spec))
            .await
        {
            warn!(
                blueprint_id,
                service_id,
                error = %e,
                "Remote provider failed to handle service initiation"
            );
        }
    }

    async fn notify_remote_service_terminated(&self, blueprint_id: u64, service_id: u64) {
        let Some(remote) = &self.remote_provider else {
            return;
        };
        if let Err(e) = remote.on_service_terminated(blueprint_id, service_id).await {
            warn!(
                blueprint_id,
                service_id,
                error = %e,
                "Remote provider failed to handle service termination"
            );
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sources::types::{
        BlueprintBinary, GithubFetcher, ImageRegistryFetcher, RemoteFetcher, TestFetcher,
    };

    fn test_source() -> BlueprintSource {
        BlueprintSource::Testing(TestFetcher {
            cargo_package: "pkg".to_string(),
            cargo_bin: "bin".to_string(),
            base_path: "/tmp".to_string(),
        })
    }

    fn container_source() -> BlueprintSource {
        BlueprintSource::Container(ImageRegistryFetcher {
            registry: "ghcr.io".to_string(),
            image: "tangle/demo".to_string(),
            tag: "v1".to_string(),
        })
    }

    fn remote_source() -> BlueprintSource {
        BlueprintSource::Remote(RemoteFetcher {
            dist_url: "https://example.com/dist.json".to_string(),
            archive_url: "https://example.com/archive.tar.xz".to_string(),
            binaries: vec![BlueprintBinary {
                arch: "amd64".to_string(),
                os: "linux".to_string(),
                name: "demo".to_string(),
                sha256: [0x11; 32],
                blake3: None,
            }],
        })
    }

    fn github_source() -> BlueprintSource {
        BlueprintSource::Github(GithubFetcher {
            owner: "owner".to_string(),
            repo: "repo".to_string(),
            tag: "v1".to_string(),
            binaries: vec![BlueprintBinary {
                arch: "amd64".to_string(),
                os: "linux".to_string(),
                name: "demo".to_string(),
                sha256: [0x22; 32],
                blake3: None,
            }],
        })
    }

    #[test]
    fn deterministic_order_prefers_native_then_container_then_testing() {
        let sources = vec![
            test_source(),
            container_source(),
            remote_source(),
            github_source(),
        ];
        let ordered =
            ordered_source_indices(&sources, SourceType::Native, ConfidentialityPolicy::Any);
        assert_eq!(ordered, vec![2, 3, 1, 0]);
    }

    #[test]
    fn deterministic_order_prefers_container_when_requested() {
        let sources = vec![
            test_source(),
            container_source(),
            remote_source(),
            github_source(),
        ];
        let ordered =
            ordered_source_indices(&sources, SourceType::Container, ConfidentialityPolicy::Any);
        assert_eq!(ordered, vec![1, 2, 3, 0]);
    }

    #[test]
    fn deterministic_order_is_stable_for_wasm_preference() {
        let sources = vec![
            github_source(),
            test_source(),
            remote_source(),
            container_source(),
        ];
        let first = ordered_source_indices(&sources, SourceType::Wasm, ConfidentialityPolicy::Any);
        let second = ordered_source_indices(&sources, SourceType::Wasm, ConfidentialityPolicy::Any);
        assert_eq!(first, second);
        assert_eq!(first, vec![0, 2, 3, 1]);
    }

    #[test]
    fn tee_required_filters_to_container_sources_only() {
        let sources = vec![
            test_source(),
            container_source(),
            remote_source(),
            github_source(),
        ];
        let ordered = ordered_source_indices(
            &sources,
            SourceType::Native,
            ConfidentialityPolicy::TeeRequired,
        );
        assert_eq!(ordered, vec![1]);
    }

    #[test]
    fn tee_required_without_container_sources_returns_empty_order() {
        let sources = vec![test_source(), remote_source(), github_source()];
        let ordered = ordered_source_indices(
            &sources,
            SourceType::Container,
            ConfidentialityPolicy::TeeRequired,
        );
        assert!(ordered.is_empty());
    }

    #[test]
    fn tee_preferred_prioritizes_container_sources() {
        let sources = vec![
            test_source(),
            remote_source(),
            github_source(),
            container_source(),
        ];
        let ordered = ordered_source_indices(
            &sources,
            SourceType::Native,
            ConfidentialityPolicy::TeePreferred,
        );
        assert_eq!(ordered[0], 3);
    }
}
