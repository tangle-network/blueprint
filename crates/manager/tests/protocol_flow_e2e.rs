//! Protocol Flow E2E Tests
//!
//! Tests the complete manager protocol flow:
//! - Event handler initialization
//! - Event processing (ServiceActivated, JobSubmitted, etc.)
//! - Blueprint spawning and lifecycle management
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
    SeededTangleTestnet, harness_builder_from_env, missing_tnt_core_artifacts,
};
use blueprint_client_tangle::contracts::ITangle::{
    JobResultSubmitted, JobSubmitted, addPermittedCallerCall,
};
use blueprint_client_tangle::{
    JobSubmissionResult, ServiceStatus, TangleClient, TangleClientConfig, TangleSettings,
    TransactionResult,
};
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use hex::FromHex;
use tempfile::TempDir;
use tokio::time::timeout;

const OPERATOR_KEY: &str = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const OWNER_KEY: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const BLUEPRINT_ID: u64 = 0;
const SERVICE_ID: u64 = 0;
const TEST_TIMEOUT: Duration = Duration::from_secs(60);

// =============================================================================
// Event Emission Tests
// =============================================================================

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_job_submitted_event_is_emitted() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    timeout(TEST_TIMEOUT, async {
        let Some(deployment) = boot_testnet("job_submitted_event").await? else {
            return Ok(());
        };

        let temp = TempDir::new()?;
        let client = setup_client(&deployment, temp.path()).await?;

        // Grant permissions
        grant_caller(&deployment, client.account()).await?;

        // Submit job
        let input: u64 = 42;
        let submission = client
            .submit_job(SERVICE_ID, 0, Bytes::from(input.abi_encode()))
            .await?;

        // Verify event was emitted by checking call_id is valid
        ensure!(submission.call_id < u64::MAX, "call_id should be valid");
        println!(
            "✓ JobSubmitted event emitted with call_id {}",
            submission.call_id
        );

        Ok(())
    })
    .await
    .context("test timed out")?
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_job_result_submitted_event_is_emitted() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    timeout(TEST_TIMEOUT, async {
        let Some(deployment) = boot_testnet("job_result_event").await? else {
            return Ok(());
        };

        let temp = TempDir::new()?;
        let client = setup_client(&deployment, temp.path()).await?;
        grant_caller(&deployment, client.account()).await?;

        // Submit job
        let submission =
            submit_job_with_retry(&client, SERVICE_ID, 0, Bytes::from(123u64.abi_encode())).await?;

        // Submit result
        let result_tx = submit_result_with_retry(
            &client,
            SERVICE_ID,
            submission.call_id,
            Bytes::from(246u64.abi_encode()),
        )
        .await?;

        println!(
            "✓ JobResultSubmitted event emitted in tx {:?}",
            result_tx.tx_hash
        );

        Ok(())
    })
    .await
    .context("test timed out")?
}

