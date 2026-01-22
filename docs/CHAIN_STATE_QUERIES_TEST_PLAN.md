# Chain State Queries Commands Test Plan

This document provides a comprehensive test plan for all chain state query commands in `cargo-tangle`. These commands are read-only operations that query the blockchain for blueprint, service, and request information.

**Target Commands:**
1. `list blueprints` - List all registered blueprints on-chain
2. `list requests` - List all pending service requests
3. `list services` - List all active services on the network

**Last Updated:** 2026-01-22

---

## Prerequisites

### Required Components
1. **Service Lifecycle Test Environment** - Complete Phase 0 from `SERVICE_LIFECYCLE_TEST_PLAN.md` first
2. **Deployed Blueprint** - At least one blueprint must be deployed
3. **Active Service** (Optional) - For testing `list services` with actual data
4. **Pending Request** (Optional) - For testing `list requests` with actual data

### macOS C++ Build Fix

If you encounter RocksDB C++ compilation errors like `fatal error: 'memory' file not found`, run this before cargo commands:

```bash
export SDKROOT=$(xcrun --show-sdk-path) && export CXXFLAGS="-isysroot $SDKROOT -I$SDKROOT/usr/include/c++/v1 -stdlib=libc++"
```

### Verify Prerequisites
```bash
# Check contracts are deployed
cast call $TANGLE "blueprintCount()(uint64)" --rpc-url http://127.0.0.1:8545

# Check if there are any blueprints
cast call $TANGLE "blueprintCount()(uint64)" --rpc-url http://127.0.0.1:8545

# Check if there are any services
cast call $TANGLE "serviceCount()(uint64)" --rpc-url http://127.0.0.1:8545
```

---

## Directory Structure

This test plan uses the same environment from Service Lifecycle tests:
```
service-lifecycle-test/
└── svc-test-blueprint/
    ├── operator-keystore/   # Operator 1 keys (Anvil account 1)
    ├── user-keystore/       # User keys (Anvil account 2)
    ├── settings.env         # Environment configuration
    └── dist/                # Blueprint artifacts
```

---

## Terminal Overview

This test plan requires **2 terminals** (minimal):

| Terminal | Purpose | Steps |
|----------|---------|-------|
| Terminal 1 | Anvil (local blockchain) | From Service Lifecycle Setup |
| Terminal 2 | CLI commands (list tests) | All Test Sections |

---

## Understanding Chain State Queries Architecture

### Command Hierarchy

The chain state query commands are organized under the `list` subcommand:

```
cargo tangle blueprint list [SUBCOMMAND]
                         ├── blueprints    # List all registered blueprints
                         ├── requests      # List all pending service requests
                         └── services      # List all active services
```

**Aliases:** The `list` command has a visible alias `ls`:
```bash
cargo tangle blueprint ls blueprints
cargo tangle blueprint ls requests
cargo tangle blueprint ls services
```

### Contract Interactions

The list commands interact with the following contract functions:

**ITangle (at `$TANGLE`):**
- `blueprintCount()` - Get total number of blueprints
- `getBlueprintInfo(uint64)` - Get details for a specific blueprint
- `serviceCount()` - Get total number of services
- `getServiceInfo(uint64)` - Get details for a specific service
- `serviceRequestCount()` - Get total number of service requests
- `getServiceRequest(uint64)` - Get details for a specific request

### Key Differences from Service Commands

| Feature | `list blueprints` | `list requests` | `list services` | `service list` | `service list-requests` |
|---------|-------------------|-----------------|-----------------|----------------|------------------------|
| **JSON Output** | No | No | No | Yes (`--json`) | Yes (`--json`) |
| **Alias** | `ls blueprints` | `ls requests` | `ls services` | N/A | N/A |
| **Output Mode** | Human-readable only | Human-readable only | Human-readable only | Configurable | Configurable |

**Implementation Note:** The `list requests` and `list services` commands use the same underlying functions as `service list-requests` and `service list`, but always pass `false` for the JSON output flag.

