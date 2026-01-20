# E2E Testing Notes

This document tracks the history of local E2E testing, including steps taken, issues/bugs found, and fixes implemented.

---

## Session 1: 2026-01-19

### Prerequisites: CLI Installation

**Issue 1: macOS ARM64 C++ compilation failures**

When running `cargo install cargo-tangle --path ./cli --force`, the build failed with:
- `rocksdb`: `fatal error: 'cstdlib' not found` - caused by invalid `-mmacosx-version-min=26.2` deployment target
- `protobuf-src`: `ld: library 'atomic' not found` - protobuf 27.x links against libatomic which doesn't exist on macOS ARM64

**Root Cause:** Darwin 25.x (macOS 16 beta) causes `cc-rs` to derive an invalid deployment target. Additionally, `protobuf-src` builds protobuf from source which requires `libatomic` on the linker path.

**Workaround:** Set environment variables before building:
```bash
export MACOSX_DEPLOYMENT_TARGET=14.0
export SDKROOT=$(xcrun --show-sdk-path)
export CXXFLAGS="-isysroot $SDKROOT -I$SDKROOT/usr/include/c++/v1 -stdlib=libc++"
export PROTOC=/opt/homebrew/bin/protoc
```

**Note:** Requires Homebrew protobuf installed (`brew install protobuf`).

---

### Step 1: Create a New Blueprint

**Command:**
```bash
cd /Users/tlinhsmacbook/development/tangle
cargo tangle blueprint create --name hello-blueprint --skip-prompts --project-description "Hello Blueprint for local E2E testing" --project-authors "Tangle"
```

**Note:** The `--skip-prompts` flag requires `--project-description` and `--project-authors` flags. The test plan documentation should be updated to include these.

**Result:** ✅ Success - Blueprint created at `/Users/tlinhsmacbook/development/tangle/hello-blueprint`

---

### Step 2: Start Anvil

**Command:**
```bash
anvil --host 0.0.0.0 --port 8545 --base-fee 0 --gas-price 0 --gas-limit 100000000 --hardfork cancun
```

**Result:** ✅ Success - Anvil running on port 8545 with deterministic accounts

---

### Step 3: Package and Serve Blueprint Artifacts

**Issue 2: Rust version mismatch in blueprint template**

The generated blueprint template specifies Rust 1.86 in `rust-toolchain.toml`, but the blueprint SDK dependencies require Rust 1.88.

**Fix:** Update `hello-blueprint/rust-toolchain.toml` to use `channel = "1.88"`.

**Note:** The blueprint template should be updated to use Rust 1.88.

**Commands executed:**
```bash
# Build (with macOS env vars)
export MACOSX_DEPLOYMENT_TARGET=14.0
export SDKROOT=$(xcrun --show-sdk-path)
export CXXFLAGS="-isysroot $SDKROOT -I$SDKROOT/usr/include/c++/v1 -stdlib=libc++"
export PROTOC=/opt/homebrew/bin/protoc
cargo build --release -p hello-blueprint-bin

# Package
mkdir -p ./dist
tar -cJf ./dist/hello-blueprint.tar.xz -C ./target/release hello-blueprint

# Create definition.json and dist.json (see test plan for full commands)

# Serve artifacts
cd ./dist && python3 -m http.server 8081
```

**Result:** ✅ Success - Binary built, packaged, and HTTP server running on port 8081

---

### Step 4: Deploy Contracts

**Issue 3: Contract size limit errors**

The forge script fails with errors like:
```
Error: `TangleBlueprintsFacet` is above the contract size limit (24976 > 24576).
```

**Root Cause:** The Tangle facet contracts exceed EVM's 24KB code size limit (EIP-170).

**Fix:** Use `--disable-code-size-limit` flag with forge script, and start Anvil with `--code-size-limit 50000`.

**Commands:**
```bash
# Start Anvil with increased code size limit
anvil --host 0.0.0.0 --port 8545 --base-fee 0 --gas-price 0 --gas-limit 100000000 --hardfork cancun --code-size-limit 50000

# Deploy contracts with disabled size limit check
forge script script/v2/DeployContractsOnly.s.sol:DeployContractsOnly \
  --rpc-url http://127.0.0.1:8545 \
  --broadcast \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
  --disable-code-size-limit \
  -vvv
```

**Deployed Addresses:**
- TANGLE: `0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9`
- RESTAKING (MultiAssetDelegation): `0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512`
- STATUS_REGISTRY: `0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf`

**Result:** ✅ Success - Contracts deployed and verified callable

**⚠️ CRITICAL: Production Blocker**

The contract size limit issue is **not just a local testing problem** - it will cause deployment failures on testnet/mainnet.

EIP-170 enforces a hard 24KB (24,576 bytes) limit on all standard EVM networks:
- Ethereum mainnet
- All testnets (Sepolia, Holesky, etc.)
- Most L2s and EVM-compatible chains

