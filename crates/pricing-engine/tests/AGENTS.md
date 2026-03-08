# tests

## Purpose
Integration test suite for the pricing-engine crate covering TOML config loading, EIP-712 cryptographic signing verification, and end-to-end gRPC server + blockchain quote submission via Anvil testnet.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `pricing_config_test.rs` - Config loading and price calculation validation: loads `config/default_pricing.toml`, validates 9 resource types, tests pricing formula.
  - **Key items**: `test_default_pricing_config()`, `test_resource_price_calculation()`, formula: `count * rate * ttl_blocks * 6s * security_factor`
- `signer_test.rs` - EIP-712 signing/verification roundtrips: sign, verify, tamper-detect, digest mutation.
  - **Key items**: `test_sign_and_verify_quote()`, `OperatorSigner`, `SignableQuote`, `verify_quote()`
- `evm_listener.rs` - E2E integration tests (14+ tests): gRPC server roundtrips, PoW validation, on-chain quote submission via Anvil.
  - **Key items**: `setup_pricing_engine()`, `request_quote()`, `convert_to_onchain_quote()`, `MockClient`, `e2e_quote_submission_on_chain()`, `e2e_create_service_from_quote_on_chain()`
  - **Interactions**: Feature-gated by `pricing-engine-e2e-tests`; boots `SeededTangleTestnet` for on-chain tests
- `utils.rs` - Deterministic test fixtures and config builders.
  - **Key items**: `create_test_config()`, `create_test_quote_details()`, `sample_benchmark_profile()`, `sample_pricing_map()`

## Key APIs (no snippets)
- **Pricing**: `init_pricing_config()`, `calculate_resource_price()`, `PriceModel`, `ResourceUnit`
- **Signing**: `OperatorSigner::sign_quote()`, `verify_quote()`, `quote_digest_eip712()`
- **gRPC**: `PricingEngineService`, `PricingEngineClient::get_price()`, `GetPriceRequest/Response`
- **Blockchain**: `EvmEventListener`, `TangleClient`, `ITangleServices::createServiceFromQuotesCall`

## Relationships
- **Depends on**: `blueprint-crypto::k256`, `blueprint-keystore`, `blueprint-client-tangle`, `blueprint-anvil-testing-utils`, `alloy-*`, `tonic`, `tokio`
- **Used by**: CI pipeline for validating pricing-engine correctness

## Notes
- E2E tests skip gracefully without `pricing-engine-e2e-tests` feature or missing tnt-core artifacts
- PoW difficulty=1 for fast CI; configurable per test
- 30-minute Anvil test timeout; per-test 2-5 second event timeouts
- Each test creates temp keystore/database via `tempfile`
- Decimal precision for monetary calculations (`rust_decimal::Decimal`)
