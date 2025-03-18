#![allow(dead_code)]
use gadget_crypto::KeyType;
use gadget_networking::{
    NetworkConfig, NetworkService, service::AllowedKeys, service_handle::NetworkServiceHandle,
};
use libp2p::{
    Multiaddr, PeerId,
    identity::{self, Keypair},
};
use std::string::ToString;
use std::time::Duration;
use tokio::time::timeout;
use tracing::info;

pub fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(true)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .try_init();
}
