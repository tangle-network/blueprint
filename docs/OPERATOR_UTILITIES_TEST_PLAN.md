# Operator Utilities Commands Test Plan

This document provides a comprehensive test plan for all operator utility commands in `cargo-tangle`. It builds upon the test environment established in the Service Lifecycle tests and covers operator registration, staking, heartbeat/status, and lifecycle management.

**Target Commands:**
1. `operator register` - Register as an operator on the restaking layer
2. `operator increase-stake` - Add more stake to operator balance
3. `operator show-status` - Display operator heartbeat and status info
4. `operator submit-heartbeat` - Send a heartbeat to prove operator liveness
5. `operator show-restaking` - Show current restaking status and stake amounts
6. `operator list-delegators` - List all delegators staking with this operator
7. `operator join-service` - Join an existing dynamic service (duplicate of `service join`)
8. `operator leave-service` - Leave a dynamic service (duplicate of `service leave`)
9. `operator schedule-unstake` - Schedule an unstake operation
10. `operator execute-unstake` - Execute a matured unstake operation
11. `operator schedule-leaving` - Schedule operator departure from network
12. `operator complete-leaving` - Finalize operator departure after cooldown

**Last Updated:** 2026-01-22

---

## Prerequisites

### Required Components
1. **Service Lifecycle Test Environment** - Complete Phase 0 from `SERVICE_LIFECYCLE_TEST_PLAN.md` first
2. **Active Service** - At least one dynamic service must be created for status/heartbeat tests
3. **Operator Running** - Blueprint manager should be running for heartbeat tests

### macOS C++ Build Fix

If you encounter RocksDB C++ compilation errors like `fatal error: 'memory' file not found`, run this before cargo commands:

```bash
export SDKROOT=$(xcrun --show-sdk-path) && export CXXFLAGS="-isysroot $SDKROOT -I$SDKROOT/usr/include/c++/v1 -stdlib=libc++"
```

### Verify Prerequisites
```bash
# Check contracts are deployed
cast call $RESTAKING "bondToken()(address)" --rpc-url http://127.0.0.1:8545

# Check if operator is already registered
cast call $RESTAKING "isOperator(address)(bool)" 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 --rpc-url http://127.0.0.1:8545

# Check service is active (for status/heartbeat tests)
cargo tangle blueprint service list \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING
```

---

## Directory Structure

This test plan uses the same environment from Service Lifecycle tests:
```
service-lifecycle-test/
└── svc-test-blueprint/
    ├── operator-keystore/   # Operator 1 keys (Anvil account 1)
    ├── operator2-keystore/  # Operator 2 keys (Anvil account 3)
    ├── operator3-keystore/  # Operator 3 keys (Anvil account 4) - for new registration tests
    ├── user-keystore/       # User keys (Anvil account 2)
    ├── settings.env         # Environment configuration
    └── dist/                # Blueprint artifacts
```

---

## Terminal Overview

This test plan requires **4 terminals**:

| Terminal | Purpose | Steps |
|----------|---------|-------|
| Terminal 1 | Anvil (local blockchain) | From Service Lifecycle Setup |
| Terminal 2 | HTTP server (artifact hosting) | From Service Lifecycle Setup |
| Terminal 3 | CLI commands (operator tests) | All Test Sections |
| Terminal 4 | Blueprint manager (for heartbeat tests) | Optional |

---

## Understanding Operator Utilities Architecture

### Contract Interactions

The operator commands interact with three primary contracts:

1. **ITangle** (at `$TANGLE`)
   - `joinService(serviceId, exposureBps)` - Join a dynamic service
   - `leaveService(serviceId)` - Leave a service (immediate, requires exitQueueDuration=0)

2. **IMultiAssetDelegation** (at `$RESTAKING`)
   - `registerOperator()` / `registerOperatorWithAsset()` - Register as operator
   - `increaseStake()` / `increaseStakeWithAsset()` - Add stake
   - `scheduleOperatorUnstake(amount)` - Schedule unstake with delay
   - `executeOperatorUnstake()` - Execute matured unstake
   - `startLeaving()` - Begin operator departure
   - `completeLeaving()` - Finalize departure
   - `getOperatorMetadata(operator)` - Query operator info
   - `getOperatorSelfStake(operator)` - Query self stake
   - `getOperatorDelegatedStake(operator)` - Query delegated stake
   - `getOperatorDelegators(operator)` - List delegators

