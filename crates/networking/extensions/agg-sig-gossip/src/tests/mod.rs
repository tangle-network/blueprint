#![allow(clippy::too_many_lines)]

use crate::{
    protocol::{AggregationConfig, SignatureAggregationProtocol},
    signature_weight::EqualWeight,
};
use blueprint_core::info;
use gadget_crypto::aggregation::AggregatableSignature;
use gadget_networking::{
    service_handle::NetworkServiceHandle,
    test_utils::{create_whitelisted_nodes, setup_log, wait_for_all_handshakes},
    types::ParticipantId,
};
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

// Constants for tests
const TEST_TIMEOUT: Duration = Duration::from_secs(10);
const PROTOCOL_NAME: &str = "signature-aggregation/1.0.0";

// Generic function to run a signature aggregation test with any signature type
async fn run_signature_aggregation_test<S: AggregatableSignature + 'static>(
    num_nodes: usize,
    threshold_percentage: u8,
    generate_keys_fn: impl Fn(usize) -> Vec<S::Secret>,
    malicious_nodes: Vec<usize>,
) where
    S::Secret: Clone,
    S::Public: Clone,
    S::Signature: Clone,
{
    setup_log();
    info!(
        "Starting signature aggregation test with {} nodes",
        num_nodes
    );

    // Create whitelisted nodes
    let mut nodes = create_whitelisted_nodes::<S>(num_nodes, false).await;
    info!("Created {} nodes successfully", nodes.len());

    // Start all nodes
    info!("Starting all nodes");
    let mut handles = Vec::new();
    for (i, node) in nodes.iter_mut().enumerate() {
        info!("Starting node {}", i);
        handles.push(node.start().await.expect("Failed to start node"));
        info!("Node {} started successfully", i);
    }

    // Convert handles to mutable references for wait_for_all_handshakes
    let handle_refs: Vec<&mut NetworkServiceHandle<S>> = handles.iter_mut().collect();

    // Wait for all handshakes to complete
    info!(
        "Waiting for handshake completion between {} nodes",
        nodes.len()
    );
    wait_for_all_handshakes(&handle_refs, TEST_TIMEOUT).await;
    info!("All handshakes completed successfully");

    // Generate keys for the signature aggregation protocol
    let secrets = generate_keys_fn(num_nodes);
    let mut public_keys = HashMap::new();
    for i in 0..num_nodes {
        let public_key = S::public_from_secret(&secrets[i]);
        public_keys.insert(ParticipantId(i as u16), public_key);
    }

    // Test messages
    let regular_message = b"test message".to_vec();
    let malicious_message = b"different message".to_vec();

    // Run the protocol directly on each node
    let mut results = Vec::new();
    for i in 0..num_nodes {
        let message = if malicious_nodes.contains(&i) {
            malicious_message.clone()
        } else {
            regular_message.clone()
        };

        let config = AggregationConfig {
            local_id: ParticipantId(i as u16),
            max_participants: num_nodes as u16,
            num_aggregators: 1, // Use single aggregator for simplicity
            timeout: Duration::from_secs(5),
            protocol_id: PROTOCOL_NAME.to_string(),
        };

        let weight_scheme = EqualWeight::new(num_nodes, threshold_percentage);

        let mut protocol = SignatureAggregationProtocol::new(config, weight_scheme);

        let mut secret = secrets[i].clone();
        let handle = handles[i].clone();

        let public_keys_clone = public_keys.clone();
        let result = tokio::spawn(async move {
            protocol
                .run(message, &mut secret, &public_keys_clone, &handle)
                .await
        });

        results.push(result);
    }

    // Wait for results
    let final_results = futures::future::join_all(results).await;

    // Process results
    info!("Processing test results");
    let mut diagnostics = Vec::new();

    for (i, result) in final_results.iter().enumerate() {
        let node_type = if malicious_nodes.contains(&i) {
            "malicious"
        } else {
            "honest"
        };

        let config = AggregationConfig {
            local_id: ParticipantId(i as u16),
            max_participants: num_nodes as u16,
            num_aggregators: 1, // Match what was used in test
            timeout: Duration::from_secs(5),
            protocol_id: PROTOCOL_NAME.to_string(),
        };

        let aggregator_selector =
            crate::AggregatorSelector::new(config.max_participants, config.num_aggregators);

        // Calculate if this node was an aggregator
        let participants: HashSet<ParticipantId> = public_keys.keys().cloned().collect();
        let is_aggregator = {
            let mut selector = aggregator_selector.clone();
            selector.select_aggregators(&participants);
            selector.is_aggregator(ParticipantId(i as u16))
        };

        match result {
            Ok(Ok((result, _))) => {
                diagnostics.push(format!(
                    "Node {}: {} (aggregator={}) SUCCESS - contributors: {}, malicious: {}",
                    i,
                    node_type,
                    is_aggregator,
                    result.contributors.len(),
                    result.malicious_participants.len()
                ));

                if !malicious_nodes.contains(&i) {
                    info!(
                        "Node {} completed successfully with {} contributors and {} malicious nodes",
                        i,
                        result.contributors.len(),
                        result.malicious_participants.len()
                    );
                }
            }
            Ok(Err(e)) => {
                diagnostics.push(format!(
                    "Node {}: {} (aggregator={}) ERROR: {:?}",
                    i, node_type, is_aggregator, e
                ));

                if !malicious_nodes.contains(&i) {
                    panic!("Honest node {} failed: {:?}", i, e);
                }
            }
            Err(e) => {
                diagnostics.push(format!(
                    "Node {}: {} (aggregator={}) PANIC: {:?}",
                    i, node_type, is_aggregator, e
                ));
                panic!("Task for node {} panicked: {:?}", i, e);
            }
        }
    }

    // Print all diagnostics together for easier troubleshooting
    info!("\n=== Test Run Diagnostics ===");
    for diag in &diagnostics {
        info!("{}", diag);
    }
    info!("===========================\n");

    info!("Signature aggregation test completed successfully");
}

