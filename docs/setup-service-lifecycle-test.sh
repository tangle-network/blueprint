#!/bin/bash
set -e

# ============================================================================
# Service Lifecycle Test Setup Script
# ============================================================================
# This script sets up everything needed to test service lifecycle commands.
# Run from the parent directory containing blueprint/ and tnt-core/
#
# Usage:
#   ./setup-service-lifecycle-test.sh
#
# Environment Variables (optional):
#   BLUEPRINT_SDK_PATH - Path to blueprint SDK (default: ../blueprint)
#   TNT_CORE_PATH - Path to tnt-core contracts (default: ../tnt-core)
# ============================================================================

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Configuration
BLUEPRINT_SDK_PATH="${BLUEPRINT_SDK_PATH:-../blueprint}"
TNT_CORE_PATH="${TNT_CORE_PATH:-../tnt-core}"
TEST_DIR="service-lifecycle-test"
HTTP_PORT=8081

echo ""
echo -e "${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║       Service Lifecycle Test Environment Setup            ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""

# Validate paths
log_info "Validating paths..."

if [ ! -d "$BLUEPRINT_SDK_PATH" ]; then
    log_error "Blueprint SDK not found at $BLUEPRINT_SDK_PATH"
    echo "Set BLUEPRINT_SDK_PATH environment variable to the correct path"
    exit 1
fi
log_success "Blueprint SDK found at $BLUEPRINT_SDK_PATH"

if [ ! -d "$TNT_CORE_PATH" ]; then
    log_error "tnt-core not found at $TNT_CORE_PATH"
    echo "Set TNT_CORE_PATH environment variable to the correct path"
    exit 1
fi
log_success "tnt-core found at $TNT_CORE_PATH"

# Check for required tools
log_info "Checking required tools..."

if ! command -v cargo &> /dev/null; then
    log_error "cargo not found. Please install Rust."
    exit 1
fi
log_success "cargo found"

if ! command -v anvil &> /dev/null; then
    log_error "anvil not found. Please install Foundry (foundryup)."
    exit 1
fi
log_success "anvil found"

if ! command -v forge &> /dev/null; then
    log_error "forge not found. Please install Foundry (foundryup)."
    exit 1
fi
log_success "forge found"

if ! command -v cast &> /dev/null; then
    log_error "cast not found. Please install Foundry (foundryup)."
    exit 1
fi
log_success "cast found"

if ! command -v python3 &> /dev/null; then
    log_error "python3 not found. Please install Python 3."
    exit 1
fi
log_success "python3 found"

# Check if cargo-tangle is installed
if ! cargo tangle --help &> /dev/null; then
    log_warn "cargo-tangle not found. Installing..."
    cargo install cargo-tangle --path "$BLUEPRINT_SDK_PATH/cli" --force
fi
log_success "cargo-tangle installed"

# Create test directory
echo ""
log_info "Creating test directory: $TEST_DIR"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Store absolute paths
BLUEPRINT_SDK_ABS=$(cd "$BLUEPRINT_SDK_PATH" && pwd)
TNT_CORE_ABS=$(cd "$TNT_CORE_PATH" && pwd)
TEST_DIR_ABS=$(pwd)

# ============================================================================
# Step 1: Create blueprint
# ============================================================================
echo ""
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
log_info "Step 1: Creating test blueprint..."
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

cargo tangle blueprint create \
  --name svc-test-blueprint \
  --skip-prompts \
  --project-description "Service Lifecycle Test Blueprint" \
  --project-authors "Tangle"

cd svc-test-blueprint
log_success "Blueprint created"

# Fix rust toolchain version
log_info "Fixing Rust toolchain version..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' 's/channel = "1.86"/channel = "1.88"/' rust-toolchain.toml 2>/dev/null || true
else
    sed -i 's/channel = "1.86"/channel = "1.88"/' rust-toolchain.toml 2>/dev/null || true
fi
log_success "Rust toolchain updated to 1.88"

