# Job System Commands Test Plan

This document provides a comprehensive test plan for all job system commands in `cargo-tangle`. It builds upon the test environment established in the Service Lifecycle tests and covers job discovery, submission, inspection, and result watching.

**Target Commands:**
1. `jobs list` - List all job definitions for a blueprint
2. `jobs show` - Show metadata for a specific job call
3. `jobs submit` - Submit a job invocation to a service
4. `jobs watch` - Wait for a job result using a call identifier

**Last Updated:** 2026-01-20

---

## Prerequisites

### Required Components
1. **Service Lifecycle Test Environment** - Complete Phase 0 from `SERVICE_LIFECYCLE_TEST_PLAN.md` first
2. **Active Service** - At least one service must be created and have an operator running
3. **Blueprint with Jobs** - The deployed blueprint must define at least one job

### macOS C++ Build Fix

If you encounter RocksDB C++ compilation errors like `fatal error: 'memory' file not found`, run this before cargo commands:

```bash
export SDKROOT=$(xcrun --show-sdk-path) && export CXXFLAGS="-isysroot $SDKROOT -I$SDKROOT/usr/include/c++/v1 -stdlib=libc++"
```

### Verify Prerequisites
```bash
# Check blueprint has jobs defined
cargo tangle blueprint jobs list \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 0

# Check service is active
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
    ├── operator-keystore/  # Operator keys
    ├── user-keystore/      # User keys (for job submission)
    ├── settings.env        # Environment configuration
    └── dist/               # Blueprint artifacts
```

---

## Terminal Overview

This test plan requires **4 terminals**:

| Terminal | Purpose | Steps |
|----------|---------|-------|
| Terminal 1 | Anvil (local blockchain) | From Service Lifecycle Setup |
| Terminal 2 | HTTP server (artifact hosting) | From Service Lifecycle Setup |
| Terminal 3 | CLI commands (job system tests) | All Test Sections |
| Terminal 4 | Blueprint manager (operator runtime) | Must be running for job processing |

---

## Understanding Job System Architecture

### How Jobs Work

1. **Blueprint Definition** - Jobs are defined in the blueprint's `definition.json` with:
   - Job name and description
   - Parameter schema (input types)
   - Result schema (output types)

2. **Job Router** - The blueprint binary has a router that maps job indices to handler functions:
   ```rust
   pub const CREATE_DOCUMENT_JOB: u8 = 0;

   pub fn router() -> Router {
       Router::new().route(CREATE_DOCUMENT_JOB, create_document)
   }
   ```

3. **Job Submission Flow**:
   ```
   User → submitJob(serviceId, jobIndex, inputs) → Contract emits JobSubmitted event
                                                          ↓
   Operator ← picks up job ← Router dispatches to handler
                                                          ↓
   Operator → submitResult(serviceId, callId, result) → Contract emits JobResultSubmitted event
                                                          ↓
                                                   User receives result
   ```

### Default Blueprint Job Schema

The test blueprint (`svc-test-blueprint`) created by `cargo tangle blueprint create` has one job:

**Job 0: "hello"**
- **Description:** Greets the caller with a personalized message
- **Input Schema:** `(string)` - The name to greet
- **Output Schema:** `(string)` - The greeting message

Example:
- Input: `("Alice")`
- Output: `("Hello, Alice!")`

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

### Step 0.2: Create a Service for Job Testing (if needed)

If no service exists from previous testing:

```bash
# Request a service
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

# Approve it (as operator)
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

### Step 0.3: Start Operator Runtime (Terminal 4)

The operator must be running to process job submissions:

```bash
# Terminal 4
cd /path/to/service-lifecycle-test/svc-test-blueprint

cargo tangle blueprint service spawn \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./operator-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 0 \
  --data-dir ./data-job-test

# Expected: Manager starts and runs, processing incoming jobs
```

---

## Phase 1: Jobs List Command

### Test 1.1: List Jobs with Human-Readable Output

**Goal:** Verify basic job listing functionality

```bash
cargo tangle blueprint jobs list \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 0

# Expected output:
# Jobs
# =============================================
# Job 0
#   Name: hello
#   Description: Greets the caller with a personalized message
#   Parameters:
#     - arg_0: string (or similar)
#   Results:
#     - result_0: string (or similar)
# =============================================
```

**Verification:**
- Output shows at least one job
- Job index, name, description are displayed
- Parameter and result schemas are shown

### Test 1.2: List Jobs with JSON Output

**Goal:** Verify JSON output format for automation

```bash
cargo tangle blueprint jobs list \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 0 \
  --json | jq '.'

