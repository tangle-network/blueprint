# blueprint-sdk

Umbrella crate for building Blueprint services on Tangle.

`blueprint-sdk` re-exports the core runtime surface (jobs, router, runner, protocol integrations, optional gateways like x402/webhooks) so most projects can depend on one crate.

## What to use it for

- Building Blueprint services with a single dependency surface
- Wiring router + runner + producers + consumers
- Opting into protocol/gateway features through crate features

## Minimal runner wiring

```rust,ignore
use blueprint_sdk::Router;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment;

async fn ping() -> &'static str { "pong" }

let env = BlueprintEnvironment::default();
let router = Router::new().route(0, ping);

BlueprintRunner::builder((), env)
    .router(router)
    .run()
    .await?;
```

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/sdk
- Developer docs: https://docs.tangle.tools/developers/blueprint-sdk
