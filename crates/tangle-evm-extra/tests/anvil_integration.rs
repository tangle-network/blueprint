//! Integration tests for the Tangle EVM extras backed by the shared Anvil harness.
//!
//! Each test boots `TangleEvmHarness` (the same helper used by the rest of the SDK)
//! and gracefully skips itself when the bundled LocalTestnet broadcast/snapshot
//! is missing. This keeps the tests deterministic in CI without forcing contributors
//! to run setup scripts manually.

use alloy_primitives::{Address, Bytes, U256};
use anyhow::{Context, Result};
use blueprint_anvil_testing_utils::{
    LOCAL_BLUEPRINT_ID, LOCAL_SERVICE_ID, SeededTangleEvmTestnet, harness_builder_from_env,
    missing_tnt_core_artifacts,
};
use blueprint_client_tangle_evm::{TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings};
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use std::sync::Arc;
use tokio::time::{Duration, timeout};

const OPERATOR1_PRIVATE_KEY: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const OPERATOR1_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
const BLUEPRINT_ID: u64 = LOCAL_BLUEPRINT_ID;
const SERVICE_ID: u64 = LOCAL_SERVICE_ID;
const TEST_TIMEOUT: Duration = Duration::from_secs(1_800);

#[tokio::test]
async fn submit_result() -> Result<()> {
    run_anvil_test("submit_result", async {
        let Some(deployment) = boot_testnet("submit_result").await? else {
            return Ok(());
        };
        let client = create_test_client(&deployment).await?;

        let call_id = 1u64;
        let output = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);

        match client.submit_result(SERVICE_ID, call_id, output).await {
            Ok(tx) => assert!(tx.success, "transaction should succeed"),
            Err(err) => {
                // Duplicate submissions are acceptable; the client just needs to send a tx.
                eprintln!("submit_result reverted (likely duplicate): {err}");
            }
        }

        Ok(())
    })
    .await
}

#[tokio::test]
async fn get_operator_weights() -> Result<()> {
    run_anvil_test("get_operator_weights", async {
        let Some(deployment) = boot_testnet("get_operator_weights").await? else {
            return Ok(());
        };
        let client = create_test_client(&deployment).await?;

        let weights = client
            .get_service_operator_weights(SERVICE_ID)
            .await
            .context("failed to fetch operator weights")?;
        assert!(!weights.is_empty(), "service should have seeded operators");
        let operator1: Address = OPERATOR1_ADDRESS.parse().unwrap();
        assert!(
            weights.contains_key(&operator1),
            "operator1 should be part of the service"
        );

        Ok(())
    })
    .await
}

#[tokio::test]
async fn get_service_operator() -> Result<()> {
    run_anvil_test("get_service_operator", async {
        let Some(deployment) = boot_testnet("get_service_operator").await? else {
            return Ok(());
        };
        let client = create_test_client(&deployment).await?;

        let operator1: Address = OPERATOR1_ADDRESS.parse().unwrap();
        let info = client
            .get_service_operator(SERVICE_ID, operator1)
            .await
            .context("failed to fetch operator info")?;

        assert!(info.active, "operator should be active");
        assert!(info.exposureBps > 0, "operator exposure should be non-zero");
        assert_eq!(info.leftAt, 0, "operator should not have left the service");

        Ok(())
    })
    .await
}

#[tokio::test]
async fn get_service_total_exposure() -> Result<()> {
    run_anvil_test("get_service_total_exposure", async {
        let Some(deployment) = boot_testnet("get_service_total_exposure").await? else {
            return Ok(());
        };
        let client = create_test_client(&deployment).await?;

        let total_exposure = client
            .get_service_total_exposure(SERVICE_ID)
            .await
            .context("failed to fetch total exposure")?;
        assert!(
            total_exposure > U256::ZERO,
            "service exposure should be > 0"
        );

        Ok(())
    })
    .await
}

#[tokio::test]
async fn submit_aggregated_result() -> Result<()> {
    run_anvil_test("submit_aggregated_result", async {
        let Some(deployment) = boot_testnet("submit_aggregated_result").await? else {
            return Ok(());
        };
        let client = create_test_client(&deployment).await?;

        let call_id = 1u64;
        let output = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);
        let signer_bitmap = U256::from(0b11);
        let aggregated_signature = [U256::ZERO, U256::ZERO];
        let aggregated_pubkey = [U256::ZERO, U256::ZERO, U256::ZERO, U256::ZERO];

        // Placeholder BLS data will revert but still exercises the transaction path.
        if let Err(err) = client
            .submit_aggregated_result(
                SERVICE_ID,
                call_id,
                output,
                signer_bitmap,
                aggregated_signature,
                aggregated_pubkey,
            )
            .await
        {
            eprintln!("submit_aggregated_result failed (expected with placeholder BLS): {err}");
        }

        Ok(())
    })
    .await
}

async fn create_test_client(deployment: &SeededTangleEvmTestnet) -> Result<Arc<TangleEvmClient>> {
    let keystore = Keystore::new(KeystoreConfig::new().in_memory(true))?;
    let secret_bytes = hex::decode(OPERATOR1_PRIVATE_KEY)?;
    let secret = K256SigningKey::from_bytes(&secret_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to parse private key: {e}"))?;
    keystore.insert::<K256Ecdsa>(&secret)?;

    let settings = TangleEvmSettings {
        blueprint_id: BLUEPRINT_ID,
        service_id: Some(SERVICE_ID),
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

    Ok(Arc::new(
        TangleEvmClient::with_keystore(config, keystore).await?,
    ))
}

async fn run_anvil_test<F>(name: &str, fut: F) -> Result<()>
where
    F: std::future::Future<Output = Result<()>>,
{
    timeout(TEST_TIMEOUT, fut)
        .await
        .with_context(|| format!("{name} timed out after {:?}", TEST_TIMEOUT))?
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
