# EigenLayer Multi-AVS Framework

High-level framework for running multiple EigenLayer AVS services with a single operator.

## Quick Start

### 1. Generate Keys

```bash
# Generate ECDSA key (operator address)
cargo tangle blueprint generate-keys -k ecdsa -p ./keystore

# Generate BLS key (for aggregation)
cargo tangle blueprint generate-keys -k bls -p ./keystore
```

### 2. Register with an AVS

Create a configuration file `my-avs-config.json`:

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
  "blueprint_path": "/path/to/your/avs/blueprint",
  "runtime_target": "hypervisor",
  "allocation_delay": 0,
  "deposit_amount": 5000000000000000000000,
  "stake_amount": 1000000000000000000,
  "operator_sets": [0]
}
```

Register:

```bash
cargo tangle blueprint eigenlayer register \
  --config my-avs-config.json \
  --keystore-uri ./keystore
```

Or override runtime target via CLI:

```bash
cargo tangle blueprint eigenlayer register \
  --config my-avs-config.json \
  --keystore-uri ./keystore \
  --runtime native
```

### 3. List Registrations

```bash
# List all registrations
cargo tangle blueprint eigenlayer list

# List only active registrations
cargo tangle blueprint eigenlayer list --active-only

# JSON output
cargo tangle blueprint eigenlayer list --format json
```

### 4. Run the Manager

```bash
cargo tangle blueprint run \
  --protocol eigenlayer \
  --config ./config.toml
```

The manager will:
- Read all active AVS registrations from `~/.tangle/eigenlayer_registrations.json`
- Spawn a separate blueprint instance for each AVS
- Monitor rewards and slashing events
- Auto-restart failed blueprints

### 5. Deregister from an AVS

```bash
cargo tangle blueprint eigenlayer deregister \
  --service-manager 0x... \
  --keystore-uri ./keystore
```

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
  - **For testing only** - no isolation
  - Direct process execution

- **`hypervisor`** - cloud-hypervisor VM (default)
  - Production-ready VM isolation
  - Strong security boundaries
  - Resource limits enforced
  - **Recommended for production**

- **`container`** - Docker/Kata containers (**Coming Soon**)
  - Not yet implemented
  - Will require extending config with container image field
  - For now, use `native` for testing or `hypervisor` for production

Set via config file:
```json
{
  "runtime_target": "hypervisor"
}
```

Or override via CLI:
```bash
--runtime native  # Testing only
--runtime hypervisor  # Production (requires Linux/KVM)
```

### Background Services

Operator-level monitoring (runs once per operator):
- **Rewards**: Check claimable rewards every hour
- **Slashing**: Monitor for slashing events every 5 minutes

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

## CLI Commands

### `register`

Register with a new EigenLayer AVS.

```bash
cargo tangle blueprint eigenlayer register \
  --config <CONFIG_FILE> \
  --keystore-uri <KEYSTORE_PATH> \
  [--runtime <RUNTIME>] \
  [--verify]
```

**Arguments**:
- `--config`: Path to JSON configuration file
- `--keystore-uri`: Keystore path (default: `./keystore`)
- `--runtime`: Runtime target (`native`, `hypervisor`, `container`) - overrides config file
- `--verify`: Perform on-chain verification (optional)

**Aliases**: `reg`

### `deregister`

Deregister from an EigenLayer AVS.

```bash
cargo tangle blueprint eigenlayer deregister \
  --service-manager <ADDRESS> \
  --keystore-uri <KEYSTORE_PATH>
```

**Arguments**:
- `--service-manager`: Service manager contract address
- `--keystore-uri`: Keystore path (default: `./keystore`)

**Aliases**: `dereg`

### `list`

List all registered AVS services.

```bash
cargo tangle blueprint eigenlayer list \
  [--active-only] \
  [--format <FORMAT>]
```

**Arguments**:
- `--active-only`: Show only active registrations
- `--format`: Output format: `table` (default) or `json`

**Aliases**: `ls`

### `sync`

Synchronize local registrations with on-chain state.

```bash
cargo tangle blueprint eigenlayer sync \
  --http-rpc-url <URL> \
  --keystore-uri <KEYSTORE_PATH> \
  [--settings-file <FILE>]
```

**Arguments**:
- `--http-rpc-url`: HTTP RPC endpoint (default: `http://127.0.0.1:8545`)
- `--keystore-uri`: Keystore path (default: `./keystore`)
- `--settings-file`: Protocol settings file (optional)

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

## Configuration File Format

### AVS Registration Config

```json
{
  "service_manager": "0x...",              // ServiceManager contract
  "registry_coordinator": "0x...",         // RegistryCoordinator contract
  "operator_state_retriever": "0x...",     // OperatorStateRetriever contract
  "strategy_manager": "0x...",             // StrategyManager contract
  "delegation_manager": "0x...",           // DelegationManager contract
  "avs_directory": "0x...",                // AVSDirectory contract
  "rewards_coordinator": "0x...",          // RewardsCoordinator contract
  "permission_controller": "0x...",        // PermissionController contract (optional)
  "allocation_manager": "0x...",           // AllocationManager contract (optional)
  "strategy_address": "0x...",             // Strategy contract for deposits
  "stake_registry": "0x...",               // StakeRegistry contract
  "blueprint_path": "/path/to/blueprint",  // Path to AVS blueprint binary
  "runtime_target": "hypervisor",          // Runtime: native, hypervisor, container
  "allocation_delay": 0,                   // Allocation delay in seconds
  "deposit_amount": 5000000000000000000000, // Deposit amount (wei)
  "stake_amount": 1000000000000000000,     // Stake amount (wei)
  "operator_sets": [0]                     // Operator sets to register with
}
```

## Examples

See [`examples/incredible-squaring-eigenlayer/`](../../examples/incredible-squaring-eigenlayer/) for a complete AVS blueprint example.

## Troubleshooting

### Registration file not found

The first time you register, the file `~/.tangle/eigenlayer_registrations.json` will be created automatically.

### Blueprint fails to spawn

Check:
1. `blueprint_path` in config points to a valid Rust project with `Cargo.toml`
2. Project has a binary target matching the directory name
3. Keystore contains valid ECDSA and BLS keys

### On-chain verification fails

Use `sync` command to verify on-chain state:

```bash
cargo tangle blueprint eigenlayer sync \
  --http-rpc-url http://your-rpc-endpoint \
  --keystore-uri ./keystore
```

## Architecture Diagrams

### Registration Flow

```
CLI (register) → RegistrationStateManager → ~/.tangle/eigenlayer_registrations.json
                                                         ↓
Blueprint Manager → Load registrations → Spawn AVS blueprints
```

### Multi-AVS Runtime

```
Blueprint Manager
├── Background Services (operator-level)
│   ├── RewardsManager (hourly)
│   └── SlashingMonitor (5 min)
└── AVS Blueprints (per-AVS)
    ├── AVS #1 (blueprint_id=hash(service_manager_1))
    ├── AVS #2 (blueprint_id=hash(service_manager_2))
    └── AVS #3 (blueprint_id=hash(service_manager_3))
```

## Testing

```bash
# Run all tests
cargo test -p blueprint-eigenlayer-extra

# Run manager tests
cargo test -p blueprint-manager --test multi_avs_test
cargo test -p blueprint-manager --test protocol_integration
```
