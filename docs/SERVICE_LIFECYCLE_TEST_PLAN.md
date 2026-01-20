# Service Lifecycle Commands Test Plan

This document provides a comprehensive test plan for all service lifecycle commands in `cargo-tangle`. It includes a setup script to quickly prepare the test environment and detailed test cases for each command.

**Target Commands:**
1. `service request` - Request a new service instance
2. `service approve` - Approve a pending service request
3. `service reject` - Reject a pending service request
4. `service join` - Join an existing dynamic service
5. `service leave` - Leave a dynamic service
6. `service spawn` - Manually spawn a service runtime
7. `service list` - List all active services
8. `service list-requests` - List all pending service requests
9. `service show-request` - Display details of a specific request

**Last Updated:** 2026-01-20

---

## Prerequisites

### Required Software
- Rust 1.88+ (check with `rustc --version`)
- Foundry (anvil, cast, forge)
- Python 3 (for HTTP server)
- jq (for JSON parsing)

### Install Foundry
```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

### Install cargo-tangle CLI
```bash
cd /path/to/blueprint
cargo install cargo-tangle --path ./cli --force
```

### macOS Environment Variables (if on macOS)
```bash
export MACOSX_DEPLOYMENT_TARGET=14.0
export SDKROOT=$(xcrun --show-sdk-path)
export CXXFLAGS="-isysroot $SDKROOT -I$SDKROOT/usr/include/c++/v1 -stdlib=libc++"
export PROTOC=/opt/homebrew/bin/protoc
```

---

## Directory Structure

After setup, your directories should look like:
```
parent-directory/
├── blueprint/              # Blueprint SDK repo
├── tnt-core/               # Tangle contracts repo
└── service-lifecycle-test/ # Test workspace (created by setup script)
    ├── setup.sh            # Setup automation script
    ├── operator-keystore/  # Operator keys
    ├── user-keystore/      # User keys
    ├── operator2-keystore/ # Second operator (for multi-operator tests)
    ├── settings.env        # Environment configuration
    └── dist/               # Blueprint artifacts
```

---

## Terminal Overview

This test plan requires **4 terminals**:

| Terminal | Purpose | Steps |
|----------|---------|-------|
| Terminal 1 | Anvil (local blockchain) | Setup Phase |
| Terminal 2 | HTTP server (artifact hosting) | Setup Phase |
| Terminal 3 | CLI commands (all service lifecycle tests) | All Test Sections |
| Terminal 4 | Blueprint manager (operator runtime) | Dynamic service tests |

---

## Quick Setup Script

Save this script as `setup-service-lifecycle-test.sh` in your parent directory:

```bash
#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== Service Lifecycle Test Setup ===${NC}"

# Configuration
BLUEPRINT_SDK_PATH="${BLUEPRINT_SDK_PATH:-../blueprint}"
TNT_CORE_PATH="${TNT_CORE_PATH:-../tnt-core}"
TEST_DIR="service-lifecycle-test"
HTTP_PORT=8081

# Validate paths
if [ ! -d "$BLUEPRINT_SDK_PATH" ]; then
    echo -e "${RED}Error: Blueprint SDK not found at $BLUEPRINT_SDK_PATH${NC}"
    echo "Set BLUEPRINT_SDK_PATH environment variable to the correct path"
    exit 1
fi

if [ ! -d "$TNT_CORE_PATH" ]; then
    echo -e "${RED}Error: tnt-core not found at $TNT_CORE_PATH${NC}"
    echo "Set TNT_CORE_PATH environment variable to the correct path"
    exit 1
fi

# Create test directory
echo -e "${YELLOW}Creating test directory...${NC}"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Step 1: Create blueprint
echo -e "${YELLOW}Step 1: Creating test blueprint...${NC}"
cargo tangle blueprint create \
  --name svc-test-blueprint \
  --skip-prompts \
  --project-description "Service Lifecycle Test Blueprint" \
  --project-authors "Tangle"

cd svc-test-blueprint

# Fix rust toolchain version
sed -i '' 's/channel = "1.86"/channel = "1.88"/' rust-toolchain.toml 2>/dev/null || \
sed -i 's/channel = "1.86"/channel = "1.88"/' rust-toolchain.toml

