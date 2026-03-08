# net

## Purpose
Network infrastructure for VM sandbox runtime environments. Manages IP address allocation via a thread-safe pool (`NetworkManager`), kernel-level firewall rules via nftables for VM network isolation and NAT, and TAP interface readiness polling. Gated by the `vm-sandbox` feature.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - IP pool allocation via `NetworkManager` with Netlink monitoring and TAP interface readiness.
  - **Key items**: `NetworkManager`, `Lease` (RAII guard returning IP on drop), `wait_for_interface()`, `initial_snapshot()`, `spawn_watcher()`
  - **Interactions**: Uses `rtnetlink` to monitor OS address changes; `Lease` holds `Weak<RwLock<Inner>>` for independent drop; polls `/sys/class/net/` for interface existence (100ms intervals, 5s timeout)
- `nftables.rs` - Firewall rule orchestration using nftables batch API for VM networking.
  - **Key items**: `setup_rules()`, `cleanup_firewall()`, `remove_rules()`, `check_net_admin_capability()`, `TANGLE_ROUTER_TABLE`, `FORWARD_CHAIN`, `NAT_CHAIN`
  - **Interactions**: Raises/lowers `CAP_NET_ADMIN` around all operations; creates per-VM forwarding + masquerade rules identified by comment strings containing TAP interface name

## Key APIs (no snippets)
- **Types**: `NetworkManager` (thread-safe IPv4 pool, `Arc<RwLock<Inner>>`), `Lease` (RAII IP lease with `addr()` accessor)
- **Functions**: `NetworkManager::new(candidates)`, `NetworkManager::allocate()`, `wait_for_interface(iface_name)`, `setup_rules(host_iface, tap_iface, vm_ip)`, `remove_rules(tap_iface)`, `cleanup_firewall(host_iface)`, `check_net_admin_capability()`

## Relationships
- **Depends on**: `rtnetlink` (Netlink connection), `nftables` crate (batch API), `caps` (Linux capabilities)
- **Used by**: `BlueprintManagerContext::new()` creates `NetworkManager` and verifies CAP_NET_ADMIN; `HypervisorInstance` calls `allocate()` during prepare, `wait_for_interface()`/`setup_rules()` during boot, `remove_rules()` during shutdown
- **Data/control flow**:
  - Context init creates NetworkManager with IP candidates and spawns Netlink watcher
  - VM prepare allocates IP via `Lease`; VM boot waits for TAP then configures nftables (forwarding + NAT)
  - VM shutdown removes per-VM rules; Lease drop returns IP to pool
  - nftables rules: established/related traffic accepted, new from VM subnet forwarded, masquerade outgoing

## Notes
