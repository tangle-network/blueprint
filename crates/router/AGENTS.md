# router

## Purpose
Job routing layer (`blueprint-router`). Maps incoming `JobId` values to async handler functions using a tower-service-based dispatch model. Fully `no_std` compatible. This is the dispatch core used by `blueprint-runner` to route job calls from producers to the correct handler.

## Contents (one hop)
### Subdirectories
- [x] `docs/` - Documentation: `with_context.md` (guide on using `with_context` for shared state)
- [x] `src/` - Crate source: `routing.rs` (main `Router` type and route registration), `job_id_router.rs` (ID-based dispatch logic), `boxed.rs` (type-erased service wrappers), `future.rs` (router future types), `util.rs` (internal utilities), `test_helpers/` (testing utilities), `tests.rs`

### Files
- `CHANGELOG.md` - Release history
- `Cargo.toml` - Crate manifest (`blueprint-router`); depends on `blueprint-core`, `bytes`, `tower`, `hashbrown`, `futures`, `pin-project-lite`, `document-features`; feature `tracing` (default)
- `README.md` - Overview of job routing, route registration, handler dispatch

## Key APIs (no snippets)
- `Router` (from `routing` module) -- main router type; `.route(id, handler)` registers handlers by job ID; `.with_context(ctx)` attaches shared context; composable via `.merge()`
- `JobId` -- identifier type for routing (supports conversion from `u32`, `u64`, byte arrays, etc.)
- Tower `Service` integration -- router implements tower `Service` for middleware compatibility (timeouts, rate limiting, load shedding)

## Relationships
- Depends on `blueprint-core` (job primitives, `JobCall`, `JobResult`)
- Used by `blueprint-runner` as the core dispatch mechanism
- Re-exported through `blueprint-sdk` as `blueprint_sdk::Router`
- Handlers must implement the `Job` trait (defined in `blueprint-core`)
- Fully `no_std` -- uses `hashbrown` instead of `std::collections`, `alloc` for heap allocation
