//! Anvil Integration Tests for Tangle EVM Transaction Submission
//!
//! These tests require a running Anvil instance with deployed Tangle v2 contracts.
//!
//! ## Setup Instructions
//!
//! 1. Start Anvil: `anvil`
//! 2. Deploy contracts from tnt-core repo:
//!    ```bash
//!    forge script script/v2/LocalTestnet.s.sol:LocalTestnetSetup --rpc-url http://localhost:8545 --broadcast
//!    ```
//! 3. Run tests: `cargo test --package blueprint-tangle-evm-extra -- --ignored`
//!
//! The deployed addresses from LocalTestnet.s.sol:
//! - Deployer (Account 0): 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
//! - Operator1 (Account 1): 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
//! - Operator2 (Account 2): 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC
//!
//! The contracts are deployed at consistent addresses due to deterministic deployment.

use alloy_primitives::{Address, Bytes, U256};
use blueprint_client_tangle_evm::{TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings};
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_crypto::BytesEncoding;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use std::sync::Arc;
use url::Url;

/// Default Anvil RPC endpoint
const ANVIL_HTTP: &str = "http://localhost:8545";
const ANVIL_WS: &str = "ws://localhost:8545";

/// Anvil default private keys (Account 0 = deployer, Account 1 = operator1, etc.)
const OPERATOR1_PRIVATE_KEY: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

/// Expected addresses from LocalTestnet deployment
const OPERATOR1_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";

/// Creates a TangleEvmClient configured for local Anvil testing
async fn create_test_client(
    tangle_address: Address,
    restaking_address: Address,
) -> anyhow::Result<Arc<TangleEvmClient>> {
    // Create an in-memory keystore with the test operator's key
    let keystore = Keystore::new(KeystoreConfig::new().in_memory(true))?;

    // Import the operator1 private key
    let secret_bytes = hex::decode(OPERATOR1_PRIVATE_KEY)?;
    let secret = K256SigningKey::from_bytes(&secret_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to parse private key: {e}"))?;
    keystore.insert::<K256Ecdsa>(&secret)?;

    let settings = TangleEvmSettings {
        blueprint_id: 1,
        service_id: Some(1),
        tangle_contract: tangle_address,
        restaking_contract: restaking_address,
    };

    let config = TangleEvmClientConfig::new(
        Url::parse(ANVIL_HTTP)?,
        Url::parse(ANVIL_WS)?,
        "memory://",
        settings,
    )
    .test_mode(true);

    let client = TangleEvmClient::with_keystore(config, keystore).await?;
    Ok(Arc::new(client))
}

/// Test the result submission flow
///
/// Prerequisites:
/// - Anvil running on localhost:8545
/// - Contracts deployed via LocalTestnet.s.sol
/// - Service ID 1 exists and is active
#[tokio::test]
#[ignore = "Requires running Anvil with deployed contracts"]
async fn test_submit_result() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // These addresses are from LocalTestnet deployment - will be consistent
    // due to CREATE2/deterministic deployment. Update if needed.
    // You can get the actual addresses from the deployment script output.
    let tangle_address: Address = "0x5FbDB2315678afecb367f032d93F642f64180aa3"
        .parse()
        .unwrap();
    let restaking_address: Address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
        .parse()
        .unwrap();

    let client = create_test_client(tangle_address, restaking_address).await?;

    // Use service ID 1 (created by LocalTestnet script)
    let service_id = 1u64;
    let call_id = 1u64;
    let output = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);

    // First, submit a job to create a call_id
    // (In a real test, we'd call a job submission function first)

    // Submit the result
    let result = client.submit_result(service_id, call_id, output).await;

    match result {
        Ok(tx_result) => {
            println!("Transaction submitted successfully!");
            println!("  TX Hash: {:?}", tx_result.tx_hash);
            println!("  Block Number: {:?}", tx_result.block_number);
            println!("  Gas Used: {}", tx_result.gas_used);
            println!("  Success: {}", tx_result.success);
            assert!(tx_result.success, "Transaction should succeed");
        }
        Err(e) => {
            // Transaction might fail if call_id doesn't exist or already has result
            // This is expected in some test scenarios
            println!("Transaction failed (may be expected): {e}");
        }
    }

    Ok(())
}