# Update to use local SDK
BLUEPRINT_SDK_ABS=$(cd "$BLUEPRINT_SDK_PATH" && pwd)
sed -i '' "s|blueprint-sdk = { git = \"https://github.com/tangle-network/blueprint\", branch = \"v2\"|blueprint-sdk = { path = \"$BLUEPRINT_SDK_ABS/crates/sdk\"|" svc-test-blueprint-bin/Cargo.toml 2>/dev/null || \
sed -i "s|blueprint-sdk = { git = \"https://github.com/tangle-network/blueprint\", branch = \"v2\"|blueprint-sdk = { path = \"$BLUEPRINT_SDK_ABS/crates/sdk\"|" svc-test-blueprint-bin/Cargo.toml

sed -i '' "s|blueprint-sdk = { git = \"https://github.com/tangle-network/blueprint\", branch = \"v2\"|blueprint-sdk = { path = \"$BLUEPRINT_SDK_ABS/crates/sdk\"|" svc-test-blueprint-lib/Cargo.toml 2>/dev/null || \
sed -i "s|blueprint-sdk = { git = \"https://github.com/tangle-network/blueprint\", branch = \"v2\"|blueprint-sdk = { path = \"$BLUEPRINT_SDK_ABS/crates/sdk\"|" svc-test-blueprint-lib/Cargo.toml

sed -i '' "s|blueprint-anvil-testing-utils = { git = \"https://github.com/tangle-network/blueprint\", branch = \"v2\" }|blueprint-anvil-testing-utils = { path = \"$BLUEPRINT_SDK_ABS/crates/testing-utils/anvil\" }|" svc-test-blueprint-lib/Cargo.toml 2>/dev/null || \
sed -i "s|blueprint-anvil-testing-utils = { git = \"https://github.com/tangle-network/blueprint\", branch = \"v2\" }|blueprint-anvil-testing-utils = { path = \"$BLUEPRINT_SDK_ABS/crates/testing-utils/anvil\" }|" svc-test-blueprint-lib/Cargo.toml

# Step 2: Build blueprint
echo -e "${YELLOW}Step 2: Building blueprint...${NC}"
cargo build --release -p svc-test-blueprint-bin

# Step 3: Package artifacts
echo -e "${YELLOW}Step 3: Packaging artifacts...${NC}"
mkdir -p ./dist

# Create archive
tar -cJf ./dist/svc-test-blueprint.tar.xz -C ../../target/release svc-test-blueprint

# Compute SHA256 of binary
if [[ "$OSTYPE" == "darwin"* ]]; then
    SHA256=$(shasum -a 256 ../../target/release/svc-test-blueprint | awk '{print $1}')
else
    SHA256=$(sha256sum ../../target/release/svc-test-blueprint | awk '{print $1}')
fi

echo "Binary SHA256: $SHA256"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
case "$ARCH" in
  x86_64) ARCH="amd64" ;;
  aarch64|arm64) ARCH="arm64" ;;
esac

# Create definition.json
cat > ./dist/definition.json << EOF
{
  "metadata_uri": "http://localhost:$HTTP_PORT/metadata.json",
  "manager": "0x0000000000000000000000000000000000000000",
  "metadata": {
    "name": "svc-test-blueprint",
    "description": "Service Lifecycle Test Blueprint",
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
    "entrypoint": "./svc-test-blueprint",
    "remote": {
      "dist_url": "http://localhost:$HTTP_PORT/dist.json",
      "archive_url": "http://localhost:$HTTP_PORT/svc-test-blueprint.tar.xz"
    },
    "binaries": [{
      "name": "svc-test-blueprint",
      "arch": "$ARCH",
      "os": "$OS",
      "sha256": "$SHA256"
    }]
  }],
  "supported_memberships": ["dynamic"]
}
EOF

# Create dist.json
cat > ./dist/dist.json << EOF
{
  "dist_version": "0.0.0",
  "announcement_title": "",
  "announcement_changelog": "",
  "announcement_github_body": "",
  "announcement_is_prerelease": false,
  "releases": [],
  "artifacts": {
    "svc-test-blueprint.tar.xz": {
      "name": "svc-test-blueprint.tar.xz",
      "kind": "executable-zip",
      "target_triples": ["${ARCH}-apple-darwin"],
      "assets": [
        {
          "name": "svc-test-blueprint",
          "kind": "executable"
        }
      ],
      "checksum": "$SHA256"
    }
  }
}
EOF

# Step 4: Setup keystores
echo -e "${YELLOW}Step 4: Setting up keystores...${NC}"

