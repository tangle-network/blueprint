use crate::types::ParticipantId;
use alloy_primitives::Address;
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::KeyType;
use blueprint_crypto::hashing::keccak_256;
use crossbeam_channel::Receiver;
use dashmap::{DashMap, DashSet};
use libp2p::{PeerId, core::Multiaddr, identify};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
    time::{Duration, Instant, SystemTime},
};
use tokio::sync::broadcast;
use tracing::{debug, info};

use super::utils::{get_address_from_pubkey, secp256k1_ecdsa_recover};

/// A key that can be used to verify a peer
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VerificationIdentifierKey<K: KeyType> {
    EvmAddress(Address),
    InstancePublicKey(K::Public),
}

impl<K: KeyType> VerificationIdentifierKey<K> {
    /// Verify a signature against the verification identifier key
    ///
    /// # Arguments
    /// * `msg` - The message to verify
    /// * `signature` - The signature to verify
    ///
    /// # Returns
    /// `true` if the signature is valid, `false` otherwise
    ///
    /// # Errors
    /// Returns an error if the signature is invalid
    pub fn verify(
        &self,
        msg: &[u8],
        signature: &[u8],
    ) -> Result<bool, Box<dyn blueprint_std::error::Error>> {
        match self {
            VerificationIdentifierKey::EvmAddress(address) => {
                let msg = keccak_256(msg);
                let mut sig: [u8; 65] = [0u8; 65];
                sig[..signature.len()].copy_from_slice(signature);

                let pubkey = secp256k1_ecdsa_recover(&sig, &msg)?;

                let address_from_pk = get_address_from_pubkey(&pubkey);
                Ok(address_from_pk == *address)
            }
            VerificationIdentifierKey::InstancePublicKey(public_key) => {
                let signature = K::Signature::from_bytes(signature)?;
                Ok(K::verify(public_key, msg, &signature))
            }
        }
    }
}

/// A collection of whitelisted keys with thread-safe interior mutability
/// that preserves the order of keys
#[derive(Debug, Clone, Default)]
pub struct WhitelistedKeys<K: KeyType> {
    // We use RwLock to provide interior mutability in a thread-safe way
    // The Vec ensures ordering is preserved exactly as provided
    inner: Arc<RwLock<Vec<VerificationIdentifierKey<K>>>>,
}

