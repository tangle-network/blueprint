# blueprint-runner

Execution runtime for Blueprint jobs.

`blueprint-runner` owns the long-running execution loop: consume `JobCall`s from producers, route calls to handlers, and forward `JobResult`s to consumers. It also hosts background services (for example webhooks/x402 gateways).

## Core responsibilities

- Runner builder and lifecycle management
- Producer/consumer orchestration
- Background service execution
- Protocol-aware runtime hooks (feature-gated)

## Minimal setup

```rust,ignore
use blueprint_runner::BlueprintRunner;
use blueprint_runner::config::BlueprintEnvironment;
use blueprint_router::Router;

async fn ping() -> &'static str { "pong" }

let env = BlueprintEnvironment::default();
let router = Router::new().route(0, ping);

BlueprintRunner::builder((), env)
    .router(router)
    .run()
    .await?;
```

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/runner
- Runner docs: https://docs.tangle.tools/developers/blueprint-runner/introduction
