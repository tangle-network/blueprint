# Local End-to-End Test Plan (Clean Slate)

This document describes how to test the complete blueprint lifecycle locally **from scratch** using the production-like CLI flow. This approach matches the testnet/mainnet workflow exactly and is recommended for integration testing.

This approach mirrors exactly what you'd do on testnet/mainnet:
1. Create a blueprint using the CLI
2. Build and package your blueprint binary
3. Serve artifacts locally (simulates GitHub/IPFS)
4. Deploy using the same CLI commands as production

---

## Prerequisites

### Install Foundry

```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

### Install Homebrew Protobuf (macOS only)

The `protobuf-src` crate builds protobuf from source, which requires `libatomic` that doesn't exist on macOS ARM64. Using Homebrew's pre-built protobuf avoids this issue.

```bash
brew install protobuf
```

### Set macOS Environment Variables

> **Required for macOS (especially ARM64/Apple Silicon)**. These environment variables fix C++ compilation issues with RocksDB and protobuf dependencies.

```bash
# Fix for macOS ARM64 C++ compilation (RocksDB, protobuf-src)
export MACOSX_DEPLOYMENT_TARGET=14.0
export SDKROOT=$(xcrun --show-sdk-path)
export CXXFLAGS="-isysroot $SDKROOT -I$SDKROOT/usr/include/c++/v1 -stdlib=libc++"
export PROTOC=/opt/homebrew/bin/protoc
```

> **Tip**: Add these to your shell profile (`~/.zshrc` or `~/.bashrc`) to avoid setting them every session.

### Install cargo-tangle CLI

```bash
# From the blueprint SDK repo root
cargo install cargo-tangle --path ./cli --force
```

---

## Key Accounts (Anvil Deterministic)

| Index | Address | Private Key | Role |
|-------|---------|-------------|------|
| 0 | `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` | `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` | Deployer |
| 1 | `0x70997970C51812dc3A010C7d01b50e0d17dc79C8` | `0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d` | Operator |
| 2 | `0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC` | `0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a` | User |

---

## Terminal Overview

This guide requires **4 terminals**:

| Terminal | Purpose | Steps |
|----------|---------|-------|
| Terminal 1 | Anvil (local blockchain) | Step 2 |
| Terminal 2 | HTTP server (artifact hosting) | Step 3 |
| Terminal 3 | CLI commands (deploy, register, submit) | Steps 4-10, 12-14 |
| Terminal 4 | Blueprint manager (operator runtime) | Step 11 |

> **Important**: Terminal 3 holds all environment variables (`$TANGLE`, `$RESTAKING`, `$STATUS_REGISTRY`, `$BLUEPRINT_ID`, `$REQUEST_ID`, `$SERVICE_ID`, `$CALL_ID`). Steps 4-14 (except Step 11) must run in this terminal.

---

## Expected Directory Structure

After setup, your directories should look like this:

```
parent-directory/
├── blueprint/          # Blueprint SDK repo (you're here)
├── tnt-core/           # Tangle contracts repo (clone from GitHub)
└── hello-blueprint/    # Created in Step 1
```

The `cd ../hello-blueprint` commands assume this sibling structure. If your directories are elsewhere, adjust paths accordingly.

---

## Step 1: Create a New Blueprint

> **Important**: Create the blueprint **outside** the blueprint SDK repository to avoid workspace conflicts. This step can be done in any terminal before opening the 4 dedicated terminals.

```bash
# Navigate to parent directory (outside the blueprint SDK repo)
# From the blueprint SDK directory:
cd ..

# Create a new blueprint using the CLI
# Note: --skip-prompts requires --project-description and --project-authors
cargo tangle blueprint create \
  --name hello-blueprint \
  --skip-prompts \
  --project-description "Hello Blueprint for local E2E testing" \
  --project-authors "Tangle"

# Navigate to the created blueprint
cd hello-blueprint
```

> **Note**: The CLI may show an error about "No [package] section found in Cargo.toml" - this is a known issue because the template generates a workspace. The blueprint is still created successfully and this error can be ignored.

### Fix Rust Version (if needed)

The generated blueprint template may specify an older Rust version. If you encounter compilation errors about missing features, update the Rust version:

```bash
# Check current Rust version in the template
cat rust-toolchain.toml

