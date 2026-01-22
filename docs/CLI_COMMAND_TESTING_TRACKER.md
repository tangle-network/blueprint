# CLI Command Testing Tracker

This document tracks the testing status of all `cargo-tangle` CLI commands. Use this to ensure comprehensive test coverage during QA.

**Legend:**
- ✅ Tested - Command verified working in E2E tests
- ⚠️ Partial - Some options/flags tested, others not
- ❌ Not Tested - Command not yet verified
- 🔇 Ignored - Intentionally skipped for now (low priority or out of scope)
- 🚫 N/A - Not applicable for local testing (e.g., mainnet-only features)

**Last Updated:** 2026-01-21 (Job System commands completed - 30/30 tests passed)

---

## Summary

| Category | Total | Tested | Partial | Not Tested | Ignored |
|----------|-------|--------|---------|------------|---------|
| Blueprint Lifecycle | 5 | 4 | 0 | 1 | 0 |
| Key Management | 5 | 1 | 0 | 0 | 4 |
| Service Lifecycle | 9 | 9 | 0 | 0 | 0 |
| Job System | 4 | 4 | 0 | 0 | 0 |
| Operator Utilities | 13 | 0 | 0 | 13 | 0 |
| Delegator Utilities | 14 | 0 | 0 | 14 | 0 |
| Chain State Queries | 3 | 0 | 0 | 3 | 0 |
| Cloud Deployment | 7 | 0 | 0 | 7 | 0 |
| EigenLayer AVS | 7 | 0 | 0 | 7 | 0 |
| Debug | 1 | 0 | 0 | 1 | 0 |
| **Total** | **68** | **18** | **0** | **46** | **4** |

---

## 1. Blueprint Lifecycle (`blueprint` / `bp`)

| Command | Description | Status | Tested Options | Notes | Reference |
|---------|-------------|--------|----------------|-------|-----------|
| `blueprint create` | Scaffold a new blueprint project from a template | ✅ | `--name`, `--skip-prompts`, `--project-description`, `--project-authors` | | E2E Step 1 |
| `blueprint deploy tangle` | Deploy a blueprint definition to Tangle network | ✅ | `--network testnet`, `--definition`, `--settings-file` | | E2E Step 7 |
| `blueprint deploy eigenlayer` | Deploy a blueprint to EigenLayer AVS | ❌ | | | |
| `blueprint run` | Start the operator manager to run a blueprint | ✅ | `-p tangle-evm`, `-k`, `-f`, `-d` | | E2E Step 11 |
| `blueprint preregister` | Generate registration data without submitting on-chain | ❌ | | | |
| `blueprint register` | Register an operator for a specific blueprint | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--status-registry-contract`, `--blueprint-id`, `--rpc-endpoint` | | E2E Step 8 |

### Tested Command Examples

```bash
# Create blueprint (Step 1)
cargo tangle blueprint create \
  --name hello-blueprint \
  --skip-prompts \
  --project-description "Hello Blueprint for local E2E testing" \
  --project-authors "Tangle"

# Deploy blueprint (Step 7)
cargo tangle blueprint deploy tangle \
  --network testnet \
  --definition ./dist/definition.json \
  --settings-file ./settings.env

# Register operator (Step 8)
cargo tangle blueprint register \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --rpc-endpoint "http://localhost:9000"

# Run operator (Step 11)
cargo tangle blueprint run \
  -p tangle-evm \
  -k ./operator-keystore \
  -f ./settings.env \
  -d ./data
```

---

## 2. Key Management (`key` / `k`)

| Command | Description | Status | Tested Options | Notes | Reference |
|---------|-------------|--------|----------------|-------|-----------|
| `key generate` | Create a new cryptographic key pair | 🔇 | | Low priority for E2E | |
| `key import` | Import an existing private key into the keystore | ✅ | `--key-type ecdsa`, `--secret`, `--keystore-path`, `--protocol tangle-evm` | | E2E Steps 5, 9 |
| `key export` | Export a key from the keystore | 🔇 | | Low priority for E2E | |
| `key list` | Show all keys stored in the keystore | 🔇 | | Low priority for E2E | |
| `key generate-mnemonic` | Generate a BIP39 mnemonic seed phrase | 🔇 | | Low priority for E2E | |

### Tested Command Examples

```bash
# Import operator key (Step 5)
cargo tangle key import \
  --key-type ecdsa \
  --secret 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \
  --keystore-path ./operator-keystore \
  --protocol tangle-evm

# Import user key (Step 9)
cargo tangle key import \
  --key-type ecdsa \
  --secret 5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a \
  --keystore-path ./user-keystore \
  --protocol tangle-evm
