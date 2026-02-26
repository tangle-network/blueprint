# blueprint-x402

x402 payment gateway for Blueprint SDK.

This crate exposes paid HTTP job execution for Blueprint runners: clients settle via x402, then the gateway injects a verified `JobCall` into the runner.

## What it provides

- `X402Gateway` background service (axum + x402 middleware)
- `X402Producer` stream that converts verified payments into `JobCall`s
- Per-job x402 policy model:
  - `disabled`
  - `public_paid`
  - `restricted_paid`
- Restricted auth modes:
  - `payer_is_caller`
  - `delegated_caller_signature`
- Auth dry-run endpoint for restricted policy checks:
  - `POST /x402/jobs/{service_id}/{job_index}/auth-dry-run`

## Quick integration

```rust,ignore
use blueprint_x402::{X402Config, X402Gateway};
use blueprint_runner::BlueprintRunner;

let config = X402Config::from_toml("x402.toml")?;
let (gateway, producer) = X402Gateway::new(config, job_pricing)?;

BlueprintRunner::builder((), env)
    .router(router)
    .producer(producer)
    .background_service(gateway)
    .run()
    .await?;
```

`job_pricing` is a `(service_id, job_index) -> price_wei` map.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/x402
- Example blueprint (end-to-end): https://github.com/tangle-network/blueprint/blob/main/examples/x402-blueprint/README.md
- Example x402 config: https://github.com/tangle-network/blueprint/blob/main/examples/x402-blueprint/config/x402.toml
- Delegated auth dry-run helper script: https://github.com/tangle-network/blueprint/blob/main/scripts/x402-auth-dry-run.sh
- Docs site page: https://docs.tangle.tools/developers/blueprint-runner/x402
- Trigger model context (cron/on-chain/webhooks/x402): https://docs.tangle.tools/developers/blueprint-runner/job-triggers
