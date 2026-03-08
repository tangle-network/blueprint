# job

## Purpose
Extension traits that add convenience extractor methods to `JobCall` and `Parts`, following a sealed-trait pattern so they cannot be implemented outside this crate.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports `call` and `parts` modules.
- `call.rs` - `JobCallExt` sealed trait on `JobCall` with `extract`, `extract_with_context`, `extract_parts`, `extract_parts_with_context`. Delegates to `FromJobCall` / `FromJobCallParts` extractors.
- `parts.rs` - `JobCallPartsExt` sealed trait on `Parts` with `extract` and `extract_with_ctx`. Delegates to `FromJobCallParts` extractors.

## Key APIs
- `JobCallExt` trait (sealed, implemented for `JobCall`) -- apply extractors that consume or borrow a job call
- `JobCallPartsExt` trait (sealed, implemented for `Parts`) -- apply extractors to call metadata without consuming the body

## Relationships
- Depends on `crate::JobCall`, `crate::job::call::Parts`, `crate::extract::{FromJobCall, FromJobCallParts}`
- Re-exported through `blueprint-core` and `blueprint-sdk` public API
