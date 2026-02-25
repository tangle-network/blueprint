//! Service Lifecycle End-to-End Tests
//!
//! This module tests the complete service lifecycle on Tangle EVM:
//! 1. Operator registration
//! 2. Service request
//! 3. Service approval/activation
//! 4. Job submission and result processing
//! 5. Service termination
//!
//! These tests require `RUN_TNT_E2E=1` and the bundled LocalTestnet broadcast/snapshot.

use std::path::Path;
use std::str::FromStr;
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
use anyhow::{Context, Result, anyhow, ensure};
use blueprint_anvil_testing_utils::{
    SeededTangleTestnet, harness_builder_from_env, missing_tnt_core_artifacts,
};
use blueprint_client_tangle::contracts::ITangle;
use blueprint_client_tangle::contracts::ITangle::addPermittedCallerCall;
use blueprint_client_tangle::{ServiceStatus, TangleClient, TangleClientConfig, TangleSettings};
use blueprint_core::Job;
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_router::Router;
use blueprint_tangle_extra::extract::{TangleArg, TangleResult};
use blueprint_tangle_extra::{TangleConsumer, TangleLayer, TangleProducer};
use futures_util::future::poll_fn;
use futures_util::pin_mut;
use futures_util::{SinkExt, StreamExt, stream};
use hex::FromHex;
use tempfile::TempDir;
use tokio::sync::oneshot;
use tokio::time::timeout;
use tower::Service;

// Well-known Anvil test accounts
const OPERATOR1_PRIVATE_KEY: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const SERVICE_OWNER_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

// Test constants
const BLUEPRINT_ID: u64 = 0;
const SERVICE_ID: u64 = 0;
const JOB_INDEX: u8 = 0;
const ANVIL_TEST_TIMEOUT: Duration = Duration::from_secs(300);

/// Complete service lifecycle test harness
struct LifecycleTestHarness {
    deployment: SeededTangleTestnet,
    operator_client: Arc<TangleClient>,
    owner_client: Arc<TangleClient>,
    _temp_dir: TempDir,
}

impl LifecycleTestHarness {
    async fn new(deployment: SeededTangleTestnet) -> Result<Self> {
        let temp = TempDir::new().context("failed to create tempdir")?;
        let keystore_path = temp.path().join("keystore");
        std::fs::create_dir_all(&keystore_path)?;
        seed_operator_key(&keystore_path, OPERATOR1_PRIVATE_KEY)?;

        let operator_client =
            create_client(&deployment, &keystore_path, BLUEPRINT_ID, Some(SERVICE_ID)).await?;

        // Create owner client with separate keystore
        let owner_keystore_path = temp.path().join("owner_keystore");
        std::fs::create_dir_all(&owner_keystore_path)?;
        seed_operator_key(&owner_keystore_path, SERVICE_OWNER_PRIVATE_KEY)?;
        let owner_client = create_client(
            &deployment,
            &owner_keystore_path,
            BLUEPRINT_ID,
            Some(SERVICE_ID),
        )
        .await?;

        Ok(Self {
            deployment,
            operator_client,
            owner_client,
            _temp_dir: temp,
        })
    }