### Output Fields

**list blueprints:**
- Blueprint ID
- Owner address
- Manager address
- Created At timestamp
- Operator Count
- Membership Model (Static/Dynamic)
- Pricing Model
- Active status

**list requests:**
- Request ID
- Blueprint ID
- Requester address
- Created At timestamp
- TTL
- Operator Count
- Approval Count
- Payment Token address
- Payment Amount
- Membership Model
- Operator Bounds (min/max)
- Rejected status

**list services:**
- Service ID
- Blueprint ID
- Owner address
- Created At timestamp
- TTL
- Operator Bounds (min/max)
- Membership Model
- Pricing Model
- Status (Active/Terminated/etc.)

---

## Phase 0: Environment Setup

### Step 0.1: Verify Environment from Service Lifecycle Tests

Ensure the service lifecycle test environment is running:

```bash
# Terminal 1: Anvil should be running
# Terminal 2 (optional): HTTP server if deploying new blueprints

# Set environment variables
export TANGLE=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
export RESTAKING=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
export STATUS_REGISTRY=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf

cd /path/to/service-lifecycle-test/svc-test-blueprint
```

### Step 0.2: Verify Contract State

```bash
# Check blueprint count
cast call $TANGLE "blueprintCount()(uint64)" --rpc-url http://127.0.0.1:8545

# Check service count
cast call $TANGLE "serviceCount()(uint64)" --rpc-url http://127.0.0.1:8545

# Check service request count
cast call $TANGLE "serviceRequestCount()(uint64)" --rpc-url http://127.0.0.1:8545
```

### Step 0.3: Create Test Data (If Needed)

If no blueprints/services exist, create them using the service lifecycle test plan commands:

```bash
# Deploy a blueprint (if none exist)
cargo tangle blueprint deploy tangle \
  --network testnet \
  --definition ./dist/definition.json \
  --settings-file ./settings.env

# Create a service request (if none exist)
cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --ttl 3600
```

---

## Phase 1: List Blueprints Command

### Test 1.1: List Blueprints (Basic)

**Goal:** Query and display all registered blueprints

```bash
cargo tangle blueprint list blueprints \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Blueprints
# =============================================
# Blueprint ID: 0
# Owner: 0x...
# Manager: 0x...
# Created At: <timestamp>
# Operator Count: <count>
# Membership Model: <Static|Dynamic>
# Pricing Model: <model>
# Active: <true|false>
# =============================================
```

**Verification:**
- Command executes without error
- Output displays all registered blueprints
- All expected fields are shown

### Test 1.2: List Blueprints with Alias

**Goal:** Verify the `ls` alias works correctly

```bash
cargo tangle blueprint ls blueprints \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Same output as Test 1.1
```

### Test 1.3: List Blueprints (Empty State)

**Goal:** Verify behavior when no blueprints are registered

**Setup Option A: Fresh Anvil Instance**

Start a new Anvil instance and deploy contracts without creating any blueprints:

```bash
# Terminal 1: Start fresh Anvil
anvil --block-time 2

# Terminal 2: Deploy contracts only (no blueprints)
cd /path/to/blueprint
forge script script/LocalDeploy.s.sol:LocalDeploy --rpc-url http://127.0.0.1:8545 --broadcast

# Set new contract addresses from deployment output
export TANGLE_EMPTY=<new_tangle_address>
export RESTAKING_EMPTY=<new_restaking_address>
```

**Setup Option B: Use Mock Contract Address**

Use a freshly deployed contract with no blueprints or verify the blueprint count is 0:

```bash
# Verify no blueprints exist
cast call $TANGLE_EMPTY "blueprintCount()(uint64)" --rpc-url http://127.0.0.1:8545
# Expected: 0
```

**Test Command:**

```bash
cargo tangle blueprint list blueprints \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE_EMPTY \
  --restaking-contract $RESTAKING_EMPTY
```

