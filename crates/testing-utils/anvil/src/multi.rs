//! Multi-blueprint harness for testing interactions between cooperating blueprints.
//!
//! [`MultiHarness`] deploys multiple blueprints on a shared Anvil instance,
//! each with its own [`Router`], service ID, and operator fleet.  This enables
//! full end-to-end tests that exercise cross-blueprint communication (e.g. a
//! trading blueprint that consumes validation results from a separate validator
//! blueprint).
//!
//! # Example
//!
//! ```rust,ignore
//! use blueprint_anvil_testing_utils::{OperatorFleet, OperatorSpec};
//!
//! let multi = MultiHarness::builder()
//!     .add_blueprint("trading", trading_router(), 0)
//!     .add_blueprint_with_operators("validator", validator_router(), 1,
//!         OperatorFleet::<3, 0>::new([
//!             OperatorSpec::honest("val-0", KEY_0),
//!             OperatorSpec::honest("val-1", KEY_1),
//!             OperatorSpec::honest("val-2", KEY_2),
//!         ]),
//!     )
//!     .spawn()
//!     .await?;
//!
//! let trading = multi.handle("trading").unwrap();
//! let validator = multi.handle("validator").unwrap();
//!
//! // Submit jobs to each blueprint independently
//! let sub = trading.submit_job(0, payload).await?;
//! ```

use crate::blueprint::{build_operator_runtimes, OperatorSpec};
use crate::{
    LOCAL_BLUEPRINT_ID, SeededTangleTestnet, seed_operator_key, start_tangle_testnet,
};
use alloy_primitives::Bytes;
use anyhow::{Context, Result};
use blueprint_client_tangle::{
    JobSubmissionResult, TangleClient, TangleClientConfig, TangleSettings,
};
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_router::Router;
use blueprint_runner::config::{BlueprintEnvironment, ProtocolSettings};
use blueprint_runner::error::RunnerError;
use blueprint_runner::{BlueprintConfig, BlueprintRunner};
use blueprint_runner::tangle::config::TangleProtocolSettings;
use blueprint_std::collections::{BTreeMap, VecDeque};
use blueprint_tangle_extra::TangleProducer;
use hex::FromHex;
pub use crate::blueprint::{OperatorFleet, OperatorBehaviorRef, HonestOperator};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tempfile::TempDir;
use tokio::sync::Notify;
use tokio::task::JoinHandle;
use tokio::time::Duration;

use crate::blueprint::SERVICE_OWNER_PRIVATE_KEY;

/// Specification for a single blueprint within a [`MultiHarness`].
struct BlueprintSpec {
    name: String,
    router: Router,
    service_id: u64,
    blueprint_id: u64,
    poll_interval: Duration,
    operator_specs: Option<Vec<OperatorSpec>>,
}

/// Builder for [`MultiHarness`].
pub struct MultiHarnessBuilder {
    specs: Vec<BlueprintSpec>,
    include_anvil_logs: bool,
    env_vars: Vec<(String, String)>,
    state_dir_env: Option<String>,
}

