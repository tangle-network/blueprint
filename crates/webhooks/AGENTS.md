# webhooks

## Purpose
Webhook gateway for the Blueprint SDK. Exposes HTTP endpoints that authenticate inbound requests (TradingView alerts, price feeds, monitoring systems, etc.) and inject verified `JobCall`s into a running Blueprint via a producer stream.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Gateway implementation: `gateway.rs` (axum-based `BackgroundService` with per-endpoint POST handlers and `/webhooks/health`), `producer.rs` (`WebhookProducer` stream + `WebhookEvent`-to-`JobCall` conversion), `auth.rs` (constant-time verification for `none`/`bearer`/`hmac-sha256`/`api-key`), `config.rs` (TOML-loadable `WebhookConfig` with env-var secret resolution), `error.rs`

### Files
- `Cargo.toml` - Crate manifest (`blueprint-webhooks`); depends on `blueprint-core`, `blueprint-runner`, `axum`, `hmac`/`sha2`/`subtle` for HMAC auth
- `README.md` - Quick integration guide, auth modes, and related links

## Key APIs (no snippets)
- `WebhookGateway::new(config) -> (WebhookGateway, WebhookProducer)` - Creates paired gateway (BackgroundService) and producer (Stream)
- `WebhookConfig::from_toml(path)` - Load and validate TOML configuration with per-endpoint route/auth settings
- `WebhookProducer` - Implements `Stream<Item = Result<JobCall, BoxError>>` for integration with `BlueprintRunner::producer()`
- `auth::verify(endpoint, headers, body)` - Authenticate a request against endpoint config (constant-time comparison)
- `WebhookError` - Error enum with conversion to `RunnerError`
- Auth modes: `none`, `bearer` (token), `hmac-sha256` (signature header), `api-key` (header)

## Relationships
- Depends on `blueprint-core` for `JobCall`, `JobId`, `MetadataMap`, `MetadataValue`
- Depends on `blueprint-runner` for `BackgroundService` trait and `RunnerError`
- Re-exported by `blueprint-sdk` as `webhooks` module (behind `webhooks` feature)
- Designed to be plugged into `BlueprintRunner` alongside a router and other producers

## Notes
- Config supports `env:VAR_NAME` prefix for secret resolution from environment variables
- All auth comparisons use constant-time equality (`subtle` crate) to prevent timing attacks
- The gateway injects metadata headers (origin, path, name, service ID, call ID) into each `JobCall`