3. **IOperatorStatusRegistry** (at `$STATUS_REGISTRY`)
   - `getLastHeartbeat(serviceId, operator)` - Query last heartbeat
   - `getOperatorStatus(serviceId, operator)` - Query status code
   - `isOnline(serviceId, operator)` - Check online status
   - `submitHeartbeat(...)` - Submit signed heartbeat

### Bond Token Auto-Detection

The CLI automatically detects the bond token type:
- If `bondToken() == Address::ZERO` → Use native ETH (send with value)
- If `bondToken() != Address::ZERO` → Use ERC20 (auto-approve then call with token)

### Operator Lifecycle

```
Unregistered → register → Active Operator
                             ↓
                    increase-stake (add more)
                             ↓
              schedule-unstake (with delay)
                             ↓
              execute-unstake (after delay)
                             ↓
               start-leaving (schedule departure)
                             ↓
             complete-leaving (finalize, becomes unregistered)
```

---

## Phase 0: Environment Verification

### Step 0.1: Verify Environment from Service Lifecycle Tests

Ensure the service lifecycle test environment is running:

```bash
# Terminal 1: Anvil should be running
# Terminal 2: HTTP server should be running

# Terminal 3: Verify contracts are deployed
export TANGLE=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
export RESTAKING=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
export STATUS_REGISTRY=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf

cd /path/to/service-lifecycle-test/svc-test-blueprint
```

### Step 0.2: Setup Additional Keystore for New Operator Registration

For testing `operator register`, we need an account that is NOT already registered:

```bash
# Create keystore for Anvil account 4 (index 4)
# Address: 0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65
# Private Key: 0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a

mkdir -p ./operator3-keystore
cargo tangle key import \
  --key-type ecdsa \
  --secret 47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a \
  --keystore-path ./operator3-keystore \
  --protocol tangle-evm

# Fund this account with ETH for gas and registration
cast send 0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65 --value 10ether \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
  --rpc-url http://127.0.0.1:8545
```

### Step 0.3: Ensure Dynamic Service Exists

For status/heartbeat tests, we need an active dynamic service:

```bash
# Check for existing services
cargo tangle blueprint service list \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# If no service exists, create one:
cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --ttl 7200

# Approve it
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id 0

export SERVICE_ID=0
export BLUEPRINT_ID=0
```

---

## Phase 1: Operator Registration and Staking

### Test 1.1: Show Restaking Status (Pre-Registration)

**Goal:** Verify restaking query for an unregistered operator

```bash
cargo tangle blueprint operator restaking \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Restaking Status
# =============================================
# Operator: 0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65
# Stake: 0
# Self Stake: 0
# Delegated Stake: 0
# Delegation Count: 0
# Status: 0 (Inactive)
# Leaving Round: 0
# Commission (bps): <value>
# Current Round: <value>
# =============================================
```

**Verification:**
- Stake should be 0
- Status should be 0 (Inactive)

### Test 1.2: Register New Operator with Native ETH

**Goal:** Register a new operator using native ETH as stake

```bash
# Register with 1 ETH stake
cargo tangle blueprint operator register \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 1000000000000000000

# Expected output:
# Operator registration: submitted tx_hash=0x...
# Operator registration: confirmed block=Some(N) gas_used=...
```

**Verification:**
```bash
# Verify operator is registered
cast call $RESTAKING "isOperator(address)(bool)" 0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65 --rpc-url http://127.0.0.1:8545
# Expected: true
```

### Test 1.3: Show Restaking Status (Post-Registration)

**Goal:** Verify restaking status after registration

```bash
cargo tangle blueprint operator restaking \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Stake: 1000000000000000000 (or similar)
# Status: 1 (Active)
```

### Test 1.4: Show Restaking Status with JSON Output

