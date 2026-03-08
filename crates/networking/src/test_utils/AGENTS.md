# test_utils

## Purpose
Provides test infrastructure for the networking crate, including test node construction, startup with port verification, and helpers for waiting on peer discovery, handshake completion, and peer info population. Used by all integration tests in the `tests/` module.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Contains `setup_log` for tracing initialization, `TestNode<K>` struct wrapping a `NetworkService` with auto-generated or specified keys, and helpers: `wait_for_condition` (generic polling), `wait_for_peer_discovery` (all-pairs discovery), `wait_for_peer_info` (bidirectional identify info), `wait_for_handshake_completion` (two-node handshake), `wait_for_all_handshakes` (N-node all-pairs with exponential backoff and detailed diagnostics), `create_node_with_keys`, and `create_whitelisted_nodes` (generates N nodes with mutually whitelisted keys).

## Key APIs (no snippets)
- `TestNode::new` / `TestNode::new_with_keys` - Create test nodes with auto or specified keypairs, binding to `0.0.0.0:0`.
- `TestNode::start` - Starts the network service, waits for listen address and port readiness (10s timeout).
- `create_whitelisted_nodes` - Creates N nodes where each node's instance public key is whitelisted by all others.
- `wait_for_all_handshakes` - Waits for all pairs of nodes to complete handshake verification with exponential backoff (50ms-500ms), progress logging, and detailed timeout diagnostics.
- `wait_for_peer_discovery` - Polls until all handles see all other peers.

## Relationships
- Uses `crate::NetworkConfig`, `crate::NetworkService`, `crate::service::AllowedKeys`, `crate::service_handle::NetworkServiceHandle`.
- Used by `crate::tests::*` (blueprint_protocol, discovery, gossip, handshake test modules).
