use crate::error::{Error, Result};
use futures::channel::mpsc::UnboundedReceiver;
use futures::stream::TryStreamExt;
use nix::libc;
use rtnetlink::packet_core::{ErrorMessage, NetlinkMessage, NetlinkPayload};
use rtnetlink::packet_route::link::LinkFlags;
use rtnetlink::packet_route::{
    RouteNetlinkMessage,
    address::{AddressAttribute, AddressMessage},
};
use rtnetlink::sys::SocketAddr;
use rtnetlink::{Handle, new_connection};
use std::collections::HashSet;
use std::io;
use std::net::Ipv4Addr;
use std::sync::{Arc, Weak};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

pub struct Lease {
    addr: Ipv4Addr,
    pool: Weak<RwLock<Inner>>,
}

impl Lease {
    pub fn addr(&self) -> Ipv4Addr {
        self.addr
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        if let Some(pool) = self.pool.upgrade() {
            let mut guard = pool.blocking_write();
            guard.unavailable.remove(&self.addr);
        }
    }
}

struct Inner {
    _conn_task_handle: JoinHandle<()>,
    conn_handle: Handle,
    candidates: Vec<Ipv4Addr>,
    unavailable: HashSet<Ipv4Addr>,
}

#[derive(Clone)]
pub struct NetworkManager {
    inner: Arc<RwLock<Inner>>,
}

impl NetworkManager {
    pub async fn new(candidates: Vec<Ipv4Addr>) -> Result<Self> {
        let (conn, handle, msgs) = new_connection()?;
        let conn_task_handle = tokio::spawn(conn);

        let unavailable = Self::initial_snapshot().await?;
        let inner = Arc::new(RwLock::new(Inner {
            _conn_task_handle: conn_task_handle,
            conn_handle: handle,
            candidates,
            unavailable,
        }));
        spawn_watcher(Arc::downgrade(&inner), msgs);
        Ok(Self { inner })
    }

    /// Lease the first free address. `None` ⇒ pool exhausted.
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

    async fn conn_handle(&self) -> Handle {
        self.inner.read().await.conn_handle.clone()
    }

    /// Create a TAP interface for a VM
    ///
    /// This creates the interface,
    pub async fn new_tap_interface(&self, vm_id: u32) -> Result<(Lease, String)> {
        async fn configure_tap(handle: &Handle, dev: &str, ip: Ipv4Addr) -> Result<()> {
            let mut stream = handle.link().get().match_name(dev.to_string()).execute();

            let Some(link) = stream.try_next().await? else {
                return Err(io::Error::new(io::ErrorKind::Other, "link not found").into());
            };

            let link_index = link.header.index;

            let mut up_msg = link.clone();
            up_msg.header.change_mask = LinkFlags::Up;
            up_msg.header.flags |= LinkFlags::Up;
            handle.link().set(up_msg).execute().await?;

            handle
                .address()
                .add(link_index, ip.into(), 32)
                .execute()
                .await?;

            Ok(())
        }

        let interface = super::create_tap_interface(vm_id)?;

        loop {
            let Some(lease) = self.allocate().await else {
                return Err(
                    io::Error::new(io::ErrorKind::QuotaExceeded, "IP pool exhausted").into(),
                );
            };
            let ip = lease.addr();

            match configure_tap(&self.conn_handle().await, &interface, ip).await {
                Ok(()) => {
                    return Ok((lease, interface));
                }
                Err(Error::Net(rtnetlink::Error::NetlinkError(ErrorMessage {
                    code: Some(code),
                    ..
                }))) if code.get() == libc::EEXIST => {
                    tracing::warn!("collision on {ip}, retrying");
                }
                Err(e) => return Err(e),
            }
        }
    }

    async fn initial_snapshot() -> Result<HashSet<Ipv4Addr>> {
        let (conn, handle, _msgs) = new_connection()?;
        tokio::spawn(conn);

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
