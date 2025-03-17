<p align="center">
  <img src="https://cdn.prod.website-files.com/6494562b44a28080aafcbad4/65aaf8af22c118aa4441278f_Tangle%20Logo_topnav_dark.svg" alt="Tangle Network Logo" width="150"/>
</p>

<h1 align="center">Blueprint SDK</h1>

<p align="center"><em>A comprehensive toolkit for building, deploying, and managing blueprints on the Tangle Network.</em></p>

<p align="center">
  <a href="https://github.com/tangle-network/blueprint/actions"><img src="https://img.shields.io/github/actions/workflow/status/tangle-network/blueprint/ci.yml?branch=main&logo=github" alt="Build Status"></a>
  <a href="https://github.com/tangle-network/blueprint/releases"><img src="https://img.shields.io/github/v/release/tangle-network/blueprint?logo=github" alt="Latest Release"></a>
  <a href="https://github.com/tangle-network/blueprint/blob/main/LICENSE"><img src="https://img.shields.io/github/license/tangle-network/blueprint" alt="License"></a>
  <a href="https://discord.com/invite/cv8EfJu3Tn"><img src="https://img.shields.io/discord/833784453251596298?label=Discord" alt="Discord"></a>
  <a href="https://t.me/tanglenet"><img src="https://img.shields.io/badge/Telegram-2CA5E0?logo=telegram&logoColor=white" alt="Telegram"></a>
</p>

---

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

### Installation

Install the Tangle CLI:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tangle-network/gadget/releases/download/cargo-tangle/v0.1.1-beta.7/cargo-tangle-installer.sh | sh
```

Or install it from source:

```bash
cargo install cargo-tangle --git https://github.com/tangle-network/gadget --force
```

### Creating Your First Blueprint
1. **Create a New Blueprint:**

```bash
cargo tangle blueprint create --name my_blueprint
```

2. **Build Your Blueprint:**

```bash
cargo build
```

3. **Deploy to the Tangle Network:**

```bash
cargo tangle blueprint deploy --rpc-url wss://rpc.tangle.tools --package my_blueprint
```

---

## 🔑 Key Management

### Key Generation
Generate cryptographic keys using the CLI:
```bash
cargo tangle blueprint generate-keys -k <KEY_TYPE> -p <PATH> -s <SURI/SEED> --show-secret
```

### Supported Key Types
| Key Type  | Description                                | Use Case                          |
|-----------|--------------------------------------------|-----------------------------------|
| sr25519   | Schnorrkel/Ristretto x25519                | Tangle Network account keys       |
| ecdsa     | Elliptic Curve Digital Signature Algorithm | EVM compatible chains             |
| bls_bn254 | BLS signatures on BN254 curve              | EigenLayer validators             |
| ed25519   | Edwards-curve Digital Signature Algorithm  | General purpose signatures        |
| bls381    | BLS signatures on BLS12-381 curve          | Advanced cryptographic operations |

### Environment Variables
Configure your project with these variables:

| Variable     | Description                   | Example                                          |
|--------------|-------------------------------|--------------------------------------------------|
| SIGNER       | Substrate signer account SURI | `export SIGNER="//Alice"`                        |
| EVM_SIGNER   | EVM signer private key        | `export EVM_SIGNER="0xcb6df..."`                 |
| RPC_URL      | Tangle Network RPC endpoint   | `export RPC_URL="wss://rpc.tangle.tools"`        |
| HTTP_RPC_URL | HTTP RPC endpoint             | `export HTTP_RPC_URL="https://rpc.tangle.tools"` |

---

## 🧪 Development

### Testing
Run the complete test suite:
```bash
# Run all tests
cargo test

# Run tests for a specific package
cargo test --package my_blueprint

# Run tests with debug logging enabled
RUST_LOG=gadget=debug cargo test
```

---

## 📮 Support
For support or inquiries:
- **Issues:** Report bugs or request features via GitHub Issues.
- **Discussions:** Engage with the community in GitHub Discussions.
- **Discord:** Join our [Discord server](https://discord.com/invite/cv8EfJu3Tn) for real-time assistance.

---

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
