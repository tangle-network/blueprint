# tests

## Purpose
Comprehensive integration and E2E test suite for Blueprint Manager covering service lifecycle, protocol integration (Tangle EVM, EigenLayer), source fetching (GitHub, container, remote), and multi-runtime targets (Native/Hypervisor/Container).

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `common.rs` - Shared test utilities: context creation with RocksDB, EigenLayer AVS harness setup.
  - **Key items**: `create_test_context()`, `setup_incredible_squaring_avs_harness()`, `LifecycleTestHarness`, `RunnerTestHarness`
- `blueprint_sources_e2e.rs` - Source fetching E2E: GitHub releases, container images, remote HTTP/IPFS with SHA256 validation.
  - **Key items**: `BlueprintSourceHandler`, `GithubBinaryFetcher`, `RemoteBinaryFetcher`, `ContainerSource`, `BlueprintBinary`
- `service_lifecycle_tests.rs` - Unit tests for `Status` state machine and `ProcessHandle` channel mechanics.
  - **Key items**: `Status` enum (NotStarted/Pending/Running/Finished/Error), `ProcessHandle`, `wait_for_status_change()`
- `service_lifecycle_e2e.rs` - Full Tangle service lifecycle: registration, job submission, result polling.
  - **Key items**: `LifecycleTestHarness`, square job E2E, 300s timeout
- `multi_service_e2e.rs` - Multiple services with job isolation verification.
  - **Key items**: service independence, 120s timeout
- `protocol_flow_e2e.rs` - Protocol event emission (JobSubmitted, JobResultSubmitted).
  - **Key items**: event verification, 60s timeout
- `protocol_integration.rs` - ProtocolManager abstraction tests.
  - **Key items**: `ProtocolManager`, initialization, 10s timeout
- `remote_fetcher.rs` - Remote binary fetcher: checksums, caching, cache reuse.
  - **Key items**: `RemoteBinaryFetcher`, SHA256 validation, cache hit counting
- `runtime_target_test.rs` - Runtime validation across Native, Hypervisor, Container targets.
  - **Key items**: platform checks, feature-gated (Hypervisor/Container), 120s timeout
- `serverless_integration.rs` - FaaS compatibility analysis and strategy selection.
  - **Key items**: compatibility scoring, strategy recommendation
- `service_instancing_e2e.rs` - Multi-operator scenarios with permission flows.
  - **Key items**: operator permission via `addPermittedCallerCall`, 180s timeout
- `source_handler_integration.rs` - Cross-source integration validation.
  - **Key items**: GitHub, container, remote source handlers
- `tangle_runner.rs` - BlueprintRunner + Tangle protocol: producer/router/consumer pipeline.
  - **Key items**: `TangleProducer`, `TangleConsumer`, `Router`, Bridge with RocksDB auth, 1800s timeout
- `eigenlayer_e2e_test.rs` - EigenLayer AVS lifecycle E2E.
  - **Key items**: AVS harness, operator registration, 300s timeout

## Key APIs (no snippets)
- **Types**: `BlueprintManagerContext`, `ProtocolManager`, `TangleClient`, `Status`, `ProcessHandle`, `Router`, `TangleProducer`/`TangleConsumer`
- **Harnesses**: `create_test_context()`, `LifecycleTestHarness`, `RunnerTestHarness`, `setup_incredible_squaring_avs_harness()`

## Relationships
- **Depends on**: `blueprint-manager` (all internal modules), `blueprint-runner`, `blueprint-client-tangle`, `blueprint-auth`, `blueprint-anvil-testing-utils`, `alloy-*`
- **Used by**: CI pipeline for validating manager functionality

## Notes
- NO MOCKS: tests validate real implementations (networking, job submission, event polling)
- Socket paths use `/tmp` to avoid Unix SUN_LEN ~104-108 byte limit
- Some tests marked `@serial(env)` for env var isolation
- Feature-gated: Hypervisor/Container tests conditional on target OS
- Generous timeouts (60-1800s) balance coverage vs CI time
- ~6,286 lines across 14 test files
