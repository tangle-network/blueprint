use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use anyhow::{Context, Result};
use blueprint_anvil_testing_utils::{BlueprintHarness, missing_tnt_core_artifacts};
use hello_tangle_blueprint::{
    CREATE_DOCUMENT_JOB, DocumentReceipt, DocumentRequest, clear_store, router,
};
use tokio::time::timeout;

const ANVIL_TEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(600);

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn creates_and_reads_documents() -> Result<()> {
    timeout(ANVIL_TEST_TIMEOUT, async {
        clear_store().await;

        let harness = match BlueprintHarness::builder(router())
            .poll_interval(std::time::Duration::from_millis(50))
            .spawn()
            .await
        {
            Ok(harness) => harness,
            Err(err) => {
                if missing_tnt_core_artifacts(&err) {
                    eprintln!("Skipping creates_and_reads_documents: {err}");
                    return Ok(());
                }
                return Err(err);
            }
        };

        let client = harness.client();
        let service = client.get_service(harness.service_id()).await?;
        println!(
            "Service {:?} status {:?}, owner {:#x}, pricing {:?}",
            harness.service_id(),
            service.status,
            service.owner,
            service.pricing
        );
        println!(
            "Owner permitted? {} Operator permitted? {}",
            client
                .tangle_contract()
                .isPermittedCaller(harness.service_id(), service.owner)
                .call()
                .await
                .unwrap_or(false),
            client
                .tangle_contract()
                .isPermittedCaller(harness.service_id(), client.account())
                .call()
                .await
                .unwrap_or(false)
        );

        let payload = DocumentRequest {
            docId: "doc-42".to_string(),
            contents: "hello-tangle".to_string(),
        }
        .abi_encode();

        let submission = harness
            .submit_job(CREATE_DOCUMENT_JOB, Bytes::from(payload))
            .await?;
        let output = harness.wait_for_job_result(submission).await?;
        let receipt = DocumentReceipt::abi_decode(&output)?;

        assert_eq!(receipt.docId, "doc-42");
        assert_eq!(receipt.contents, "hello-tangle");
        assert!(
            receipt.operator.starts_with("0x"),
            "operator metadata should be hex-encoded"
        );

        harness.shutdown().await;
        Ok(())
    })
    .await
    .with_context(|| {
        format!(
            "creates_and_reads_documents timed out after {:?}",
            ANVIL_TEST_TIMEOUT
        )
    })?
}
