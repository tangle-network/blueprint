// This file is part of Tangle.
// Copyright (C) 2022-2023 Webb Technologies Inc.
//
// Tangle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Tangle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Tangle.  If not, see <http://www.gnu.org/licenses/>.

use crate::PerTestNodeInput;
use async_trait::async_trait;
use environment_utils::transaction_manager::tangle::SubxtPalletSubmitter;
use gadget_common::config::Network;
use gadget_common::locks::TokioMutexExt;
use gadget_common::prelude::{
    DebugLogger, ECDSAKeyStore, GadgetEnvironment, InMemoryBackend, NodeInput, PairSigner,
    PrometheusConfig, UnboundedReceiver, UnboundedSender,
};
use gadget_common::tangle_subxt::subxt::ext::subxt_core::utils;
use gadget_common::tangle_subxt::subxt::{OnlineClient, SubstrateConfig};
use gadget_common::Error;
use gadget_core::job_manager::{ProtocolMessageMetadata, SendFuture, WorkManagerInterface};
use libp2p::Multiaddr;
use sp_application_crypto::ecdsa;
use sp_core::{sr25519, Pair};
use std::collections::HashMap;
use std::net::IpAddr;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tangle_environment::runtime::{TangleConfig, TangleRuntime};
use tangle_environment::TangleEnvironment;
use tangle_primitives::AccountId;
use url::Url;

pub fn id_to_ecdsa_pair(id: u8) -> ecdsa::Pair {
    ecdsa::Pair::from_string(&format!("//Alice///{id}"), None).expect("static values are valid")
}

pub fn id_to_sr25519_pair(id: u8) -> sr25519::Pair {
    sr25519::Pair::from_string(&format!("//Alice///{id}"), None).expect("static values are valid")
}

pub fn id_to_public(id: u8) -> ecdsa::Public {
    id_to_ecdsa_pair(id).public()
}

pub fn id_to_sr25519_public(id: u8) -> sr25519::Public {
    id_to_sr25519_pair(id).public()
}

type PeersRx<Env> =
    Arc<HashMap<ecdsa::Public, gadget_io::tokio::sync::Mutex<UnboundedReceiver<Env>>>>;

pub struct MockNetwork<Env: GadgetEnvironment> {
    peers_tx: Arc<
        HashMap<
            ecdsa::Public,
            UnboundedSender<<Env::WorkManager as WorkManagerInterface>::ProtocolMessage>,
        >,
    >,
    peers_rx: PeersRx<<Env::WorkManager as WorkManagerInterface>::ProtocolMessage>,
    my_id: ecdsa::Public,
}

impl<Env: GadgetEnvironment> Clone for MockNetwork<Env> {
    fn clone(&self) -> Self {
        Self {
            peers_tx: self.peers_tx.clone(),
            peers_rx: self.peers_rx.clone(),
            my_id: self.my_id,
        }
    }
}

impl<Env: GadgetEnvironment> MockNetwork<Env> {
    pub fn setup(ids: &Vec<ecdsa::Public>) -> Vec<Self> {
        let mut peers_tx = HashMap::new();
        let mut peers_rx = HashMap::new();
        let mut networks = Vec::new();

        for id in ids {
            let (tx, rx) = gadget_io::tokio::sync::mpsc::unbounded_channel();
            peers_tx.insert(*id, tx);
            peers_rx.insert(*id, gadget_io::tokio::sync::Mutex::new(rx));
        }

        let peers_tx = Arc::new(peers_tx);
        let peers_rx = Arc::new(peers_rx);

        for id in ids {
            let network = Self {
                peers_tx: peers_tx.clone(),
                peers_rx: peers_rx.clone(),
                my_id: *id,
            };
            networks.push(network);
        }

        networks
    }
}

