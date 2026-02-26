# blueprint-std

Shared std/core/alloc exports and utilities used across Blueprint crates.

## What it includes

- Re-exports for `alloc`, `core`, and `std` (feature-aware).
- Convenience modules for IO/error and synchronization helpers.
- Shared random/perf-trace helpers used by other workspace crates.

## When to use

Use inside workspace crates when you want consistent std/no-std compatible imports.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/std
