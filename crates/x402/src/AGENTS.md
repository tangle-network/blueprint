# src

## Purpose
Implements a cross-chain EVM payment gateway using the x402 payment protocol. The gateway runs as a `BackgroundService` in the Blueprint runner, accepting stablecoin payments on supported EVM chains, verifying them via a facilitator, and injecting `JobCall`s into the runner's producer stream. Supports public-paid and restricted-paid invocation modes with on-chain caller permission checks.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Crate root; declares modules and re-exports `X402Config`, `JobPolicyConfig`, `X402CallerAuthMode`, `X402InvocationMode`, `X402Error`, `X402Gateway`, `X402Producer`, `QuoteRegistry`, `SettlementOption`
- `config.rs` - Configuration types: `X402Config` (TOML-loadable), `AcceptedToken` (with `convert_wei_to_amount` for price conversion using exchange rates and basis-point markup), `JobPolicyConfig`, `X402InvocationMode` enum (Disabled/PublicPaid/RestrictedPaid), `X402CallerAuthMode` enum (PayerIsCaller/DelegatedCallerSignature/PaymentOnly)
- `error.rs` - `X402Error` enum (Config, QuoteNotFound, PaymentVerification, PriceConversion, JobNotAvailable, Server, ProducerChannelClosed, Io, TomlParse) with conversion to `RunnerError`
- `gateway.rs` - `X402Gateway` implementing `BackgroundService`; builds an axum router with x402 middleware per job endpoint, handles payment verification via facilitator, supports restricted-paid mode with `isPermittedCaller` on-chain dry-run checks, dynamic quote generation, and delegated caller signature verification
- `producer.rs` - `X402Producer` (implements `Stream<Item = Result<JobCall, BoxError>>`) and `VerifiedPayment` struct; converts verified payments into `JobCall`s with x402-specific metadata (origin, quote digest, payment network/token, caller address)
- `quote_registry.rs` - `QuoteRegistry`: thread-safe in-memory store (backed by `DashMap`) mapping EIP-712 quote digests to `QuoteEntry` records; supports insert, dynamic insert, get, consume (single-use), and garbage collection of expired/consumed entries
- `settlement.rs` - `SettlementOption` struct: serializable description of how a client can pay (network, asset, amount, pay_to, scheme, x402_endpoint), included in RFQ responses

## Key APIs
- `X402Gateway::new(config, job_pricing) -> (X402Gateway, X402Producer)`: creates paired gateway and producer
- `X402Config::from_toml(path)`: load and validate TOML configuration
- `AcceptedToken::convert_wei_to_amount(wei_price)`: convert wei to token smallest units with rate and markup
- `QuoteRegistry::insert_dynamic(service_id, job_index, price_wei) -> [u8; 32]`: register a dynamically priced quote
- `QuoteRegistry::consume(digest) -> Option<QuoteEntry>`: atomically consume a quote (prevents double-spend)
- `X402Producer`: implements `Stream` for integration with `BlueprintRunner::producer()`
- `VerifiedPayment::into_job_call()`: convert a verified payment into a `JobCall`

## Relationships
- Depends on `blueprint-runner` for `BackgroundService` trait and `RunnerError`
- Depends on `blueprint-core` for `JobCall`, `JobId`, `MetadataMap`
- Uses `x402-axum` for the x402 payment middleware and `x402-types` for protocol types
- Uses `alloy-primitives` / `alloy-provider` for EVM address parsing and on-chain calls
- Uses `tnt-core-bindings` for `ITangle` contract interface (restricted-paid permission checks)
- Configuration schema documented in `crates/x402/config/x402.example.toml`
