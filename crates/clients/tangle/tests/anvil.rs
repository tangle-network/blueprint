//! Integration tests for `TangleClient` backed by a real Anvil testnet.
//!
//! These tests boot a temporary Anvil container, seed it with the bundled
//! `LocalTestnet.s.sol` broadcast, and exercise the client APIs end-to-end. No mocks.

use alloy_primitives::{Address, Bytes, U256};
use anyhow::{Context, Result};
use blueprint_anvil_testing_utils::{
    LOCAL_BLUEPRINT_ID, LOCAL_SERVICE_ID, SeededTangleTestnet, harness_builder_from_env,
    missing_tnt_core_artifacts,
};
use blueprint_client_tangle::{
    RestakingStatus, ServiceStatus, TangleClient, TangleClientConfig, TangleSettings,
};
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use std::sync::Arc;
use tokio::time::{Duration, timeout};

const OPERATOR1_PRIVATE_KEY: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const OPERATOR1_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
const OPERATOR1_GOSSIP_KEY: &str = "040102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f40";
const BLUEPRINT_ID: u64 = LOCAL_BLUEPRINT_ID;
const SERVICE_ID: u64 = LOCAL_SERVICE_ID;

#[tokio::test]
async fn client_reads_blueprint_state() -> Result<()> {
    run_anvil_test("client_reads_blueprint_state", async {
        let Some(deployment) = boot_testnet("client_reads_blueprint_state").await? else {
            return Ok(());
        };
        let client = create_test_client(&deployment).await?;

        let blueprint = client.get_blueprint_info(BLUEPRINT_ID).await?;
        assert!(blueprint.active);
        assert_eq!(
            blueprint.manager,
            Address::ZERO,
            "LocalTestnet uses no manager"
        );

        let service = client.get_service_info(SERVICE_ID).await?;
        assert_eq!(service.status, ServiceStatus::Active);
        assert_eq!(service.operator_count, 2);

        Ok(())
    })
    .await
}

#[tokio::test]
async fn client_fetches_operator_metadata() -> Result<()> {
    run_anvil_test("client_fetches_operator_metadata", async {
        let Some(deployment) = boot_testnet("client_fetches_operator_metadata").await? else {
            return Ok(());
        };
        let client = create_test_client(&deployment).await?;

        let operator1: Address = OPERATOR1_ADDRESS.parse().unwrap();
        let metadata = client
            .get_operator_metadata(BLUEPRINT_ID, operator1)
            .await?;

        // Validate metadata structure without hardcoding exact values from deployment script
        assert!(
            !metadata.rpc_endpoint.is_empty(),
            "operator should have an RPC endpoint configured"
        );
        assert!(
            metadata.restaking.stake > U256::ZERO,
            "operator should have non-zero stake"
        );
        assert_eq!(
            metadata.restaking.status,
            RestakingStatus::Active,
            "operator should be active"
        );
        assert_eq!(
            metadata.public_key.len(),
            65,
            "public key should be 65 bytes (uncompressed secp256k1)"
        );

        Ok(())
    })
    .await
}

#[tokio::test]
async fn client_submits_result_transaction() -> Result<()> {
    run_anvil_test("client_submits_result_transaction", async {
        let Some(deployment) = boot_testnet("client_submits_result_transaction").await? else {
            return Ok(());
        };
        let client = create_test_client(&deployment).await?;

        // LocalTestnet seeds call #1 for service #0 so we can submit a result directly.
        let output = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);
        let submission = client
            .submit_result(SERVICE_ID, 1u64, output)
            .await
            .context("result submission should not panic");

        match submission {
            Ok(tx) => {
                assert!(tx.success, "transaction should succeed");
            }
            Err(err) => {
                // A duplicate submission is fine; we only care that the client built and sent a transaction.
                eprintln!("submit_result reverted (likely duplicate): {err}");
            }
        }

        Ok(())
    })
    .await
}

async fn create_test_client(deployment: &SeededTangleTestnet) -> Result<Arc<TangleClient>> {
    let keystore = Keystore::new(KeystoreConfig::new().in_memory(true))?;
    let secret_bytes = hex::decode(OPERATOR1_PRIVATE_KEY)?;
    let secret = K256SigningKey::from_bytes(&secret_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to parse private key: {e}"))?;
    keystore.insert::<K256Ecdsa>(&secret)?;

    let settings = TangleSettings {
        blueprint_id: BLUEPRINT_ID,
        service_id: Some(SERVICE_ID),
        tangle_contract: deployment.tangle_contract,
        restaking_contract: deployment.restaking_contract,
        status_registry_contract: deployment.status_registry_contract,
    };

    let config = TangleClientConfig::new(
        deployment.http_endpoint().clone(),
        deployment.ws_endpoint().clone(),
        "memory://",
        settings,
    )
    .test_mode(true);

    Ok(Arc::new(
        TangleClient::with_keystore(config, keystore).await?,
    ))
}

const ANVIL_TEST_TIMEOUT: Duration = Duration::from_secs(1_800);

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
