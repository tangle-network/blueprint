//! Blueprint runner harness helpers for Anvil-backed integration tests.
//!
//! These helpers spin up a seeded Anvil instance, seed a local keystore, and
//! prepare a [`BlueprintEnvironment`] + [`Router`] pair that mirrors the setup
//! operators use in production. Example blueprints can plug into this harness to
//! run end-to-end tests without reimplementing the boilerplate every time.

use crate::{
    LOCAL_BLUEPRINT_ID, LOCAL_SERVICE_ID, SeededTangleEvmTestnet, start_tangle_evm_testnet,
};
use alloy_primitives::{Address, Bytes};
use alloy_rpc_types::Filter;
#[cfg(feature = "aggregation")]
use anyhow::anyhow;
use anyhow::{Context, Result};
use blueprint_client_tangle_evm::{
    JobSubmissionResult, TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings,
    contracts::ITangle,
};
use blueprint_core::error::BoxError;
use blueprint_core::{JobResult, error};
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_router::Router;
use blueprint_runner::config::{BlueprintEnvironment, ProtocolSettings};
use blueprint_runner::error::RunnerError;
use blueprint_runner::tangle_evm::config::TangleEvmProtocolSettings;
use blueprint_runner::{BlueprintConfig, BlueprintRunner};
use blueprint_std::collections::VecDeque;
#[cfg(feature = "aggregation")]
use blueprint_tangle_evm_extra::{
    AggregatingConsumer, AggregationServiceConfig,
    cache::{SharedServiceConfigCache, shared_cache},
};
use blueprint_tangle_evm_extra::{TangleEvmConsumer, TangleEvmProducer};
use core::pin::Pin;
use futures_util::{Sink, SinkExt};
use hex::FromHex;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tempfile::TempDir;
use tokio::sync::Notify;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep, timeout};

const OPERATOR1_PRIVATE_KEY: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const OPERATOR2_PRIVATE_KEY: &str =
    "5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";
const SERVICE_OWNER_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

/// Builder for [`BlueprintHarness`].
pub struct BlueprintHarnessBuilder {
    router: Router,
    include_anvil_logs: bool,
    poll_interval: Duration,
    blueprint_id: u64,
    service_id: u64,
    operator_specs: Option<Vec<OperatorSpec>>,
    faulty_count: usize,
    #[cfg(feature = "aggregation")]
    aggregating_consumer: Option<AggregatingConsumerHarnessConfig>,
}

impl BlueprintHarnessBuilder {
    /// Instantiate a new builder for the provided router.
    #[must_use]
    pub fn new(router: Router) -> Self {
        Self {
            router,
            include_anvil_logs: false,
            poll_interval: Duration::from_millis(100),
            blueprint_id: LOCAL_BLUEPRINT_ID,
            service_id: LOCAL_SERVICE_ID,
            operator_specs: None,
            faulty_count: 0,
            #[cfg(feature = "aggregation")]
            aggregating_consumer: None,
        }
    }

    /// Enable or disable Anvil stdout logs.
    #[must_use]
    pub fn include_anvil_logs(mut self, include: bool) -> Self {
        self.include_anvil_logs = include;
        self
    }

    /// Override the default poll interval used by the [`TangleEvmProducer`].
    #[must_use]
    pub fn poll_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    /// Override the blueprint ID baked into the seeded contracts.
    #[must_use]
    pub fn blueprint_id(mut self, blueprint_id: u64) -> Self {
        self.blueprint_id = blueprint_id;
        self
    }

    /// Override the service ID baked into the seeded contracts.
    #[must_use]
    pub fn service_id(mut self, service_id: u64) -> Self {
        self.service_id = service_id;
        self
    }

    /// Override the operator fleet used by the harness.
    #[must_use]
    pub fn operator_fleet<const N: usize, const F: usize>(
        mut self,
        fleet: OperatorFleet<N, F>,
    ) -> Self {
        self.operator_specs = Some(fleet.into_vec());
        self.faulty_count = F;
        self
    }

