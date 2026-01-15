# Local End-to-End Test Plan (Clean Slate)

This document describes how to test the complete blueprint lifecycle locally **from scratch** using the production-like CLI flow. This approach matches the testnet/mainnet workflow exactly and is recommended for integration testing.

This approach mirrors exactly what you'd do on testnet/mainnet:
1. Create a blueprint using the CLI
2. Build and package your blueprint binary
3. Serve artifacts locally (simulates GitHub/IPFS)
4. Deploy using the same CLI commands as production

### Prerequisites

```bash
# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
foundryup

# Install cargo-tangle CLI (from this repo)
cargo install cargo-tangle --path ./cli --force

# macOS C++ fix
export SDKROOT=$(xcrun --show-sdk-path)
export CXXFLAGS="-isysroot $SDKROOT -I$SDKROOT/usr/include/c++/v1 -stdlib=libc++"
```

### Key Accounts (Anvil Deterministic)

| Index | Address | Private Key | Role |
|-------|---------|-------------|------|
| 0 | `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` | `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` | Deployer |
| 1 | `0x70997970C51812dc3A010C7d01b50e0d17dc79C8` | `0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d` | Operator |
| 2 | `0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC` | `0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a` | User |

### Step 1: Create a New Blueprint

> **Important**: Create the blueprint **outside** the blueprint SDK repository to avoid workspace conflicts. Navigate to a sibling directory first.

```bash
# Navigate to parent directory (outside the blueprint SDK repo)
cd ..

# Create a new blueprint using the CLI
cargo tangle blueprint create --name hello-blueprint --skip-prompts

# Navigate to the created blueprint
cd hello-blueprint
```

> **Note**: The CLI may show an error about "No [package] section found in Cargo.toml" - this is a known issue because the template generates a workspace. The blueprint is still created successfully and this error can be ignored.

The created blueprint structure:
```
hello-blueprint/
├── Cargo.toml                    # Workspace root
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

### Step 2: Start Anvil

```bash
# Terminal 1
anvil --host 0.0.0.0 --port 8545 --base-fee 0 --gas-price 0 --gas-limit 100000000 --hardfork cancun
```

### Step 3: Package and Serve Blueprint Artifacts (Manual)

> **Note**: Packaging is done manually for local testing. In production, use GitHub Actions with cargo-dist to automate this.

```bash
# Terminal 2
cd hello-blueprint

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

### Step 4: Deploy Contracts

```bash
# Terminal 3
# Navigate to the tnt-core repo (contracts repo)
cd /path/to/tnt-core  # Update this path to your tnt-core location

forge script script/v2/DeployContractsOnly.s.sol:DeployContractsOnly \
  --rpc-url http://127.0.0.1:8545 --broadcast -vvv

# Save addresses (deterministic with Anvil)
export TANGLE=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
export RESTAKING=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
export STATUS_REGISTRY=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf
```

### Step 5: Setup Operator Keystore

```bash
cd hello-blueprint
mkdir -p ./operator-keystore

cargo tangle key import \
  --key-type ecdsa \
  --secret 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \
  --keystore-path ./operator-keystore \
  --protocol tangleevm
```

### Step 6: Register as Restaking Operator

> **Note**: Use `cast` directly to register as a restaking operator.

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

### Step 7: Deploy Blueprint On-Chain

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

### Step 8: Register Operator for Blueprint

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

### Step 9: Setup User Keystore

```bash
cd hello-blueprint
mkdir -p ./user-keystore

cargo tangle key import \
  --key-type ecdsa \
  --secret 5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a \
  --keystore-path ./user-keystore \
  --protocol tangleevm
```

### Step 10: Request Service

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

### Step 11: Run the Operator

> **Important**: The manager must be running *before* the service is approved. The manager only processes new `ServiceActivated` events, so if you approve first, it won't see the event and won't start the blueprint binary.

```bash
# Terminal 4 (or background)
# Note: The run command uses --settings-file instead of individual contract addresses
cargo tangle blueprint run \
  -p tangleevm \
  -k ./operator-keystore \
  -f ./settings.env

# Expected output:
# Starting blueprint manager for blueprint ID: 0
# Preparing Blueprint to run, this may take a few minutes...
# Starting blueprint execution...
# Blueprint is running. Press Ctrl+C to stop.
```

This:
- Fetches binary from `http://localhost:8081/hello-blueprint.tar.xz`
- Verifies SHA256 hash
- Extracts and runs the binary (which uses `BlueprintRunner` with producer/consumer)
- The binary listens for `JobSubmitted` events and submits results back

### Step 12: Approve Service (as Operator)

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

### Step 13: Submit a Job

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

### Step 14: Watch for Result

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

# Expected output: ABI-encoded HelloResponse { message: "Hello, Alice!", operator: "0x70997970..." }
# Decode with: cast abi-decode "f((string,string))" <result>
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

The protocol argument uses `tangleevm` (lowercase, no hyphens):
```bash
cargo tangle blueprint run -p tangleevm ...
```

The CLI accepts the following equivalent values for the protocol:
- `tangleevm` (canonical)
- `tangle-evm`
- `tangle_evm`

All three are parsed correctly, but `tangleevm` is the canonical form used internally and in environment variables (`PROTOCOL=tangleevm`).

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
| Create blueprint | `cargo tangle blueprint create --name <name>` | Works |
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

## Cleanup

```bash
pkill -f "anvil"
pkill -f "python.*http.server"  # if using python for serving
cd hello-blueprint
rm -rf ./operator-keystore ./user-keystore ./dist

# Remove the created blueprint directory
cd ..
rm -rf hello-blueprint

# Optionally clean up built artifacts
# cargo clean
```
