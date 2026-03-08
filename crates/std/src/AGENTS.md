# src

## Purpose
Provides a standard-library compatibility layer that works in both `std` and `no_std` environments. When `std` is enabled, it re-exports `std::*` directly. When `std` is disabled, it re-exports `core` and `alloc` types to provide the same API surface. Also provides cross-environment utilities: a cryptographic RNG wrapper, performance tracing macros, parallel iteration macros, and `num_traits` re-exports.

## Contents (one hop)
### Subdirectories
- [x] `io/` - No-std-compatible replacements for `std::io` traits (`Read`, `Write`, `Cursor`, `Error`, `ErrorKind`); only compiled when `std` feature is disabled

### Files
- `lib.rs` - Crate root; conditionally re-exports `std::*` or `core`/`alloc` equivalents; defines `cfg_iter!`, `cfg_iter_mut!`, `cfg_into_iter!`, `cfg_chunks!`, `cfg_chunks_mut!` macros for optional parallel iteration via rayon; re-exports `num_traits::{One, Zero}`, `rand_helper`, and `perf_trace`
- `error.rs` - Minimal `Error` trait for `no_std` with `source()` method and `From` conversions to `Box<dyn Error>`
- `perf_trace.rs` - Wall-clock performance tracing via `start_timer!`, `end_timer!`, `add_to_trace!` macros; prints nested, indented timing output when `print-trace` feature is enabled; compiles to no-ops otherwise
- `rand_helper.rs` - `BlueprintRng` struct wrapping `OsRng` (std) or `StdRng` (no_std); `UniformRand` trait for generic random sampling; `test_rng()` for deterministic test randomness

## Key APIs
- `cfg_iter!`, `cfg_iter_mut!`, `cfg_into_iter!`, `cfg_chunks!`, `cfg_chunks_mut!`: macros toggling between rayon parallel and sequential iteration
- `BlueprintRng`: cross-environment CSRNG implementing `RngCore + CryptoRng`
- `UniformRand` trait: `rand<R: Rng>(rng) -> Self`
- `test_rng()`: deterministic RNG for reproducible tests
- `start_timer!` / `end_timer!` / `add_to_trace!`: nestable performance tracing macros
- Re-exports `One`, `Zero` from `num_traits`

## Relationships
- Foundation crate used by nearly every other crate in the workspace
- Depends on `rand`, `num_traits`, optionally `rayon` (parallel feature) and `colored` (print-trace feature)
- The `io/` submodule is only active in no_std builds; std builds use `std::io` directly
