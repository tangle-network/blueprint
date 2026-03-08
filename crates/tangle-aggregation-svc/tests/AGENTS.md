# tests

## Purpose
Integration test suite for the Tangle BLS Signature Aggregation Service. Validates multi-operator signature collection, threshold-based aggregation (count and stake-weighted), cryptographic verification, and task lifecycle management.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `integration.rs` - 15 integration tests (1,029 lines) covering full service lifecycle with deterministic BLS keypairs.
  - **Key items**: `generate_keypair(seed)`, `sign_message()`, `serialize_signature()`, `serialize_pubkey()`
  - **Interactions**: Uses `blueprint-crypto-bn254` (`ArkBlsBn254`) for BLS key generation, signing, aggregation, and verification

## Key APIs (no snippets)
- **Service**: `AggregationService::new(config)`, `init_task()`, `submit_signature()`, `get_status()`, `get_aggregated_result()`, `mark_submitted()`, `get_stats()`
- **Config**: `ServiceConfig` (verify_on_submit, validate_output), `TaskConfig` (threshold_type, operator_stakes, ttl)
- **Crypto**: `ArkBlsBn254::generate_with_seed()`, `sign_with_secret()`, `aggregate()`, `verify_aggregate()`
- **Types**: `SubmitSignatureRequest`, `TaskStatus`, `TaskForAggregation`, `ThresholdType::Count | StakeWeighted`

## Relationships
- **Depends on**: `blueprint-tangle-aggregation-svc` (parent crate), `blueprint-crypto-bn254`, `blueprint-crypto-core`, `alloy-primitives::U256`
- **Used by**: CI pipeline for aggregation service validation

## Files (detailed)

### `integration.rs`
- **Role**: 15 E2E tests covering the full aggregation service API surface.
- **Key items**: 3-of-3 consensus flow, stake-weighted thresholds (4 operators, 50% bps), duplicate rejection, invalid signature rejection, output validation, non-signer tracking, TTL expiry, service stats, multi-service redundancy, race condition handling
- **Interactions**: All tests use deterministic seeded BLS keypairs for reproducibility
- **Knobs / invariants**: Count threshold requires exact N signatures; stake-weighted uses basis points (0-10000); `mark_submitted()` is idempotent; result queryable before but not after marking

## End-to-end flow
1. Create `AggregationService` with config (verify_on_submit=true, validate_output=true)
2. Generate deterministic BLS keypairs via `ArkBlsBn254::generate_with_seed()`
3. Init task with service_id, call_id, output, operator count, threshold
4. Each operator signs `create_signing_message()` and submits via `submit_signature()`
5. Service verifies each signature, tracks in signer bitmap (U256)
6. When threshold met: `get_aggregated_result()` returns aggregated sig/pubkey
7. Verify with `ArkBlsBn254::verify_aggregate()` -> true
8. `mark_submitted()` prevents re-aggregation

## Notes
- All tests deterministic via seeded keypair generation
- Covers both Count and StakeWeighted threshold models
- Tests idempotent `mark_submitted()` for distributed "anyone submit" pattern
- TTL expiry test uses 50ms TTL with sleep to trigger expiration
- Non-signer tracking records which operators didn't sign for on-chain attribution