    /// Configure the harness to run with an [`AggregatingConsumer`].
    #[cfg(feature = "aggregation")]
    #[must_use]
    pub fn aggregating_consumer(mut self, config: AggregatingConsumerHarnessConfig) -> Self {
        self.aggregating_consumer = Some(config);
        self
    }

    /// Spawn the harness.
    pub async fn spawn(self) -> Result<BlueprintHarness> {
        BlueprintHarness::spawn(self).await
    }
}

#[cfg(feature = "aggregation")]
#[derive(Clone)]
pub struct AggregatingConsumerHarnessConfig {
    service_config: AggregationServiceConfig,
    cache: SharedServiceConfigCache,
    auto_operator_index: bool,
}

#[cfg(feature = "aggregation")]
impl AggregatingConsumerHarnessConfig {
    /// Create a new config that auto-detects the operator index.
    #[must_use]
    pub fn new(service_config: AggregationServiceConfig) -> Self {
        Self {
            service_config,
            cache: shared_cache(),
            auto_operator_index: true,
        }
    }

    /// Provide a shared cache handle for the aggregating consumer.
    #[must_use]
    pub fn with_cache(mut self, cache: SharedServiceConfigCache) -> Self {
        self.cache = cache;
        self
    }

    /// Explicitly set the operator index to avoid auto-detection.
    #[must_use]
    pub fn with_fixed_operator_index(mut self, operator_index: u32) -> Self {
        self.service_config.operator_index = operator_index;
        self.auto_operator_index = false;
        self
    }

    pub(crate) async fn prepare_for_client(
        &mut self,
        client: &TangleEvmClient,
        service_id: u64,
    ) -> Result<()> {
        if self.auto_operator_index {
            let operators = self
                .cache
                .get_service_operators(client, service_id)
                .await
                .map_err(|e| anyhow!("failed to fetch service operators: {e}"))?;
            let operator_idx = operators.index_of(&client.account()).ok_or_else(|| {
                anyhow!(
                    "operator {:#x} not registered in service {service_id}",
                    client.account()
                )
            })?;
            self.service_config.operator_index = operator_idx as u32;
            self.auto_operator_index = false;
        }
        Ok(())
    }

    pub(crate) fn cache(&self) -> SharedServiceConfigCache {
        self.cache.clone()
    }

    pub(crate) fn service_config(&self) -> &AggregationServiceConfig {
        &self.service_config
    }
}

/// Behavior hook allowing tests to customize how an operator handles results.
pub trait OperatorBehavior: Send + Sync {
    /// Human-readable description used in logs.
    fn describe(&self) -> &'static str;
    /// Transform an emitted job result before it gets submitted.
    fn transform(&self, result: JobResult) -> OperatorOutcome;
}

/// Result of applying an [`OperatorBehavior`].
pub enum OperatorOutcome {
    /// Submit the provided result.
    Submit(JobResult),
    /// Drop the result with a debug string.
    Drop(&'static str),
}

/// Reference-counted behavior handle.
#[derive(Clone)]
pub struct OperatorBehaviorRef(Arc<dyn OperatorBehavior>);

impl OperatorBehaviorRef {
    /// Wrap a behavior implementation.
    pub fn new<B>(behavior: B) -> Self
    where
        B: OperatorBehavior + 'static,
    {
        Self(Arc::new(behavior))
    }

    fn describe(&self) -> &'static str {
        self.0.describe()
    }

    fn transform(&self, result: JobResult) -> OperatorOutcome {
        self.0.transform(result)
    }
}

/// Honest operator implementation that forwards every result unchanged.
#[derive(Clone, Copy)]
pub struct HonestOperator;

impl OperatorBehavior for HonestOperator {
    fn describe(&self) -> &'static str {
        "honest"
    }

    fn transform(&self, result: JobResult) -> OperatorOutcome {
        OperatorOutcome::Submit(result)
    }
}

/// Malicious operator that drops all results.
#[derive(Clone, Copy)]
pub struct DropAllOperator;