# Expected JSON structure:
# [
#   {
#     "index": 0,
#     "name": "hello",
#     "description": "Greets the caller with a personalized message",
#     "metadata_uri": null,
#     "parameters": {
#       "defined": true,
#       "fields": ["arg_0: string"]
#     },
#     "results": {
#       "defined": true,
#       "fields": ["result_0: string"]
#     }
#   }
# ]
```

**Verification:**
- Output is valid JSON array
- Each job has: index, name, description, parameters, results fields
- Schema fields are properly formatted

### Test 1.3: List Jobs from Non-Existent Blueprint (Should Fail)

**Goal:** Verify error handling for invalid blueprint ID

```bash
cargo tangle blueprint jobs list \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 99999

# Expected: Error indicating blueprint not found
```

### Test 1.4: Verify Warning for Unverified Sources

**Goal:** Check that CLI warns when blueprint sources lack binary hashes

```bash
# This test depends on how the blueprint was deployed
# If sources lack binary hashes, stderr should contain:
# "warning: blueprint definition includes source entries without binary hashes"
```

---

## Phase 2: Jobs Submit Command - Basic Submission

### Test 2.1: Submit Job with Hex Payload

**Goal:** Submit a job using raw hex-encoded payload

```bash
# Encode the input: "Alice" as ABI tuple (string)
# Using cast to encode
PAYLOAD=$(cast abi-encode "f(string)" "Alice")

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

# Expected output:
# Submitted job 0 to service 0. Call ID: 0 (tx: 0x...)

export CALL_ID_1=0
```

**Verification:**
- Transaction submitted successfully
- Call ID is returned
- No errors in operator terminal (Terminal 4)

### Test 2.2: Submit Job with Params File (Array Format)

**Goal:** Submit a job using JSON parameters file

```bash
# Create params file
cat > /tmp/job-params-array.json << EOF
["Bob"]
EOF

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
  --params-file /tmp/job-params-array.json

# Expected: Call ID returned

export CALL_ID_2=1
```

### Test 2.3: Submit Job with Params File (Object Format)

**Goal:** Submit a job using named parameters

```bash
# Create params file with named fields
# Note: Field name depends on schema - check jobs list output first
cat > /tmp/job-params-object.json << EOF
{"arg_0": "Charlie"}
EOF

# Or if schema has named parameters:
# {"name": "Charlie"}

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
  --params-file /tmp/job-params-object.json

export CALL_ID_3=2
```

### Test 2.4: Submit Job with Payload File

**Goal:** Submit a job using raw binary file

```bash
# Create raw payload file (ABI-encoded)
cast abi-encode "f(string)" "Dave" | xxd -r -p > /tmp/job-payload.bin

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
  --payload-file /tmp/job-payload.bin

export CALL_ID_4=3
```

### Test 2.5: Submit Job with Interactive Prompt

**Goal:** Submit a job using interactive parameter input

```bash
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
  --prompt

# Interactive prompt appears:
# Enter parameter values for job `hello` (index 0). Use Solidity literal syntax for arrays/tuples.
# arg_0 [string]: Eve
#
# Expected: Job submitted with entered value

export CALL_ID_5=4
```

### Test 2.6: Submit Job with JSON Output

**Goal:** Verify JSON output format for job submission

```bash
PAYLOAD=$(cast abi-encode "f(string)" "Frank")

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
  --payload-hex $PAYLOAD \
  --json

# Expected JSON output:
# {"event":"job_submitted","service_id":0,"blueprint_id":0,"job":0,"call_id":5,"tx_hash":"0x..."}

export CALL_ID_6=5
```

---

## Phase 3: Jobs Submit with Watch

### Test 3.1: Submit Job with --watch Flag

**Goal:** Submit a job and wait for the result in one command

> **Note:** This test requires the operator runtime to be running (Terminal 4)

```bash
PAYLOAD=$(cast abi-encode "f(string)" "Grace")

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
  --payload-hex $PAYLOAD \
  --watch \
  --timeout-secs 120

# Expected output (after operator processes):
# Submitted job 0 to service 0. Call ID: X (tx: 0x...)
# Job result ready (Y bytes). Decoded output:
#   result_0 (string) = "Hello, Grace!"
```

**Verification:**
- Job submission transaction confirmed
- CLI waits for result
- Result is decoded using schema and displayed

### Test 3.2: Submit Job with --watch and JSON Output

**Goal:** Verify JSON output for submit with watch

```bash
PAYLOAD=$(cast abi-encode "f(string)" "Henry")

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
  --payload-hex $PAYLOAD \
  --watch \
  --timeout-secs 120 \
  --json