```

---

## 3. Service Lifecycle (`service` / `svc`)

| Command | Description | Status | Tested Options | Notes | Reference |
|---------|-------------|--------|----------------|-------|-----------|
| `service request` | Request a new service instance from operators | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--status-registry-contract`, `--blueprint-id`, `--operator`, `--ttl`, `--operator-exposure-bps`, `--permitted-caller`, `--config-hex`, `--payment-token`, `--payment-amount`, `--json`, `--security-requirement` | 7/7 tests passed including multi-operator, exposure, payments, security requirements | Service Lifecycle Tests Phase 1-2, 10 |
| `service approve` | Approve a pending service request as an operator | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--status-registry-contract`, `--request-id`, `--restaking-percent`, `--json`, `--security-commitment` | 3/3 tests passed including custom restaking and security commitments | Service Lifecycle Tests Phase 3, 10 |
| `service reject` | Reject a pending service request as an operator | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--status-registry-contract`, `--request-id`, `--json` | 3/3 tests passed; correctly prevents approval of rejected requests | Service Lifecycle Tests Phase 4 |
| `service join` | Join an existing dynamic service as an operator | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--status-registry-contract`, `--service-id`, `--exposure-bps` | 5/5 tests passed; validates exposure 0 < bps <= 10000 | Service Lifecycle Tests Phase 5 |
| `service leave` | Leave a dynamic service as an operator | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--status-registry-contract`, `--service-id` | 3/3 tests passed via cast; ⚠️ CLI needs exit queue commands (Feature Request #1) | Service Lifecycle Tests Phase 6 |
| `service spawn` | Manually spawn a service runtime process | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--status-registry-contract`, `--blueprint-id`, `--service-id`, `--data-dir`, `--spawn-method`, `--no-vm`, `--dry-run` | 4/4 tests passed including native method, no-vm, dry-run | Service Lifecycle Tests Phase 8 |
| `service list` | List all active services on the network | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--json` | 2/2 tests passed; preferred over `list services` (has `--json`) | Service Lifecycle Tests Phase 7 |
| `service list-requests` | List all pending service requests | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--json` | 2/2 tests passed; preferred over `list requests` (has `--json`) | Service Lifecycle Tests Phase 7 |
| `service show-request` | Display details of a specific service request | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--request-id` | 1/1 tests passed | Service Lifecycle Tests Phase 7 |

### Tested Command Examples

```bash
# Request service (basic)
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

# Request service with security requirements
cargo tangle blueprint service request \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --operator 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 \
  --security-requirement native:_:100:500 \
  --ttl 3600

# Approve service with security commitment
cargo tangle blueprint service approve \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id 0 \
  --security-commitment native:_:250

# Reject service request
cargo tangle blueprint service reject \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --request-id 0

# Join dynamic service with custom exposure
cargo tangle blueprint service join \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --service-id 0 \
  --exposure-bps 5000

# Spawn service with native method
cargo tangle blueprint service spawn \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 0 \
  --spawn-method native \
  --data-dir ./data

# List services with JSON output
cargo tangle blueprint service list \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --json

# Show specific request details
cargo tangle blueprint service show \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --request-id 0
```

### Known Limitations

**Feature Request #1: Exit Queue Commands**
- The `service leave` command calls `leaveService()` which only works when `exitQueueDuration == 0`
- Default contract config has 7-day exit queue, requiring:
  1. `scheduleExit(serviceId)` - after 1-day min commitment
  2. Wait 7 days
  3. `executeExit(serviceId)`
- Workaround: Use `cast` to call contract functions directly
- Proposed CLI additions: `service schedule-exit`, `service execute-exit`, `service cancel-exit`

---

## 4. Job System (`jobs` / `j`)

**Test Documentation:**
- Test Plan: [`JOB_SYSTEM_TEST_PLAN.md`](./JOB_SYSTEM_TEST_PLAN.md)
- Progress Tracker: [`JOB_SYSTEM_TEST_PROGRESS.md`](./JOB_SYSTEM_TEST_PROGRESS.md)

| Command | Description | Status | Tested Options | Notes | Reference |
|---------|-------------|--------|----------------|-------|-----------|
| `jobs list` | List all available jobs defined in a blueprint | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--blueprint-id`, `--json` | 4/4 tests passed; human-readable and JSON output verified | JOB_SYSTEM_TEST_PROGRESS.md Phase 1 |
| `jobs show` | Show detailed metadata for a specific job call | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--blueprint-id`, `--service-id`, `--call-id`, `--json` | 4/4 tests passed; before/after completion, JSON output verified | JOB_SYSTEM_TEST_PROGRESS.md Phase 5 |
| `jobs submit` | Submit a job invocation to a running service | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--status-registry-contract`, `--blueprint-id`, `--service-id`, `--job`, `--payload-hex`, `--payload-file`, `--params-file`, `--prompt`, `--watch`, `--json`, `--timeout-secs` | 9/9 tests passed; all input methods verified including TLV v2 object format, complex types (int, address, array, tuple) | JOB_SYSTEM_TEST_PROGRESS.md Phase 2-3, 7 |
| `jobs watch` | Wait for a job result and display the output | ✅ | `--http-rpc-url`, `--ws-rpc-url`, `--keystore-path`, `--tangle-contract`, `--restaking-contract`, `--blueprint-id`, `--service-id`, `--call-id`, `--timeout-secs` | 3/3 tests passed; timeout behavior, non-existent call handling, custom timeout verified | JOB_SYSTEM_TEST_PROGRESS.md Phase 4 |

### Tested Command Examples

```bash
# Encode payload
PAYLOAD=$(cast abi-encode "f((string))" "(Alice)")

# Submit job (Step 13)
cargo tangle blueprint jobs submit \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 0 \
  --job 0 \
  --payload-hex $PAYLOAD

# Watch for result (Step 14)
cargo tangle blueprint jobs watch \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 0 \
  --call-id 0 \
  --timeout-secs 120
```

---

## 5. Operator Utilities (`operator` / `op`)

**Test Documentation:**
- Test Plan: [`OPERATOR_UTILITIES_TEST_PLAN.md`](./OPERATOR_UTILITIES_TEST_PLAN.md)
- Progress Tracker: [`OPERATOR_UTILITIES_TEST_PROGRESS.md`](./OPERATOR_UTILITIES_TEST_PROGRESS.md)

| Command | Description | Status | Tested Options | Notes | Reference |
|---------|-------------|--------|----------------|-------|-----------|
| `operator register` | Register as an operator on the restaking layer | ❌ | | | |
| `operator show-status` | Display operator heartbeat and status info | ❌ | | | |
| `operator submit-heartbeat` | Send a heartbeat to prove operator liveness | ❌ | | | |
| `operator show-restaking` | Show current restaking status and stake amounts | ❌ | | | |
| `operator join-service` | Join an existing dynamic service | ❌ | | ⚠️ Duplicate of `service join` (no validation, requires unused `--blueprint-id`) | |
| `operator leave-service` | Leave a dynamic service | ❌ | | ⚠️ Duplicate of `service leave` (no validation, requires unused `--blueprint-id`) | |
| `operator list-delegators` | List all delegators staking with this operator | ❌ | | | |
| `operator unstake` | Schedule an unstake operation | ❌ | | | |
| `operator execute-unstake` | Execute a matured unstake operation | ❌ | | | |
| `operator increase-stake` | Add more stake to operator balance | ❌ | | | |
| `operator schedule-leaving` | Schedule operator departure from network | ❌ | | | |
| `operator complete-leaving` | Finalize operator departure after cooldown | ❌ | | | |

**Note:** Operator registration in E2E tests was done via `cast send` directly to the contract, not via CLI.

---

## 6. Delegator Utilities (`delegator` / `del`)

| Command | Description | Status | Tested Options | Notes | Reference |
|---------|-------------|--------|----------------|-------|-----------|
| `delegator show` | Display deposits, locks, and delegation summary | ❌ | | | |
| `delegator balance` | Check ERC20 token balance for an address | ❌ | | | |
| `delegator allowance` | Check ERC20 allowance for restaking contract | ❌ | | | |
| `delegator approve` | Approve tokens for use by restaking contract | ❌ | | | |
| `delegator deposit` | Deposit tokens without delegating to an operator | ❌ | | | |
| `delegator delegate` | Delegate staked tokens to an operator | ❌ | | | |
| `delegator undelegate` | Schedule removal of delegation from an operator | ❌ | | | |
| `delegator execute-unstakes` | Execute all matured unstake operations | ❌ | | | |
| `delegator execute-unstake-and-withdraw` | Execute a specific unstake and withdraw funds | ❌ | | | |
| `delegator schedule-withdrawal` | Schedule a withdrawal of deposited funds | ❌ | | | |
| `delegator execute-withdrawals` | Execute all matured withdrawal operations | ❌ | | | |
| `delegator list-delegations` | List all active delegations | ❌ | | | |
| `delegator list-pending-unstakes` | List all pending unstake operations | ❌ | | | |
| `delegator list-pending-withdrawals` | List all pending withdrawal operations | ❌ | | | |

---

## 7. Chain State Queries (`list` / `l`)

| Command | Description | Status | Tested Options | Notes | Reference |
|---------|-------------|--------|----------------|-------|-----------|
| `list blueprints` | Show all blueprints registered on-chain | ❌ | | | |
| `list requests` | Show all pending service requests | ❌ | | ⚠️ Duplicate of `service list-requests` (no `--json` support) | |
| `list services` | Show all active services on the network | ❌ | | ⚠️ Duplicate of `service list` (no `--json` support) | |

---

## 8. Cloud Deployment (`cloud` / `c`)

**Feature Flag:** `remote-providers`

| Command | Description | Status | Tested Options | Notes | Reference |
|---------|-------------|--------|----------------|-------|-----------|
| `cloud configure` | Set up credentials for a cloud provider | ❌ | | | |
| `cloud configure-policy` | Configure deployment policies and preferences | ❌ | | | |
| `cloud show-policy` | Display the current deployment policy | ❌ | | | |
| `cloud estimate` | Estimate costs for a deployment across providers | ❌ | | | |
| `cloud logs` | View logs from a cloud deployment | ❌ | | | |
| `cloud status` | Check the status of a cloud deployment | ❌ | | | |
| `cloud update` | Update an existing cloud deployment | ❌ | | | |

---

## 9. EigenLayer AVS Management (`eigenlayer`)

| Command | Description | Status | Tested Options | Notes | Reference |
|---------|-------------|--------|----------------|-------|-----------|
| `eigenlayer register` | Register as an operator with an EigenLayer AVS | ❌ | | | |
| `eigenlayer deregister` | Deregister from an EigenLayer AVS | ❌ | | | |
| `eigenlayer list` | List all AVS registrations for this operator | ❌ | | | |
| `eigenlayer sync` | Sync local state with on-chain AVS registrations | ❌ | | | |
| `eigenlayer rewards show` | Display earned rewards information | ❌ | | | |
| `eigenlayer rewards claim` | Claim earned rewards from AVS participation | ❌ | | | |
| `eigenlayer rewards set-claimer` | Set the address that can claim rewards | ❌ | | | |

---

## 10. Debug (`debug`)

| Command | Description | Status | Tested Options | Notes | Reference |
|---------|-------------|--------|----------------|-------|-----------|
| `debug spawn` | Launch a local Anvil testnet and run blueprint | ❌ | | | |

---

## Duplicate Commands

The following commands have overlapping functionality. When testing, prefer the "Recommended" version.

| Command A | Command B | Same Code? | Difference | Recommended | Status |
|-----------|-----------|------------|------------|-------------|--------|
| `service join` | `operator join-service` | Yes | `service join` validates exposure > 0 and <= MAX_BPS | `service join` | ✅ Tested |
| `service leave` | `operator leave-service` | Yes | `service leave` validates operator is active first | `service leave` | ✅ Tested |
| `service list` | `list services` | Yes | `service list` supports `--json` flag | `service list` | ✅ Tested |
| `service list-requests` | `list requests` | Yes | `service list-requests` supports `--json` flag | `service list-requests` | ✅ Tested |

**Testing Strategy:** Test only the recommended version. If it passes, the duplicate will also work since they share the same underlying implementation.

---

## Testing Priorities

### High Priority (Core Workflow)
These are essential for the basic operator/user workflow:

1. ❌ `list blueprints` - Users need to discover blueprints
2. ✅ `service list` - Users need to see active services (preferred over `list services`) - **DONE**
3. ❌ `operator register` - Operators need CLI-based registration

### Medium Priority (Extended Workflow)
1. ✅ `service reject` - Operators may need to reject requests - **DONE**
2. ✅ `service list-requests` - List pending requests (preferred over `list requests`) - **DONE**
3. ✅ `jobs list` - Discover available jobs - **DONE**
4. ✅ `jobs show` - Understand job parameters - **DONE**

### Lower Priority (Advanced Features)
1. ❌ All delegator commands
2. ❌ All cloud commands
3. ❌ All EigenLayer commands
4. ❌ Debug commands

### Ignored (Out of Scope)
1. 🔇 Key management commands (except `import`) - Low priority for E2E testing

### Completed
1. ✅ All service lifecycle commands (9/9) - See SERVICE_LIFECYCLE_TEST_PROGRESS.md
2. ✅ All job system commands (4/4) - See JOB_SYSTEM_TEST_PROGRESS.md (30/30 tests passed)

---

## How to Update This Document

When testing a command:

1. Change status from ❌ to ✅
2. Add tested options to the "Tested Options" column
3. Add any relevant notes
4. Add reference to test session/step
5. Update the summary counts at the top

Example update:
```markdown
| `key generate` | Create a new cryptographic key pair | ✅ | `--key-type ecdsa`, `--keystore-path` | Works correctly | Session 4, Step X |
```
