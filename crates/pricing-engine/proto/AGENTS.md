# proto

## Purpose
Protocol Buffer definitions for the Pricing Engine gRPC service. Defines the `PricingEngine` service contract with two RPCs for calculating and signing price quotes for blueprint deployments and job executions.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `pricing.proto` - gRPC service definition (192 lines) with two RPCs, three pricing models, asset security requirements, and x402 cross-chain settlement options.
  - **Key items**: `PricingEngine` service, `GetPrice`/`GetJobPrice` RPCs, `PricingModelHint` enum (PAY_ONCE, SUBSCRIPTION, EVENT_DRIVEN), `QuoteDetails`, `JobQuoteDetails`, `ResourcePricing`, `SettlementOption`, `AssetSecurityRequirements`

## Key APIs (no snippets)
- **Service**: `PricingEngine` with `GetPrice(GetPriceRequest) -> GetPriceResponse` and `GetJobPrice(GetJobPriceRequest) -> GetJobPriceResponse`
- **Enums**: `PricingModelHint` (PAY_ONCE default, SUBSCRIPTION, EVENT_DRIVEN), `AssetType` (CUSTOM, ERC20)
- **Messages**: `QuoteDetails` (blueprint-level quote), `JobQuoteDetails` (per-job quote), `ResourcePricing` (kind, count, rate), `SettlementOption` (network, token, amount, pay_to)

## Relationships
- **Depends on**: `tonic-build` (compilation in `build.rs`), `protobuf_src` (bundled protoc)
- **Used by**: `src/lib.rs` (re-exports generated code as `pub mod proto`), `src/service/rpc/server.rs` (implements service trait), `src/remote.rs` (gRPC client), tests
- **Data/control flow**:
  - `build.rs` compiles proto -> generates `$OUT_DIR/pricing_engine.rs`
  - Server implements `PricingEngine` trait, validates PoW, calculates prices, signs with EIP-712
  - Signatures are 65 bytes (r || s || v) for on-chain verification

## Files (detailed)

### `pricing.proto`
- **Role**: Single proto file defining the complete gRPC API for pricing quotes.
- **Key items**: 2 RPCs, 3 pricing models, `AssetSecurityRequirements`/`AssetSecurityCommitment`, `SettlementOption` for x402
- **Interactions**: Both requests include `proof_of_work` (SHA2-based, difficulty 20) and timestamp (30s tolerance)
- **Knobs / invariants**: PAY_ONCE is default model; proto3 optional fields require `--experimental_allow_proto3_optional` in build.rs; PoW prevents DDoS

## End-to-end flow
1. `build.rs` compiles `pricing.proto` via `tonic-build`
2. Client sends `GetPriceRequest` with blueprint_id, ttl, pricing model, PoW
3. Server validates timestamp (30s), verifies PoW, calculates price
4. Server builds `QuoteDetails`, signs with EIP-712, returns response
5. Client can submit signed quote on-chain for service creation

## Notes
- Proto3 semantics: all fields optional; server validates presence in Rust
- x402 settlement options are optional and don't break existing clients
- Clock drift tolerance of 30 seconds prevents replay attacks