impl MultiHarnessBuilder {
    /// Create a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            specs: Vec::new(),
            include_anvil_logs: false,
            env_vars: Vec::new(),
            state_dir_env: None,
        }
    }

    /// Add a blueprint to the multi-harness.
    ///
    /// Each blueprint gets its own [`Router`] and service ID.  The `name` is
    /// used to look up the [`BlueprintHandle`] after spawning.
    #[must_use]
    pub fn add_blueprint(
        mut self,
        name: impl Into<String>,
        router: Router,
        service_id: u64,
    ) -> Self {
        self.specs.push(BlueprintSpec {
            name: name.into(),
            router,
            service_id,
            blueprint_id: LOCAL_BLUEPRINT_ID,
            poll_interval: Duration::from_millis(100),
            operator_specs: None,
        });
        self
    }

    /// Add a blueprint with a custom operator fleet.
    ///
    /// Each operator gets its own keystore and [`TangleConsumer`] sink, enabling
    /// multi-operator result submission and Byzantine fault simulation.
    #[must_use]
    pub fn add_blueprint_with_operators<const N: usize, const F: usize>(
        mut self,
        name: impl Into<String>,
        router: Router,
        service_id: u64,
        fleet: OperatorFleet<N, F>,
    ) -> Self {
        self.specs.push(BlueprintSpec {
            name: name.into(),
            router,
            service_id,
            blueprint_id: LOCAL_BLUEPRINT_ID,
            poll_interval: Duration::from_millis(100),
            operator_specs: Some(fleet.into_vec()),
        });
        self
    }

    /// Add a blueprint with custom blueprint_id, poll_interval, and optional operator fleet.
    #[must_use]
    pub fn add_blueprint_with_config(
        mut self,
        name: impl Into<String>,
        router: Router,
        service_id: u64,
        blueprint_id: u64,
        poll_interval: Duration,
        operator_specs: Option<Vec<OperatorSpec>>,
    ) -> Self {
        self.specs.push(BlueprintSpec {
            name: name.into(),
            router,
            service_id,
            blueprint_id,
            poll_interval,
            operator_specs,
        });
        self
    }

    /// Enable or disable Anvil stdout logs.
    #[must_use]
    pub fn include_anvil_logs(mut self, include: bool) -> Self {
        self.include_anvil_logs = include;
        self
    }

    /// Set environment variables for the multi-harness lifetime.
    #[must_use]
    pub fn with_env_vars(mut self, vars: impl IntoIterator<Item = (String, String)>) -> Self {
        self.env_vars.extend(vars);
        self
    }

    /// Point a state-directory env var at the harness temp directory.
    #[must_use]
    pub fn with_state_dir_env(mut self, env_var_name: impl Into<String>) -> Self {
        self.state_dir_env = Some(env_var_name.into());
        self
    }

    /// Spawn the multi-harness.
    pub async fn spawn(self) -> Result<MultiHarness> {
        MultiHarness::spawn(self).await
    }
}

impl Default for MultiHarnessBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle for interacting with a single blueprint within a [`MultiHarness`].
pub struct BlueprintHandle {
    name: String,
    caller_client: Arc<TangleClient>,
    event_client: Arc<TangleClient>,
    local_results: Arc<Mutex<VecDeque<Vec<u8>>>>,
    local_notify: Arc<Notify>,
    service_id: u64,
    blueprint_id: u64,
}

impl BlueprintHandle {
    /// Submit a job to this blueprint.
    pub async fn submit_job(&self, job_index: u8, payload: Bytes) -> Result<JobSubmissionResult> {
        self.caller_client
            .submit_job(self.service_id, job_index, payload)
            .await
            .context("failed to submit job")
    }

    /// Wait for a job result with a custom timeout.
    pub async fn wait_for_job_result_with_deadline(
        &self,
        submission: JobSubmissionResult,
        timeout_duration: Duration,
    ) -> Result<Vec<u8>> {
        let local_results = Arc::clone(&self.local_results);
        let local_notify = Arc::clone(&self.local_notify);
        let event_client = Arc::clone(&self.event_client);
        let service_id = self.service_id;

        let local_wait = async {
            loop {
                let notified = local_notify.notified();
                if let Some(output) = local_results.lock().unwrap().pop_front() {
                    return output;
                }
                notified.await;
                if let Some(output) = local_results.lock().unwrap().pop_front() {
                    return output;
                }
            }
        };

        let on_chain_wait = wait_for_on_chain_result(event_client, submission, service_id);

        let fut = async {
            tokio::select! {
                output = local_wait => Ok(output),
                output = on_chain_wait => output,
            }
        };

        tokio::time::timeout(timeout_duration, fut)
            .await
            .context("timed out waiting for job result")?
    }

