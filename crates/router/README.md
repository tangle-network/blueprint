# blueprint-router

Job routing layer for Blueprint runtimes.

`blueprint-router` maps incoming `JobId` values to async handlers and is the dispatch core used by `blueprint-runner`.

## What to use it for

- Registering job handlers by ID
- Composing route tables for service job surfaces
- Dispatching calls from heterogeneous producer sources

## Example

```rust,ignore
use blueprint_router::Router;

async fn echo() -> &'static str { "ok" }

let router = Router::new().route(0, echo);
```

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/router
- Router docs: https://docs.tangle.tools/developers/blueprint-runner/routers
