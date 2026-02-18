use alloy_network::EthereumWallet;
use alloy_primitives::Bytes;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::transaction::TransactionRequest;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::{SolCall, SolValue};
use anyhow::{Context, Result, bail};
use blueprint_anvil_testing_utils::{BlueprintHarness, missing_tnt_core_artifacts};
use blueprint_client_tangle::contracts::ITangle::addPermittedCallerCall;
use oauth_blueprint_lib::{
    ADMIN_PURGE_JOB_ID, AdminPurgeResult, WRITE_DOC_JOB_ID, WriteDocResult, reset_state_for_tests,
    router,
};
use once_cell::sync::Lazy;
use std::sync::Once;
use std::time::Duration;
use tokio::sync::Mutex as AsyncMutex;
use tokio::time::timeout;
use tracing_subscriber::{EnvFilter, fmt};

const ANVIL_TEST_TIMEOUT: Duration = Duration::from_secs(600);
const SERVICE_OWNER_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const UNAUTHORIZED_CALLER_PRIVATE_KEY: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
static HARNESS_LOCK: Lazy<AsyncMutex<()>> = Lazy::new(|| AsyncMutex::new(()));
static LOG_INIT: Once = Once::new();

fn setup_log() {
    LOG_INIT.call_once(|| {
        let _ = fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();
    });
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn writes_and_purges_documents() -> Result<()> {
    setup_log();
    let guard = HARNESS_LOCK.lock().await;
    let result = timeout(ANVIL_TEST_TIMEOUT, async {
        reset_state_for_tests().await;

        let Some(harness) = spawn_harness().await? else {
            return Ok(());
        };
        permit_submitter(&harness).await?;

        let write_payload = (
            "doc-7".to_string(),
            "oauth data".to_string(),
            "tenant.alpha".to_string(),
        )
            .abi_encode();
        let submission = harness
            .submit_job(WRITE_DOC_JOB_ID, Bytes::from(write_payload))
            .await?;
        let output = harness
            .wait_for_job_result_with_deadline(submission, Duration::from_secs(120))
            .await?;
        let receipt = WriteDocResult::abi_decode(&output)?;
        assert!(receipt.ok);
        assert_eq!(receipt.docId, "doc-7");

        let purge_payload = "tenant.alpha".to_string().abi_encode();
        let purge_submission = harness
            .submit_job(ADMIN_PURGE_JOB_ID, Bytes::from(purge_payload))
            .await?;
        let purge_output = harness
            .wait_for_job_result_with_deadline(purge_submission, Duration::from_secs(120))
            .await?;
        let purge = AdminPurgeResult::abi_decode(&purge_output)?;
        assert!(purge.purged);
        assert_eq!(purge.target, "tenant.alpha");

        harness.shutdown().await;
        Ok(())
    })
    .await;
    drop(guard);
    result.context("writes_and_purges_documents timed out")?
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn rejects_unpermitted_submitters() -> Result<()> {
    setup_log();
    let guard = HARNESS_LOCK.lock().await;
    let result = timeout(ANVIL_TEST_TIMEOUT, async {
        reset_state_for_tests().await;

        let Some(harness) = spawn_harness().await? else {
            return Ok(());
        };

        let payload = (
            "doc-unauthorized".to_string(),
            "oauth data".to_string(),
            "tenant.beta".to_string(),
        )
            .abi_encode();

        match harness
            .submit_job_with_private_key(
                UNAUTHORIZED_CALLER_PRIVATE_KEY,
                WRITE_DOC_JOB_ID,
                Bytes::from(payload),
            )
            .await
        {
            Ok(_) => bail!("submit_job unexpectedly succeeded without a permit"),
            Err(err) => {
                let matched = err.chain().any(|cause| {
                    let text = cause.to_string();
                    text.contains("JobSubmitted")
                        || text.contains("permit")
                        || text.contains("failed to submit job")
                });
                assert!(matched, "unexpected submit error {err}");
            }
        }

        harness.shutdown().await;
        Ok(())
    })
    .await;
    drop(guard);
    result.context("rejects_unpermitted_submitters timed out")?
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn drop_impl_cleans_up_without_shutdown() -> Result<()> {
    setup_log();
    let guard = HARNESS_LOCK.lock().await;
    let result = timeout(ANVIL_TEST_TIMEOUT, async {
        reset_state_for_tests().await;

        {
            let Some(harness) = spawn_harness().await? else {
                return Ok(());
            };
            permit_submitter(&harness).await?;

            let payload = (
                "doc-alpha".to_string(),
                "payload".to_string(),
                "tenant.gamma".to_string(),
            )
                .abi_encode();
            let submission = harness
                .submit_job(WRITE_DOC_JOB_ID, Bytes::from(payload))
                .await?;
            let output = harness
                .wait_for_job_result_with_deadline(submission, Duration::from_secs(120))
                .await?;
            let receipt = WriteDocResult::abi_decode(&output)?;
            assert!(receipt.ok);
            assert_eq!(receipt.docId, "doc-alpha");

            // allow Drop to close the harness without shutdown
        }

        reset_state_for_tests().await;

        let Some(harness) = spawn_harness().await? else {
            return Ok(());
        };
        permit_submitter(&harness).await?;

        let doc_payload = (
            "doc-beta".to_string(),
            "payload".to_string(),
            "tenant.gamma".to_string(),
        )
            .abi_encode();
        let write_submission = harness
            .submit_job(WRITE_DOC_JOB_ID, Bytes::from(doc_payload))
            .await?;
        let write_output = harness
            .wait_for_job_result_with_deadline(write_submission, Duration::from_secs(120))
            .await?;
        let write_receipt = WriteDocResult::abi_decode(&write_output)?;
        assert!(write_receipt.ok);
        assert_eq!(write_receipt.docId, "doc-beta");

        harness.shutdown().await;
        Ok(())
    })
    .await;
    drop(guard);
    result.context("drop_impl_cleans_up_without_shutdown timed out")?
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn bad_payload_surfaces_decode_error() -> Result<()> {
    setup_log();
    let guard = HARNESS_LOCK.lock().await;
    let result = timeout(ANVIL_TEST_TIMEOUT, async {
        reset_state_for_tests().await;

        let Some(harness) = spawn_harness().await? else {
            return Ok(());
        };
        permit_submitter(&harness).await?;

        let submission = harness
            .submit_job(WRITE_DOC_JOB_ID, Bytes::from_static(&[5u8, 7u8, 9u8]))
            .await?;

        let err = harness
            .wait_for_job_result_with_deadline(submission, Duration::from_secs(5))
            .await
            .expect_err("decode failure should bubble through harness");
        let msg = err.to_string();
        assert!(
            msg.contains("timed out") || msg.contains("decode") || msg.contains("Decode"),
            "expected timeout or decode error, got {err}"
        );

        harness.shutdown().await;
        Ok(())
    })
    .await;
    drop(guard);
    result.context("bad_payload_surfaces_decode_error timed out")?
}

async fn spawn_harness() -> Result<Option<BlueprintHarness>> {
    match BlueprintHarness::builder(router())
        .poll_interval(Duration::from_millis(50))
        .spawn()
        .await
    {
        Ok(harness) => Ok(Some(harness)),
        Err(err) => {
            if missing_tnt_core_artifacts(&err) {
                eprintln!("Skipping OAuth harness test: {err}");
                Ok(None)
            } else {
                Err(err)
            }
        }
    }
}

async fn permit_submitter(harness: &BlueprintHarness) -> Result<()> {
    use std::str::FromStr;

    let signer = PrivateKeySigner::from_str(SERVICE_OWNER_PRIVATE_KEY)?;
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(harness.deployment().http_endpoint().as_str())
        .await?;

    let client = harness.client();
    let permit = addPermittedCallerCall {
        serviceId: harness.service_id(),
        caller: client.account(),
    };
    let tx = TransactionRequest::default()
        .to(client.tangle_address())
        .input(permit.abi_encode().into());
    provider.send_transaction(tx).await?.get_receipt().await?;
    Ok(())
}
