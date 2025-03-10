use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    task::{Context, Poll},
    time::Duration,
};

use crate::error::Error;
use gadget_crypto::KeyType;
use libp2p::{
    autonat,
    core::Multiaddr,
    identify,
    identity::PeerId,
    kad::{self, store::MemoryStore},
    mdns::{self, Event as MdnsEvent},
    relay,
    swarm::{
        NetworkBehaviour, ToSwarm, behaviour::toggle::Toggle, derive_prelude::*,
        dial_opts::DialOpts,
    },
    upnp,
};
use tokio::time::Interval;
use tracing::trace;
use tracing::{debug, info};

use super::PeerInfo;

#[derive(NetworkBehaviour)]
pub struct DerivedDiscoveryBehaviour {
    /// Kademlia discovery
    pub kademlia: Toggle<kad::Behaviour<MemoryStore>>,
    /// Local network discovery via mDNS
    pub mdns: Toggle<mdns::tokio::Behaviour>,
    /// Identify protocol for peer information exchange
    pub identify: identify::Behaviour,
    /// NAT traversal
    pub autonat: autonat::Behaviour,
    /// `UPnP` port mapping
    pub upnp: Toggle<upnp::tokio::Behaviour>,
    /// Circuit relay for NAT traversal
    pub relay: Toggle<relay::Behaviour>,
}

/// Event generated by the `DiscoveryBehaviour`.
#[derive(Debug)]
pub enum DiscoveryEvent {
    /// Event that notifies that we connected to the node with the given peer
    /// id.
    PeerConnected(PeerId),

    /// Event that notifies that we disconnected with the node with the given
    /// peer id.
    PeerDisconnected(PeerId),

    /// Discovery event
    Discovery(Box<DerivedDiscoveryBehaviourEvent>),
}

pub struct DiscoveryBehaviour<K: KeyType> {
    /// Discovery behaviour
    pub discovery: DerivedDiscoveryBehaviour,
    /// Stream that fires when we need to perform the next random Kademlia
    /// query.
    pub next_kad_random_query: Interval,
    /// After `next_kad_random_query` triggers, the next one triggers after this
    /// duration.
    pub duration_to_next_kad: Duration,
    /// Events to return in priority when polled.
    pub pending_events: VecDeque<DiscoveryEvent>,
    /// Number of nodes we're currently connected to.
    pub n_node_connected: u32,
    /// Peers
    pub peers: HashSet<PeerId>,
    /// Peer info
    pub peer_info: HashMap<PeerId, PeerInfo>,
    /// Target peer count
    pub target_peer_count: u32,
    /// Options to configure dials to known peers.
    pub pending_dial_opts: VecDeque<DialOpts>,
    /// Phantom key type
    pub _marker: gadget_std::marker::PhantomData<K>,
}

impl<K: KeyType> DiscoveryBehaviour<K> {
    /// Bootstrap Kademlia network
    ///
    /// # Errors
    ///
    /// * If Kademlia is not activated
    pub fn bootstrap(&mut self) -> Result<kad::QueryId, Error> {
        if let Some(active_kad) = self.discovery.kademlia.as_mut() {
            active_kad.bootstrap().map_err(Into::into)
        } else {
            Err(Error::KademliaNotActivated)
        }
    }

    #[must_use]
    pub fn get_peers(&self) -> &HashSet<PeerId> {
        &self.peers
    }

    #[must_use]
    pub fn get_peer_info(&self, peer_id: &PeerId) -> Option<&PeerInfo> {
        self.peer_info.get(peer_id)
    }

    #[must_use]
    pub fn nat_status(&self) -> autonat::NatStatus {
        self.discovery.autonat.nat_status()
    }

    #[must_use]
    pub fn get_peer_addresses(&self) -> HashMap<PeerId, HashSet<Multiaddr>> {
        self.peer_info
            .iter()
            .map(|(peer_id, info)| (*peer_id, info.addresses.clone()))
            .collect()
    }
}

impl<K: KeyType> NetworkBehaviour for DiscoveryBehaviour<K> {
    type ConnectionHandler = <DerivedDiscoveryBehaviour as NetworkBehaviour>::ConnectionHandler;
    type ToSwarm = DiscoveryEvent;

    fn handle_established_inbound_connection(
        &mut self,
        connection_id: ConnectionId,
        peer: PeerId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        debug!(%peer, "Handling inbound connection");
        self.discovery.handle_established_inbound_connection(
            connection_id,
            peer,
            local_addr,
            remote_addr,
        )
    }

