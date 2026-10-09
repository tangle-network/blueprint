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

## Write failures and limits

Mutations publish the staged map to the current instance only after its JSON
file has been written and renamed successfully. A serialization or temporary-file
I/O error leaves the prior map visible; missing-key `remove` and `update` do not
write. Values should clone independently: side effects through shared interior
state or an update closure's external state cannot be rolled back.

This is failure atomicity for one database instance, not crash durability:
writes do not sync the file or parent directory to stable storage. Separate
instances/processes sharing a path are not coordinated. Existing invalid-JSON
opening behavior is unchanged. This store alone is not a durable lifecycle or
credential-incarnation fence.