    /// Wait for a job result with the default 30s timeout.
    pub async fn wait_for_job_result(&self, submission: JobSubmissionResult) -> Result<Vec<u8>> {
        self.wait_for_job_result_with_deadline(submission, Duration::from_secs(30))
            .await
    }

    /// Blueprint name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Service ID for this blueprint.
    pub fn service_id(&self) -> u64 {
        self.service_id
    }

    /// Blueprint ID.
    pub fn blueprint_id(&self) -> u64 {
        self.blueprint_id
    }
}

/// Multi-blueprint harness that deploys several blueprints on the same Anvil
/// instance.
///
/// Each blueprint gets its own [`BlueprintRunner`], service ID, and local
/// result queue.  The shared Anvil node means cross-blueprint on-chain
/// interactions (e.g. reading each other's contract state) work naturally.
pub struct MultiHarness {
    handles: BTreeMap<String, BlueprintHandle>,
    deployment: SeededTangleTestnet,
    temp_dir: Option<TempDir>,
    runner_tasks: Vec<JoinHandle<()>>,
    saved_env_vars: Vec<(String, Option<String>)>,
}

impl MultiHarness {
    /// Create a builder.
    #[must_use]
    pub fn builder() -> MultiHarnessBuilder {
        MultiHarnessBuilder::new()
    }

    async fn spawn(builder: MultiHarnessBuilder) -> Result<Self> {
        let MultiHarnessBuilder {
            specs,
            include_anvil_logs,
            env_vars,
            state_dir_env,
        } = builder;

        anyhow::ensure!(!specs.is_empty(), "MultiHarness requires at least one blueprint");

        let deployment = start_tangle_testnet(include_anvil_logs)
            .await
            .context("failed to boot seeded Tangle EVM testnet")?;

        let temp_dir = TempDir::new().context("failed to create tempdir for multi-harness")?;

        // Apply env vars.
        let mut saved_env_vars = Vec::new();
        if let Some(ref var_name) = state_dir_env {
            let prev = std::env::var(var_name).ok();
            saved_env_vars.push((var_name.clone(), prev));
            let state_path = temp_dir.path().join("blueprint-state");
            std::fs::create_dir_all(&state_path)?;
            // SAFETY: test harness runs single-threaded during setup; env vars
            // are set before any runner tasks are spawned and restored on
            // shutdown/drop.
            unsafe { std::env::set_var(var_name, &state_path); }
        }
        for (key, value) in &env_vars {
            let prev = std::env::var(key).ok();
            saved_env_vars.push((key.clone(), prev));
            // SAFETY: same as above — single-threaded harness setup phase.
            unsafe { std::env::set_var(key, value); }
        }

        let mut handles = BTreeMap::new();
        let mut runner_tasks = Vec::new();

        for spec in specs {
            let keystore_path = temp_dir.path().join(format!("keystore-{}", spec.name));
            std::fs::create_dir_all(&keystore_path)?;
            seed_operator_key(&keystore_path)?;

            let data_dir = temp_dir.path().join(format!("data-{}", spec.name));
            std::fs::create_dir_all(&data_dir)?;

            let env = build_environment(
                &deployment,
                &keystore_path,
                &data_dir,
                spec.blueprint_id,
                spec.service_id,
            );

            let event_client = create_client(
                &deployment,
                &keystore_path,
                spec.blueprint_id,
                spec.service_id,
            )
            .await?;

            let caller_client = create_service_owner_client(
                &deployment,
                spec.blueprint_id,
                spec.service_id,
            )
            .await?;

            let runner_client = create_client(
                &deployment,
                &keystore_path,
                spec.blueprint_id,
                spec.service_id,
            )
            .await?;

            let local_results = Arc::new(Mutex::new(VecDeque::new()));
            let local_notify = Arc::new(Notify::new());

            let operator_specs = spec.operator_specs.unwrap_or_else(
                crate::blueprint::default_operator_specs,
            );

            blueprint_core::info!(
                target: "multi-harness",
                blueprint = spec.name.as_str(),
                operators = operator_specs.len(),
                "spawning operator fleet for blueprint"
            );

            // Build multi-operator consumer (handles 1..N operators with
            // individual keystores and configurable behavior).
            let (consumer, mut operator_tasks) = build_operator_runtimes(
                &operator_specs,
                &deployment,
                spec.blueprint_id,
                spec.service_id,
                Arc::clone(&local_results),
                Arc::clone(&local_notify),
            )
            .await?;

            runner_tasks.append(&mut operator_tasks);

            let start_block = runner_client
                .block_number()
                .await
                .context("failed to get block number for producer")?
                .saturating_sub(1);
            let producer = TangleProducer::from_block(
                (*runner_client).clone(),
                spec.service_id,
                start_block,
            )
            .with_poll_interval(spec.poll_interval);

            let runner_env = env.clone();
            let runner_router = spec.router.clone();
            let blueprint_name = spec.name.clone();

            let runner_task = tokio::spawn(async move {
                if let Err(err) = BlueprintRunner::builder(MultiHarnessConfig, runner_env)
                    .router(runner_router)
                    .producer(producer)
                    .consumer(consumer)
                    .run()
                    .await
                {
                    eprintln!("Blueprint runner [{blueprint_name}] exited: {err}");
                }
            });

            runner_tasks.push(runner_task);

            handles.insert(
                spec.name.clone(),
                BlueprintHandle {
                    name: spec.name,
                    caller_client,
                    event_client,
                    local_results,
                    local_notify,
                    service_id: spec.service_id,
                    blueprint_id: spec.blueprint_id,
                },
            );
        }

        Ok(Self {
            handles,
            deployment,
            temp_dir: Some(temp_dir),
            runner_tasks,
            saved_env_vars,
        })
    }

