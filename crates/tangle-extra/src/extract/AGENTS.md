# extract

## Purpose
Tangle-specific extractors for job call metadata. Provides typed wrappers that pull values like call ID, service ID, block number, caller address, and timestamps from `JobCall` metadata headers. Also provides `TangleArg<T>` / `TangleResult<T>` for ABI-encoded and compact-binary input/output handling.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - All extractor types, compact binary decoding helpers, ABI format heuristics, `TangleArg<T>` (dual-format input decoder), `TangleResult<T>` (ABI-encoded output wrapper), and tests
- `block_hash.rs` - (empty/re-exported) `BlockHash` extractor defined in mod.rs
- `block_number.rs` - (empty/re-exported) `BlockNumber` extractor defined in mod.rs
- `call_id.rs` - (empty/re-exported) `CallId` extractor defined in mod.rs
- `caller.rs` - (empty/re-exported) `Caller` extractor defined in mod.rs
- `service_id.rs` - (empty/re-exported) `ServiceId` extractor defined in mod.rs
- `timestamp.rs` - (empty/re-exported) `Timestamp` extractor defined in mod.rs

## Key APIs
- `CallId(u64)` - extracts from `X-TANGLE-CALL-ID` metadata; implements `FromJobCallParts` and `OptionalFromJobCallParts`
- `ServiceId(u64)` - extracts from `X-TANGLE-SERVICE-ID`
- `JobIndex(u8)` - extracts from `X-TANGLE-JOB-INDEX`
- `BlockNumber(u64)` - extracts from `X-TANGLE-BLOCK-NUMBER`
- `BlockHash([u8; 32])` - extracts from `X-TANGLE-BLOCK-HASH`
- `Timestamp(u64)` - extracts from `X-TANGLE-TIMESTAMP`
- `Caller([u8; 20])` - extracts from `X-TANGLE-CALLER`; has `as_address()` for alloy `Address`
- `TangleArg<T>` - `FromJobCall` extractor that auto-detects compact binary vs ABI encoding; supports both `--params-file` (compact) and `--payload-hex` (ABI) CLI formats
- `TangleResult<T>` - `IntoJobResult` wrapper that ABI-encodes the inner value for on-chain submission

## Relationships
- Depends on `blueprint_core` for `FromJobCall`, `FromJobCallParts`, `OptionalFromJobCallParts`, `JobCall`, `JobResult`, `IntoJobResult` traits and macros
- Depends on `alloy_sol_types::SolValue` for ABI encoding/decoding
- Each extractor defines corresponding rejection types (e.g., `MissingCallId`, `InvalidCallId`, `CallIdRejection`)
- Metadata keys are set by `TangleProducer` when converting on-chain events to `JobCall`s
