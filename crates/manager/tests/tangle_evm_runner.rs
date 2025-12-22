use core::convert::TryFrom;
use std::path::Path;
use std::str::FromStr;
use std::string::ToString;
use std::sync::Arc;
use std::time::Duration;

use alloy_network::EthereumWallet;
use alloy_primitives::{Address, Bytes};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::Filter;
use alloy_rpc_types::transaction::TransactionRequest;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::SolCall;
use alloy_sol_types::SolValue;
use anyhow::{Context, Result, anyhow};
use blueprint_anvil_testing_utils::{
    SeededTangleEvmTestnet, harness_builder_from_env, missing_tnt_core_artifacts,
};
use blueprint_auth::db::{RocksDb, RocksDbConfig};
use blueprint_client_tangle_evm::contracts::ITangle::addPermittedCallerCall;
use blueprint_client_tangle_evm::{
    JobSubmissionResult, TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings,
};
use blueprint_core::{Job, JobResult};
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_manager_bridge::server::{Bridge, BridgeHandle};
use blueprint_router::Router;
use blueprint_runner::config::{BlueprintEnvironment, ProtocolSettings};
use blueprint_runner::tangle_evm::config::TangleEvmProtocolSettings;
use blueprint_runner::{BlueprintConfig, BlueprintRunner};
use blueprint_tangle_evm_extra::extract::{TangleEvmArg, TangleEvmResult};
use blueprint_tangle_evm_extra::{TangleEvmConsumer, TangleEvmLayer, TangleEvmProducer};
use futures_util::future::poll_fn;
use futures_util::pin_mut;
use futures_util::{SinkExt, StreamExt, stream};
use hex::FromHex;
use tempfile::TempDir;
use tokio::sync::oneshot;
use tokio::time::timeout;
use tower::Service;

const OPERATOR1_PRIVATE_KEY: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const SERVICE_OWNER_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const BLUEPRINT_ID: u64 = 0;
const SERVICE_ID: u64 = 0;
const JOB_INDEX: u8 = 0;

const ANVIL_TEST_TIMEOUT: Duration = Duration::from_secs(1_800);
const JOB_RESULT_TIMEOUT: Duration = Duration::from_secs(300);

struct RunnerTestHarness {
    deployment: SeededTangleEvmTestnet,
    env: BlueprintEnvironment,
    runner_client: Arc<TangleEvmClient>,
    control_client: Arc<TangleEvmClient>,
    start_block: u64,
    _temp_dir: TempDir,
    bridge_handle: BridgeHandle,
}

impl RunnerTestHarness {
    fn runner_env(&self) -> BlueprintEnvironment {
        self.env.clone()
    }

    fn router(&self) -> Router {
        Router::new().route(JOB_INDEX, echo_job.layer(TangleEvmLayer))
    }

    fn producer(&self) -> TangleEvmProducer {
        TangleEvmProducer::from_block((*self.runner_client).clone(), SERVICE_ID, self.start_block)
            .with_poll_interval(Duration::from_millis(100))
    }

    fn consumer(&self) -> TangleEvmConsumer {
        TangleEvmConsumer::new((*self.runner_client).clone())
    }

    fn control_client(&self) -> Arc<TangleEvmClient> {
        Arc::clone(&self.control_client)
    }

    async fn grant_runner_permissions(&self) -> Result<()> {
        grant_permitted_caller(
            self.deployment.http_endpoint().as_str(),
            self.deployment.tangle_contract,
            self.runner_client.account(),
        )
        .await
        .context("failed to permit runner caller")
    }
}

impl Drop for RunnerTestHarness {
    fn drop(&mut self) {
        // Explicitly reference the bridge handle so the compiler knows it is intentional.
        let _ = &self.bridge_handle;
    }
}