# If it shows channel = "1.86" or older, update to 1.88
sed -i '' 's/channel = "1.86"/channel = "1.88"/' rust-toolchain.toml
# On Linux: sed -i 's/channel = "1.86"/channel = "1.88"/' rust-toolchain.toml
```

### Use Local Blueprint SDK (Required for Local Testing)

The generated blueprint template uses the remote git repository for `blueprint-sdk`. For local E2E testing against your local SDK changes, you **must** update the Cargo.toml files to use local path dependencies instead.

Update `hello-blueprint-bin/Cargo.toml`:
```bash
# macOS:
sed -i '' 's|blueprint-sdk = { git = "https://github.com/tangle-network/blueprint", branch = "v2"|blueprint-sdk = { path = "../../blueprint/crates/sdk"|' hello-blueprint-bin/Cargo.toml

# Linux:
# sed -i 's|blueprint-sdk = { git = "https://github.com/tangle-network/blueprint", branch = "v2"|blueprint-sdk = { path = "../../blueprint/crates/sdk"|' hello-blueprint-bin/Cargo.toml
```

Update `hello-blueprint-lib/Cargo.toml`:
```bash
# macOS:
sed -i '' 's|blueprint-sdk = { git = "https://github.com/tangle-network/blueprint", branch = "v2"|blueprint-sdk = { path = "../../blueprint/crates/sdk"|' hello-blueprint-lib/Cargo.toml
sed -i '' 's|blueprint-anvil-testing-utils = { git = "https://github.com/tangle-network/blueprint", branch = "v2" }|blueprint-anvil-testing-utils = { path = "../../blueprint/crates/testing-utils/anvil" }|' hello-blueprint-lib/Cargo.toml

# Linux:
# sed -i 's|blueprint-sdk = { git = "https://github.com/tangle-network/blueprint", branch = "v2"|blueprint-sdk = { path = "../../blueprint/crates/sdk"|' hello-blueprint-lib/Cargo.toml
# sed -i 's|blueprint-anvil-testing-utils = { git = "https://github.com/tangle-network/blueprint", branch = "v2" }|blueprint-anvil-testing-utils = { path = "../../blueprint/crates/testing-utils/anvil" }|' hello-blueprint-lib/Cargo.toml
```

> **Why is this needed?** The `--protocol tangle-evm` argument requires an alias that may not be in the remote v2 branch yet. Using local path dependencies ensures your blueprint binary uses the same SDK version as your local CLI, avoiding protocol parsing mismatches.

The created blueprint structure:
```
hello-blueprint/
├── Cargo.toml                    # Workspace root
├── rust-toolchain.toml           # Rust version (update to 1.88 if needed)
├── hello-blueprint-lib/          # Library crate (job logic)
│   ├── Cargo.toml
│   └── src/lib.rs                # Hello job implementation
└── hello-blueprint-bin/          # Binary crate (runtime executable)
    ├── Cargo.toml
    └── src/main.rs
```

The generated `hello-blueprint-lib/src/lib.rs` contains:

```rust
// ABI types for on-chain interaction
sol! {
    struct HelloRequest {
        string name;
    }

    struct HelloResponse {
        string message;
        string operator;
    }
}

// Job handler
pub async fn hello(
    Caller(caller): Caller,
    TangleEvmArg(request): TangleEvmArg<HelloRequest>,
) -> TangleEvmResult<HelloResponse> {
    let caller_address = Address::from_slice(&caller);
    let message = format!("Hello, {}!", request.name);

    TangleEvmResult(HelloResponse {
        message,
        operator: format!("{caller_address:#x}"),
    })
}

