# Delegator Utilities Commands Test Plan

This document provides a comprehensive test plan for all delegator utility commands in `cargo-tangle`. It covers ERC20 token operations, deposits, delegations, undelegations, and withdrawals for delegators (as opposed to operators).

**Target Commands:**
1. `delegator positions` - Display deposits, locks, delegations, and pending requests
2. `delegator delegations` - List all active delegations
3. `delegator pending-unstakes` - List pending delegator unstakes
4. `delegator pending-withdrawals` - List pending delegator withdrawals
5. `delegator balance` - Check ERC20 token balance for an address
6. `delegator allowance` - Check ERC20 allowance for restaking contract
7. `delegator approve` - Approve tokens for use by restaking contract
8. `delegator deposit` - Deposit tokens without delegating to an operator
9. `delegator delegate` - Delegate staked tokens to an operator
10. `delegator undelegate` - Schedule removal of delegation from an operator
11. `delegator execute-unstake` - Execute all matured unstake operations
12. `delegator execute-unstake-withdraw` - Execute a specific unstake and withdraw funds
13. `delegator schedule-withdraw` - Schedule a withdrawal of deposited funds
14. `delegator execute-withdraw` - Execute all matured withdrawal operations

**Last Updated:** 2026-01-22

---

## Prerequisites

### Required Components
1. **Service Lifecycle Test Environment** - Complete Phase 0 from `SERVICE_LIFECYCLE_TEST_PLAN.md` first
2. **Registered Operator** - At least one operator must be registered for delegation tests
3. **ERC20 Token (Optional)** - For ERC20 token tests (can use MockERC20 from test deployments)

### macOS C++ Build Fix

If you encounter RocksDB C++ compilation errors like `fatal error: 'memory' file not found`, run this before cargo commands:

```bash
export SDKROOT=$(xcrun --show-sdk-path) && export CXXFLAGS="-isysroot $SDKROOT -I$SDKROOT/usr/include/c++/v1 -stdlib=libc++"
```

### Verify Prerequisites
```bash
# Check contracts are deployed
cast call $RESTAKING "bondToken()(address)" --rpc-url http://127.0.0.1:8545

# Check if operator is registered (for delegation tests)
cast call $RESTAKING "isOperator(address)(bool)" 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 --rpc-url http://127.0.0.1:8545

# Check current round (for delay calculations)
cast call $RESTAKING "currentRound()(uint64)" --rpc-url http://127.0.0.1:8545
```

---

## Directory Structure

This test plan uses the same environment from Service Lifecycle tests:
```
service-lifecycle-test/
└── svc-test-blueprint/
    ├── operator-keystore/   # Operator 1 keys (Anvil account 1)
    ├── operator2-keystore/  # Operator 2 keys (Anvil account 3)
    ├── delegator-keystore/  # Delegator keys (Anvil account 3)
    ├── user-keystore/       # User keys (Anvil account 2)
    ├── settings.env         # Environment configuration
    └── dist/                # Blueprint artifacts
```

---

## Terminal Overview

This test plan requires **3 terminals**:

| Terminal | Purpose | Steps |
|----------|---------|-------|
| Terminal 1 | Anvil (local blockchain) | From Service Lifecycle Setup |
| Terminal 2 | HTTP server (artifact hosting) | From Service Lifecycle Setup |
| Terminal 3 | CLI commands (delegator tests) | All Test Sections |

---

## Understanding Delegator Utilities Architecture

### Contract Interactions

The delegator commands interact with the IMultiAssetDelegation contract (at `$RESTAKING`):

**View Functions:**
- `getDeposit(address delegator, address token)` - Query deposit info
- `getLocks(address delegator, address token)` - Query lock info
- `getDelegations(address delegator)` - Query all delegations
- `getPendingUnstakes(address delegator)` - Query pending unstakes
- `getPendingWithdrawals(address delegator)` - Query pending withdrawals

**State-Changing Functions:**
- `deposit()` - Deposit native tokens
- `depositERC20(address token, uint256 amount)` - Deposit ERC20 tokens
- `delegate(address operator, uint256 amount)` - Delegate native tokens
- `delegateWithOptions(...)` - Delegate with selection mode and blueprint IDs
- `depositAndDelegate(address operator)` - Deposit and delegate in one transaction
- `depositAndDelegateWithOptions(...)` - Full-featured deposit + delegate
- `scheduleDelegatorUnstake(address operator, address token, uint256 amount)` - Schedule undelegation
- `executeDelegatorUnstake()` - Execute all matured unstakes
- `executeDelegatorUnstakeAndWithdraw(...)` - Execute specific unstake and withdraw
- `scheduleWithdraw(address token, uint256 amount)` - Schedule a withdrawal
- `executeWithdraw()` - Execute all matured withdrawals

