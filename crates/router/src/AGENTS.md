# src

## Purpose
Implementation of the `blueprint-router` crate, providing a `no_std`-compatible job routing system inspired by axum's `Router`. Routes incoming `JobCall` requests to registered handler functions based on `JobId`, with support for Tower middleware layers, context injection, and concurrent job execution.

## Contents (one hop)
### Subdirectories
- [x] `test_helpers/` - Test utilities including logging setup for router tests

### Files
- `lib.rs` - Crate root (`no_std`); declares modules, re-exports `routing::*` (the `Router` type and related items)
- `routing.rs` - `Router<Ctx>` struct: `route()` to register jobs by ID, `layer()` to apply Tower middleware, `with_context()` to inject shared context, `merge()` to combine routers; implements `tower::Service<JobCall>` for request dispatch
- `job_id_router.rs` - Internal `JobIdRouter<Ctx>` that maps `JobId` to `Handler` variants (concrete `Route` or type-erased `BoxedIntoRoute`); performs lookup and concurrent execution via `FuturesUnordered`
- `boxed.rs` - Type-erased route boxing for heterogeneous handler storage
- `future.rs` - `Route` and `RouteFuture` types wrapping Tower services for async job execution
- `util.rs` - Internal utilities including type downcasting helpers
- `tests.rs` - Unit tests covering basic routing, unknown job IDs, multiple routes, context injection, layer application, router merging, and concurrent dispatch

## Key APIs (no snippets)
- `Router::new()` - Create an empty router
- `Router::route(job_id, handler)` - Register a job handler for a given ID
- `Router::layer(layer)` - Apply a Tower `Layer` to all routes
- `Router::with_context(ctx)` - Inject shared context, converting `Router<Ctx>` to `Router<()>`
- `Router::merge(other)` - Combine two routers
- `Router` implements `tower::Service<JobCall>` - Standard Tower service interface for dispatching job calls

## Relationships
- Depends on `blueprint-core` for `Job`, `JobCall`, `JobId`, `JobResult`, `IntoJobResult`
- Depends on `tower` for `Service` and `Layer` traits
- Used by `blueprint-runner` as the central dispatch mechanism for job execution
- Used by all testing utilities to construct test routers
