#![allow(dead_code)]

use crate::{
    NetworkConfig, NetworkService, service::AllowedKeys, service_handle::NetworkServiceHandle,
};
use blueprint_crypto::KeyType;
use libp2p::{
    Multiaddr, PeerId,
    identity::{self, Keypair},
};
use std::{collections::HashSet, time::Duration};
use tokio::time::timeout;
use tracing::info;

mod blueprint_protocol;
mod discovery;
mod gossip;
mod handshake;

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .try_init();
}

/// Test node configuration for network tests
pub struct TestNode<K: KeyType> {
    pub service: Option<NetworkService<K>>,
    pub peer_id: PeerId,
    pub listen_addr: Option<Multiaddr>,
    pub instance_key_pair: K::Secret,
    pub local_key: Keypair,
    pub using_evm_address_for_handshake_verification: bool,
}

impl<K: KeyType> TestNode<K> {
    /// Create a new test node with auto-generated keys
    pub fn new(
        network_name: &str,
        instance_id: &str,
        allowed_keys: AllowedKeys<K>,
        bootstrap_peers: Vec<Multiaddr>,
        using_evm_address_for_handshake_verification: bool,
    ) -> Self {
        Self::new_with_keys(
            network_name,
            instance_id,
            allowed_keys,
            bootstrap_peers,
            None,
            None,
            using_evm_address_for_handshake_verification,
        )
    }

    /// Create a new test node with specified keys
    pub fn new_with_keys(
        network_name: &str,
        instance_id: &str,
        allowed_keys: AllowedKeys<K>,
        bootstrap_peers: Vec<Multiaddr>,
        instance_key_pair: Option<K::Secret>,
        local_key: Option<Keypair>,
        using_evm_address_for_handshake_verification: bool,
    ) -> Self {
        let local_key = local_key.unwrap_or_else(identity::Keypair::generate_ed25519);
        let peer_id = local_key.public().to_peer_id();

        // Bind to all interfaces instead of just localhost
        let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
        info!("Creating test node {peer_id} with TCP address: {listen_addr}");

        let instance_key_pair =
            instance_key_pair.unwrap_or_else(|| K::generate_with_seed(None).unwrap());

        let config = NetworkConfig {
            network_name: network_name.to_string(),
            instance_id: instance_id.to_string(),
            instance_key_pair: instance_key_pair.clone(),
            local_key: local_key.clone(),
            listen_addr: listen_addr.clone(),
            target_peer_count: 10,
            bootstrap_peers,
            enable_mdns: true,
            enable_kademlia: true,
            using_evm_address_for_handshake_verification,
        };

        let (_, allowed_keys_rx) = crossbeam_channel::unbounded();
        let service = NetworkService::new(config, allowed_keys, allowed_keys_rx)
            .expect("Failed to create network service");

        Self {
            service: Some(service),
            peer_id,
            listen_addr: None,
            instance_key_pair,
            local_key,
            using_evm_address_for_handshake_verification,
        }
    }

    /// Start the node and wait for it to be fully initialized
    pub async fn start(&mut self) -> Result<NetworkServiceHandle<K>, &'static str> {
        // Take ownership of the service
        let service = self.service.take().ok_or("Service already started")?;
        let handle = service.start();

