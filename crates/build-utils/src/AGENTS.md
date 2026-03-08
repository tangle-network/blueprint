# src

## Purpose
Build-script utilities for compiling Solidity smart contracts using Foundry's `forge` tool. Handles contract compilation, EVM version pinning, and Soldeer dependency management for blueprint projects that include on-chain components.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Provides `build_contracts`, `soldeer_install`, `soldeer_update`, and `find_forge_executable` functions for use in `build.rs` scripts.

## Key APIs (no snippets)
- `build_contracts(contract_dirs)` -- compiles Solidity contracts in the given directories using `forge build` with EVM version pinned to Shanghai and Solidity 0.8.27. Automatically patches `foundry.toml` to inject `evm_version = "shanghai"` if missing.
- `soldeer_install()` -- runs `forge soldeer install` if the `dependencies/` directory is empty or missing.
- `soldeer_update()` -- runs `forge soldeer update -d` to refresh Soldeer dependencies.
- `find_forge_executable()` -- locates the `forge` binary via `which` and panics if Foundry is not installed.

## Relationships
- Depends on `blueprint-std` for filesystem, environment, and process abstractions.
- Used in `build.rs` scripts of crates that include Solidity contracts (e.g., EigenLayer and EVM client crates).
- Requires Foundry (`forge`) to be installed on the build system.