impl OperatorBehavior for DropAllOperator {
    fn describe(&self) -> &'static str {
        "drop-all"
    }

    fn transform(&self, _result: JobResult) -> OperatorOutcome {
        OperatorOutcome::Drop("dropping job result (faulty operator)")
    }
}

#[derive(Clone)]
enum OperatorSecret {
    Hex(String),
}

impl OperatorSecret {
    fn as_str(&self) -> &str {
        match self {
            OperatorSecret::Hex(v) => v.as_str(),
        }
    }
}

impl From<&'static str> for OperatorSecret {
    fn from(value: &'static str) -> Self {
        Self::Hex(value.to_string())
    }
}

/// Operator configuration supplied to the harness.
#[derive(Clone)]
pub struct OperatorSpec {
    label: String,
    private_key: OperatorSecret,
    behavior: OperatorBehaviorRef,
    #[cfg(feature = "aggregation")]
    aggregation: Option<AggregatingConsumerHarnessConfig>,
}

impl OperatorSpec {
    /// Honest operator using the provided hex-encoded private key.
    pub fn honest(label: impl Into<String>, private_key_hex: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            private_key: OperatorSecret::Hex(private_key_hex.into()),
            behavior: OperatorBehaviorRef::new(HonestOperator),
            #[cfg(feature = "aggregation")]
            aggregation: None,
        }
    }

    /// Override the operator behavior.
    pub fn with_behavior(mut self, behavior: OperatorBehaviorRef) -> Self {
        self.behavior = behavior;
        self
    }

    #[cfg(feature = "aggregation")]
    /// Attach an aggregation config for this operator.
    pub fn with_aggregation(mut self, config: AggregatingConsumerHarnessConfig) -> Self {
        self.aggregation = Some(config);
        self
    }
}

impl Default for OperatorSpec {
    fn default() -> Self {
        OperatorSpec::honest("operator-0", OPERATOR1_PRIVATE_KEY)
    }
}

/// Compile-time operator fleet descriptor.
pub struct OperatorFleet<const N: usize, const F: usize> {
    specs: [OperatorSpec; N],
}

impl<const N: usize, const F: usize> OperatorFleet<N, F> {
    /// Create a new fleet definition (`F` faulty operators).
    pub fn new(specs: [OperatorSpec; N]) -> Self {
        assert!(
            F <= N,
            "faulty operator count ({F}) must be <= operator count ({N})"
        );
        Self { specs }
    }

    fn into_vec(self) -> Vec<OperatorSpec> {
        self.specs.into_iter().collect()
    }
}

fn default_operator_specs() -> Vec<OperatorSpec> {
    vec![OperatorSpec::honest("operator-0", OPERATOR1_PRIVATE_KEY)]
}

type BoxedConsumer = Pin<Box<dyn Sink<JobResult, Error = BoxError> + Send>>;

struct MultiOperatorConsumer {
    senders: Vec<UnboundedSender<JobResult>>,
    local_results: Arc<Mutex<VecDeque<Vec<u8>>>>,
    local_notify: Arc<Notify>,
}

impl MultiOperatorConsumer {
    fn new(
        senders: Vec<UnboundedSender<JobResult>>,
        local_results: Arc<Mutex<VecDeque<Vec<u8>>>>,
        local_notify: Arc<Notify>,
    ) -> Self {
        Self {
            senders,
            local_results,
            local_notify,
        }
    }
}

impl Sink<JobResult> for MultiOperatorConsumer {
    type Error = BoxError;