**Goal:** Verify JSON output format

```bash
cargo tangle blueprint operator restaking \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --json | jq '.'

# Expected JSON structure:
# {
#   "operator": "0x15d34aaf54267db7d7c367839aaf71a00a2c6a65",
#   "stake": "1000000000000000000",
#   "self_stake": "1000000000000000000",
#   "delegated_stake": "0",
#   "delegation_count": 0,
#   "status": 1,
#   "leaving_round": 0,
#   "commission_bps": ...,
#   "current_round": ...
# }
```

### Test 1.5: Increase Stake

**Goal:** Add more stake to an existing operator

```bash
cargo tangle blueprint operator increase-stake \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 500000000000000000

# Expected output:
# Stake increase: submitted tx_hash=0x...
# Stake increase: confirmed block=Some(N) gas_used=...
```

**Verification:**
```bash
# Check new stake amount
cargo tangle blueprint operator restaking \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Stake should now be 1.5 ETH (1500000000000000000)
```

### Test 1.6: Increase Stake with JSON Output

**Goal:** Verify JSON output for stake increase

```bash
cargo tangle blueprint operator increase-stake \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 100000000000000000 \
  --json

# Expected JSON:
# {"event":"tx_submitted","tx_hash":"0x..."}
# {"event":"tx_confirmed","block":N,"gas_used":...}
```

### Test 1.7: Register Already-Registered Operator (Should Fail)

**Goal:** Verify error when registering an already-registered operator

```bash
cargo tangle blueprint operator register \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 1000000000000000000

# Expected: Contract revert error (operator already registered)
```

---

## Phase 2: Operator Status and Heartbeat

> **Note:** These tests require:
> 1. An active dynamic service where the operator is a member
> 2. The `status_registry_contract` to be configured

### Test 2.1: Show Operator Status (No Heartbeat Yet)

**Goal:** Query operator status before any heartbeat is submitted

```bash
cargo tangle blueprint operator status \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --service-id $SERVICE_ID

# Expected output:
# Operator Status
# =============================================
# Service ID: 0
# Operator: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
# Status Code: 0
# Last Heartbeat: 0 (or timestamp)
# Online: false (or true depending on heartbeat expiry)
# =============================================
```

### Test 2.2: Submit Heartbeat

**Goal:** Submit a signed heartbeat to prove operator liveness

```bash
cargo tangle blueprint operator heartbeat \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --service-id $SERVICE_ID

# Expected output:
# Heartbeat: submitted tx_hash=0x...
# Heartbeat: confirmed block=Some(N) gas_used=...
```

### Test 2.3: Show Operator Status (After Heartbeat)

**Goal:** Verify status is updated after heartbeat submission

```bash
cargo tangle blueprint operator status \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --service-id $SERVICE_ID

# Expected:
# Last Heartbeat: <recent timestamp>
# Online: true (if within heartbeat expiry window)
```

### Test 2.4: Submit Heartbeat with Custom Status Code

**Goal:** Submit heartbeat with a non-default status code

```bash
cargo tangle blueprint operator heartbeat \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --service-id $SERVICE_ID \
  --status-code 1

# Expected: Heartbeat submitted with status_code=1
```

### Test 2.5: Show Operator Status with JSON Output

**Goal:** Verify JSON output format for status

```bash
cargo tangle blueprint operator status \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --service-id $SERVICE_ID \
  --json | jq '.'

# Expected JSON:
# {
#   "service_id": 0,
#   "operator": "0x70997970c51812dc3a010c7d01b50e0d17dc79c8",
#   "status_code": 1,
#   "last_heartbeat": <timestamp>,
#   "online": true
# }
```

### Test 2.6: Submit Heartbeat with JSON Output

**Goal:** Verify JSON output for heartbeat submission

```bash
cargo tangle blueprint operator heartbeat \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --service-id $SERVICE_ID \
  --json

# Expected JSON with tx_hash and confirmation details
```

### Test 2.7: Query Status for Different Operator

**Goal:** Query status for a specific operator address

```bash
cargo tangle blueprint operator status \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --service-id $SERVICE_ID \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8

# Expected: Status for operator 1 (even though using user keystore)
```

