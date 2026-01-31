![Tangle Network Banner](https://raw.githubusercontent.com/tangle-network/tangle/refs/heads/main/assets/Tangle%20%20Banner.png)

<h1 align="center">Blueprint SDK</h1>

<p align="center"><em>A comprehensive toolkit for building, deploying, and managing blueprints on the Tangle Network.</em></p>

<p align="center">
  <a href="https://github.com/tangle-network/blueprint/actions"><img src="https://img.shields.io/github/actions/workflow/status/tangle-network/blueprint/ci.yml?branch=main&logo=github" alt="Build Status"></a>
  <a href="https://github.com/tangle-network/blueprint/releases"><img src="https://img.shields.io/github/v/release/tangle-network/blueprint?sort=semver&filter=blueprint-sdk-*&display_name=release" alt="Latest Release"></a>
  <a href="https://github.com/tangle-network/blueprint/blob/main/LICENSE"><img src="https://img.shields.io/crates/l/blueprint-sdk" alt="License"></a>
  <a href="https://discord.com/invite/cv8EfJu3Tn"><img src="https://img.shields.io/discord/833784453251596298?label=Discord" alt="Discord"></a>
  <a href="https://t.me/tanglenet"><img src="https://img.shields.io/endpoint?color=neon&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Ftanglenet" alt="Telegram"></a>
</p>

## Overview

The Blueprint SDK is a modular Rust toolkit for building decentralized services—called Blueprints—that run across networks like Tangle, EigenLayer, and standard EVM chains.

Blueprints turn complex on-chain and off-chain infrastructure into reproducible, deployable units of logic—think Infrastructure-as-Code for crypto systems.
With one SDK, you can design anything from oracles and MPC networks to agent-based AI services or zk-proof markets, and deploy them seamlessly.

The SDK unifies:
- Job orchestration and routing across async, event-driven systems
- P2P networking with secure message handling and round-based protocol support
- Cryptographic primitives and keystore management for signing, verification, and MPC
- EVM and EigenLayer extensions for direct smart contract and restaking integrations
- Testing and benchmarking utilities for reproducible environments and performance tuning

In short, Blueprints let developers move from concept to distributed protocol with minimal friction.

We also have a [documentation site](https://docs.tangle.tools/) on all things Tangle to help you get started.

### SDK Components

The following components make up the SDK, providing everything from job creation and routing utilities to specialized
tools for networking and testing.

* [`blueprint-sdk`] - Main crate for the Tangle Blueprint SDK, re-exporting all of the following
* [`blueprint-benchmarking`] - Utilities for benchmarking blueprints
* [`blueprint-build-utils`] - Utilities for simplifying build-time tasks (e.g., building contracts, installing dependencies)
* [`blueprint-chain-setup`] - (**Meta-crate**) Utilities for setting local testnets
    * [`blueprint-chain-setup-anvil`] - Utilities for setting up [Anvil] testnets
* [`blueprint-clients`] - (**Meta-crate**) Clients for interacting with Tangle EVM, [Eigenlayer], and other networks
    * [`blueprint-client-core`] - Core client primitives and traits
    * [`blueprint-client-tangle-evm`] - Client for interacting with the Tangle v2 EVM contracts
    * [`blueprint-client-eigenlayer`] - Client for interacting with the [Eigenlayer] Network
    * [`blueprint-client-evm`] - Client for interacting with the EVM Network
* [`blueprint-contexts`] - Extensions for adding functionality to custom blueprint context types
* [`blueprint-context-derive`] - Derive macros for implementing context extension traits
* [`blueprint-core`] - Core components for building blueprints, primarily job system primitives
* [`blueprint-crypto`] - (**Meta-crate**) Cryptographic utilities
    * [`blueprint-crypto-core`] - Core cryptographic utilities (traits, types)
    * [`blueprint-crypto-bls`] - Utilities for working with BLS signatures and keys
    * [`blueprint-crypto-bn254`] - Utilities for working with BN254 signatures and keys
    * [`blueprint-crypto-ed25519`] - Utilities for working with Ed25519 signatures and keys
    * [`blueprint-crypto-hashing`] - Cryptographic hashing utilities
    * [`blueprint-crypto-k256`] - Utilities for working with [secp256k1] signatures and keys
    * [`blueprint-crypto-sr25519`] - Utilities for working with sr25519 signatures and keys
* [`blueprint-keystore`] - Flexible keystore implementation, supporting local and remote signers
* [`blueprint-macros`] - Utility macros for simplifying blueprint development
* [`blueprint-manager`] - A program executor that connects to the Tangle network and runs protocols dynamically on the fly
    * [`blueprint-manager-bridge`] - IPC bridge for manager-blueprint communication
* [`blueprint-metrics`] (**Meta-crate**) Utilities for collecting metrics
    * [`blueprint-metrics-rpc-calls`] - Utilities for collecting metrics from RPC calls
* [`blueprint-networking`] - P2P networking support for blueprints
    * [`blueprint-networking-round-based-extension`] - A networking compatibility layer for [round-based] MPC protocols
    * [`blueprint-networking-agg-sig-gossip`] - Aggregated signature gossip extension
    * [`blueprint-networking-gossip-primitives`] - Gossip protocol primitives
* [`blueprint-pricing-engine`] - Pricing engine for computing resource costs
* [`blueprint-producers-extra`] - Additional protocol-independent event producers
* [`blueprint-profiling`] - Profiling utilities for performance analysis
* [`blueprint-qos`] - Quality of Service monitoring and metrics
* [`blueprint-router`] - A job router for dynamically scheduling jobs
* [`blueprint-runner`] - The blueprint job runner, executing jobs in a protocol-specific manner
* [`blueprint-std`] - Standard library extensions, for use within the SDK
* [`blueprint-stores`] - (**Meta-crate**) Storage providers for blueprints
    * [`blueprint-store-local-database`] - A local JSON key-value database
* [`blueprint-remote-providers`] - Remote cloud provider integrations (AWS, GCP, Azure, etc.)
* [`blueprint-tangle-aggregation-svc`] - Tangle aggregation service for BLS signature aggregation
* [`blueprint-tangle-evm-extra`] - Tangle v2 EVM-specific producers, consumers, and extractors
* [`blueprint-tangle-extra`] - Tangle-specific extensions and utilities
* [`blueprint-evm-extra`] - EVM specific extensions for blueprints
* [`blueprint-eigenlayer-extra`] - Eigenlayer specific extensions for blueprints
* [`blueprint-faas`] - FaaS (Function-as-a-Service) execution support
* [`blueprint-auth`] - Authentication and authorization utilities
* [`blueprint-testing-utils`] - (**Meta-crate**) Utilities for testing blueprints
    * [`blueprint-core-testing-utils`] - Core testing utility primitives
    * [`blueprint-anvil-testing-utils`] - Utilities for creating and interacting with Anvil testnets
    * [`blueprint-eigenlayer-testing-utils`] - Utilities for creating end-to-end tests for Eigenlayer blueprints

[`blueprint-sdk`]: https://docs.rs/blueprint-sdk
[`blueprint-benchmarking`]: https://docs.rs/blueprint-benchmarking
[`blueprint-build-utils`]: https://docs.rs/blueprint-build-utils
[`blueprint-chain-setup`]: https://docs.rs/blueprint-chain-setup
[`blueprint-chain-setup-anvil`]: https://docs.rs/blueprint-chain-setup-anvil
[`blueprint-clients`]: https://docs.rs/blueprint-clients
[`blueprint-client-core`]: https://docs.rs/blueprint-client-core
[`blueprint-client-tangle-evm`]: https://docs.rs/blueprint-client-tangle-evm
[`blueprint-client-eigenlayer`]: https://docs.rs/blueprint-client-eigenlayer
[`blueprint-client-evm`]: https://docs.rs/blueprint-client-evm
[`blueprint-contexts`]: https://docs.rs/blueprint-contexts
[`blueprint-context-derive`]: https://docs.rs/blueprint-context-derive
[`blueprint-core`]: https://docs.rs/blueprint-core
[`blueprint-crypto`]: https://docs.rs/blueprint-crypto
[`blueprint-crypto-core`]: https://docs.rs/blueprint-crypto-core
[`blueprint-crypto-bls`]: https://docs.rs/blueprint-crypto-bls
[`blueprint-crypto-bn254`]: https://docs.rs/blueprint-crypto-bn254
[`blueprint-crypto-ed25519`]: https://docs.rs/blueprint-crypto-ed25519
[`blueprint-crypto-hashing`]: https://docs.rs/blueprint-crypto-hashing
[`blueprint-crypto-k256`]: https://docs.rs/blueprint-crypto-k256
[`blueprint-crypto-sr25519`]: https://docs.rs/blueprint-crypto-sr25519
[`blueprint-keystore`]: https://docs.rs/blueprint-keystore
[`blueprint-macros`]: https://docs.rs/blueprint-macros
[`blueprint-manager`]: https://docs.rs/blueprint-manager
[`blueprint-manager-bridge`]: https://docs.rs/blueprint-manager-bridge
[`blueprint-metrics`]: https://docs.rs/blueprint-metrics
[`blueprint-metrics-rpc-calls`]: https://docs.rs/blueprint-metrics-rpc-calls
[`blueprint-networking`]: https://docs.rs/blueprint-networking
[`blueprint-networking-round-based-extension`]: https://docs.rs/blueprint-networking-round-based-extension
[`blueprint-networking-agg-sig-gossip`]: https://docs.rs/blueprint-networking-agg-sig-gossip
[`blueprint-networking-gossip-primitives`]: https://docs.rs/blueprint-networking-gossip-primitives
[`blueprint-pricing-engine`]: https://docs.rs/blueprint-pricing-engine
[`blueprint-producers-extra`]: https://docs.rs/blueprint-producers-extra
[`blueprint-profiling`]: https://docs.rs/blueprint-profiling
[`blueprint-qos`]: https://docs.rs/blueprint-qos
[`blueprint-router`]: https://docs.rs/blueprint-router
[`blueprint-runner`]: https://docs.rs/blueprint-runner
[`blueprint-std`]: https://docs.rs/blueprint-std
[`blueprint-stores`]: https://docs.rs/blueprint-stores
[`blueprint-store-local-database`]: https://docs.rs/blueprint-store-local-database
[`blueprint-tangle-evm-extra`]: https://docs.rs/blueprint-tangle-evm-extra
[`blueprint-evm-extra`]: https://docs.rs/blueprint-evm-extra
[`blueprint-eigenlayer-extra`]: https://docs.rs/blueprint-eigenlayer-extra
[`blueprint-faas`]: https://docs.rs/blueprint-faas
[`blueprint-auth`]: https://docs.rs/blueprint-auth
[`blueprint-remote-providers`]: https://docs.rs/blueprint-remote-providers
[`blueprint-tangle-aggregation-svc`]: https://docs.rs/blueprint-tangle-aggregation-svc
[`blueprint-tangle-extra`]: https://docs.rs/blueprint-tangle-extra
[`blueprint-testing-utils`]: https://docs.rs/blueprint-testing-utils
[`blueprint-core-testing-utils`]: https://docs.rs/blueprint-core-testing-utils
[`blueprint-anvil-testing-utils`]: https://docs.rs/blueprint-anvil-testing-utils
[`blueprint-eigenlayer-testing-utils`]: https://docs.rs/blueprint-eigenlayer-testing-utils


## 🚀 Getting Started

### 📋 Prerequisites

Ensure you have the following installed:

- [Rust]
- **OpenSSL Development Packages**

#### For Ubuntu/Debian:

```bash
sudo apt update && sudo apt install build-essential cmake libssl-dev pkg-config
```

#### For macOS:

```bash
brew install openssl cmake
```

### 🔧 CLI Installation

You can install the Tangle CLI in two ways:

#### 🚩 **Option 1: Install Script (recommended)**

Install the latest stable version of `cargo-tangle` using the installation script:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tangle-network/blueprint/releases/download/cargo-tangle/v0.1.1-beta.7/cargo-tangle-installer.sh | sh
```

#### 🚩 **Option 2: Install from source**

Install the latest git version of `cargo-tangle` using the following command:

```bash
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint --force
```

### ✨ Creating Your First Blueprint

After installation, you can create, build, register, and run your first blueprint against the Tangle EVM:

```bash
# Create a new blueprint named "my_blueprint"
cargo tangle blueprint create --name my_blueprint

# Navigate into the blueprint directory and build
cd my_blueprint
cargo build

# Deploy your blueprint to the Tangle Network
# Write the contract coordinates used by your service
cat > settings.env <<'EOF'
BLUEPRINT_ID=0
SERVICE_ID=0
TANGLE_CONTRACT=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
RESTAKING_CONTRACT=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
STATUS_REGISTRY_CONTRACT=0xdC64a140Aa3E981100a9BecA4E685f962f0CF6C9
EOF

# Register the operator with the on-chain contracts (optional)
cargo tangle blueprint register-tangle-evm \
  --http-rpc-url https://rpc.tangle.tools \
  --ws-rpc-url wss://rpc.tangle.tools \
  --keystore-path ./keystore \
  --tangle-contract 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  --restaking-contract 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
  --status-registry-contract 0xdC64a140Aa3E981100a9BecA4E685f962f0CF6C9 \
  --blueprint-id 0

# Capture the preregistration payload (TLV file written to ./data by default)
cargo tangle blueprint preregister \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./keystore \
  --settings-file ./settings.env

# Run the blueprint manager against local Anvil (or a real RPC)
cargo tangle blueprint run \
  --protocol tangle-evm \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./keystore \
  --settings-file ./settings.env
```

The preregistration flow mirrors the production pipeline: the CLI boots the manager
in `REGISTRATION_MODE_ON`, launches the blueprint with `REGISTRATION_CAPTURE_ONLY=1`,
and waits for it to emit `registration_inputs.bin` under
`./data/blueprint-<id>-*/`. Each blueprint can use the helper
`blueprint_sdk::registration::write_registration_inputs` to persist its TLV payload
when `BlueprintEnvironment::registration_mode()` returns `true`.

#### Runtime Preferences

`cargo tangle blueprint run` and `cargo tangle blueprint preregister` accept `--preferred-source`
(`native`, `container`, or `wasm`) plus `--vm/--no-vm` so you can control how the manager fetches and
executes artifacts. Pass `--save-runtime-prefs` to write those choices back to `settings.env`
(`PREFERRED_SOURCE` / `USE_VM`) so future commands inherit the same behavior.

Every CLI action that submits a transaction now reports `tx_submitted` / `tx_confirmed`
lines (or JSON objects when `--json` is used), making it easy to track hashes and block
confirmations in logs or CI pipelines.

And your blueprint is ready to go!

### Deploying to Testnet/Mainnet

When targeting real Tangle networks, provide a blueprint definition manifest that mirrors the on-chain schema. The file can be JSON, YAML, or TOML and must describe the blueprint metadata, jobs, and artifact sources (container images or native binaries). Once authored, pass it via `--definition`:

```bash
cargo tangle blueprint deploy tangle \
  --network testnet \
  --definition ./definition.json
```

At minimum the manifest requires `metadata_uri`, `manager`, at least one job, and one source. Fields such as schemas or blueprint-specific config are optional and default to empty values. See `MIGRATION_EVM_ONLY.md` for a detailed example.

### 🧪 Testing Locally

The `blueprint-anvil-testing-utils` crate exposes `harness_builder_from_env` for `TangleEvmHarness`, which replays the `LocalTestnet.s.sol` broadcast so every integration test runs against deterministic contract state. Useful commands:

```bash
# Client integration tests (Anvil-backed)
cargo test -p blueprint-client-tangle-evm --test anvil

# Pricing engine QoS listener
cargo test -p blueprint-pricing-engine --test evm_listener

# Blueprint runner end-to-end harness
cargo test -p blueprint-manager --test tangle_evm_runner

# Example blueprint harness (router + runner wired together)
cargo test -p hello-tangle-blueprint --test anvil
```

Each suite boots its own Anvil container via `testcontainers`, so Docker is required when running locally or in CI.

> **Note:** The harness loads `crates/chain-setup/anvil/snapshots/localtestnet-state.json` and falls back to the bundled `localtestnet-broadcast.json` if the snapshot is missing or fails validation. Refresh fixtures with `scripts/fetch-localtestnet-fixtures.sh`, and set `RUN_TNT_E2E=1` to opt into the longer suites.

For a keystore-to-runner walkthrough (keys, env vars, harness commands, and manual
`TangleEvmClient` snippets) see [`docs/operators/anvil.md`](docs/operators/anvil.md).

For additional commands, advanced configurations, and complete CLI usage, see the [official CLI reference](https://docs.tangle.tools/developers/cli/reference).

## 📮 Support

For support or inquiries:
- **Issues:** Report bugs or request features via GitHub Issues.
- **Discussions:** Engage with the community in GitHub Discussions.
- For real-time assistance and announcements:
    - Join our [Discord server](https://discord.com/invite/cv8EfJu3Tn)
    - Join our [Telegram channel](https://t.me/tanglenet)

## 📜 License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 📬 Feedback and Contributions

We welcome feedback and contributions to improve this blueprint.
Please open an issue or submit a pull request on our GitHub repository.
Please let us know if you fork this blueprint and extend it too!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[Rust]: https://www.rust-lang.org/tools/install
[Tangle Network]: https://tangle.tools
[Eigenlayer]: https://eigenlayer.xyz
[secp256k1]: https://en.bitcoin.it/wiki/Secp256k1
[round-based]: https://crates.io/crates/round-based
[anvil]: https://book.getfoundry.sh/reference/anvil/
