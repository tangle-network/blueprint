# Tangle CLI

Create, run, and operate blueprints on the Tangle EVM and EigenLayer.

## Overview

The CLI bundles every workflow needed for the EVM-only SDK:

- `cargo tangle blueprint create` scaffolds a new blueprint from the templates.
- `cargo tangle blueprint run --protocol tangle-evm` boots a manager connected to the Tangle v2 contracts.
- `cargo tangle blueprint register-tangle-evm` registers an operator against `ITangle`, `MultiAssetDelegation`, and `OperatorStatusRegistry`.
- `cargo tangle key *` manages local and remote k256 keys via `blueprint-keystore`.

All Substrate helpers have been removed; the CLI now targets EVM-first flows only.

## Installation

> Linux, macOS, and Windows (WSL2) are supported.

```bash
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint --force
```

## Creating a New Blueprint

```bash
cargo tangle blueprint create --name my_blueprint
```

The scaffold asks for a source template, optional variables, and whether to skip prompts. The generated workspace already depends on `blueprint-sdk` with the `tangle-evm` feature.

## Running a Blueprint on Tangle EVM

The runner expects RPC URLs, a keystore, and the EVM contract coordinates. You can provide them via CLI flags or a `settings.env` file that the command loads before boot.

```bash
BLUEPRINT_ID=0 \
SERVICE_ID=0 \
TANGLE_CONTRACT=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
RESTAKING_CONTRACT=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
STATUS_REGISTRY_CONTRACT=0xdC64a140Aa3E981100a9BecA4E685f962f0CF6C9 \
cargo tangle blueprint run \
  --protocol tangle-evm \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./keystore \
  --settings-file ./settings.env
```

| Variable | Description |
| --- | --- |
| `BLUEPRINT_ID` | Blueprint registered on `ITangle`. |
| `SERVICE_ID` | Optional fixed service (leave unset to process all). |
| `TANGLE_CONTRACT` | `ITangle` contract address. |
| `RESTAKING_CONTRACT` | `MultiAssetDelegation` contract. |
| `STATUS_REGISTRY_CONTRACT` | `OperatorStatusRegistry` heartbeat contract. |

The CLI automatically ensures an ECDSA key exists under `--keystore-path` and derives the operator address from it.

## Registering an Operator

`register-tangle-evm` performs the on-chain registration + announcement flow in one command:

```bash
cargo tangle blueprint register-tangle-evm \
  --http-rpc-url https://rpc.tangle.tools \
  --ws-rpc-url wss://rpc.tangle.tools \
  --keystore-path ./keystore \
  --tangle-contract 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  --restaking-contract 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
  --status-registry-contract 0xdC64a140Aa3E981100a9BecA4E685f962f0CF6C9 \
  --blueprint-id 0 \
  --registration-inputs ./registration.tlv
```

You can optionally set `--rpc-endpoint` to push metadata to a remote operator directory service during registration.

## Service Lifecycle Commands

The `cargo tangle blueprint service` namespace mirrors everything exposed in `Tangle.sol`:

- `service request` submits a pending service with per-operator exposures, rich config payloads, and optional asset security requirements.
- `service approve` / `service reject` lets operators respond to a request; approvals can include explicit asset commitments when the request included requirements.
- `service join` / `service leave` let operators participate in dynamic membership services; leaving succeeds when the service's exit queue allows the legacy helper.
- `service list` and `service requests` surface active services vs. pending requests, with a `--json` toggle for scripting.

### Requesting a Service

```bash
cargo tangle blueprint service request \
  --blueprint-id 1 \
  --operator 0x... --operator 0x... \
  --operator-exposure-bps 7000 --operator-exposure-bps 3000 \
  --permitted-caller 0xdeadbeef... \
  --config-file ./service-config.bin \
  --ttl 86400 \
  --payment-token 0x0000000000000000000000000000000000000000 \
  --payment-amount 1000000000000000000 \
  --security-requirement native:_ :5000:10000
```

- Provide one `--operator-exposure-bps` per operator (basis points, 10_000 = 100%). Leave unset to fall back to the default BPS of 100% per operator.
- Security requirements use the format `KIND:TOKEN:MIN:MAX` where `KIND` is `native` or `erc20`. Use `_`/`0` for the native token placeholder.
- TTLs are expressed in seconds to match the `Tangle.sol` schema.

### Approving or Rejecting

```bash
# Simple restaking approval
cargo tangle blueprint service approve \
  --request-id 42 \
  --restaking-percent 50

# Approval that matches request-level security requirements
cargo tangle blueprint service approve \
  --request-id 42 \
  --security-commitment native:_ :6000
```