# Operator 1 keystore (Anvil account 1)
mkdir -p ./operator-keystore
cargo tangle key import \
  --key-type ecdsa \
  --secret 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \
  --keystore-path ./operator-keystore \
  --protocol tangle-evm

# Operator 2 keystore (Anvil account 3)
mkdir -p ./operator2-keystore
cargo tangle key import \
  --key-type ecdsa \
  --secret 7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6 \
  --keystore-path ./operator2-keystore \
  --protocol tangle-evm

# User keystore (Anvil account 2)
mkdir -p ./user-keystore
cargo tangle key import \
  --key-type ecdsa \
  --secret 5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a \
  --keystore-path ./user-keystore \
  --protocol tangle-evm

# Go back to test dir root for contract deployment
cd ..

echo -e "${GREEN}=== Setup Complete ===${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Terminal 1: Start Anvil"
echo "   anvil --host 0.0.0.0 --port 8545 --base-fee 0 --gas-price 0 --gas-limit 100000000 --hardfork cancun --code-size-limit 50000"
echo ""
echo "2. Terminal 2: Start HTTP server"
echo "   cd $TEST_DIR/svc-test-blueprint/dist && python3 -m http.server $HTTP_PORT"
echo ""
echo "3. Terminal 3: Deploy contracts and run tests"
echo "   cd $TNT_CORE_PATH"
echo "   forge script script/v2/DeployContractsOnly.s.sol:DeployContractsOnly --rpc-url http://127.0.0.1:8545 --broadcast --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --disable-code-size-limit -vvv"
echo ""
echo "   Then export these addresses:"
echo "   export TANGLE=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"
echo "   export RESTAKING=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
echo "   export STATUS_REGISTRY=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf"
echo ""
echo "   And create settings.env in the blueprint directory"
```

---

## Phase 0: Environment Setup

### Step 0.1: Run Setup Script

```bash
# From parent directory containing blueprint/ and tnt-core/
chmod +x setup-service-lifecycle-test.sh
./setup-service-lifecycle-test.sh
```

### Step 0.2: Start Anvil (Terminal 1)

```bash
anvil --host 0.0.0.0 --port 8545 --base-fee 0 --gas-price 0 --gas-limit 100000000 --hardfork cancun --code-size-limit 50000
```

### Step 0.3: Start HTTP Server (Terminal 2)

```bash
cd service-lifecycle-test/svc-test-blueprint/dist
python3 -m http.server 8081
```

### Step 0.4: Deploy Contracts (Terminal 3)

```bash
cd /path/to/tnt-core

forge script script/v2/DeployContractsOnly.s.sol:DeployContractsOnly \
  --rpc-url http://127.0.0.1:8545 \
  --broadcast \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
  --disable-code-size-limit \
  -vvv

# Export contract addresses (deterministic with Anvil)
export TANGLE=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
export RESTAKING=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
export STATUS_REGISTRY=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf
```

### Step 0.5: Register Operators on Restaking Layer (Terminal 3)

```bash
# Operator 1: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
cast send $RESTAKING "registerOperator()" \
  --value 2ether \
  --private-key 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \
  --rpc-url http://127.0.0.1:8545

# Verify
cast call $RESTAKING "isOperator(address)(bool)" 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 --rpc-url http://127.0.0.1:8545
# Expected: true

# Operator 2: 0x90F79bf6EB2c4f870365E785982E1f101E93b906
cast send $RESTAKING "registerOperator()" \
  --value 2ether \
  --private-key 0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6 \
  --rpc-url http://127.0.0.1:8545

# Verify
cast call $RESTAKING "isOperator(address)(bool)" 0x90F79bf6EB2c4f870365E785982E1f101E93b906 --rpc-url http://127.0.0.1:8545
# Expected: true
```

### Step 0.6: Deploy Blueprint and Register Operators (Terminal 3)

```bash
cd /path/to/service-lifecycle-test/svc-test-blueprint

# Create settings.env
cat > ./settings.env << EOF
HTTP_RPC_URL=http://127.0.0.1:8545
WS_RPC_URL=ws://127.0.0.1:8546
KEYSTORE_PATH=./operator-keystore
BLUEPRINT_KEYSTORE_URI=./operator-keystore
TANGLE_CONTRACT=$TANGLE
RESTAKING_CONTRACT=$RESTAKING
STATUS_REGISTRY_CONTRACT=$STATUS_REGISTRY
BLUEPRINT_ID=0
SERVICE_ID=0
EOF

# Deploy blueprint
cargo tangle blueprint deploy tangle \
  --network testnet \
  --definition ./dist/definition.json \
  --settings-file ./settings.env

