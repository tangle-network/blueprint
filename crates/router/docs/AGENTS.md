# docs

## Purpose
Educational documentation for the `Router::with_context()` API, explaining how to attach shared application state to routes and clarifying the generic type semantics where `Router<Ctx>` represents a *missing* context requirement.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `with_context.md` - Complete guide (253 lines) covering basic usage, returning routers from functions, type parameter semantics, chaining multiple contexts, compiler error examples, and performance notes.
  - **Key items**: `Router::with_context(ctx)`, `Context<Ctx>` extractor, `Router<()>` (fully provisioned), `BlueprintRunner::builder().router(routes)`
  - **Interactions**: Embedded in Rustdoc via `#[doc = include_str!("../docs/with_context.md")]` on `Router::with_context()` method

## Key APIs (no snippets)
- **`Router::with_context(context: Ctx) -> Router<Ctx2>`** - Attaches context, converts missing-context router to next state
- **`Context<Ctx>` extractor** - Gives handlers access to shared context
- **`BlueprintRunner::builder().router(routes)`** - Accepts only `Router<()>` (fully provisioned)

## Relationships
- **Depends on**: `blueprint-core/extract/context.rs`, `router/src/routing.rs`
- **Used by**: End users building blueprint services; `blueprint-runner` for route acceptance
- **Tested by**: `router/src/tests.rs` (with_context_converts_router_type, context_accessible_via_arc_state)

## Files (detailed)

### `with_context.md`
- **Role**: Narrative guide with 15+ Rust code examples explaining the non-intuitive generic parameter semantics.
- **Key items**: 7 sections covering basic usage through performance optimization
- **Interactions**: Included inline in API documentation via `include_str!` macro
- **Knobs / invariants**: Core teaching: `Router<Ctx>` means "missing Ctx", not "holds Ctx"

## End-to-end flow
1. Define app context struct
2. Create routes returning `Router<AppContext>` (missing context)
3. Call `.with_context(context_instance)` -> `Router<()>` (fully provisioned)
4. Pass `Router<()>` to `BlueprintRunner`
5. Handlers access context via `Context<AppContext>` extractor

## Notes
- Addresses non-intuitive semantics that commonly confuse new users
- Self-contained; sole substantive docs file in this directory
