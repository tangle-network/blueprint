# Protocol Abstraction Architecture

## Overview

This document describes the new protocol abstraction layer for the Blueprint Manager. This architecture enables the manager to support multiple blockchain protocols (Tangle, EigenLayer) with a clean, maintainable design.

## Design Principles

1. **Protocol-Agnostic Core**: The manager's core logic doesn't know about specific protocols
2. **Clear Separation**: Each protocol has its own module with client + event handler
3. **Type Safety**: Rust's type system prevents protocol mixing at compile time
4. **Reusability**: Common infrastructure is shared across protocols
5. **Extensibility**: Adding new protocols is straightforward

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Blueprint Manager                         │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │          Protocol Abstraction Layer                     │ │
│  │                                                          │ │
│  │  ┌─────────────────┐      ┌────────────────────────┐  │ │
│  │  │ ProtocolFactory │      │  Protocol Event Loop   │  │ │
│  │  └─────────────────┘      └────────────────────────┘  │ │
│  │           │                          │                  │ │
│  │           ▼                          ▼                  │ │
│  │  ┌────────────────────────┬────────────────────────┐  │ │
│  │  │   ProtocolClient       │  ProtocolEventHandler  │  │ │
│  │  │   (trait)              │  (trait)               │  │ │
│  │  └────────────────────────┴────────────────────────┘  │ │
│  └────────────────┬──────────────────┬────────────────────┘ │
│                   │                  │                       │
│      ┌────────────┴───────┬──────────┴──────────┐          │
│      ▼                    ▼                      ▼          │
│  ┌─────────────┐    ┌──────────────┐     ┌────────────┐   │
│  │   Tangle    │    │  EigenLayer  │     │   Future   │   │
│  │  Protocol   │    │   Protocol   │     │  Protocols │   │
│  │             │    │              │     │            │   │
│  │ ┌─────────┐ │    │ ┌──────────┐ │     │            │   │
│  │ │ Client  │ │    │ │  Client  │ │     │            │   │
│  │ └─────────┘ │    │ └──────────┘ │     │            │   │
│  │ ┌─────────┐ │    │ ┌──────────┐ │     │            │   │
│  │ │ Handler │ │    │ │  Handler │ │     │            │   │
│  │ └─────────┘ │    │ └──────────┘ │     │            │   │
│  └─────────────┘    └──────────────┘     └────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Module Structure

```
crates/manager/src/
├── protocol/
│   ├── mod.rs                 # Main protocol module, ProtocolFactory
│   ├── traits.rs              # Core traits: ProtocolClient, ProtocolEventHandler
│   ├── types.rs               # Common types: ProtocolType, ProtocolEvent
│   │
│   ├── tangle/                # Tangle protocol implementation
│   │   ├── mod.rs
│   │   ├── client.rs          # TangleProtocolClient
│   │   └── event_handler.rs   # TangleEventHandler
│   │
│   └── eigenlayer/            # EigenLayer protocol implementation
│       ├── mod.rs
│       ├── client.rs          # EigenlayerProtocolClient
│       └── event_handler.rs   # EigenlayerEventHandler
│
├── executor/
│   ├── mod.rs                 # Updated to use protocol abstraction
│   └── event_handler.rs       # Common service spawning logic (protocol-agnostic)
│
└── ...
```

## Core Traits

### `ProtocolClient`

Handles connection to a protocol and streams events.

```rust
pub trait ProtocolClient: Send + Sync {
    fn next_event(&mut self) -> Pin<Box<dyn Future<Output = Option<ProtocolEvent>> + Send + '_>>;
    fn protocol_type(&self) -> ProtocolType;
}
```

### `ProtocolEventHandler`

Processes events and manages blueprint lifecycle.

```rust
pub trait ProtocolEventHandler: Send + Sync {
    fn initialize(...) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
    fn handle_event(...) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
    fn protocol_type(&self) -> ProtocolType;
}
```

## Event Flow

1. **Initialization**:
   ```
   ProtocolFactory::create(protocol_type)
   → (ProtocolClient, ProtocolEventHandler)
   ```

2. **Setup**:
   ```
   handler.initialize(client, env, ctx, active_blueprints)
   → Query initial state (registered operators, active services)
   ```