async fn setup_runner_test(test_name: &str) -> Result<Option<RunnerTestHarness>> {
    let Some(deployment) = boot_testnet(test_name).await? else {
        return Ok(None);
    };

    let temp = TempDir::new().context("failed to create tempdir")?;
    let keystore_path = temp.path().join("keystore");
    std::fs::create_dir_all(&keystore_path)?;
    seed_operator_key(&keystore_path)?;

    let data_dir = temp.path().join("data");
    std::fs::create_dir_all(&data_dir)?;

    let bridge_runtime = temp.path().join("bridge-runtime");
    std::fs::create_dir_all(&bridge_runtime)?;
    let bridge_db_path = bridge_runtime.join("auth-db");
    let auth_db = RocksDb::open(&bridge_db_path, &RocksDbConfig::default())
        .context("failed to open bridge db")?;
    let bridge = Bridge::new(bridge_runtime, String::from("runner"), auth_db, true);
    let bridge_socket_path = bridge.base_socket_path();
    let (bridge_handle, _bridge_alive) = bridge.spawn()?;

    let mut env = BlueprintEnvironment::default();
    env.http_rpc_endpoint = deployment.http_endpoint().clone();
    env.ws_rpc_endpoint = deployment.ws_endpoint().clone();
    env.keystore_uri = keystore_path.display().to_string();
    env.data_dir = data_dir.clone();
    env.bridge_socket_path = Some(bridge_socket_path);
    env.protocol_settings = ProtocolSettings::TangleEvm(TangleEvmProtocolSettings {
        blueprint_id: BLUEPRINT_ID,
        service_id: Some(SERVICE_ID),
        tangle_contract: deployment.tangle_contract,
        restaking_contract: deployment.restaking_contract,
        status_registry_contract: deployment.status_registry_contract,
    });
    env.test_mode = true;

    let runner_client = create_fs_client(&deployment, &keystore_path).await?;
    let control_client = create_fs_client(&deployment, &keystore_path).await?;
    let start_block = runner_client
        .block_number()
        .await
        .context("failed to read runner start block")?
        .saturating_sub(1);

    Ok(Some(RunnerTestHarness {
        deployment,
        env,
        runner_client,
        control_client,
        start_block,
        _temp_dir: temp,
        bridge_handle,
    }))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn blueprint_runner_processes_jobs_on_tangle_evm() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("blueprint_runner_processes_jobs_on_tangle_evm", async {
        let Some(harness) =
            setup_runner_test("blueprint_runner_processes_jobs_on_tangle_evm").await?
        else {
            return Ok(());
        };

        harness.grant_runner_permissions().await?;

        let producer = harness.producer();
        let consumer = harness.consumer();
        let router = harness.router();
        let runner_env = harness.runner_env();
        let control_client = harness.control_client();
        let runner_task = tokio::spawn(async move {
            BlueprintRunner::builder(TestRunnerConfig, runner_env)
                .router(router)
                .producer(producer)
                .consumer(consumer)
                .run()
                .await
                .expect("blueprint runner should stay alive");
        });

        let raw_payload = b"tangle-runner".to_vec();
        let encoded_payload = Bytes::from(raw_payload.clone().abi_encode());
        let submission = control_client
            .submit_job(SERVICE_ID, JOB_INDEX, encoded_payload.clone())
            .await
            .context("failed to submit job")?;

        let output = wait_for_job_result((*control_client).clone(), submission)
            .await
            .context("job result not observed")?;
        let decoded = Vec::<u8>::abi_decode(&output).context("failed to decode job result")?;
        assert_eq!(decoded, raw_payload);

        runner_task.abort();
        let _ = runner_task.await;
        drop(harness);

        Ok(())
    })
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn minimal_runner_processes_jobs_on_tangle_evm() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("minimal_runner_processes_jobs_on_tangle_evm", async {
        let Some(harness) =
            setup_runner_test("minimal_runner_processes_jobs_on_tangle_evm").await?
        else {
            return Ok(());
        };

        harness.grant_runner_permissions().await?;

        let producer = harness.producer();
        let consumer = harness.consumer();
        let router = harness.router();
        let control_client = harness.control_client();

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (result_tx, result_rx) = oneshot::channel();
        let minimal_task = tokio::spawn(async move {
            run_minimal_runner_loop(producer, router, consumer, shutdown_rx, Some(result_tx)).await
        });

        let raw_payload = b"tangle-runner".to_vec();
        let encoded_payload = Bytes::from(raw_payload.clone().abi_encode());
        let submission = control_client
            .submit_job(SERVICE_ID, JOB_INDEX, encoded_payload.clone())
            .await
            .context("failed to submit job")?;

        let output = timeout(JOB_RESULT_TIMEOUT, result_rx)
            .await
            .context("timed out waiting for local job result")?
            .context("local job result channel closed")?;
        let decoded = Vec::<u8>::abi_decode(&output).context("failed to decode job result")?;
        assert_eq!(decoded, raw_payload);
        wait_for_job_completion((*control_client).clone(), submission.call_id)
            .await
            .context("job completion not observed on-chain")?;

        let _ = shutdown_tx.send(());
        minimal_task
            .await
            .context("minimal runner task panicked")?
            .context("minimal runner exited with error")?;

        drop(harness);

        Ok(())
    })
    .await
}