#[async_trait]
impl<Env: GadgetEnvironment> Network<Env> for MockNetwork<Env>
where
    Env::ProtocolMessage: Clone,
{
    async fn next_message(
        &self,
    ) -> Option<<Env::WorkManager as WorkManagerInterface>::ProtocolMessage> {
        self.peers_rx
            .get(&self.my_id)?
            .lock_timeout(Duration::from_millis(500))
            .await
            .recv()
            .await
    }

    async fn send_message(
        &self,
        message: <Env::WorkManager as WorkManagerInterface>::ProtocolMessage,
    ) -> Result<(), Error> {
        let _check_message_has_ids = message.sender_network_id().ok_or(Error::MissingNetworkId)?;
        if let Some(peer_id) = message.recipient_network_id() {
            let tx = self
                .peers_tx
                .get(&peer_id)
                .ok_or(Error::PeerNotFound { id: peer_id })?;
            tx.send(message).map_err(|err| Error::NetworkError {
                err: err.to_string(),
            })?;
        } else {
            // Broadcast to everyone except ourself
            for (peer_id, tx) in self.peers_tx.iter() {
                if peer_id != &self.my_id {
                    tx.send(message.clone())
                        .map_err(|err| Error::NetworkError {
                            err: err.to_string(),
                        })?;
                }
            }
        }
        Ok(())
    }
}

const LOCAL_BIND_ADDR: &str = "127.0.0.1";

/// N: number of nodes
/// K: Number of networks accessible per node (should be equal to the number of services in a given blueprint)
/// D: Any data that you want to pass to pass with NodeInput.
/// F: A function that generates a service's execution via a series of shells. Each shell executes a subset of the service,
/// as each service may have a set of operations that are executed in parallel, sequentially, or concurrently.
pub async fn new_test_ext_blueprint_manager<
    const N: usize,
    const K: usize,
    D: Send + Clone + 'static,
    F: Fn(PerTestNodeInput<D>) -> Fut,
    Fut: SendFuture<'static, ()>,
>(
    additional_params: D,
    f: F,
) -> LocalhostTestExt {
    const LOCAL_TANGLE_NODE: &str = "ws://127.0.0.1:9944";
    const NAME_IDS: [&str; 3] = ["alice", "bob", "charlie"];

    assert!(N < 4, "Only up to 3 nodes are supported");

    let bind_addrs = (0..N)
        .into_iter()
        .map(|_| find_open_tcp_bind_port())
        .map(|port| {
            (
                Multiaddr::from_str(&format!("/ip4/{LOCAL_BIND_ADDR}/tcp/{port}"))
                    .expect("Should parse MultiAddr"),
                port,
            )
        })
        .collect::<Vec<_>>();

    let multi_addrs = bind_addrs
        .iter()
        .map(|(addr, _)| addr.clone())
        .collect::<Vec<_>>();

    for (node_index, (my_addr, my_port)) in bind_addrs.iter().enumerate() {
        let my_alias = NAME_IDS[node_index];

        let test_input = PerTestNodeInput {
            instance_id: *node_index as _,
            bind_ip: IpAddr::try_from(LOCAL_BIND_ADDR).expect("Should be a valid IP"),
            bind_port: *my_port,
            bootnodes: multi_addrs
                .iter()
                .filter(|addr| *addr != my_addr)
                .cloned()
                .collect(),
            base_path: format!("../tangle/tmp/{my_alias}"),
            verbose: 4,
            pretty: false,
            extra_input: additional_params.clone(),
            local_tangle_node: Url::parse(LOCAL_TANGLE_NODE).expect("Should parse URL"),
        };

        let task = f(test_input);
        gadget_io::tokio::task::spawn(task);
    }

    // The local test node runs on ws://127.0.0.1:9944
    let client = OnlineClient::<SubstrateConfig>::from_url(LOCAL_TANGLE_NODE)
        .await
        .expect("Failed to create primary localhost client");
    let localhost_externalities = LocalhostTestExt::from(client);

    localhost_externalities
}

fn find_open_tcp_bind_port() -> u16 {
    let listener = std::net::TcpListener::bind(LOCAL_BIND_ADDR).expect("Should bind to localhost");
    listener
        .local_addr()
        .expect("Should have a local address")
        .port()
}

