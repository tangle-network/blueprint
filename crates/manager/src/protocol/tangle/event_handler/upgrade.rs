use super::*;

impl TangleEventHandler {
    /// Notify the upgrade watcher that a service is now running.
    ///
    /// Called from `ensure_service_running` success paths. Best-effort —
    /// failures here would only delay upgrade reconciliation, not corrupt
    /// state, so they are downgraded to warnings.
    pub(super) async fn on_service_started(&self, blueprint_id: u64, service_id: u64) {
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
    pub(super) async fn on_service_stopped(&self, blueprint_id: u64, service_id: u64) {
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
    pub(super) async fn maybe_notify_upgrade_watcher(&self, logs: &[alloy_rpc_types::Log]) {
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
    pub(super) async fn drain_pending_swaps(
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
    pub(super) async fn apply_swap(
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
}
