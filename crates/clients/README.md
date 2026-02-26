# blueprint-clients

Meta-crate that re-exports Blueprint client crates.

## When to use

- You want one dependency that exposes Tangle, EVM, and EigenLayer client surfaces.
- You prefer feature-gated client composition instead of importing each client crate directly.

## Re-exports

- `blueprint-client-core`
- `blueprint-client-evm`
- `blueprint-client-tangle`
- `blueprint-client-eigenlayer`

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/clients
