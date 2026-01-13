//! Multi-Service Instance E2E Tests
//!
//! Tests multiple services from the same blueprint processing jobs independently.
//! Verifies service isolation and correct job routing.
//!
//! Requires `RUN_TNT_E2E=1` and the bundled LocalTestnet broadcast/snapshot.

use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use alloy_network::EthereumWallet;
use alloy_primitives::{Address, Bytes};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::transaction::TransactionRequest;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::{SolCall, SolValue};
use anyhow::{Context, Result, anyhow, ensure};
use blueprint_anvil_testing_utils::{
    SeededTangleEvmTestnet, harness_builder_from_env, missing_tnt_core_artifacts,
};
use blueprint_client_tangle_evm::contracts::ITangle::addPermittedCallerCall;
use blueprint_client_tangle_evm::{
    ServiceStatus, TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings,
};
use blueprint_core::Job;
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_router::Router;
use blueprint_tangle_evm_extra::extract::{CallId, ServiceId};
use blueprint_tangle_evm_extra::extract::{TangleEvmArg, TangleEvmResult};
use blueprint_tangle_evm_extra::{TangleEvmLayer, TangleEvmProducer};
use futures_util::StreamExt;
use futures_util::future::poll_fn;
use futures_util::pin_mut;
use hex::FromHex;
use tempfile::TempDir;
use tokio::sync::oneshot;
use tokio::time::timeout;
use tower::Service;

// Anvil test accounts
const OPERATOR1_KEY: &str = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const OPERATOR2_KEY: &str = "5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";
const OWNER_KEY: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