/// N: number of nodes
/// K: Number of networks accessible per node (should be equal to the number of services in a given blueprint)
/// D: Any data that you want to pass with NodeInput.
/// F: A function that generates a service's execution via a series of shells. Each shell executes a subset of the service,
/// as each service may have a set of operations that are executed in parallel, sequentially, or concurrently.
pub async fn new_test_ext<
    const N: usize,
    const K: usize,
    D: Send + Clone + 'static,
    F: Fn(NodeInput<TangleEnvironment, MockNetwork<TangleEnvironment>, InMemoryBackend, D>) -> Fut,
    Fut: SendFuture<'static, ()>,
>(
    additional_params: D,
    f: F,
) -> LocalhostTestExt {
    let role_pairs = (0..N)
        .map(|i| id_to_ecdsa_pair(i as u8))
        .collect::<Vec<_>>();
    let roles_identities = role_pairs
        .iter()
        .map(|pair| pair.public())
        .collect::<Vec<_>>();

    let pairs = (0..N)
        .map(|i| id_to_sr25519_pair(i as u8))
        .collect::<Vec<_>>();

    /*
    let account_ids = pairs
        .iter()
        .map(|pair| pair.public().into())
        .collect::<Vec<AccountId>>();


    let balances = account_ids
        .iter()
        .map(|public| (public.clone(), 100u128))
        .collect::<Vec<_>>();*/

    let networks = (0..K)
        .map(|_| MockNetwork::setup(&roles_identities))
        .collect::<Vec<_>>();

    // Transpose networks
    let networks = (0..N)
        .map(|i| {
            networks
                .iter()
                .map(|network| network[i].clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Each client connects to ws://127.0.0.1:9944. This client is for the test environment
    let client = OnlineClient::<SubstrateConfig>::new()
        .await
        .expect("Failed to create primary localhost client");
    let localhost_externalities = LocalhostTestExt::from(client);

    for (node_index, ((role_pair, pair), networks)) in
        role_pairs.into_iter().zip(pairs).zip(networks).enumerate()
    {
        let account_id: utils::AccountId32 = pair.public().0.into();
        let mut localhost_clients = Vec::new();

        for _ in 0..K {
            // Each client connects to ws://127.0.0.1:9944
            let client = OnlineClient::<SubstrateConfig>::new()
                .await
                .expect("Failed to create localhost client");

            let client = TangleRuntime::new(client, account_id.clone());
            localhost_clients.push(client);
        }

        let logger = DebugLogger {
            id: format!("Peer {node_index}"),
        };

        let pair_signer = PairSigner::<TangleConfig>::new(pair);

        let tx_manager = Arc::new(
            SubxtPalletSubmitter::new(pair_signer, logger.clone())
                .await
                .expect("Failed to create tx manager"),
        );

        let keystore = ECDSAKeyStore::in_memory(role_pair);
        let prometheus_config = PrometheusConfig::Disabled;

        let input = NodeInput {
            clients: localhost_clients,
            networks,
            account_id: sr25519::Public(account_id.0),
            logger,
            tx_manager: tx_manager as _,
            keystore,
            node_index,
            additional_params: additional_params.clone(),
            prometheus_config,
        };

        let task = f(input);
        gadget_io::tokio::task::spawn(task);
    }

    localhost_externalities
}

pub fn mock_pub_key(id: u8) -> AccountId {
    sr25519::Public::from_raw([id; 32]).into()
}

pub struct LocalhostTestExt {
    client: OnlineClient<SubstrateConfig>,
}

impl LocalhostTestExt {
    /// An identity function (For future reverse-compatible changes)
    pub fn execute_with<
        T: FnOnce(&OnlineClient<SubstrateConfig>) -> R + Send + 'static,
        R: Send + 'static,
    >(
        &self,
        function: T,
    ) -> R {
        function(&self.client)
    }

    /// An identity function (For future reverse-compatible changes)
    pub async fn execute_with_async<
        T: FnOnce(&OnlineClient<SubstrateConfig>) -> R + Send + 'static,
        R: Send + 'static,
    >(
        &self,
        function: T,
    ) -> R {
        function(&self.client)
    }
}

impl Deref for LocalhostTestExt {
    type Target = OnlineClient<SubstrateConfig>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for LocalhostTestExt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}

impl From<OnlineClient<SubstrateConfig>> for LocalhostTestExt {
    fn from(client: OnlineClient<SubstrateConfig>) -> Self {
        Self { client }
    }
}
