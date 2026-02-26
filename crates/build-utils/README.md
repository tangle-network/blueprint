# blueprint-build-utils

Build-time helpers for Blueprint projects that compile smart contracts and manage Foundry dependencies.

## When to use

- In `build.rs` or setup scripts for blueprint repos with Solidity contracts.
- To standardize `forge build` invocation across workspace crates.

## Key API surface

- `build_contracts(...)`: compile configured contract directories.
- `soldeer_install()` / `soldeer_update()`: dependency bootstrap/update helpers.
- `find_forge_executable()`: resolves local Foundry binary.

## Notes

- Utilities assume Foundry tooling is installed and available.
- Contract builds pin EVM/toolchain settings for consistency.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/build-utils
