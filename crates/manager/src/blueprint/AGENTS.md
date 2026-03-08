# blueprint

## Purpose
Blueprint metadata types and platform-aware binary selection logic. Provides the `ActiveBlueprints` registry (mapping blueprint ID to service ID to running `Service` instances) and utilities for selecting the correct binary for the current OS and architecture.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Defines `ActiveBlueprints` type alias (`HashMap<u64, HashMap<u64, Service>>`) and re-exports the `native` submodule.
- `native.rs` - `FilteredBlueprint` struct (blueprint_id, services, sources, name, registration_mode, protocol). `get_blueprint_binary()` function that matches a `BlueprintBinary` to the current OS/arch using normalized OS names (darwin/macos, windows, linux, bsd) and architecture names (amd64->x86_64, arm64->aarch64).

## Key APIs (no snippets)
- `ActiveBlueprints` - the central registry of running blueprint services, keyed by blueprint_id then service_id
- `FilteredBlueprint` - metadata for a blueprint that has been filtered to the operator's registered services
- `get_blueprint_binary(&[BlueprintBinary]) -> Option<&BlueprintBinary>` - selects the platform-matching binary

## Relationships
- `ActiveBlueprints` used by `protocol/` event handlers and `executor/` to track running services
- `FilteredBlueprint` constructed by `protocol/tangle/metadata.rs` and `protocol/eigenlayer/event_handler.rs`
- `get_blueprint_binary` called by `sources/github.rs` and `sources/remote.rs` during binary fetching
- References `sources/types::BlueprintSource` and `sources/types::BlueprintBinary`
