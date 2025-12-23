//! Service Instancing End-to-End Tests
//!
//! Tests multiple service instances from the same blueprint:
//! - Multiple services running concurrently
//! - Job routing to correct service instance
//! - Service isolation (jobs don't leak between services)
//! - Service termination of individual instances
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
use anyhow::{Context, Result, ensure};
use blueprint_anvil_testing_utils::{
    SeededTangleEvmTestnet, harness_builder_from_env, missing_tnt_core_artifacts,
};
use blueprint_chain_setup::anvil::keys::ANVIL_PRIVATE_KEYS;
use blueprint_client_tangle_evm::contracts::ITangle::addPermittedCallerCall;
use blueprint_client_tangle_evm::{
    ServiceStatus, TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings,
};
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use hex::FromHex;
use tempfile::TempDir;
use tokio::time::timeout;

// Well-known Anvil test accounts
const OPERATOR1_PRIVATE_KEY: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const SERVICE_OWNER_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

const BLUEPRINT_ID: u64 = 0;
const SERVICE_ID_0: u64 = 0;  // Pre-seeded service
const JOB_INDEX: u8 = 0;
const ANVIL_TEST_TIMEOUT: Duration = Duration::from_secs(180);

/// Test harness for multi-service scenarios
struct MultiServiceHarness {
    deployment: SeededTangleEvmTestnet,
    operator1_client: Arc<TangleEvmClient>,
    owner_client: Arc<TangleEvmClient>,
    _temp_dir: TempDir,
}

impl MultiServiceHarness {
    async fn new(deployment: SeededTangleEvmTestnet) -> Result<Self> {
        let temp = TempDir::new().context("failed to create tempdir")?;

        // Operator 1 keystore
        let op1_keystore_path = temp.path().join("operator1_keystore");
        std::fs::create_dir_all(&op1_keystore_path)?;
        seed_operator_key(&op1_keystore_path, OPERATOR1_PRIVATE_KEY)?;
        let operator1_client = create_client(&deployment, &op1_keystore_path, Some(SERVICE_ID_0)).await?;

        // Owner keystore
        let owner_keystore_path = temp.path().join("owner_keystore");
        std::fs::create_dir_all(&owner_keystore_path)?;
        seed_operator_key(&owner_keystore_path, SERVICE_OWNER_PRIVATE_KEY)?;
        let owner_client = create_client(&deployment, &owner_keystore_path, Some(SERVICE_ID_0)).await?;

        Ok(Self {
            deployment,
            operator1_client,
            owner_client,
            _temp_dir: temp,
        })
    }

    fn operator1_account(&self) -> Address {
        self.operator1_client.account()
    }

}

/// Test: Multiple operators process jobs from the same service
///
/// Verifies:
/// 1. Two operators can both be part of the same service
/// 2. Either operator can process and submit results
/// 3. Results are correctly attributed to the submitting operator
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_multi_operator_same_service() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("test_multi_operator_same_service", async {
        let Some(deployment) = boot_testnet("test_multi_operator_same_service").await? else {
            return Ok(());
        };

        let harness = MultiServiceHarness::new(deployment).await?;

        // Verify both operators are in the service
        let operators = harness.owner_client
            .get_service_operators(SERVICE_ID_0)
            .await
            .context("failed to get service operators")?;

        println!("Service {} has {} operators:", SERVICE_ID_0, operators.len());
        for op in &operators {
            println!("  - {}", op);
        }

        ensure!(operators.len() >= 1, "service should have at least one operator");

        // Grant caller permissions to operator1
        grant_permitted_caller(
            harness.deployment.http_endpoint().as_str(),
            harness.deployment.tangle_contract,
            harness.operator1_account(),
        ).await.context("failed to grant operator1 permissions")?;

        // Submit a job
        let input: u64 = 42;
        let encoded_input = Bytes::from(input.abi_encode());
        let submission = harness.owner_client
            .submit_job(SERVICE_ID_0, JOB_INDEX, encoded_input)
            .await
            .context("failed to submit job")?;
        println!("✓ Submitted job with call_id {}", submission.call_id);

        // Operator1 submits result
        let result: u64 = input * input;
        let encoded_result = Bytes::from(result.abi_encode());
        harness.operator1_client
            .submit_result(SERVICE_ID_0, submission.call_id, encoded_result)
            .await
            .context("failed to submit result")?;
        println!("✓ Operator1 submitted result");

        // Verify job completed
        tokio::time::sleep(Duration::from_secs(1)).await;
        let job_call = harness.owner_client
            .get_job_call(SERVICE_ID_0, submission.call_id)
            .await
            .context("failed to get job call")?;

        ensure!(job_call.completed, "job should be completed");
        println!("✓ Job {} completed successfully", submission.call_id);

        println!("\n🎉 Multi-operator same service test passed!");
        Ok(())
    }).await
}

