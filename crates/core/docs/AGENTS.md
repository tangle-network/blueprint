# docs

## Purpose
Developer documentation for the Blueprint SDK job system, included in rustdoc output via `#[doc = include_str!(...)]`. Covers job fundamentals and troubleshooting guidance.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `debugging_job_type_errors.md` - Explains why functions may fail to satisfy the `Job` trait bound, lists the exact requirements (async, up to 16 `Send` arguments, `FromJobCallParts`/`FromJobCall` extractors, `IntoJobResult` return), and recommends the `#[debug_job]` proc-macro from `blueprint-macros` for better error messages.
- `jobs_intro.md` - Brief introduction explaining that a "job" is an async function taking zero or more extractors and returning something convertible into a job result; jobs contain application logic and blueprints route between them.

## Key APIs (no snippets)
- References `Job` trait, `FromJobCallParts`, `FromJobCall`, `IntoJobResult` traits.
- References `#[debug_job]` attribute macro from `blueprint-macros`.

## Relationships
- Included by `crates/core/` source files (likely `crates/core/src/job/`) via `include_str!` for inline rustdoc.
- Documents the extractor and job result patterns defined in `crates/core/src/extract/` and `crates/core/src/job/`.
