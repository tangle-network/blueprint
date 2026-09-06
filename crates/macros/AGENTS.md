# Blueprint macros

Read [src/lib.rs](src/lib.rs) for the UI-test runner and supported test filters.
The compile-pass and compile-fail suites require nightly Rust; a stable run can omit them.
Use the toolchain required by the suite before claiming macro diagnostics are verified.

Keep `.stderr` expectations aligned with their `.rs` fixtures, including source spans.
Review changed diagnostics before accepting regenerated expectations.
Preserve both accepted programs and negative cases when changing macro expansion.
Use path-only workspace dev-dependencies where required by the root publishing rules.