**Expected Output:**
```
No blueprints registered
```

**Verification:**
- Command exits successfully (exit code 0)
- Message is displayed in yellow (visual verification)
- No blueprint data is printed
- No separator lines (`=====`) are shown

### Test 1.4: List Multiple Blueprints

**Goal:** Verify correct listing of multiple blueprints

> **Prerequisites:** Deploy multiple blueprints first

```bash
cargo tangle blueprint list blueprints \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: List shows all blueprints (0, 1, 2, etc.)
```

---

## Phase 2: List Requests Command

### Test 2.1: List Requests (Basic)

**Goal:** Query and display all pending service requests

```bash
cargo tangle blueprint list requests \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Service Requests
# =============================================
# Request ID: 0
# Blueprint ID: 0
# Requester: 0x...
# Created At: <timestamp>
# TTL: <ttl>
# Operator Count: <count>
# Approval Count: <count>
# Payment Token: 0x...
# Payment Amount: <amount>
# Membership: <model>
# Operator Bounds: <min> - <max>
# Rejected: <true|false>
# =============================================
```

**Verification:**
- Command executes without error
- Output displays all service requests
- All expected fields are shown

### Test 2.2: List Requests with Alias

**Goal:** Verify the `ls` alias works correctly

```bash
cargo tangle blueprint ls requests \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Same output as Test 2.1
```

### Test 2.3: List Requests (Empty State)

**Goal:** Verify behavior when no service requests exist

**Setup Option A: Fresh Environment (Preferred)**

Use a fresh deployment where no service requests have been created:

```bash
# Verify no service requests exist
cast call $TANGLE "serviceRequestCount()(uint64)" --rpc-url http://127.0.0.1:8545
# Expected: 0
```

**Setup Option B: Clear Existing Requests**

If requests exist, approve or reject all pending requests to clear them:

```bash
# Get current request count
cast call $TANGLE "serviceRequestCount()(uint64)" --rpc-url http://127.0.0.1:8545

# For each pending request, either approve or reject it
# (Note: Even rejected requests may still appear in the list)
```

**Setup Option C: Fresh Anvil with Contracts Only**

Start fresh Anvil, deploy contracts, and deploy a blueprint (but don't create any service requests):

```bash
# Deploy contracts and blueprint (see Phase 0 setup)
# Do NOT run the service request command
```

**Test Command:**

```bash
cargo tangle blueprint list requests \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING
```

**Expected Output:**
```
No service requests found
```

**Verification:**
- Command exits successfully (exit code 0)
- Message is displayed in yellow (visual verification)
- No request data is printed
- No "Service Requests" header is shown
- No separator lines (`=====`) are shown

**Alternative Test with Alias:**

```bash
cargo tangle blueprint ls requests \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Same output - "No service requests found"
```

### Test 2.4: List Multiple Requests

**Goal:** Verify correct listing of multiple requests

> **Prerequisites:** Create multiple service requests first

```bash
cargo tangle blueprint list requests \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: List shows all requests with different statuses
```

### Test 2.5: Verify Requests Include Rejected

**Goal:** Confirm rejected requests are still shown in the list

> **Prerequisites:** Have at least one rejected request

```bash
# Create a request, then reject it:
cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --ttl 3600

# Reject it:
cargo tangle blueprint service reject \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id <REQUEST_ID>

# List requests - should show the rejected request with Rejected: true
cargo tangle blueprint list requests \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Request shown with "Rejected: true"
```

---

## Phase 3: List Services Command

### Test 3.1: List Services (Basic)

**Goal:** Query and display all registered services

```bash
cargo tangle blueprint list services \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Services
# =============================================
# Service ID: 0
# Blueprint ID: 0
# Owner: 0x...
# Created At: <timestamp>
# TTL: <ttl>
# Operator Bounds: <min> - <max>
# Membership: <model>
# Pricing: <model>
# Status: <Active|Terminated|...>
# =============================================
```

**Verification:**
- Command executes without error
- Output displays all services
- All expected fields are shown

### Test 3.2: List Services with Alias

**Goal:** Verify the `ls` alias works correctly

```bash
cargo tangle blueprint ls services \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Same output as Test 3.1
```

### Test 3.3: List Services (Empty State)

**Goal:** Verify behavior when no services exist

**Setup Option A: Fresh Environment (Preferred)**

Use a fresh deployment where no services have been created:

```bash
# Verify no services exist
cast call $TANGLE "serviceCount()(uint64)" --rpc-url http://127.0.0.1:8545
# Expected: 0
```

**Setup Option B: Environment with Only Blueprints**

Deploy blueprint(s) but don't create any services (i.e., don't create and approve service requests):

