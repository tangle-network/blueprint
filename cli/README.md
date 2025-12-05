# Tangle CLI

Create and Deploy blueprints on Tangle Network.

## Table of Contents

- [Tangle CLI](#tangle-cli)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Installation](#installation)
    - [Feature flags](#feature-flags)
  - [Creating a New Blueprint](#creating-a-new-blueprint)
    - [Example](#example)
  - [Build The Blueprint](#build-the-blueprint)
  - [Unit Testing](#unit-testing)
  - [Deploying the Blueprint to a Local Tangle Node](#deploying-the-blueprint-to-a-local-tangle-node)
    - [Example](#example-1)
  - [Optional Environment Variables for Deployment](#optional-environment-variables-for-deployment)
    - [Example of ENV Variables](#example-of-env-variables)
  - [Cloud Deployment](#cloud-deployment)
    - [Configure Cloud Provider](#configure-cloud-provider)
    - [Cost Estimation](#cost-estimation)
    - [Deploy Blueprint to Cloud](#deploy-blueprint-to-cloud)
    - [Monitor Cloud Deployments](#monitor-cloud-deployments)
  - [Interacting with a deployed Blueprint](#interacting-with-a-deployed-blueprint)
  - [Generating Keys from the Command Line](#generating-keys-from-the-command-line)
    - [Flags](#flags)

## Overview

The Tangle CLI is a command-line tool that allows you to create and deploy blueprints on the Tangle network. It
provides a simple and efficient way to manage your blueprints, making it easy to get started with Tangle
Blueprints.

## Installation

To install the Tangle CLI, run the following command:

> Supported on Linux, MacOS, and Windows (WSL2)

```bash
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint --force
```

### Feature flags

The CLI supports optional features that can be enabled at build time:

**`remote-providers`** - Enables cloud deployment functionality

Adds support for deploying blueprints to AWS, GCP, Azure, DigitalOcean, and Vultr. This enables the `cargo tangle cloud` subcommand and the `--remote` flag for blueprint deployment.

```bash
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint \
  --features remote-providers --force
```

Without this feature, cloud commands are not available and using `--remote` will show:
```
❌ Remote deployment requires the 'remote-providers' feature.
   Build with: cargo build --features remote-providers
```

**`vm-debug`** - Enables VM sandbox debugging (Linux only)

```bash
cargo build --features vm-debug
```

## Creating a New Tangle Blueprint

To create a new blueprint using the Tangle CLI, use the following command:

```bash
cargo tangle blueprint create --name <blueprint_name>
```

Replace `<blueprint_name>` with the desired name for your blueprint.

### Example

```bash
cargo tangle blueprint create --name my_blueprint
```

## Build The Blueprint

To build the blueprint, you can simply use cargo as you would with any rust project:

```bash
cargo build
```

## Unit Testing

To run the unit tests, use the following command:

```bash
cargo test
```

## Deploying the Blueprint to a Local Tangle Node

To deploy the blueprint to a local Tangle node, use the following command:

```bash
cargo tangle blueprint deploy tangle --devnet --package <package_name>
```

Replace `<package_name>` with the name of the package to deploy, or it can be omitted if the workspace has only one package. Using the devnet flag automatically starts a local Tangle testnet
and creates a keystore for you. The deployment keystore is generated at `./deploy-keystore` with Bob's account keys. Additionally, it generates a second keystore for testing purposes at `./test-keystore` with Alice's account keys.

### Example

```bash
cargo tangle blueprint deploy tangle --ws-rpc-url ws://localhost:9944 --keystore-path ./my-keystore --package my_blueprint
```

Expected output:

```
Blueprint #0 created successfully by 5F3sa2TJAWMqDhXG6jhV4N8ko9rUjC2q7z6z5V5s5V5s5V5s with extrinsic hash: 0x1234567890abcdef
```

## Optional Environment Variables for Deployment

The following environment variables are optional for deploying the blueprint:

- `SIGNER`: The SURI of the signer account.
- `EVM_SIGNER`: The SURI of the EVM signer account.

These environment variables can be specified instead of supplying a keystore when deploying a blueprint. It should be noted that these environment variables are not prioritized over a supplied keystore.

### Example of ENV Variables

```bash
export SIGNER="//Alice" # Substrate Signer account
export EVM_SIGNER="0xcb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854" # EVM signer account
```

## Cloud Deployment

> **Note:** Cloud deployment requires the `remote-providers` feature flag. See [Feature flags](#feature-flags) for installation instructions.

The Tangle CLI supports deploying blueprints to cloud providers for scalable, distributed execution:

### Configure Cloud Provider

```bash
# Configure AWS
cargo tangle cloud configure aws --region us-east-1 --set-default

# Configure GCP  
cargo tangle cloud configure gcp --region us-central1

# Configure other providers
cargo tangle cloud configure digitalocean --region nyc1
cargo tangle cloud configure vultr --region ewr
cargo tangle cloud configure azure --region eastus
```

### Cost Estimation

```bash
# Compare costs across all providers
cargo tangle cloud estimate --compare --cpu 4 --memory 16

# Estimate for specific provider with spot pricing
cargo tangle cloud estimate --provider aws --spot --duration 30d

# GPU-enabled instances
cargo tangle cloud estimate --provider gcp --gpu 1 --cpu 8 --memory 32
```

### Deploy Blueprint to Cloud

```bash
# Deploy with remote deployment flag
cargo tangle blueprint deploy tangle --remote --package my_blueprint

# Deploy with specific policy
cargo tangle cloud policy --gpu-providers gcp,aws --cost-providers vultr,do
cargo tangle blueprint deploy tangle --remote --package my_blueprint
```

### Monitor Cloud Deployments

```bash
# Check status of all deployments
cargo tangle cloud status

# Check specific deployment
cargo tangle cloud status --deployment-id dep-abc123

# Terminate deployment
cargo tangle cloud terminate --deployment-id dep-abc123
```

## Interacting with a deployed Blueprint

Once the blueprint is deployed, it can now be used on-chain. We have a collection of CLI commands that are useful for interacting with Blueprints, including the ones covered above:

- Create: `cargo tangle blueprint create`
- Deploy: `cargo tangle blueprint deploy`
- List Service Requests: `cargo tangle blueprint list-requests`
- List Blueprints: `cargo tangle blueprint list-blueprints`
- Register: `cargo tangle blueprint register`
- Request Service: `cargo tangle blueprint request-service`
- Accept Service Request: `cargo tangle blueprint accept`
- Reject Service Request: `cargo tangle blueprint reject`
- Run Blueprint: `cargo tangle blueprint run`
- Submit Job: `cargo tangle blueprint submit`

Further details on each command, as well as a full demo, can be found on our [Tangle CLI docs page](https://docs.tangle.tools/developers/cli/tangle).

## EigenLayer Multi-AVS Commands

The Tangle CLI provides commands for managing multiple EigenLayer AVS registrations with a single operator.

### Quick Start: EigenLayer

#### 1. Generate Keys

```bash
# Generate ECDSA key (operator address)
cargo tangle blueprint generate-keys -k ecdsa -p ./keystore

# Generate BLS key (for aggregation)
cargo tangle blueprint generate-keys -k bls -p ./keystore
```

#### 2. Register with an AVS

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

#### 3. List Registrations

```bash
# List all registrations
cargo tangle blueprint eigenlayer list

# List only active registrations
cargo tangle blueprint eigenlayer list --active-only

# JSON output
cargo tangle blueprint eigenlayer list --format json
```

#### 4. Run the Manager

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
