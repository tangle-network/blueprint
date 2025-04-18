#![allow(clippy::too_many_lines)]

use crate::{
    discovery::peers::{VerificationIdentifierKey, WhitelistedKeys},
    test_utils::{
        TestNode, create_whitelisted_nodes, init_tracing, wait_for_all_handshakes,
        wait_for_handshake_completion,
    },
    types::{MessageRouting, ParticipantInfo, ProtocolMessage},
};
use blueprint_crypto::{KeyType, sp_core::SpEcdsa};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, time::Duration};
use tokio::time::timeout;
use tracing::info;

const TEST_TIMEOUT: Duration = Duration::from_secs(10);
const PROTOCOL_NAME: &str = "summation/1.0.0";

// Protocol message types
#[derive(Debug, Clone, Serialize, Deserialize)]
enum SummationMessage {
    Number(u64),
    Verification { sum: u128 },
}

// Helper to create a protocol message
fn create_protocol_message<T: Serialize, K: KeyType>(
    message: T,
    message_id: u64,
    round_id: u16,
    sender_peer_id: PeerId,
    target_peer: Option<PeerId>,
) -> (MessageRouting<K>, Vec<u8>) {
    let payload = bincode::serialize(&message).expect("Failed to serialize message");
    let routing = MessageRouting {
        message_id,
        round_id,
        sender: ParticipantInfo::<K>::new_with_peer_id(sender_peer_id),
        recipient: target_peer.map(|peer_id| ParticipantInfo::<K>::new_with_peer_id(peer_id)),
    };
    (routing, payload)
}

// Helper to extract number from message
fn extract_number_from_message<K: KeyType>(msg: &ProtocolMessage<K>) -> u64 {
    match bincode::deserialize::<SummationMessage>(&msg.payload).expect("Failed to deserialize") {
        SummationMessage::Number(n) => n,
        SummationMessage::Verification { .. } => panic!("Expected number message"),
    }
}

// Helper to extract sum from verification message
fn extract_sum_from_verification<K: KeyType>(msg: &ProtocolMessage<K>) -> u128 {
    match bincode::deserialize::<SummationMessage>(&msg.payload).expect("Failed to deserialize") {
        SummationMessage::Verification { sum } => sum,
        SummationMessage::Number(_) => panic!("Expected verification message"),
    }
}