```bash
# After deploying blueprint, verify service count is still 0
cast call $TANGLE "serviceCount()(uint64)" --rpc-url http://127.0.0.1:8545
# Expected: 0

# Blueprint count should be > 0
cast call $TANGLE "blueprintCount()(uint64)" --rpc-url http://127.0.0.1:8545
# Expected: 1 (or more)
```

**Setup Option C: Fresh Anvil Instance**

Start fresh Anvil with deployed contracts but no services:

```bash
# Terminal 1: Start fresh Anvil
anvil --block-time 2

# Terminal 2: Deploy contracts and optionally a blueprint
# Do NOT create or approve any service requests
```

**Test Command:**

```bash
cargo tangle blueprint list services \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING
```

**Expected Output:**
```
No services found
```

**Verification:**
- Command exits successfully (exit code 0)
- Message is displayed in yellow (visual verification)
- No service data is printed
- No "Services" header is shown
- No separator lines (`=====`) are shown

**Alternative Test with Alias:**

```bash
cargo tangle blueprint ls services \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Same output - "No services found"
```

**Comparison with `service list` Command:**

```bash
# The service list command should also show empty state
cargo tangle blueprint service list \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: "No services found"

# With JSON flag (empty array)
cargo tangle blueprint service list --json \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: []
```

### Test 3.4: List Multiple Services

**Goal:** Verify correct listing of multiple services

> **Prerequisites:** Create multiple services first

```bash
cargo tangle blueprint list services \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: List shows all services (0, 1, 2, etc.) with various statuses
```

### Test 3.5: Verify Services Include Different Statuses

**Goal:** Confirm services with different statuses are shown correctly

```bash
# List services and verify:
# - Active services show "Status: Active"
# - Terminated services show "Status: Terminated" (if any)
# - Expired services show appropriate status

cargo tangle blueprint list services \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING
```

---

## Phase 4: Comparison with Service Commands

### Test 4.1: Compare `list services` vs `service list`

**Goal:** Verify both commands return the same data (but different format options)

```bash
# Run list services (no JSON support)
cargo tangle blueprint list services \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Run service list (with JSON support)
cargo tangle blueprint service list \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Both should show the same services with same data
```

### Test 4.2: Compare `list requests` vs `service list-requests`

**Goal:** Verify both commands return the same data

```bash
# Run list requests (no JSON support)
cargo tangle blueprint list requests \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Run service list-requests (with JSON support)
cargo tangle blueprint service list-requests \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Both should show the same requests with same data
```

---

## Phase 5: Error Handling and Edge Cases

### Test 5.1: Invalid Contract Address

**Goal:** Verify error handling for invalid Tangle contract address

```bash
cargo tangle blueprint list blueprints \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract 0x0000000000000000000000000000000000000001 \
  --restaking-contract $RESTAKING

# Expected: Error - contract call fails or returns empty
```

### Test 5.2: Invalid RPC URL

**Goal:** Verify error handling for unreachable RPC

```bash
cargo tangle blueprint list blueprints \
  --http-rpc-url http://127.0.0.1:9999 \
  --ws-rpc-url ws://127.0.0.1:9999 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Error - connection refused or timeout
```

