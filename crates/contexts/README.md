# blueprint-contexts

Context extension modules used to compose runtime capabilities into Blueprint job contexts.

## Feature-gated modules

- `tangle`: Tangle client context helpers.
- `eigenlayer`: EigenLayer context helpers.
- `keystore`: signer/keystore context access.
- `instrumented_evm_client`: EVM client instrumentation context.

## When to use

Use when building custom context structs passed into job handlers and you want reusable extension traits/utilities.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/contexts
