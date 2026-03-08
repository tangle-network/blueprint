# std

## Purpose
Shared std/core/alloc facade for the Blueprint workspace. Provides consistent imports that work in both `std` and `no_std` environments, plus utility macros for optional parallelism via Rayon, random number helpers, and performance tracing.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Crate source: conditional re-exports of `alloc`/`core`/`std`, `io/` shim for no_std, `rand_helper.rs`, `perf_trace.rs`, parallel-iterator macros

### Files
- `Cargo.toml` - Crate manifest (`blueprint-std`); features: `std` (default), `parallel` (Rayon), `print-trace`, `getrandom`
- `CHANGELOG.md` - Release history
- `README.md` - Brief description of re-exports and usage guidance

## Key APIs (no snippets)
- Conditional re-exports: `alloc::*` / `core::*` (no_std) or `std::*` (std), with explicit `fmt`, `borrow`, `slice`, `str`, `sync`, `io` modules
- `cfg_iter!`, `cfg_iter_mut!`, `cfg_into_iter!`, `cfg_chunks!`, `cfg_chunks_mut!` - Macros that switch between sequential and Rayon parallel iterators based on the `parallel` feature
- `One`, `Zero` - Re-exports from `num-traits`
- `rand_helper` module - Random number generation utilities
- `perf_trace` module - Performance tracing utilities (gated behind `print-trace`)

## Relationships
- Depended on by nearly every workspace crate as the standard library abstraction layer
- Dependencies: `rand`, `num-traits`, `thiserror`, `colored`; optional `rayon` (feature `parallel`)
- Re-exported by `blueprint-sdk` as `std`

## Notes
- `no_std` compatible: when `std` feature is off, uses `alloc` and `core` re-exports with a custom `io` shim
- The `parallel` feature requires `std`