---

## Phase 3: Delegator Queries

### Test 3.1: List Delegators (No Delegators)

**Goal:** Query delegators for an operator with no delegations

```bash
cargo tangle blueprint operator delegators \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Operator Delegators
# =============================================
# Operator: 0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65
# Delegator Count: 0
# (no delegators listed)
# =============================================
```

### Test 3.2: List Delegators for Different Operator

**Goal:** Query delegators for a specific operator address

```bash
cargo tangle blueprint operator delegators \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8

# Expected: List of delegators for operator 1
```

### Test 3.3: List Delegators with JSON Output

**Goal:** Verify JSON output format

```bash
cargo tangle blueprint operator delegators \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --json | jq '.'

# Expected JSON:
# {
#   "operator": "0x70997970c51812dc3a010c7d01b50e0d17dc79c8",
#   "delegators": []
# }
```

---

## Phase 4: Service Join/Leave (Duplicate Commands)

> **Note:** These are duplicates of `service join` and `service leave`. Testing ensures
> both entry points work correctly.

### Test 4.1: Join Service via Operator Command

**Goal:** Verify `operator join` works as a duplicate of `service join`

```bash
# First, have operator 3 register for the blueprint
cargo tangle blueprint register \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --rpc-endpoint "http://localhost:9002"

# Now join the service
cargo tangle blueprint operator join \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --service-id $SERVICE_ID

# Expected output:
# Service join: submitted tx_hash=0x...
# Service join: confirmed block=Some(N) gas_used=...
# Joined service 0 with exposure 10000 bps
```

### Test 4.2: Join Service with Custom Exposure

**Goal:** Join with specific exposure value

```bash
# Create a new service for this test
cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --ttl 7200

# Approve it
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id 1

# Join with 50% exposure
cargo tangle blueprint operator join \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 1 \
  --exposure-bps 5000

# Expected: Joined with 5000 bps exposure
```

### Test 4.3: Leave Service via Operator Command

**Goal:** Verify `operator leave` works (subject to exit queue limitations)

```bash
cargo tangle blueprint operator leave \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id $BLUEPRINT_ID \
  --service-id $SERVICE_ID

# Expected: May fail with exit queue error (same as service leave)
# Error: ExitNotExecutable or ExitTooEarly
```

> **Note:** Same limitations as `service leave` - requires exit queue handling.
> See Feature Request #1 in SERVICE_LIFECYCLE_TEST_PROGRESS.md.

---

## Phase 5: Unstake Operations

### Test 5.1: Schedule Operator Unstake

**Goal:** Schedule an unstake operation with delay

```bash
cargo tangle blueprint operator schedule-unstake \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 100000000000000000

# Expected output:
# Unstake scheduled: submitted tx_hash=0x...
# Unstake scheduled: confirmed block=Some(N) gas_used=...
```

### Test 5.2: Execute Operator Unstake (Before Delay)

**Goal:** Verify execute fails before delay period

```bash
cargo tangle blueprint operator execute-unstake \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: May be no-op or error if delay not passed
```

### Test 5.3: Execute Operator Unstake (After Delay)

**Goal:** Execute unstake after delay period (use Anvil time manipulation)

```bash
# Advance time past unstake delay (check contract for exact duration)
cast rpc evm_increaseTime 604800 --rpc-url http://127.0.0.1:8545
cast rpc evm_mine --rpc-url http://127.0.0.1:8545

# Execute the unstake
cargo tangle blueprint operator execute-unstake \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Unstake executed successfully
```

### Test 5.4: Schedule Unstake with JSON Output

**Goal:** Verify JSON output format

```bash
cargo tangle blueprint operator schedule-unstake \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 50000000000000000 \
  --json

# Expected JSON with tx_submitted and tx_confirmed events
```

---

## Phase 6: Operator Leaving (Full Departure)

### Test 6.1: Schedule Operator Leaving

**Goal:** Initiate operator departure from the network

```bash
cargo tangle blueprint operator start-leaving \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Start leaving: submitted tx_hash=0x...
# Start leaving: confirmed block=Some(N) gas_used=...
```

