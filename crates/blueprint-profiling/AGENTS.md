# blueprint-profiling

## Purpose
Crate `blueprint-profiling`: Automated profiling system for Blueprint jobs, inspired by Substrate's benchmarking framework. Measures execution time (avg, p95, p99), memory usage, and CPU time across configurable sample runs. Produces `BlueprintProfiles` that can be serialized to JSON files, compressed with gzip for on-chain storage, or encoded as base64 for embedding in service metadata fields.

## Contents (one hop)
### Subdirectories
- [x] `examples/` - Usage examples: `basic_usage.rs`, `memory_profiling.rs`, `simple_profiling.rs`.
- [x] `src/` - Single `lib.rs` containing all profiling types, runner, statistics computation, compression, and serialization logic.
- [x] `tests/` - Basic profiling integration test (`basic_profiling.rs`).

### Files
- `Cargo.toml` - Crate manifest (`blueprint-profiling`). Deps: `serde`/`serde_json`, `chrono`, `flate2` (gzip), `base64`, `libc` (unix). No workspace blueprint dependencies.
- `README.md` - Crate documentation.

## Key APIs (no snippets)
- `ProfileRunner::profile_job(job_fn, config)` -- executes a job function multiple times with warmup, collects `ResourceMeasurement` samples, returns a `JobProfile` with statistical summary.
- `ProfileConfig` -- controls sample size, warmup runs, and max execution time.
- `JobProfile` -- per-job statistics: avg/p95/p99 duration, peak memory, statefulness, persistent connections.
- `BlueprintProfiles` -- collection of job profiles for a blueprint with serialization methods.
- `BlueprintProfiles::to_compressed_bytes()` / `from_compressed_bytes()` -- gzip round-trip for on-chain storage.
- `BlueprintProfiles::to_base64_string()` / `from_base64_string()` -- base64-encoded compressed format for `ServiceMetadata.profiling_data`.
- `BlueprintProfiles::to_description_field()` / `from_description_field()` -- temporary encoding with `[PROFILING_DATA_V1]` prefix marker.
- `has_profiling_data(description)` -- checks if a description string contains profiling data.

## Relationships
- Standalone crate with no workspace blueprint dependencies (only `serde`, `chrono`, `flate2`, `base64`).
- Output consumed by the Blueprint Manager (`blueprint-manager`) for deployment decisions (FaaS vs. long-running).
- Output consumed by `blueprint-client-tangle` for on-chain metadata storage.
- Complementary to `blueprint-benchmarking` which provides lower-level runtime measurement.
