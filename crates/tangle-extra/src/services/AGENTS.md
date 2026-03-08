# services

## Purpose
Background keeper services that automate lifecycle operations on Tangle v2 contracts. Each keeper monitors a specific contract and triggers periodic operations (epoch distributions, round advancement, stream drips, subscription billing) when conditions are met.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports all keepers and core abstractions
- `keeper.rs` - Core abstractions: `BackgroundKeeper` trait (`start`, `check_and_execute`); `KeeperConfig` (RPC endpoint, keystore, contract addresses, check intervals, monitored operators); `KeeperHandle` (join handle wrapper); `KeeperError`/`KeeperResult` types; wallet/provider construction from keystore
- `epoch.rs` - `EpochKeeper` implementing `BackgroundKeeper`; monitors `InflationPool` contract and calls `distributeEpoch()` when `isEpochReady()` returns true
- `round.rs` - `RoundKeeper` implementing `BackgroundKeeper`; monitors `MultiAssetDelegation` contract and advances rounds when enough time has passed
- `stream.rs` - `StreamKeeper` implementing `BackgroundKeeper`; monitors `StreamingPaymentManager` for pending drips above a threshold and triggers distribution to operators
- `billing.rs` - `SubscriptionBillingKeeper` implementing `BackgroundKeeper`; discovers subscription-based services via `ITangle` and calls `billSubscriptionBatch` when payment intervals elapse

## Key APIs
- `BackgroundKeeper` trait - `const NAME`, `fn start(config, shutdown_rx) -> KeeperHandle`, `async fn check_and_execute(config) -> KeeperResult<bool>`
- `KeeperConfig::new(http_rpc, keystore)` - builder with `with_inflation_pool`, `with_multi_asset_delegation`, `with_streaming_payment_manager`, `with_tangle_contract`, and interval setters
- `KeeperConfig::get_provider()` / `get_read_provider()` - constructs alloy providers with or without wallet signing
- `KeeperConfig::get_operator_address()` - derives operator address from keystore ECDSA key
- `KeeperHandle::join()` - awaits background task completion

## Relationships
- Depends on `alloy` for provider/contract interaction and `blueprint_keystore` for signing keys
- Uses `tokio::sync::broadcast` for shutdown signaling
- Designed to be started alongside the blueprint runner as background services
- Contract interfaces are defined inline via `alloy::sol!` macros
