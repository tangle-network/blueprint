# producer

## Purpose
Provides `Stream`-based producers that poll EVM chains for new event logs and convert them into `JobCall`s. Includes a generic `PollingProducer` for arbitrary events and a `TangleProducer` specialized for Tangle's `JobSubmitted` events.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Shared `logs_to_job_calls` function that groups logs by block number, populates metadata (block number, hash, timestamp, contract address), inserts logs into extensions, and creates one `JobCall` per log with the event signature as the job ID. Re-exports submodule types.
- `polling.rs` - `PollingProducer` that implements `Stream<Item = Result<JobCall, TransportError>>`. Uses a three-state machine (Idle / FetchingBlockNumber / FetchingLogs) to poll blocks in configurable step sizes with confirmation depth for finality. `PollingConfig` provides builder methods for start block, interval, confirmations, and step size.
- `tangle.rs` - `TangleProducer` that polls specifically for `JobSubmitted(uint64 serviceId, uint64 callId, uint8 jobIndex, address caller, bytes inputs)` events from a Tangle Jobs contract. Decodes event data and populates job-specific metadata (`ServiceId`, `CallId`, `JobIndex`, `Caller`) and stores inputs in extensions. `TangleProducerConfig` configures the contract address and polling parameters.

## Key APIs
- `PollingProducer::new(provider, config)` - creates a generic event polling producer
- `PollingConfig` - builder for poll interval, start block, confirmations, step size
- `TangleProducer::new(provider, config)` - creates a Tangle job event producer
- `TangleProducerConfig::new(contract_address)` - builder for Tangle-specific polling
- Both implement `futures::Stream<Item = Result<JobCall, TransportError>>`

## Relationships
- Populates metadata keys consumed by extractors in `crate::extract` (block, contract, job fields)
- Populates `Vec<Log>` in extensions consumed by event extractors and filters in `crate::extract::event` and `crate::filters`
- `TangleProducer` creates `JobCall`s that are eventually consumed by `crate::consumer::TangleConsumer`
