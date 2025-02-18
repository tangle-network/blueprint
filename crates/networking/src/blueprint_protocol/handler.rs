use std::time::{Duration, Instant};

use gadget_crypto::tangle_pair_signer::sp_core::offchain::Duration as GadgetDuration;
use libp2p::{gossipsub, request_response, PeerId};
use tracing::{debug, warn};

use crate::key_types::InstanceMsgPublicKey;

use super::{BlueprintProtocolBehaviour, InstanceMessageRequest, InstanceMessageResponse};

const INBOUND_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(30);
const OUTBOUND_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(30);

impl BlueprintProtocolBehaviour {
    pub fn handle_request_response_event(
        &mut self,
        event: request_response::Event<InstanceMessageRequest, InstanceMessageResponse>,
    ) {
        match event {
            request_response::Event::Message {
                peer,
                message:
                    request_response::Message::Request {
                        request:
                            InstanceMessageRequest::Handshake {
                                public_key,
                                signature,
                            },
                        channel,
                        ..
                    },
                ..
            } => {
                debug!(%peer, "Received handshake request");

                // Check if we already have a pending outbound handshake
                if let Some(outbound_time) = self.outbound_handshakes.get(&peer) {
                    // If we have an outbound handshake and their peer_id is less than ours,
                    // we should wait for their response instead of responding to their request
                    // TODO: Fix
                    // if peer < &self.local_peer_id {
                    //     debug!(%peer, "Deferring inbound handshake - waiting for outbound response");
                    //     return;
                    // }
                    // If we have an outbound handshake and their peer_id is greater than ours,
                    // we should handle their request and cancel our outbound attempt
                    self.outbound_handshakes.remove(&peer);
                }

                // Verify the handshake
                match self.verify_handshake(&peer, &public_key, &signature) {
                    Ok(()) => {
                        // Store the handshake request
                        self.inbound_handshakes.insert(peer, Instant::now());

                        // Send handshake response
                        let response = InstanceMessageResponse::Handshake {
                            public_key: self.instance_public_key.clone(),
                            signature: self.sign_handshake(&peer),
                        };

                        if let Err(e) = self.send_response(channel, response) {
                            warn!(%peer, "Failed to send handshake response: {:?}", e);
                            self.handle_handshake_failure(&peer, "Failed to send response");
                            return;
                        }

                        // Mark handshake as completed
                        self.complete_handshake(&peer, &public_key);
                    }
                    Err(e) => {
                        warn!(%peer, "Invalid handshake request: {:?}", e);
                        let response = InstanceMessageResponse::Error {
                            code: 400,
                            message: format!("Invalid handshake: {:?}", e),
                        };
                        if let Err(e) = self.send_response(channel, response) {
                            warn!(%peer, "Failed to send error response: {:?}", e);
                        }
                        self.handle_handshake_failure(&peer, "Invalid handshake");
                    }
                }
            }
            request_response::Event::Message {
                peer,
                message:
                    request_response::Message::Response {
                        response:
                            InstanceMessageResponse::Handshake {
                                public_key,
                                signature,
                            },
                        ..
                    },
                ..
            } => {
                debug!(%peer, "Received handshake response");

                // Verify we have a pending outbound handshake
                if !self.outbound_handshakes.contains_key(&peer) {
                    warn!(%peer, "Received unexpected handshake response");
                    return;
                }

                // Verify the handshake
                match self.verify_handshake(&peer, &public_key, &signature) {
                    Ok(()) => {
                        // Mark handshake as completed
                        self.complete_handshake(&peer, &public_key);
                    }
                    Err(e) => {
                        warn!(%peer, "Invalid handshake response: {:?}", e);
                        self.handle_handshake_failure(&peer, "Invalid handshake response");
                    }
                }

                // Remove the outbound handshake
                self.outbound_handshakes.remove(&peer);
            }
            request_response::Event::Message {
                peer,
                message:
                    request_response::Message::Response {
                        response: InstanceMessageResponse::Error { code, message },
                        ..
                    },
                ..
            } => {
                warn!(%peer, code, %message, "Received error response");
                self.handle_handshake_failure(&peer, &message);
            }
            request_response::Event::Message {
                peer,
                message:
                    request_response::Message::Request {
                        request:
                            InstanceMessageRequest::Protocol {
                                protocol,
                                payload,
                                metadata,
                            },
                        channel,
                        ..
                    },
                ..
            } => {
                // Only accept protocol messages from peers we've completed handshakes with
                if !self.peer_manager.is_peer_verified(&peer) {
                    warn!(%peer, "Received protocol message from unverified peer");
                    let response = InstanceMessageResponse::Error {
                        code: 403,
                        message: "Handshake required".to_string(),
                    };
                    if let Err(e) = self.send_response(channel, response) {
                        warn!(%peer, "Failed to send error response: {:?}", e);
                    }
                    return;
                }

                debug!(%peer, %protocol, "Received protocol request");
                // Handle protocol message...
            }
            _ => {}
        }

        // Check for expired handshakes
        self.check_expired_handshakes();
    }

    /// Check for and remove expired handshakes
    fn check_expired_handshakes(&mut self) {
        let now = Instant::now();

        // Check inbound handshakes
        let expired_inbound: Vec<_> = self
            .inbound_handshakes
            .clone()
            .into_read_only()
            .iter()
            .filter(|(_, &time)| now.duration_since(time) > INBOUND_HANDSHAKE_TIMEOUT)
            .map(|(peer_id, _)| *peer_id)
            .collect();

        for peer_id in expired_inbound {
            self.inbound_handshakes.remove(&peer_id);
            self.handle_handshake_failure(&peer_id, "Inbound handshake timeout");
        }

        // Check outbound handshakes
        let expired_outbound: Vec<_> = self
            .outbound_handshakes
            .clone()
            .into_read_only()
            .iter()
            .filter(|(_, &time)| now.duration_since(time) > OUTBOUND_HANDSHAKE_TIMEOUT)
            .map(|(peer_id, _)| *peer_id)
            .collect();

        for peer_id in expired_outbound {
            self.outbound_handshakes.remove(&peer_id);
            self.handle_handshake_failure(&peer_id, "Outbound handshake timeout");
        }
    }

    /// Complete a successful handshake
    fn complete_handshake(&mut self, peer: &PeerId, public_key: &InstanceMsgPublicKey) {
        debug!(%peer, "Completed handshake");

        // Remove from pending handshakes
        self.inbound_handshakes.remove(peer);
        self.outbound_handshakes.remove(peer);

        // Add to verified peers
        self.peer_manager.verify_peer(peer);

        // Update peer manager
        self.peer_manager
            .add_peer_id_to_public_key(peer, public_key);
    }

    pub fn handle_gossipsub_event(&mut self, event: gossipsub::Event) {
        match event {
            gossipsub::Event::Message {
                propagation_source,
                message_id,
                message,
            } => {
                // Only accept gossip from verified peers
                if !self.peer_manager.is_peer_verified(&propagation_source) {
                    warn!(%propagation_source, "Received gossip from unverified peer");
                    return;
                }

                debug!(%propagation_source, "Received gossip message");
            }
            gossipsub::Event::Subscribed { peer_id, topic } => {
                debug!(%peer_id, %topic, "Peer subscribed to topic");
            }
            gossipsub::Event::Unsubscribed { peer_id, topic } => {
                debug!(%peer_id, %topic, "Peer unsubscribed from topic");
            }
            _ => {}
        }
    }
}