        // Wait for the node to be fully initialized
        let timeout_duration = Duration::from_secs(10); // Increased timeout
        match timeout(timeout_duration, async {
            // First wait for the listening address
            while self.listen_addr.is_none() {
                if let Some(addr) = handle.get_listen_addr() {
                    info!("Node {} listening on {}", self.peer_id, addr);
                    self.listen_addr = Some(addr.clone());

                    // Extract port from multiaddr
                    let addr_str = addr.to_string();
                    let port = addr_str.split('/').nth(4).unwrap_or("0").to_string();

                    // Try localhost first
                    let localhost_addr = format!("127.0.0.1:{}", port);
                    match tokio::net::TcpStream::connect(&localhost_addr).await {
                        Ok(_) => {
                            info!("Successfully verified localhost port for {}", self.peer_id);
                            break;
                        }
                        Err(e) => {
                            info!("Localhost port not ready for {}: {}", self.peer_id, e);
                            // Try external IP
                            let external_addr = format!("10.0.1.142:{}", port);
                            match tokio::net::TcpStream::connect(&external_addr).await {
                                Ok(_) => {
                                    info!(
                                        "Successfully verified external port for {}",
                                        self.peer_id
                                    );
                                    break;
                                }
                                Err(e) => {
                                    info!("External port not ready for {}: {}", self.peer_id, e);
                                    tokio::time::sleep(Duration::from_millis(100)).await;
                                    continue;
                                }
                            }
                        }
                    }
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }

            // Give the node a moment to initialize protocols
            tokio::time::sleep(Duration::from_millis(500)).await;

            Ok::<(), &'static str>(())
        })
        .await
        {
            Ok(Ok(())) => {
                info!("Node {} fully initialized", self.peer_id);
                Ok(handle)
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Err("Timeout waiting for node to initialize"),
        }
    }

    /// Get the actual listening address
    pub fn get_listen_addr(&self) -> Option<Multiaddr> {
        self.listen_addr.clone()
    }

    /// Update the allowed keys for this node
    pub fn update_allowed_keys(&self, allowed_keys: AllowedKeys<K>) {
        if let Some(service) = &self.service {
            service.peer_manager.update_whitelisted_keys(allowed_keys);
        }
    }
}

/// Wait for a condition with timeout
pub async fn wait_for_condition<F>(timeout: Duration, mut condition: F) -> Result<(), &'static str>
where
    F: FnMut() -> bool,
{
    let start = std::time::Instant::now();
    while !condition() {
        if start.elapsed() > timeout {
            return Err("Timeout waiting for condition");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    Ok(())
}

/// Wait for peers to discover each other
pub async fn wait_for_peer_discovery<K: KeyType>(
    handles: &[&NetworkServiceHandle<K>],
    timeout: Duration,
) -> Result<(), &'static str> {
    info!("Waiting for peer discovery...");

    wait_for_condition(timeout, || {
        for (i, handle1) in handles.iter().enumerate() {
            for (j, handle2) in handles.iter().enumerate() {
                if i != j && !handle1.peers().contains(&handle2.local_peer_id) {
                    return false;
                }
            }
        }
        true
    })
    .await
}

/// Wait for peer info to be updated
pub async fn wait_for_peer_info<K: KeyType>(
    handle1: &NetworkServiceHandle<K>,
    handle2: &NetworkServiceHandle<K>,
    timeout: Duration,
) {
    info!("Waiting for identify info...");

    match tokio::time::timeout(timeout, async {
        loop {
            let peer_info1 = handle1.peer_info(&handle2.local_peer_id);
            let peer_info2 = handle2.peer_info(&handle1.local_peer_id);

            if let Some(peer_info) = peer_info1 {
                if peer_info.identify_info.is_some() {
                    // Also verify reverse direction
                    if let Some(peer_info) = peer_info2 {
                        if peer_info.identify_info.is_some() {
                            info!("Identify info populated in both directions");
                            break;
                        }
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    {
        Ok(()) => info!("Peer info updated successfully in both directions"),
        Err(e) => panic!("Peer info update timed out: {e}"),
    }
}

// Helper to wait for handshake completion between multiple nodes
async fn wait_for_all_handshakes<K: KeyType>(
    handles: &[&mut NetworkServiceHandle<K>],
    timeout_length: Duration,
) {
    info!("Starting handshake wait for {} nodes", handles.len());
    timeout(timeout_length, async {
        loop {
            let mut all_verified = true;
            for (i, handle1) in handles.iter().enumerate() {
                for (j, handle2) in handles.iter().enumerate() {
                    if i != j {
                        let verified = handle1
                            .peer_manager
                            .is_peer_verified(&handle2.local_peer_id);
                        if !verified {
                            info!("Node {} -> Node {}: handshake not verified yet", i, j);
                            all_verified = false;
                            break;
                        }
                    }
                }
                if !all_verified {
                    break;
                }
            }
            if all_verified {
                info!("All handshakes completed successfully");
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("Handshake verification timed out");
}

// Helper to wait for handshake completion between two nodes
async fn wait_for_handshake_completion<K: KeyType>(
    handle1: &NetworkServiceHandle<K>,
    handle2: &NetworkServiceHandle<K>,
    timeout_length: Duration,
) {
    timeout(timeout_length, async {
        loop {
            if handle1
                .peer_manager
                .is_peer_verified(&handle2.local_peer_id)
                && handle2
                    .peer_manager
                    .is_peer_verified(&handle1.local_peer_id)
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("Handshake verification timed out");
}

// Helper to create a whitelisted test node
fn create_node_with_keys<K: KeyType>(
    network: &str,
    instance: &str,
    allowed_keys: AllowedKeys<K>,
    key_pair: Option<K::Secret>,
    using_evm_address_for_handshake_verification: bool,
) -> TestNode<K> {
    TestNode::new_with_keys(
        network,
        instance,
        allowed_keys,
        vec![],
        key_pair,
        None,
        using_evm_address_for_handshake_verification,
    )
}

// Helper to create a set of nodes with whitelisted keys
async fn create_whitelisted_nodes<K: KeyType>(
    count: usize,
    using_evm_address_for_handshake_verification: bool,
) -> Vec<TestNode<K>> {
    let mut nodes = Vec::with_capacity(count);
    let mut key_pairs = Vec::with_capacity(count);
    let mut allowed_keys = HashSet::new();

    // Generate all key pairs first
    for _ in 0..count {
        let key_pair = K::generate_with_seed(None).unwrap();
        key_pairs.push(key_pair.clone());
        allowed_keys.insert(K::public_from_secret(&key_pair));
    }

    // Create nodes with whitelisted keys
    for key_pair in key_pairs {
        nodes.push(create_node_with_keys(
            "test-net",
            "sum-test",
            AllowedKeys::InstancePublicKeys(allowed_keys.clone()),
            Some(key_pair),
            using_evm_address_for_handshake_verification,
        ));
    }

    nodes
}