# Update to use local SDK
log_info "Updating Cargo.toml to use local SDK..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "s|blueprint-sdk = { git = \"https://github.com/tangle-network/blueprint\", branch = \"v2\"|blueprint-sdk = { path = \"$BLUEPRINT_SDK_ABS/crates/sdk\"|" svc-test-blueprint-bin/Cargo.toml 2>/dev/null || true
    sed -i '' "s|blueprint-sdk = { git = \"https://github.com/tangle-network/blueprint\", branch = \"v2\"|blueprint-sdk = { path = \"$BLUEPRINT_SDK_ABS/crates/sdk\"|" svc-test-blueprint-lib/Cargo.toml 2>/dev/null || true
    sed -i '' "s|blueprint-anvil-testing-utils = { git = \"https://github.com/tangle-network/blueprint\", branch = \"v2\" }|blueprint-anvil-testing-utils = { path = \"$BLUEPRINT_SDK_ABS/crates/testing-utils/anvil\" }|" svc-test-blueprint-lib/Cargo.toml 2>/dev/null || true
else
    sed -i "s|blueprint-sdk = { git = \"https://github.com/tangle-network/blueprint\", branch = \"v2\"|blueprint-sdk = { path = \"$BLUEPRINT_SDK_ABS/crates/sdk\"|" svc-test-blueprint-bin/Cargo.toml 2>/dev/null || true
    sed -i "s|blueprint-sdk = { git = \"https://github.com/tangle-network/blueprint\", branch = \"v2\"|blueprint-sdk = { path = \"$BLUEPRINT_SDK_ABS/crates/sdk\"|" svc-test-blueprint-lib/Cargo.toml 2>/dev/null || true
    sed -i "s|blueprint-anvil-testing-utils = { git = \"https://github.com/tangle-network/blueprint\", branch = \"v2\" }|blueprint-anvil-testing-utils = { path = \"$BLUEPRINT_SDK_ABS/crates/testing-utils/anvil\" }|" svc-test-blueprint-lib/Cargo.toml 2>/dev/null || true
fi
log_success "Cargo.toml updated to use local SDK"

# ============================================================================
# Step 2: Build blueprint
# ============================================================================
echo ""
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
log_info "Step 2: Building blueprint (this may take a few minutes)..."
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

# Set macOS environment if needed
if [[ "$OSTYPE" == "darwin"* ]]; then
    export MACOSX_DEPLOYMENT_TARGET=14.0
    export SDKROOT=$(xcrun --show-sdk-path)
    export CXXFLAGS="-isysroot $SDKROOT -I$SDKROOT/usr/include/c++/v1 -stdlib=libc++"
    if [ -f /opt/homebrew/bin/protoc ]; then
        export PROTOC=/opt/homebrew/bin/protoc
    fi
fi

cargo build --release -p svc-test-blueprint-bin
log_success "Blueprint built successfully"

# ============================================================================
# Step 3: Package artifacts
# ============================================================================
echo ""
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
log_info "Step 3: Packaging artifacts..."
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

mkdir -p ./dist

# Create archive
tar -cJf ./dist/svc-test-blueprint.tar.xz -C ../../target/release svc-test-blueprint
log_success "Archive created"

# Compute SHA256 of binary
if [[ "$OSTYPE" == "darwin"* ]]; then
    SHA256=$(shasum -a 256 ../../target/release/svc-test-blueprint | awk '{print $1}')
else
    SHA256=$(sha256sum ../../target/release/svc-test-blueprint | awk '{print $1}')
fi

log_info "Binary SHA256: $SHA256"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
case "$ARCH" in
  x86_64) ARCH="amd64" ;;
  aarch64|arm64) ARCH="arm64" ;;
esac

log_info "Detected: OS=$OS, ARCH=$ARCH"

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
log_success "definition.json created"

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
log_success "dist.json created"

# ============================================================================
# Step 4: Setup keystores
# ============================================================================
echo ""
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
log_info "Step 4: Setting up keystores..."
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

