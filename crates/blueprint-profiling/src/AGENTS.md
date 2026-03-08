# src

## Purpose
Implements the Blueprint job profiling system, which measures execution time, memory usage, and CPU time across multiple samples to produce statistical profiles. These profiles drive deployment decisions (container vs FaaS) and can be serialized for on-chain storage.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Contains all profiling types and logic: error types, configuration, measurement, statistical computation, profile serialization (JSON, gzip-compressed bytes, base64), and the description-field encoding scheme for on-chain storage.

## Key APIs (no snippets)
- `ProfileConfig` -- controls sample size, warmup runs, and max execution time per sample.
- `ProfileRunner::profile_job(job_fn, config)` -- executes an async job multiple times, collecting `ResourceMeasurement` samples, and returns a `JobProfile` with avg/p95/p99 durations and peak memory.
- `JobProfile` -- serializable struct with duration percentiles, peak memory, statefulness, and connection persistence flags.
- `BlueprintProfiles` -- top-level container mapping job IDs to `JobProfile`s; supports `save_to_file`/`load_from_file`, `to_compressed_bytes`/`from_compressed_bytes` (gzip), `to_base64_string`/`from_base64_string`, and `to_description_field`/`from_description_field` (prefixed with `[PROFILING_DATA_V1]`).
- `has_profiling_data(description)` -- checks whether a string contains the profiling data marker.
- `ProfilingError` -- error enum for execution failures, insufficient samples, and invalid configuration.
- `get_current_memory_bytes()` / `get_cpu_time_us()` -- platform-specific (Unix via `libc::getrusage`) resource measurement helpers.

## Relationships
- Depends on `serde`, `chrono`, `flate2` (gzip), `base64`, `libc` (Unix), and `thiserror`.
- Profiles are consumed by `blueprint-manager` to make container-vs-FaaS deployment decisions.
- The `[PROFILING_DATA_V1]` encoding is used in service metadata description fields on the Tangle chain.
- Tested by `crates/blueprint-profiling/tests/` and inline `#[cfg(test)]` module.