**ERC20 Helper Functions:**
- `balanceOf(address owner)` - Query ERC20 balance
- `allowance(address owner, address spender)` - Query ERC20 allowance
- `approve(address spender, uint256 amount)` - Approve ERC20 spending

### Delegator Lifecycle

```
New Delegator                   deposit()
                                   ↓
                            Has Deposit Balance
                                   ↓
                      delegate() / delegateWithOptions()
                                   ↓
                            Active Delegation
                                   ↓
                     scheduleDelegatorUnstake()
                                   ↓
                          Pending Unstake
                                   ↓
               [Wait for delegationBondLessDelay]
                                   ↓
    executeDelegatorUnstake() OR executeDelegatorUnstakeAndWithdraw()
                                   ↓
                        Funds in Deposit Balance
                                   ↓
                         scheduleWithdraw()
                                   ↓
                       Pending Withdrawal
                                   ↓
               [Wait for leaveDelegatorsDelay]
                                   ↓
                         executeWithdraw()
                                   ↓
                   Funds Returned to Delegator
```

### Key Delay Parameters

The contract has several delay parameters that affect when operations can be executed:

- **delegationBondLessDelay**: Rounds to wait after scheduling unstake before executing
- **leaveDelegatorsDelay**: Rounds to wait after scheduling withdrawal before executing
- **Round Duration**: Time (in seconds) for each round (typically 21600 = 6 hours)

### Blueprint Selection Modes

When delegating, delegators can choose how their stake is used for blueprints:

- **All** (default): Stake can be used by any blueprint the operator supports
- **Fixed**: Stake is locked to specific blueprint IDs (specified via `--blueprint-id` flags)

---

## Phase 0: Environment Setup

### Step 0.1: Verify Environment from Service Lifecycle Tests

Ensure the service lifecycle test environment is running:

```bash
# Terminal 1: Anvil should be running
# Terminal 2: HTTP server should be running

# Terminal 3: Set environment variables
export TANGLE=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
export RESTAKING=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
export STATUS_REGISTRY=0x99bbA657f2BbC93c02D617f8bA121cB8Fc104Acf

cd /path/to/service-lifecycle-test/svc-test-blueprint
```

### Step 0.2: Setup Delegator Keystore

For testing delegator commands, we need a separate account from operators:

```bash
# Create keystore for Anvil account 3 (can also be used as delegator)
# Address: 0x90F79bf6EB2c4f870365E785982E1f101E93b906
# Private Key: 0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6

mkdir -p ./delegator-keystore
cargo tangle key import \
  --key-type ecdsa \
  --secret 7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6 \
  --keystore-path ./delegator-keystore \
  --protocol tangle-evm

# Verify account has ETH (should have 10000 ETH on Anvil)
cast balance 0x90F79bf6EB2c4f870365E785982E1f101E93b906 --rpc-url http://127.0.0.1:8545
```

### Step 0.3: Verify Operator is Registered

Delegators need at least one registered operator to delegate to:

```bash
# Check Operator 1 is registered
cast call $RESTAKING "isOperator(address)(bool)" 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 --rpc-url http://127.0.0.1:8545
# Expected: true
```

---

## Phase 1: Query Commands (Read-Only)

### Test 1.1: Show Positions (Empty State)

**Goal:** Query delegator positions before any deposits

```bash
cargo tangle delegator positions \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Delegator: 0x90f79bf6eb2c4f870365e785982e1f101e93b906
# Token: 0x0000000000000000000000000000000000000000
# Deposit: amount=0 delegated=0
# Locks: none
# Delegations: none
# Pending Unstakes: none
# Pending Withdrawals: none
```

### Test 1.2: Show Positions with JSON Output

**Goal:** Verify JSON output format for positions