- `--security-commitment` mirrors the `KIND:TOKEN:EXPOSURE` structure expected by `approveServiceWithCommitments`.
- Use `service reject --request-id <ID>` to decline participation.

### Listing Services/Requests

```bash
cargo tangle blueprint service list --json
cargo tangle blueprint service requests --json
```

Both commands read via `TangleEvmClient::{list_services,list_service_requests}` and print either friendly tables or JSON for automation.

### Spawning a Service Runtime

Kick off the blueprint manager using a specific runtime without re-running the full deploy flow:

```bash
cargo tangle blueprint service spawn \
  --http-rpc-url https://rpc.tangle.tools \
  --ws-rpc-url wss://rpc.tangle.tools \
  --keystore-path ./keystore \
  --tangle-contract 0xCf7E... \
  --restaking-contract 0xe7f1... \
  --status-registry-contract 0xdC64... \
  --blueprint-id 1 \
  --service-id 2 \
  --spawn-method vm \
  --data-dir ./data \
  --allow-unchecked-attestations
```

The command reuses the same manager wiring as `cargo tangle blueprint run`, so any RPC endpoint + runtime combo works (VM/native/container). For devnet, omit the overrides and the defaults point at the local harness.

## Key Management

All keys are EVM-native. Use the `cargo tangle key` subcommands to handle k256 material:

```bash
# Generate a new operator key
cargo tangle key generate --key-type ecdsa --output ./keystore

# Import an existing hex secret into the keystore
cargo tangle key import --key-type ecdsa \
  --secret 0x0123... \
  --keystore-path ./keystore \
  --protocol tangle-evm

# List local keys
cargo tangle key list --keystore-path ./keystore
```

The keystore supports filesystem, in-memory, and remote HSM backends through `blueprint-keystore`.

## EigenLayer Helpers

EigenLayer support remains available under `cargo tangle blueprint eigenlayer <subcommand>`. Use it to register AVSs, list allocations, or run the EigenLayer manager by supplying the addresses exported from the EigenLayer contracts.

## Operator Status

Inspect the latest heartbeat/online status from `OperatorStatusRegistry`:

```bash
cargo tangle operator status \
  --http-rpc-url https://rpc.tangle.tools \
  --ws-rpc-url wss://rpc.tangle.tools \
  --keystore-path ./keystore \
  --tangle-contract 0xCf7E... \
  --restaking-contract 0xe7f1... \
  --status-registry-contract 0xdC64... \
  --blueprint-id 1 \
  --service-id 2 \
  --operator 0xdeadbeef... \
  --json
```

Omit `--operator` to query the locally configured operator. Add `--json` for machine-friendly output (timestamp, raw status code, online boolean).

## Need More?

- End-to-end demos and advanced options live on the [CLI reference](https://docs.tangle.tools/developers/cli/reference).
- For pricing/QoS flows, combine the CLI with `crates/pricing-engine` and the new `OPERATOR_*` env vars described in that README.

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

#### 5. Deregister from an AVS

```bash
cargo tangle blueprint eigenlayer deregister \
  --service-manager 0x... \
  --keystore-uri ./keystore
```

### EigenLayer Command Reference

#### `eigenlayer register`

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

#### `eigenlayer deregister`

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

#### `eigenlayer list`

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

#### `eigenlayer sync`

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

### Runtime Targets

Each AVS can specify its execution runtime:

- **`native`** - Bare process (no sandbox)
  - Fastest startup and lowest overhead
  - For testing only - no isolation
  - Direct process execution

- **`hypervisor`** - cloud-hypervisor VM (default)
  - Production-ready VM isolation
  - Strong security boundaries
  - Resource limits enforced
  - Recommended for production

- **`container`** - Docker/Kata containers (Coming Soon)
  - Not yet implemented
  - For now, use `native` for testing or `hypervisor` for production

Set via config file or override via CLI `--runtime` flag.

## Generating Keys from the Command Line

The following command will generate a keypair for a given key type:

```shell
cargo tangle blueprint generate-keys -k <KEY_TYPE> -p <PATH> -s <SURI/SEED> --show-secret
```

where it is optional to include the path, seed, or the show-secret flags.

### Flags

- `-k` or `--key-type`: Required flag. The key type to generate (sr25519, ecdsa, bls_bn254, ed25519, bls381).
- `-p` or `--path`: The path to write the generated keypair to. If not provided, the keypair will be written solely to stdout.
- `-s` or `--seed`: The suri/seed to generate the keypair from. If not provided, a random keypair will be generated.
- `--show-secret`: Denotes that the Private Key should also be printed to stdout. If not provided, only the public key will be printed.

For all of our features for created and using keys, see the [key management](https://docs.tangle.tools/developers/cli/keys) section of our CLI docs.