// Router exposing job ID 0
pub fn router() -> Router {
    Router::new().route(HELLO_JOB_ID, hello.layer(TangleEvmLayer))
}
```

---

## Step 2: Start Anvil

> **Important**: Use `--code-size-limit 50000` to allow deployment of the Tangle facet contracts, which exceed the default 24KB EVM code size limit (EIP-170).

```bash
# Terminal 1
anvil --host 0.0.0.0 --port 8545 --base-fee 0 --gas-price 0 --gas-limit 100000000 --hardfork cancun --code-size-limit 50000
```

---

## Step 3: Package and Serve Blueprint Artifacts (Manual)

> **Note**: Packaging is done manually for local testing. In production, use GitHub Actions with cargo-dist to automate this.

### Production vs Local: How They Align

In production, the **blueprint developer** publishes artifacts before deploying to the chain. When they push a git tag (e.g., `v1.0.0`), GitHub Actions with cargo-dist automatically:

1. Builds binaries for multiple platforms (Linux, macOS, Windows)
2. Creates compressed archives and computes SHA256 hashes
3. Uploads everything to GitHub Releases
4. Generates `dist-manifest.json` with artifact metadata

Operators then download these artifacts when running `cargo tangle blueprint run`.

The manual steps below simulate this production flow locally:

| Local Command | Production Equivalent |
|---------------|----------------------|
| `cargo build --release` | GitHub Actions matrix build |
| `tar -cJf ...` | cargo-dist archive creation |
| `shasum -a 256 ...` | cargo-dist checksum generation |
| `definition.json` creation | Committed to repo or generated |
| `dist.json` creation | cargo-dist `dist-manifest.json` |
| `python3 -m http.server` | GitHub Releases / IPFS hosting |

The key difference is the URL: locally we use `http://localhost:8081/...`, while production uses `https://github.com/.../releases/...` or IPFS URLs.

```bash
# Terminal 2
# Navigate to the hello-blueprint directory you created in Step 1
# If hello-blueprint is a sibling of the blueprint SDK:
cd /path/to/hello-blueprint  # Update this path to your hello-blueprint location

# Ensure macOS environment variables are set (if on macOS)
export MACOSX_DEPLOYMENT_TARGET=14.0
export SDKROOT=$(xcrun --show-sdk-path)
export CXXFLAGS="-isysroot $SDKROOT -I$SDKROOT/usr/include/c++/v1 -stdlib=libc++"
export PROTOC=/opt/homebrew/bin/protoc

# 1. Build the binary (note: binary name is hello-blueprint)
cargo build --release -p hello-blueprint-bin

# 2. Create dist directory and package the binary
mkdir -p ./dist
tar -cJf ./dist/hello-blueprint.tar.xz -C ../target/release hello-blueprint

# 3. Compute SHA256 hash of the BINARY (not the archive!)
# The hash must be of the extracted binary, not the archive itself
# macOS:
SHA256=$(shasum -a 256 ../target/release/hello-blueprint | awk '{print $1}')
# Linux:
# SHA256=$(sha256sum ../target/release/hello-blueprint | awk '{print $1}')

echo "Binary SHA256: $SHA256"

# 4. Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
case "$ARCH" in
  x86_64) ARCH="amd64" ;;
  aarch64|arm64) ARCH="arm64" ;;
esac

# 5. Create definition.json
# Note: Using port 8081 as 8080 is often used by Docker
cat > ./dist/definition.json << EOF
{
  "metadata_uri": "http://localhost:8081/metadata.json",
  "manager": "0x0000000000000000000000000000000000000000",
  "metadata": {
    "name": "hello-blueprint",
    "description": "Hello Blueprint for local development",
    "author": "Tangle",
    "category": "Test",
    "license": "MIT"
  },
  "jobs": [
    {
      "name": "hello",
      "description": "Greets the caller with a personalized message",
      "params_schema": "0x",
      "result_schema": "0x"
    }
  ],
  "sources": [{
    "kind": "native",
    "fetcher": "http",
    "entrypoint": "./hello-blueprint",
    "remote": {
      "dist_url": "http://localhost:8081/dist.json",
      "archive_url": "http://localhost:8081/hello-blueprint.tar.xz"
    },
    "binaries": [{
      "name": "hello-blueprint",
      "arch": "$ARCH",
      "os": "$OS",
      "sha256": "$SHA256"
    }]
  }],
  "supported_memberships": ["dynamic"]
}
EOF

# 6. Create dist.json (cargo-dist manifest format)
cat > ./dist/dist.json << EOF
{
  "dist_version": "0.0.0",
  "announcement_title": "",
  "announcement_changelog": "",
  "announcement_github_body": "",
  "announcement_is_prerelease": false,
  "releases": [],
  "artifacts": {
    "hello-blueprint.tar.xz": {
      "name": "hello-blueprint.tar.xz",
      "kind": "executable-zip",
      "target_triples": ["${ARCH}-apple-darwin"],
      "assets": [
        {
          "name": "hello-blueprint",
          "kind": "executable"
        }
      ],
      "checksum": "$SHA256"
    }
  }
}
EOF

# 7. Start HTTP server to serve artifacts (using port 8081)
cd ./dist
python3 -m http.server 8081
```

