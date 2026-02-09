use crate::{
    NetworkConfig, NetworkService, service::AllowedKeys, service_handle::NetworkServiceHandle,
};
use blueprint_core::info;
use blueprint_crypto::KeyType;
use libp2p::{
    Multiaddr, PeerId,
    identity::{self, Keypair},
};
use std::{collections::HashSet, fmt::Write, time::Duration};
use tokio::time::timeout;

pub fn setup_log() {
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
    ///
    /// # Arguments
    ///
    /// * `network_name` - The name of the network
    /// * `instance_id` - The instance ID of the node
    /// * `allowed_keys` - The allowed keys for the node
    /// * `bootstrap_peers` - The bootstrap peers for the node
    /// * `instance_key_pair` - The instance key pair for the node
    /// * `local_key` - The local key for the node
    /// * `using_evm_address_for_handshake_verification` - Whether to use the EVM address for handshake verification
    ///
    /// # Returns
    ///
    /// Returns a new test node
    ///
    /// # Panics
    ///
    /// Panics if the local key is not provided and cannot be generated
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
    ///
    /// # Errors
    ///
    /// Returns an error if the service is already started
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

    /// Insert the allowed keys for this node
    pub fn insert_allowed_keys(&self, allowed_keys: AllowedKeys<K>) {
        if let Some(service) = &self.service {
            service.peer_manager.insert_whitelisted_keys(allowed_keys);
        }
    }
}

