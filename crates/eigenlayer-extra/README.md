# EigenLayer Multi-AVS Framework

High-level framework for running multiple EigenLayer AVS services with a single operator.

## Overview

This crate provides Rust APIs for:
- Managing multiple AVS registrations with persistent state
- Discovering operator AVS registrations on-chain
- Monitoring rewards and slashing events
- Configuring blueprint execution runtimes (native, hypervisor, container)

For CLI usage, see the [CLI README](../../cli/README.md#eigenlayer-multi-avs-commands).

## Architecture

### Multi-AVS Support

Each registered AVS gets:
- **Unique Blueprint ID**: Derived from service_manager address
- **Separate Process**: Independent blueprint binary instance
- **Isolated State**: No shared state between AVS services
- **Contract-Specific Args**: Each blueprint receives its AVS contract addresses
- **Configurable Runtime**: Choose execution environment per AVS

### Runtime Targets

Each AVS can specify its execution runtime:

- **`native`** - Bare process (no sandbox)
  - Fastest startup and lowest overhead
  - For testing only - no isolation

- **`hypervisor`** - cloud-hypervisor VM (default)
  - Production-ready VM isolation
  - Strong security boundaries
  - Resource limits enforced
  - Recommended for production

- **`container`** - Docker/Kata containers (Coming Soon)
  - Not yet implemented
  - For now, use `native` for testing or `hypervisor` for production

### Registration State

Stored in `~/.tangle/eigenlayer_registrations.json`:

```json
{
  "registrations": {
    "0x...service_manager_address": {
      "operator_address": "0x...",
      "config": { ... },
      "status": "Active",
      "registered_at": "2024-10-17T..."
    }
  }
}
```

### Background Services

Operator-level monitoring (runs once per operator):
- **Rewards**: Check claimable rewards every hour
- **Slashing**: Monitor for slashing events every 5 minutes

## API

### Discovery

```rust
use blueprint_eigenlayer_extra::AvsDiscoveryService;

let service = AvsDiscoveryService::new(env);

// Discover all AVS registrations for an operator
let discovered = service.discover_avs_registrations(operator_address).await?;

// Check specific AVS registration
let is_registered = service.is_operator_registered_to_avs(
    operator_address,
    registry_coordinator,
).await?;
```

### Registration Management

```rust
use blueprint_eigenlayer_extra::{RegistrationStateManager, AvsRegistration};

// Load registrations
let manager = RegistrationStateManager::load()?;

// Register new AVS
let registration = AvsRegistration::new(operator_address, config);
manager.register(registration)?;

// Deregister
manager.deregister(service_manager_address)?;

// List all registrations
let registrations = manager.registrations();
```

### Rewards & Slashing

```rust
use blueprint_eigenlayer_extra::{RewardsManager, SlashingMonitor};

// Check claimable rewards
let rewards_manager = RewardsManager::new(env);
let amount = rewards_manager.get_claimable_rewards().await?;

// Monitor slashing
let slashing_monitor = SlashingMonitor::new(env);
let is_slashed = slashing_monitor.is_operator_slashed().await?;
```

## Configuration

### AVS Registration Config

```json
{
  "service_manager": "0x...",
  "registry_coordinator": "0x...",
  "operator_state_retriever": "0x...",
  "strategy_manager": "0x...",
  "delegation_manager": "0x...",
  "avs_directory": "0x...",
  "rewards_coordinator": "0x...",
  "permission_controller": "0x...",
  "allocation_manager": "0x...",
  "strategy_address": "0x...",
  "stake_registry": "0x...",
  "blueprint_path": "/path/to/blueprint",
  "runtime_target": "hypervisor",
  "allocation_delay": 0,
  "deposit_amount": 5000000000000000000000,
  "stake_amount": 1000000000000000000,
  "operator_sets": [0]
}
```

## Examples

See [`examples/incredible-squaring-eigenlayer/`](../../examples/incredible-squaring-eigenlayer/) for a complete AVS blueprint example.

## Testing

```bash
# Run all tests
cargo test -p blueprint-eigenlayer-extra

# Run manager tests
cargo test -p blueprint-manager --test multi_avs_test
cargo test -p blueprint-manager --test protocol_integration
```