3. **Event Loop**:
   ```
   loop {
       event = client.next_event()
       handler.handle_event(event, env, ctx, active_blueprints)
       → Start/stop services as needed
   }
   ```

## Protocol Implementations

### Tangle

- **Client**: Wraps `TangleClient`, streams finality notifications
- **Handler**: Processes Substrate events (ServiceInitiated, JobCalled, etc.)
- **Events**: Blueprint registration, service lifecycle, job execution

### EigenLayer

- **Client**: Wraps EVM provider, polls for contract events
- **Handler**: Processes EigenLayer AVS events (TaskCreated, TaskResponded, etc.)
- **Events**: Task creation, operator registration, response submission

## Migration Path

### From Current Code

**Old** (Tangle-only):
```rust
// In executor/mod.rs
let tangle_client = TangleClient::new(...).await?;
while let Some(event) = tangle_client.next_event().await {
    handle_tangle_event(&event, ...).await?;
}
```

**New** (Protocol-agnostic):
```rust
// In executor/mod.rs
let protocol_type = env.protocol.into();
let (client, handler) = ProtocolFactory::create(protocol_type, env, ctx).await?;

run_protocol_event_loop(client, handler, env, ctx, active_blueprints).await?;
```

### Benefits

1. **No code duplication**: Event loop is unified
2. **Type safe**: Can't mix Tangle/EigenLayer events
3. **Clear ownership**: Each protocol owns its logic
4. **Easy testing**: Mock the traits for unit tests
5. **Future-proof**: Adding protocols is just implementing the traits

## Implementation Status

### ✅ Completed
- [x] Protocol abstraction traits (with `as_any()` for downcasting)
- [x] Common types (ProtocolType, ProtocolEvent)
- [x] ProtocolFactory with unified event loop
- [x] Tangle protocol client
- [x] Tangle event handler (fully refactored from existing code)
- [x] EigenLayer protocol stubs (client + handler)
- [x] Verification: Manager crate compiles successfully

### 🚧 In Progress
- [ ] EigenLayer protocol client implementation
- [ ] EigenLayer event handler implementation

### 📋 Next Steps
- [ ] Update executor/mod.rs to use protocol abstraction
- [ ] Update CLI to route protocols correctly
- [ ] Add comprehensive tests
- [ ] Complete EigenLayer implementation

## File Locations

| Component | File |
|-----------|------|
| Core traits | `crates/manager/src/protocol/traits.rs` |
| Types | `crates/manager/src/protocol/types.rs` |
| Factory | `crates/manager/src/protocol/mod.rs` |
| Tangle client | `crates/manager/src/protocol/tangle/client.rs` |
| Tangle handler | `crates/manager/src/protocol/tangle/event_handler.rs` |
| EigenLayer client | `crates/manager/src/protocol/eigenlayer/client.rs` |
| EigenLayer handler | `crates/manager/src/protocol/eigenlayer/event_handler.rs` |

## Design Decisions

### Why traits instead of enums?

- **Extensibility**: New protocols don't require modifying core code
- **Separation**: Each protocol's logic stays in its module
- **Testability**: Easy to mock for testing

### Why Box<dyn Trait>?

- **Flexibility**: Supports different protocol implementations
- **Type erasure**: Manager doesn't need to know concrete types
- **Performance**: Acceptable overhead for manager's event-driven model

### Why async traits?

- **Natural fit**: Event handling is inherently async
- **Composability**: Works well with tokio ecosystem

## Next Steps for Implementation

1. **Complete Tangle handler** - Extract logic from `event_handler.rs`
2. **Implement EigenLayer client** - EVM provider + event polling
3. **Implement EigenLayer handler** - Task/operator event processing
4. **Integrate into executor** - Replace Tangle-specific code
5. **Update CLI** - Add protocol routing
6. **Add tests** - Unit + integration tests

## Questions & Decisions

### Open Questions
- Should we support multiple protocols simultaneously?
- How do we handle protocol-specific configuration?
- What's the upgrade path for existing deployments?

### Decisions Made
- ✅ One protocol per manager instance (simplicity)
- ✅ Protocol type in BlueprintEnvironment (already exists)
- ✅ Gradual rollout (Tangle first, then EigenLayer)

---

**Author**: Claude Code (with expert guidance)
**Date**: 2025-10-16
**Status**: In Progress
