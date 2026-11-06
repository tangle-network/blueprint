use super::*;

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

fn source_kind_label(source: &BlueprintSource) -> &'static str {
    match source {
        BlueprintSource::Github(_) => "github",
        BlueprintSource::Remote(_) => "remote",
        BlueprintSource::Container(_) => "container",
        BlueprintSource::Testing(_) => "testing",
    }
}

impl TangleEventHandler {
    pub(super) async fn ensure_service_running(
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
            let limits = ResourceLimits::from(&metadata.gpu_requirements);
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
                let limits = ResourceLimits::from(&metadata.gpu_requirements);
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
                            health.await?;
                        }
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
                    Err(e) => last_err = Some(e),
                }
            }
        }
        Err(last_err.unwrap_or(Error::NoFetchers))
    }
}

fn build_source_handler(
    source: &BlueprintSource,
    blueprint_id: u64,
    blueprint_name: String,
    allow_unchecked_attestations: bool,
) -> Box<DynBlueprintSource<'static>> {
    match source {
        BlueprintSource::Testing(fetcher) => DynBlueprintSource::new_box(TestSourceFetcher::new(
            fetcher.clone(),
            blueprint_id,
            blueprint_name,
        )),
        BlueprintSource::Github(fetcher) => DynBlueprintSource::new_box(GithubBinaryFetcher::new(
            fetcher.clone(),
            blueprint_id,
            blueprint_name,
            allow_unchecked_attestations,
        )),
        BlueprintSource::Container(fetcher) => {
            DynBlueprintSource::new_box(crate::sources::container::ContainerSource::new(
                fetcher.clone(),
                blueprint_id,
                blueprint_name,
            ))
        }
        BlueprintSource::Remote(fetcher) => DynBlueprintSource::new_box(RemoteBinaryFetcher::new(
            fetcher.clone(),
            blueprint_id,
            blueprint_name,
        )),
    }
}
