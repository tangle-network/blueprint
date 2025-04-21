#![allow(clippy::too_many_lines)]

use crate::{
    protocol::{ProtocolConfig, SignatureAggregationProtocol},
    signature_weight::{EqualWeight, SignatureWeight},
};
use blueprint_core::info;
use blueprint_crypto::{KeyType, aggregation::AggregatableSignature, hashing::blake3_256};
use blueprint_networking::{
    service_handle::NetworkServiceHandle,
    test_utils::{create_whitelisted_nodes, wait_for_all_handshakes},
};
use blueprint_std::{collections::HashMap, time::Duration};
use tracing_subscriber::EnvFilter;

// Constants for tests
const TEST_TIMEOUT: Duration = Duration::from_secs(10);

pub fn setup_log() {
    let filter = EnvFilter::new(
        "blueprint_networking=info,blueprint_networking_agg_sig_gossip_extension=debug",
    );

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .try_init();
}

// Generic function to run a signature aggregation test with any signature type
async fn run_signature_aggregation_test<S: AggregatableSignature + 'static>(
    num_nodes: usize,
    threshold_percentage: u8,
    network_name: &str,
    instance_name: &str,
    generate_keys_fn: impl Fn(usize) -> Vec<S::Secret>,
) where
    S::Secret: Clone,
    S::Public: Clone,
    S::Signature: Clone,
{
    setup_log();
    info!(
        "Starting signature aggregation test with {} nodes, threshold {}%",
        num_nodes, threshold_percentage
    );

    // Create whitelisted nodes
    let mut nodes = create_whitelisted_nodes::<S>(num_nodes, network_name, instance_name, false);
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
    info!("==================== STARTING PROTOCOL PHASE ====================");

    // Generate keys for the signature aggregation protocol
    let secrets = generate_keys_fn(num_nodes);
    let mut public_keys = HashMap::new();
    for (i, secret) in secrets.iter().enumerate() {
        let public_key = S::public_from_secret(&handles[i].local_signing_key);
        public_keys.insert(handles[i].local_peer_id, public_key);
        info!("Generated key pair for node {}", i);
    }

    // Test message
    let message = b"test message";
    let message_hash = blake3_256(message);

    // Increase timeout for testing
    let protocol_timeout = Duration::from_secs(15);
    info!("Protocol timeout set to {:?}", protocol_timeout);

    // Use multiple aggregators for better reliability
    let num_aggregators = 2;
    info!("Using {} aggregators", num_aggregators);

    // Run the protocol directly on each node
    let mut results = Vec::new();
    info!("Starting protocol on {} nodes", num_nodes);
    for i in 0..num_nodes {
        let config = ProtocolConfig {
            network_handle: handles[i].clone(),
            num_aggregators,
            timeout: protocol_timeout,
        };

        let weight_scheme = EqualWeight::new(num_nodes, threshold_percentage);
        info!(
            "Node {} threshold weight: {}",
            i,
            weight_scheme.threshold_weight()
        );

        let mut protocol =
            SignatureAggregationProtocol::new(config, weight_scheme, public_keys.clone());

        // Check if this node is an aggregator
        let is_aggregator = protocol.is_aggregator();
        info!("Node {} is_aggregator: {}", i, is_aggregator);

        info!("Node {} about to start protocol execution", i);

        let result = tokio::spawn(async move {
            info!("Node {} starting protocol execution", i);
            info!("Node {} preparing to sign and broadcast message hash", i);
            let result = protocol.run(&message_hash).await;

            if result.is_ok() {
                info!("Node {} protocol completed successfully", i);
                if let Ok(agg_result) = &result {
                    info!(
                        "Node {} aggregation successful - contributors: {}, total_weight: {}",
                        i,
                        agg_result.contributors.len(),
                        agg_result.total_weight.unwrap_or(0)
                    );
                }
            } else {
                info!("Node {} protocol failed: {:?}", i, result);
            }

            result
        });

        results.push(result);
    }

    // Wait for results
    info!("Waiting for all nodes to complete the protocol");
    let final_results = futures::future::join_all(results).await;
    info!("All nodes completed their protocol runs");

    // Process results
    info!("Processing test results");
    let mut diagnostics = Vec::new();

    for (i, result) in final_results.iter().enumerate() {
        let config = ProtocolConfig {
            network_handle: handles[i].clone(),
            num_aggregators,
            timeout: protocol_timeout,
        };

        let aggregator_selector =
            crate::aggregator_selection::AggregatorSelector::new(config.num_aggregators);

        let is_aggregator = aggregator_selector.is_aggregator::<S>(
            handles[i].local_peer_id,
            &public_keys,
            message.as_ref(),
        );

        match result {
            Ok(Ok(result)) => {
                diagnostics.push(format!(
                    "Node {}: honest (aggregator={}) SUCCESS - contributors: {}",
                    i,
                    is_aggregator,
                    result.contributors.len()
                ));

                info!(
                    "Node {} completed successfully with {} contributors",
                    i,
                    result.contributors.len()
                );
            }
            Ok(Err(e)) => {
                diagnostics.push(format!(
                    "Node {}: honest (aggregator={}) ERROR: {:?}",
                    i, is_aggregator, e
                ));
                panic!("Node {} failed: {:?}", i, e);
            }
            Err(e) => {
                diagnostics.push(format!(
                    "Node {}: honest (aggregator={}) PANIC: {:?}",
                    i, is_aggregator, e
                ));
                panic!("Task for node {} panicked: {:?}", i, e);
            }
        }
    }

    info!("\n=== Test Run Diagnostics ===");
    for diag in &diagnostics {
        info!("{}", diag);
    }
    info!("===========================\n");

    info!("Signature aggregation test completed successfully");
}