impl<K: KeyType> WhitelistedKeys<K> {
    #[must_use]
    pub fn new(keys: Vec<VerificationIdentifierKey<K>>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(keys)),
        }
    }

    #[must_use]
    pub fn new_from_hashset(keys: HashSet<VerificationIdentifierKey<K>>) -> Self {
        Self::new(keys.into_iter().collect())
    }

    /// Get a read-only view of the keys
    pub fn keys(&self) -> Vec<VerificationIdentifierKey<K>> {
        match self.inner.read() {
            Ok(guard) => guard.clone(),
            Err(_) => {
                // Handle poisoned lock - this is a fallback
                debug!("WhitelistedKeys lock was poisoned, falling back to empty keys");
                Vec::new()
            }
        }
    }

    /// Update the keys with a new list
    pub fn update(&self, keys: Vec<VerificationIdentifierKey<K>>) {
        if let Ok(mut guard) = self.inner.write() {
            debug!("Updating whitelisted keys with {} keys", keys.len());
            *guard = keys;
        } else {
            debug!("Failed to update whitelisted keys due to poisoned lock");
        }
    }

    /// Checks if a verification identifier key is whitelisted
    ///
    /// # Arguments
    /// * `key` - The verification identifier key to check
    ///
    /// # Returns
    /// `true` if the key is whitelisted, `false` otherwise
    pub fn contains(&self, key: &VerificationIdentifierKey<K>) -> bool {
        match self.inner.read() {
            Ok(guard) => guard.contains(key),
            Err(_) => {
                debug!("WhitelistedKeys lock was poisoned during contains check");
                false
            }
        }
    }

    /// Get a specific key by index, maintaining the original order
    pub fn get_by_index(&self, index: usize) -> Option<VerificationIdentifierKey<K>> {
        match self.inner.read() {
            Ok(guard) => guard.get(index).cloned(),
            Err(_) => {
                debug!("WhitelistedKeys lock was poisoned during get_by_index");
                None
            }
        }
    }

    /// Get the number of keys in the whitelist
    pub fn len(&self) -> usize {
        match self.inner.read() {
            Ok(guard) => guard.len(),
            Err(_) => {
                debug!("WhitelistedKeys lock was poisoned during len check");
                0
            }
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Information about a peer's connection and behavior
#[derive(Clone, Debug)]
pub struct PeerInfo {
    /// Known addresses for the peer
    pub addresses: HashSet<Multiaddr>,
    /// Information from the identify protocol
    pub identify_info: Option<identify::Info>,
    /// When the peer was last seen
    pub last_seen: SystemTime,
    /// Latest ping latency
    pub ping_latency: Option<Duration>,
    /// Number of successful protocol interactions
    pub successes: u32,
    /// Number of failed protocol interactions
    pub failures: u32,
    /// Average response time for protocol requests
    pub average_response_time: Option<Duration>,
}

impl Default for PeerInfo {
    fn default() -> Self {
        Self {
            addresses: HashSet::new(),
            identify_info: None,
            last_seen: SystemTime::now(),
            ping_latency: None,
            successes: 0,
            failures: 0,
            average_response_time: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PeerEvent {
    /// A peer was added or updated
    PeerUpdated {
        peer_id: PeerId,
        info: Box<PeerInfo>,
    },
    /// A peer was removed
    PeerRemoved { peer_id: PeerId, reason: String },
    /// A peer was banned
    PeerBanned {
        peer_id: PeerId,
        reason: String,
        expires_at: Option<Instant>,
    },
    /// A peer was unbanned
    PeerUnbanned { peer_id: PeerId },
}

pub struct PeerManager<K: KeyType> {
    /// Active peers and their information
    peers: DashMap<PeerId, PeerInfo>,
    /// Verified peers from completed handshakes
    verified_peers: DashSet<PeerId>,
    /// Handshake keys to peer ids
    verification_id_keys_to_peer_ids: Arc<DashMap<VerificationIdentifierKey<K>, PeerId>>,
    /// Peer ids to handshake keys
    peer_ids_to_verification_id_keys: Arc<DashMap<PeerId, VerificationIdentifierKey<K>>>,
    /// Banned peers with optional expiration time
    banned_peers: DashMap<PeerId, Option<Instant>>,
    /// Allowed public keys
    whitelisted_keys: WhitelistedKeys<K>,
    /// Event sender for peer updates
    event_tx: broadcast::Sender<PeerEvent>,
}

impl<K: KeyType> Default for PeerManager<K> {
    fn default() -> Self {
        Self::new(WhitelistedKeys::new(Vec::default()))
    }
}

impl<K: KeyType> PeerManager<K> {
    #[must_use]
    pub fn new(whitelisted_keys: WhitelistedKeys<K>) -> Self {
        let (event_tx, _) = broadcast::channel(100);
        Self {
            peers: DashMap::default(),
            banned_peers: DashMap::default(),
            verified_peers: DashSet::default(),
            verification_id_keys_to_peer_ids: Arc::new(DashMap::default()),
            peer_ids_to_verification_id_keys: Arc::new(DashMap::default()),
            whitelisted_keys,
            event_tx,
        }
    }

    /// Spawns a dedicated thread to run the whitelist updater
    ///
    /// # Arguments
    /// * `whitelisted_keys_rx` - A channel to receive whitelisted keys updates
    ///
    /// # Returns
    /// A join handle for the spawned thread
    #[must_use]
    pub fn spawn_whitelist_updater(
        self: Arc<Self>,
        whitelisted_keys_rx: Receiver<WhitelistedKeys<K>>,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            debug!("Starting whitelist updater thread");
            let receiver = whitelisted_keys_rx;
            let self_clone = self.clone();
            while let Ok(whitelisted_keys) = receiver.recv() {
                // Update the internal whitelist when new keys are received
                self_clone.update_whitelist(&whitelisted_keys);
            }
            debug!("Whitelist updater thread terminated");
        })
    }

    /// Updates the whitelist with new keys
    /// This can be called from an immutable reference (e.g. Arc<PeerManager>)
    ///
    /// # Arguments
    /// * `keys` - The new whitelist to update with
    pub fn update_whitelist(&self, keys: &WhitelistedKeys<K>) {
        // Create a new Vec with keys from the input WhitelistedKeys
        let new_keys = keys.keys();

        // Log the update
        info!("Updating whitelist with {} keys", new_keys.len());

        // Create a list to store key-to-peer mappings we want to preserve
        let mut preserved_mappings = Vec::new();

        // Preserve existing peer ID mappings for verification keys that are still valid
        for key in &new_keys {
            if let Some(peer_id) = self.get_peer_id_from_verification_id_key(key) {
                preserved_mappings.push((key.clone(), peer_id));
            }
        }

        // Update our whitelisted_keys - this uses the RwLock inside WhitelistedKeys
        self.whitelisted_keys.update(new_keys);

        // Restore the preserved mappings
        for (key, peer_id) in preserved_mappings {
            self.verification_id_keys_to_peer_ids.insert(key, peer_id);
        }
    }

    #[must_use]
    pub fn is_key_whitelisted(&self, key: &VerificationIdentifierKey<K>) -> bool {
        self.whitelisted_keys.contains(key)
    }

    pub fn handle_nonwhitelisted_peer(&self, peer: &PeerId) {
        self.remove_peer(peer, "non-whitelisted");
        self.ban_peer(*peer, "non-whitelisted", None);
    }

    /// Get a subscription to peer events
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<PeerEvent> {
        self.event_tx.subscribe()
    }

    /// Update or add peer information
    pub fn update_peer(&self, peer_id: PeerId, mut info: PeerInfo) {
        // Update last seen time
        info.last_seen = SystemTime::now();

        // Insert or update peer info
        self.peers.insert(peer_id, info.clone());

        // Emit event
        let _ = self.event_tx.send(PeerEvent::PeerUpdated {
            peer_id,
            info: Box::new(info),
        });
    }

    /// Remove a peer
    pub fn remove_peer(&self, peer_id: &PeerId, reason: impl Into<String>) {
        if self.peers.remove(peer_id).is_some() {
            let reason = reason.into();
            debug!(%peer_id, %reason, "removed peer");
            let _ = self.event_tx.send(PeerEvent::PeerRemoved {
                peer_id: *peer_id,
                reason,
            });
        }
    }

    /// Verify a peer
    pub fn verify_peer(&self, peer_id: &PeerId) {
        self.verified_peers.insert(*peer_id);
    }

    /// Check if a peer is verified
    #[must_use]
    pub fn is_peer_verified(&self, peer_id: &PeerId) -> bool {
        self.verified_peers.contains(peer_id)
    }

    /// Ban a peer with optional expiration
    pub fn ban_peer(&self, peer_id: PeerId, reason: impl Into<String>, duration: Option<Duration>) {
        let expires_at = duration.map(|d| Instant::now() + d);

        // Remove from active peers
        self.remove_peer(&peer_id, "banned");

        // Add to banned peers
        self.banned_peers.insert(peer_id, expires_at);

        let reason = reason.into();
        debug!(%peer_id, %reason, "banned peer");
        let _ = self.event_tx.send(PeerEvent::PeerBanned {
            peer_id,
            reason,
            expires_at,
        });
    }

    /// Bans a peer with the default duration(`1h`)
    pub fn ban_peer_with_default_duration(&self, peer: PeerId, reason: impl Into<String>) {
        const BAN_PEER_DURATION: Duration = Duration::from_secs(60 * 60); //1h
        self.ban_peer(peer, reason, Some(BAN_PEER_DURATION));
    }

    /// Unban a peer
    pub fn unban_peer(&self, peer_id: &PeerId) {
        if self.banned_peers.remove(peer_id).is_some() {
            debug!(%peer_id, "unbanned peer");
            let _ = self
                .event_tx
                .send(PeerEvent::PeerUnbanned { peer_id: *peer_id });
        }
    }

    /// Check if a peer is banned
    #[must_use]
    pub fn is_banned(&self, peer_id: &PeerId) -> bool {
        self.banned_peers.contains_key(peer_id)
    }

    /// Get peer information
    #[must_use]
    pub fn get_peer_info(&self, peer_id: &PeerId) -> Option<PeerInfo> {
        self.peers.get(peer_id).map(|info| info.value().clone())
    }

    /// Get all active peers
    #[must_use]
    pub fn get_peers(&self) -> DashMap<PeerId, PeerInfo> {
        self.peers.clone()
    }

    /// Get number of active peers
    #[must_use]
    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    /// Start the background task to clean up expired bans
    pub async fn run_ban_cleanup(self: Arc<Self>) {
        loop {
            let now = Instant::now();
            let mut to_unban = Vec::new();

            // Find expired bans
            let banned_peers = self.banned_peers.clone().into_read_only();
            for (peer_id, expires_at) in banned_peers.iter() {
                if let Some(expiry) = expires_at {
                    if now >= *expiry {
                        to_unban.push(*peer_id);
                    }
                }
            }

            // Unban expired peers
            for peer_id in to_unban {
                self.unban_peer(&peer_id);
            }

            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }

    /// Add a peer id to the public key to peer id map after verifying handshake
    pub fn link_peer_id_to_verification_id_key(
        &self,
        peer_id: &PeerId,
        verification_id_key: &VerificationIdentifierKey<K>,
    ) {
        self.verification_id_keys_to_peer_ids
            .insert(verification_id_key.clone(), *peer_id);
        self.peer_ids_to_verification_id_keys
            .insert(*peer_id, verification_id_key.clone());
    }

    /// Remove a peer id from the public key to peer id map
    pub fn remove_peer_id_from_verification_id_key(&self, peer_id: &PeerId) {
        self.verification_id_keys_to_peer_ids
            .retain(|_, id| id != peer_id);
    }

    #[must_use]
    pub fn get_peer_id_from_verification_id_key(
        &self,
        verification_id_key: &VerificationIdentifierKey<K>,
    ) -> Option<PeerId> {
        self.verification_id_keys_to_peer_ids
            .get(verification_id_key)
            .map(|id| *id)
    }

    /// This function iterates over the currently known whitelisted keys and finds the
    /// position (index) of the key corresponding to the given `peer_id`.
    ///
    /// # Panics
    ///
    /// Panics if the read locks for internal maps cannot be acquired (poisoned lock).
    /// Panics if the key derived from the `peer_id` is not found within the internal
    /// `whitelisted_keys` set. This typically indicates an inconsistent state where
    /// a peer is known but not part of the current official whitelist.
    #[must_use]
    pub fn get_party_index_from_peer_id(&self, peer_id: &PeerId) -> Option<ParticipantId> {
        // Get the verification key from the DashMap (no lock needed for read)
        let key_ref = self
            .peer_ids_to_verification_id_keys
            .get(peer_id)
            .expect("Peer ID not found in key map");
        let key = key_ref.value(); // Get the actual key from the Ref

        // Read lock the inner IndexSet within WhitelistedKeys
        let locked_whitelist_inner = self
            .whitelisted_keys
            .inner // Access the inner Arc<RwLock<...>>
            .read()
            .expect("Whitelist inner lock poisoned");

        locked_whitelist_inner // This guard dereferences to IndexSet
            .iter()
            .position(|k| k == key) // Compare with the dereferenced key
            .and_then(|p_index| u16::try_from(p_index).ok())
            .map(ParticipantId)
    }
}