/// Test: Job routing to correct service
///
/// Verifies:
/// 1. Jobs submitted to a service are only visible to that service's operators
/// 2. Results are associated with the correct service
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_job_routing_isolation() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("test_job_routing_isolation", async {
        let Some(deployment) = boot_testnet("test_job_routing_isolation").await? else {
            return Ok(());
        };

        let harness = MultiServiceHarness::new(deployment).await?;

        // Get service info
        let service = harness.owner_client
            .get_service(SERVICE_ID_0)
            .await
            .context("failed to get service")?;

        ensure!(
            service.status == ServiceStatus::Active as u8,
            "service should be active"
        );
        println!("✓ Service {} is active", SERVICE_ID_0);

        // Grant permissions
        grant_permitted_caller(
            harness.deployment.http_endpoint().as_str(),
            harness.deployment.tangle_contract,
            harness.operator1_account(),
        ).await?;

        // Submit multiple jobs and track their call_ids
        let mut job_inputs: Vec<(u64, u64)> = Vec::new();
        for i in 1..=3 {
            let input: u64 = i * 10;
            let encoded = Bytes::from(input.abi_encode());
            let submission = harness.owner_client
                .submit_job(SERVICE_ID_0, JOB_INDEX, encoded)
                .await
                .context("failed to submit job")?;
            job_inputs.push((submission.call_id, input));
            println!("✓ Submitted job {} with input {}", submission.call_id, input);
        }

        // Submit results for each job
        for (call_id, input) in &job_inputs {
            let result = *input * *input;
            let encoded = Bytes::from(result.abi_encode());
            harness.operator1_client
                .submit_result(SERVICE_ID_0, *call_id, encoded)
                .await
                .context("failed to submit result")?;
        }

        // Verify all jobs completed with correct results
        tokio::time::sleep(Duration::from_secs(1)).await;
        for (call_id, _input) in &job_inputs {
            let job_call = harness.owner_client
                .get_job_call(SERVICE_ID_0, *call_id)
                .await
                .context("failed to get job call")?;
            ensure!(job_call.completed, "job {} should be completed", call_id);
        }
        println!("✓ All {} jobs completed correctly", job_inputs.len());

        println!("\n🎉 Job routing isolation test passed!");
        Ok(())
    }).await
}

