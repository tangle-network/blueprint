# evm

## Purpose
Provides the `load_abi` proc-macro implementation that reads a JSON ABI file at compile time and expands it into a `const &str` containing the ABI array.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Parses `load_abi!(IDENT, "path/to/file.json")` invocations. Resolves the file path relative to `CARGO_MANIFEST_DIR`, reads the JSON, extracts the `"abi"` field, and generates a `const IDENT: &str = "<abi json>";` declaration. Produces a compile error if the file does not exist.

## Key APIs
- `load_abi(input: TokenStream) -> TokenStream` - proc-macro implementation called from the parent crate's `#[proc_macro]` entry point

## Relationships
- Called by the parent `blueprint-macros` crate which exposes it as a public proc-macro
- Reads Foundry/Hardhat-style JSON artifacts containing an `"abi"` key