export BLUEPRINT_ID=0

# Register Operator 1
cargo tangle blueprint register \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --rpc-endpoint "http://localhost:9000"

# Register Operator 2
cargo tangle blueprint register \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator2-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --rpc-endpoint "http://localhost:9001"
```

---

## Phase 1: Basic Service Request/Approve Flow

### Test 1.1: Basic Service Request (Single Operator)

**Goal:** Verify basic service request with minimal parameters

```bash
# Terminal 3
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
# Service request: confirmed block=Some(N) gas_used=...
# Request ID: 0

export REQUEST_ID_1=0
```

**Verification:**
```bash
# Check request exists
cargo tangle blueprint service show --request-id $REQUEST_ID_1 \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING
```

### Test 1.2: Service Approve (Default Restaking)

**Goal:** Verify operator can approve with default 50% restaking

```bash
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id $REQUEST_ID_1

# Expected output:
# Service approval: submitted tx_hash=0x...
# Service approval: confirmed block=Some(N) gas_used=...

export SERVICE_ID_1=0
```

**Verification:**
```bash
# Check service was created
cargo tangle blueprint service list \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Should show service 0 in Active state
```

---

## Phase 2: Service Request Variations

### Test 2.1: Multi-Operator Service Request

**Goal:** Request service from multiple operators

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
  --operator 0x90F79bf6EB2c4f870365E785982E1f101E93b906 \
  --ttl 7200

export REQUEST_ID_2=1
```

### Test 2.2: Service Request with Operator Exposure

**Goal:** Request service with specific operator exposure values

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
  --operator 0x90F79bf6EB2c4f870365E785982E1f101E93b906 \
  --operator-exposure-bps 5000 \
  --operator-exposure-bps 7500 \
  --ttl 3600

export REQUEST_ID_3=2
```

### Test 2.3: Service Request with Permitted Callers

**Goal:** Request service with additional permitted callers

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
  --permitted-caller 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 \
  --ttl 3600

export REQUEST_ID_4=3
```

### Test 2.4: Service Request with Config Hex

**Goal:** Request service with hex-encoded configuration payload

```bash
# Create a simple config payload (ABI-encoded string "test-config")
CONFIG_HEX=$(cast abi-encode "f(string)" "test-config")

cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --config-hex $CONFIG_HEX \
  --ttl 3600

export REQUEST_ID_5=4
```

### Test 2.5: Service Request with Payment

**Goal:** Request service with ETH payment

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
  --payment-token 0x0000000000000000000000000000000000000000 \
  --payment-amount 1000000000000000000 \
  --ttl 3600

export REQUEST_ID_6=5
```

### Test 2.6: Service Request with TTL=0 (Never Expires)

**Goal:** Request service that never expires

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
  --ttl 0

export REQUEST_ID_7=6
```

### Test 2.7: Service Request with JSON Output

**Goal:** Verify JSON output format for automation

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
  --ttl 3600 \
  --json 2>&1 | tee request_output.json

# Verify JSON format
jq '.request_id' request_output.json
```

---

## Phase 3: Service Approve Variations

### Test 3.1: Approve with Custom Restaking Percentage

**Goal:** Approve with 100% restaking commitment

```bash
# First approve request 2 (multi-operator) with operator 1
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id $REQUEST_ID_2 \
  --restaking-percent 100
```

### Test 3.2: Second Operator Approves Same Request

**Goal:** Verify multi-operator approval workflow

```bash
# Approve request 2 with operator 2
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator2-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id $REQUEST_ID_2 \
  --restaking-percent 75

export SERVICE_ID_2=1
```

### Test 3.3: Approve with JSON Output

**Goal:** Verify JSON output format

```bash
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id $REQUEST_ID_4 \
  --json

export SERVICE_ID_4=3
```

---

## Phase 4: Service Reject

### Test 4.1: Basic Service Reject

**Goal:** Verify operator can reject a pending request

```bash
# Reject request 5 (the one with config)
cargo tangle blueprint service reject \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id $REQUEST_ID_5

# Expected output:
# Service rejection: submitted tx_hash=0x...
# Service rejection: confirmed block=Some(N) gas_used=...
```

### Test 4.2: Verify Rejected Request Cannot Be Approved

**Goal:** Ensure rejection is final

```bash
# Try to approve rejected request (should fail)
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id $REQUEST_ID_5

