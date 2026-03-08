# sources

## Purpose
Blueprint source fetching and spawning abstraction. Defines the `BlueprintSourceHandler` trait for fetching blueprint binaries/images from various origins (GitHub releases, remote URLs/IPFS, container registries, local cargo builds) and spawning them as `Service` instances. Also defines the environment variables and CLI arguments passed to child blueprint processes.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `BlueprintSourceHandler` trait with `fetch()` (download/build binary) and `spawn()` (create `Service`). `DynBlueprintSource` dynamic dispatch wrapper (generated via `dynosaur`). `BlueprintArgs` struct encoding CLI arguments (test_mode, pretty, verbose, dry_run, extra_args) with `encode()` for command-line serialization. `BlueprintEnvVars` struct encoding environment variables (RPC endpoints, keystore, data_dir, blueprint/service IDs, protocol, chain, contract addresses, bridge socket, registration mode) with `encode()` for env-var serialization. Archive safety helpers: `is_safe_archive_path()` rejects traversal/absolute paths, `unpack_archive_safely()` rejects symlinks/hardlinks.
- `types.rs` - Data types for blueprint sources: `TestFetcher` (cargo package/bin/base_path), `GithubFetcher` (owner/repo/tag/binaries), `ImageRegistryFetcher` (registry/image/tag), `RemoteFetcher` (dist_url/archive_url/binaries), `BlueprintBinary` (arch/os/name/sha256/blake3), `BlueprintSource` enum.
- `github.rs` - `GithubBinaryFetcher` implementing `BlueprintSourceHandler`. Downloads from GitHub releases using cargo-dist manifest (`dist-manifest.json`), extracts tar.xz archives, verifies GitHub attestations via `gh attestation verify`, and resolves platform-appropriate binaries.
- `remote.rs` - `RemoteBinaryFetcher` implementing `BlueprintSourceHandler`. Downloads from arbitrary URLs (including IPFS via gateway), enforces size limits (`MAX_ARCHIVE_BYTES`, default 512 MiB), verifies SHA-256 and optional BLAKE3 digests, retries with exponential backoff, and caches downloads keyed by URL hash.
- `container.rs` - `ContainerSource` implementing `BlueprintSourceHandler`. Pulls Docker images via `docker pull` and spawns container-based services via `Service::new_container()`.
- `testing.rs` - `TestSourceFetcher` implementing `BlueprintSourceHandler`. Builds blueprint binaries from source via `cargo build` in the git repository, used for local development and testing.

## Key APIs (no snippets)
- `BlueprintSourceHandler` trait - `fetch()` and `spawn()` for blueprint acquisition and execution
- `BlueprintArgs` - CLI argument encoding for child processes
- `BlueprintEnvVars` - environment variable encoding for child processes (RPC URLs, keystore, contracts, etc.)
- `BlueprintSource` enum - discriminated union of all source types

## Relationships
- `BlueprintSourceHandler::spawn()` creates `Service` instances via `rt/service.rs`
- Source types constructed from on-chain metadata by `protocol/tangle/metadata.rs`
- `BlueprintArgs` and `BlueprintEnvVars` constructed in `protocol/` event handlers using `config::BlueprintManagerConfig`
- `github.rs` and `remote.rs` use `blueprint/native::get_blueprint_binary()` for platform matching
- Archive safety used by both `github.rs` and `remote.rs` during extraction