    fn handle_established_outbound_connection(
        &mut self,
        connection_id: ConnectionId,
        peer: PeerId,
        addr: &libp2p::Multiaddr,
        role_override: libp2p::core::Endpoint,
        port_use: PortUse,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        debug!(%peer, "Handling outbound connection");
        self.peer_info
            .entry(peer)
            .or_insert_with(|| {
                debug!(%peer, "Creating new peer info for outbound connection");
                PeerInfo {
                    addresses: HashSet::new(),
                    identify_info: None,
                    last_seen: std::time::SystemTime::now(),
                    ping_latency: None,
                    successes: 0,
                    failures: 0,
                    average_response_time: None,
                }
            })
            .addresses
            .insert(addr.clone());
        self.discovery.handle_established_outbound_connection(
            connection_id,
            peer,
            addr,
            role_override,
            port_use,
        )
    }

    fn on_swarm_event(&mut self, event: FromSwarm<'_>) {
        match &event {
            FromSwarm::ConnectionEstablished(e) => {
                if e.other_established == 0 {
                    debug!(%e.peer_id, "First connection established with peer");
                    self.n_node_connected += 1;
                    self.peers.insert(e.peer_id);
                    self.pending_events
                        .push_back(DiscoveryEvent::PeerConnected(e.peer_id));
                }
            }
            FromSwarm::ConnectionClosed(e) => {
                if e.remaining_established == 0 {
                    debug!(%e.peer_id, "Last connection closed with peer");
                    self.n_node_connected -= 1;
                    self.peers.remove(&e.peer_id);
                    self.peer_info.remove(&e.peer_id);
                    self.pending_events
                        .push_back(DiscoveryEvent::PeerDisconnected(e.peer_id));
                }
            }
            _ => {}
        }
        self.discovery.on_swarm_event(event);
    }

    fn on_connection_handler_event(
        &mut self,
        peer_id: PeerId,
        connection: ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
        self.discovery
            .on_connection_handler_event(peer_id, connection, event);
    }

