# tests

## Purpose
Unit tests for BlueprintRunner builder pattern validation and error handling. Covers builder component requirements, producer error propagation, component chaining, custom config acceptance, and error Display formatting.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `runner_tests.rs` - 12 tests (318 lines) covering builder validation, producer error handling, builder chaining, blueprint config, and error display.
  - **Key items**: `BlueprintRunner::builder()`, `PendingProducer`, `ErrorProducer`, `EndingProducer`, `TestBackgroundService`, `ContinueRunningConfig`

## Key APIs (no snippets)
- **Types tested**: `BlueprintRunner` (builder pattern), `BlueprintConfig` trait, `BackgroundService` trait, `BlueprintEnvironment` (test_mode), `RunnerError`, `ProducerError`, `JobCallError`
- **Helpers**: `test_env()`, `PendingProducer`, `ErrorProducer`, `EndingProducer`, `TestBackgroundService`, `ContinueRunningConfig`

## Relationships
- **Depends on**: `blueprint-runner` (parent crate), `blueprint-router`, `tokio`
- **Used by**: CI pipeline for runner API validation

## Files (detailed)

### `runner_tests.rs`
- **Role**: Comprehensive builder API surface testing with mock producers and services.
- **Key items**: 3 builder validation tests, 2 producer error tests, 3 builder chain tests, 1 config test, 3 error display tests
- **Interactions**: All tests use `test_env()` which creates `BlueprintEnvironment` with `test_mode=true`
- **Knobs / invariants**: 500ms timeout wrapper for bounded execution; `handle.abort()` for spawned task cleanup; 9 async tests (`#[tokio::test]`), 3 sync tests

## End-to-end flow
1. Create test environment with `test_mode=true`
2. Build runner via `.builder().router(router).producer(producer)`
3. Validate builder errors (NoRouter, NoProducers) or successful construction
4. For error propagation: spawn runner, await error within timeout
5. For chaining: add multiple producers, background services, shutdown handlers

## Notes
- Intentionally scoped to builder API; integration tests require actual protocol implementations
- All tests isolated, no shared state, fully parallelizable
- No external fixtures or test data files