// BLS Tests
mod bls_tests {
    use super::*;
    use gadget_crypto::{
        KeyType,
        sp_core::{SpBls381, SpBls381Pair},
    };

    fn generate_bls_test_keys(num_keys: usize) -> Vec<SpBls381Pair> {
        let mut keys = Vec::with_capacity(num_keys);
        for i in 0..num_keys {
            let seed = [i as u8; 32];
            keys.push(SpBls381::generate_with_seed(Some(&seed)).unwrap());
        }
        keys
    }

    #[tokio::test]
    async fn test_bls381_basic_aggregation() {
        run_signature_aggregation_test::<SpBls381>(
            3,  // 3 nodes
            67, // 67% threshold (2 out of 3)
            generate_bls_test_keys,
            vec![], // No malicious nodes
        )
        .await;
    }

    #[tokio::test]
    async fn test_bls381_malicious_participant() {
        run_signature_aggregation_test::<SpBls381>(
            4,  // 4 nodes
            75, // 75% threshold (3 out of 4)
            generate_bls_test_keys,
            vec![3], // Node 3 is malicious
        )
        .await;
    }
}

// BN254 Tests
mod bn254_tests {
    use super::*;
    use gadget_crypto::KeyType;
    use gadget_crypto::bn254::{ArkBlsBn254, ArkBlsBn254Secret};

    fn generate_bn254_test_keys(num_keys: usize) -> Vec<ArkBlsBn254Secret> {
        let mut keys = Vec::with_capacity(num_keys);
        for i in 0..num_keys {
            let seed = [i as u8; 32];
            keys.push(ArkBlsBn254::generate_with_seed(Some(&seed)).unwrap());
        }
        keys
    }

    #[tokio::test]
    async fn test_bn254_basic_aggregation() {
        run_signature_aggregation_test::<ArkBlsBn254>(
            3,  // 3 nodes
            67, // 67% threshold (2 out of 3)
            generate_bn254_test_keys,
            vec![], // No malicious nodes
        )
        .await;
    }

    #[tokio::test]
    async fn test_bn254_malicious_participant() {
        run_signature_aggregation_test::<ArkBlsBn254>(
            4,  // 4 nodes
            75, // 75% threshold (3 out of 4)
            generate_bn254_test_keys,
            vec![3], // Node 3 is malicious
        )
        .await;
    }
}