/// Wait for a condition with timeout
///
/// # Arguments
///
/// * `timeout` - The timeout for the wait
/// * `condition` - The condition to wait for
///
/// # Errors
///
/// Returns an error if the condition timed out
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
///
/// # Arguments
///
/// * `handles` - The handles to wait for peer discovery
/// * `timeout` - The timeout for the wait
///
/// # Errors
///
/// Returns an error if the peer discovery timed out
pub async fn wait_for_peer_discovery<K: KeyType>(
    handles: &[NetworkServiceHandle<K>],
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
///
/// # Arguments
///
/// * `handle1` - The first handle
/// * `handle2` - The second handle
/// * `timeout` - The timeout for the wait
///
/// # Panics
///
/// Panics if the peer info timed out
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

/// Helper to wait for handshake completion between multiple nodes
///
/// Uses exponential backoff with jitter for reliability in CI environments.
/// Provides detailed diagnostics on which node pairs are failing.
///
/// # Arguments
///
/// * `handles` - The handles to wait for handshake completion
/// * `timeout_length` - The timeout for the wait
///
/// # Panics
///
/// Panics if the handshake verification timed out, with details about which pairs failed
pub async fn wait_for_all_handshakes<K: KeyType>(
    handles: &[&mut NetworkServiceHandle<K>],
    timeout_length: Duration,
) {
    use std::collections::HashMap;

    let num_nodes = handles.len();
    let total_pairs = num_nodes * (num_nodes - 1);
    info!(
        "Starting handshake wait for {} nodes ({} pairs)",
        num_nodes, total_pairs
    );

    // Track verification progress for each pair
    let mut verified_pairs: HashMap<(usize, usize), bool> = HashMap::new();
    for i in 0..num_nodes {
        for j in 0..num_nodes {
            if i != j {
                verified_pairs.insert((i, j), false);
            }
        }
    }

    // Exponential backoff parameters
    let initial_delay = Duration::from_millis(50);
    let max_delay = Duration::from_millis(500);
    let mut current_delay = initial_delay;
    let backoff_factor = 1.5f64;

    let start_time = std::time::Instant::now();
    let mut last_progress_time = start_time;
    let mut last_verified_count = 0usize;

    let result = timeout(timeout_length, async {
        loop {
            let mut _newly_verified = 0usize;
            let mut pending_pairs = Vec::new();

            // Check all pairs
            for i in 0..num_nodes {
                for j in 0..num_nodes {
                    if i != j {
                        let was_verified = verified_pairs.get(&(i, j)).copied().unwrap_or(false);
                        if !was_verified {
                            let is_verified = handles[i]
                                .peer_manager
                                .is_peer_verified(&handles[j].local_peer_id);

                            if is_verified {
                                verified_pairs.insert((i, j), true);
                                _newly_verified += 1;
                                info!(
                                    "✓ Node {} -> Node {} handshake verified (peer_id: {})",
                                    i, j, handles[j].local_peer_id
                                );
                            } else {
                                pending_pairs.push((i, j));
                            }
                        }
                    }
                }
            }

            // Count total verified
            let verified_count = verified_pairs.values().filter(|&&v| v).count();

            // Check if we made progress
            if verified_count > last_verified_count {
                last_progress_time = std::time::Instant::now();
                last_verified_count = verified_count;
                // Reset backoff on progress
                current_delay = initial_delay;
            }

            // Check for completion
            if verified_count == total_pairs {
                info!(
                    "All {} handshakes completed successfully in {:?}",
                    total_pairs,
                    start_time.elapsed()
                );
                return Ok::<(), String>(());
            }

            // Log progress periodically (every 2 seconds of no progress)
            let stall_duration = last_progress_time.elapsed();
            if stall_duration > Duration::from_secs(2) && !pending_pairs.is_empty() {
                info!(
                    "Handshake progress: {}/{} verified, {} pending. Stalled for {:?}",
                    verified_count,
                    total_pairs,
                    pending_pairs.len(),
                    stall_duration
                );
                // Log first few pending pairs for debugging
                for (i, j) in pending_pairs.iter().take(3) {
                    let peer_i = &handles[*i].local_peer_id;
                    let peer_j = &handles[*j].local_peer_id;
                    info!(
                        "  Pending: Node {} ({}) -> Node {} ({})",
                        i, peer_i, j, peer_j
                    );
                }
            }

            // Only increase backoff when no progress was made this iteration.
            // Previously, the delay increased unconditionally, causing slow crypto
            // backends (BN254, W3fBls377) to hit 500ms polling intervals before
            // verifications could complete, leading to flaky timeouts.
            if verified_count == last_verified_count {
                current_delay = Duration::from_secs_f64(
                    (current_delay.as_secs_f64() * backoff_factor).min(max_delay.as_secs_f64()),
                );
            }

            // Wait with jitter based on time
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos();
            let jitter = Duration::from_millis(u64::from(nanos % 50));
            tokio::time::sleep(current_delay + jitter).await;
        }
    })
    .await;

    match result {
        Ok(Ok(())) => {}
        Ok(Err(e)) => panic!("Handshake error: {}", e),
        Err(_) => {
            // Timeout - provide detailed diagnostics
            let verified_count = verified_pairs.values().filter(|&&v| v).count();
            let pending: Vec<_> = verified_pairs
                .iter()
                .filter(|entry| !*entry.1)
                .map(|entry| (entry.0.0, entry.0.1))
                .collect();

            let mut error_msg = format!(
                "Handshake verification timed out after {:?}. Verified {}/{} pairs.\n",
                timeout_length, verified_count, total_pairs
            );
            error_msg.push_str("Pending handshakes:\n");
            for (i, j) in pending.iter().take(10) {
                let peer_i = &handles[*i].local_peer_id;
                let peer_j = &handles[*j].local_peer_id;
                let _ = writeln!(
                    error_msg,
                    "  Node {} ({}) -> Node {} ({})",
                    i, peer_i, j, peer_j
                );
            }
            if pending.len() > 10 {
                let _ = writeln!(error_msg, "  ... and {} more", pending.len() - 10);
            }
            panic!("{}", error_msg);
        }
    }
}

/// Helper to wait for handshake completion between two nodes
///
/// # Arguments
///
/// * `handle1` - The first handle
/// * `handle2` - The second handle
/// * `timeout_length` - The timeout for the wait
///
/// # Panics
///
/// Panics if the handshake verification timed out
pub async fn wait_for_handshake_completion<K: KeyType>(
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

/// Helper to create a whitelisted test node
///
/// # Arguments
///
/// * `network` - The network name
/// * `instance` - The instance ID
/// * `allowed_keys` - The allowed keys for the node
/// * `key_pair` - The key pair for the node
/// * `using_evm_address_for_handshake_verification` - Whether to use the EVM address for handshake verification
///
/// # Returns
///
/// Returns a new test node
pub fn create_node_with_keys<K: KeyType>(
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

/// Helper to create a set of nodes with whitelisted keys
///
/// # Arguments
///
/// * `count` - The number of nodes to create
/// * `using_evm_address_for_handshake_verification` - Whether to use the EVM address for handshake verification
///
/// # Returns
///
/// Returns a vector of test nodes
///
/// # Panics
///
/// Panics if the local key is not provided and cannot be generated
#[must_use]
pub fn create_whitelisted_nodes<K: KeyType>(
    count: usize,
    network_name: &str,
    instance_name: &str,
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
    for key_pair in &key_pairs {
        nodes.push(create_node_with_keys(
            network_name,
            instance_name,
            AllowedKeys::InstancePublicKeys(allowed_keys.clone()),
            Some(key_pair.clone()),
            using_evm_address_for_handshake_verification,
        ));
    }

    nodes
}