# Operator 1 keystore (Anvil account 1)
mkdir -p ./operator-keystore
cargo tangle key import \
  --key-type ecdsa \
  --secret 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \
  --keystore-path ./operator-keystore \
  --protocol tangle
log_success "Operator 1 keystore created (0x70997970C51812dc3A010C7d01b50e0d17dc79C8)"

# Operator 2 keystore (Anvil account 3)
mkdir -p ./operator2-keystore
cargo tangle key import \
  --key-type ecdsa \
  --secret 7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6 \
  --keystore-path ./operator2-keystore \
  --protocol tangle
log_success "Operator 2 keystore created (0x90F79bf6EB2c4f870365E785982E1f101E93b906)"

# User keystore (Anvil account 2)
mkdir -p ./user-keystore
cargo tangle key import \
  --key-type ecdsa \
  --secret 5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a \
  --keystore-path ./user-keystore \
  --protocol tangle
log_success "User keystore created (0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC)"

# ============================================================================
# Step 5: Create settings.env template
# ============================================================================
echo ""
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
log_info "Step 5: Creating settings.env template..."
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

cat > ./settings.env << EOF
# Tangle EVM Settings for Service Lifecycle Testing
# Contract addresses are deterministic with Anvil defaults

HTTP_RPC_URL=http://127.0.0.1:8545
WS_RPC_URL=ws://127.0.0.1:8546
KEYSTORE_PATH=./operator-keystore
BLUEPRINT_KEYSTORE_URI=./operator-keystore

# Contract Addresses (Anvil deterministic)
TANGLE_CONTRACT=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
RESTAKING_CONTRACT=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
STATUS_REGISTRY_CONTRACT=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf

# Blueprint/Service IDs (will be set during testing)
BLUEPRINT_ID=0
SERVICE_ID=0
EOF
log_success "settings.env created"

# ============================================================================
# Step 6: Create helper scripts
# ============================================================================
echo ""
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
log_info "Step 6: Creating helper scripts..."
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

# Script to start anvil
cat > ./start-anvil.sh << 'EOF'
#!/bin/bash
echo "Starting Anvil..."
anvil --host 0.0.0.0 --port 8545 --base-fee 0 --gas-price 0 --gas-limit 100000000 --hardfork cancun --code-size-limit 50000
EOF
chmod +x ./start-anvil.sh
log_success "start-anvil.sh created"

# Script to start HTTP server
cat > ./start-http-server.sh << 'EOF'
#!/bin/bash
echo "Starting HTTP server on port 8081..."
cd ./dist
python3 -m http.server 8081
EOF
chmod +x ./start-http-server.sh
log_success "start-http-server.sh created"

# Script to deploy contracts and setup
cat > ./deploy-and-setup.sh << EOF
#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "\${YELLOW}Deploying contracts...\${NC}"
cd "$TNT_CORE_ABS"

forge script script/v2/DeployContractsOnly.s.sol:DeployContractsOnly \\
  --rpc-url http://127.0.0.1:8545 \\
  --broadcast \\
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \\
  --disable-code-size-limit \\
  -vvv

echo -e "\${GREEN}Contracts deployed!\${NC}"

# Export contract addresses
export TANGLE=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
export RESTAKING=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
export STATUS_REGISTRY=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf

echo -e "\${YELLOW}Registering operators on restaking layer...\${NC}"

# Operator 1
cast send \$RESTAKING "registerOperator()" \\
  --value 2ether \\
  --private-key 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \\
  --rpc-url http://127.0.0.1:8545

# Operator 2
cast send \$RESTAKING "registerOperator()" \\
  --value 2ether \\
  --private-key 0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6 \\
  --rpc-url http://127.0.0.1:8545

echo -e "\${GREEN}Operators registered on restaking layer!\${NC}"

