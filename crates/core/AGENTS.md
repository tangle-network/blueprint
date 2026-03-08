# core

## Purpose
Crate `blueprint-core`: Foundation crate defining the job system primitives that the entire Blueprint SDK builds upon. Provides the `Job` trait, `JobCall` request type (modeled after HTTP's `Request`), `JobResult` response type, extraction traits (`FromJobCall`, `FromJobCallParts`), extension traits for job call manipulation, and feature-gated tracing macros. Designed as a `no_std` crate with `alloc` support.

## Contents (one hop)
### Subdirectories
- [x] `docs/` - Developer documentation: `jobs_intro.md` (job system introduction), `debugging_job_type_errors.md` (troubleshooting guide for job type errors).
- [x] `src/` - Core modules: `job/` (Job trait, JobCall, JobResult, JobId), `extract/` (FromJobCall, FromJobCallParts extraction traits), `ext_traits/` (extension traits for job call parts), `error.rs`, `extensions.rs` (type-map extensions), `metadata.rs`, `macros.rs` (internal helper macros).

### Files
- `CHANGELOG.md` - Version history.
- `Cargo.toml` - Crate manifest (`blueprint-core`). Deps: `bytes`, `futures-util`, `pin-project-lite`, `tower`, `hashbrown`, `tiny-keccak`, `tracing`. Features: `std`, `tracing`.
- `README.md` - Crate documentation.

## Key APIs (no snippets)
- `Job` trait -- defines a blueprint job handler that processes `JobCall` requests.
- `JobCall` -- the request type for job invocations, containing headers, extensions, and a body (similar to HTTP Request).
- `JobResult` -- the response type for job completions.
- `JobId` -- unique identifier for a job type.
- `FromJobCall` / `FromJobCallParts` -- extraction traits for pulling typed data from job calls (similar to Axum extractors).
- `IntoJobResult` / `IntoJobResultParts` -- conversion traits for producing job results.
- `JobCallExt` / `JobCallPartsExt` -- extension traits for manipulating job call components.
- `Error` -- core error type.
- Tracing macros (`info!`, `warn!`, `error!`, `debug!`, `trace!`) -- feature-gated wrappers around `tracing` that compile to no-ops when the `tracing` feature is disabled.

## Relationships
- Foundation dependency for nearly every other crate in the workspace.
- Depended on by `blueprint-runner`, `blueprint-router`, `blueprint-clients`, `blueprint-auth`, `blueprint-faas`, `blueprint-remote-providers`, and more.
- Uses `tower` for service composition patterns.
- `no_std` compatible with `alloc`, making it usable in constrained environments.
