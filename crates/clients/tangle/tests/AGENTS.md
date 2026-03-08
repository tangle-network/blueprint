# tests

## Purpose
Integration tests for `TangleClient` against a real Anvil EVM testnet seeded with the LocalTestnet deployment script. Exercises client APIs end-to-end without mocks.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `anvil.rs` - Three integration tests: `client_reads_blueprint_state` (verifies blueprint active status and service operator count), `client_fetches_operator_metadata` (validates RPC endpoint, stake, restaking status, and public key format), and `client_submits_result_transaction` (submits a job result and verifies transaction success). Includes helpers: `create_test_client()` (builds `TangleClient` with in-memory keystore and seeded K256 ECDSA key), `boot_testnet()` (spawns `SeededTangleTestnet` or skips if tnt-core artifacts missing), and `run_anvil_test()` (wraps tests in a 30-minute timeout).

## Key APIs
- `TangleClient::with_keystore()` - client construction with explicit keystore
- `TangleClient::get_blueprint_info()` / `get_service_info()` / `get_operator_metadata()` - read queries
- `TangleClient::submit_result()` - job result submission
- `SeededTangleTestnet` / `harness_builder_from_env()` - test harness from `blueprint_anvil_testing_utils`

## Relationships
- Uses `blueprint_anvil_testing_utils` for testnet lifecycle (`SeededTangleTestnet`, `harness_builder_from_env`, `missing_tnt_core_artifacts`)
- Uses `blueprint_client_tangle` types (`TangleClient`, `TangleClientConfig`, `TangleSettings`, `RestakingStatus`, `ServiceStatus`)
- Uses `blueprint_keystore` with in-memory backend for test key management
- Tests reference `LOCAL_BLUEPRINT_ID` and `LOCAL_SERVICE_ID` constants from the seeded deployment
