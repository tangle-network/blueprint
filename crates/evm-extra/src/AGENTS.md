# src

## Purpose
Source directory for the EVM job utilities crate. Provides a pipeline architecture for processing EVM blockchain events: producers watch for new blocks/events, extractors parse relevant data from blocks and transactions, filters select specific events/contracts, and consumers process the results (e.g., submitting to Tangle).

## Contents (one hop)
### Subdirectories
- [x] `consumer/` - Event consumers that process extracted EVM data; includes Tangle-specific consumer for submitting job results
- [x] `extract/` - Data extraction from EVM blocks, transactions, events, contracts, and job call data
- [x] `filters/` - Filtering logic for selecting specific contracts and events from EVM data streams
- [x] `producer/` - Block/event producers: polling-based block watching and Tangle-integrated event production

### Files
- `lib.rs` - Crate root; declares the four pipeline modules (`consumer`, `extract`, `filters`, `producer`) and `util`
- `util.rs` - EVM provider utilities: `get_provider_http`, `get_provider_ws`, `get_wallet_provider_http`, `get_provider_from_signer`, `wait_transaction`; defines `SIGNATURE_EXPIRY` constant

## Key APIs (no snippets)
- `util::get_provider_http` / `get_provider_ws` / `get_wallet_provider_http` - Factory functions for Alloy RPC providers
- `util::get_provider_from_signer` - Creates a wallet-backed provider from a private key string
- `util::wait_transaction` - Awaits transaction confirmation and returns receipt
- Producer/Consumer/Filter/Extract pipeline pattern for EVM event processing

## Relationships
- Depends on `alloy-provider`, `alloy-network`, `alloy-signer-local`, `alloy-primitives`
- Used by blueprints that need to react to on-chain EVM events
- The Tangle consumer/producer modules bridge EVM events to the Tangle network

## Notes
- Strict lint configuration: denies missing debug/copy impls, unsafe code, and missing docs
- Supports `no_std` at the crate level with `extern crate alloc`
- Provider utilities panic on invalid URLs (documented in doc comments)
