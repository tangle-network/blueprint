# build-utils

## Purpose
Crate `blueprint-build-utils`: Build script utilities for compiling Solidity smart contracts using Foundry's `forge` tool and managing Soldeer dependencies. Intended for use in `build.rs` scripts of crates that depend on compiled EVM contracts.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Single `lib.rs` with contract build and dependency management functions.

### Files
- `CHANGELOG.md` - Version history.
- `Cargo.toml` - Crate manifest (`blueprint-build-utils`). Sole dependency: `blueprint-std` (std feature).
- `README.md` - Crate documentation.

## Key APIs (no snippets)
- `build_contracts(contract_dirs)` -- compiles Solidity contracts in specified directories using `forge build` with Shanghai EVM version and Solc 0.8.27. Automatically pins `evm_version` in `foundry.toml` files for consistency.
- `soldeer_install()` -- runs `forge soldeer install` to populate the `dependencies/` directory if empty.
- `soldeer_update()` -- runs `forge soldeer update -d` to refresh Soldeer dependencies.
- `find_forge_executable()` -- locates the `forge` binary on the system PATH.

## Relationships
- Depends on `blueprint-std` for filesystem and process utilities.
- Used by other crates' `build.rs` scripts (e.g., EVM-related crates, testing utilities) to compile Solidity contracts at build time.
- Requires Foundry (`forge`) to be installed on the build system.