fn generate_test_keys<K: KeyType>(num_keys: usize) -> Vec<K::Secret> {
    let mut keys = Vec::with_capacity(num_keys);
    for i in 0..num_keys {
        let seed = [u8::try_from(i).unwrap(); 32];
        keys.push(K::generate_with_seed(Some(&seed)).unwrap());
    }
    keys
}

// BLS Tests
mod bls_tests {
    use super::*;
    use blueprint_crypto::{sp_core::SpBls377, sp_core::SpBls381};

    #[tokio::test]
    async fn test_bls381_basic_aggregation() {
        run_signature_aggregation_test::<SpBls381>(
            3,  // 3 nodes
            67, // 67% threshold (2 out of 3)
            "basic_bls381_aggregation",
            "1.0.0",
            generate_test_keys::<SpBls381>,
        )
        .await;
    }

    #[tokio::test]
    async fn test_bls377_basic_aggregation() {
        run_signature_aggregation_test::<SpBls377>(
            3,  // 3 nodes
            67, // 67% threshold (2 out of 3)
            "basic_bls377_aggregation",
            "1.0.0",
            generate_test_keys::<SpBls377>,
        )
        .await;
    }
}

// BN254 Tests
mod bn254_tests {
    use super::*;
    use blueprint_crypto::bn254::ArkBlsBn254;

    #[tokio::test]
    async fn test_bn254_basic_aggregation() {
        run_signature_aggregation_test::<ArkBlsBn254>(
            3,  // 3 nodes
            67, // 67% threshold (2 out of 3)
            "basic_bn254_aggregation",
            "1.0.0",
            generate_test_keys::<ArkBlsBn254>,
        )
        .await;
    }
}

mod w3f_bls_tests {
    use super::*;
    use blueprint_crypto::bls::{bls377::W3fBls377, bls381::W3fBls381};

    #[tokio::test]
    async fn test_w3f_bls381_basic_aggregation() {
        run_signature_aggregation_test::<W3fBls381>(
            3,  // 3 nodes
            67, // 67% threshold (2 out of 3),
            "basic_w3f_bls381_aggregation",
            "1.0.0",
            generate_test_keys::<W3fBls381>,
        )
        .await;
    }

    #[tokio::test]
    async fn test_w3f_bls377_basic_aggregation() {
        run_signature_aggregation_test::<W3fBls377>(
            3,  // 3 nodes
            67, // 67% threshold (2 out of 3),
            "basic_w3f_bls377_aggregation",
            "1.0.0",
            generate_test_keys::<W3fBls377>,
        )
        .await;
    }
}