# Expected JSON output (two events):
# {"event":"job_submitted","service_id":0,"blueprint_id":0,"job":0,"call_id":X,"tx_hash":"0x..."}
# {"event":"job_result","service_id":0,"call_id":X,"decoded":["result_0 (string) = \"Hello, Henry!\""],"length":Y}
```

### Test 3.3: Submit Job with --watch Timeout (No Operator Running)

**Goal:** Verify timeout behavior when result is not submitted

> **Note:** Stop the operator runtime first (Ctrl+C in Terminal 4)

```bash
PAYLOAD=$(cast abi-encode "f(string)" "Timeout")

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
  --payload-hex $PAYLOAD \
  --watch \
  --timeout-secs 5

# Expected: Error after 5 seconds
# "timed out waiting for result for call X"

# Restart operator for subsequent tests
```

---

## Phase 4: Jobs Watch Command

### Test 4.1: Watch for Job Result (Separate Command)

**Goal:** Watch for a previously submitted job's result

> **Prerequisites:**
> 1. Submit a job without --watch first
> 2. Note the call_id
> 3. Ensure operator is running

```bash
# Submit a job first
PAYLOAD=$(cast abi-encode "f(string)" "Ivy")
SUBMIT_OUTPUT=$(cargo tangle blueprint jobs submit \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 0 \
  --job 0 \
  --payload-hex $PAYLOAD)

# Extract call ID (from output like "Call ID: X")
WATCH_CALL_ID=$(echo "$SUBMIT_OUTPUT" | grep -oP 'Call ID: \K\d+')

# Watch for result
cargo tangle blueprint jobs watch \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 0 \
  --service-id 0 \
  --call-id $WATCH_CALL_ID \
  --timeout-secs 120

# Expected: Result displayed when operator processes the job
```

### Test 4.2: Watch for Non-Existent Call (Should Timeout)

**Goal:** Verify watch times out for invalid call ID

```bash
cargo tangle blueprint jobs watch \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 0 \
  --service-id 0 \
  --call-id 99999 \
  --timeout-secs 5

# Expected: Timeout error after 5 seconds
```

### Test 4.3: Watch with Custom Timeout

**Goal:** Verify timeout parameter works correctly

```bash
# This should timeout quickly
cargo tangle blueprint jobs watch \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 0 \
  --service-id 0 \
  --call-id 99999 \
  --timeout-secs 2

# Expected: Timeout in approximately 2 seconds
```

---

## Phase 5: Jobs Show Command

### Test 5.1: Show Job Call Metadata (Before Completion)

**Goal:** Inspect a job call that hasn't been processed yet

> **Prerequisites:** Stop operator first to prevent immediate processing

```bash
# Stop operator (Ctrl+C in Terminal 4)

# Submit a job
PAYLOAD=$(cast abi-encode "f(string)" "Pending")
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

# Note the call ID
export PENDING_CALL_ID=X

# Show call metadata
cargo tangle blueprint jobs show \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 0 \
  --service-id 0 \
  --call-id $PENDING_CALL_ID

# Expected output:
# Job Call X
# Service ID: 0
# Blueprint ID: 0
# Job Index: 0
# Job Name: hello
# ...
# Completed: false
# Result Count: 0
```

### Test 5.2: Show Job Call Metadata (After Completion)

**Goal:** Inspect a completed job call

```bash
# Restart operator (Terminal 4)
# Wait for it to process the pending job

# Show call metadata again
cargo tangle blueprint jobs show \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 0 \
  --service-id 0 \
  --call-id $PENDING_CALL_ID

# Expected:
# Completed: true
# Result Count: 1
```

### Test 5.3: Show Job Call with JSON Output

**Goal:** Verify JSON output format

```bash
cargo tangle blueprint jobs show \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 0 \
  --service-id 0 \
  --call-id $PENDING_CALL_ID \
  --json | jq '.'

# Expected JSON structure:
# {
#   "service_id": 0,
#   "call_id": X,
#   "blueprint_id": 0,
#   "job_index": 0,
#   "job_name": "hello",
#   "job_description": "...",
#   "caller": "0x...",
#   "created_at": 12345,
#   "result_count": 1,
#   "payment_wei": "0",
#   "completed": true,
#   "parameters": {...},
#   "results": {...}
# }
```

### Test 5.4: Show Non-Existent Job Call (Should Fail)

**Goal:** Verify error handling for invalid call ID

```bash
cargo tangle blueprint jobs show \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --blueprint-id 0 \
  --service-id 0 \
  --call-id 99999

