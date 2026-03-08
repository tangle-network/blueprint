# extract

## Purpose
Extractor framework for decomposing `JobCall` values into typed components. Mirrors the axum extractor pattern: job handlers declare extractors as function arguments, and the framework automatically pulls data from the job call's parts (metadata, extensions, job ID) or body. Extractors that only need metadata implement `FromJobCallParts`; those that consume the body implement `FromJobCall`.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Defines the two core extractor traits (`FromJobCall`, `FromJobCallParts`), private marker types (`ViaParts`, `ViaJobCall`), and the blanket impl that lets any `FromJobCallParts` extractor also work as a `FromJobCall` extractor. Re-exports all public extractors.
- `context.rs` - `Context<S>` extractor that pulls application context (or sub-context via `FromRef`) from the router's shared state. Extensively documented with usage patterns for shared mutable state.
- `extension.rs` - `Extension<T>` extractor that retrieves a typed value from the `JobCall`'s `Extensions` map. Also implements `OptionalFromJobCallParts` for `Option<Extension<T>>`.
- `from_ref.rs` - `FromRef<T>` trait for reference-to-value conversions, enabling sub-context extraction from a parent context type. Blanket impl for `T: Clone`.
- `job_call_parts.rs` - Built-in `FromJobCall`/`FromJobCallParts` impls for common types: `JobCall` itself, `Parts`, `MetadataMap`, `Bytes`, `BytesMut`, `String`.
- `job_id.rs` - `JobId<T>` extractor that pulls the job ID from call parts and converts it via `TryFrom<crate::JobId>`. Works for all numeric primitives and custom types.
- `metadata.rs` - `Metadata` extractor that clones the entire `MetadataMap` from the job call parts.
- `option.rs` - `OptionalFromJobCall` and `OptionalFromJobCallParts` traits that customize `Option<T>` extraction behavior, allowing extractors to return `None` instead of rejecting.
- `rejection.rs` - Rejection types for extraction failures (e.g., `InvalidUtf8`). Uses the `__define_rejection!` macro.
- `tuple.rs` - Macro-generated `FromJobCall`/`FromJobCallParts` impls for tuples of extractors (up to N elements). All-but-last elements use `FromJobCallParts`; the last may use `FromJobCall` to consume the body.

## Key APIs
- `FromJobCall<Ctx, M>` trait -- extract a value by consuming the entire `JobCall`
- `FromJobCallParts<Ctx>` trait -- extract a value from `&mut Parts` without consuming the body
- `Context<S>` struct -- extract application context or sub-context
- `Extension<T>` struct -- extract a typed value from call extensions
- `FromRef<T>` trait -- convert a reference to the parent context into a child context
- `JobId<T>` struct -- extract and convert the job identifier
- `OptionalFromJobCall` / `OptionalFromJobCallParts` traits -- optional extraction (returns `Option`)

## Relationships
- Depends on `crate::job::call::{JobCall, Parts}`, `crate::job::result::IntoJobResult`, `crate::metadata::MetadataMap`, `crate::extensions::Extensions`
- Used by `crate::job::Job` trait impl (macro-generated handler impls decompose `JobCall` via these extractors)
- Extension traits in `crate::ext_traits::job` add convenience `.extract()` methods that delegate to these traits
- Re-exported through `blueprint-core` and `blueprint-sdk` public API