    fn operator_account(&self) -> Address {
        self.operator_client.account()
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_full_service_lifecycle() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("test_full_service_lifecycle", async {
        let Some(deployment) = boot_testnet("test_full_service_lifecycle").await? else {
            return Ok(());
        };

        let harness = LifecycleTestHarness::new(deployment).await?;

        // Step 1: Verify operator is registered (seeded state)
        let is_registered = harness
            .operator_client
            .is_operator_registered(BLUEPRINT_ID, harness.operator_account())
            .await
            .context("failed to check operator registration")?;
        ensure!(
            is_registered,
            "operator should be registered in seeded state"
        );
        println!(
            "✓ Operator {} is registered for blueprint {}",
            harness.operator_account(),
            BLUEPRINT_ID
        );

        // Step 2: Verify service is active (seeded state)
        let service = harness
            .operator_client
            .get_service(SERVICE_ID)
            .await
            .context("failed to get service")?;
        ensure!(
            service.status == ServiceStatus::Active as u8,
            "service should be active in seeded state"
        );
        println!("✓ Service {} is active", SERVICE_ID);

        // Step 3: Check service operators
        let operators = harness
            .operator_client
            .get_service_operators(SERVICE_ID)
            .await
            .context("failed to get service operators")?;
        println!("✓ Service has {} operator(s)", operators.len());
        ensure!(
            !operators.is_empty(),
            "service should have at least one operator"
        );

        // Step 4: Verify operator is in service
        let is_service_operator = harness
            .operator_client
            .is_service_operator(SERVICE_ID, harness.operator_account())
            .await
            .context("failed to check service operator")?;
        ensure!(is_service_operator, "operator should be in service");
        println!("✓ Operator is part of service {}", SERVICE_ID);

        // Step 5: Grant caller permissions for job submission
        grant_permitted_caller(
            harness.deployment.http_endpoint().as_str(),
            harness.deployment.tangle_contract,
            harness.operator_account(),
        )
        .await
        .context("failed to grant caller permissions")?;
        println!("✓ Granted caller permissions to operator");

        // Step 6: Set up runner to process the job
        let temp = TempDir::new()?;
        let keystore_path = temp.path().join("runner_keystore");
        std::fs::create_dir_all(&keystore_path)?;
        seed_operator_key(&keystore_path, OPERATOR1_PRIVATE_KEY)?;

        let runner_client = create_client(
            &harness.deployment,
            &keystore_path,
            BLUEPRINT_ID,
            Some(SERVICE_ID),
        )
        .await?;

        let start_block = runner_client
            .block_number()
            .await
            .unwrap_or_default()
            .saturating_sub(1);
        let producer =
            TangleProducer::from_block((*runner_client).clone(), SERVICE_ID, start_block)
                .with_poll_interval(Duration::from_millis(100));
        let consumer = TangleConsumer::new((*runner_client).clone());
        let router = Router::new().route(JOB_INDEX, square_job.layer(TangleLayer));

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (result_tx, result_rx) = oneshot::channel();
        let runner_task = tokio::spawn(async move {
            run_minimal_runner_loop(producer, router, consumer, shutdown_rx, Some(result_tx)).await
        });

        // Step 7: Submit a job
        let input_value: u64 = 42;
        let encoded_input = Bytes::from(input_value.abi_encode());
        let submission = harness
            .owner_client
            .submit_job(SERVICE_ID, JOB_INDEX, encoded_input)
            .await
            .context("failed to submit job")?;
        let call_id = submission.call_id;
        println!("✓ Submitted job with call_id {}", call_id);

        // Step 8: Wait for job result
        let on_chain_result = wait_for_job_result_on_chain(
            (*harness.operator_client).clone(),
            call_id,
            submission.tx.block_number,
        );
        let output = timeout(Duration::from_secs(120), async {
            tokio::select! {
                output = result_rx => output.context("local job result channel closed"),
                output = on_chain_result => output,
            }
        })
        .await
        .context("timed out waiting for job result")??;
        let result: u64 = u64::abi_decode(&output).context("failed to decode job result")?;
        ensure!(
            result == input_value * input_value,
            "expected {} but got {}",
            input_value * input_value,
            result
        );
        println!(
            "✓ Received correct job result: {} = {}²",
            result, input_value
        );

        // Step 9: Verify job result on-chain
        wait_for_job_completion((*harness.operator_client).clone(), call_id)
            .await
            .context("job should be marked as completed")?;
        println!("✓ Job {} verified as completed on-chain", call_id);

        // Cleanup
        let _ = shutdown_tx.send(());
        let _ = runner_task.await;

        println!("\n🎉 Full service lifecycle test passed!");
        Ok(())
    })
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_operator_status_queries() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("test_operator_status_queries", async {
        let Some(deployment) = boot_testnet("test_operator_status_queries").await? else {
            return Ok(());
        };

        let harness = LifecycleTestHarness::new(deployment).await?;

        // Test operator active status
        let is_active = harness
            .operator_client
            .is_operator_active(harness.operator_account())
            .await
            .context("failed to check operator active status")?;
        println!("Operator active status: {}", is_active);

        // Test operator stake
        let stake = harness
            .operator_client
            .get_operator_stake(harness.operator_account())
            .await
            .context("failed to get operator stake")?;
        println!("Operator stake: {} wei", stake);

        // Test minimum operator stake
        let min_stake = harness
            .operator_client
            .min_operator_stake()
            .await
            .context("failed to get minimum stake")?;
        println!("Minimum operator stake: {} wei", min_stake);

        // Test blueprint count
        let blueprint_count = harness
            .operator_client
            .blueprint_count()
            .await
            .context("failed to get blueprint count")?;
        println!("Total blueprints: {}", blueprint_count);
        ensure!(blueprint_count > 0, "should have at least one blueprint");

        // Test service count
        let service_count = harness
            .operator_client
            .service_count()
            .await
            .context("failed to get service count")?;
        println!("Total services: {}", service_count);
        ensure!(service_count > 0, "should have at least one service");

        println!("\n✓ Operator status queries passed!");
        Ok(())
    })
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_service_operator_weights() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("test_service_operator_weights", async {
        let Some(deployment) = boot_testnet("test_service_operator_weights").await? else {
            return Ok(());
        };

        let harness = LifecycleTestHarness::new(deployment).await?;

        // Get service operator weights
        let weights = harness
            .operator_client
            .get_service_operator_weights(SERVICE_ID)
            .await
            .context("failed to get operator weights")?;

        println!("Service {} operator weights:", SERVICE_ID);
        for (operator, weight) in &weights {
            println!("  {} => {}", operator, weight);
        }

        // Get total exposure
        let total_exposure = harness
            .operator_client
            .get_service_total_exposure(SERVICE_ID)
            .await
            .context("failed to get total exposure")?;
        println!("Total service exposure: {} wei", total_exposure);

        println!("\n✓ Service operator weights test passed!");
        Ok(())
    })
    .await
}

// Helper functions

async fn square_job(TangleArg((x,)): TangleArg<(u64,)>) -> TangleResult<u64> {
    TangleResult(x * x)
}

async fn run_minimal_runner_loop(
    producer: TangleProducer,
    mut router: Router,
    mut consumer: TangleConsumer,
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
                let Some(job) = maybe_job else { continue };
                let job_call = match job {
                    Ok(jc) => jc,
                    Err(e) => {
                        eprintln!("Producer error: {:?}", e);
                        continue;
                    }
                };

                match router.call(job_call).await {
                    Ok(Some(results)) => {
                        if let Some(tx) = result_tx.take() {
                            if let Some(blueprint_core::JobResult::Ok { body, .. }) = results.get(0) {
                                let _ = tx.send(body.to_vec());
                            }
                        }
                        let mut result_stream = stream::iter(results.into_iter().map(Ok));
                        consumer.send_all(&mut result_stream).await
                            .map_err(|e| anyhow!("consumer send failed: {e}"))?;
                        consumer.flush().await
                            .map_err(|e| anyhow!("consumer flush failed: {e}"))?;
                    }
                    Ok(None) => {}
                    Err(e) => eprintln!("Router error: {:?}", e),
                }
            }
        }
    }

    Ok(())
}

