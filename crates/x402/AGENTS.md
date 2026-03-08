# x402

## Purpose
x402 payment gateway for the Blueprint SDK. Exposes paid HTTP job execution endpoints where clients settle via the x402 payment protocol (stablecoins on any supported EVM chain), and the gateway injects verified `JobCall`s into the Blueprint runner. Integrates with the RFQ/pricing system so operator-signed price quotes serve as chain-agnostic invoices.

## Contents (one hop)
### Subdirectories
- [x] `config/` - Example TOML configuration (`x402.example.toml`) documenting all operator-configurable settings: bind address, facilitator URL, job policies, accepted tokens
- [x] `src/` - Gateway implementation: `gateway.rs` (axum + x402 middleware `BackgroundService`), `producer.rs` (`X402Producer` stream), `config.rs` (TOML-loadable `X402Config`), `quote_registry.rs` (`QuoteRegistry` for price quote management), `settlement.rs` (`SettlementOption` for chain/token configuration), `error.rs`

### Files
- `Cargo.toml` - Crate manifest (`blueprint-x402`); depends on `x402-types`, `x402-axum`, `x402-chain-eip155` for the x402 protocol, plus `alloy-primitives`, `alloy-provider`, `tnt-core-bindings`
- `README.md` - Quick integration guide, policy model, auth modes, and related links

## Key APIs (no snippets)
- `X402Gateway::new(config, job_pricing) -> (X402Gateway, X402Producer)` - Creates paired gateway (BackgroundService) and producer (Stream)
- `X402Config::from_toml(path)` - Load and validate TOML configuration
- `X402Producer` - Implements `Stream<Item = Result<JobCall, BoxError>>` for integration with `BlueprintRunner::producer()`
- `QuoteRegistry` - Manages operator-signed price quotes with TTL
- `SettlementOption` - Defines accepted token/chain/rate for payment settlement
- `JobPolicyConfig` - Per-job policy: `disabled`, `public_paid`, `restricted_paid`
- `X402CallerAuthMode` - Auth for restricted jobs: `payer_is_caller`, `delegated_caller_signature`
- `X402InvocationMode` - How jobs are invoked after payment verification
- `X402Error` - Error enum with HTTP status mappings
- Auth dry-run endpoint: `POST /x402/jobs/{service_id}/{job_index}/auth-dry-run`

## Relationships
- Depends on `blueprint-core` for `JobCall` and job primitives
- Depends on `blueprint-router` for router type references
- Depends on `blueprint-runner` for `BackgroundService` trait
- Uses `x402-types`, `x402-axum`, `x402-chain-eip155` for the x402 protocol implementation
- Re-exported by `blueprint-sdk` as `x402` module (behind `x402` feature)
- Designed to be plugged into `BlueprintRunner` alongside a router and other producers

## Notes
- Supports all EVM chains with `transferWithAuthorization` (EIP-3009): Base, Ethereum, Polygon, Arbitrum, Optimism, etc.
- Three job policy modes: `disabled` (no x402), `public_paid` (anyone can pay and invoke), `restricted_paid` (auth required)
- The `config/x402.example.toml` documents the complete configuration schema including accepted tokens with CAIP-2 network identifiers
