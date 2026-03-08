# consumer

## Purpose
Provides `Sink<JobResult>` consumers for submitting job results and transactions to EVM chains. Includes a generic `EVMConsumer` for raw transaction submission and a `TangleConsumer` that calls the Tangle Jobs contract's `submitResult` function.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Generic `EVMConsumer` that deserializes `JobResult` bodies into `TransactionRequest`s and sends them via an Alloy provider with wallet. Defines `AlloyProviderWithWallet` type alias. Implements `Sink<JobResult>` with internal state machine (WaitingForResult / ProcessingTransaction).
- `tangle.rs` - `TangleConsumer` that extracts `ServiceId` and `CallId` from `JobResult` metadata and submits results to a Tangle Jobs contract by ABI-encoding a `submitResult(uint64, uint64, bytes)` call. Also implements `Sink<JobResult>` with the same state machine pattern.

## Key APIs
- `EVMConsumer::new(provider, wallet)` - creates a generic EVM transaction consumer
- `TangleConsumer::new(provider, wallet, contract_address)` - creates a Tangle-specific result consumer
- Both implement `futures::Sink<JobResult>` with `BoxError` as the error type

## Relationships
- Depends on `crate::extract::{CallId, ServiceId}` for reading Tangle metadata keys from job results
- Uses `blueprint_core::JobResult` as the input type
- Paired with producers in `crate::producer` which generate `JobCall`s that flow through handlers and produce `JobResult`s consumed here