### Test 5.3: Missing Required Arguments

**Goal:** Verify CLI argument validation

```bash
# Missing --tangle-contract
cargo tangle blueprint list blueprints \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --restaking-contract $RESTAKING

# Expected: Error - missing required argument
```

### Test 5.4: Large Dataset Performance

**Goal:** Verify command handles many blueprints/services

> **Note:** This is an optional stress test

```bash
# If many blueprints exist (10+), verify:
# - Command completes in reasonable time
# - All items are listed
# - No truncation or pagination issues

cargo tangle blueprint list blueprints \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING
```

---

## Cleanup

```bash
# Stop all processes (if running)
pkill -f "anvil"
pkill -f "python.*http.server"

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

---

## Quick Reference: Contract Addresses (Anvil Deterministic)

| Contract | Address |
|----------|---------|
| Tangle | `0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9` |
| Restaking | `0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512` |
| Status Registry | `0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf` |

---

## Quick Reference: Command Summary

| Command | Description | Key Flags |
|---------|-------------|-----------|
| `list blueprints` | List all registered blueprints | Network args only |
| `list requests` | List all pending service requests | Network args only |
| `list services` | List all active services | Network args only |

### Command Aliases

| Full Command | Alias |
|--------------|-------|
| `blueprint list blueprints` | `blueprint ls blueprints` |
| `blueprint list requests` | `blueprint ls requests` |
| `blueprint list services` | `blueprint ls services` |

---

## Quick Reference: Common Flags

All commands support these flags:
- `--http-rpc-url` - HTTP RPC endpoint (default: `http://127.0.0.1:8545`)
- `--ws-rpc-url` - WebSocket RPC endpoint (default: `ws://127.0.0.1:8546`)
- `--keystore-path` - Path to keystore directory (default: `./keystore`)
- `--tangle-contract` - Tangle contract address
- `--restaking-contract` - Restaking contract address

**Note:** Unlike `service list` and `service list-requests`, these commands do **NOT** support `--json` output. If JSON output is needed, use the `service` subcommands instead.

---

## Known Limitations

### Limitation #1: No JSON Output Support

**Affected Commands:** `list blueprints`, `list requests`, `list services`

**Description:** These commands only support human-readable output. They do not have a `--json` flag like the equivalent `service list` and `service list-requests` commands.

**Workaround:** Use the `service` subcommands instead:
- `service list --json` instead of `list services`
- `service list-requests --json` instead of `list requests`

**Note:** There is no `service` equivalent for `list blueprints`. If JSON output for blueprints is needed, consider using `cast` or direct contract calls.

### Limitation #2: No Filtering Options

**Affected Commands:** All list commands

**Description:** Commands do not support filtering by owner, status, blueprint ID, etc.

**Workaround:** Use `jq` or other tools to filter the JSON output from equivalent service commands, or use direct contract calls with `cast`.

---

## Feature Requests (Discovered During Testing)

Document any feature requests that arise during testing here:

1. **JSON Output for List Commands** - Add `--json` flag to `list blueprints`, `list requests`, `list services` for consistency with service commands
2. **Filtering Options** - Add ability to filter by owner, status, or blueprint ID
3. **Pagination** - Add pagination for large datasets

---

## Test Summary

| Phase | Total Tests | Description |
|-------|-------------|-------------|
| Phase 0 | 3 | Environment Setup |
| Phase 1 | 4 | List Blueprints |
| Phase 2 | 5 | List Requests |
| Phase 3 | 5 | List Services |
| Phase 4 | 2 | Comparison with Service Commands |
| Phase 5 | 4 | Error Handling and Edge Cases |
| **Total** | **23** | |

**Note:** Empty state tests (1.3, 2.3, 3.3) require a fresh deployment with no data. See the progress tracker (`CHAIN_STATE_QUERIES_TEST_PROGRESS.md` Phase 6) for detailed setup and execution instructions.