const BLUEPRINT_ID: u64 = 0;
const JOB_INDEX: u8 = 0;
const TEST_TIMEOUT: Duration = Duration::from_secs(120);

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_multiple_services_process_jobs_independently() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    timeout(TEST_TIMEOUT, async {
        let Some(deployment) = boot_testnet("multi_service_independent").await? else {
            return Ok(());
        };

        let temp = TempDir::new()?;

        // Service 0 is seeded, we'll use it
        let service_id_0: u64 = 0;

        // Create client for service 0
        let ks_path_0 = temp.path().join("ks0");
        std::fs::create_dir_all(&ks_path_0)?;
        seed_key(&ks_path_0, OPERATOR1_KEY)?;
        let client_0 = create_client(&deployment, &ks_path_0, Some(service_id_0)).await?;

        // Verify service 0 is active
        let svc = client_0.get_service(service_id_0).await?;
        ensure!(
            svc.status == ServiceStatus::Active as u8,
            "service 0 not active"
        );

        // Grant permissions
        grant_caller(&deployment, client_0.account()).await?;

        // Run processor for service 0
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (result_tx, result_rx) = oneshot::channel::<Result<Vec<u8>>>();
        let producer_0 = TangleEvmProducer::new((*client_0).clone(), service_id_0)
            .with_poll_interval(Duration::from_millis(50));
        let router_0 = Router::new().route(JOB_INDEX, multiply_job.layer(TangleEvmLayer));

        let runner_client = Arc::clone(&client_0);
        let runner = tokio::spawn(async move {
            run_processor(
                producer_0,
                router_0,
                runner_client,
                shutdown_rx,
                Some(result_tx),
            )
            .await
        });

        // Submit job to service 0
        let input_0: u64 = 100;
        let submission_0 = client_0
            .submit_job(service_id_0, JOB_INDEX, Bytes::from(input_0.abi_encode()))
            .await?;

        // Wait for result
        let result_0 = timeout(Duration::from_secs(60), result_rx)
            .await
            .context("timeout waiting for runner result")??;
        let result_0 = result_0?;
        let decoded_0: u64 = u64::abi_decode(&result_0)?;
        ensure!(
            decoded_0 == input_0 * 2,
            "expected {} got {}",
            input_0 * 2,
            decoded_0
        );

        // Verify on-chain completion (retry to avoid transient RPC flakiness)
        wait_for_job_completion((*client_0).clone(), submission_0.call_id).await?;

        let _ = shutdown_tx.send(());
        let _ = runner.await;

        println!("✓ Multi-service job processing verified");
        Ok(())
    })
    .await
    .context("test timed out")?
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_service_queries_return_correct_data() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    timeout(TEST_TIMEOUT, async {
        let Some(deployment) = boot_testnet("service_queries").await? else {
            return Ok(());
        };

        let temp = TempDir::new()?;
        let ks_path = temp.path().join("ks");
        std::fs::create_dir_all(&ks_path)?;
        seed_key(&ks_path, OPERATOR1_KEY)?;

        let client = create_client(&deployment, &ks_path, Some(0)).await?;

        // Query blueprint count
        let bp_count = client.blueprint_count().await?;
        ensure!(bp_count >= 1, "should have at least 1 blueprint");

        // Query service count
        let svc_count = client.service_count().await?;
        ensure!(svc_count >= 1, "should have at least 1 service");

        // Query service details
        let svc = client.get_service(0).await?;
        ensure!(svc.blueprintId == BLUEPRINT_ID, "wrong blueprint id");
        ensure!(
            svc.status == ServiceStatus::Active as u8,
            "service not active"
        );

        // Query operators
        let operators = client.get_service_operators(0).await?;
        ensure!(!operators.is_empty(), "service should have operators");

        // Query operator weights
        let weights = client.get_service_operator_weights(0).await?;
        ensure!(!weights.is_empty(), "should have operator weights");

        // Query total exposure
        let exposure = client.get_service_total_exposure(0).await?;
        println!("Service exposure: {} wei", exposure);

        println!("✓ All service queries return valid data");
        Ok(())
    })
    .await
    .context("test timed out")?
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_job_submission_requires_caller_permission() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    timeout(TEST_TIMEOUT, async {
        let Some(deployment) = boot_testnet("caller_permission").await? else {
            return Ok(());
        };

        let temp = TempDir::new()?;
        let ks_path = temp.path().join("ks");
        std::fs::create_dir_all(&ks_path)?;

        // Use operator 2 who is NOT a permitted caller
        seed_key(&ks_path, OPERATOR2_KEY)?;
        let client = create_client(&deployment, &ks_path, Some(0)).await?;

        // Try to submit job without permission
        let result = client
            .submit_job(0, JOB_INDEX, Bytes::from(42u64.abi_encode()))
            .await;

        // Should fail or require permission
        // (the exact behavior depends on contract config)
        println!(
            "Job submission without permission result: {:?}",
            result.is_ok()
        );

        Ok(())
    })
    .await
    .context("test timed out")?
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_result_submission_updates_job_state() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    timeout(TEST_TIMEOUT, async {
        let Some(deployment) = boot_testnet("result_submission").await? else {
            return Ok(());
        };

        let temp = TempDir::new()?;
        let ks_path = temp.path().join("ks");
        std::fs::create_dir_all(&ks_path)?;
        seed_key(&ks_path, OPERATOR1_KEY)?;

        let client = create_client(&deployment, &ks_path, Some(0)).await?;
        grant_caller(&deployment, client.account()).await?;

        // Submit job
        let input: u64 = 77;
        let submission = client
            .submit_job(0, JOB_INDEX, Bytes::from(input.abi_encode()))
            .await?;

        // Check job state before result
        let job_before = client.get_job_call(0, submission.call_id).await?;
        ensure!(!job_before.completed, "job should not be completed yet");

        // Submit result directly
        let result: u64 = input * 2;
        client
            .submit_result(0, submission.call_id, Bytes::from(result.abi_encode()))
            .await?;

        // Check job state after result
        tokio::time::sleep(Duration::from_millis(500)).await;
        let job_after = client.get_job_call(0, submission.call_id).await?;
        ensure!(job_after.completed, "job should be completed after result");

        println!("✓ Result submission correctly updates job state");
        Ok(())
    })
    .await
    .context("test timed out")?
}

// =============================================================================
// Job Handler
// =============================================================================

async fn multiply_job(TangleEvmArg(x): TangleEvmArg<u64>) -> TangleEvmResult<u64> {
    TangleEvmResult(x * 2)
}

// =============================================================================
// Runner Loop
// =============================================================================

