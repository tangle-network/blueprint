use crate::{
    service::AllowedKeys,
    service_handle::NetworkServiceHandle,
    test_utils::{
        TestNode, create_whitelisted_nodes, setup_log, wait_for_all_handshakes,
        wait_for_handshake_completion,
    },
    types::{MessageRouting, ProtocolMessage},
};
use blueprint_core::info;
use blueprint_crypto::{KeyType, sp_core::SpEcdsa};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, time::Duration};
use tokio::time::timeout;

const TEST_TIMEOUT: Duration = Duration::from_secs(20);

// Protocol message types
#[derive(Debug, Clone, Serialize, Deserialize)]
enum SummationMessage {
    Number(u64),
    Verification { sum: u64 },
}

// Helper to create a protocol message
fn create_protocol_message<T: Serialize>(
    message: T,
    message_id: u64,
    round_id: u16,
    sender: PeerId,
    target_peer: Option<PeerId>,
) -> (MessageRouting, Vec<u8>) {
    let payload = bincode::serialize(&message).expect("Failed to serialize message");
    let routing = MessageRouting {
        message_id,
        round_id,
        sender,
        recipient: target_peer,
    };
    (routing, payload)
}

// Helper to extract number from message
fn extract_number_from_message(msg: &ProtocolMessage) -> u64 {
    match bincode::deserialize::<SummationMessage>(&msg.payload).expect("Failed to deserialize") {
        SummationMessage::Number(n) => n,
        SummationMessage::Verification { .. } => panic!("Expected number message"),
    }
}

// Helper to extract sum from verification message
fn extract_sum_from_verification(msg: &ProtocolMessage) -> u64 {
    match bincode::deserialize::<SummationMessage>(&msg.payload).expect("Failed to deserialize") {
        SummationMessage::Verification { sum } => sum,
        SummationMessage::Number(_) => panic!("Expected verification message"),
    }
}

