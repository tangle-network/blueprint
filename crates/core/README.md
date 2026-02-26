# blueprint-core

Core runtime primitives used across the Blueprint SDK.

## What lives here

- Job model: `Job`, `JobId`, `JobCall`, `JobResult`.
- Extractors and metadata primitives for handler inputs.
- Core error and extension traits.

## When to depend directly

Most apps should use `blueprint-sdk`. Depend on `blueprint-core` directly for low-level integrations, custom runtimes, or framework extensions.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/core
- Runner docs: https://docs.tangle.tools/developers/blueprint-runner/introduction
