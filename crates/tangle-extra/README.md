# blueprint-tangle-extra

Producer/Consumer extras for Tangle (v2) blueprints with BLS signature aggregation support.

## Overview

This crate provides the infrastructure for building blueprints that interact with Tangle v2 EVM contracts, including:

- **TangleProducer**: Polls for `JobSubmitted` events and streams job calls
- **TangleConsumer**: Submits individual job results via `submitResult`
- **AggregatingConsumer**: Aggregates BLS signatures and submits via `submitAggregatedResult`
- **AggregationStrategy**: Choose between HTTP service or P2P gossip for signature aggregation

## Features

| Feature | Description |
|---------|-------------|
| `std` (default) | Standard library support |
| `aggregation` | HTTP-based aggregation service client |
| `p2p-aggregation` | Peer-to-peer gossip-based aggregation |

## Quick Start

### Basic Blueprint (No Aggregation)

```rust
use blueprint_sdk::BlueprintRunner;
use blueprint_tangle_extra::{TangleConsumer, TangleProducer, TangleLayer};

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    let env = BlueprintEnvironment::load()?;
    let client = env.tangle_client().await?;
    let service_id = env.protocol_settings.tangle()?.service_id.unwrap();

    BlueprintRunner::builder(TangleConfig::default(), env)
        .router(Router::new()
            .route(0, my_job.layer(TangleLayer)))
        .producer(TangleProducer::new(client.clone(), service_id))
        .consumer(TangleConsumer::new(client))
        .run()
        .await
}
```

### With BLS Aggregation (HTTP Service)

```rust
use blueprint_tangle_extra::{
    AggregatingConsumer, AggregationStrategy, HttpServiceConfig,
};

let consumer = AggregatingConsumer::new(client.clone())
    .with_aggregation_strategy(AggregationStrategy::HttpService(
        HttpServiceConfig::new("http://localhost:8080", bls_secret, operator_index)
            .with_wait_for_threshold(true)
            .with_threshold_timeout(Duration::from_secs(60))
    ));
```

### With BLS Aggregation (P2P Gossip)

```rust
use blueprint_tangle_extra::{
    AggregatingConsumer, AggregationStrategy, P2PGossipConfig, ThresholdType,
};
use std::collections::HashMap;

// Discover operator BLS public keys (from on-chain registration)
let participant_keys: HashMap<PeerId, ArkBlsBn254Public> =
    discover_operator_keys(&client, service_id).await?;

// Count-based threshold (67% of operators must sign)
let p2p_config = P2PGossipConfig::new(network_handle, participant_keys)
    .with_threshold_percentage(67);

let consumer = AggregatingConsumer::new(client.clone())
    .with_aggregation_strategy(AggregationStrategy::P2PGossip(p2p_config));
```

## Threshold Types

The aggregation system supports two threshold types that match the on-chain `Types.ThresholdType`:

### Count-Based (Default)

Each operator has equal weight (1). Threshold is a percentage of operator count.

```rust
// 67% of operators must sign
let config = P2PGossipConfig::new(network_handle, participant_keys)
    .with_threshold_percentage(67);

// Or using basis points (matches on-chain format):
let config = P2PGossipConfig::new(network_handle, participant_keys)
    .with_threshold_bps(6700)  // 67.00%
    .with_threshold_type(ThresholdType::CountBased);
```

### Stake-Weighted

Operators are weighted by their stake exposure (`ServiceOperator.exposureBps` from the contract).

```rust
// Query operator weights from chain (exposureBps values)
let operator_weights: HashMap<PeerId, u64> = HashMap::from([
    (peer_id_1, 5000),  // 50% stake exposure
    (peer_id_2, 3000),  // 30% stake exposure
    (peer_id_3, 2000),  // 20% stake exposure
]);

// 75% of total stake must sign
let config = P2PGossipConfig::new(network_handle, participant_keys)
    .with_stake_weighted_threshold(7500, operator_weights);
```

## P2PGossipConfig API

| Method | Description |
|--------|-------------|
| `new(handle, keys)` | Create with defaults (67% count-based) |
| `with_threshold_percentage(u8)` | Set threshold as percentage (0-100) |
| `with_threshold_bps(u16)` | Set threshold in basis points (0-10000) |
| `with_threshold_type(ThresholdType)` | Set explicit threshold type |
| `with_operator_weights(HashMap)` | Set operator weights for stake-weighted |
| `with_stake_weighted_threshold(bps, weights)` | Convenience for stake-weighted setup |
| `with_timeout(Duration)` | Protocol timeout (default: 30s) |
| `with_num_aggregators(u16)` | Number of aggregator nodes (default: 2) |

## On-Chain Integration

The threshold configuration comes from your `BlueprintServiceManager` contract:

```solidity
function getAggregationThreshold(uint64 serviceId, uint8 jobIndex)
    external view returns (uint16 thresholdBps, uint8 thresholdType);

// thresholdBps: e.g., 6700 = 67%
// thresholdType: 0 = CountBased, 1 = StakeWeighted
```

The `AggregatingConsumer` queries this to determine how to aggregate signatures, then submits the result via:

```solidity
function submitAggregatedResult(
    uint64 serviceId,
    uint64 callId,
    bytes calldata output,
    uint256 signerBitmap,
    uint256[2] calldata aggregatedSignature,
    uint256[4] calldata aggregatedPubkey
) external;
```

## Architecture

```
                    ┌─────────────────┐
                    │  Job Submitted  │
                    │    (on-chain)   │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │  TangleProducer │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │  Blueprint Job  │
                    │    Handler      │
                    └────────┬────────┘
                             │
              ┌──────────────┴──────────────┐
              │                             │
    ┌─────────▼─────────┐         ┌────────▼────────┐
    │  TangleConsumer   │         │AggregatingConsumer│
    │  (single result)  │         │ (BLS aggregated) │
    └─────────┬─────────┘         └────────┬────────┘
              │                             │
              │                 ┌───────────┴───────────┐
              │                 │                       │
              │       ┌─────────▼─────────┐   ┌────────▼────────┐
              │       │   HTTP Service    │   │   P2P Gossip    │
              │       │   Aggregation     │   │   Aggregation   │
              │       └─────────┬─────────┘   └────────┬────────┘
              │                 │                       │
              │                 └───────────┬───────────┘
              │                             │
    ┌─────────▼─────────┐         ┌────────▼────────┐
    │   submitResult    │         │submitAggregated │
    │    (on-chain)     │         │    (on-chain)   │
    └───────────────────┘         └─────────────────┘
```

## License

Licensed under the Apache License, Version 2.0.