#[tokio::test]
#[serial_test::serial]
async fn test_summation_protocol_basic() {
    setup_log();
    info!("Starting summation protocol test");

    // Create nodes with whitelisted keys
    let instance_key_pair2 = SpEcdsa::generate_with_seed(None).unwrap();
    let mut allowed_keys1 = HashSet::new();
    allowed_keys1.insert(instance_key_pair2.public());

    let mut node1 = TestNode::<SpEcdsa>::new(
        "test-net",
        "sum-test",
        AllowedKeys::InstancePublicKeys(allowed_keys1),
        vec![],
        false,
    );

    let mut allowed_keys2 = HashSet::new();
    allowed_keys2.insert(node1.instance_key_pair.public());
    let mut node2 = TestNode::<SpEcdsa>::new_with_keys(
        "test-net",
        "sum-test",
        AllowedKeys::InstancePublicKeys(allowed_keys2),
        vec![],
        Some(instance_key_pair2),
        None,
        false,
    );

    info!("Starting nodes");
    let mut handle1 = node1.start().await.expect("Failed to start node1");
    let mut handle2 = node2.start().await.expect("Failed to start node2");

    info!("Waiting for handshake completion");
    wait_for_handshake_completion(&handle1, &handle2, TEST_TIMEOUT).await;

    // ----------------------------------------------
    //     ROUND 1: GENERATE NUMBERS AND GOSSIP
    // ----------------------------------------------
    // Generate test numbers
    let num1 = 42;
    let num2 = 58;
    let expected_sum = num1 + num2;
    let message_id = 0;
    let round_id = 0;

    info!("Sending numbers via gossip");
    // Send numbers via gossip from node1 handle1
    let (routing, payload) = create_protocol_message(
        SummationMessage::Number(num1),
        message_id,
        round_id,
        handle1.local_peer_id,
        None,
    );
    handle1
        .send(routing, payload)
        .expect("Failed to send number from node1");

    // Send numbers via gossip from node2 handle2
    let (routing, payload) = create_protocol_message(
        SummationMessage::Number(num2),
        message_id,
        round_id,
        handle2.local_peer_id,
        None,
    );
    handle2
        .send(routing, payload)
        .expect("Failed to send number from node2");

    info!("Waiting for messages to be processed");

    // Wait for messages and compute sums
    let mut sum1 = num1;
    let mut sum2 = num2;
    let mut node1_received = false;
    let mut node2_received = false;

    timeout(TEST_TIMEOUT, async {
        loop {
            // Process incoming messages
            if let Some(msg) = handle1.next_protocol_message() {
                if !node1_received {
                    sum1 += extract_number_from_message(&msg);
                    node1_received = true;
                }
            }
            if let Some(msg) = handle2.next_protocol_message() {
                if !node2_received {
                    sum2 += extract_number_from_message(&msg);
                    node2_received = true;
                }
            }

            // Check if both nodes have received messages
            if node1_received && node2_received {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("Timeout waiting for summation completion");

    // -----------------------------------------------
    //      ROUND 2: VERIFY NUMBERS AND GOSSIP
    // -----------------------------------------------
    let message_id = 1;
    let round_id = 1;

    info!("Verifying sums via P2P messages");
    // Verify sums via P2P messages
    let (routing, payload) = create_protocol_message(
        SummationMessage::Verification { sum: sum1 },
        message_id,
        round_id,
        handle1.local_peer_id,
        Some(handle2.local_peer_id),
    );
    handle1
        .send(routing, payload)
        .expect("Failed to send verification from node1");

    let (routing, payload) = create_protocol_message(
        SummationMessage::Verification { sum: sum2 },
        message_id,
        round_id,
        handle2.local_peer_id,
        Some(handle1.local_peer_id),
    );
    handle2
        .send(routing, payload)
        .expect("Failed to send verification from node2");

    info!("Waiting for verification messages");
    // Wait for verification messages
    timeout(TEST_TIMEOUT, async {
        let mut node1_verified = false;
        let mut node2_verified = false;

        loop {
            // Process verification messages
            if let Some(msg) = handle1.next_protocol_message() {
                if !node1_verified {
                    assert_eq!(extract_sum_from_verification(&msg), expected_sum);
                    node1_verified = true;
                }
            }
            if let Some(msg) = handle2.next_protocol_message() {
                if !node2_verified {
                    assert_eq!(extract_sum_from_verification(&msg), expected_sum);
                    node2_verified = true;
                }
            }

            if node1_verified && node2_verified {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("Timeout waiting for verification completion");

    info!("Summation protocol test completed successfully");
}

#[tokio::test]
#[serial_test::serial]
async fn test_summation_protocol_multi_node() {
    setup_log();
    info!("Starting multi-node summation protocol test");

    // Create 3 nodes with whitelisted keys
    info!("Creating whitelisted nodes");
    let mut nodes =
        create_whitelisted_nodes::<SpEcdsa>(3, "summation-test", "test-instance", false);
    info!("Created {} nodes successfully", nodes.len());

    // Start all nodes
    info!("Starting all nodes");
    let mut handles = Vec::new();
    for (i, node) in nodes.iter_mut().enumerate() {
        info!("Starting node {}", i);
        handles.push(node.start().await.expect("Failed to start node"));
        info!("Node {} started successfully", i);
    }

    // Convert handles to mutable references
    info!("Converting handles to mutable references");
    let mut handles: Vec<&mut NetworkServiceHandle<SpEcdsa>> = handles.iter_mut().collect();
    let handles_len = handles.len();
    info!("Converted {} handles", handles_len);

    // Wait for all handshakes to complete
    info!(
        "Waiting for handshake completion between {} nodes",
        handles_len
    );
    wait_for_all_handshakes(&handles, TEST_TIMEOUT).await;
    info!("All handshakes completed successfully");

    // ----------------------------------------------
    //     ROUND 1: GENERATE NUMBERS AND GOSSIP
    // ----------------------------------------------

    // Generate test numbers
    let numbers = vec![42, 58, 100];
    let expected_sum: u64 = numbers.iter().sum();
    let message_id = 0;
    let round_id = 0;
    info!(
        "Generated test numbers: {:?}, expected sum: {}",
        numbers, expected_sum
    );

    info!("Sending numbers via gossip");
    // Each node broadcasts its number
    for (i, handle) in handles.iter().enumerate() {
        info!("Node {} broadcasting number {}", i, numbers[i]);
        let (routing, payload) = create_protocol_message(
            SummationMessage::Number(numbers[i]),
            message_id,
            round_id,
            handle.local_peer_id,
            None,
        );
        handle
            .send(routing, payload)
            .expect("Failed to send number");
        info!("Node {} successfully broadcast its number", i);
        // Add a small delay between broadcasts to avoid message collisions
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    info!("Waiting for messages to be processed");

    // Wait for all nodes to receive all numbers
    let mut sums = numbers.clone();
    let mut received = vec![0; handles_len];

    timeout(TEST_TIMEOUT, async {
        loop {
            for (i, handle) in handles.iter_mut().enumerate() {
                if let Some(msg) = handle.next_protocol_message() {
                    if received[i] < handles_len - 1 {
                        let num = extract_number_from_message(&msg);
                        sums[i] += num;
                        received[i] += 1;
                        info!(
                            "Node {} received number {}, total sum: {}, received count: {}",
                            i, num, sums[i], received[i]
                        );
                    }
                }
            }

            let all_received = received.iter().all(|&r| r == handles_len - 1);
            info!(
                "Current received counts: {:?}, target count: {}",
                received,
                handles_len - 1
            );
            if all_received {
                info!("All nodes have received all numbers");
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("Timeout waiting for summation completion");

    // -------------------------------------------------
    //      ROUND 2: VERIFY NUMBERS AND GOSSIP
    // -------------------------------------------------
    let message_id = 1;
    let round_id = 1;

    info!("Verifying sums via P2P messages");
    info!("Final sums: {:?}", sums);
    // Each node verifies with every other node
    for (i, sender) in handles.iter().enumerate() {
        for (j, _) in handles.iter().enumerate() {
            if i != j {
                info!(
                    "Node {} sending verification sum {} to node {}",
                    i, sums[i], j
                );
                let (routing, payload) = create_protocol_message(
                    SummationMessage::Verification { sum: sums[i] },
                    message_id,
                    round_id,
                    sender.local_peer_id,
                    Some(handles[j].local_peer_id),
                );
                sender
                    .send(routing, payload)
                    .expect("Failed to send verification");
            }
        }
    }

    info!("Waiting for verification messages");
    // Wait for all verifications
    timeout(TEST_TIMEOUT, async {
        let mut verified = vec![0; handles_len];
        loop {
            for (i, handle) in handles.iter_mut().enumerate() {
                if let Some(msg) = handle.next_protocol_message() {
                    if verified[i] < handles_len - 1 {
                        let sum = extract_sum_from_verification(&msg);
                        info!(
                            "Node {} received verification sum {}, expected {}",
                            i, sum, expected_sum
                        );
                        assert_eq!(sum, expected_sum);
                        verified[i] += 1;
                        info!("Node {} verification count: {}", i, verified[i]);
                    }
                }
            }

            let all_verified = verified.iter().all(|&v| v == handles_len - 1);
            info!("Current verification counts: {:?}", verified);
            if all_verified {
                info!("All nodes have verified all sums");
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("Timeout waiting for verification completion");

    info!("Multi-node summation protocol test completed successfully");
}