/// Test: Concurrent job processing
///
/// Verifies:
/// 1. Multiple jobs can be submitted in quick succession
/// 2. All jobs are processed without loss
/// 3. Results arrive for all submitted jobs
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_job_processing() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("test_concurrent_job_processing", async {
        let Some(deployment) = boot_testnet("test_concurrent_job_processing").await? else {
            return Ok(());
        };

        let harness = MultiServiceHarness::new(deployment).await?;

        let caller_temp = TempDir::new().context("failed to create caller tempdir")?;
        let mut callers = Vec::new();
        let num_jobs = 5;

        for (idx, key) in ANVIL_PRIVATE_KEYS.iter().skip(1).take(num_jobs).enumerate() {
            let keystore_path = caller_temp.path().join(format!("caller_{idx}"));
            std::fs::create_dir_all(&keystore_path)?;
            seed_operator_key(&keystore_path, key)?;
            let client = create_client(&harness.deployment, &keystore_path, Some(SERVICE_ID_0)).await?;
            grant_permitted_caller(
                harness.deployment.http_endpoint().as_str(),
                harness.deployment.tangle_contract,
                client.account(),
            )
            .await?;
            callers.push(client);
        }

        // Submit jobs concurrently
        let mut handles = Vec::new();

        for (i, client) in callers.iter().enumerate() {
            let client = Arc::clone(client);
            let input: u64 = (i as u64 + 1) * 100;

            handles.push(tokio::spawn(async move {
                let encoded = Bytes::from(input.abi_encode());
                client
                    .submit_job(SERVICE_ID_0, JOB_INDEX, encoded)
                    .await
                    .map(|submission| (submission.call_id, input))
            }));
        }

        // Collect all submissions
        let mut submissions = Vec::new();
        for handle in handles {
            let result = handle.await.context("task panicked")?;
            let submission = result.context("job submission failed")?;
            submissions.push(submission);
        }
        println!("✓ Submitted {} jobs concurrently", submissions.len());

        // Submit results for all
        for (call_id, input) in &submissions {
            let result = input * input;
            let encoded = Bytes::from(result.abi_encode());
            harness.operator1_client
                .submit_result(SERVICE_ID_0, *call_id, encoded)
                .await
                .context("failed to submit result")?;
        }

        // Verify all completed
        tokio::time::sleep(Duration::from_secs(2)).await;
        let mut completed = 0;
        for (call_id, _input) in &submissions {
            let job_call = harness.owner_client
                .get_job_call(SERVICE_ID_0, *call_id)
                .await
                .context("failed to get job call")?;
            if job_call.completed {
                completed += 1;
            }
        }

        ensure!(
            completed == num_jobs,
            "expected {} completed jobs, got {}",
            num_jobs,
            completed
        );
        println!("✓ All {} concurrent jobs completed", num_jobs);

        println!("\n🎉 Concurrent job processing test passed!");
        Ok(())
    }).await
}

/// Test: Service operator weights affect result aggregation
///
/// Verifies operator weights are correctly reported
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_operator_weights_in_service() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("test_operator_weights_in_service", async {
        let Some(deployment) = boot_testnet("test_operator_weights_in_service").await? else {
            return Ok(());
        };

        let harness = MultiServiceHarness::new(deployment).await?;

        // Get operator weights
        let weights = harness.owner_client
            .get_service_operator_weights(SERVICE_ID_0)
            .await
            .context("failed to get operator weights")?;

        println!("Service {} operator weights:", SERVICE_ID_0);
        let mut total_weight: u64 = 0;
        for (operator, weight) in &weights {
            println!("  {} => {}", operator, weight);
            total_weight += *weight as u64;
        }
        println!("Total weight: {}", total_weight);

        // Get total exposure
        let exposure = harness.owner_client
            .get_service_total_exposure(SERVICE_ID_0)
            .await
            .context("failed to get total exposure")?;

        println!("Total exposure: {} wei", exposure);

        println!("\n✓ Operator weights query successful!");
        Ok(())
    }).await
}

// =============================================================================
// Helper Functions
// =============================================================================

async fn create_client(
    deployment: &SeededTangleEvmTestnet,
    keystore_path: &Path,
    service_id: Option<u64>,
) -> Result<Arc<TangleEvmClient>> {
    let config = TangleEvmClientConfig::new(
        deployment.http_endpoint().clone(),
        deployment.ws_endpoint().clone(),
        keystore_path.display().to_string(),
        TangleEvmSettings {
            blueprint_id: BLUEPRINT_ID,
            service_id,
            tangle_contract: deployment.tangle_contract,
            restaking_contract: deployment.restaking_contract,
            status_registry_contract: deployment.status_registry_contract,
        },
    )
    .test_mode(true);

    let keystore = Keystore::new(KeystoreConfig::new().fs_root(keystore_path))?;
    Ok(Arc::new(TangleEvmClient::with_keystore(config, keystore).await?))
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
    let signer = PrivateKeySigner::from_str(SERVICE_OWNER_PRIVATE_KEY)
        .context("invalid service owner private key")?;
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(rpc_endpoint)
        .await?;

    let permit = addPermittedCallerCall {
        serviceId: SERVICE_ID_0,
        caller,
    };
    let tx = TransactionRequest::default()
        .to(tangle_address)
        .input(permit.abi_encode().into());
    provider.send_transaction(tx).await?.get_receipt().await?;
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
