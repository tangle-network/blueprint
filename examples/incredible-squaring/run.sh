#!/bin/sh

# Exit on error
set -e

# Function to check if tangle network is running
check_tangle_network() {
    echo "Checking if tangle network is running..."
    if curl -s -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
        http://127.0.0.1:9944 > /dev/null; then
        echo "Tangle network is already running"
        return 0
    else
        echo "Tangle network is not running"
        return 1
    fi
}

# Function to check if keystore exists
check_keystore() {
    local key_path="$1"
    if [ -f "$key_path" ]; then
        echo "Key already exists at $key_path"
        return 0
    else
        return 1
    fi
}

echo "Starting setup process..."

# Step 1: Create keystore directory if not exists
if [ ! -d "./target/keystore" ]; then
    echo "Creating keystore directory..."
    mkdir -p ./target/keystore
else
    echo "Keystore directory already exists"
fi

# Step 2: Import keys if not exist
echo "Checking and importing keys..."
sr25519_key="./target/keystore/Sr25519/bdbd805d4c8dbe9c16942dc1146539944f34675620748bcb12585e671205aef1"
ecdsa_key="./target/keystore/Ecdsa/4c5d99a279a40b7ddb46776caac4216224376f6ae1fe43316be506106673ea76"

if ! check_keystore "$sr25519_key"; then
    echo "Importing sr25519 key..."
    cargo tangle key import -t sr25519 -k target/keystore -x e5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
fi

if ! check_keystore "$ecdsa_key"; then
    echo "Importing ecdsa key..."
    cargo tangle key import -t ecdsa -k target/keystore -x cb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854
fi

# Step 3: Build contracts if needed
if [ ! -d "contracts/out" ]; then
    echo "Building contracts..."
    forge build
else
    echo "Contracts already built"
fi

# Step 4: Run the tangle network locally if not running
if ! check_tangle_network; then
    echo "Starting tangle network..."
    ./target/release/tangle --tmp --dev --validator -linfo --alice --rpc-cors all --rpc-methods=unsafe --rpc-external --rpc-port 9944 -levm=debug -lgadget=trace --sealing manual &
    TANGLE_PID=$!

    echo "Waiting for network to start..."
    while ! check_tangle_network; do
        sleep 2
        echo "Still waiting for network..."
    done
    echo "Network is up!"
else
    echo "Using existing tangle network"
fi

# Function to check if blueprint exists
check_blueprint() {
    local id="$1"
    if cargo tangle blueprint lb | grep -q "Blueprint ID: $id"; then
        echo "Blueprint $id exists"
        return 0
    else
        echo "Blueprint $id does not exist"
        return 1
    fi
}

# Step 5: Deploy MBSM if needed
cargo tangle blueprint deploy-mbsm

# Step 6: Build the blueprint if needed
if [ ! -f "target/debug/espresso-raas-blueprint-cli" ]; then
    echo "Building blueprint..."
    cargo build --workspace
else
    echo "Blueprint already built"
fi

# Step 7-8: Deploy and verify blueprint if needed
echo "Checking blueprint deployment..."
if ! check_blueprint "0"; then
    echo "Deploying blueprint..."
    cargo tangle blueprint deploy tangle --http-rpc-url http://127.0.0.1:9944 --ws-rpc-url ws://127.0.0.1:9944 -k target/keystore

    echo "Verifying deployment..."
    cargo tangle blueprint lb
else
    echo "Blueprint already deployed"
fi

# Step 9-11: Service setup
read -p "Do you want to setup a new service instance? (y/n) " setup_service
if [ "$setup_service" = "y" ]; then
    echo "Registering as operator..."
    cargo tangle blueprint register --blueprint-id 0 --keystore-uri ./target/keystore

    echo "Requesting service instance..."
    cargo tangle blueprint request-service --blueprint-id 0 --keystore-uri ./target/keystore --value 0 --target-operators 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

    echo "Approving service instance..."
    cargo tangle blueprint accept-request --request-id 0 --keystore-uri ./target/keystore
else
    echo "Skipping service instance setup"
fi

# Step 12: Run the blueprint if requested
read -p "Do you want to run the blueprint now? (y/n) " run_blueprint
if [ "$run_blueprint" = "y" ]; then
    echo "Running blueprint..."
    echo "Note: Enter '0' for both blueprint ID and service instance ID when prompted"
    RUST_LOG=blueprint-rejection=trace,tangle-producer=debug,tangle-consumer=trace,blueprint-router=trace,blueprint-runner=trace,incredible_squaring_blueprint=debug,incredible_squaring_blueprint_bin=debug\
    ./target/debug/incredible-squaring-blueprint-bin run --protocol tangle\
         --blueprint-id 0\
         --service-id 0\
        --http-rpc-url http://localhost:9944\
        --ws-rpc-url ws://localhost:9944\
        --chain local_testnet\
        --keystore-uri ./target/keystore -vvvv

else
    echo "Skipping blueprint run"
fi

echo "Setup complete!"


