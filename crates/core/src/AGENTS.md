# src

## Purpose
Source directory for the `blueprint-core` crate, the foundational `no_std`-compatible crate providing the job system primitives for the Blueprint SDK. Defines the core abstractions for job calls, job results, extractors, metadata, extensions, and the `Job` trait that all blueprint job handlers implement.

## Contents (one hop)
### Subdirectories
- [x] `ext_traits/` - Extension traits that add convenience methods (e.g., `.extract()`) to `JobCall` and `JobCallParts`. Contains a `job/` subdirectory with job-specific extension trait implementations.
- [x] `extract/` - Extractor framework for decomposing `JobCall` values into typed components. Mirrors the axum extractor pattern with `FromJobCall` and `FromJobCallParts` traits, `Context<S>`, `Extension<T>`, `JobId<T>`, `Metadata`, and tuple extractors. Also defines rejection types and optional extraction traits.
- [x] `job/` - Job system types including `JobCall` (incoming job request with parts and body), `JobResult` (job response), `JobId`, `Job` trait, job futures, and the `tower::Service` adapter. Contains a `result/` subdirectory.

### Files
- `error.rs` - `Error` type wrapping a cloneable trait object (`CloneableError`) and `BoxError` alias for type-erased errors
- `extensions.rs` - `Extensions` type map for storing typed protocol extensions on `JobCall` and `JobResult`; uses `TypeId`-keyed `HashMap` with a no-op hasher for zero-cost lookups, supports `Clone`
- `lib.rs` - Crate root (`no_std`); declares all modules, re-exports primary types (`JobCall`, `JobResult`, `IntoJobResult`, `Job`, `JobId`, `FromJobCall`, `FromJobCallParts`, `Bytes`, `Error`), and defines feature-gated tracing macros (`info!`, `warn!`, `error!`, `debug!`, `trace!`)
- `macros.rs` - Internal macros: `__log_rejection!` for tracing extraction failures, `all_the_tuples!` for generating tuple impls (up to 16 elements), `__impl_deref!`/`__impl_from!` for newtype boilerplate, `opaque_future!` for type-erasing futures, and `__define_rejection!`/`__composite_rejection!` for defining rejection error types
- `metadata.rs` - `MetadataMap<T>` (BTreeMap-backed typed metadata map) and `MetadataValue` (byte-based metadata value with sensitivity marking, string/number conversions, and Debug redaction for sensitive values)

## Key APIs
- `JobCall` -- incoming job request with metadata, extensions, job ID, and body
- `JobResult` -- job response (Ok/Err variants)
- `Job` trait -- implemented by job handler functions via macro-generated impls
- `FromJobCall<Ctx, M>` / `FromJobCallParts<Ctx>` -- extractor traits for typed decomposition of job calls
- `Extensions` -- type map for attaching arbitrary typed data to job calls/results
- `MetadataMap<T>` / `MetadataValue` -- typed metadata with sensitivity support
- `Error` / `BoxError` -- cloneable error wrapper for the job system
- Tracing macros (`info!`, `warn!`, etc.) -- feature-gated, compile to no-ops when tracing is disabled

## Relationships
- This is the lowest-level crate in the Blueprint SDK; depended on by nearly every other crate
- `blueprint-router` and `blueprint-runner` use the `Job` trait and `JobCall`/`JobResult` types for job dispatch
- `blueprint-sdk` re-exports all public types from this crate
- The extractor pattern is modeled after axum's extractor system
- `no_std` compatible; uses `alloc` for heap allocations