    #[allow(clippy::type_complexity, clippy::too_many_lines)]
    fn poll(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<ToSwarm<Self::ToSwarm, libp2p::swarm::THandlerInEvent<Self>>> {
        // Immediately process the content of `discovered`.
        if let Some(ev) = self.pending_events.pop_front() {
            return Poll::Ready(ToSwarm::GenerateEvent(ev));
        }

        // Dial to peers
        if let Some(opts) = self.pending_dial_opts.pop_front() {
            return Poll::Ready(ToSwarm::Dial { opts });
        }

        // Poll the stream that fires when we need to start a random Kademlia query.
        while self.next_kad_random_query.poll_tick(cx).is_ready() {
            if self.n_node_connected < self.target_peer_count {
                // We still have not hit the discovery max, send random request for peers.
                let random_peer_id = PeerId::random();
                debug!(
                    "Libp2p <= Starting random Kademlia request for {:?}",
                    random_peer_id
                );
                if let Some(kademlia) = self.discovery.kademlia.as_mut() {
                    kademlia.get_closest_peers(random_peer_id);
                }
            }

            // Schedule the next random query with exponentially increasing delay,
            // capped at 60 seconds.
            self.next_kad_random_query = tokio::time::interval(self.duration_to_next_kad);
            // we need to reset the interval, otherwise the next tick completes immediately.
            self.next_kad_random_query.reset();

            self.duration_to_next_kad =
                cmp::min(self.duration_to_next_kad * 2, Duration::from_secs(60));
        }

        // Poll discovery events.
        while let Poll::Ready(ev) = self.discovery.poll(cx) {
            match ev {
                ToSwarm::GenerateEvent(ev) => {
                    match &ev {
                        DerivedDiscoveryBehaviourEvent::Identify(identify::Event::Received {
                            peer_id,
                            info,
                            connection_id,
                        }) => {
                            debug!(%peer_id, "Received identify event in discovery behaviour");
                            self.peer_info.entry(*peer_id).or_default().identify_info =
                                Some(info.clone());
                            if let Some(kademlia) = self.discovery.kademlia.as_mut() {
                                for address in &info.listen_addrs {
                                    kademlia.add_address(peer_id, address.clone());
                                }
                            }
                            self.pending_events
                                .push_back(DiscoveryEvent::Discovery(Box::new(
                                    DerivedDiscoveryBehaviourEvent::Identify(
                                        identify::Event::Received {
                                            peer_id: *peer_id,
                                            info: info.clone(),
                                            connection_id: *connection_id,
                                        },
                                    ),
                                )));
                        }
                        DerivedDiscoveryBehaviourEvent::Identify(identify::Event::Sent {
                            ..
                        }) => {
                            debug!("Identify event sent");
                        }
                        DerivedDiscoveryBehaviourEvent::Identify(identify::Event::Pushed {
                            ..
                        }) => {
                            debug!("Identify event pushed");
                        }
                        DerivedDiscoveryBehaviourEvent::Identify(identify::Event::Error {
                            ..
                        }) => {
                            debug!("Identify event error");
                        }

                        DerivedDiscoveryBehaviourEvent::Autonat(_) => {}
                        DerivedDiscoveryBehaviourEvent::Upnp(ev) => match ev {
                            upnp::Event::NewExternalAddr(addr) => {
                                info!("UPnP NewExternalAddr: {addr}");
                            }
                            upnp::Event::ExpiredExternalAddr(addr) => {
                                info!("UPnP ExpiredExternalAddr: {addr}");
                            }
                            upnp::Event::GatewayNotFound => {
                                info!("UPnP GatewayNotFound");
                            }
                            upnp::Event::NonRoutableGateway => {
                                info!("UPnP NonRoutableGateway");
                            }
                        },
                        DerivedDiscoveryBehaviourEvent::Kademlia(ev) => match ev {
                            // Adding to Kademlia buckets is automatic with our config,
                            // no need to do manually.
                            kad::Event::RoutingUpdated { .. }
                            | kad::Event::RoutablePeer { .. }
                            | kad::Event::PendingRoutablePeer { .. } => {
                                // Intentionally ignore
                            }
                            other => {
                                trace!("Libp2p => Unhandled Kademlia event: {:?}", other);
                            }
                        },
                        DerivedDiscoveryBehaviourEvent::Mdns(ev) => match ev {
                            MdnsEvent::Discovered(list) => {
                                if self.n_node_connected >= self.target_peer_count {
                                    // Already over discovery max, don't add discovered peers.
                                    // We could potentially buffer these addresses to be added later,
                                    // but mdns is not an important use case and may be removed in future.
                                    continue;
                                }

                                // Add any discovered peers to Kademlia
                                for (peer_id, multiaddr) in list {
                                    if let Some(kad) = self.discovery.kademlia.as_mut() {
                                        kad.add_address(peer_id, multiaddr.clone());
                                    }
                                }
                            }
                            MdnsEvent::Expired(_) => {}
                        },
                        DerivedDiscoveryBehaviourEvent::Relay(relay_event) => match relay_event {
                            relay::Event::ReservationReqAccepted { src_peer_id, .. } => {
                                debug!("Relay accepted reservation request from: {src_peer_id:#?}");
                            }
                            relay::Event::ReservationReqDenied { src_peer_id } => {
                                debug!("Reservation request was denied for: {src_peer_id:#?}");
                            }
                            relay::Event::ReservationTimedOut { src_peer_id } => {
                                debug!("Reservation timed out for: {src_peer_id:#?}");
                            }
                            _ => {}
                        },
                    }
                    self.pending_events
                        .push_back(DiscoveryEvent::Discovery(Box::new(ev)));
                }
                ToSwarm::Dial { opts } => {
                    return Poll::Ready(ToSwarm::Dial { opts });
                }
                ToSwarm::NotifyHandler {
                    peer_id,
                    handler,
                    event,
                } => {
                    return Poll::Ready(ToSwarm::NotifyHandler {
                        peer_id,
                        handler,
                        event,
                    });
                }
                ToSwarm::CloseConnection {
                    peer_id,
                    connection,
                } => {
                    return Poll::Ready(ToSwarm::CloseConnection {
                        peer_id,
                        connection,
                    });
                }
                ToSwarm::ListenOn { opts } => return Poll::Ready(ToSwarm::ListenOn { opts }),
                ToSwarm::RemoveListener { id } => {
                    return Poll::Ready(ToSwarm::RemoveListener { id });
                }
                ToSwarm::NewExternalAddrCandidate(addr) => {
                    return Poll::Ready(ToSwarm::NewExternalAddrCandidate(addr));
                }
                ToSwarm::ExternalAddrConfirmed(addr) => {
                    return Poll::Ready(ToSwarm::ExternalAddrConfirmed(addr));
                }
                ToSwarm::ExternalAddrExpired(addr) => {
                    return Poll::Ready(ToSwarm::ExternalAddrExpired(addr));
                }
                _ => {}
            }
        }

        Poll::Pending
    }
}