```bash
cargo tangle delegator positions \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --json | jq '.'

# Expected JSON structure:
# {
#   "delegator": "0x90f79bf6eb2c4f870365e785982e1f101e93b906",
#   "token": "0x0000000000000000000000000000000000000000",
#   "deposit": {
#     "amount": "0",
#     "delegated_amount": "0"
#   },
#   "locks": [],
#   "delegations": [],
#   "pending_unstakes": [],
#   "pending_withdrawals": []
# }
```

### Test 1.3: Show Positions for Different Address

**Goal:** Query positions for a specific delegator address

```bash
cargo tangle delegator positions \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --delegator 0x90F79bf6EB2c4f870365E785982E1f101E93b906

# Expected: Positions for the specified delegator address
```

### Test 1.4: List Delegations (Empty)

**Goal:** Query delegations before any are made

```bash
cargo tangle delegator delegations \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Delegator: 0x90f79bf6eb2c4f870365e785982e1f101e93b906
# No delegations found
```

### Test 1.5: List Delegations with JSON Output

**Goal:** Verify JSON output for empty delegations

```bash
cargo tangle delegator delegations \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --json | jq '.'

# Expected:
# {
#   "delegator": "0x90f79bf6eb2c4f870365e785982e1f101e93b906",
#   "delegations": []
# }
```

### Test 1.6: List Pending Unstakes (Empty)

**Goal:** Query pending unstakes before any exist

```bash
cargo tangle delegator pending-unstakes \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Delegator: 0x90f79bf6eb2c4f870365e785982e1f101e93b906
# No pending unstakes
```

### Test 1.7: List Pending Withdrawals (Empty)

**Goal:** Query pending withdrawals before any exist

```bash
cargo tangle delegator pending-withdrawals \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Delegator: 0x90f79bf6eb2c4f870365e785982e1f101e93b906
# No pending withdrawals
```

---

## Phase 2: ERC20 Token Operations

### Test 2.1: Check ERC20 Balance

**Goal:** Query ERC20 token balance for an address

> **Note:** This test requires an ERC20 token to be deployed. Use MockERC20 if available, or skip if no token is deployed.

```bash
# Using a deployed ERC20 token (replace with actual token address)
export ERC20_TOKEN=0x8f86403A4DE0BB5791fa46B8e795C547942fe4Cf

cargo tangle delegator balance \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --token $ERC20_TOKEN

# Expected output:
# Owner: 0x90f79bf6eb2c4f870365e785982e1f101e93b906
# Token: 0x8f86403a4de0bb5791fa46b8e795c547942fe4cf
# Balance: <amount>
```

### Test 2.2: Check ERC20 Balance with JSON Output

**Goal:** Verify JSON output format for balance query

```bash
cargo tangle delegator balance \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --token $ERC20_TOKEN \
  --json | jq '.'

# Expected JSON:
# {
#   "owner": "0x90f79bf6eb2c4f870365e785982e1f101e93b906",
#   "token": "0x8f86403a4de0bb5791fa46b8e795c547942fe4cf",
#   "balance": "..."
# }
```

### Test 2.3: Check ERC20 Balance for Different Owner

**Goal:** Query balance for a specific owner address

```bash
cargo tangle delegator balance \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --token $ERC20_TOKEN \
  --owner 0x90F79bf6EB2c4f870365E785982E1f101E93b906

# Expected: Balance for the specified owner address
```

### Test 2.4: Check ERC20 Allowance

**Goal:** Query ERC20 allowance for restaking contract

```bash
cargo tangle delegator allowance \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --token $ERC20_TOKEN

# Expected output (spender defaults to restaking contract):
# Owner: 0x90f79bf6eb2c4f870365e785982e1f101e93b906
# Spender: 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512
# Token: 0x8f86403a4de0bb5791fa46b8e795c547942fe4cf
# Allowance: 0
```

### Test 2.5: Check ERC20 Allowance with Custom Spender

**Goal:** Query allowance for a specific spender

```bash
cargo tangle delegator allowance \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --token $ERC20_TOKEN \
  --spender 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9

# Expected: Allowance for the specified spender
```

### Test 2.6: Approve ERC20 Tokens

**Goal:** Approve tokens for restaking contract to spend

```bash
cargo tangle delegator approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --token $ERC20_TOKEN \
  --amount 1000000000000000000

# Expected output:
# Delegator approve: submitted tx_hash=0x...
# Delegator approve: confirmed block=Some(N) gas_used=...
```

### Test 2.7: Approve ERC20 Tokens with JSON Output

**Goal:** Verify JSON output for approval transaction