**Verification:**
```bash
# Check leaving status
cargo tangle blueprint operator restaking \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Leaving Round should be non-zero
```

### Test 6.2: Complete Operator Leaving (Before Delay)

**Goal:** Verify completion fails before delay period

```bash
cargo tangle blueprint operator complete-leaving \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Error - leaving delay not passed
```

### Test 6.3: Complete Operator Leaving (After Delay)

**Goal:** Complete operator departure after delay

```bash
# Advance time past leaving delay
cast rpc evm_increaseTime 604800 --rpc-url http://127.0.0.1:8545
cast rpc evm_mine --rpc-url http://127.0.0.1:8545

# Complete leaving
cargo tangle blueprint operator complete-leaving \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Leaving completed successfully
```

**Verification:**
```bash
# Check operator is no longer registered
cast call $RESTAKING "isOperator(address)(bool)" 0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65 --rpc-url http://127.0.0.1:8545
# Expected: false
```

### Test 6.4: Schedule Leaving with JSON Output

**Goal:** Verify JSON output format

```bash
# Re-register operator first (if needed for this test)
cargo tangle blueprint operator start-leaving \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --json

# Expected JSON with tx_submitted and tx_confirmed events
```

---

## Phase 7: Error Handling and Edge Cases

### Test 7.1: Register with Zero Amount (Should Fail)

**Goal:** Verify error when registering with zero stake

```bash
cargo tangle blueprint operator register \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator3-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 0

# Expected: Error - insufficient stake or similar
```

### Test 7.2: Increase Stake for Unregistered Operator (Should Fail)

**Goal:** Verify error when increasing stake for non-operator

```bash
# Use a keystore for an unregistered account
cargo tangle blueprint operator increase-stake \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 1000000000000000000

# Expected: Contract revert - not an operator
```

### Test 7.3: Status Without Status Registry (Should Fail)

**Goal:** Verify error when status registry is not configured

```bash
cargo tangle blueprint operator status \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 0 \
  --service-id 0

# Expected: Error about status registry not configured
```

### Test 7.4: Heartbeat Without Status Registry (Should Fail)

**Goal:** Verify error when submitting heartbeat without status registry

```bash
cargo tangle blueprint operator heartbeat \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 0 \
  --service-id 0

# Expected: Error about status registry not configured
```

### Test 7.5: Join Non-Existent Service (Should Fail)

**Goal:** Verify error when joining invalid service

```bash
cargo tangle blueprint operator join \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 99999

# Expected: Contract revert - ServiceNotFound
```

### Test 7.6: Schedule Unstake Exceeding Balance (Should Fail)

**Goal:** Verify error when unstaking more than available

```bash
cargo tangle blueprint operator schedule-unstake \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 999999999999999999999999999999

# Expected: Contract revert - insufficient balance
```

---

## Cleanup

```bash
# Stop all processes
pkill -f "anvil"
pkill -f "python.*http.server"
pkill -f "svc-test-blueprint"

# Remove test artifacts
rm -rf ./operator3-keystore

# Remove test directory (optional)
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
| 4 | `0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65` | `0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a` | Operator 3 (New) |

---

## Quick Reference: Contract Addresses (Anvil Deterministic)

| Contract | Address |
|----------|---------|
| Tangle | `0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9` |
| Restaking | `0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512` |
| Status Registry | `0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf` |

---

## Quick Reference: Command Aliases

| Full Command | Aliases |
|--------------|---------|
| `operator register` | - |
| `operator increase-stake` | - |
| `operator status` | `operator show-status` |
| `operator heartbeat` | `operator submit-heartbeat`, `operator hb` |
| `operator restaking` | `operator show-restaking` |
| `operator delegators` | `operator list-delegators` |
| `operator join` | `operator join-service` |
| `operator leave` | `operator leave-service` |
| `operator schedule-unstake` | `operator unstake` |
| `operator execute-unstake` | - |
| `operator start-leaving` | `operator schedule-leaving` |
| `operator complete-leaving` | - |