    fn poll_ready(
        self: Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        core::task::Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: JobResult) -> Result<(), Self::Error> {
        println!("blueprint-harness: received job result");
        if let JobResult::Ok { body, .. } = &item {
            self.local_results
                .lock()
                .unwrap()
                .push_back(body.clone().to_vec());
            self.local_notify.notify_waiters();
        }
        let senders = &mut self.get_mut().senders;
        let mut remaining = Vec::with_capacity(senders.len());
        let mut any_success = false;
        for sender in senders.drain(..) {
            if sender.send(item.clone()).is_err() {
                blueprint_core::warn!(
                    target: "blueprint-harness",
                    "operator channel closed while forwarding job result"
                );
                continue;
            }
            any_success = true;
            remaining.push(sender);
        }
        *senders = remaining;
        if !any_success {
            return Err(BoxError::from("all operator channels closed".to_string()));
        }
        Ok(())
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        core::task::Poll::Ready(Ok(()))
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        self.senders.clear();
        core::task::Poll::Ready(Ok(()))
    }
}

/// End-to-end harness that wires a [`Router`] into a [`BlueprintRunner`]
/// backed by an Anvil testnet seeded with the `LocalTestnet.s.sol` contracts.
pub struct BlueprintHarness {
    client: Arc<TangleEvmClient>,
    event_client: Arc<TangleEvmClient>,
    caller_client: Arc<TangleEvmClient>,
    local_results: Arc<Mutex<VecDeque<Vec<u8>>>>,
    local_notify: Arc<Notify>,
    env: BlueprintEnvironment,
    deployment: SeededTangleEvmTestnet,
    temp_dir: Option<TempDir>,
    runner_task: Option<JoinHandle<()>>,
    operator_tasks: Vec<JoinHandle<()>>,
    service_id: u64,
    blueprint_id: u64,
}

impl BlueprintHarness {
    /// Create a builder for the provided router.
    #[must_use]
    pub fn builder(router: Router) -> BlueprintHarnessBuilder {
        BlueprintHarnessBuilder::new(router)
    }

    async fn spawn(builder: BlueprintHarnessBuilder) -> Result<Self> {
        let BlueprintHarnessBuilder {
            router,
            include_anvil_logs,
            poll_interval,
            blueprint_id,
            service_id,
            operator_specs,
            faulty_count,
            #[cfg(feature = "aggregation")]
            aggregating_consumer,
        } = builder;

        let deployment = start_tangle_evm_testnet(include_anvil_logs)
            .await
            .context("failed to boot seeded Tangle EVM testnet")?;

        let temp_dir = TempDir::new().context("failed to create tempdir for harness")?;
        let keystore_path = temp_dir.path().join("keystore");
        std::fs::create_dir_all(&keystore_path)?;
        seed_operator_key(&keystore_path)?;

        let data_dir = temp_dir.path().join("data");
        std::fs::create_dir_all(&data_dir)?;
        let env = build_environment(
            &deployment,
            &keystore_path,
            &data_dir,
            blueprint_id,
            service_id,
        );

        let client = create_client(&deployment, &keystore_path, blueprint_id, service_id).await?;
        let event_client =
            create_client(&deployment, &keystore_path, blueprint_id, service_id).await?;
        let caller_client =
            create_service_owner_client(&deployment, blueprint_id, service_id).await?;

        let mut operator_specs = operator_specs.unwrap_or_else(default_operator_specs);
        let local_results = Arc::new(Mutex::new(VecDeque::new()));
        let local_notify = Arc::new(Notify::new());
        #[cfg(feature = "aggregation")]
        if let Some(config) = aggregating_consumer {
            if operator_specs.is_empty() {
                operator_specs.push(OperatorSpec::default());
            }
            for spec in &mut operator_specs {
                spec.aggregation = Some(config.clone());
            }
        }
        if operator_specs.is_empty() {
            operator_specs.push(OperatorSpec::default());
        }
        blueprint_core::info!(
            target: "blueprint-harness",
            operators = operator_specs.len(),
            faulty = faulty_count,
            "spawning operator fleet"
        );
        let (consumer, operator_tasks) = build_operator_runtimes(
            &operator_specs,
            &deployment,
            blueprint_id,
            service_id,
            Arc::clone(&local_results),
            Arc::clone(&local_notify),
        )
        .await?;

        let runner_env = env.clone();
        let runner_router = router.clone();
        let runner_client = Arc::clone(&client);
        let runner_service_id = service_id;
        let start_block = runner_client
            .block_number()
            .await
            .unwrap_or_default()
            .saturating_sub(1);
        let producer =
            TangleEvmProducer::from_block((*runner_client).clone(), runner_service_id, start_block)
                .with_poll_interval(poll_interval);

        let runner_task = tokio::spawn(async move {
            if let Err(err) = BlueprintRunner::builder(HarnessConfig, runner_env)
                .router(runner_router)
                .producer(producer)
                .consumer(consumer)
                .run()
                .await
            {
                error!("Blueprint runner exited unexpectedly: {err}");
            }
        });

        Ok(Self {
            client,
            event_client,
            caller_client,
            local_results,
            local_notify,
            env,
            deployment,
            temp_dir: Some(temp_dir),
            runner_task: Some(runner_task),
            operator_tasks,
            service_id,
            blueprint_id,
        })
    }