```bash
cargo tangle delegator approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --token $ERC20_TOKEN \
  --amount 500000000000000000 \
  --json

# Expected JSON with tx events:
# {"event":"tx_submitted","action":"Delegator approve","tx_hash":"0x..."}
# {"event":"tx_confirmed","action":"Delegator approve",...}
```

### Test 2.8: Verify Allowance After Approval

**Goal:** Confirm allowance was updated

```bash
cargo tangle delegator allowance \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --token $ERC20_TOKEN

# Expected: Allowance should be 1500000000000000000 (sum of both approvals)
```

---

## Phase 3: Deposit Operations

### Test 3.1: Deposit Native ETH

**Goal:** Deposit native tokens without delegating

```bash
cargo tangle delegator deposit \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 1000000000000000000

# Expected output:
# Delegator deposit: submitted tx_hash=0x...
# Delegator deposit: confirmed block=Some(N) gas_used=...
```

### Test 3.2: Verify Deposit in Positions

**Goal:** Confirm deposit reflected in positions

```bash
cargo tangle delegator positions \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected:
# Deposit: amount=1000000000000000000 delegated=0
```

### Test 3.3: Deposit with JSON Output

**Goal:** Verify JSON output for deposit transaction

```bash
cargo tangle delegator deposit \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 500000000000000000 \
  --json

# Expected JSON with tx events
```

### Test 3.4: Deposit ERC20 Tokens

**Goal:** Deposit ERC20 tokens (requires prior approval)

> **Prerequisite:** Complete Test 2.6 or 2.7 to have allowance

```bash
cargo tangle delegator deposit \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --token $ERC20_TOKEN \
  --amount 500000000000000000

# Expected: Transaction confirmed (if delegator has ERC20 tokens and allowance)
# May fail with insufficient balance or allowance if tokens not available
```

---

## Phase 4: Delegation Operations

### Test 4.1: Delegate Native Tokens (Direct)

**Goal:** Delegate tokens in one transaction (deposit + delegate)

```bash
cargo tangle delegator delegate \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --amount 500000000000000000

# Expected output:
# Delegator delegate: submitted tx_hash=0x...
# Delegator delegate: confirmed block=Some(N) gas_used=...
```

### Test 4.2: Verify Delegation Created

**Goal:** Confirm delegation appears in listings

```bash
cargo tangle delegator delegations \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Delegator: 0x90f79bf6eb2c4f870365e785982e1f101e93b906
# Delegation #0
#   Operator: 0x70997970c51812dc3a010c7d01b50e0d17dc79c8
#   Shares: <value>
#   Asset: native (0x0000000000000000000000000000000000000000)
#   Selection: All
```

### Test 4.3: Delegate from Existing Deposit

**Goal:** Delegate using already-deposited funds

```bash
# First verify we have deposit balance
cargo tangle delegator positions \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Delegate from deposit (using --from-deposit flag)
cargo tangle delegator delegate \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --amount 200000000000000000 \
  --from-deposit

# Expected: Delegation created using existing deposit balance
```

### Test 4.4: Delegate with Fixed Selection Mode

**Goal:** Delegate with specific blueprint IDs

```bash
cargo tangle delegator delegate \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --amount 100000000000000000 \
  --selection fixed \
  --blueprint-id 0

# Expected: Delegation created with Fixed selection mode
```

### Test 4.5: Verify Fixed Selection in Delegations

**Goal:** Confirm selection mode appears in delegation listing

```bash
cargo tangle delegator delegations \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --json | jq '.delegations[-1]'

# Expected JSON shows selection_mode: "Fixed" and blueprint_ids: [0]
```

### Test 4.6: Delegate with JSON Output

**Goal:** Verify JSON output for delegation

```bash
cargo tangle delegator delegate \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --amount 100000000000000000 \
  --json

# Expected JSON with tx events
```

---

## Phase 5: Undelegation Operations

### Test 5.1: Schedule Undelegation (Undelegate)

**Goal:** Schedule an unstake operation

```bash
cargo tangle delegator undelegate \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --amount 100000000000000000

# Expected output:
# Delegator undelegate: submitted tx_hash=0x...
# Delegator undelegate: confirmed block=Some(N) gas_used=...
```

### Test 5.2: Verify Pending Unstake Created

**Goal:** Confirm unstake appears in pending list

