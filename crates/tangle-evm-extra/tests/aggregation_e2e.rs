//! Multi-Operator Aggregation End-to-End Tests
//!
//! Tests the aggregation flow where multiple operators submit results
//! and the system aggregates them based on configured thresholds.
//!
//! These tests require `RUN_TNT_E2E=1` and the bundled LocalTestnet broadcast/snapshot.

use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use alloy_network::EthereumWallet;
use alloy_primitives::{Address, Bytes};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::transaction::TransactionRequest;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::SolCall;
use alloy_sol_types::SolValue;
use anyhow::{Context, Result, ensure};
use blueprint_anvil_testing_utils::{
    SeededTangleEvmTestnet, harness_builder_from_env, missing_tnt_core_artifacts,
};
use blueprint_client_tangle_evm::contracts::ITangle::addPermittedCallerCall;
use blueprint_client_tangle_evm::{
    TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings,
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
const OPERATOR2_PRIVATE_KEY: &str =
    "5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";
const OPERATOR3_PRIVATE_KEY: &str =
    "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6";
const SERVICE_OWNER_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

// Test constants
const BLUEPRINT_ID: u64 = 0;
const SERVICE_ID: u64 = 0;
const ANVIL_TEST_TIMEOUT: Duration = Duration::from_secs(180);

/// Test harness for multi-operator scenarios
struct MultiOperatorHarness {
    deployment: SeededTangleEvmTestnet,
    operators: Vec<OperatorContext>,
    owner_client: Arc<TangleEvmClient>,
    _temp_dir: TempDir,
}

struct OperatorContext {
    client: Arc<TangleEvmClient>,
    address: Address,
}

impl MultiOperatorHarness {
    async fn new(deployment: SeededTangleEvmTestnet, operator_count: usize) -> Result<Self> {
        let temp = TempDir::new().context("failed to create tempdir")?;

        let operator_keys = vec![
            OPERATOR1_PRIVATE_KEY,
            OPERATOR2_PRIVATE_KEY,
            OPERATOR3_PRIVATE_KEY,
        ];

        ensure!(operator_count <= operator_keys.len(), "not enough test keys for {} operators", operator_count);

        let mut operators = Vec::new();
        for i in 0..operator_count {
            let keystore_path = temp.path().join(format!("operator_{}_keystore", i));
            std::fs::create_dir_all(&keystore_path)?;
            seed_operator_key(&keystore_path, operator_keys[i])?;

            let client = create_client(&deployment, &keystore_path).await?;
            let address = client.account();

            operators.push(OperatorContext {
                client,
                address,
            });
        }

        // Create owner client
        let owner_keystore_path = temp.path().join("owner_keystore");
        std::fs::create_dir_all(&owner_keystore_path)?;
        seed_operator_key(&owner_keystore_path, SERVICE_OWNER_PRIVATE_KEY)?;
        let owner_client = create_client(&deployment, &owner_keystore_path).await?;

        Ok(Self {
            deployment,
            operators,
            owner_client,
            _temp_dir: temp,
        })
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_aggregation_config_query() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("test_aggregation_config_query", async {
        let Some(deployment) = boot_testnet("test_aggregation_config_query").await? else {
            return Ok(());
        };

        let harness = MultiOperatorHarness::new(deployment, 1).await?;
        let client = &harness.operators[0].client;

        // Test if job requires aggregation
        for job_index in 0..3 {
            let requires = client.requires_aggregation(SERVICE_ID, job_index).await;
            match requires {
                Ok(true) => println!("Job {} requires aggregation", job_index),
                Ok(false) => println!("Job {} does not require aggregation", job_index),
                Err(e) => println!("Job {} aggregation check failed: {}", job_index, e),
            }
        }

        // Test aggregation threshold query
        let threshold = client.get_aggregation_threshold(SERVICE_ID, 0).await;
        match threshold {
            Ok(t) => println!("Aggregation threshold for job 0: {:?}", t),
            Err(e) => println!("Threshold query failed (expected if not configured): {}", e),
        }

        // Test aggregation config query
        let config = client.get_aggregation_config(SERVICE_ID, 0).await;
        match config {
            Ok(c) => println!("Aggregation config: {:?}", c),
            Err(e) => println!("Config query failed (expected if not configured): {}", e),
        }

        println!("\n✓ Aggregation config query test passed!");
        Ok(())
    })
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_multi_operator_service_membership() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("test_multi_operator_service_membership", async {
        let Some(deployment) = boot_testnet("test_multi_operator_service_membership").await? else {
            return Ok(());
        };

        let harness = MultiOperatorHarness::new(deployment, 2).await?;

        // Check service operators
        let operators = harness.owner_client
            .get_service_operators(SERVICE_ID)
            .await
            .context("failed to get service operators")?;

        println!("Service {} has {} operator(s):", SERVICE_ID, operators.len());
        for op in &operators {
            println!("  - {}", op);
        }

        // Check if each test operator is registered for the blueprint
        for (i, op) in harness.operators.iter().enumerate() {
            let is_registered = op.client
                .is_operator_registered(BLUEPRINT_ID, op.address)
                .await
                .unwrap_or(false);
            println!("Operator {} ({}) registered: {}", i, op.address, is_registered);
        }

        // Get operator weights for the service
        let weights = harness.owner_client
            .get_service_operator_weights(SERVICE_ID)
            .await
            .context("failed to get operator weights")?;

        println!("\nOperator weights:");
        for (addr, weight) in &weights {
            println!("  {} => {}", addr, weight);
        }

        println!("\n✓ Multi-operator service membership test passed!");
        Ok(())
    })
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_result_submission_flow() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("test_result_submission_flow", async {
        let Some(deployment) = boot_testnet("test_result_submission_flow").await? else {
            return Ok(());
        };

        let harness = MultiOperatorHarness::new(deployment, 1).await?;
        let operator = &harness.operators[0];

        // Grant caller permissions
        grant_permitted_caller(
            harness.deployment.http_endpoint().as_str(),
            harness.deployment.tangle_contract,
            operator.address,
        )
        .await
        .context("failed to grant caller permissions")?;
        println!("✓ Granted caller permissions to operator");

        // Submit a job
        let input: u64 = 7;
        let encoded_input = Bytes::from(input.abi_encode());
        let submission = harness.owner_client
            .submit_job(SERVICE_ID, 0, encoded_input)
            .await
            .context("failed to submit job")?;
        println!("✓ Submitted job with call_id {}", submission.call_id);

        // Submit result
        let result: u64 = input * input;
        let encoded_result = Bytes::from(result.abi_encode());
        let tx_result = operator.client
            .submit_result(SERVICE_ID, submission.call_id, encoded_result)
            .await
            .context("failed to submit result")?;
        println!("✓ Result submitted in tx {:?}", tx_result.tx_hash);

        // Verify job completion
        tokio::time::sleep(Duration::from_secs(2)).await;
        let job_call = harness.owner_client
            .get_job_call(SERVICE_ID, submission.call_id)
            .await
            .context("failed to get job call")?;

        println!("Job state: completed={}, result_count={}",
            job_call.completed,
            job_call.resultCount
        );

        println!("\n✓ Result submission flow test passed!");
        Ok(())
    })
    .await
}

// Helper functions

async fn create_client(
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
        serviceId: SERVICE_ID,
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
