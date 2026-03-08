# tests

## Purpose
Integration tests for the signature aggregation gossip protocol (`agg-sig-gossip`). Validates distributed signature collection and aggregation across multiple network nodes using BLS381, BLS377, and BN254 cryptographic schemes with threshold-based weight verification.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Generic test harness and concrete test cases for signature aggregation.
  - **Key items**: `run_signature_aggregation_test<S>()` (generic async harness), `unique_test_suffix()`, `dialable_addr()`, `setup_log()`, `TEST_TIMEOUT` (180s)
  - **Interactions**: Creates `TestNode` instances with shared allowed keys, calls `wait_for_all_handshakes()` for sync, instantiates `SignatureAggregationProtocol` per node with `ProtocolConfig::for_testing()` and `EqualWeight`, runs `.run(&message_hash)` concurrently via `join_all()`

## Key APIs (no snippets)
- **Functions**: `run_signature_aggregation_test<S: AggregatableSignature>(num_nodes, threshold_percentage, network_name, instance_name)` - parameterized test harness
- **Test modules**: `bls_tests` (BLS381/BLS377), `bn254_tests` (ArkBlsBn254), `w3f_bls_tests` (W3F BLS381/BLS377)

## Relationships
- **Depends on**: `SignatureAggregationProtocol`, `ProtocolConfig`, `EqualWeight` (from parent crate); `TestNode`, `NetworkServiceHandle`, `wait_for_all_handshakes()` (from `blueprint-networking`); `AggregatableSignature`, `blake3_256()` (from `blueprint-crypto`)
- **Used by**: CI test suite (requires `--test-threads=1` due to mDNS cross-contamination)
- **Data/control flow**:
  - Generate N keypairs, create TestNodes with bootstrap peers, wait for handshakes
  - Each node signs shared message hash (blake3), broadcasts signature shares
  - Protocol collects signatures, checks weighted threshold (67%), aggregates
  - Tests validate all nodes complete successfully with correct contributor count
  - 2s cleanup delay after shutdown for mDNS expiry

## Notes
- All tests use `#[serial_test::serial]` to prevent port/mDNS conflicts
- Unique test suffixes (PID + nanoseconds) prevent network name collisions