async fn run_minimal_runner_loop(
    producer: TangleEvmProducer,
    mut router: Router,
    mut consumer: TangleEvmConsumer,
    mut shutdown_rx: oneshot::Receiver<()>,
    mut result_tx: Option<oneshot::Sender<Vec<u8>>>,
) -> Result<()> {
    let mut router = router.as_service();
    poll_fn(|ctx| router.poll_ready(ctx)).await.unwrap_or(());
    pin_mut!(producer);

    loop {
        tokio::select! {
            _ = &mut shutdown_rx => break,
            maybe_job = producer.next() => {
                let Some(job) = maybe_job else {
                    continue;
                };

                let job_call = match job {
                    Ok(job_call) => job_call,
                    Err(err) => {
                        blueprint_core::error!(
                            target: "tangle-evm-minimal-runner",
                            error = ?err,
                            "Producer error in minimal loop"
                        );
                        continue;
                    }
                };

                let block_number = job_call
                    .metadata()
                    .get("tangle_evm.block_number")
                    .and_then(|value| u64::try_from(value).ok());
                let service_id = job_call
                    .metadata()
                    .get("tangle_evm.service_id")
                    .and_then(|value| u64::try_from(value).ok());
                blueprint_core::info!(
                    target: "tangle-evm-minimal-runner",
                    job_id = ?job_call.job_id(),
                    block_number = ?block_number,
                    service_id = ?service_id,
                    "Processing job call in minimal loop"
                );

                match router.call(job_call).await {
                    Ok(Some(results)) => {
                        let result_len = results.len();
                        if let Some(first) = results.get(0) {
                            match first {
                                JobResult::Ok { head, .. } => {
                                    blueprint_core::info!(
                                        target: "tangle-evm-minimal-runner",
                                        metadata = ?head.metadata,
                                        "First job result metadata"
                                    );
                                }
                                JobResult::Err(error) => {
                                    blueprint_core::error!(
                                        target: "tangle-evm-minimal-runner",
                                        ?error,
                                        "Job result returned error"
                                    );
                                }
                            }
                        }
                        if let Some(tx) = result_tx.take() {
                            if let Some(JobResult::Ok { body, .. }) = results.get(0) {
                                let _ = tx.send(body.to_vec());
                            }
                        }
                        let mut result_stream = stream::iter(results.into_iter().map(Ok));
                        blueprint_core::info!(
                            target: "tangle-evm-minimal-runner",
                            result_count = result_len,
                            "Router produced results"
                        );
                        consumer
                            .send_all(&mut result_stream)
                            .await
                            .map_err(|err| anyhow!("failed to forward router output: {err}"))?;
                        consumer
                            .flush()
                            .await
                            .map_err(|err| anyhow!("failed to flush consumer: {err}"))?;
                    }
                    Ok(None) => {}
                    Err(err) => {
                        blueprint_core::error!(
                            target: "tangle-evm-minimal-runner",
                            error = ?err,
                            "Router failed to process job"
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

async fn wait_for_job_result(
    client: TangleEvmClient,
    submission: JobSubmissionResult,
) -> Result<Vec<u8>> {
    use blueprint_client_tangle_evm::contracts::ITangle;
    use tokio::time::sleep;

    let tangle_address = client.tangle_address();
    let mut from_block = if let Some(block_number) = submission.tx.block_number {
        block_number
    } else {
        client.block_number().await?.saturating_sub(1)
    };

    let fut = async {
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
                    if decoded.inner.serviceId == SERVICE_ID
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
    };

    timeout(JOB_RESULT_TIMEOUT, fut)
        .await
        .context("timed out waiting for JobResultSubmitted")?
}

async fn wait_for_job_completion(
    client: TangleEvmClient,
    call_id: u64,
) -> Result<()> {
    use tokio::time::sleep;

    timeout(JOB_RESULT_TIMEOUT, async {
        loop {
            let call = client.get_job_call(SERVICE_ID, call_id).await?;
            if call.completed {
                return Ok(());
            }
            sleep(Duration::from_millis(200)).await;
        }
    })
    .await
    .context("timed out waiting for job completion")?
}

async fn create_fs_client(
    deployment: &SeededTangleEvmTestnet,
    keystore_path: &Path,
) -> Result<Arc<TangleEvmClient>> {
    let config = TangleEvmClientConfig::new(
        deployment.http_endpoint().clone(),
        deployment.ws_endpoint().clone(),
        keystore_path.display().to_string(),
        TangleEvmSettings {
            blueprint_id: BLUEPRINT_ID,
            service_id: Some(SERVICE_ID),
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

fn seed_operator_key(path: &Path) -> Result<()> {
    let config = KeystoreConfig::new().fs_root(path);
    let keystore = Keystore::new(config)?;
    let secret = Vec::from_hex(OPERATOR1_PRIVATE_KEY)?;
    let signing_key = K256SigningKey::from_bytes(&secret)?;
    keystore.insert::<K256Ecdsa>(&signing_key)?;
    Ok(())
}

async fn grant_permitted_caller(
    rpc_endpoint: &str,
    tangle_address: Address,
    caller: Address,
) -> Result<()> {
    let signer = PrivateKeySigner::from_str(SERVICE_OWNER_PRIVATE_KEY)
        .context("invalid service owner private key")?;
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(rpc_endpoint)
        .await
        .context("failed to build service owner provider")?;

    let permit = addPermittedCallerCall {
        serviceId: SERVICE_ID,
        caller,
    };
    let tx = TransactionRequest::default()
        .to(tangle_address)
        .input(permit.abi_encode().into());
    provider
        .send_transaction(tx)
        .await
        .context("failed to send permit transaction")?
        .get_receipt()
        .await
        .context("permit transaction reverted")?;
    Ok(())
}

async fn echo_job(TangleEvmArg(input): TangleEvmArg<Vec<u8>>) -> TangleEvmResult<Vec<u8>> {
    println!("echo_job invoked with {} bytes", input.len());
    TangleEvmResult(input)
}

#[derive(Clone)]
struct TestRunnerConfig;

impl BlueprintConfig for TestRunnerConfig {
    fn register(
        &self,
        _env: &BlueprintEnvironment,
    ) -> impl std::future::Future<Output = Result<(), blueprint_runner::error::RunnerError>> + Send
    {
        async { Ok(()) }
    }

    fn requires_registration(
        &self,
        _env: &BlueprintEnvironment,
    ) -> impl std::future::Future<Output = Result<bool, blueprint_runner::error::RunnerError>> + Send
    {
        async { Ok(false) }
    }

    fn should_exit_after_registration(&self) -> bool {
        false
    }
}

async fn run_anvil_test<F>(name: &str, fut: F) -> Result<()>
where
    F: std::future::Future<Output = Result<()>>,
{
    timeout(ANVIL_TEST_TIMEOUT, fut)
        .await
        .with_context(|| format!("{name} timed out after {:?}", ANVIL_TEST_TIMEOUT))?
}

async fn boot_testnet(test_name: &str) -> Result<Option<SeededTangleEvmTestnet>> {
    match harness_builder_from_env().spawn().await {
        Ok(deployment) => Ok(Some(deployment)),
        Err(err) => {
            if missing_tnt_core_artifacts(&err) {
                eprintln!("Skipping {test_name}: {err}");
                Ok(None)
            } else {
                Err(err)
            }
        }
    }
}