    /// Access the configured blueprint environment.
    #[must_use]
    pub fn environment(&self) -> &BlueprintEnvironment {
        &self.env
    }

    /// Access the underlying Anvil deployment.
    #[must_use]
    pub fn deployment(&self) -> &SeededTangleEvmTestnet {
        &self.deployment
    }

    /// Return a clone of the underlying client.
    #[must_use]
    pub fn client(&self) -> Arc<TangleEvmClient> {
        Arc::clone(&self.client)
    }

    /// Service identifier wired into the harness.
    #[must_use]
    pub fn service_id(&self) -> u64 {
        self.service_id
    }

    /// Address used by the harness when submitting jobs.
    #[must_use]
    pub fn caller_account(&self) -> Address {
        self.caller_client.account()
    }

    /// Blueprint identifier wired into the harness.
    #[must_use]
    pub fn blueprint_id(&self) -> u64 {
        self.blueprint_id
    }

    /// Manually submit a result using another operator key.
    pub async fn submit_result_with_key(
        &self,
        operator_private_key: &str,
        call_id: u64,
        output: Bytes,
    ) -> Result<()> {
        let client = create_ephemeral_operator_client(
            &self.deployment,
            self.blueprint_id,
            self.service_id,
            operator_private_key,
        )
        .await?;

        client
            .submit_result(self.service_id, call_id, output)
            .await
            .context("failed to submit operator result")?;
        Ok(())
    }

    /// Submit a job using a custom private key.
    pub async fn submit_job_with_private_key(
        &self,
        private_key_hex: &str,
        job_index: u8,
        payload: Bytes,
    ) -> Result<JobSubmissionResult> {
        let client = create_ephemeral_operator_client(
            &self.deployment,
            self.blueprint_id,
            self.service_id,
            private_key_hex,
        )
        .await?;
        client
            .submit_job(self.service_id, job_index, payload)
            .await
            .context("failed to submit job")
    }

    /// Convenience helper for submitting results as the second seeded operator.
    pub async fn submit_second_operator_result(&self, call_id: u64, output: Bytes) -> Result<()> {
        self.submit_result_with_key(OPERATOR2_PRIVATE_KEY, call_id, output)
            .await
    }

    /// Submit pre-encoded job data to the harness service.
    ///
    /// The keystore/data directories created for the harness live until
    /// [`BlueprintHarness::shutdown`] is awaited, so call it when the test
    /// completes to clean up the temporary state.
    pub async fn submit_job(&self, job_index: u8, payload: Bytes) -> Result<JobSubmissionResult> {
        self.caller_client
            .submit_job(self.service_id, job_index, payload)
            .await
            .context("failed to submit job")
    }

    /// Wait for a [`JobResult::Ok`] emitted by the harness runner.
    pub async fn wait_for_job_result(&self, submission: JobSubmissionResult) -> Result<Vec<u8>> {
        self.wait_for_job_result_with_deadline(submission, Duration::from_secs(30))
            .await
    }

