# src

## Purpose
End-to-end test harness for Tangle EVM blueprints backed by Anvil. Provides `BlueprintHarness` which boots a seeded Anvil testnet, deploys the Tangle contract stack, wires up a `BlueprintRunner` with `TangleProducer`/`TangleConsumer`, and exposes helpers for submitting jobs and waiting for results.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Re-exports all submodules
- `anvil.rs` - `wait_for_responses()` helper that polls a mutex-guarded counter until a target response count is reached or timeout expires
- `blueprint.rs` - `BlueprintHarnessBuilder` (builder pattern for harness configuration); `BlueprintHarness` (end-to-end harness wiring `Router` into `BlueprintRunner` with Anvil backend); `OperatorBehavior` trait and implementations (`HonestOperator`, `DropAllOperator`); `OperatorSpec` / `OperatorFleet` for multi-operator test scenarios; `MultiOperatorConsumer` sink that fans out job results to per-operator channels; env-var lifecycle management; keystore seeding; client construction helpers
- `tangle.rs` - `SeededTangleTestnet` struct and `start_tangle_testnet()` that boots Anvil with deterministic state from `LocalTestnet.s.sol` broadcast artifacts; replays deployment transactions; provides contract addresses and client creation helpers
- `multi.rs` - `MultiHarness` for testing interactions between multiple cooperating blueprints on a shared Anvil instance, each with independent routers, service IDs, and operator fleets

## Key APIs
- `BlueprintHarness::builder(router)` - creates a `BlueprintHarnessBuilder`
- `BlueprintHarnessBuilder::spawn()` - boots Anvil, seeds contracts, starts runner, returns `BlueprintHarness`
- `BlueprintHarness::submit_job(job_index, payload)` - submits an ABI-encoded job to the harness service
- `BlueprintHarness::wait_for_job_result(submission)` - waits for local or on-chain result with 30s timeout
- `BlueprintHarness::shutdown()` - aborts tasks, restores env vars, cleans temp dir
- `OperatorFleet::<N, F>::new(specs)` - compile-time fleet descriptor with `N` operators and `F` faulty
- `start_tangle_testnet(include_logs)` - boots seeded Anvil with Tangle contracts
- `MultiHarness::builder()` - creates multi-blueprint test scenarios

## Relationships
- Depends on `blueprint_runner`, `blueprint_router`, `blueprint_client_tangle`, `blueprint_keystore` for core runner infrastructure
- Depends on `blueprint_chain_setup_anvil` for Anvil container management and snapshot loading
- Depends on `blueprint_tangle_extra` for `TangleProducer`/`TangleConsumer` (and optionally `AggregatingConsumer`)
- Uses `blueprint_core_testing_utils::Error` as the base error type
- Used by example blueprints and integration tests
