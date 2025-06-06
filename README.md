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

Blueprints are Infrastructure-as-Code templates that allow developers to quickly build crypto services. The Blueprint SDK
comes equipped with a variety of tools, from event listeners and p2p networking to flexible keystores, allowing you to rapidly
prototype distributed systems. With these tools, developers can get started building anything from oracles to bridge
security systems, zk prover networks, AI agent orchestration systems. Deploy these applications on the [Tangle Network], [Eigenlayer], or natively.

We also have a [documentation site](https://docs.tangle.tools/) on all things Tangle to help you get started.

### SDK Components

The following components make up the SDK, providing everything from job creation and routing utilities to specialized
tools for networking and testing.

* [`blueprint-sdk`] - Main crate for the Tangle Blueprint SDK, re-exporting all of the following
* [`blueprint-benchmarking`] - Utilities for benchmarking blueprints
* [`blueprint-build-utils`] - Utilities for simplifying build-time tasks (e.g., building contracts, installing dependencies)
* [`blueprint-chain-setup`] - (**Meta-crate**) Utilities for setting local testnets
    * [`blueprint-chain-setup-common`] - Common utilities for setting up testnets
    * [`blueprint-chain-setup-anvil`] - Utilities for setting up [Anvil] testnets
    * [`blueprint-chain-setup-tangle`] - Utilities for setting up Tangle testnets
* [`blueprint-clients`] - (**Meta-crate**) Clients for interacting with Tangle, [Eigenlayer], and other networks
    * [`blueprint-client-tangle`] - Client for interacting with the Tangle Network
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
    * [`blueprint-crypto-k256`] - Utilities for working with [secp256k1] signatures and keys
    * [`blueprint-crypto-sp-core`] - Blueprint-compatible crypto wrappers around [sp-core] primitives
    * [`blueprint-crypto-sr25519`] - Utilities for working with sr25519 signatures and keys
    * [`blueprint-crypto-tangle-pair-signer`] - Pair signer type for Tangle
* [`blueprint-keystore`] - Flexible keystore implementation, supporting local and remote signers
* [`blueprint-macros`] - Utility macros for simplifying blueprint development
* [`blueprint-manager`] - A program executor that connects to the Tangle network and runs protocols dynamically on the fly
* [`blueprint-metrics`] (**Meta-crate**) Utilities for collecting metrics
    * [`blueprint-metrics-rpc-calls`] - Utilities for collecting metrics from RPC calls
* [`blueprint-networking`] - P2P networking support for blueprints
    * [`blueprint-networking-round-based-extension`] - A networking compatibility layer for [round-based] MPC protocols
* [`blueprint-producers-extra`] - Additional protocol-independent event producers
* [`blueprint-router`] - A job router for dynamically scheduling jobs
* [`blueprint-runner`] - The blueprint job runner, executing jobs in a protocol-specific manner
* [`blueprint-std`] - Standard library extensions, for use within the SDK
* [`blueprint-stores`] - (**Meta-crate**) Storage providers for blueprints
    * [`blueprint-store-local-database`] - A local JSON key-value database
* [`blueprint-tangle-extra`] - Tangle specific extensions for blueprints
* [`blueprint-evm-extra`] - EVM specific extensions for blueprints
* [`blueprint-eigenlayer-extra`] - Eigenlayer specific extensions for blueprints
* [`blueprint-testing-utils`] - (**Meta-crate**) Utilities for testing blueprints
    * [`blueprint-core-testing-utils`] - Core testing utility primitives
    * [`blueprint-anvil-testing-utils`] - Utilities for creating and interacting with Anvil testnets
    * [`blueprint-tangle-testing-utils`] - Utilities for creating end-to-end tests for Tangle blueprints
    * [`blueprint-eigenlayer-testing-utils`] - Utilities for creating end-to-end tests for Eigenlayer blueprints

[`blueprint-sdk`]: https://docs.rs/blueprint-sdk
[`blueprint-benchmarking`]: https://docs.rs/blueprint-benchmarking
[`blueprint-build-utils`]: https://docs.rs/blueprint-build-utils
[`blueprint-chain-setup`]: https://docs.rs/blueprint-chain-setup
[`blueprint-chain-setup-common`]: https://docs.rs/blueprint-chain-setup-common
[`blueprint-chain-setup-anvil`]: https://docs.rs/blueprint-chain-setup-anvil
[`blueprint-chain-setup-tangle`]: https://docs.rs/blueprint-chain-setup-tangle
[`blueprint-clients`]: https://docs.rs/blueprint-clients
[`blueprint-client-tangle`]: https://docs.rs/blueprint-client-tangle
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
[`blueprint-crypto-k256`]: https://docs.rs/blueprint-crypto-k256
[`blueprint-crypto-sp-core`]: https://docs.rs/blueprint-crypto-sp-core
[`blueprint-crypto-sr25519`]: https://docs.rs/blueprint-crypto-sr25519
[`blueprint-crypto-tangle-pair-signer`]: https://docs.rs/blueprint-crypto-tangle-pair-signer
[`blueprint-keystore`]: https://docs.rs/blueprint-keystore
[`blueprint-macros`]: https://docs.rs/blueprint-macros
[`blueprint-manager`]: https://docs.rs/blueprint-manager
[`blueprint-metrics`]: https://docs.rs/blueprint-metrics
[`blueprint-metrics-rpc-calls`]: https://docs.rs/blueprint-metrics-rpc-calls
[`blueprint-networking`]: https://docs.rs/blueprint-networking
[`blueprint-networking-round-based-extension`]: https://docs.rs/blueprint-networking-round-based-extension
[`blueprint-producers-extra`]: https://docs.rs/blueprint-producers-extra
[`blueprint-router`]: https://docs.rs/blueprint-router
[`blueprint-runner`]: https://docs.rs/blueprint-runner
[`blueprint-std`]: https://docs.rs/blueprint-std
[`blueprint-stores`]: https://docs.rs/blueprint-stores
[`blueprint-store-local-database`]: https://docs.rs/blueprint-store-local-database
[`blueprint-tangle-extra`]: https://docs.rs/blueprint-tangle-extra
[`blueprint-evm-extra`]: https://docs.rs/blueprint-evm-extra
[`blueprint-eigenlayer-extra`]: https://docs.rs/blueprint-eigenlayer-extra
[`blueprint-testing-utils`]: https://docs.rs/blueprint-testing-utils
[`blueprint-core-testing-utils`]: https://docs.rs/blueprint-core-testing-utils
[`blueprint-anvil-testing-utils`]: https://docs.rs/blueprint-anvil-testing-utils
[`blueprint-tangle-testing-utils`]: https://docs.rs/blueprint-tangle-testing-utils
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

After installation, you can create, build, and deploy your first blueprint using the following commands:

```bash
# Create a new blueprint named "my_blueprint"
cargo tangle blueprint create --name my_blueprint

# Navigate into the blueprint directory and build
cd my_blueprint
cargo build

# Deploy your blueprint to the Tangle Network
cargo tangle blueprint deploy --rpc-url wss://rpc.tangle.tools --package my_blueprint
```

And your blueprint is ready to go!

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
[sp-core]: https://crates.io/crates/sp-core
[round-based]: https://crates.io/crates/round-based
[anvil]: https://book.getfoundry.sh/reference/anvil/