#[tokio::test]
#[serial_test::serial]
async fn test_summation_protocol_basic() {
    init_tracing();
    info!("Starting summation protocol test");

    // Create nodes with whitelisted keys
    let instance_key_pair2 = SpEcdsa::generate_with_seed(None).unwrap();
    let mut allowed_keys1 = HashSet::new();
    allowed_keys1.insert(VerificationIdentifierKey::InstancePublicKey(
        instance_key_pair2.public(),
    ));

    let mut node1 = TestNode::<SpEcdsa>::new(
        "summation-basic",
        "v1.0.0",
        WhitelistedKeys::new_from_hashset(allowed_keys1),
        &[],
        false,
    );

    let mut allowed_keys2 = HashSet::new();
    allowed_keys2.insert(VerificationIdentifierKey::InstancePublicKey(
        node1.instance_key_pair.public(),
    ));
    let mut node2 = TestNode::<SpEcdsa>::new_with_keys(
        "summation-basic",
        "v1.0.0",
        WhitelistedKeys::new_from_hashset(allowed_keys2),
        &[],
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
    let (routing, payload) = create_protocol_message::<_, SpEcdsa>(
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
    let (routing, payload) = create_protocol_message::<_, SpEcdsa>(
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

    // Wait for both nodes to receive the other's number
    timeout(TEST_TIMEOUT, async {
        let mut node1_received = false;
        let mut node2_received = false;

        while !node1_received || !node2_received {
            if let Some(msg) = handle1.next_protocol_message() {
                let num = extract_number_from_message::<SpEcdsa>(&msg);
                assert_eq!(num, num2);
                node1_received = true;
                info!("Node 1 received number from node 2");
            }

            if let Some(msg) = handle2.next_protocol_message() {
                let num = extract_number_from_message::<SpEcdsa>(&msg);
                assert_eq!(num, num1);
                node2_received = true;
                info!("Node 2 received number from node 1");
            }

            if !node1_received || !node2_received {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    })
    .await
    .expect("Timeout waiting for messages");

    // -------------------------------------------------
    //      ROUND 2: VERIFY NUMBERS AND GOSSIP
    // -------------------------------------------------
    let message_id = 1;
    let round_id = 1;

    info!("Verifying sums via P2P messages");
    // Node 1 sends verification to node 2
    let (routing, payload) = create_protocol_message::<_, SpEcdsa>(
        SummationMessage::Verification {
            sum: u128::from(expected_sum),
        },
        message_id,
        round_id,
        handle1.local_peer_id,
        Some(handle2.local_peer_id),
    );
    handle1
        .send(routing, payload)
        .expect("Failed to send verification from node1");

    // Node 2 sends verification to node 1
    let (routing, payload) = create_protocol_message::<_, SpEcdsa>(
        SummationMessage::Verification {
            sum: u128::from(expected_sum),
        },
        message_id,
        round_id,
        handle2.local_peer_id,
        Some(handle1.local_peer_id),
    );
    handle2
        .send(routing, payload)
        .expect("Failed to send verification from node2");

    info!("Waiting for verification messages");
    // Wait for both nodes to receive verification
    timeout(TEST_TIMEOUT, async {
        let mut node1_verified = false;
        let mut node2_verified = false;

        while !node1_verified || !node2_verified {
            if let Some(msg) = handle1.next_protocol_message() {
                let sum = extract_sum_from_verification::<SpEcdsa>(&msg);
                assert_eq!(sum, u128::from(expected_sum));
                node1_verified = true;
                info!("Node 1 received verification from node 2");
            }

            if let Some(msg) = handle2.next_protocol_message() {
                let sum = extract_sum_from_verification::<SpEcdsa>(&msg);
                assert_eq!(sum, u128::from(expected_sum));
                node2_verified = true;
                info!("Node 2 received verification from node 1");
            }

            if !node1_verified || !node2_verified {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    })
    .await
    .expect("Timeout waiting for verification");

    info!("Summation protocol test completed successfully");
}

#[tokio::test]
#[serial_test::serial]
async fn test_summation_protocol_multi_node() {
    init_tracing();
    let network_name = "summation-multi";
    let instance_id = "v1.0.0";
    let num_nodes = 3;

    info!("Creating {} whitelisted nodes", num_nodes);
    let mut nodes =
        create_whitelisted_nodes::<SpEcdsa>(num_nodes, network_name, instance_id, false).await;

    // Start all nodes and get handles
    info!("Starting all nodes");
    let mut handles = Vec::with_capacity(nodes.len());
    for node in &mut nodes {
        handles.push(node.handle.clone());
    }

    info!("Waiting for all handshakes");
    // Create a vector of references to TestNode for wait_for_all_handshakes
    let node_refs: Vec<&TestNode<SpEcdsa>> = nodes.iter().collect();
    wait_for_all_handshakes(&node_refs, 30).await;

    let handles_len = handles.len();
    info!("All {} nodes connected and verified", handles_len);

    // ----------------------------------------------
    //     ROUND 1: GENERATE NUMBERS AND GOSSIP
    // ----------------------------------------------
    // Generate random numbers for each node
    let mut numbers = Vec::with_capacity(handles_len);
    let mut expected_sum = 0;
    for i in 0..handles_len {
        let num = (i + 1) as u64 * 10;
        numbers.push(num);
        expected_sum += num;
    }
    info!(
        "Generated numbers: {:?}, expected sum: {}",
        numbers, expected_sum
    );

    // Each node broadcasts its number
    let message_id = 0;
    let round_id = 0;

    for (i, handle) in handles.iter().enumerate() {
        info!("Node {} broadcasting number {}", i, numbers[i]);
        let (routing, payload) = create_protocol_message::<_, SpEcdsa>(
            SummationMessage::Number(numbers[i]),
            message_id,
            round_id,
            handle.local_peer_id,
            None,
        );
        handle
            .send(routing, payload)
            .expect("Failed to broadcast number");
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
                        let num = extract_number_from_message::<SpEcdsa>(&msg);
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
                let (routing, payload) = create_protocol_message::<_, SpEcdsa>(
                    SummationMessage::Verification {
                        sum: u128::from(sums[i]),
                    },
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
                        let sum = extract_sum_from_verification::<SpEcdsa>(&msg);
                        info!(
                            "Node {} received verification sum {}, expected {}",
                            i, sum, expected_sum
                        );
                        assert_eq!(sum, u128::from(expected_sum));
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
