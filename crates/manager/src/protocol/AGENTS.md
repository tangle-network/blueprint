# protocol

## Purpose
Protocol abstraction layer that provides a unified interface for the blueprint manager to operate across different blockchain protocols (Tangle and EigenLayer). Routes initialization, event polling, and event handling to protocol-specific implementations via enum dispatch.

## Contents (one hop)
### Subdirectories
- [x] `eigenlayer/` - EigenLayer AVS protocol client and event handler. Connects to EVM chains, polls for new task/response events, and manages service lifecycle for EigenLayer blueprints.
- [x] `tangle/` - Tangle Network protocol client and event handler. Connects to Tangle via RPC, polls for block events (service initiated/terminated, job calls), parses on-chain blueprint metadata into source fetchers, and manages service lifecycle.

### Files
- `mod.rs` - `ProtocolManager` enum with `Tangle` and `Eigenlayer` variants, each holding a protocol client and event handler. Methods: `new()` (async constructor routing on `ProtocolType`), `initialize()`, `next_event()`, `handle_event()`, and `run()` (init + event loop). Tests verify `ProtocolType` conversion from `ProtocolSettings`.
- `types.rs` - `ProtocolType` enum (Tangle, Eigenlayer) with conversions from `Protocol` and `ProtocolSettings`. `ProtocolEvent` enum wrapping `TangleProtocolEvent` (block_number, block_hash, timestamp, logs, inner `TangleEvent`) and `EigenlayerProtocolEvent` (block_number, block_hash, logs). Accessor methods `as_tangle()`, `as_eigenlayer()`, `block_number()`.

## Key APIs (no snippets)
- `ProtocolManager` - enum-dispatched manager with `new()`, `initialize()`, `next_event()`, `handle_event()`, `run()`
- `ProtocolType` - protocol discriminant (Tangle or Eigenlayer)
- `ProtocolEvent` - unified event type carrying protocol-specific block/log data
- `TangleProtocolEvent` / `EigenlayerProtocolEvent` - protocol-specific event data

## Relationships
- Created and driven by `executor/mod.rs` in `run_blueprint_manager_with_keystore()`
- Each variant delegates to its subdirectory's client (event source) and handler (event processing)
- Event handlers use `sources/` to fetch and spawn blueprints, `rt/service.rs` to manage services, and `blueprint/ActiveBlueprints` as the service registry
- `ProtocolType` derived from `BlueprintEnvironment::protocol_settings`