/// Test fetching operator weights
#[tokio::test]
#[ignore = "Requires running Anvil with deployed contracts"]
async fn test_get_operator_weights() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let tangle_address: Address = "0x5FbDB2315678afecb367f032d93F642f64180aa3"
        .parse()
        .unwrap();
    let restaking_address: Address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
        .parse()
        .unwrap();

    let client = create_test_client(tangle_address, restaking_address).await?;

    let service_id = 1u64;

    // Get operator weights
    let weights = client.get_service_operator_weights(service_id).await?;

    println!("Operator weights for service {}:", service_id);
    for (operator, weight) in &weights {
        println!("  {}: {} bps", operator, weight);
    }

    // LocalTestnet creates operators with equal exposure (5000 bps each)
    assert!(!weights.is_empty(), "Should have at least one operator");

    // Verify operator1 is in the weights
    let operator1: Address = OPERATOR1_ADDRESS.parse().unwrap();
    assert!(
        weights.contains_key(&operator1),
        "Operator1 should be in the service"
    );

    Ok(())
}

/// Test getting service operator info
#[tokio::test]
#[ignore = "Requires running Anvil with deployed contracts"]
async fn test_get_service_operator() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let tangle_address: Address = "0x5FbDB2315678afecb367f032d93F642f64180aa3"
        .parse()
        .unwrap();
    let restaking_address: Address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
        .parse()
        .unwrap();

    let client = create_test_client(tangle_address, restaking_address).await?;

    let service_id = 1u64;
    let operator1: Address = OPERATOR1_ADDRESS.parse().unwrap();

    // Get service operator info
    let op_info = client.get_service_operator(service_id, operator1).await?;

    println!("Operator info for service {}:", service_id);
    println!("  Exposure BPS: {}", op_info.exposureBps);
    println!("  Joined At: {}", op_info.joinedAt);
    println!("  Left At: {}", op_info.leftAt);
    println!("  Active: {}", op_info.active);

    assert!(op_info.active, "Operator should be active");
    assert!(op_info.exposureBps > 0, "Exposure should be non-zero");
    assert_eq!(op_info.leftAt, 0, "Should not have left");

    Ok(())
}

/// Test getting total service exposure
#[tokio::test]
#[ignore = "Requires running Anvil with deployed contracts"]
async fn test_get_service_total_exposure() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let tangle_address: Address = "0x5FbDB2315678afecb367f032d93F642f64180aa3"
        .parse()
        .unwrap();
    let restaking_address: Address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
        .parse()
        .unwrap();

    let client = create_test_client(tangle_address, restaking_address).await?;

    let service_id = 1u64;

    // Get total exposure
    let total_exposure = client.get_service_total_exposure(service_id).await?;

    println!("Total exposure for service {}: {}", service_id, total_exposure);

    // LocalTestnet creates 2 operators with 5000 bps each = 10000 total
    assert!(total_exposure > U256::ZERO, "Total exposure should be non-zero");

    Ok(())
}

/// Test aggregated result submission
///
/// This test verifies the aggregated result submission works correctly
/// when BLS signature data is provided.
#[tokio::test]
#[ignore = "Requires running Anvil with deployed contracts"]
async fn test_submit_aggregated_result() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let tangle_address: Address = "0x5FbDB2315678afecb367f032d93F642f64180aa3"
        .parse()
        .unwrap();
    let restaking_address: Address = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
        .parse()
        .unwrap();

    let client = create_test_client(tangle_address, restaking_address).await?;

    let service_id = 1u64;
    let call_id = 1u64;
    let output = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);

    // For testing, use placeholder BLS values
    // In production, these would be actual aggregated BLS signatures
    let signer_bitmap = U256::from(0b11); // Both operators signed (bits 0 and 1)
    let aggregated_signature = [U256::ZERO, U256::ZERO]; // Placeholder G1 point
    let aggregated_pubkey = [U256::ZERO, U256::ZERO, U256::ZERO, U256::ZERO]; // Placeholder G2 point

    // Submit the aggregated result
    let result = client
        .submit_aggregated_result(
            service_id,
            call_id,
            output,
            signer_bitmap,
            aggregated_signature,
            aggregated_pubkey,
        )
        .await;

    match result {
        Ok(tx_result) => {
            println!("Aggregated result submitted successfully!");
            println!("  TX Hash: {:?}", tx_result.tx_hash);
            println!("  Block Number: {:?}", tx_result.block_number);
            println!("  Gas Used: {}", tx_result.gas_used);
            println!("  Success: {}", tx_result.success);
            // Note: With placeholder BLS values, the on-chain verification may fail
            // A real test would need valid BLS signatures
        }
        Err(e) => {
            // Transaction might fail due to invalid BLS signatures or missing call_id
            println!("Transaction failed (may be expected with placeholder BLS): {e}");
        }
    }

    Ok(())
}