# Expected: Error - call not found or similar
```

---

## Phase 6: Error Handling and Edge Cases

### Test 6.1: Submit to Non-Existent Service (Should Fail)

**Goal:** Verify error when submitting to invalid service

```bash
PAYLOAD=$(cast abi-encode "f(string)" "Error")

cargo tangle blueprint jobs submit \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 99999 \
  --job 0 \
  --payload-hex $PAYLOAD

# Expected: Contract revert error
```

### Test 6.2: Submit Invalid Job Index (Should Fail)

**Goal:** Verify error when using invalid job index

```bash
PAYLOAD=$(cast abi-encode "f(string)" "Error")

cargo tangle blueprint jobs submit \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./user-keystore \
  --tangle-contract $TANGLE \
  --restaking-contract $RESTAKING \
  --status-registry-contract $STATUS_REGISTRY \
  --blueprint-id 0 \
  --service-id 0 \
  --job 255 \
  --payload-hex $PAYLOAD

# Expected: Error - invalid job index
```

### Test 6.3: Submit with Invalid Hex Payload (Should Fail)

**Goal:** Verify error handling for malformed hex

```bash
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
  --payload-hex "not-valid-hex"

# Expected: Error - invalid hex
```

### Test 6.4: Submit with Invalid JSON Params File (Should Fail)

**Goal:** Verify error handling for malformed JSON

```bash
echo "not valid json" > /tmp/invalid-params.json

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
  --params-file /tmp/invalid-params.json

# Expected: Error - failed to parse JSON
```

### Test 6.5: Submit with Mismatched Parameter Count (Should Fail)

**Goal:** Verify error when params don't match schema

```bash
# Job expects 1 parameter, provide 2
cat > /tmp/wrong-params.json << EOF
["Alice", "Extra"]
EOF

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
  --params-file /tmp/wrong-params.json

# Expected: Error - expected 1 arguments but file contains 2 values
```

### Test 6.6: Conflicting Input Options (Should Fail)

**Goal:** Verify mutual exclusivity of input options

```bash
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
  --payload-hex "0x1234" \
  --params-file /tmp/job-params-array.json

# Expected: Error - conflicting arguments
```

---

## Phase 7: Complex Parameter Types (If Blueprint Supports)

> **Note:** These tests require a blueprint with jobs that accept complex types.
> The default `svc-test-blueprint` may only have a simple string job.
> Skip this phase if complex types are not available.

### Test 7.1: Submit Job with Integer Parameters

```bash
# If blueprint has a job that accepts uint256
PAYLOAD=$(cast abi-encode "f(uint256)" 12345)

cargo tangle blueprint jobs submit \
  --blueprint-id 0 \
  --service-id 0 \
  --job <JOB_INDEX> \
  --payload-hex $PAYLOAD \
  ...
```

### Test 7.2: Submit Job with Address Parameter

```bash
PAYLOAD=$(cast abi-encode "f(address)" "0x70997970C51812dc3A010C7d01b50e0d17dc79C8")
# ... submit with payload
```

### Test 7.3: Submit Job with Array Parameter

```bash
# JSON params file with array
cat > /tmp/array-params.json << EOF
[["item1", "item2", "item3"]]
EOF

# ... submit with --params-file
```

### Test 7.4: Submit Job with Tuple Parameter

```bash
# JSON params file with tuple (as object)
cat > /tmp/tuple-params.json << EOF
[{
  "field1": "value1",
  "field2": 123
}]
EOF

# ... submit with --params-file
```

---

## Cleanup

```bash
# Stop all processes
pkill -f "anvil"
pkill -f "python.*http.server"
pkill -f "svc-test-blueprint"

# Remove test artifacts
rm /tmp/job-*.json /tmp/job-*.bin /tmp/*-params.json 2>/dev/null

# Remove test directory (optional)
cd /path/to/parent
rm -rf service-lifecycle-test
```

---

## Quick Reference: Common ABI Encoding

Use `cast abi-encode` from Foundry to encode job parameters:

```bash
# String
cast abi-encode "f(string)" "Hello"

# Multiple strings
cast abi-encode "f(string,string)" "Hello" "World"

# Integer
cast abi-encode "f(uint256)" 12345

# Address
cast abi-encode "f(address)" "0x70997970C51812dc3A010C7d01b50e0d17dc79C8"

# Bytes
cast abi-encode "f(bytes)" "0xdeadbeef"

# Tuple (struct)
cast abi-encode "f((string,uint256))" "(Hello,123)"

# Array
cast abi-encode "f(string[])" "[Hello,World]"
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
