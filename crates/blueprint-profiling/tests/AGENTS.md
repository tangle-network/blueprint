# tests

## Purpose
Integration tests for the blueprint-profiling crate, verifying that the profiling runner produces correct statistical results for both fast and slow jobs.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `basic_profiling.rs` - Two async tests: `profile_fast_job` verifies that a lightweight computation produces a valid `JobProfile` with correct sample count, monotonic percentile ordering, reasonable memory bounds, and default statefulness flags; `profile_slow_job_detects_delay` confirms that a 40ms sleep is reflected in the average duration measurement.

## Key APIs (no snippets)
- Tests exercise `ProfileRunner::profile_job` with custom `ProfileConfig` settings.
- Assertions validate `JobProfile` fields: `sample_size`, `avg_duration_ms`, `p95_duration_ms`, `p99_duration_ms`, `peak_memory_mb`, `stateful`, `persistent_connections`.

## Relationships
- Depends on `blueprint-profiling` crate (`ProfileConfig`, `ProfileRunner`).
- Complements the inline unit tests in `crates/blueprint-profiling/src/lib.rs` which cover compression, base64 encoding, and percentile calculation.
