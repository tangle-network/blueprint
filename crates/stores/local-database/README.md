# blueprint-store-local-database

Local JSON-backed key/value storage provider.

## What it provides

- `LocalDatabase<T>` typed key/value API.
- Atomic flush behavior (temp-file + rename) for safer writes.
- Common operations: `set`, `get`, `remove`, `update`, `replace`, `entries`.

## When to use

Use for simple local persistence needs in development and lightweight runtime state.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/stores/local-database
