# job

## Purpose
Defines the core job abstraction: the `Job` trait, `JobCall` request type, `JobResult` response type, and the `JobService` adapter that bridges jobs into Tower `Service`s. This is the central execution primitive of the blueprint SDK -- async functions that accept extractors as arguments are automatically implemented as `Job`s via macro-generated trait impls.

## Contents (one hop)
### Subdirectories
- [x] `result/` - `JobResult` enum (Ok with parts+body or Err), `IntoJobResult` trait for converting handler return values into results, `IntoJobResultParts` trait for attaching metadata to results, and the `Void` sentinel type for jobs that produce no output.

### Files
- `mod.rs` - `Job<T, Ctx>` trait with `call()` and `layer()` methods. Macro-generated impls for async functions of 0..N extractor arguments. `Layered` wrapper for applying Tower layers. `JobWithoutContextExt` for stateless handlers. Also implements `Job` for any `T: IntoJobResult` (static data as a handler).
- `call.rs` - `JobCall<T>` struct (header `Parts` + body `T`, defaults to `Bytes`). `Parts` struct holding `JobId`, `MetadataMap`, and `Extensions`. Builder methods for constructing calls from parts.
- `future.rs` - Future types for job execution: `IntoServiceFuture` (wraps a job's future into a service response) and `LayeredFuture` (wraps a layered service oneshot).
- `id.rs` - `JobId` type: a 256-bit identifier (`[u64; 4]`). Implements `From` for all numeric primitives (stored in low limb), `[u8; 32]` (transmute), `&str`/`String`/`Vec<u8>` (keccak-256 hash), and `()` (zero).
- `service.rs` - `JobService<J, T, Ctx>` adapter that wraps a `Job` + context into a Tower `Service<JobCall>`. Always ready (poll_ready returns `Ok`).

## Key APIs
- `Job<T, Ctx>` trait -- `call(self, call: JobCall, ctx: Ctx) -> Self::Future`
- `JobCall<T>` struct -- request type with `into_parts()`, `from_parts()`, `map()`, accessor methods
- `Parts` struct -- job call metadata (job_id, metadata map, extensions)
- `JobId` struct -- 256-bit job identifier with broad `From` impls
- `JobService<J, T, Ctx>` struct -- Tower Service adapter
- `JobWithoutContextExt` trait -- `into_service()` for stateless jobs
- `Layered<L, J, T, Ctx>` struct -- applies a Tower `Layer` to a job

## Relationships
- Depends on `crate::extract::{FromJobCall, FromJobCallParts}` for the macro-generated `Job` impls
- Depends on `crate::job::result::IntoJobResult` for converting handler returns
- Used by `blueprint-router` for job dispatch and `blueprint-runner` for execution
- `JobCall` and `JobResult` are the primary input/output types flowing through the entire pipeline
- Re-exported as top-level types in `blueprint-core` and `blueprint-sdk`
