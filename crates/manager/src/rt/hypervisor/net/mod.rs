pub mod nftables;

use crate::config::BlueprintManagerConfig;
use crate::error::{Error, Result};
use blueprint_core::debug;
use futures::channel::mpsc::UnboundedReceiver;
use futures::stream::TryStreamExt;
use rtnetlink::packet_core::{NetlinkMessage, NetlinkPayload};
use rtnetlink::packet_route::{
    RouteNetlinkMessage,
    address::{AddressAttribute, AddressMessage},
};
use rtnetlink::sys::SocketAddr;
use rtnetlink::{Handle, new_connection};
use std::collections::HashSet;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::sync::{Arc, Weak};
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

pub struct Lease {
    addr: Ipv4Addr,
    pool: Weak<RwLock<Inner>>,
}

impl Lease {
    #[must_use]
    pub fn addr(&self) -> Ipv4Addr {
        self.addr
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        if let Some(pool) = self.pool.upgrade() {
            tokio::task::block_in_place(move || {
                let mut guard = pool.blocking_write();
                guard.unavailable.remove(&self.addr);
            });
        }
    }
}

struct Inner {
    _conn_task_handle: JoinHandle<()>,
    _conn_handle: Handle,
    candidates: Vec<Ipv4Addr>,
    unavailable: HashSet<Ipv4Addr>,
}

#[derive(Clone)]
pub struct NetworkManager {
    inner: Arc<RwLock<Inner>>,
}

impl NetworkManager {
    /// Create a new `NetworkManager`
    ///
    /// This will take a snapshot of all currently occupied addresses.
    ///
    /// # Errors
    ///
    /// * Unable to start a `NetLink` connection
    /// * Unable to check used addresses
    pub async fn new(candidates: Vec<Ipv4Addr>) -> Result<Self> {
        let (conn, handle, msgs) = new_connection()?;
        let conn_task_handle = tokio::spawn(conn);

        let unavailable = Self::initial_snapshot(handle.clone()).await?;
        let inner = Arc::new(RwLock::new(Inner {
            _conn_task_handle: conn_task_handle,
            _conn_handle: handle,
            candidates,
            unavailable,
        }));
        spawn_watcher(Arc::downgrade(&inner), msgs);
        Ok(Self { inner })
    }

    /// Lease the first free address
    pub async fn allocate(&self) -> Option<Lease> {
        let mut guard = self.inner.write().await;

        for &addr in &guard.candidates {
            if !guard.unavailable.contains(&addr) {
                guard.unavailable.insert(addr);
                return Some(Lease {
                    addr,
                    pool: Arc::downgrade(&self.inner),
                });
            }
        }
        None
    }

    async fn initial_snapshot(handle: Handle) -> Result<HashSet<Ipv4Addr>> {
        let mut set = HashSet::new();
        let mut addrs = handle.address().get().execute();
        while let Some(msg) = addrs.try_next().await? {
            if let Some(addr) = addr_from_msg(&msg) {
                set.insert(addr);
            }
        }
        Ok(set)
    }
}

fn addr_from_msg(msg: &AddressMessage) -> Option<Ipv4Addr> {
    msg.attributes.iter().find_map(|attr| match attr {
        AddressAttribute::Address(std::net::IpAddr::V4(a)) => Some(*a),
        _ => None,
    })
}

fn spawn_watcher(
    pool: Weak<RwLock<Inner>>,
    mut msgs: UnboundedReceiver<(NetlinkMessage<RouteNetlinkMessage>, SocketAddr)>,
) {
    tokio::spawn(async move {
        while let Ok(Some((msg, _))) = msgs.try_next() {
            let Some(inner) = pool.upgrade() else { break };
            let mut guard = inner.write().await;

            match msg.payload {
                NetlinkPayload::InnerMessage(RouteNetlinkMessage::NewAddress(ref a)) => {
                    if let Some(addr) = addr_from_msg(a) {
                        guard.unavailable.insert(addr);
                    }
                }
                NetlinkPayload::InnerMessage(RouteNetlinkMessage::DelAddress(ref a)) => {
                    if let Some(addr) = addr_from_msg(a) {
                        guard.unavailable.remove(&addr);
                    }
                }
                _ => {}
            }
        }
    });
}

pub(super) async fn wait_for_interface(iface_name: &str) -> Result<()> {
    let iface_path = PathBuf::from(format!("/sys/class/net/{iface_name}"));

    debug!("Waiting for interface `{iface_name}` to appear...");

    let res = tokio::time::timeout(Duration::from_secs(5), async move {
        loop {
            if iface_path.exists() {
                return Ok(());
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await;

    let Ok(res) = res else {
        return Err(Error::Other(format!(
            "Interface `{iface_name}` never appeared"
        )));
    };

    res
}

pub(crate) async fn init_manager_config(
    manager_config: &mut BlueprintManagerConfig,
) -> Result<(NetworkManager, String)> {
    nftables::check_net_admin_capability()?;
    manager_config.verify_network_interface()?;

    let network_interface = manager_config
        .network_interface
        .clone()
        .expect("interface set by verify_network_interface");

    let network_candidates = manager_config
        .default_address_pool
        .hosts()
        .filter(|ip| ip.octets()[3] != 0 && ip.octets()[3] != 255)
        .collect();

    let manager = NetworkManager::new(network_candidates).await?;

    Ok((manager, network_interface))
}