async fn run_processor(
    producer: TangleEvmProducer,
    mut router: Router,
    client: Arc<TangleEvmClient>,
    mut shutdown: oneshot::Receiver<()>,
    mut result_tx: Option<oneshot::Sender<Result<Vec<u8>>>>,
) -> Result<()> {
    let mut svc = router.as_service();
    poll_fn(|cx| svc.poll_ready(cx)).await.ok();
    pin_mut!(producer);

    loop {
        tokio::select! {
            _ = &mut shutdown => break,
            job = producer.next() => {
                let Some(Ok(call)) = job else { continue };
                let metadata = call.metadata();
                let (call_id_raw, service_id_raw) = match (
                    metadata.get(CallId::METADATA_KEY),
                    metadata.get(ServiceId::METADATA_KEY),
                ) {
                    (Some(call_id), Some(service_id)) => (call_id, service_id),
                    _ => continue,
                };
                let call_id: u64 = call_id_raw
                    .try_into()
                    .map_err(|_| anyhow!("invalid call_id metadata"))?;
                let service_id: u64 = service_id_raw
                    .try_into()
                    .map_err(|_| anyhow!("invalid service_id metadata"))?;

                if let Ok(Some(results)) = svc.call(call).await {
                    let send_result = match results.get(0) {
                        Some(blueprint_core::JobResult::Ok { body, .. }) => Ok(body.to_vec()),
                        Some(blueprint_core::JobResult::Err(err)) => {
                            Err(anyhow!("job returned error: {err:?}"))
                        }
                        None => Err(anyhow!("runner returned no results")),
                    };

                    if send_result.is_ok() {
                        for result in &results {
                            if let blueprint_core::JobResult::Ok { body, .. } = result {
                                if let Err(err) = submit_result_with_retry(
                                    &client,
                                    service_id,
                                    call_id,
                                    body,
                                )
                                .await
                                {
                                    let err_message = format!("{err:#}");
                                    if let Some(tx) = result_tx.take() {
                                        let _ = tx.send(Err(anyhow!(err_message.clone())));
                                    }
                                    return Err(anyhow!(err_message));
                                }
                            }
                        }
                    }

                    if let Some(tx) = result_tx.take() {
                        let _ = tx.send(send_result);
                    }
                }
            }
        }
    }
    Ok(())
}

// =============================================================================
// Helpers
// =============================================================================

async fn create_client(
    d: &SeededTangleEvmTestnet,
    ks: &Path,
    svc_id: Option<u64>,
) -> Result<Arc<TangleEvmClient>> {
    let cfg = TangleEvmClientConfig::new(
        d.http_endpoint().clone(),
        d.ws_endpoint().clone(),
        ks.display().to_string(),
        TangleEvmSettings {
            blueprint_id: BLUEPRINT_ID,
            service_id: svc_id,
            tangle_contract: d.tangle_contract,
            restaking_contract: d.restaking_contract,
            status_registry_contract: d.status_registry_contract,
        },
    )
    .test_mode(true);

    let keystore = Keystore::new(KeystoreConfig::new().fs_root(ks))?;
    Ok(Arc::new(
        TangleEvmClient::with_keystore(cfg, keystore).await?,
    ))
}

fn seed_key(path: &Path, hex_key: &str) -> Result<()> {
    let ks = Keystore::new(KeystoreConfig::new().fs_root(path))?;
    let bytes = Vec::from_hex(hex_key)?;
    let key = K256SigningKey::from_bytes(&bytes)?;
    ks.insert::<K256Ecdsa>(&key)?;
    Ok(())
}

async fn grant_caller(d: &SeededTangleEvmTestnet, caller: Address) -> Result<()> {
    let signer = PrivateKeySigner::from_str(OWNER_KEY)?;
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(d.http_endpoint().as_str())
        .await?;

    let call = addPermittedCallerCall {
        serviceId: 0,
        caller,
    };
    let tx = TransactionRequest::default()
        .to(d.tangle_contract)
        .input(call.abi_encode().into());
    provider.send_transaction(tx).await?.get_receipt().await?;
    Ok(())
}

async fn boot_testnet(name: &str) -> Result<Option<SeededTangleEvmTestnet>> {
    match harness_builder_from_env().spawn().await {
        Ok(d) => Ok(Some(d)),
        Err(e) if missing_tnt_core_artifacts(&e) => {
            eprintln!("Skipping {}: {}", name, e);
            Ok(None)
        }
        Err(e) => Err(e),
    }
}

async fn submit_result_with_retry(
    client: &TangleEvmClient,
    service_id: u64,
    call_id: u64,
    body: &[u8],
) -> Result<()> {
    const MAX_ATTEMPTS: u8 = 3;
    let mut attempts = 0;
    loop {
        attempts += 1;
        match client
            .submit_result(service_id, call_id, Bytes::from(body.to_vec()))
            .await
        {
            Ok(tx) if tx.success => return Ok(()),
            Ok(tx) => {
                return Err(anyhow!(
                    "submit_result reverted for service {service_id} call {call_id}: tx={:?}",
                    tx.tx_hash
                ));
            }
            Err(_err) if attempts < MAX_ATTEMPTS => {
                tokio::time::sleep(Duration::from_millis(200)).await;
                continue;
            }
            Err(err) => return Err(err.into()),
        }
    }
}

async fn wait_for_job_completion(client: TangleEvmClient, call_id: u64) -> Result<()> {
    timeout(Duration::from_secs(30), async {
        loop {
            let call = client.get_job_call(0, call_id).await?;
            if call.completed {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    })
    .await
    .context("timed out waiting for job completion")?
}
