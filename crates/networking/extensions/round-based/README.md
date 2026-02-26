# blueprint-networking-round-based-extension

Adapter between Blueprint networking and `round-based` protocol APIs.

## What it provides

- `RoundBasedNetworkAdapter` implementing `round_based::Delivery`.
- Sender/receiver adapters that translate between protocol messages and network transport payloads.
- Party index <-> peer ID mapping utilities for round-based sessions.

## When to use

Use for MPC/threshold protocols that already target the `round-based` crate and need Blueprint network transport.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/networking/extensions/round-based