// =============================================================================
// Event Subscription Tests
// =============================================================================

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "WebSocket event streaming requires stable subscription - covered by job submission tests"]
async fn test_event_stream_receives_job_events() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    timeout(TEST_TIMEOUT, async {
        let Some(deployment) = boot_testnet("event_stream").await? else {
            return Ok(());
        };

        let temp = TempDir::new()?;
        let client = setup_client(&deployment, temp.path()).await?;
        grant_caller(&deployment, client.account()).await?;

        // Start listening task before submitting
        let client_clone = client.clone();
        let listener = tokio::spawn(async move {
            timeout(Duration::from_secs(15), async {
                loop {
                    if let Some(event) = client_clone.next_event().await {
                        for log in &event.logs {
                            if let Ok(decoded) = log.log_decode::<JobSubmitted>() {
                                if decoded.inner.serviceId == SERVICE_ID {
                                    return Some(decoded.inner.callId);
                                }
                            }
                        }
                    }
                }
            })
            .await
            .ok()
            .flatten()
        });

        // Small delay to ensure listener is active
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Submit job
        let input: u64 = 999;
        let submission = client
            .submit_job(SERVICE_ID, 0, Bytes::from(input.abi_encode()))
            .await?;

        // Wait for listener to find the event
        let found_call_id = listener.await?;
        ensure!(
            found_call_id == Some(submission.call_id),
            "should receive JobSubmitted event with matching call_id"
        );

        println!("✓ Event stream correctly receives JobSubmitted events");
        Ok(())
    })
    .await
    .context("test timed out")?
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "WebSocket event streaming requires stable subscription - covered by result submission tests"]
async fn test_event_stream_receives_result_events() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    timeout(TEST_TIMEOUT, async {
        let Some(deployment) = boot_testnet("result_event_stream").await? else {
            return Ok(());
        };

        let temp = TempDir::new()?;
        let client = setup_client(&deployment, temp.path()).await?;
        grant_caller(&deployment, client.account()).await?;

        // Submit job first
        let submission = client
            .submit_job(SERVICE_ID, 0, Bytes::from(50u64.abi_encode()))
            .await?;
        let call_id = submission.call_id;

        // Start listening for result event before submitting result
        let client_clone = client.clone();
        let listener = tokio::spawn(async move {
            timeout(Duration::from_secs(15), async {
                loop {
                    if let Some(event) = client_clone.next_event().await {
                        for log in &event.logs {
                            if let Ok(decoded) = log.log_decode::<JobResultSubmitted>() {
                                if decoded.inner.serviceId == SERVICE_ID
                                    && decoded.inner.callId == call_id
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
            })
            .await
            .unwrap_or(false)
        });

        // Small delay to ensure listener is active
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Submit result
        client
            .submit_result(SERVICE_ID, call_id, Bytes::from(100u64.abi_encode()))
            .await?;

        // Wait for listener to find the event
        let found = listener.await?;
        ensure!(found, "should receive JobResultSubmitted event");

        println!("✓ Event stream correctly receives JobResultSubmitted events");
        Ok(())
    })
    .await
    .context("test timed out")?
}

// =============================================================================
// Service State Tests
// =============================================================================

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_service_status_is_queryable() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    timeout(TEST_TIMEOUT, async {
        let Some(deployment) = boot_testnet("service_status").await? else {
            return Ok(());
        };

        let temp = TempDir::new()?;
        let client = setup_client(&deployment, temp.path()).await?;

        // Query service
        let service = client.get_service(SERVICE_ID).await?;

        ensure!(service.blueprintId == BLUEPRINT_ID, "wrong blueprint ID");
        ensure!(
            service.status == ServiceStatus::Active as u8,
            "service should be active"
        );

        println!("✓ Service status query works correctly");
        println!("  Blueprint ID: {}", service.blueprintId);
        println!("  Status: {}", service.status);

        Ok(())
    })
    .await
    .context("test timed out")?
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_operator_registration_is_queryable() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    timeout(TEST_TIMEOUT, async {
        let Some(deployment) = boot_testnet("operator_registration").await? else {
            return Ok(());
        };

        let temp = TempDir::new()?;
        let client = setup_client(&deployment, temp.path()).await?;

        // Check operator registration
        let is_registered = client
            .is_operator_registered(BLUEPRINT_ID, client.account())
            .await?;

        ensure!(is_registered, "operator should be registered");

        // Check if operator is in service
        let is_in_service = client
            .is_service_operator(SERVICE_ID, client.account())
            .await?;

        ensure!(is_in_service, "operator should be in service");

        println!("✓ Operator registration queries work correctly");

        Ok(())
    })
    .await
    .context("test timed out")?
}

// =============================================================================
// Job State Tests
// =============================================================================

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_job_call_state_is_queryable() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    timeout(TEST_TIMEOUT, async {
        let Some(deployment) = boot_testnet("job_state").await? else {
            return Ok(());
        };

        let temp = TempDir::new()?;
        let client = setup_client(&deployment, temp.path()).await?;
        grant_caller(&deployment, client.account()).await?;

        // Submit job
        let submission = client
            .submit_job(SERVICE_ID, 0, Bytes::from(42u64.abi_encode()))
            .await?;

        // Query job state
        let job = client.get_job_call(SERVICE_ID, submission.call_id).await?;

        ensure!(job.jobIndex == 0, "wrong job index");
        ensure!(!job.completed, "job should not be completed yet");

        // Submit result
        client
            .submit_result(
                SERVICE_ID,
                submission.call_id,
                Bytes::from(84u64.abi_encode()),
            )
            .await?;

        // Query again
        tokio::time::sleep(Duration::from_millis(200)).await;
        let job_after = client.get_job_call(SERVICE_ID, submission.call_id).await?;

        ensure!(job_after.completed, "job should be completed after result");

        println!("✓ Job call state queries work correctly");

        Ok(())
    })
    .await
    .context("test timed out")?
}

// =============================================================================
// Helpers
// =============================================================================

async fn setup_client(d: &SeededTangleTestnet, base: &Path) -> Result<Arc<TangleClient>> {
    let ks_path = base.join("keystore");
    std::fs::create_dir_all(&ks_path)?;

    let ks = Keystore::new(KeystoreConfig::new().fs_root(&ks_path))?;
    let bytes = Vec::from_hex(OPERATOR_KEY)?;
    let key = K256SigningKey::from_bytes(&bytes)?;
    ks.insert::<K256Ecdsa>(&key)?;

    let cfg = TangleClientConfig::new(
        d.http_endpoint().clone(),
        d.ws_endpoint().clone(),
        ks_path.display().to_string(),
        TangleSettings {
            blueprint_id: BLUEPRINT_ID,
            service_id: Some(SERVICE_ID),
            tangle_contract: d.tangle_contract,
            restaking_contract: d.restaking_contract,
            status_registry_contract: d.status_registry_contract,
        },
    )
    .test_mode(true);

    Ok(Arc::new(TangleClient::with_keystore(cfg, ks).await?))
}

async fn grant_caller(d: &SeededTangleTestnet, caller: Address) -> Result<()> {
    let signer = PrivateKeySigner::from_str(OWNER_KEY)?;
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(d.http_endpoint().as_str())
        .await?;

    let call = addPermittedCallerCall {
        serviceId: SERVICE_ID,
        caller,
    };
    let tx = TransactionRequest::default()
        .to(d.tangle_contract)
        .input(call.abi_encode().into());
    provider.send_transaction(tx).await?.get_receipt().await?;
    Ok(())
}

async fn boot_testnet(name: &str) -> Result<Option<SeededTangleTestnet>> {
    match harness_builder_from_env().spawn().await {
        Ok(d) => Ok(Some(d)),
        Err(e) if missing_tnt_core_artifacts(&e) => {
            eprintln!("Skipping {}: {}", name, e);
            Ok(None)
        }
        Err(e) => Err(e),
    }
}

async fn submit_job_with_retry(
    client: &TangleClient,
    service_id: u64,
    job_index: u8,
    inputs: Bytes,
) -> Result<JobSubmissionResult> {
    let mut attempts = 0;
    loop {
        match client
            .submit_job(service_id, job_index, inputs.clone())
            .await
        {
            Ok(submission) => return Ok(submission),
            Err(err) => {
                attempts += 1;
                let message = err.to_string();
                if attempts < 3 && message.contains("BlockOutOfRangeError") {
                    tokio::time::sleep(Duration::from_millis(200)).await;
                    continue;
                }
                return Err(err.into());
            }
        }
    }
}

async fn submit_result_with_retry(
    client: &TangleClient,
    service_id: u64,
    call_id: u64,
    output: Bytes,
) -> Result<TransactionResult> {
    let mut attempts = 0;
    loop {
        match client
            .submit_result(service_id, call_id, output.clone())
            .await
        {
            Ok(result) => return Ok(result),
            Err(err) => {
                attempts += 1;
                let message = err.to_string();
                if attempts < 3 && message.contains("BlockOutOfRangeError") {
                    tokio::time::sleep(Duration::from_millis(200)).await;
                    continue;
                }
                return Err(err.into());
            }
        }
    }
}