```bash
cargo tangle delegator pending-unstakes \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Delegator: 0x90f79bf6eb2c4f870365e785982e1f101e93b906
# Pending Unstake #0
#   Operator: 0x70997970c51812dc3a010c7d01b50e0d17dc79c8
#   Shares: <value>
#   Asset: native (0x0000000000000000000000000000000000000000)
#   Selection: All
#   Requested Round: <round>
```

### Test 5.3: Undelegate with JSON Output

**Goal:** Verify JSON output for undelegation

```bash
cargo tangle delegator undelegate \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --amount 50000000000000000 \
  --json

# Expected JSON with tx events
```

### Test 5.4: Execute Unstake (Before Delay)

**Goal:** Verify execute fails before delay period

```bash
cargo tangle delegator execute-unstake \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: May be no-op (no matured unstakes) or error about delay
```

### Test 5.5: Execute Unstake (After Delay)

**Goal:** Execute unstake after delay period

```bash
# First, advance time and rounds
# (Use helper script or manual cast calls to advance)
cast rpc evm_increaseTime 604800 --rpc-url http://127.0.0.1:8545
cast rpc evm_mine --rpc-url http://127.0.0.1:8545

# Advance rounds if needed (depends on delegationBondLessDelay)
# ... (see advance_rounds helper from operator tests)

cargo tangle delegator execute-unstake \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Unstakes executed, funds returned to deposit balance
```

### Test 5.6: Execute Unstake with JSON Output

**Goal:** Verify JSON output for execute-unstake

```bash
cargo tangle delegator execute-unstake \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --json

# Expected JSON with tx events
```

---

## Phase 6: Execute Unstake and Withdraw Combined

### Test 6.1: Execute Unstake and Withdraw

**Goal:** Execute a specific unstake and withdraw in one operation

> **Prerequisite:** Must have a pending unstake that has matured

```bash
# First, check pending unstakes to get parameters
cargo tangle delegator pending-unstakes \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --json

# Execute unstake and withdraw (adjust parameters based on pending unstake)
cargo tangle delegator execute-unstake-withdraw \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --shares 50000000000000000 \
  --requested-round <ROUND_FROM_PENDING>

# Expected: Transaction confirmed, funds withdrawn to delegator
```

### Test 6.2: Execute Unstake and Withdraw with Custom Receiver

**Goal:** Withdraw to a different address

```bash
cargo tangle delegator execute-unstake-withdraw \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --shares 25000000000000000 \
  --requested-round <ROUND> \
  --receiver 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC

# Expected: Funds sent to receiver address
```

### Test 6.3: Execute Unstake and Withdraw with JSON Output

**Goal:** Verify JSON output

```bash
cargo tangle delegator execute-unstake-withdraw \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --shares <SHARES> \
  --requested-round <ROUND> \
  --json

# Expected JSON with tx events
```

---

## Phase 7: Withdrawal Operations

### Test 7.1: Schedule Withdrawal

**Goal:** Schedule a withdrawal of deposited funds

> **Prerequisite:** Must have undelegated funds in deposit balance

```bash
cargo tangle delegator schedule-withdraw \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 100000000000000000

# Expected output:
# Delegator schedule-withdraw: submitted tx_hash=0x...
# Delegator schedule-withdraw: confirmed block=Some(N) gas_used=...
```

### Test 7.2: Verify Pending Withdrawal Created

**Goal:** Confirm withdrawal appears in pending list

```bash
cargo tangle delegator pending-withdrawals \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected output:
# Delegator: 0x90f79bf6eb2c4f870365e785982e1f101e93b906
# Pending Withdrawal #0
#   Asset: native (0x0000000000000000000000000000000000000000)
#   Amount: 100000000000000000
#   Requested Round: <round>
```

### Test 7.3: Schedule Withdrawal with JSON Output

**Goal:** Verify JSON output

```bash
cargo tangle delegator schedule-withdraw \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 50000000000000000 \
  --json

# Expected JSON with tx events
```

### Test 7.4: Execute Withdrawal (Before Delay)

**Goal:** Verify execute fails before delay period

```bash
cargo tangle delegator execute-withdraw \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: May be no-op or error about delay not passed
```

### Test 7.5: Execute Withdrawal (After Delay)

**Goal:** Execute withdrawal after delay period

