# x402

## Purpose
x402 / MPP payment gateway for the Blueprint SDK. Exposes paid HTTP job execution endpoints under TWO parallel ingresses:
1. **x402** (`/x402/jobs/...`) — the legacy ingress, speaks `X-PAYMENT` / `X-Payment-Response` headers via `x402-axum`. Settles stablecoins on any EVM chain with EIP-3009 / Permit2.
2. **MPP** (`/mpp/jobs/...`, opt-in) — the IETF Payment HTTP Authentication Scheme (`WWW-Authenticate: Payment` / `Authorization: Payment` / `Payment-Receipt`). Built on the `mpp` crate (`tempoxyz/mpp-rs`). RFC 9457 Problem Details for errors. See <https://paymentauth.org> and <https://mpp.dev>.

Both ingresses share **all** downstream plumbing — job pricing, accepted tokens / cross-chain markup, restricted-caller policies (including on-chain `isPermittedCaller` checks and the delegated-signature replay guard), the quote registry, and the producer/runner injection path. Only the wire format on the request side differs. The MPP credential payload wraps the same EIP-3009 `PaymentPayload` x402 clients already produce, so existing x402 wallets work over MPP unchanged.

## Contents (one hop)
### Subdirectories
- [x] `config/` - Example TOML configuration (`x402.example.toml`) documenting all operator-configurable settings: bind address, facilitator URL, job policies, accepted tokens, optional MPP section
- [x] `src/` - Gateway implementation:
  - `gateway.rs` - axum + `x402-axum` middleware `BackgroundService`, shared `handle_paid_job_inner` ingress helper
  - `producer.rs` - `X402Producer` stream
  - `config.rs` - TOML-loadable `X402Config` with optional `MppConfig` section
  - `quote_registry.rs` - `QuoteRegistry` for price quote management
  - `settlement.rs` - `SettlementOption` (with `PaymentProtocol` discriminator)
  - `error.rs` - `X402Error` with `Mpp(String)` variant
  - `mpp/` - MPP ingress module (see below)
- [x] `src/mpp/` - MPP ingress: `mod.rs` (re-exports), `state.rs` (`MppGatewayState`), `method.rs` (`BlueprintEvmChargeMethod` impl `ChargeMethod`), `credential.rs` (`MppCredentialPayload`, `MppMethodDetails`), `routes.rs` (axum handlers)

### Files
- `Cargo.toml` - Crate manifest (`blueprint-x402`); depends on `x402-types`, `x402-axum`, `x402-chain-eip155` for the x402 protocol; `mpp` (`server`+`axum` features only — NOT `tempo`/`evm` to avoid pulling tempo-alloy / tempo-primitives) for the MPP ingress; `alloy-primitives`, `alloy-provider`, `tnt-core-bindings`, `base64`, `time`
- `README.md` - Quick integration guide, policy model, auth modes, MPP enablement, and related links

## Key APIs (no snippets)
- `X402Gateway::new(config, job_pricing) -> (X402Gateway, X402Producer)` - Creates paired gateway (BackgroundService) and producer (Stream). Constructs `MppGatewayState` automatically when `config.mpp` is `Some`.
- `X402Config::from_toml(path)` - Load and validate TOML configuration (including the optional `[mpp]` section)
- `X402Producer` - Implements `Stream<Item = Result<JobCall, BoxError>>` for integration with `BlueprintRunner::producer()`
- `QuoteRegistry` - Manages operator-signed price quotes with TTL
- `SettlementOption` - Defines accepted token/chain/rate; carries a `PaymentProtocol` (`X402` or `Mpp`) discriminator
- `JobPolicyConfig` - Per-job policy: `disabled`, `public_paid`, `restricted_paid`
- `X402CallerAuthMode` - Auth for restricted jobs: `payer_is_caller`, `delegated_caller_signature`
- `X402InvocationMode` - How jobs are invoked after payment verification
- `MppConfig` - Optional MPP ingress config: `realm`, `secret_key` (≥32 bytes), `challenge_ttl_secs`
- `BlueprintEvmChargeMethod` - `mpp::ChargeMethod` impl that forwards verification to the configured x402 facilitator. Method name `"x402-evm"`.
- `MppCredentialPayload` / `MppMethodDetails` - Wire types for the `x402-evm` MPP method
- `X402Error` - Error enum with HTTP status mappings; `Mpp(String)` variant for MPP-specific failures
- Auth dry-run endpoint: `POST /x402/jobs/{service_id}/{job_index}/auth-dry-run`
- MPP request endpoint: `POST /mpp/jobs/{service_id}/{job_index}` (only when MPP is configured)
- MPP discovery endpoint: `GET /mpp/jobs/{service_id}/{job_index}/price` (only when MPP is configured)

## Relationships
- Depends on `blueprint-core` for `JobCall` and job primitives
- Depends on `blueprint-router` for router type references
- Depends on `blueprint-runner` for `BackgroundService` trait
- Uses `x402-types`, `x402-axum`, `x402-chain-eip155` for the x402 protocol implementation
- Uses `mpp` (server + axum features only) for the MPP wire format and `ChargeMethod` trait
- The `mpp` `tempo` / `evm` features are intentionally NOT enabled (they would pull tempo-alloy/tempo-primitives, which are Tempo-blockchain-specific and conflict with the Blueprint EVM model)
- Re-exported by `blueprint-sdk` as `x402` module (behind `x402` feature)
- Designed to be plugged into `BlueprintRunner` alongside a router and other producers

## Notes
- Supports all EVM chains with `transferWithAuthorization` (EIP-3009): Base, Ethereum, Polygon, Arbitrum, Optimism, etc.
- Three job policy modes: `disabled` (no x402), `public_paid` (anyone can pay and invoke), `restricted_paid` (auth required)
- The `config/x402.example.toml` documents the complete configuration schema including accepted tokens with CAIP-2 network identifiers and the optional `[mpp]` section
- MPP and x402 ingresses are wire-protocol parallel: a single Tangle blueprint can accept both formats simultaneously, sharing the same job pricing and policy enforcement
- The MPP credential format wraps a base64url-encoded x402 v1 `PaymentPayload`, so x402 client wallets work over MPP without modification — they just put the same signed authorisation in `Authorization: Payment` instead of `X-PAYMENT`
- MPP errors use RFC 9457 Problem Details (`application/problem+json`) per the IETF spec, with `type` URIs under `https://paymentauth.org/problems/`
- MPP `secret_key` MUST be ≥32 bytes and SHOULD be rotated on a schedule (rotation invalidates outstanding challenges; in-flight clients transparently retry on a fresh 402)