# Expected: Transaction should revert
```

### Test 4.3: Reject with JSON Output

**Goal:** Verify JSON output format for rejection

```bash
cargo tangle blueprint service reject \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id $REQUEST_ID_6 \
  --json
```

---

## Phase 5: Service Join (Dynamic Services)

### Test 5.1: Setup - Create and Approve a Dynamic Service

**Goal:** Create a service that allows dynamic membership

```bash
# Create a new request for dynamic service testing
cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --ttl 7200

export REQUEST_ID_DYNAMIC=7

# Approve it
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id $REQUEST_ID_DYNAMIC

export SERVICE_ID_DYNAMIC=4
```

### Test 5.2: Join Service with Default Exposure

**Goal:** Operator 2 joins an existing service

```bash
cargo tangle blueprint service join \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator2-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --service-id $SERVICE_ID_DYNAMIC

# Expected output includes:
# Service joined: service_id=4, exposure_bps=10000
```

### Test 5.3: Join Service with Custom Exposure

**Goal:** Join with specific exposure value

```bash
# First create another dynamic service for this test
cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --ttl 7200

export REQUEST_ID_DYNAMIC2=8

cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id $REQUEST_ID_DYNAMIC2

export SERVICE_ID_DYNAMIC2=5

# Join with 50% exposure (5000 bps)
cargo tangle blueprint service join \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator2-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --service-id $SERVICE_ID_DYNAMIC2 \
  --exposure-bps 5000
```

### Test 5.4: Join Validation - Zero Exposure (Should Fail)

**Goal:** Verify CLI prevents zero exposure

```bash
cargo tangle blueprint service join \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator2-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --service-id $SERVICE_ID_DYNAMIC \
  --exposure-bps 0

# Expected: Error "Exposure must be greater than 0 bps"
```

### Test 5.5: Join Validation - Excessive Exposure (Should Fail)

**Goal:** Verify CLI prevents exposure > 10000 bps

```bash
cargo tangle blueprint service join \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator2-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --service-id $SERVICE_ID_DYNAMIC \
  --exposure-bps 15000

# Expected: Error "Exposure cannot exceed 10000 bps"
```

---

## Phase 6: Service Leave

### Test 6.1: Leave Service (Normal Flow)

**Goal:** Operator leaves a service they're active in

```bash
# Operator 2 leaves SERVICE_ID_DYNAMIC (they joined in Test 5.2)
cargo tangle blueprint service leave \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator2-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --service-id $SERVICE_ID_DYNAMIC

# Expected output:
# Service leave: submitted tx_hash=0x...
# Service leave: confirmed block=Some(N) gas_used=...
```

### Test 6.2: Leave Validation - Not Active (Should Fail)

**Goal:** Verify CLI prevents leaving a service you're not active in

```bash
# Try to leave again (already left in 6.1)
cargo tangle blueprint service leave \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator2-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --service-id $SERVICE_ID_DYNAMIC

# Expected: Error "Operator is not active in service N"
```

### Test 6.3: Leave with JSON Output

**Goal:** Verify JSON output format

```bash
# First rejoin so we can test leave with JSON
cargo tangle blueprint service join \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator2-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --service-id $SERVICE_ID_DYNAMIC

# Now leave with JSON
cargo tangle blueprint service leave \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator2-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --service-id $SERVICE_ID_DYNAMIC \
  --json
```

---

## Phase 7: Service List Commands

### Test 7.1: List All Services

**Goal:** Verify service list command works

```bash
cargo tangle blueprint service list \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: List of all services created during tests
```

### Test 7.2: List Services with JSON Output

**Goal:** Verify JSON output format

```bash
cargo tangle blueprint service list \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --json | jq '.'

# Verify all expected fields are present
```

### Test 7.3: List All Service Requests

**Goal:** Verify service requests list command

```bash
cargo tangle blueprint service requests \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Should show all requests including rejected ones
```

### Test 7.4: List Requests with JSON Output

**Goal:** Verify JSON output format for requests

```bash
cargo tangle blueprint service requests \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --json | jq '.'
```

### Test 7.5: Show Specific Request Details

**Goal:** Verify show-request command

```bash
cargo tangle blueprint service show \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --request-id 0

# Should show detailed info for request 0
```

---

## Phase 8: Service Spawn

### Test 8.1: Spawn Service (Basic)

**Goal:** Verify service spawn command starts the manager

> **Note:** This test requires Terminal 4. The manager will run continuously.

```bash
# Terminal 4
cd /path/to/service-lifecycle-test/svc-test-blueprint

