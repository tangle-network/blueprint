use crate::service::AllowedKeys;
use alloy_primitives::Address;
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::KeyType;
use blueprint_crypto::hashing::keccak_256;
use crossbeam_channel::Receiver;
use dashmap::{DashMap, DashSet};
use libp2p::{PeerId, core::Multiaddr, identify};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};
use tokio::sync::broadcast;
use tracing::debug;

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
    /// Banned peers with optional expiration time
    banned_peers: DashMap<PeerId, Option<Instant>>,
    /// Whitelisted keys for the peer manager, must remain ordered and provide stable ordering.
    whitelisted_keys: Arc<RwLock<Vec<VerificationIdentifierKey<K>>>>,
    /// Event sender for peer updates
    event_tx: broadcast::Sender<PeerEvent>,
}

impl<K: KeyType> Default for PeerManager<K> {
    fn default() -> Self {
        Self::new(AllowedKeys::InstancePublicKeys(HashSet::default()))
    }
}

impl<K: KeyType> PeerManager<K> {
    #[must_use]
    pub fn new(allowed_keys: AllowedKeys<K>) -> Self {
        let (event_tx, _) = broadcast::channel(100);
        Self {
            peers: DashMap::default(),
            banned_peers: DashMap::default(),
            verified_peers: DashSet::default(),
            verification_id_keys_to_peer_ids: Arc::new(DashMap::default()),
            whitelisted_keys: match allowed_keys {
                AllowedKeys::EvmAddresses(addresses) => Arc::new(RwLock::new(
                    addresses
                        .into_iter()
                        .map(VerificationIdentifierKey::EvmAddress)
                        .collect(),
                )),
                AllowedKeys::InstancePublicKeys(keys) => Arc::new(RwLock::new(
                    keys.into_iter()
                        .map(VerificationIdentifierKey::InstancePublicKey)
                        .collect(),
                )),
            },
            event_tx,
        }
    }

    /// Run the allowed keys updater.
    /// This will clear the whitelisted keys and update them with the new allowed keys.
    ///
    /// # Arguments
    /// * `allowed_keys_rx` - A channel to receive allowed keys updates
    pub fn run_allowed_keys_updater(&self, allowed_keys_rx: &Receiver<AllowedKeys<K>>) {
        while let Ok(allowed_keys) = allowed_keys_rx.recv() {
            self.clear_whitelisted_keys();
            self.insert_whitelisted_keys(allowed_keys);
        }
    }

    /// Clears the whitelisted keys
    pub fn clear_whitelisted_keys(&self) {
        self.whitelisted_keys.write().clear();
    }

    /// Updates the whitelisted keys
    /// This will update the whitelisted keys with the new allowed keys
    ///
    /// # Arguments
    /// * `keys` - The allowed keys to update with
    pub fn insert_whitelisted_keys(&self, keys: AllowedKeys<K>) {
        match keys {
            AllowedKeys::EvmAddresses(addresses) => {
                self.whitelisted_keys.write().extend(
                    addresses
                        .into_iter()
                        .map(VerificationIdentifierKey::EvmAddress),
                );
            }
            AllowedKeys::InstancePublicKeys(keys) => {
                self.whitelisted_keys.write().extend(
                    keys.into_iter()
                        .map(VerificationIdentifierKey::InstancePublicKey),
                );
            }
        }
    }

    #[must_use]
    pub fn is_key_whitelisted(&self, key: &VerificationIdentifierKey<K>) -> bool {
        self.whitelisted_keys.read().contains(key)
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

    /// Log a successful interaction with a peer
    pub fn log_success(&self, peer_id: &PeerId, duration: Duration) {
        if let Some(mut info) = self.peers.get_mut(peer_id) {
            info.successes += 1;
            update_average_time(&mut info, duration);
            self.update_peer(*peer_id, info.clone());
        }
    }

    /// Log a failed interaction with a peer
    pub fn log_failure(&self, peer_id: &PeerId, duration: Duration) {
        if let Some(mut info) = self.peers.get_mut(peer_id) {
            info.failures += 1;
            update_average_time(&mut info, duration);
            self.update_peer(*peer_id, info.clone());
        }
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

    /// Get the position (index) of a verification key in the whitelist
    ///
    /// # Arguments
    /// * `key` - The verification identifier key to find
    ///
    /// # Returns
    /// Returns the index of the key in the whitelist if found, None otherwise
    #[must_use]
    pub fn get_key_position_in_whitelist(
        &self,
        key: &VerificationIdentifierKey<K>,
    ) -> Option<usize> {
        let whitelist = self.whitelisted_keys.read();
        whitelist.iter().position(|k| k == key)
    }

    /// Get the verification key for an index in the whitelist
    #[must_use]
    pub fn get_key_from_whitelist_index(
        &self,
        index: usize,
    ) -> Option<VerificationIdentifierKey<K>> {
        self.whitelisted_keys.read().get(index).cloned()
    }

    /// Get the peer id for an index in the whitelist
    #[must_use]
    pub fn get_peer_id_from_whitelist_index(&self, index: usize) -> Option<PeerId> {
        self.whitelisted_keys
            .read()
            .get(index)
            .and_then(|k| self.get_peer_id_from_verification_id_key(k))
    }

    /// Get the index of a peer id in the whitelist
    #[must_use]
    pub fn get_whitelist_index_from_peer_id(&self, peer_id: &PeerId) -> Option<usize> {
        self.whitelisted_keys
            .read()
            .iter()
            .position(|k| self.get_peer_id_from_verification_id_key(k) == Some(*peer_id))
    }
}

/// Update the average response time for a peer
fn update_average_time(info: &mut PeerInfo, duration: Duration) {
    const ALPHA: u32 = 5; // Smoothing factor for the moving average

    if info.average_response_time.is_none() {
        info.average_response_time = Some(duration);
    } else if duration < info.average_response_time.unwrap() {
        let delta = (info.average_response_time.unwrap() - duration) / ALPHA;
        info.average_response_time = Some(info.average_response_time.unwrap() - delta);
    } else {
        let delta = (duration - info.average_response_time.unwrap()) / ALPHA;
        info.average_response_time = Some(info.average_response_time.unwrap() + delta);
    }
}