```bash
# Advance time and rounds
cast rpc evm_increaseTime 604800 --rpc-url http://127.0.0.1:8545
cast rpc evm_mine --rpc-url http://127.0.0.1:8545
# ... advance rounds as needed

cargo tangle delegator execute-withdraw \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING

# Expected: Withdrawals executed, funds returned to delegator wallet
```

### Test 7.6: Execute Withdrawal with JSON Output

**Goal:** Verify JSON output

```bash
cargo tangle delegator execute-withdraw \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --json

# Expected JSON with tx events
```

---

## Phase 8: Error Handling and Edge Cases

### Test 8.1: Deposit with Zero Amount (Should Fail)

**Goal:** Verify error when depositing zero

```bash
cargo tangle delegator deposit \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 0

# Expected: Error - zero amount or contract revert
```

### Test 8.2: Delegate to Non-Operator (Should Fail)

**Goal:** Verify error when delegating to unregistered operator

```bash
cargo tangle delegator delegate \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x0000000000000000000000000000000000000001 \
  --amount 100000000000000000

# Expected: Contract revert - operator not registered
```

### Test 8.3: Delegate More Than Deposit (from-deposit)

**Goal:** Verify error when delegating more than available

```bash
cargo tangle delegator delegate \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --amount 999999999999999999999999999999 \
  --from-deposit

# Expected: Contract revert - insufficient balance
```

### Test 8.4: Undelegate More Than Delegated (Should Fail)

**Goal:** Verify error when undelegating more than delegated

```bash
cargo tangle delegator undelegate \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --amount 999999999999999999999999999999

# Expected: Contract revert - insufficient delegation
```

### Test 8.5: Schedule Withdrawal More Than Available (Should Fail)

**Goal:** Verify error when scheduling withdrawal exceeding balance

```bash
cargo tangle delegator schedule-withdraw \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --amount 999999999999999999999999999999

# Expected: Contract revert - insufficient available balance
```

### Test 8.6: Balance for Invalid Token Address

**Goal:** Test behavior with invalid token address

```bash
cargo tangle delegator balance \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./delegator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --token 0x0000000000000000000000000000000000000001

# Expected: Error - contract call fails or returns 0
```

---

## Cleanup

```bash
# Stop all processes
pkill -f "anvil"
pkill -f "python.*http.server"

# Remove test keystores
rm -rf ./delegator-keystore

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
| 3 | `0x90F79bf6EB2c4f870365E785982E1f101E93b906` | `0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6` | Delegator / Operator 2 |

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
| `delegator positions` | Show all positions | `--delegator`, `--token`, `--json` |
| `delegator delegations` | List active delegations | `--delegator`, `--json` |
| `delegator pending-unstakes` | List pending unstakes | `--delegator`, `--json` |
| `delegator pending-withdrawals` | List pending withdrawals | `--delegator`, `--json` |
| `delegator balance` | Check ERC20 balance | `--token`, `--owner`, `--json` |
| `delegator allowance` | Check ERC20 allowance | `--token`, `--owner`, `--spender`, `--json` |
| `delegator approve` | Approve ERC20 spending | `--token`, `--amount`, `--spender`, `--json` |
| `delegator deposit` | Deposit tokens | `--token`, `--amount`, `--json` |
| `delegator delegate` | Delegate to operator | `--operator`, `--amount`, `--token`, `--selection`, `--blueprint-id`, `--from-deposit`, `--json` |
| `delegator undelegate` | Schedule unstake | `--operator`, `--amount`, `--token`, `--json` |
| `delegator execute-unstake` | Execute matured unstakes | `--json` |
| `delegator execute-unstake-withdraw` | Execute unstake + withdraw | `--operator`, `--token`, `--shares`, `--requested-round`, `--receiver`, `--json` |
| `delegator schedule-withdraw` | Schedule withdrawal | `--token`, `--amount`, `--json` |
| `delegator execute-withdraw` | Execute matured withdrawals | `--json` |

---

## Quick Reference: Common Flags

All commands support these flags:
- `--http-rpc-url` - HTTP RPC endpoint (e.g., `http://127.0.0.1:8545`)
- `--ws-rpc-url` - WebSocket RPC endpoint (e.g., `ws://127.0.0.1:8546`)
- `--keystore-path` - Path to keystore directory
- `--tangle-contract` - Tangle contract address
- `--restaking-contract` - Restaking contract address
- `--json` - Output in JSON format

Default token address is `0x0000000000000000000000000000000000000000` (native ETH).
