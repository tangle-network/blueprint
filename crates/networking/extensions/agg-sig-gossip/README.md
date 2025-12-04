# blueprint-networking-agg-sig-gossip-extension

Peer-to-peer BLS signature aggregation protocol for Blueprint SDK.

## Overview

This crate implements a gossip-based protocol for aggregating BLS signatures across multiple operators. It handles:

- **Signature Collection**: Operators sign messages and gossip signatures to peers
- **Threshold Checking**: Configurable weight schemes (equal or stake-weighted)
- **Aggregation**: Selected aggregator nodes combine signatures when threshold is met
- **Malicious Detection**: Identifies and reports equivocation and invalid signatures

## Core Types

### SignatureAggregationProtocol

The main protocol handler that coordinates signature collection and aggregation.

```rust
use blueprint_networking_agg_sig_gossip_extension::{
    SignatureAggregationProtocol, ProtocolConfig, EqualWeight,
};

// Create protocol config
let config = ProtocolConfig::new(network_handle, num_aggregators, timeout);

// Create weight scheme (67% threshold)
let weight_scheme = EqualWeight::new(num_participants, 67);

// Create and run protocol
let mut protocol = SignatureAggregationProtocol::new(
    config,
    weight_scheme,
    participant_public_keys,
);

let result = protocol.run(&message).await?;
println!("Aggregated {} signatures", result.contributors.len());
```

### Weight Schemes

Three weight scheme implementations for different threshold types:

#### EqualWeight

Each participant has weight 1. Threshold is percentage of participant count.

```rust
use blueprint_networking_agg_sig_gossip_extension::EqualWeight;

// 67% of 10 participants = 7 signatures required
let scheme = EqualWeight::new(10, 67);
assert_eq!(scheme.threshold_weight(), 6); // floor(10 * 67 / 100)
```

#### CustomWeight

Arbitrary weights per participant with absolute threshold.

```rust
use blueprint_networking_agg_sig_gossip_extension::CustomWeight;
use std::collections::HashMap;

let weights: HashMap<PeerId, u64> = HashMap::from([
    (peer_1, 5000),  // 50% stake
    (peer_2, 3000),  // 30% stake
    (peer_3, 2000),  // 20% stake
]);

// Require 7500 out of 10000 total weight (75%)
let scheme = CustomWeight::new(weights, 7500);
```

#### DynamicWeight

Runtime-selectable between EqualWeight and CustomWeight.

```rust
use blueprint_networking_agg_sig_gossip_extension::DynamicWeight;

// At runtime, choose based on configuration
let scheme = if use_stake_weighted {
    DynamicWeight::custom(operator_weights, threshold_weight)
} else {
    DynamicWeight::equal(num_participants, threshold_percentage)
};
```

### SignatureWeight Trait

All weight schemes implement this trait:

```rust
pub trait SignatureWeight {
    /// Weight of a single participant
    fn weight(&self, peer_id: &PeerId) -> u64;

    /// Total weight of all participants
    fn total_weight(&self) -> u64;

    /// Required weight for valid aggregate
    fn threshold_weight(&self) -> u64;

    /// Calculate weight of a set of participants
    fn calculate_weight(&self, participants: &HashSet<PeerId>) -> u64;

    /// Check if participants meet threshold
    fn meets_threshold(&self, participants: &HashSet<PeerId>) -> bool;
}
```

## Protocol Messages

```rust
pub enum AggSigMessage<S: AggregatableSignature> {
    /// Signature share from a participant
    SignatureShare {
        signer_id: PeerId,
        signature: S::Signature,
        message: Vec<u8>,
    },

    /// Report of malicious behavior
    MaliciousReport {
        operator: PeerId,
        evidence: MaliciousEvidence<S>,
    },

    /// Protocol completion with aggregated result
    ProtocolComplete(AggregationResult<S>),
}
```

## Protocol Flow

```
┌─────────┐     ┌─────────┐     ┌─────────┐
│ Node A  │     │ Node B  │     │ Node C  │
└────┬────┘     └────┬────┘     └────┬────┘
     │               │               │
     │ SignatureShare│               │
     │──────────────►│               │
     │               │ SignatureShare│
     │               │──────────────►│
     │               │               │
     │ SignatureShare│               │
     │◄──────────────│               │
     │               │               │
     │ SignatureShare│ SignatureShare│
     │◄──────────────┼───────────────│
     │               │               │
     │  (threshold met - if aggregator)
     │               │               │
     │ ProtocolComplete              │
     │──────────────►│──────────────►│
     │               │               │
```

## Aggregator Selection

Aggregators are deterministically selected based on the message hash:

```rust
pub struct AggregatorSelector {
    target_aggregators: u16,  // How many aggregators to select
}

// Selection is based on XOR distance from message hash
let selected = selector.select_aggregators(&participant_keys, &message);
let is_aggregator = selector.is_aggregator(my_peer_id, &participant_keys, &message);
```

## Configuration

```rust
pub struct ProtocolConfig<S: AggregatableSignature> {
    /// Network handle for sending/receiving messages
    pub network_handle: NetworkServiceHandle<S>,

    /// Number of aggregators to select (default: 2)
    pub num_aggregators: u16,

    /// Protocol timeout (default: 30s)
    pub timeout: Duration,

    /// Message poll interval (default: 25ms)
    pub message_poll_interval: Duration,

    /// Threshold check interval (default: 50ms)
    pub threshold_check_interval: Duration,
}
```

## Malicious Detection

The protocol detects and reports:

- **Invalid Signatures**: Signature doesn't verify against public key
- **Equivocation**: Same signer signs different messages

```rust
pub enum MaliciousEvidence<S: AggregatableSignature> {
    /// Signer signed multiple different messages
    Equivocation {
        message1: Vec<u8>,
        signature1: S::Signature,
        message2: Vec<u8>,
        signature2: S::Signature,
    },

    /// Signature is invalid for the given message
    InvalidSignature {
        message: Vec<u8>,
        signature: S::Signature,
    },
}
```

## Integration with blueprint-tangle-evm-extra

This crate is typically used through `blueprint-tangle-evm-extra`:

```rust
use blueprint_tangle_evm_extra::{P2PGossipConfig, AggregationStrategy};

let config = P2PGossipConfig::new(network_handle, participant_keys)
    .with_threshold_percentage(67);

let strategy = AggregationStrategy::P2PGossip(config);
```

## Testing

The crate includes comprehensive tests using mock networks:

```rust
use blueprint_gossip_primitives::MockNetwork;

#[tokio::test]
async fn test_aggregation() {
    let mock = MockNetwork::new(MockNetworkConfig::default());
    // ... test protocol with mock network
}
```

## License

Licensed under the Apache License, Version 2.0.
