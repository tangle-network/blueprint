# tests

## Purpose
Integration tests for `blueprint-tangle-extra` crate: validates producer/consumer functionality, aggregation workflows, and TangleClient APIs using Anvil-based seeded testnets.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `aggregation_e2e.rs` - Multi-operator aggregation E2E (343 lines, 3 tests): aggregation config queries, operator membership/weights, and result submission flow.
  - **Key items**: `MultiOperatorHarness`, `TangleClient::requires_aggregation()`, `get_aggregation_config()`, `get_service_operators()`, `submit_result()`
  - **Interactions**: Uses filesystem keystores, seeded Anvil testnet, 180s timeout per test
- `anvil_integration.rs` - TangleClient API unit tests (205 lines, 5 tests): result submission, operator weights, operator info, total exposure, aggregated result submission.
  - **Key items**: `TangleClient::submit_result()`, `get_service_operator_weights()`, `get_service_operator()`, `get_service_total_exposure()`, `submit_aggregated_result()`
  - **Interactions**: In-memory keystore, single client per test, 1800s timeout
- `producer.rs` - TangleProducer event streaming validation (237 lines, 1 test): submits realistic job and validates metadata extraction from stream.
  - **Key items**: `TangleProducer::from_block()`, `with_poll_interval(50ms)`, `TangleArg<(String, String, String)>`, `ServiceId`, `CallId` extractors
  - **Interactions**: Multi-threaded tokio (2 workers), 600s stream wait within 1800s timeout

## Key APIs (no snippets)
- **Producer**: `TangleProducer::from_block()`, `with_poll_interval()`, `Stream::next()` yielding `JobCall`
- **Client**: `TangleClient` methods: `requires_aggregation()`, `get_aggregation_config()`, `get_service_operators()`, `submit_result()`, `submit_aggregated_result()`, `submit_job()`
- **Extractors**: `TangleArg<T>`, `ServiceId`, `CallId` (metadata extractors from JobCall)

## Relationships
- **Depends on**: `blueprint-tangle-extra` (parent), `blueprint-client-tangle`, `blueprint-anvil-testing-utils` (SeededTangleTestnet), `blueprint-crypto` (K256Ecdsa), `blueprint-keystore`, `alloy-*`
- **Used by**: CI pipeline with `--test-threads=1` (serial execution required)

## Notes
- All tests skip gracefully when tnt-core artifacts are missing
- Serial execution required (mDNS/port conflicts); marked as serial crate in CI
- Deterministic: all tests use well-known Anvil private keys
- Filesystem keystores (aggregation_e2e) vs in-memory (anvil_integration, producer)
