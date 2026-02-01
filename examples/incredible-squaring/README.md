# Incredible Squaring Blueprint

The Incredible Squaring blueprint showcases a complete Tangle v2 EVM workflow:

- Job `0` (`square`) squares an input with a single operator result.
- Job `1` (`verified_square`) requires two matching operator results.
- Job `2` (`consensus_square`) requires a three-operator quorum and is the canonical “BLS aggregation” demo.

The Rust crate wires these jobs into `TangleLayer`, exposes a `Router` for the runner, and includes tests that cover
direct submission, aggregation logic, and full Anvil-backed end-to-end flows.

## Requirements

The harness boots Anvil using the snapshot in `crates/chain-setup/anvil/snapshots/localtestnet-state.json`. The
`LocalTestnet.s.sol` broadcast is bundled in `crates/chain-setup/anvil/snapshots/localtestnet-broadcast.json`.
Refresh fixtures with `scripts/fetch-localtestnet-fixtures.sh`. Opt-in to the heavy integration tests with:

```bash
export RUN_TNT_E2E=1
```

`run.sh` mirrors how CI opts into the Anvil suites while keeping everything local to this repo.

## Build

```bash
cargo build -p incredible-squaring-blueprint
```

## End-to-end test (Anvil harness)

The harness bootstraps Anvil with the seeded `LocalTestnet.s.sol` broadcast, spins up the blueprint runner, and submits a
job through the Tangle contracts. Run it directly:

```bash
./run.sh                   # convenience wrapper
# or
cargo test -p incredible-squaring-blueprint-lib --test anvil -- --nocapture
```

The test uses the shared `router()` from `incredible-squaring-lib`, so the runner and the harness always agree on job
indices and Tangle layers.

## Additional tests

| Test target | Command | What it covers |
|-------------|---------|----------------|
| `direct_submission.rs` | `cargo test -p incredible-squaring-blueprint-lib --test direct_submission` | Multi-operator flows without aggregation |
| `integration.rs` | `cargo test -p incredible-squaring-blueprint-lib --test integration` | BLS aggregation service, thresholds, duplicate handling |
| `e2e.rs` | `cargo test -p incredible-squaring-blueprint-lib --test e2e` | ABI encoding/decoding + extractor plumbing |

All tests run entirely in Rust (no JS runners) and are wired to `blueprint-sdk`’s new Tangle EVM helpers.

## Running the blueprint binary

To manually run the operator binary against a seeded Anvil deployment:

```bash
export PROTOCOL=tangle
export BLUEPRINT_ID=0
export SERVICE_ID=0
export TANGLE_CONTRACT=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
export RESTAKING_CONTRACT=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
export STATUS_REGISTRY_CONTRACT=0xdC64a140Aa3E981100a9BecA4E685f962f0CF6C9
export BLUEPRINT_KEYSTORE_URI=$(pwd)/target/keystore

cargo tangle key --algo ecdsa --keystore "$BLUEPRINT_KEYSTORE_URI" --name local-operator
RUST_LOG=info cargo run -p incredible-squaring-blueprint-bin -- run \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-uri "$BLUEPRINT_KEYSTORE_URI" \
  --data-dir ./target/incredible-squaring-data
```

The CLI flags configure the runner side (`BlueprintEnvironment`) while the env vars feed
`TangleProtocolSettings`. Point the RPC URLs at your local Anvil deployment, ensure the keystore contains
the operator key you registered on-chain, and the runner will automatically connect via `TangleProducer` /
`TangleConsumer`.
