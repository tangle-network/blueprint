# ext_traits

## Purpose
Houses extension traits that add ergonomic convenience methods to core types. Currently contains only the `job` submodule, which provides sealed extension traits on `JobCall` and `Parts` for inline extractor usage.

## Contents (one hop)
### Subdirectories
- [x] `job/` - Sealed extension traits (`JobCallExt`, `JobCallPartsExt`) that add `.extract()` and `.extract_with_context()` methods to `JobCall` and `Parts`, delegating to `FromJobCall`/`FromJobCallParts` extractors.

### Files
- `mod.rs` - Re-exports the `job` submodule.

## Key APIs
- `JobCallExt` trait (via `job/`) -- `.extract()`, `.extract_with_context()`, `.extract_parts()`, `.extract_parts_with_context()` on `JobCall`
- `JobCallPartsExt` trait (via `job/`) -- `.extract()`, `.extract_with_ctx()` on `Parts`

## Relationships
- Depends on `crate::JobCall`, `crate::job::call::Parts`, `crate::extract::{FromJobCall, FromJobCallParts}`
- Re-exported through `blueprint-core` and `blueprint-sdk` public API
