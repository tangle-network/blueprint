//! Integration test for the TangleEvmProducer backed by the Anvil harness.
//!
//! Requires `TNT_CORE_PATH` to point at a local `tnt-core` checkout so the harness
//! can replay the Foundry broadcast artifacts.

use alloy_primitives::Bytes;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::transaction::TransactionRequest;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::{SolCall, SolValue, sol};
use anyhow::{Context, Result, ensure};
use blueprint_anvil_testing_utils::{
    LOCAL_BLUEPRINT_ID, LOCAL_SERVICE_ID, SeededTangleEvmTestnet, harness_builder_from_env,
    missing_tnt_core_artifacts,
};
use blueprint_client_tangle_evm::contracts::ITangle::{addPermittedCallerCall, submitJobCall};
use blueprint_client_tangle_evm::{
    ServiceStatus, TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings,
};
use blueprint_core::extract::FromJobCall;
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_tangle_evm_extra::TangleEvmProducer;
use blueprint_tangle_evm_extra::extract::{CallId, ServiceId, TangleEvmArg};
use futures_util::{StreamExt, pin_mut};
use std::str::FromStr;
use tokio::time::{Duration, timeout};

const SERVICE_OWNER_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const JOB_INDEX: u8 = 0;
const DOC_ID: &str = "state-of-the-union";
const DOC_CONTENT: &str = r#"{"message":"payload"}"#;
const CALLER_ACCOUNT: &str = "tenant.sdk.test";

sol! {
    struct WriteDocInputs {
        string docId;
        string content;
        string account;
    }
}

const PRODUCER_STREAM_TIMEOUT: Duration = Duration::from_secs(600);
const TEST_TIMEOUT: Duration = Duration::from_secs(1_800);

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn tangle_producer_streams_real_job_submissions() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    run_anvil_test("tangle_producer_streams_real_job_submissions", async {
        let Some(deployment) = boot_testnet("tangle_producer_streams_real_job_submissions").await?
        else {
            return Ok(());
        };
        log_testnet_endpoints(&deployment);

        let client = create_client(&deployment).await?;
        log_service_status(&client).await?;

        let start_block = client.block_number().await.unwrap_or_default();
        let producer = TangleEvmProducer::from_block(client.clone(), LOCAL_SERVICE_ID, start_block)
            .with_poll_interval(Duration::from_millis(50));

        submit_job(&client).await?;

        pin_mut!(producer);
        let job_call = timeout(PRODUCER_STREAM_TIMEOUT, async {
            loop {
                match producer.next().await {
                    Some(Ok(job)) => break Ok(job),
                    Some(Err(err)) => {
                        tracing::warn!("producer error: {err:?}");
                    }
                    None => break Err(anyhow::anyhow!("producer ended unexpectedly")),
                }
            }
        })
        .await
        .context("timed out waiting for job event")??;

        let (mut parts, _) = job_call.clone().into_parts();
        let service_id = ServiceId::try_from(&mut parts).expect("service id missing");
        assert_eq!(service_id.0, LOCAL_SERVICE_ID);
        let call_id = CallId::try_from(&mut parts).expect("call id missing");
        assert_eq!(u32::from(job_call.job_id()), u32::from(JOB_INDEX));

        let TangleEvmArg((doc_id, content, account)) =
            TangleEvmArg::<(String, String, String)>::from_job_call(job_call, &())
                .await
                .expect("job inputs should decode");
        assert_eq!(doc_id, DOC_ID);
        assert_eq!(content, DOC_CONTENT);
        assert_eq!(account, CALLER_ACCOUNT);

        Ok(())
    })
    .await
}

async fn create_client(deployment: &SeededTangleEvmTestnet) -> Result<TangleEvmClient> {
    let keystore = Keystore::new(KeystoreConfig::new().in_memory(true))?;
    let secret_bytes = hex::decode(SERVICE_OWNER_PRIVATE_KEY)?;
    let secret = K256SigningKey::from_bytes(&secret_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to parse private key: {e}"))?;
    keystore.insert::<K256Ecdsa>(&secret)?;

    let settings = TangleEvmSettings {
        blueprint_id: LOCAL_BLUEPRINT_ID,
        service_id: Some(LOCAL_SERVICE_ID),
        tangle_contract: deployment.tangle_contract,
        restaking_contract: deployment.restaking_contract,
        status_registry_contract: deployment.status_registry_contract,
    };

    let config = TangleEvmClientConfig::new(
        deployment.http_endpoint().clone(),
        deployment.ws_endpoint().clone(),
        "memory://",
        settings,
    )
    .test_mode(true);

    TangleEvmClient::with_keystore(config, keystore)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to build client: {e}"))
}

async fn submit_job(client: &TangleEvmClient) -> Result<()> {
    let payload = WriteDocInputs {
        docId: DOC_ID.to_string(),
        content: DOC_CONTENT.to_string(),
        account: CALLER_ACCOUNT.to_string(),
    }
    .abi_encode();

    let signer = PrivateKeySigner::from_str(SERVICE_OWNER_PRIVATE_KEY)
        .context("invalid service owner private key")?;
    let caller_address = signer.address();
    let wallet = alloy_network::EthereumWallet::from(signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(client.config.http_rpc_endpoint.as_str())
        .await
        .context("failed to build writable provider")?;

    let permit = addPermittedCallerCall {
        serviceId: LOCAL_SERVICE_ID,
        caller: caller_address,
    };
    let permit_tx = TransactionRequest::default()
        .to(client.tangle_address())
        .input(permit.abi_encode().into());
    let permit_receipt = provider
        .send_transaction(permit_tx)
        .await
        .context("failed to add permitted caller")?
        .get_receipt()
        .await
        .context("add permitted caller reverted")?;
    ensure!(
        permit_receipt.status(),
        "add permitted caller transaction failed"
    );
    tracing::info!(
        "Permitted caller added at block {:?}",
        permit_receipt.block_number
    );

    let call = submitJobCall {
        serviceId: LOCAL_SERVICE_ID,
        jobIndex: JOB_INDEX,
        inputs: Bytes::from(payload),
    };

    let tx = TransactionRequest::default()
        .to(client.tangle_address())
        .input(call.abi_encode().into());

    let job_receipt = provider
        .send_transaction(tx)
        .await
        .context("failed to submit job transaction")?
        .get_receipt()
        .await
        .context("job submission transaction reverted")?;
    ensure!(job_receipt.status(), "job submission transaction failed");
    tracing::info!("Job submitted at block {:?}", job_receipt.block_number);

    Ok(())
}

async fn log_service_status(client: &TangleEvmClient) -> Result<()> {
    let service = client
        .get_service_info(LOCAL_SERVICE_ID)
        .await
        .context("failed to read seeded service")?;
    tracing::info!(
        "Service {} status: {:?}, operator_count: {}",
        LOCAL_SERVICE_ID,
        service.status,
        service.operator_count
    );
    assert_eq!(service.status, ServiceStatus::Active);
    Ok(())
}

fn log_testnet_endpoints(deployment: &SeededTangleEvmTestnet) {
    tracing::info!(
        http = %deployment.http_endpoint(),
        ws = %deployment.ws_endpoint(),
        "Anvil harness endpoints"
    );
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

async fn run_anvil_test<F>(name: &str, fut: F) -> Result<()>
where
    F: std::future::Future<Output = Result<()>>,
{
    timeout(TEST_TIMEOUT, fut)
        .await
        .with_context(|| format!("{name} timed out after {:?}", TEST_TIMEOUT))?
}
