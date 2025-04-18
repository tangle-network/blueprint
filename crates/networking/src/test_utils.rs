use crate::{
    discovery::peers::{PeerInfo, WhitelistedKeys},
    error::Error as NetworkError,
    service::{NetworkConfig, NetworkService},
    service_handle::NetworkServiceHandle,
};
use blueprint_crypto::KeyType;
use blueprint_std::Rng;
use blueprint_std::rand;
use crossbeam_channel;
use libp2p::{
    Multiaddr, PeerId,
    identity::{Keypair, PublicKey},
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

/// A test node for network testing
pub struct TestNode<K: KeyType> {
    /// Handle to the network service
    pub handle: NetworkServiceHandle<K>,
    /// Peer ID of the node
    pub peer_id: PeerId,
    /// Public key of the node
    pub public_key: PublicKey,
    /// Instance key pair of the node
    pub instance_key_pair: K::Secret,
    /// Network name of the node
    pub network_name: String,
    /// Instance ID of the node
    pub instance_id: String,
}

impl<K: KeyType> TestNode<K> {
    /// Create a new test node with default keys
    ///
    /// # Panics
    ///
    /// This function panics if:
    /// - Key generation fails
    /// - Network service creation fails
    #[must_use]
    pub fn new(
        network_name: &str,
        instance_id: &str,
        whitelisted_keys: WhitelistedKeys<K>,
        bootstrap_nodes: &[String],
        enable_mdns: bool,
    ) -> Self {
        let instance_key_pair = K::generate_with_seed(None).unwrap();
        Self::new_with_keys(
            network_name,
            instance_id,
            whitelisted_keys,
            bootstrap_nodes,
            Some(instance_key_pair),
            None,
            enable_mdns,
        )
    }

    /// Create a new test node with specified keys
    ///
    /// # Panics
    ///
    /// This function panics if:
    /// - Key generation fails
    /// - Network address parsing fails
    /// - Network service creation fails
    #[must_use]
    pub fn new_with_keys(
        network_name: &str,
        instance_id: &str,
        whitelisted_keys: WhitelistedKeys<K>,
        bootstrap_nodes: &[String],
        instance_key_pair: Option<K::Secret>,
        transport_keypair: Option<Keypair>,
        enable_mdns: bool,
    ) -> Self {
        let instance_key_pair =
            instance_key_pair.unwrap_or_else(|| K::generate_with_seed(None).unwrap());
        let transport_keypair = transport_keypair.unwrap_or_else(Keypair::generate_ed25519);
        let peer_id = PeerId::from(transport_keypair.public());

        // Create bootstrap peers list from strings
        let bootstrap_peers = bootstrap_nodes
            .iter()
            .filter_map(|addr| addr.parse::<Multiaddr>().ok())
            .collect();

        // Create a listen address
        let port = 40000 + rand::thread_rng().gen_range(0..1000);
        let listen_addr = format!("/ip4/127.0.0.1/tcp/{}", port).parse().unwrap();

        let config = NetworkConfig {
            network_name: network_name.to_string(),
            instance_id: instance_id.to_string(),
            instance_key_pair: instance_key_pair.clone(),
            local_key: transport_keypair.clone(),
            listen_addr,
            target_peer_count: 25,
            bootstrap_peers,
            enable_mdns,
            enable_kademlia: true,
            using_evm_address_for_handshake_verification: false,
        };

        // Create a channel for whitelist updates
        let (_, whitelist_rx) = crossbeam_channel::unbounded();

        let service = NetworkService::<K>::new(config, whitelisted_keys, whitelist_rx)
            .expect("Failed to create network service");

        // Create a handle and start the service
        let handle = service.start();

        Self {
            handle,
            peer_id,
            public_key: transport_keypair.public(),
            instance_key_pair,
            network_name: network_name.to_string(),
            instance_id: instance_id.to_string(),
        }
    }

    /// Start the network service and return a handle to it
    ///
    /// # Errors
    ///
    /// This function returns an error if the network service fails to start
    pub async fn start(&mut self) -> Result<NetworkServiceHandle<K>, NetworkError> {
        // Give the service a moment to start
        sleep(Duration::from_millis(100)).await;
        Ok(self.handle.clone())
    }

    /// Get the listen address of the node
    ///
    /// # Returns
    ///
    /// The listen address of the node as a String
    #[must_use]
    pub fn get_listen_addr(&self) -> Option<String> {
        self.handle.get_listen_addr().map(|addr| addr.to_string())
    }
}

/// Initialize tracing for tests
pub fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}

/// Setup logging for tests
pub fn setup_log() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}

