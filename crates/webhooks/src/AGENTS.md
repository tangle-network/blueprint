# src

## Purpose
Implements an HTTP webhook gateway that converts external HTTP requests (TradingView alerts, price feeds, monitoring systems, etc.) into Blueprint `JobCall`s. The gateway runs as a `BackgroundService` inside the Blueprint runner, verifying authentication and injecting jobs into the router via a producer stream.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Crate root; declares modules and re-exports `WebhookConfig`, `WebhookError`, `WebhookGateway`, `WebhookProducer`
- `auth.rs` - Request authentication verification supporting four methods: `none`, `bearer`, `hmac-sha256`, and `api-key`; all comparisons use constant-time equality
- `config.rs` - `WebhookConfig` (TOML-loadable, validated) and `WebhookEndpoint` structs; supports env-var secret resolution via `env:VAR_NAME` prefix
- `error.rs` - `WebhookError` enum (Config, AuthFailed, Server, ProducerChannelClosed, Io, TomlParse) with conversion to `RunnerError`
- `gateway.rs` - `WebhookGateway` struct implementing `BackgroundService`; builds an axum `Router` with per-endpoint POST handlers and a `/webhooks/health` GET endpoint
- `producer.rs` - `WebhookProducer` (implements `Stream<Item = Result<JobCall, BoxError>>`) and `WebhookEvent` struct; converts webhook payloads into `JobCall`s with metadata headers (origin, path, name, service ID, call ID)

## Key APIs
- `WebhookGateway::new(config) -> (WebhookGateway, WebhookProducer)`: creates paired gateway and producer
- `WebhookConfig::from_toml(path)`: load and validate TOML configuration
- `WebhookProducer`: implements `Stream` for integration with `BlueprintRunner::producer()`
- `auth::verify(endpoint, headers, body)`: authenticate a request against endpoint config
- `WebhookEvent::into_job_call()`: convert a verified event into a `JobCall` with webhook metadata

## Relationships
- Depends on `blueprint-runner` for `BackgroundService` trait and `RunnerError`
- Depends on `blueprint-core` for `JobCall`, `JobId`, `MetadataMap`, `MetadataValue`
- Uses `axum` for the HTTP server, `hmac`/`sha2`/`subtle` for HMAC auth
- Designed to be plugged into `BlueprintRunner` alongside a router and other producers
