# extensions

## Purpose
Aggregator directory for networking protocol extensions that build on top of the core `blueprint-networking` P2P layer. Each subdirectory is an independent crate providing a specific multi-party protocol pattern (signature aggregation, round-based MPC, or shared gossip primitives).

## Contents (one hop)
### Subdirectories
- [x] `agg-sig-gossip/` - `blueprint-networking-agg-sig-gossip-extension`: signature aggregation over gossipsub using `blueprint-crypto` aggregation primitives and `bitvec` signer bitmaps
- [x] `gossip-primitives/` - `blueprint-gossip-primitives`: reusable gossip protocol building blocks (message deduplication via LRU/blake3, async round management, optional live libp2p transport); used as a foundation by other extensions
- [x] `round-based/` - `blueprint-networking-round-based-extension`: adapter integrating the `round-based` crate with `blueprint-networking` for multi-round MPC protocols over P2P

### Files
- (none)

## Key APIs (no snippets)
- Signature aggregation gossip protocol (agg-sig-gossip)
- Gossip message primitives with deduplication and round tracking (gossip-primitives)
- Round-based MPC protocol adapter over blueprint networking (round-based)

## Relationships
- All extensions depend on `blueprint-networking` and `blueprint-crypto`
- `agg-sig-gossip` depends on `gossip-primitives`
- These extensions are consumed by blueprint services that need multi-party computation or aggregated signatures