    /// Get a handle to a named blueprint.
    pub fn handle(&self, name: &str) -> Option<&BlueprintHandle> {
        self.handles.get(name)
    }

    /// Access the underlying Anvil deployment (shared by all blueprints).
    pub fn deployment(&self) -> &SeededTangleTestnet {
        &self.deployment
    }

    /// The HTTP RPC endpoint of the shared Anvil instance.
    pub fn http_endpoint(&self) -> &url::Url {
        self.deployment.http_endpoint()
    }

    /// Shut down all blueprint runners and restore env vars.
    pub async fn shutdown(mut self) {
        for task in self.runner_tasks.drain(..) {
            task.abort();
            let _ = task.await;
        }
        self.restore_env_vars();
        let _ = self.temp_dir.take();
    }

    fn restore_env_vars(&mut self) {
        for (key, original) in self.saved_env_vars.drain(..) {
            // SAFETY: called during shutdown/drop after all runner tasks have
            // been aborted; no concurrent readers of these env vars remain.
            match original {
                Some(val) => unsafe { std::env::set_var(&key, &val) },
                None => unsafe { std::env::remove_var(&key) },
            }
        }
    }
}

impl Drop for MultiHarness {
    fn drop(&mut self) {
        for task in self.runner_tasks.drain(..) {
            task.abort();
        }
        self.restore_env_vars();
        let _ = self.temp_dir.take();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internals
// ─────────────────────────────────────────────────────────────────────────────

fn build_environment(
    deployment: &SeededTangleTestnet,
    keystore_path: &Path,
    data_dir: &Path,
    blueprint_id: u64,
    service_id: u64,
) -> BlueprintEnvironment {
    let mut env = BlueprintEnvironment::default();
    env.http_rpc_endpoint = deployment.http_endpoint().clone();
    env.ws_rpc_endpoint = deployment.ws_endpoint().clone();
    env.keystore_uri = keystore_path.display().to_string();
    env.data_dir = PathBuf::from(data_dir);
    env.protocol_settings = ProtocolSettings::Tangle(TangleProtocolSettings {
        blueprint_id,
        service_id: Some(service_id),
        tangle_contract: deployment.tangle_contract,
        restaking_contract: deployment.restaking_contract,
        status_registry_contract: deployment.status_registry_contract,
    });
    env.test_mode = true;
    env
}

async fn create_client(
    deployment: &SeededTangleTestnet,
    keystore_path: &Path,
    blueprint_id: u64,
    service_id: u64,
) -> Result<Arc<TangleClient>> {
    let config = TangleClientConfig::new(
        deployment.http_endpoint().clone(),
        deployment.ws_endpoint().clone(),
        keystore_path.display().to_string(),
        TangleSettings {
            blueprint_id,
            service_id: Some(service_id),
            tangle_contract: deployment.tangle_contract,
            restaking_contract: deployment.restaking_contract,
            status_registry_contract: deployment.status_registry_contract,
        },
    )
    .test_mode(true);

    let keystore = Keystore::new(KeystoreConfig::new().fs_root(keystore_path))?;
    Ok(Arc::new(
        TangleClient::with_keystore(config, keystore).await?,
    ))
}

async fn create_service_owner_client(
    deployment: &SeededTangleTestnet,
    blueprint_id: u64,
    service_id: u64,
) -> Result<Arc<TangleClient>> {
    let config = TangleClientConfig::new(
        deployment.http_endpoint().clone(),
        deployment.ws_endpoint().clone(),
        "memory://service-owner",
        TangleSettings {
            blueprint_id,
            service_id: Some(service_id),
            tangle_contract: deployment.tangle_contract,
            restaking_contract: deployment.restaking_contract,
            status_registry_contract: deployment.status_registry_contract,
        },
    )
    .test_mode(true);

    let keystore = {
        let config = KeystoreConfig::new().in_memory(true);
        let keystore = Keystore::new(config)?;
        let secret = Vec::from_hex(SERVICE_OWNER_PRIVATE_KEY)?;
        let signing_key = K256SigningKey::from_bytes(&secret)?;
        keystore.insert::<K256Ecdsa>(&signing_key)?;
        keystore
    };

    Ok(Arc::new(
        TangleClient::with_keystore(config, keystore).await?,
    ))
}

async fn wait_for_on_chain_result(
    client: Arc<TangleClient>,
    submission: JobSubmissionResult,
    service_id: u64,
) -> Result<Vec<u8>> {
    use alloy_rpc_types::Filter;
    use blueprint_client_tangle::contracts::ITangle;
    use tokio::time::sleep;

    let tangle_address = client.tangle_address();
    let mut from_block = if let Some(block_number) = submission.tx.block_number {
        block_number
    } else {
        client.block_number().await?.saturating_sub(1)
    };
    loop {
        let current = client.block_number().await?;
        if from_block > current {
            sleep(Duration::from_millis(200)).await;
            continue;
        }
        let filter = Filter::new()
            .address(tangle_address)
            .from_block(from_block)
            .to_block(current);
        let logs = client.get_logs(&filter).await?;
        for log in logs {
            if let Ok(decoded) = log.log_decode::<ITangle::JobResultSubmitted>() {
                if decoded.inner.serviceId == service_id
                    && decoded.inner.callId == submission.call_id
                {
                    let bytes: Vec<u8> = decoded.inner.result.clone().into();
                    return Ok(bytes);
                }
            }
        }
        from_block = current;
        sleep(Duration::from_millis(200)).await;
    }
}

#[derive(Clone, Copy, Default)]
struct MultiHarnessConfig;

impl BlueprintConfig for MultiHarnessConfig {
    async fn register(&self, _: &BlueprintEnvironment) -> std::result::Result<(), RunnerError> {
        Ok(())
    }

    async fn requires_registration(
        &self,
        _: &BlueprintEnvironment,
    ) -> std::result::Result<bool, RunnerError> {
        Ok(false)
    }

    fn should_exit_after_registration(&self) -> bool {
        false
    }
}