**What this does:**
- Builds `hello-blueprint` binary in release mode
- Creates `./dist/hello-blueprint.tar.xz` archive
- Computes SHA256 hash for integrity verification
- Generates `./dist/definition.json` pointing to `http://localhost:8081`
- Starts HTTP server on port 8081 to serve the artifacts

---

## Step 4: Deploy Contracts

> **Important**: Use `--disable-code-size-limit` to bypass EVM's 24KB contract size limit. This only works locally with Anvil's `--code-size-limit` flag. The Tangle facet contracts exceed the standard limit and will need optimization before mainnet deployment.

```bash
# Terminal 3 - This terminal will be used for Steps 4-10 and 12-14
# Navigate to the tnt-core repo (contracts repo)
cd /path/to/tnt-core  # Update this path to your tnt-core location

forge script script/v2/DeployContractsOnly.s.sol:DeployContractsOnly \
  --rpc-url http://127.0.0.1:8545 \
  --broadcast \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
  --disable-code-size-limit \
  -vvv

# Save addresses (deterministic with Anvil)
# IMPORTANT: Keep this terminal open - these variables are needed for Steps 5-14
export TANGLE=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
export RESTAKING=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
export STATUS_REGISTRY=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf
```

---

## Step 5: Setup Operator Keystore

> **Note**: Continue in Terminal 3 (same terminal as Step 4) to preserve the environment variables.

```bash
# Navigate back to hello-blueprint from tnt-core
# Adjust path based on your directory structure (tnt-core and hello-blueprint are siblings)
cd ../hello-blueprint

mkdir -p ./operator-keystore

cargo tangle key import \
  --key-type ecdsa \
  --secret 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \
  --keystore-path ./operator-keystore \
  --protocol tangle-evm
```

---

## Step 6: Register as Restaking Operator

> **Note**: Continue in Terminal 3. Use `cast` directly to register as a restaking operator.

```bash
# Register as restaking operator with 2 ETH stake
cast send $RESTAKING "registerOperator()" \
  --value 2ether \
  --private-key 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \
  --rpc-url http://127.0.0.1:8545

# Verify registration
cast call $RESTAKING "isOperator(address)(bool)" 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 --rpc-url http://127.0.0.1:8545
# Expected: true
```

---

## Step 7: Deploy Blueprint On-Chain

> **Note**: Continue in Terminal 3.

First, create a `settings.env` file with all required variables:

```bash
cat > ./settings.env << EOF
# Tangle EVM Settings for Local Development
HTTP_RPC_URL=http://127.0.0.1:8545
WS_RPC_URL=ws://127.0.0.1:8546
KEYSTORE_PATH=./operator-keystore
BLUEPRINT_KEYSTORE_URI=./operator-keystore
TANGLE_CONTRACT=$TANGLE
RESTAKING_CONTRACT=$RESTAKING
STATUS_REGISTRY_CONTRACT=$STATUS_REGISTRY
# For deploy command - will be replaced after blueprint is created
BLUEPRINT_ID=0
SERVICE_ID=0
EOF
```

Then deploy:

```bash
cargo tangle blueprint deploy tangle \
  --network testnet \
  --definition ./dist/definition.json \
  --settings-file ./settings.env

# Expected output:
# Blueprint sources:
#   [1] kind: native, fetcher: http, entrypoint: ./hello-blueprint
#
# Deploying blueprint definition (metadata http://localhost:8081/metadata.json) to testnet
# Deployment complete → network=testnet blueprint=0 service=-
# Submitted transactions:
#   tx=0x... block=Some(N) success=true

export BLUEPRINT_ID=0
```