| Contract | Size | Over Limit |
|----------|------|------------|
| TangleBlueprintsFacet | 24,976 bytes | +400 bytes |
| TangleServicesLifecycleFacet | 25,216 bytes | +640 bytes |
| TangleJobsAggregationFacet | 25,265 bytes | +689 bytes |

The `--disable-code-size-limit` flag only works locally - it cannot bypass network protocol rules.

**Required fixes before production:**
1. Split facets into smaller sub-facets
2. Move logic to external libraries (don't count toward contract size)
3. Optimize bytecode (shorter error strings, custom errors, remove unused code)

---

### Step 5: Setup Operator Keystore

**Command:**
```bash
cd hello-blueprint
mkdir -p ./operator-keystore
cargo tangle key import \
  --key-type ecdsa \
  --secret 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \
  --keystore-path ./operator-keystore \
  --protocol tangleevm
```

**Result:** ✅ Success - Operator key imported (public key: `02ba5734d8f7091719471e7f7ed6b9df170dc70cc661ca05e688601ad984f068b0`)

---

### Step 6: Register as Restaking Operator

**Commands:**
```bash
# Register with 2 ETH stake
cast send 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "registerOperator()" \
  --value 2ether \
  --private-key 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \
  --rpc-url http://127.0.0.1:8545

# Verify registration
cast call 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "isOperator(address)(bool)" \
  0x70997970C51812dc3A010C7d01b50e0d17dc79C8 --rpc-url http://127.0.0.1:8545
# Returns: true
```

**Result:** ✅ Success - Operator `0x70997970C51812dc3A010C7d01b50e0d17dc79C8` registered with 2 ETH stake

---

### Step 7: Deploy Blueprint On-Chain

**Commands:**
```bash
# Create settings.env
cat > ./settings.env << 'EOF'
HTTP_RPC_URL=http://127.0.0.1:8545
WS_RPC_URL=ws://127.0.0.1:8546
KEYSTORE_PATH=./operator-keystore
BLUEPRINT_KEYSTORE_URI=./operator-keystore
TANGLE_CONTRACT=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
RESTAKING_CONTRACT=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
STATUS_REGISTRY_CONTRACT=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf
BLUEPRINT_ID=0
SERVICE_ID=0
EOF

# Deploy blueprint
cargo tangle blueprint deploy tangle \
  --network testnet \
  --definition ./dist/definition.json \
  --settings-file ./settings.env
```

**Result:** ✅ Success - Blueprint deployed with ID `0`
- Verified: `blueprintCount()` returns `1`

---

### Step 8: Register Operator for Blueprint

**Issue 4: InvalidOperatorKey error - compressed vs uncompressed public key**

Initial attempt failed with:
```
Contract error: custom error 0xda2cd52b
```
Decoded: `InvalidOperatorKey()`

**Root Cause:** The contract expects a 65-byte uncompressed ECDSA public key (SEC1 format starting with `0x04`), but the CLI was sending a 33-byte compressed key.

**Bug Location:** `crates/clients/tangle-evm/src/client.rs:763`

```rust
// Before (wrong - 33 bytes compressed):
let ecdsa_bytes = Bytes::copy_from_slice(&verifying.to_bytes());

// After (correct - 65 bytes uncompressed):
let encoded_point = verifying.0.to_encoded_point(false);
let ecdsa_bytes = Bytes::copy_from_slice(encoded_point.as_bytes());
```

**Fix applied:** Updated client code and reinstalled CLI.

**Command:**
```bash
cargo tangle blueprint register \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  --restaking-contract 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
  --status-registry-contract 0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf \
  --blueprint-id 0 \
  --rpc-endpoint "http://localhost:9000"
```

**Result:** ✅ Success - Operator registered for blueprint 0

---

### Step 9: Setup User Keystore

**Command:**
```bash
cd hello-blueprint
mkdir -p ./user-keystore
cargo tangle key import \
  --key-type ecdsa \
  --secret 5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a \
  --keystore-path ./user-keystore \
  --protocol tangleevm
```

**Result:** ✅ Success - User key imported (public key: `039d9031e97dd78ff8c15aa86939de9b1e791066a0224e331bc962a2099a7b1f04`)

---

### Step 10: Request Service

**Command:**
```bash
cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  --restaking-contract 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
  --status-registry-contract 0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf \
  --blueprint-id 0 \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --ttl 3600
```

**Result:** ✅ Success - Service requested, Request ID: `0`

---

### Step 11: Run the Operator

**Important:** The manager must be running *before* the service is approved, as it only processes new `ServiceActivated` events.

**Command:**
```bash
cargo tangle blueprint run \
  -p tangleevm \
  -k ./operator-keystore \
  -f ./settings.env
```

**Output:**
```
Starting blueprint manager for blueprint ID: 0
Preparing Blueprint to run, this may take a few minutes...
Starting blueprint execution...
Blueprint is running. Press Ctrl+C to stop.
```

**Result:** ✅ Success - Operator running, listening for events

---

### Step 12: Approve Service (as Operator)

**Command:**
```bash
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  --restaking-contract 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
  --status-registry-contract 0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf \
  --request-id 0 \
  --restaking-percent 100
```

**Result:** ✅ Success - Service approved, Service ID: `0`

---

### Step 13: Submit a Job

**Commands:**
```bash
# Encode HelloRequest { name: "Alice" }
PAYLOAD=$(cast abi-encode "f((string))" "(Alice)")

cargo tangle blueprint jobs submit \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  --restaking-contract 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
  --status-registry-contract 0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf \
  --blueprint-id 0 \
  --service-id 0 \
  --job 0 \
  --payload-hex $PAYLOAD
```

**Result:** ✅ Success - Job submitted, Call ID: `0`

---

### Step 14: Watch for Result

**Issue 5: Job result timeout - Manager missed ServiceActivated event**

The `jobs watch` command timed out waiting for a result:
```
Error: timed out waiting for result for call 0
```

**Root Cause:** The blueprint manager only subscribes to **new** events. It does not process historical events. When we restarted the manager (after fixing the public key bug in step 8), it missed the `ServiceActivated` event that was emitted in step 12.

The test plan explicitly warns:
> **Important**: The manager must be running *before* the service is approved. The manager only processes new `ServiceActivated` events, so if you approve first, it won't see the event and won't start the blueprint binary.

**Impact:** Without detecting the ServiceActivated event, the manager doesn't know about the active service and therefore doesn't spawn the blueprint binary to process jobs.

**Attempted workaround:** Submitted a second job (Call ID: 1) after restarting the manager, but the manager still didn't detect it because it never received the ServiceActivated event for service 0.

**Potential solutions:**
1. Manager should query for existing active services on startup (enhancement)
2. Need to redo the full flow from step 10 (request new service) without restarting the manager
3. Add a CLI command to manually trigger service detection

**Result:** ❌ Blocked - Need to implement fix or restart from step 10

---

### Continuing from Step 10: Re-request Service with Manager Running

To work around Issue 5, we restarted from Step 10 with the manager already running.

**New service requests made:**
- Request ID 1 → Service ID 1 (approved while manager running)
- Request ID 2 → Service ID 2 (approved while manager running)
- ...continuing through service IDs 3, 4, 5

**Result:** Manager caught the `ServiceActivated` events but logged:
```
ServiceActivated observed but metadata unavailable, service_id: X
```

This led to the discovery of Issue 6.

---

### Issue 6: ABI Decoding Bug in Manager - Blueprint Metadata Unavailable

**Symptom:** Even with the manager running and catching `ServiceActivated` events, it reported:
```
WARN Failed to decode blueprint definition payload: Failed to decode blueprint metadata: ...
INFO ServiceActivated observed but metadata unavailable, service_id: X
```

**Error Messages (evolved through debugging):**

1. **Initial error (buffer overrun):**
   ```
   Failed to decode blueprint metadata: ABI decoding failed: buffer overrun while deserializing
   ```

2. **After offset adjustments (type check failure):**
   ```
   Failed to decode blueprint metadata: type check failed for "offset (usize)" with data: 000000000000000000000000000000000f68656c6c6f2d626c75657072696e74
   ```
   The hex decodes to: `\0...\x0fhello-blueprint` - i.e., reading string length + content instead of an offset.

3. **Later attempts (still type check failure):**
   ```
   Failed to decode blueprint definition: type check failed for "offset (usize)" with data: 703a2f2f6c6f63616c686f73743a383038312f6d657461646174612e6a736f6e
   ```
   The hex decodes to: `p://localhost:8081/metadata.json` - reading the metadata URI string instead of an offset.

**Bug Location:** `crates/manager/src/protocol/tangle_evm/metadata.rs` - `decode_blueprint_definition()` function

**Root Cause Analysis:**

The `decode_blueprint_definition` function was attempting to manually extract metadata and sources from a `BlueprintDefinition` struct using hardcoded ABI slot offsets. However:

1. The original code used offsets `(1, 5)` assuming a simple struct layout
2. The actual `BlueprintDefinition` struct has nested types including `BlueprintConfig` which contains 7 static fields that are encoded inline, not as pointers
3. The eth_call return value includes an outer wrapper (first 32 bytes = offset `0x20`) for dynamic return types

**BlueprintDefinition Struct Layout (from Types.sol):**
```solidity
struct BlueprintDefinition {
    string metadataUri;        // Slot 0: dynamic (offset pointer)
    address manager;           // Slot 1: static (20 bytes, padded)
    uint32 masterManagerRevision; // Slot 2: static
    bool hasConfig;            // Slot 3: static
    BlueprintConfig config;    // Slots 4-10: 7 static fields inline
    BlueprintMetadata metadata; // Slot 11: dynamic (offset pointer)
    Source[] sources;          // Slot 12: dynamic (offset pointer)
}

struct BlueprintConfig {
    MembershipModel membership;    // uint8 enum
    PricingModel pricing;          // uint8 enum
    uint32 minOperators;
    uint32 maxOperators;
    uint256 subscriptionRate;
    uint256 subscriptionInterval;
    uint256 eventRate;
}
```

**Debugging Attempts:**

| Attempt | Approach | Result |
|---------|----------|--------|
| 1 | Original offsets (1, 5) | Buffer overrun |
| 2 | Adjusted offsets (5, 9) | Buffer overrun |
| 3 | Adjusted offsets (11, 15) accounting for BlueprintConfig inline | Type check failed - reading string data |
| 4 | Skip first 32 bytes (outer wrapper) + adjusted offsets | Type check failed |
| 5 | Full struct decode using `abi_decode::<ITangleTypes::BlueprintDefinition>()` | Type check failed |
| 6 | Use `abi_decode_params` instead of `abi_decode` | Type check failed |

**Analysis with Raw Data:**

Used `cast call` to examine the raw hex output:
```bash
cast call 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  "getBlueprintDefinition(uint64)(((string,address,uint32,bool,(uint8,uint8,uint32,uint32,uint256,uint256,uint256),(string,string,string,string,string,string,address),((uint8,bytes),bytes32)[])))" \
  0 --rpc-url http://127.0.0.1:8545
```

The issue is complex:
- Alloy's `abi_decode` expects the data to start directly with struct fields
- `abi_decode_params` handles function return value wrapping
- The decoder is still reading string content where it expects offset values, suggesting the struct decoding itself has issues with nested dynamic types

**Current Status:** ❌ Not resolved

The fix using `abi_decode_params` has been implemented but still produces type check failures. The decoder reads UTF-8 string content (`hello-blueprint`, `http://localhost:8081/metadata.json`) where it expects numeric offset values.

**Possible Solutions to Investigate:**

1. **Manual byte-level parsing:** Instead of relying on Alloy's automatic ABI decoder, manually parse the raw bytes using known offsets from the struct layout
2. **Simplify the RPC call:** Fetch individual fields separately rather than the whole struct
3. **Use contract event data:** Parse the data from the `BlueprintDeployed` event instead of calling `getBlueprintDefinition`
4. **Debug with minimal reproduction:** Create a test case that decodes known good data

**Files Modified:**
- `crates/manager/src/protocol/tangle_evm/metadata.rs`

**Code Evolution:**

Initial implementation (offsets 1, 5):
```rust
fn decode_blueprint_definition(data: &[u8]) -> Result<(String, Vec<OnChainBlueprintSource>)> {
    let metadata_offset = <sol!(uint256)>::abi_decode(&data[32..64], false)
        .map_err(|e| Error::Other(format!("Failed to decode metadata offset: {e}")))?
        .to::<usize>();
    let sources_offset = <sol!(uint256)>::abi_decode(&data[160..192], false)
        .map_err(|e| Error::Other(format!("Failed to decode sources offset: {e}")))?
        .to::<usize>();
    // ... decode using offsets
}
```

Attempted fix (full struct decode with abi_decode_params):
```rust
type OnChainBlueprintDefinition = <ITangleTypes::BlueprintDefinition as SolType>::RustType;

fn decode_blueprint_definition(data: &[u8]) -> Result<(String, Vec<OnChainBlueprintSource>)> {
    let definition: OnChainBlueprintDefinition =
        <ITangleTypes::BlueprintDefinition as SolType>::abi_decode_params(data)
            .map_err(|e| Error::Other(format!("Failed to decode blueprint definition: {e}")))?;

    let blueprint_name = definition.metadata.name.to_string();
    let sources: Vec<OnChainBlueprintSource> = definition.sources;

    Ok((blueprint_name, sources))
}
```

**Result:** ❌ Still failing - requires further investigation

---

### Issue 6 Resolution: Use Contract Binding for Blueprint Definition

**Fix:** Switched the manager to call the contract binding directly (`getBlueprintDefinition`) instead of manually decoding raw ABI bytes.

**Code changes:**
- Added `get_blueprint_definition` in the Tangle EVM client
- Updated the manager metadata loader to use the decoded struct directly
- Removed the manual `decode_blueprint_definition` helper

**Files changed:**
- `crates/clients/tangle-evm/src/client.rs`
- `crates/manager/src/protocol/tangle_evm/metadata.rs`

**Expected impact:** The manager should now resolve blueprint metadata and sources when it sees `ServiceActivated`, allowing it to fetch the dist artifacts and spawn the blueprint binary.

**Next steps to continue E2E:**
1. Rebuild/reinstall the CLI and manager binaries from this repo.
2. Restart the manager (`cargo tangle blueprint run ...`).
3. Re-request and approve a new service while the manager is running.
4. Submit and watch a job result.

---

### Issue 7: NoMatchingBinary on macOS (arm64)

**Symptom:** Manager exited with:
```
Blueprint Manager Closed Unexpectedly: Err(NoMatchingBinary)
```

**Root Cause:** The on-chain blueprint binaries use `os=macos` and `arch=arm64`, while the manager matched against `apple-darwin` and `aarch64` without normalization.

**Fix:** Normalize OS and architecture strings when selecting a binary:
- Map `arm64` → `aarch64`
- Map `apple-darwin` / `darwin` / `macos` → `macos`

**Files changed:**
- `crates/manager/src/blueprint/native.rs`

---

### Issue 8: Blueprint Runner CLI rejected `PROTOCOL=tangle-evm`

**Symptom:** Manager log showed:
```
error: invalid value 'tangle-evm' for '--protocol'
  [possible values: tangleevm]
```

**Root Cause:** The manager set `PROTOCOL` using `Protocol::Display`, which emitted `tangle-evm`, but the blueprint runner's clap `ValueEnum` with `rename_all = "lowercase"` only accepted `tangleevm` (variant name `TangleEvm` lowercased).

**Fix:** Use the readable `tangle-evm` name everywhere:
- Added `#[value(alias = "tangle-evm")]` to `Protocol::TangleEvm` variant so clap accepts it
- `Protocol::as_str()` returns `"tangle-evm"`
- `TangleEvmProtocolSettings::protocol_name()` returns `"tangle-evm"`
- Deleted dead code: `Protocol::from_env()` method (never called) and `FromStr` impl (only used by dead code)
- Deleted unused `ConfigError::UnsupportedProtocol` error variant

**Files changed:**
- `crates/runner/src/config.rs`
- `crates/runner/src/tangle_evm/config.rs`
- `crates/runner/src/error.rs`

---

### Issue 9: Auth Proxy RocksDB LOCK (manager startup failure)

**Symptom:** Manager failed to start with:
```
IO error: While lock file: .../data/private/auth-proxy/db/LOCK: Resource temporarily unavailable
```

**Workaround:** Run the manager with a fresh data dir:
```
cargo tangle blueprint run -p tangleevm -k ./operator-keystore -f ./settings.env -d ./data-alt
```

---

## Session 2: 2026-01-19 (continued)

### Issue 10: CLI Compilation Error - UnsupportedProtocol variant doesn't exist

**Symptom:** When building the CLI with `cargo install cargo-tangle --path ./cli --force`, compilation failed:
```
error[E0599]: no variant or associated item named `UnsupportedProtocol` found for enum `ConfigError`
  --> cli/src/settings.rs:70:31
   |
70 |         _ => Err(ConfigError::UnsupportedProtocol(protocol.to_string())),
   |                               ^^^^^^^^^^^^^^^^^^^ variant or associated item not found in `ConfigError`
```

**Root Cause:** The code referenced a non-existent enum variant `UnsupportedProtocol`. The actual variant in `ConfigError` is `UnexpectedProtocol(&'static str)`.

**Fix:** Changed the error variant and adjusted the parameter type:
```rust
// Before (doesn't compile):
_ => Err(ConfigError::UnsupportedProtocol(protocol.to_string())),

// After (compiles):
_ => Err(ConfigError::UnexpectedProtocol("Unsupported protocol")),
```

**Files changed:**
- `cli/src/settings.rs:70`
- `cli/src/main.rs:1018`

**Result:** ✅ CLI now compiles successfully

---

### Issue 11: Protocol Mismatch - hello-blueprint binary rejects `tangle-evm`

**Symptom:** After service approval, the manager spawned the hello-blueprint binary but it failed:
```
error: invalid value 'tangle-evm' for '--protocol <PROTOCOL>'
  [possible values: tangleevm]

  tip: a similar value exists: 'tangleevm'
```

**Root Cause:** The hello-blueprint binary was pulling `blueprint-sdk` from the **remote git repository** (v2 branch):
```toml
# hello-blueprint-bin/Cargo.toml
blueprint-sdk = { git = "https://github.com/tangle-network/blueprint", branch = "v2", ... }
```

The remote v2 branch did not have the `#[value(alias = "tangle-evm")]` attribute on the `Protocol::TangleEvm` enum variant, so clap only accepted `tangleevm` (the lowercased variant name).

The local CLI/manager used the local codebase which had the alias, causing a mismatch.

**Fix:** Update hello-blueprint to use **local path dependencies** instead of remote git:

```toml
# hello-blueprint-bin/Cargo.toml - Before:
blueprint-sdk = { git = "https://github.com/tangle-network/blueprint", branch = "v2", default-features = false, features = ["std", "tangle-evm"] }

# hello-blueprint-bin/Cargo.toml - After:
blueprint-sdk = { path = "../../blueprint/crates/sdk", default-features = false, features = ["std", "tangle-evm"] }
```

```toml
# hello-blueprint-lib/Cargo.toml - Before:
blueprint-sdk = { git = "https://github.com/tangle-network/blueprint", branch = "v2", ... }
blueprint-anvil-testing-utils = { git = "https://github.com/tangle-network/blueprint", branch = "v2" }

# hello-blueprint-lib/Cargo.toml - After:
blueprint-sdk = { path = "../../blueprint/crates/sdk", ... }
blueprint-anvil-testing-utils = { path = "../../blueprint/crates/testing-utils/anvil" }
```

**Files changed:**
- `hello-blueprint/hello-blueprint-bin/Cargo.toml`
- `hello-blueprint/hello-blueprint-lib/Cargo.toml`

**Result:** ✅ hello-blueprint now uses the same SDK as the local CLI

---

### Issue 12: Hash Mismatch After Rebuilding Binary

**Symptom:** After rebuilding hello-blueprint with local SDK, the manager failed:
```
Blueprint Manager Closed Unexpectedly: Err(HashMismatch {
  expected: "78f6f7f70a3e19ee99af19336463cc2f03bb1cee96883defd2a5593256217620",
  actual: "e9b586bb9997b50a722e3a2fdfa62240e01944b183d1efe3d4cdd1eba9085ba3"
})
```

**Root Cause:** The on-chain blueprint definition still had the old binary hash. After rebuilding with local SDK, the binary hash changed.

**Fix:** Redeploy the blueprint with the new `definition.json` containing the updated hash:
```bash
# Repackage with new hash
tar -cJf ./dist/hello-blueprint.tar.xz -C ./target/release hello-blueprint
SHA256=$(shasum -a 256 ./target/release/hello-blueprint | awk '{print $1}')
# Update definition.json with new SHA256
# Redeploy
cargo tangle blueprint deploy tangle --network testnet --definition ./dist/definition.json --settings-file ./settings.env
```

**Result:** ✅ New blueprint deployed with ID `1`, hash matches

---

### Session 2 Final Run

**Successful E2E test with local SDK:**

1. **Blueprint ID:** 1 (redeployed with correct hash)
2. **Service ID:** 3
3. **Call ID:** 0
4. **Job Input:** `HelloRequest { name: "Alice" }`
5. **Job Result:** `HelloResponse { message: "Hello, Alice!", operator: "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc" }`

**Verification:**
```bash
cast abi-decode "f()((string,string))" 0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000d48656c6c6f2c20416c6963652100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002a30783363343463646464623661393030666132623538356464323939653033643132666134323933626300000000000000000000000000000000000000000000
# Output: ("Hello, Alice!", "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc")
```

---

### Documentation Updates

**Updated `docs/LOCAL_E2E_TEST_PLAN.md`:**

Added new section "Use Local Blueprint SDK (Required for Local Testing)" after "Fix Rust Version" section, explaining:
- Why local path dependencies are needed
- Commands to update both `hello-blueprint-bin/Cargo.toml` and `hello-blueprint-lib/Cargo.toml`
- Explanation of protocol parsing mismatch issue

---

### Session 2 Summary

**Bug fixes applied:**
1. `cli/src/settings.rs` - Changed `UnsupportedProtocol` to `UnexpectedProtocol`
2. `cli/src/main.rs` - Same fix

**Key learning:** When testing locally, the hello-blueprint binary must use **local path dependencies** to the blueprint SDK, not the remote git repository. This ensures the binary uses the same SDK version as the CLI/manager, avoiding protocol parsing mismatches.

**All changes are backward compatible** - the `tangle-evm` alias was already added in Session 1 (Issue 8).

---

### Session 1 Outcome (continued)

**Final run:** ✅ Success
- Request ID: `11`
- Service ID: `11`
- Call ID: `0`
- Job result decoded as: `("Hello, Alice!", "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc")`

**Commands used (final run):**
```bash
# Start manager with fresh data dir (avoid RocksDB lock)
cargo tangle blueprint run -p tangleevm -k ./operator-keystore -f ./settings.env -d ./data-alt

# Request + approve
cargo tangle blueprint service request ... --blueprint-id 0 --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 --ttl 3600
cargo tangle blueprint service approve ... --request-id 11 --restaking-percent 100

# Submit + watch
PAYLOAD=$(cast abi-encode "f((string))" "(Alice)")
cargo tangle blueprint jobs submit ... --service-id 11 --job 0 --payload-hex $PAYLOAD
cargo tangle blueprint jobs watch ... --service-id 11 --call-id 0 --timeout-secs 120

# Decode response (call output)
cast abi-decode "f()((string,string))" <result_hex>
```

**Notes:**
- The manager log reported `NoMatchingBinary` and `PROTOCOL` mismatch until fixes in Issue 7/8 were applied.
- The manager exited once due to RocksDB lock at `./data/private/auth-proxy/db/LOCK`; using `-d ./data-alt` resolved it.

---

## Session 3: 2026-01-19 (Clean Slate Verification)

### Purpose

Verify the full E2E test plan works from a clean slate after all bug fixes from Sessions 1 and 2 have been applied.

### Environment

- macOS ARM64 (Apple Silicon)
- Rust 1.88
- All previous bug fixes already in the codebase

### Process

**Cleanup:**
```bash
pkill -f "anvil"
pkill -f "python.*http.server"
rm -rf /Users/tlinhsmacbook/development/tangle/hello-blueprint
```

**Step 1: Create New Blueprint**
```bash
cd /Users/tlinhsmacbook/development/tangle
cargo tangle blueprint create --name hello-blueprint --skip-prompts \
  --project-description "Hello Blueprint for local E2E testing" \
  --project-authors "Tangle"
```
Result: ✅ Success

**Step 1b: Fix Rust Version and Use Local SDK**
```bash
cd hello-blueprint
sed -i '' 's/channel = "1.86"/channel = "1.88"/' rust-toolchain.toml

# Update Cargo.toml files to use local path dependencies
sed -i '' 's|blueprint-sdk = { git = "https://github.com/tangle-network/blueprint", branch = "v2"|blueprint-sdk = { path = "../../blueprint/crates/sdk"|' hello-blueprint-bin/Cargo.toml
sed -i '' 's|blueprint-sdk = { git = "https://github.com/tangle-network/blueprint", branch = "v2"|blueprint-sdk = { path = "../../blueprint/crates/sdk"|' hello-blueprint-lib/Cargo.toml
sed -i '' 's|blueprint-anvil-testing-utils = { git = "https://github.com/tangle-network/blueprint", branch = "v2" }|blueprint-anvil-testing-utils = { path = "../../blueprint/crates/testing-utils/anvil" }|' hello-blueprint-lib/Cargo.toml
```
Result: ✅ Success

**Step 2: Start Anvil**
```bash
anvil --host 0.0.0.0 --port 8545 --base-fee 0 --gas-price 0 --gas-limit 100000000 --hardfork cancun --code-size-limit 50000
```
Result: ✅ Running on port 8545

**Step 3: Package and Serve Artifacts**
```bash
# Build (with macOS env vars)
export MACOSX_DEPLOYMENT_TARGET=14.0
export SDKROOT=$(xcrun --show-sdk-path)
export CXXFLAGS="-isysroot $SDKROOT -I$SDKROOT/usr/include/c++/v1 -stdlib=libc++"
export PROTOC=/opt/homebrew/bin/protoc
cargo build --release -p hello-blueprint-bin

# Package - NOTE: target is inside hello-blueprint, not sibling
mkdir -p ./dist
tar -cJf ./dist/hello-blueprint.tar.xz -C ./target/release hello-blueprint

# Compute SHA256 and create definition.json/dist.json
SHA256=$(shasum -a 256 ./target/release/hello-blueprint | awk '{print $1}')
# Binary SHA256: e9b586bb9997b50a722e3a2fdfa62240e01944b183d1efe3d4cdd1eba9085ba3

# Created definition.json and dist.json with correct paths

# Serve artifacts
cd ./dist && python3 -m http.server 8081
```
Result: ✅ HTTP server running on port 8081

**Note:** The LOCAL_E2E_TEST_PLAN.md mentions `tar -cJf ./dist/hello-blueprint.tar.xz -C ../target/release hello-blueprint` but the actual path is `./target/release` since hello-blueprint is its own workspace with its own target directory.

**Step 4: Deploy Contracts**
```bash
cd /Users/tlinhsmacbook/development/tangle/tnt-core
forge script script/v2/DeployContractsOnly.s.sol:DeployContractsOnly \
  --rpc-url http://127.0.0.1:8545 \
  --broadcast \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
  --disable-code-size-limit \
  -vvv
```
Result: ✅ Success
- TANGLE: `0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9`
- RESTAKING: `0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512`
- STATUS_REGISTRY: `0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf`

**Step 5: Setup Operator Keystore**
```bash
cd hello-blueprint
mkdir -p ./operator-keystore
cargo tangle key import --key-type ecdsa \
  --secret 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \
  --keystore-path ./operator-keystore \
  --protocol tangle-evm
```
Result: ✅ Success - Public key: `02ba5734d8f7091719471e7f7ed6b9df170dc70cc661ca05e688601ad984f068b0`

**Step 6: Register as Restaking Operator**
```bash
cast send 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "registerOperator()" \
  --value 2ether \
  --private-key 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \
  --rpc-url http://127.0.0.1:8545

# Verify
cast call 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "isOperator(address)(bool)" \
  0x70997970C51812dc3A010C7d01b50e0d17dc79C8 --rpc-url http://127.0.0.1:8545
# Returns: true
```
Result: ✅ Success

**Step 7: Deploy Blueprint On-Chain**
```bash
# Create settings.env
cat > ./settings.env << 'EOF'
HTTP_RPC_URL=http://127.0.0.1:8545
WS_RPC_URL=ws://127.0.0.1:8546
KEYSTORE_PATH=./operator-keystore
BLUEPRINT_KEYSTORE_URI=./operator-keystore
TANGLE_CONTRACT=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
RESTAKING_CONTRACT=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
STATUS_REGISTRY_CONTRACT=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf
BLUEPRINT_ID=0
SERVICE_ID=0
EOF

cargo tangle blueprint deploy tangle \
  --network testnet \
  --definition ./dist/definition.json \
  --settings-file ./settings.env
```
Result: ✅ Success - Blueprint ID: `0`

**Step 8: Register Operator for Blueprint**
```bash
cargo tangle blueprint register \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  --restaking-contract 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
  --status-registry-contract 0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf \
  --blueprint-id 0 \
  --rpc-endpoint "http://localhost:9000"
```
Result: ✅ Success - Operator `0x70997970C51812dc3A010C7d01b50e0d17dc79C8` registered

**Step 9: Setup User Keystore**
```bash
mkdir -p ./user-keystore
cargo tangle key import --key-type ecdsa \
  --secret 5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a \
  --keystore-path ./user-keystore \
  --protocol tangle-evm
```
Result: ✅ Success - Public key: `039d9031e97dd78ff8c15aa86939de9b1e791066a0224e331bc962a2099a7b1f04`

**Step 10: Request Service**
```bash
cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  --restaking-contract 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
  --status-registry-contract 0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf \
  --blueprint-id 0 \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --ttl 3600
```
Result: ✅ Success - Request ID: `0`

**Step 11: Run the Operator**
```bash
cargo tangle blueprint run \
  -p tangle-evm \
  -k ./operator-keystore \
  -f ./settings.env \
  -d ./data
```
Result: ✅ Manager running
```
Starting blueprint manager for blueprint ID: 0
Preparing Blueprint to run, this may take a few minutes...
Starting blueprint execution...
Blueprint is running. Press Ctrl+C to stop.
```

**Step 12: Approve Service**
```bash
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  --restaking-contract 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
  --status-registry-contract 0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf \
  --request-id 0 \
  --restaking-percent 100
```
Result: ✅ Success - Service ID: `0`

**Step 13: Submit a Job**
```bash
PAYLOAD=$(cast abi-encode "f((string))" "(Alice)")
# Payload: 0x0000...416c696365...

cargo tangle blueprint jobs submit \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  --restaking-contract 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
  --status-registry-contract 0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf \
  --blueprint-id 0 \
  --service-id 0 \
  --job 0 \
  --payload-hex $PAYLOAD
```
Result: ✅ Success - Call ID: `0`

**Step 14: Watch for Result**
```bash
cargo tangle blueprint jobs watch \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
  --restaking-contract 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
  --status-registry-contract 0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf \
  --blueprint-id 0 \
  --service-id 0 \
  --call-id 0 \
  --timeout-secs 120
```
Result: ✅ Success
```
Job result ready (256 bytes): 0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000d48656c6c6f2c20416c6963652100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002a30783363343463646464623661393030666132623538356464323939653033643132666134323933626300000000000000000000000000000000000000000000
```

**Decode Result:**
```bash
cast abi-decode "f()((string,string))" 0x000000...
# Output: ("Hello, Alice!", "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc")
```

### Session 3 Summary

| Step | Description | Status |
|------|-------------|--------|
| 1 | Create new blueprint | ✅ |
| 2 | Start Anvil | ✅ |
| 3 | Package and serve artifacts | ✅ |
| 4 | Deploy contracts | ✅ |
| 5 | Setup operator keystore | ✅ |
| 6 | Register as restaking operator | ✅ |
| 7 | Deploy blueprint on-chain | ✅ |
| 8 | Register operator for blueprint | ✅ |
| 9 | Setup user keystore | ✅ |
| 10 | Request service | ✅ |
| 11 | Run the operator | ✅ |
| 12 | Approve service | ✅ |
| 13 | Submit a job | ✅ |
| 14 | Watch for result | ✅ |

**Final Result:**
- Blueprint ID: `0`
- Service ID: `0`
- Call ID: `0`
- Input: `HelloRequest { name: "Alice" }`
- Output: `HelloResponse { message: "Hello, Alice!", operator: "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc" }`

**No new issues encountered.** All bug fixes from Sessions 1 and 2 are working correctly. The E2E test plan is now verified to work from a clean slate.

**Minor documentation note:** The LOCAL_E2E_TEST_PLAN.md uses `../target/release` for the binary path, but the actual path is `./target/release` since hello-blueprint has its own workspace and target directory. This is already handled correctly in the current version of the test plan.

---
