#![allow(clippy::too_many_lines)]
// IMPORTANT: These tests are sensitive to mDNS cross-contamination between
// parallel runs. We use unique network/instance names per test to reduce
// collisions; if you still see flakes, run with:
// cargo test -p blueprint-networking-agg-sig-gossip-extension --lib -- --test-threads=1

use crate::{
    protocol::{ProtocolConfig, SignatureAggregationProtocol},
    signature_weight::{EqualWeight, SignatureWeight},
};
use blueprint_core::info;
use blueprint_crypto::{aggregation::AggregatableSignature, hashing::blake3_256};
use blueprint_networking::{
    service_handle::NetworkServiceHandle,
    test_utils::{TestNode, wait_for_all_handshakes},
    AllowedKeys,
};
use blueprint_std::{collections::HashMap, time::Duration};
use libp2p::{Multiaddr, multiaddr::Protocol};
use std::{
    collections::HashSet,
    net::Ipv4Addr,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing_subscriber::EnvFilter;

// Constants for tests
const TEST_TIMEOUT: Duration = Duration::from_secs(180);

fn unique_test_suffix() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{}-{}", std::process::id(), nanos)
}

fn dialable_addr(addr: &Multiaddr) -> Multiaddr {
    let mut out = Multiaddr::empty();
    for component in addr.iter() {
        match component {
            Protocol::Ip4(ip) if ip.is_unspecified() => {
                out.push(Protocol::Ip4(Ipv4Addr::LOCALHOST));
            }
            _ => out.push(component),
        }
    }
    out
}

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

    let suffix = unique_test_suffix();
    let network_name = format!("{}-{}", network_name, suffix);
    let instance_name = format!("{}-{}", instance_name, suffix);

    // Create whitelisted nodes with explicit bootstrap peers to avoid mDNS flakes.
    let mut key_pairs = Vec::with_capacity(num_nodes);
    let mut allowed_keys = HashSet::with_capacity(num_nodes);
    for _ in 0..num_nodes {
        let key_pair = S::generate_with_seed(None).expect("Failed to generate key pair");
        allowed_keys.insert(S::public_from_secret(&key_pair));
        key_pairs.push(key_pair);
    }

    let mut nodes = Vec::with_capacity(num_nodes);
    let mut handles = Vec::with_capacity(num_nodes);
    let mut bootstrap_peers: Vec<Multiaddr> = Vec::new();

    info!("Starting all nodes");
    for (i, key_pair) in key_pairs.iter().enumerate() {
        let mut node = TestNode::new_with_keys(
            &network_name,
            &instance_name,
            AllowedKeys::InstancePublicKeys(allowed_keys.clone()),
            bootstrap_peers.clone(),
            Some(key_pair.clone()),
            None,
            false,
        );

        info!("Starting node {}", i);
        let handle = node.start().await.expect("Failed to start node");
        info!("Node {} started successfully", i);

        if let Some(addr) = handle.get_listen_addr() {
            bootstrap_peers.push(dialable_addr(&addr));
        }

        nodes.push(node);
        handles.push(handle);
    }
    info!("Created {} nodes successfully", nodes.len());

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
    let mut public_keys = HashMap::new();
    for (i, handle) in handles.iter().enumerate().take(num_nodes) {
        let public_key: S::Public = S::public_from_secret(&handle.local_signing_key);
        public_keys.insert(handle.local_peer_id, public_key.clone());
        info!(
            "Generated key pair for node {} - peer_id: {}, public_key: {:?}",
            i, handle.local_peer_id, public_key
        );
    }

    // Log all peer IDs and their corresponding public keys
    info!("Public keys mapping:");
    for (peer_id, public_key) in &public_keys {
        info!("Peer ID: {} -> Public key: {:?}", peer_id, public_key);
    }

    // Test message
    let message = b"test message";

    // Increase timeout for testing
    let protocol_timeout = Duration::from_secs(15);
    info!("Protocol timeout set to {:?}", protocol_timeout);

    // Use multiple aggregators for better reliability
    let num_aggregators = 2;
    info!("Using {} aggregators", num_aggregators);

    // Run the protocol directly on each node
    let mut results = Vec::new();
    info!("Starting protocol on {} nodes", num_nodes);

    let shared_message_hash = blake3_256(message);

    for (i, handle) in handles.iter().enumerate().take(num_nodes) {
        // Use the testing config for more reliable CI behavior
        let config = ProtocolConfig::for_testing(handle.clone(), num_aggregators);

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
            let result = protocol.run(&shared_message_hash).await;

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
        let aggregator_selector =
            crate::aggregator_selection::AggregatorSelector::new(num_aggregators);

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

// BLS Tests
mod bls_tests {
    use super::*;
    use blueprint_crypto::bls::{bls377::W3fBls377, bls381::W3fBls381};

    #[serial_test::serial]
    #[tokio::test]
    async fn test_bls381_basic_aggregation() {
        run_signature_aggregation_test::<W3fBls381>(
            3,  // 3 nodes
            67, // 67% threshold (2 out of 3)
            "basic_bls381_aggregation",
            "1.0.0",
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn test_bls377_basic_aggregation() {
        run_signature_aggregation_test::<W3fBls377>(
            3,  // 3 nodes
            67, // 67% threshold (2 out of 3)
            "basic_bls377_aggregation",
            "1.0.0",
        )
        .await;
    }
}

// BN254 Tests
mod bn254_tests {
    use super::*;
    use blueprint_crypto::bn254::ArkBlsBn254;

    #[serial_test::serial]
    #[tokio::test]
    async fn test_bn254_basic_aggregation() {
        run_signature_aggregation_test::<ArkBlsBn254>(
            3,  // 3 nodes
            67, // 67% threshold (2 out of 3)
            "basic_bn254_aggregation",
            "1.0.0",
        )
        .await;
    }
}

mod w3f_bls_tests {
    use super::*;
    use blueprint_crypto::bls::{bls377::W3fBls377, bls381::W3fBls381};

    #[serial_test::serial]
    #[tokio::test]
    async fn test_w3f_bls381_basic_aggregation() {
        run_signature_aggregation_test::<W3fBls381>(
            3,  // 3 nodes
            67, // 67% threshold (2 out of 3),
            "basic_w3f_bls381_aggregation",
            "1.0.0",
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn test_w3f_bls377_basic_aggregation() {
        run_signature_aggregation_test::<W3fBls377>(
            3,  // 3 nodes
            67, // 67% threshold (2 out of 3),
            "basic_w3f_bls377_aggregation",
            "1.0.0",
        )
        .await;
    }
}