> **Note**: The settings file requires both `KEYSTORE_PATH` and `BLUEPRINT_KEYSTORE_URI` pointing to your keystore. The `BLUEPRINT_ID=0` is required even for initial deploy (it's ignored for new blueprint creation).

---

## Step 8: Register Operator for Blueprint

> **Note**: Continue in Terminal 3.

```bash
cargo tangle blueprint register \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --rpc-endpoint "http://localhost:9000"

# Expected output:
# Registering operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
# Registration: submitted tx_hash=0x...
# Registration: confirmed block=Some(N) gas_used=347172
# Operator ready: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
```

---

## Step 9: Setup User Keystore

> **Note**: Continue in Terminal 3.

```bash
mkdir -p ./user-keystore

cargo tangle key import \
  --key-type ecdsa \
  --secret 5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a \
  --keystore-path ./user-keystore \
  --protocol tangle-evm
```

---

## Step 10: Request Service

> **Note**: Continue in Terminal 3.

```bash
cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --ttl 3600

# Expected output:
# Service request: submitted tx_hash=0x...
# Service request: confirmed block=Some(N) gas_used=244760
# Request ID: 0

export REQUEST_ID=0
```

---

## Step 11: Run the Operator

> **CRITICAL**: The manager must be running **before** the service is approved. The manager only processes **new** `ServiceActivated` events. If you approve first, the manager won't see the event and won't start the blueprint binary. If you need to restart the manager, you must request and approve a new service.

```bash
# Terminal 4 (or background)
# Note: Use -d to specify a data directory to avoid RocksDB lock conflicts
cargo tangle blueprint run \
  -p tangle-evm \
  -k ./operator-keystore \
  -f ./settings.env \
  -d ./data

# Expected output:
# Starting blueprint manager for blueprint ID: 0
# Preparing Blueprint to run, this may take a few minutes...
# Starting blueprint execution...
# Blueprint is running. Press Ctrl+C to stop.
```

> **Troubleshooting**: If you see `IO error: While lock file: .../LOCK: Resource temporarily unavailable`, use a different data directory with `-d ./data-alt`.

This:
- Fetches binary from `http://localhost:8081/hello-blueprint.tar.xz`
- Verifies SHA256 hash
- Extracts and runs the binary (which uses `BlueprintRunner` with producer/consumer)
- The binary listens for `JobSubmitted` events and submits results back

---

## Step 12: Approve Service (as Operator)

> **Important**: Return to Terminal 3 (not Terminal 4 where the manager is running). The manager must stay running while you approve the service.

```bash
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id $REQUEST_ID \
  --restaking-percent 100

# Expected output:
# Service approval: submitted tx_hash=0x...
# Service approval: confirmed block=Some(N) gas_used=377312

export SERVICE_ID=0
```

After approval, you should see in the manager terminal (Terminal 4):
```
ServiceActivated event received for service 0
Spawning blueprint binary for service 0...
```

---

## Step 13: Submit a Job

> **Note**: Continue in Terminal 3.

The `hello` job expects a `HelloRequest` struct with a `name` field (string type).

```bash
# Encode HelloRequest { name: "Alice" }
# The struct is ABI-encoded as: (string)
PAYLOAD=$(cast abi-encode "f((string))" "(Alice)")
# Payload will be the ABI-encoded tuple with the string

echo "Payload: $PAYLOAD"

cargo tangle blueprint jobs submit \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --service-id $SERVICE_ID \
  --job 0 \
  --payload-hex $PAYLOAD

# Expected output:
# Job submission: submitted tx_hash=0x...
# Job submission: confirmed block=Some(N) gas_used=159424
# Submitted job 0 to service 0. Call ID: 0 (tx: 0x...)

export CALL_ID=0
```

---

## Step 14: Watch for Result

> **Note**: Continue in Terminal 3.

```bash
cargo tangle blueprint jobs watch \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --service-id $SERVICE_ID \
  --call-id $CALL_ID \
  --timeout-secs 120

# Expected output: ABI-encoded HelloResponse { message: "Hello, Alice!", operator: "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc" }
```

To decode the result:
```bash
# Replace <result> with the hex output from the watch command
cast abi-decode "f()((string,string))" <result>
# Expected: ("Hello, Alice!", "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc")
```

> **VERIFIED:** All steps work correctly when followed in order.

---

## Important Notes

### Binary Hash Must Match

The `sha256` field in `definition.json` must be the hash of the **extracted binary**, not the archive:
```bash
# Correct: hash of the binary inside the archive
tar -xf archive.tar.xz && shasum -a 256 binary-name

# Wrong: hash of the archive itself
shasum -a 256 archive.tar.xz
```

### dist.json Format

The `dist.json` must follow cargo-dist manifest format:
```json
{
  "dist_version": "0.0.0",
  "artifacts": {
    "binary-name.tar.xz": {
      "kind": "executable-zip",
      "assets": [{ "name": "binary-name", "kind": "executable" }]
    }
  }
}
```

### Protocol Naming Convention

The protocol argument accepts both formats:
```bash
cargo tangle blueprint run -p tangle-evm ...
# or
cargo tangle blueprint run -p tangleevm ...
```

Both are parsed correctly. The `tangle-evm` form is canonical (returned by `Protocol::as_str()`) and recommended.

### Contract Size Limit (Production Blocker)

The Tangle facet contracts exceed EVM's 24KB code size limit (EIP-170):

| Contract | Size | Over Limit |
|----------|------|------------|
| TangleBlueprintsFacet | 24,976 bytes | +400 bytes |
| TangleServicesLifecycleFacet | 25,216 bytes | +640 bytes |
| TangleJobsAggregationFacet | 25,265 bytes | +689 bytes |

The `--disable-code-size-limit` flag only works locally. Before mainnet deployment, contracts need:
1. Split into smaller sub-facets
2. Logic moved to external libraries
3. Bytecode optimization (shorter error strings, custom errors)

---

## Comparison: Local vs Mainnet

| Step | Local | Mainnet |
|------|-------|---------|
| Create blueprint | `cargo tangle blueprint create` | Same |
| Build binary | `cargo build --release` | Same |
| Package | Manual (`tar`, `sha256sum`) | Same (or future CLI) |
| Publish artifacts | `python3 -m http.server` (localhost) | Push to GitHub/IPFS |
| Definition URL | `http://localhost:8081/...` | `https://github.com/.../releases/...` |
| Deploy contracts | `forge script` (one-time) | Already deployed |
| Deploy blueprint | `cargo tangle blueprint deploy` | Same |
| Register operator | `cargo tangle blueprint register` | Same |
| Request service | `cargo tangle blueprint service request` | Same |
| Approve service | `cargo tangle blueprint service approve` | Same |
| Run operator | `cargo tangle blueprint run` | Same |
| Submit job | `cargo tangle blueprint jobs submit` | Same |

The **only difference** is artifact hosting (localhost vs production URL). All CLI commands are identical.

---

## CLI Commands Summary

| Step | Command | Status |
|------|---------|--------|
| Create blueprint | `cargo tangle blueprint create --name <name> --skip-prompts --project-description "..." --project-authors "..."` | Works |
| Navigate to blueprint | `cd <name>` | N/A (manual) |
| Package blueprint | Manual steps (see Step 3) | N/A (use GitHub/cargo-dist in production) |
| Import keys | `cargo tangle key import` | Works |
| Register restaking | `cast send` (see Step 6) | N/A (use `cast` directly) |
| Deploy blueprint | `cargo tangle blueprint deploy tangle` | Works |
| Register operator | `cargo tangle blueprint register` | Works |
| Request service | `cargo tangle blueprint service request` | Works |
| Approve service | `cargo tangle blueprint service approve` | Works |
| Run operator | `cargo tangle blueprint run` | Works |
| Submit job | `cargo tangle blueprint jobs submit` | Works |
| Watch result | `cargo tangle blueprint jobs watch` | Works |

---

## Troubleshooting

### RocksDB Lock Error

```
IO error: While lock file: .../data/private/auth-proxy/db/LOCK: Resource temporarily unavailable
```

**Solution**: Use a different data directory:
```bash
cargo tangle blueprint run ... -d ./data-alt
```

### Manager Didn't Detect ServiceActivated Event

If the manager was restarted after the service was approved, it won't detect the existing active service.

**Solution**: Request and approve a new service while the manager is running:
```bash
# With manager running in another terminal:
cargo tangle blueprint service request ... --blueprint-id 0 --operator 0x70997970... --ttl 3600
# Note the new REQUEST_ID
cargo tangle blueprint service approve ... --request-id <new_request_id> --restaking-percent 100
# Use the new SERVICE_ID for job submission
```

### macOS C++ Compilation Failures

If you see errors like:
- `fatal error: 'cstdlib' not found`
- `ld: library 'atomic' not found`

**Solution**: Set the macOS environment variables (see Prerequisites section).

### Rust Version Mismatch

If compilation fails with missing features, update `rust-toolchain.toml` to use Rust 1.88.

---

## Cleanup

```bash
pkill -f "anvil"
pkill -f "python.*http.server"  # if using python for serving
cd hello-blueprint
rm -rf ./operator-keystore ./user-keystore ./dist ./data ./data-alt

# Remove the created blueprint directory
cd ..
rm -rf hello-blueprint

# Optionally clean up built artifacts
# cargo clean
```
