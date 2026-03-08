# blockchain

## Purpose
On-chain event monitoring for the pricing engine. Polls EVM logs for service lifecycle events (activation, termination) from the Tangle contract and forwards them to the pricing service via a channel.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations for `event` and `evm_listener`.
- `event.rs` - `BlockchainEvent` enum with two variants: `ServiceActivated { service_id, blueprint_id }` and `ServiceTerminated { service_id }`.
- `evm_listener.rs` - EVM log poller implementation.
  - `EvmEventClient` async trait abstracting contract address, log fetching, and service lookup. Implemented for `Arc<TangleClient>`.
  - `EvmEventListener<C>` polls the Tangle contract at a configurable interval (default 5s), decodes `ServiceActivated` and `ServiceTerminated` events from Alloy logs, enriches activation events with `get_service()` to resolve `blueprint_id`, and sends `BlockchainEvent`s over an mpsc channel. Tracks `last_block` atomically for cursor-based pagination.

## Key APIs (no snippets)
- **Traits**: `EvmEventClient` (contract_address, get_logs, get_service)
- **Types**: `BlockchainEvent`, `EvmEventListener<C>`
- **Functions**: `EvmEventListener::new()`, `.run()` (infinite poll loop), `.poll_once()`, `.decode_event()`

## Relationships
- **Depends on**: `blueprint-client-tangle` (`TangleClient`, `ITangle` contract bindings, `ITangleTypes::Service`), `alloy-primitives`, `alloy-rpc-types`, `alloy-sol-types`
- **Used by**: Pricing engine service layer; events drive pricing state updates for service activation/termination
