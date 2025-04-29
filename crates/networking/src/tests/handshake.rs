use crate::service::AllowedKeys;
use crate::test_utils::TestNode;
use crate::test_utils::create_whitelisted_nodes;
use crate::test_utils::setup_log;
use blueprint_core::info;
use blueprint_crypto::sp_core::SpEcdsa;
use std::{collections::HashSet, time::Duration};
use tokio::time::timeout;

const TEST_TIMEOUT: Duration = Duration::from_secs(5);

#[tokio::test]
#[serial_test::serial]
async fn test_automatic_handshake() {
    setup_log();
    info!("Starting automatic handshake test");

    // Create nodes with whitelisted keys
    let mut nodes = create_whitelisted_nodes::<SpEcdsa>(2, "test-network", "test-instance", false);
    let mut node2 = nodes.pop().unwrap();
    let mut node1 = nodes.pop().unwrap();

    info!("Starting nodes");
    // Start both nodes - this should trigger automatic handshake
    let handle1 = node1.start().await.expect("Failed to start node1");
    let handle2 = node2.start().await.expect("Failed to start node2");

    // Wait for automatic handshake completion
    info!("Waiting for automatic handshake completion");
    timeout(TEST_TIMEOUT, async {
        loop {
            if handle1.peer_manager.is_peer_verified(&node2.peer_id)
                && handle2.peer_manager.is_peer_verified(&node1.peer_id)
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("Automatic handshake verification timed out");

    // Verify peer info and identify info are present
    let peer_info1 = handle1
        .peer_info(&node2.peer_id)
        .expect("Missing peer info for node2");
    let peer_info2 = handle2
        .peer_info(&node1.peer_id)
        .expect("Missing peer info for node1");

    assert!(
        peer_info1.identify_info.is_some(),
        "Missing identify info for node2"
    );
    assert!(
        peer_info2.identify_info.is_some(),
        "Missing identify info for node1"
    );
}

#[tokio::test]
#[serial_test::serial]
async fn test_handshake_with_invalid_peer() {
    setup_log();
    info!("Starting invalid peer handshake test");

    let network_name = "test-network";
    let instance_id = "test-instance";

    // Create node1 with empty whitelist
    let mut node1 = TestNode::<SpEcdsa>::new(
        network_name,
        instance_id,
        AllowedKeys::InstancePublicKeys(HashSet::new()),
        vec![],
        false,
    );

    // Create node2 with node1's key whitelisted (but node2's key is not whitelisted by node1)
    let mut allowed_keys2 = HashSet::new();
    allowed_keys2.insert(node1.instance_key_pair.public());
    let mut node2 = TestNode::<SpEcdsa>::new(
        network_name,
        instance_id,
        AllowedKeys::InstancePublicKeys(allowed_keys2),
        vec![],
        false,
    );

    info!("Starting nodes");
    let handle1 = node1.start().await.expect("Failed to start node1");
    let handle2 = node2.start().await.expect("Failed to start node2");

    // Wait for ban to be applied automatically
    info!("Waiting for automatic ban");
    timeout(TEST_TIMEOUT, async {
        loop {
            if handle1.peer_manager.is_banned(&node2.peer_id) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("Ban was not applied");

    // Verify peers remain unverified
    assert!(!handle1.peer_manager.is_peer_verified(&node2.peer_id));
    assert!(!handle2.peer_manager.is_peer_verified(&node1.peer_id));

    info!("Invalid peer handshake test completed successfully");
}
