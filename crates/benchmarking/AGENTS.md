# benchmarking

## Purpose
Crate `blueprint-benchmarking`: Provides a benchmarking harness for measuring Blueprint job execution time, CPU cores, and RAM usage. Built around a pluggable async runtime abstraction with a built-in Tokio implementation. Produces human-readable `BenchmarkSummary` reports with auto-scaled memory units.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Single `lib.rs` defining the `Runtime` trait, `TokioRuntime` implementation, `Bencher` harness, and `BenchmarkSummary` result type with `Display` formatting.

### Files
- `CHANGELOG.md` - Version history.
- `Cargo.toml` - Crate manifest (`blueprint-benchmarking`). Deps: `blueprint-std`, `tokio`, `sysinfo` (optional, behind `std` feature). Features: `std` (default).
- `README.md` - Crate documentation.

## Key APIs (no snippets)
- `Runtime` trait -- abstraction for executing futures to completion (`block_on`).
- `TokioRuntime` -- `Runtime` impl that delegates to `tokio::runtime::Handle::current()`.
- `Bencher::new(threads, runtime)` -- creates a harness with a specified core count and runtime.
- `Bencher::block_on(future)` -- runs an async workload on the configured runtime.
- `Bencher::stop(name, job_id)` -- ends the benchmark, captures elapsed time and process memory via `sysinfo`, returns a `BenchmarkSummary`.
- `BenchmarkSummary` -- contains name, job ID, elapsed duration, core count, and RAM usage; implements `Display` with auto-scaled units (B/KB/MB/GB).

## Relationships
- Depends on `blueprint-std` for `Future`, `Instant`, `Duration`, and `fmt` abstractions.
- Depends on `sysinfo` (behind the `std` feature) for process memory measurement.
- Used by blueprint authors to benchmark job execution as part of profiling workflows.
- Complementary to `blueprint-profiling` which focuses on statistical profiling and on-chain storage.
