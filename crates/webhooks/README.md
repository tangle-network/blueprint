# blueprint-webhooks

Webhook gateway for Blueprint SDK.

This crate exposes HTTP endpoints that authenticate inbound requests and inject `JobCall`s into a running Blueprint.

## What it provides

- `WebhookGateway` background service (axum)
- `WebhookProducer` stream that converts verified webhook events into `JobCall`s
- Per-endpoint route mapping (`path -> job_id`)
- Auth modes:
  - `none`
  - `bearer`
  - `hmac-sha256`
  - `api-key`
- Health endpoint:
  - `GET /webhooks/health`

## Quick integration

```rust,ignore
use blueprint_webhooks::{WebhookConfig, WebhookGateway};
use blueprint_runner::BlueprintRunner;

let config = WebhookConfig::from_toml("webhooks.toml")?;
let (gateway, producer) = WebhookGateway::new(config)?;

BlueprintRunner::builder((), env)
    .router(router)
    .producer(producer)
    .background_service(gateway)
    .run()
    .await?;
```

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/webhooks
- Docs site page: https://docs.tangle.tools/developers/blueprint-runner/webhooks
- Trigger model context (cron/on-chain/webhooks/x402): https://docs.tangle.tools/developers/blueprint-runner/job-triggers
