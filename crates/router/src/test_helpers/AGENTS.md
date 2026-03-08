# test_helpers

## Purpose
Shared test utilities for the `blueprint-router` crate. Provides trait-bound assertions (`Send`/`Sync`), a non-`Send+Sync` witness type, and a log-initialization helper for tests.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `assert_send`, `assert_sync` compile-time checks; `NotSendSync` witness struct; `setup_log` tracing-subscriber initializer

## Key APIs
- `assert_send::<T>()` / `assert_sync::<T>()` - compile-time marker functions used to verify Router types satisfy `Send`/`Sync` bounds
- `NotSendSync` - raw-pointer wrapper that is intentionally neither `Send` nor `Sync`; used in negative-bound tests
- `setup_log()` - initializes `tracing_subscriber` with INFO default and `RUST_LOG` env-filter for test output

## Relationships
- Used only within `crate::tests` (via `pub(crate)`)
- Depends on `blueprint_core::__private::tracing` and `tracing_subscriber`