    /// Wait for a job result with a custom timeout.
    pub async fn wait_for_job_result_with_deadline(
        &self,
        submission: JobSubmissionResult,
        timeout_duration: Duration,
    ) -> Result<Vec<u8>> {
        let local_wait = self.wait_for_local_result_unbounded();
        let on_chain_wait = Self::wait_for_job_result_on_chain_internal(
            Arc::clone(&self.event_client),
            submission,
            self.service_id,
        );
        let fut = async {
            tokio::select! {
                output = local_wait => Ok(output),
                output = on_chain_wait => output,
            }
        };

        timeout(timeout_duration, fut)
            .await
            .context("timed out waiting for JobResultSubmitted")?
    }

    /// Wait for a job result emitted on-chain, bypassing local result queue.
    pub async fn wait_for_job_result_on_chain_with_deadline(
        &self,
        submission: JobSubmissionResult,
        timeout_duration: Duration,
    ) -> Result<Vec<u8>> {
        timeout(
            timeout_duration,
            Self::wait_for_job_result_on_chain_internal(
                Arc::clone(&self.event_client),
                submission,
                self.service_id,
            ),
        )
        .await
        .context("timed out waiting for JobResultSubmitted")?
    }

    /// Wait for an on-chain job result using the default timeout.
    pub async fn wait_for_job_result_on_chain(
        &self,
        submission: JobSubmissionResult,
    ) -> Result<Vec<u8>> {
        self.wait_for_job_result_on_chain_with_deadline(submission, Duration::from_secs(30))
            .await
    }

    async fn wait_for_local_result_unbounded(&self) -> Vec<u8> {
        loop {
            if let Some(output) = self.take_local_result() {
                println!("blueprint-harness: drained local result from queue");
                return output;
            }
            self.local_notify.notified().await;
            if let Some(output) = self.take_local_result() {
                println!("blueprint-harness: received local result via notify");
                return output;
            }
        }
    }

    fn take_local_result(&self) -> Option<Vec<u8>> {
        self.local_results.lock().unwrap().pop_front()
    }

    /// Abort the runner task and tear down the harness.
    pub async fn shutdown(mut self) {
        if let Some(handle) = self.abort_runner() {
            let _ = handle.await;
        }
        for task in self.operator_tasks.drain(..) {
            task.abort();
            let _ = task.await;
        }
        let _ = self.temp_dir.take();
    }

    fn abort_runner(&mut self) -> Option<JoinHandle<()>> {
        self.runner_task.take().map(|handle| {
            handle.abort();
            handle
        })
    }

