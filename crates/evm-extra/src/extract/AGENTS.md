# extract

## Purpose
Provides typed extractors for EVM-specific data from `JobCallParts`, following the `FromJobCallParts` pattern from `blueprint_core`. Extractors pull values from job call metadata and extensions, enabling job handlers to declaratively access block info, event data, contract addresses, and Tangle-specific job fields.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports all extractors from submodules.
- `block.rs` - `BlockNumber`, `BlockHash`, `BlockTimestamp` extractors that read from `X-EVM-BLOCK-*` metadata keys. Each has a corresponding rejection type.
- `contract.rs` - `ContractAddress` extractor reading from `X-EVM-CONTRACT-ADDRESS` metadata.
- `event.rs` - `BlockEvents` (all logs), `Events<T>` (typed by `SolEvent`), `FirstEvent<T>`, `LastEvent<T>` extractors that read `Vec<Log>` from job call extensions and optionally decode to specific Solidity event types.
- `job.rs` - `ServiceId`, `CallId`, `JobIndex`, `Caller`, `JobInputs` extractors for Tangle-specific job metadata. Each implements both `FromJobCallParts` and `OptionalFromJobCallParts`. Includes unit tests for optional extraction semantics.
- `tx.rs` - `Tx` wrapper around `TransactionRequest` that implements `IntoJobResult` for converting EVM transactions into serialized job results.

## Key APIs
- `BlockNumber`, `BlockHash`, `BlockTimestamp` - block-level metadata extractors
- `ContractAddress` - contract address extractor
- `Events<T>`, `FirstEvent<T>`, `LastEvent<T>`, `BlockEvents` - event extractors
- `ServiceId`, `CallId`, `JobIndex`, `Caller`, `JobInputs` - Tangle job field extractors
- `Tx` - transaction-to-job-result converter implementing `IntoJobResult`

## Relationships
- Metadata keys defined here (e.g., `X-EVM-BLOCK-NUMBER`, `X-EVM-SERVICE-ID`) are populated by producers in `crate::producer`
- `ServiceId` and `CallId` metadata keys are consumed by `crate::consumer::TangleConsumer`
- Implements traits from `blueprint_core` (`FromJobCallParts`, `OptionalFromJobCallParts`, `IntoJobResult`)
