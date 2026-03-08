# service

## Purpose
Service-level integrations for the pricing engine, providing blockchain event monitoring for on-chain pricing updates and an RPC server for exposing pricing data to clients.

## Contents (one hop)
### Subdirectories
- [x] `blockchain/` - Blockchain event listener for pricing-related on-chain events, including EVM log monitoring.
- [x] `rpc/` - RPC server exposing pricing engine data to external consumers.

### Files
- `mod.rs` - Module root that declares and re-exports the `blockchain` and `rpc` submodules.

## Key APIs (no snippets)
- See subdirectory AGENTS.md files for specific APIs.

## Relationships
- Parent module in the pricing engine crate, connecting blockchain event sources and RPC exposure.
- Subdirectories depend on other pricing engine modules (`benchmark`, `cloud`, `crate::types`, `crate::error`).