/// Create a set of whitelisted nodes for testing
///
/// # Panics
///
/// This function panics if:
/// - Key generation fails
/// - Network address parsing fails
/// - Network service creation fails
#[allow(clippy::unused_async)]
pub async fn create_whitelisted_nodes<K: KeyType>(
    num_nodes: usize,
    network_name: &str,
    instance_id: &str,
    enable_mdns: bool,
) -> Vec<TestNode<K>> {
    init_tracing();

    let mut nodes = Vec::new();
    let mut peer_ids = Vec::new();
    let mut keypairs = Vec::new();

    // First create the keypairs and peer IDs
    for _ in 0..num_nodes {
        let keypair = Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());
        peer_ids.push(peer_id);
        keypairs.push(keypair);
    }

    // Then create the nodes with the whitelist
    for (i, (peer_id, keypair)) in peer_ids.iter().zip(keypairs).enumerate() {
        // Create a listen address with a unique port
        #[allow(clippy::cast_possible_truncation)]
        let port = 40000 + i as u16; // This is safe as we're unlikely to have more than 25,000 test nodes
        let listen_addr = format!("/ip4/127.0.0.1/tcp/{}", port).parse().unwrap();

        let instance_key = K::generate_with_seed(None).unwrap();

        let config = NetworkConfig {
            network_name: format!("{}-{}", network_name, i),
            instance_id: instance_id.to_string(),
            instance_key_pair: instance_key.clone(),
            local_key: keypair.clone(),
            listen_addr,
            target_peer_count: 25,
            bootstrap_peers: vec![],
            enable_mdns,
            enable_kademlia: true,
            using_evm_address_for_handshake_verification: false,
        };

        // Create a whitelist with empty keys for now
        let whitelist = WhitelistedKeys::<K>::new(vec![]);
        let (_, whitelist_rx) = crossbeam_channel::unbounded();

        let service = NetworkService::<K>::new(config, whitelist, whitelist_rx)
            .expect("Failed to create network service");

        // Create a handle and start the service
        let handle = service.start();

        nodes.push(TestNode {
            handle,
            peer_id: *peer_id,
            public_key: keypair.public(),
            instance_key_pair: instance_key,
            network_name: network_name.to_string(),
            instance_id: instance_id.to_string(),
        });
    }

    nodes
}

/// Wait for all nodes to discover each other
pub async fn wait_for_peer_discovery<K: KeyType>(nodes: &[TestNode<K>], timeout_secs: u64) -> bool {
    let expected_peers = nodes.len() - 1;
    let timeout = Duration::from_secs(timeout_secs);
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        let mut all_connected = true;
        for (i, node) in nodes.iter().enumerate() {
            let connected_peers = node.handle.peer_manager.get_peers().len();
            if connected_peers < expected_peers {
                all_connected = false;
                info!(
                    "Node {} has {} peers, waiting for {}",
                    i, connected_peers, expected_peers
                );
                break;
            }
        }

        if all_connected {
            return true;
        }

        sleep(Duration::from_millis(100)).await;
    }

    warn!("Timeout waiting for peer discovery");
    false
}

/// Wait for all handles to discover each other
///
/// # Errors
///
/// This function returns an error if the timeout is reached before all handles discover each other
pub async fn wait_for_handle_peer_discovery<K: KeyType>(
    handles: &[&NetworkServiceHandle<K>],
    timeout: Duration,
) -> Result<(), NetworkError> {
    let expected_peers = handles.len() - 1;
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        let mut all_connected = true;
        for (i, handle) in handles.iter().enumerate() {
            let connected_peers = handle.peer_manager.get_peers().len();
            if connected_peers < expected_peers {
                all_connected = false;
                info!(
                    "Handle {} has {} peers, waiting for {}",
                    i, connected_peers, expected_peers
                );
                break;
            }
        }

        if all_connected {
            info!("All handles have discovered each other");
            return Ok(());
        }

        sleep(Duration::from_millis(100)).await;
    }

    Err(NetworkError::Other(
        "Timeout waiting for peer discovery".into(),
    ))
}

/// Wait for all nodes to complete handshakes with each other
pub async fn wait_for_all_handshakes<K: KeyType>(
    nodes: &[&TestNode<K>],
    timeout_secs: u64,
) -> bool {
    let expected_peers = nodes.len() - 1;
    let timeout = Duration::from_secs(timeout_secs);
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        let mut all_verified = true;
        for (i, node) in nodes.iter().enumerate() {
            // Count verified peers
            let peers = node.handle.peer_manager.get_peers();
            let mut verified_count = 0;

            #[allow(clippy::explicit_iter_loop)]
            for peer_ref in peers.iter() {
                let peer_id = peer_ref.key();
                if node.handle.peer_manager.is_peer_verified(peer_id) {
                    verified_count += 1;
                }
            }

            if verified_count < expected_peers {
                all_verified = false;
                info!(
                    "Node {} has {} verified peers, waiting for {}",
                    i, verified_count, expected_peers
                );
                break;
            }
        }

        if all_verified {
            return true;
        }

        sleep(Duration::from_millis(100)).await;
    }

    warn!("Timeout waiting for handshakes");
    false
}

/// Wait for handshake completion between two nodes
pub async fn wait_for_handshake_completion<K: KeyType>(
    handle1: &NetworkServiceHandle<K>,
    handle2: &NetworkServiceHandle<K>,
    timeout: Duration,
) -> bool {
    let start = std::time::Instant::now();
    let peer_id1 = handle1.local_peer_id;
    let peer_id2 = handle2.local_peer_id;

    while start.elapsed() < timeout {
        let verified1 = handle1.peer_manager.is_peer_verified(&peer_id2);
        let verified2 = handle2.peer_manager.is_peer_verified(&peer_id1);

        if verified1 && verified2 {
            info!("Handshake completed between peers");
            return true;
        }

        if !verified1 {
            info!("Node 1 has not verified Node 2 yet");
        }
        if !verified2 {
            info!("Node 2 has not verified Node 1 yet");
        }

        sleep(Duration::from_millis(100)).await;
    }

    warn!("Timeout waiting for handshake completion");
    false
}

/// Wait for peer info to be updated
///
/// # Errors
///
/// This function returns an error if the timeout is reached before peer info is updated
pub async fn wait_for_peer_info<K: KeyType>(
    node: &TestNode<K>,
    peer_id: &PeerId,
    timeout_secs: u64,
) -> Result<PeerInfo, NetworkError> {
    let timeout = Duration::from_secs(timeout_secs);
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        if let Some(info) = node.handle.peer_manager.get_peer_info(peer_id) {
            return Ok(info);
        }
        sleep(Duration::from_millis(100)).await;
    }

    Err(NetworkError::Other(
        "Timeout waiting for peer info".to_string(),
    ))
}