# Ensure settings.env has correct SERVICE_ID
sed -i '' "s/SERVICE_ID=.*/SERVICE_ID=$SERVICE_ID_1/" ./settings.env 2>/dev/null || \
sed -i "s/SERVICE_ID=.*/SERVICE_ID=$SERVICE_ID_1/" ./settings.env

cargo tangle blueprint service spawn \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 0 \
  --data-dir ./data-spawn

# Expected: Manager starts and runs until Ctrl+C
```

### Test 8.2: Spawn with Native Method

**Goal:** Verify spawn with explicit native execution

```bash
cargo tangle blueprint service spawn \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 0 \
  --spawn-method Native \
  --data-dir ./data-spawn-native
```

### Test 8.3: Spawn with --no-vm Flag

**Goal:** Verify --no-vm flag disables VM sandbox

```bash
cargo tangle blueprint service spawn \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 0 \
  --no-vm \
  --data-dir ./data-spawn-novm
```

### Test 8.4: Spawn with Dry Run

**Goal:** Verify dry-run doesn't submit transactions

```bash
cargo tangle blueprint service spawn \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 0 \
  --dry-run \
  --data-dir ./data-spawn-dry

# Should complete without submitting on-chain transactions
```

---

## Phase 9: Edge Cases and Error Handling

### Test 9.1: Request with Empty Operator List (Should Fail)

**Goal:** Verify at least one operator is required

```bash
cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --ttl 3600

# Expected: Error about requiring at least one operator
```

### Test 9.2: Request with Mismatched Exposure Count (Should Fail)

**Goal:** Verify exposure count must match operator count

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
  --operator 0x90F79bf6EB2c4f870365E785982E1f101E93b906 \
  --operator-exposure-bps 5000 \
  --ttl 3600

# Expected: Error "Expected 2 operator exposure values but received 1"
```

### Test 9.3: Request with Invalid Address (Should Fail)

**Goal:** Verify address validation

```bash
cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --operator invalid-address \
  --ttl 3600

# Expected: Error about invalid address format
```

### Test 9.4: Approve Non-Existent Request (Should Fail)

**Goal:** Verify error handling for invalid request ID

```bash
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id 99999

# Expected: Contract revert or error
```

### Test 9.5: Join Non-Existent Service (Should Fail)

**Goal:** Verify error handling for invalid service ID

```bash
cargo tangle blueprint service join \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator2-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --service-id 99999

# Expected: Contract revert or error
```

---

## Phase 10: Security Requirements (Advanced)

### Test 10.1: Request with Security Requirements

**Goal:** Test service request with security requirements

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
  --security-requirement native:_:100:500 \
  --ttl 3600

export REQUEST_ID_SEC=9
```

### Test 10.2: Approve with Security Commitments

**Goal:** Approve with explicit security commitments

```bash
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id $REQUEST_ID_SEC \
  --security-commitment native:_:250
```

### Test 10.3: Invalid Security Requirement Format (Should Fail)

**Goal:** Verify security requirement format validation

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
  --security-requirement invalid-format \
  --ttl 3600

# Expected: Error "Expected format KIND:TOKEN:MIN:MAX"
```

### Test 10.4: Security Requirement Min > Max (Should Fail)

**Goal:** Verify min <= max constraint

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
  --security-requirement native:_:500:100 \
  --ttl 3600

# Expected: Error "minimum exposure cannot exceed maximum exposure"
```

---

## Cleanup

```bash
# Stop all processes
pkill -f "anvil"
pkill -f "python.*http.server"
pkill -f "svc-test-blueprint"

# Remove test directory
cd /path/to/parent
rm -rf service-lifecycle-test
```

---

## Quick Reference: Key Accounts

| Index | Address | Private Key | Role |
|-------|---------|-------------|------|
| 0 | `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` | `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` | Deployer |
| 1 | `0x70997970C51812dc3A010C7d01b50e0d17dc79C8` | `0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d` | Operator 1 |
| 2 | `0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC` | `0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a` | User |
| 3 | `0x90F79bf6EB2c4f870365E785982E1f101E93b906` | `0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6` | Operator 2 |

---

## Quick Reference: Contract Addresses (Anvil Deterministic)

| Contract | Address |
|----------|---------|
| Tangle | `0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9` |
| Restaking | `0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512` |
| Status Registry | `0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf` |