# Verify
echo ""
echo "Verifying operator registrations..."
echo -n "Operator 1 (0x70997970C51812dc3A010C7d01b50e0d17dc79C8): "
cast call \$RESTAKING "isOperator(address)(bool)" 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 --rpc-url http://127.0.0.1:8545
echo -n "Operator 2 (0x90F79bf6EB2c4f870365E785982E1f101E93b906): "
cast call \$RESTAKING "isOperator(address)(bool)" 0x90F79bf6EB2c4f870365E785982E1f101E93b906 --rpc-url http://127.0.0.1:8545

echo ""
cd "$TEST_DIR_ABS/svc-test-blueprint"

echo -e "\${YELLOW}Deploying blueprint...\${NC}"
cargo tangle blueprint deploy tangle \\
  --network testnet \\
  --definition ./dist/definition.json \\
  --settings-file ./settings.env

echo -e "\${GREEN}Blueprint deployed!\${NC}"

echo -e "\${YELLOW}Registering Operator 1 for blueprint...\${NC}"
cargo tangle blueprint register \\
  --http-rpc-url http://127.0.0.1:8545 \\
  --ws-rpc-url ws://127.0.0.1:8546 \\
  --keystore-path ./operator-keystore \\
  --tangle-contract \$TANGLE \\
  --restaking-contract \$RESTAKING \\
  --status-registry-contract \$STATUS_REGISTRY \\
  --blueprint-id 0 \\
  --rpc-endpoint "http://localhost:9000"

echo -e "\${YELLOW}Registering Operator 2 for blueprint...\${NC}"
cargo tangle blueprint register \\
  --http-rpc-url http://127.0.0.1:8545 \\
  --ws-rpc-url ws://127.0.0.1:8546 \\
  --keystore-path ./operator2-keystore \\
  --tangle-contract \$TANGLE \\
  --restaking-contract \$RESTAKING \\
  --status-registry-contract \$STATUS_REGISTRY \\
  --blueprint-id 0 \\
  --rpc-endpoint "http://localhost:9001"

echo -e "\${GREEN}Setup complete! Ready for testing.\${NC}"
echo ""
echo "Contract Addresses:"
echo "  TANGLE=\$TANGLE"
echo "  RESTAKING=\$RESTAKING"
echo "  STATUS_REGISTRY=\$STATUS_REGISTRY"
echo ""
echo "To export these in your terminal:"
echo "  export TANGLE=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"
echo "  export RESTAKING=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
echo "  export STATUS_REGISTRY=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf"
echo "  export BLUEPRINT_ID=0"
EOF
chmod +x ./deploy-and-setup.sh
log_success "deploy-and-setup.sh created"

# ============================================================================
# Summary
# ============================================================================
echo ""
echo -e "${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                    Setup Complete!                         ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Test Directory:${NC} $TEST_DIR_ABS/svc-test-blueprint"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo ""
echo "1. ${BLUE}Terminal 1${NC} - Start Anvil:"
echo "   cd $TEST_DIR_ABS/svc-test-blueprint"
echo "   ./start-anvil.sh"
echo ""
echo "2. ${BLUE}Terminal 2${NC} - Start HTTP server:"
echo "   cd $TEST_DIR_ABS/svc-test-blueprint"
echo "   ./start-http-server.sh"
echo ""
echo "3. ${BLUE}Terminal 3${NC} - Deploy contracts and setup:"
echo "   cd $TEST_DIR_ABS/svc-test-blueprint"
echo "   ./deploy-and-setup.sh"
echo ""
echo "   Then export environment variables:"
echo "   export TANGLE=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"
echo "   export RESTAKING=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
echo "   export STATUS_REGISTRY=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf"
echo "   export BLUEPRINT_ID=0"
echo ""
echo "4. Start testing! Follow the test plan in:"
echo "   $BLUEPRINT_SDK_ABS/docs/SERVICE_LIFECYCLE_TEST_PLAN.md"
echo ""
echo -e "${BLUE}Key Accounts:${NC}"
echo "  Deployer:   0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
echo "  Operator 1: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8"
echo "  User:       0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC"
echo "  Operator 2: 0x90F79bf6EB2c4f870365E785982E1f101E93b906"
echo ""
