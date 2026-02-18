use alloy_primitives::{Address, Bytes};
use alloy_sol_types::SolValue;
use anyhow::{Context, Result, bail};
use apikey_blueprint_lib::{
    PURCHASE_API_KEY_JOB_ID, PurchaseApiKeyResult, WRITE_RESOURCE_JOB_ID, WriteResourceResult,
    reset_state_for_tests, router,
};
use blueprint_anvil_testing_utils::{BlueprintHarness, missing_tnt_core_artifacts};
use once_cell::sync::Lazy;
use std::sync::Once;
use std::time::Duration;
use tokio::sync::Mutex as AsyncMutex;
use tokio::time::timeout;
use tracing_subscriber::{EnvFilter, fmt};

const ANVIL_TEST_TIMEOUT: Duration = Duration::from_secs(600);
const JOB_RESULT_TIMEOUT: Duration = Duration::from_secs(300);
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
async fn writes_and_purchases_resources() -> Result<()> {
    setup_log();
    let guard = HARNESS_LOCK.lock().await;
    let result = timeout(ANVIL_TEST_TIMEOUT, async {
        reset_state_for_tests().await;

        let Some(harness) = spawn_harness().await? else {
            return Ok(());
        };

        let service = harness
            .client()
            .get_service(harness.service_id())
            .await
            .context("failed to fetch service metadata")?;
        println!(
            "Service {} operator count: {}",
            harness.service_id(),
            service.operatorCount
        );
        let operators = harness
            .client()
            .get_service_operators(harness.service_id())
            .await
            .context("failed to list operators")?;
        println!("Operators: {:?}", operators);

        let account = Address::repeat_byte(0x11);
        let write_payload = (
            "config".to_string(),
            "app configuration data".to_string(),
            account,
        )
            .abi_encode();
        let submission = harness
            .submit_job(WRITE_RESOURCE_JOB_ID, Bytes::from(write_payload))
            .await?;
        println!("Submitted call {}", submission.call_id);
        let output = harness
            .wait_for_job_result_with_deadline(submission, JOB_RESULT_TIMEOUT)
            .await?;
        let receipt = WriteResourceResult::abi_decode(&output)?;
        assert!(receipt.ok, "write_resource should succeed");
        assert_eq!(receipt.resourceId, "config");

        let purchase_payload = ("premium".to_string(), account).abi_encode();
        let purchase = harness
            .submit_job(PURCHASE_API_KEY_JOB_ID, Bytes::from(purchase_payload))
            .await?;
        println!("Purchase call {}", purchase.call_id);
        let purchase_output = harness
            .wait_for_job_result_with_deadline(purchase, JOB_RESULT_TIMEOUT)
            .await?;
        let purchase_receipt = PurchaseApiKeyResult::abi_decode(&purchase_output)?;
        assert!(purchase_receipt.ok);
        assert!(
            !purchase_receipt.apiKeyHash.is_empty(),
            "purchase job should return api key hash"
        );

        harness.shutdown().await;
        Ok(())
    })
    .await;
    drop(guard);
    result.context("writes_and_purchases_resources timed out")?
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
            "config".to_string(),
            "invalid writers".to_string(),
            Address::repeat_byte(0xAA),
        )
            .abi_encode();

        match harness
            .submit_job_with_private_key(
                UNAUTHORIZED_CALLER_PRIVATE_KEY,
                WRITE_RESOURCE_JOB_ID,
                Bytes::from(payload),
            )
            .await
        {
            Ok(_) => bail!("submit_job unexpectedly succeeded without permit"),
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
            let payload = (
                "scratch".to_string(),
                "drop test".to_string(),
                Address::repeat_byte(0xBB),
            )
                .abi_encode();
            let submission = harness
                .submit_job(WRITE_RESOURCE_JOB_ID, Bytes::from(payload))
                .await?;
            let output = harness.wait_for_job_result(submission).await?;
            let receipt = WriteResourceResult::abi_decode(&output)?;
            assert!(receipt.ok);
            assert_eq!(receipt.resourceId, "scratch");

            // harness dropped at end of scope without an explicit shutdown
        }

        reset_state_for_tests().await;

        let Some(harness) = spawn_harness().await? else {
            return Ok(());
        };
        let payload = (
            "scratch-2".to_string(),
            "drop-second-pass".to_string(),
            Address::repeat_byte(0xCC),
        )
            .abi_encode();
        let submission = harness
            .submit_job(WRITE_RESOURCE_JOB_ID, Bytes::from(payload))
            .await?;
        let output = harness.wait_for_job_result(submission).await?;
        let receipt = WriteResourceResult::abi_decode(&output)?;
        assert!(receipt.ok);
        assert_eq!(receipt.resourceId, "scratch-2");

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

        let submission = harness
            .submit_job(WRITE_RESOURCE_JOB_ID, Bytes::from_static(&[0u8, 1u8, 2u8]))
            .await?;

        let err = harness
            .wait_for_job_result_with_deadline(submission, Duration::from_secs(5))
            .await
            .expect_err("decode failure should prevent result submission");
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
                eprintln!("Skipping API key harness test: {err}");
                Ok(None)
            } else {
                Err(err)
            }
        }
    }
}