async fn wait_for_job_completion(client: TangleClient, call_id: u64) -> Result<()> {
    use tokio::time::sleep;

    timeout(Duration::from_secs(120), async {
        loop {
            let call = client
                .get_job_call(SERVICE_ID, call_id)
                .await
                .context("failed to get job call")?;
            if call.completed {
                return Ok(());
            }
            sleep(Duration::from_millis(200)).await;
        }
    })
    .await
    .context("timed out waiting for job completion")?
}

async fn wait_for_job_result_on_chain(
    client: TangleClient,
    call_id: u64,
    start_block: Option<u64>,
) -> Result<Vec<u8>> {
    let tangle_address = client.tangle_address();
    let mut from_block = if let Some(block) = start_block {
        block
    } else {
        client.block_number().await?.saturating_sub(1)
    };

    loop {
        let current = client.block_number().await?;
        if from_block > current {
            tokio::time::sleep(Duration::from_millis(200)).await;
            continue;
        }

        let filter = Filter::new()
            .address(tangle_address)
            .from_block(from_block)
            .to_block(current);
        let logs = client.get_logs(&filter).await?;
        for log in logs {
            if let Ok(decoded) = log.log_decode::<ITangle::JobResultSubmitted>() {
                if decoded.inner.serviceId == SERVICE_ID && decoded.inner.callId == call_id {
                    let bytes: Vec<u8> = decoded.inner.result.clone().into();
                    return Ok(bytes);
                }
            }
        }
        from_block = current;
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

async fn create_client(
    deployment: &SeededTangleTestnet,
    keystore_path: &Path,
    blueprint_id: u64,
    service_id: Option<u64>,
) -> Result<Arc<TangleClient>> {
    let config = TangleClientConfig::new(
        deployment.http_endpoint().clone(),
        deployment.ws_endpoint().clone(),
        keystore_path.display().to_string(),
        TangleSettings {
            blueprint_id,
            service_id,
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

fn seed_operator_key(path: &Path, private_key: &str) -> Result<()> {
    let config = KeystoreConfig::new().fs_root(path);
    let keystore = Keystore::new(config)?;
    let secret = Vec::from_hex(private_key)?;
    let signing_key = K256SigningKey::from_bytes(&secret)?;
    keystore.insert::<K256Ecdsa>(&signing_key)?;
    Ok(())
}

async fn grant_permitted_caller(
    rpc_endpoint: &str,
    tangle_address: Address,
    caller: Address,
) -> Result<()> {
    use tokio::time::sleep;

    let signer = PrivateKeySigner::from_str(SERVICE_OWNER_PRIVATE_KEY)
        .context("invalid service owner private key")?;
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(rpc_endpoint)
        .await?;

    let permit = addPermittedCallerCall {
        serviceId: SERVICE_ID,
        caller,
    };
    let tx = TransactionRequest::default()
        .to(tangle_address)
        .input(permit.abi_encode().into());
    let pending = provider.send_transaction(tx).await?;
    let tx_hash = *pending.tx_hash();
    loop {
        match provider.get_transaction_receipt(tx_hash).await {
            Ok(Some(_)) => break,
            Ok(None) => {
                sleep(Duration::from_millis(200)).await;
            }
            Err(err) => {
                if err.to_string().contains("BlockOutOfRange") {
                    sleep(Duration::from_millis(200)).await;
                    continue;
                }
                return Err(err.into());
            }
        }
    }
    Ok(())
}

async fn run_anvil_test<F>(name: &str, fut: F) -> Result<()>
where
    F: std::future::Future<Output = Result<()>>,
{
    timeout(ANVIL_TEST_TIMEOUT, fut)
        .await
        .with_context(|| format!("{name} timed out after {:?}", ANVIL_TEST_TIMEOUT))?
}

async fn boot_testnet(test_name: &str) -> Result<Option<SeededTangleTestnet>> {
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