    async fn wait_for_job_result_on_chain_internal(
        client: Arc<TangleEvmClient>,
        submission: JobSubmissionResult,
        service_id: u64,
    ) -> Result<Vec<u8>> {
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
}

impl Drop for BlueprintHarness {
    fn drop(&mut self) {
        let _ = self.abort_runner();
        for task in self.operator_tasks.drain(..) {
            task.abort();
        }
        let _ = self.temp_dir.take();
    }
}

fn build_environment(
    deployment: &SeededTangleEvmTestnet,
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
    env.protocol_settings = ProtocolSettings::TangleEvm(TangleEvmProtocolSettings {
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
    deployment: &SeededTangleEvmTestnet,
    keystore_path: &Path,
    blueprint_id: u64,
    service_id: u64,
) -> Result<Arc<TangleEvmClient>> {
    let config = TangleEvmClientConfig::new(
        deployment.http_endpoint().clone(),
        deployment.ws_endpoint().clone(),
        keystore_path.display().to_string(),
        TangleEvmSettings {
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
        TangleEvmClient::with_keystore(config, keystore).await?,
    ))
}

async fn create_service_owner_client(
    deployment: &SeededTangleEvmTestnet,
    blueprint_id: u64,
    service_id: u64,
) -> Result<Arc<TangleEvmClient>> {
    create_ephemeral_operator_client(
        deployment,
        blueprint_id,
        service_id,
        SERVICE_OWNER_PRIVATE_KEY,
    )
    .await
}

async fn create_ephemeral_operator_client(
    deployment: &SeededTangleEvmTestnet,
    blueprint_id: u64,
    service_id: u64,
    private_key_hex: &str,
) -> Result<Arc<TangleEvmClient>> {
    let config = TangleEvmClientConfig::new(
        deployment.http_endpoint().clone(),
        deployment.ws_endpoint().clone(),
        "memory://service-owner",
        TangleEvmSettings {
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
        let secret = Vec::from_hex(private_key_hex)?;
        let signing_key = K256SigningKey::from_bytes(&secret)?;
        keystore.insert::<K256Ecdsa>(&signing_key)?;
        keystore
    };

    Ok(Arc::new(
        TangleEvmClient::with_keystore(config, keystore).await?,
    ))
}

pub fn seed_operator_key(path: &Path) -> Result<()> {
    let config = KeystoreConfig::new().fs_root(path);
    let keystore = Keystore::new(config)?;
    let secret = Vec::from_hex(OPERATOR1_PRIVATE_KEY)?;
    let signing_key = K256SigningKey::from_bytes(&secret)?;
    keystore.insert::<K256Ecdsa>(&signing_key)?;
    Ok(())
}

async fn build_operator_runtimes(
    specs: &[OperatorSpec],
    deployment: &SeededTangleEvmTestnet,
    blueprint_id: u64,
    service_id: u64,
    local_results: Arc<Mutex<VecDeque<Vec<u8>>>>,
    local_notify: Arc<Notify>,
) -> Result<(MultiOperatorConsumer, Vec<JoinHandle<()>>)> {
    let mut senders = Vec::new();
    let mut tasks = Vec::new();
    for spec in specs {
        let client = create_ephemeral_operator_client(
            deployment,
            blueprint_id,
            service_id,
            spec.private_key.as_str(),
        )
        .await?;
        let sink = build_operator_sink(Arc::clone(&client), service_id, spec).await?;
        let (tx, rx) = unbounded_channel();
        let behavior = spec.behavior.clone();
        let label = spec.label.clone();
        blueprint_core::debug!(
            target: "blueprint-harness",
            operator = label.as_str(),
            behavior = behavior.describe(),
            "wiring operator sink"
        );
        let handle = tokio::spawn(async move {
            operator_sink_task(label, behavior, rx, sink).await;
        });
        senders.push(tx);
        tasks.push(handle);
    }
    Ok((
        MultiOperatorConsumer::new(senders, local_results, local_notify),
        tasks,
    ))
}

async fn operator_sink_task(
    label: String,
    behavior: OperatorBehaviorRef,
    mut rx: UnboundedReceiver<JobResult>,
    mut sink: BoxedConsumer,
) {
    while let Some(job) = rx.recv().await {
        match behavior.transform(job) {
            OperatorOutcome::Submit(result) => {
                if let Err(err) = sink.send(result).await {
                    error!("operator {label} failed to submit result: {err}");
                    eprintln!("operator {label} failed to submit result: {err}");
                    continue;
                }
            }
            OperatorOutcome::Drop(reason) => {
                blueprint_core::trace!(
                    target: "blueprint-harness",
                    operator = label.as_str(),
                    reason,
                    "operator dropped job result"
                );
            }
        }
    }
}

async fn build_operator_sink(
    client: Arc<TangleEvmClient>,
    service_id: u64,
    spec: &OperatorSpec,
) -> Result<BoxedConsumer> {
    #[cfg(feature = "aggregation")]
    if let Some(mut cfg) = spec.aggregation.clone() {
        cfg.prepare_for_client(client.as_ref(), service_id).await?;
        let consumer = AggregatingConsumer::with_cache((*client).clone(), cfg.cache())
            .with_aggregation_config(cfg.service_config().clone());
        return Ok(Box::pin(consumer));
    }

    #[cfg(not(feature = "aggregation"))]
    let _ = spec;
    #[cfg(not(feature = "aggregation"))]
    let _ = service_id;

    Ok(Box::pin(TangleEvmConsumer::new((*client).clone())))
}

#[derive(Clone, Copy, Default)]
struct HarnessConfig;

impl BlueprintConfig for HarnessConfig {
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
