# Anvil Operator Runbook

This guide documents the full loop for exercising a Blueprint against the
seeded Anvil testnet. It covers keystore provisioning, env vars, the harness
commands used in CI, and a minimal client snippet for submitting jobs.

## 1. Prerequisites

- Docker with the default socket exposed (`testcontainers` launches Anvil).
- Rust toolchain + Foundry (Install via `foundryup` or `foundry-toolchain`).

## 2. Generate a local operator key

Use the CLI to mint an ECDSA keypair and store it on disk. The Blueprint
runner and `TangleEvmClient` can both read this format.

```bash
cargo tangle key --algo ecdsa --keystore ./local-operator-keys --name anvil-operator
export BLUEPRINT_KEYSTORE_URI="$(pwd)/local-operator-keys"
```

The `BLUEPRINT_KEYSTORE_URI` value can be fed directly into
`BlueprintEnvironment.keystore_uri` or the `--keystore-uri` flag when running a
Blueprint binary.

## 3. Boot a seeded Anvil network

Every integration suite uses the env-aware harness from `blueprint-anvil-testing-utils`:

```rust
use blueprint_anvil_testing_utils::harness_builder_from_env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let harness = harness_builder_from_env()
        .include_anvil_logs(true)
        .spawn()
        .await?;
    println!("HTTP RPC: {}", harness.http_endpoint());
    println!("WS RPC:   {}", harness.ws_endpoint());
    tokio::signal::ctrl_c().await?;
    Ok(())
}
```

> **NOTE:** The harness loads `crates/chain-setup/anvil/snapshots/localtestnet-state.json` by default so startup is instant. The fallback broadcast is bundled at `crates/chain-setup/anvil/snapshots/localtestnet-broadcast.json`. Refresh fixtures with `scripts/fetch-localtestnet-fixtures.sh`. `RUN_TNT_E2E=1` only gates the long-running integration tests—export it when you want to opt in.
> ```bash
> export RUN_TNT_E2E=1
> ```

Running this binary keeps the seeded Anvil node alive with the contracts from
`LocalTestnet.s.sol`. CI runs the exact same function inside each test via
`testcontainers`.

## 4. Run the harness end-to-end

The recommended smoke test exercises the new Blueprint harness plus a sample
router:

```bash
cargo test -p hello-tangle-blueprint --test anvil -- --nocapture
```

Behind the scenes `BlueprintHarness` performs the following:

1. Calls `harness_builder_from_env().spawn()` to spawn Anvil with all contracts.
2. Seeds a temporary filesystem keystore with the default operator key baked
   into the LocalTestnet fixture.
3. Builds a `BlueprintEnvironment` + `Router` pair and launches
   `BlueprintRunner`.
4. Uses `TangleEvmClient::submit_job` and waits for `JobResultSubmitted`.

Swap in your own router to exercise custom jobs.

## 5. Submit jobs manually with `TangleEvmClient`

The `TangleEvmClient` can be reused for local scripts as well:

```rust
use alloy_primitives::{Address, Bytes};
use blueprint_client_tangle_evm::{TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings};
use hello_tangle_blueprint::DocumentRequest;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = TangleEvmSettings {
        blueprint_id: 0,
        service_id: Some(0),
        tangle_contract: Address::from_str("0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9")?,
        restaking_contract: Address::from_str("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512")?,
        status_registry_contract: Address::from_str("0xdC64a140Aa3E981100a9BecA4E685f962f0CF6C9")?,
    };

    let config = TangleEvmClientConfig::new(
        "http://127.0.0.1:8545".parse()?,
        "ws://127.0.0.1:8546".parse()?,
        std::env::var("BLUEPRINT_KEYSTORE_URI")?,
        settings,
    )
    .test_mode(true);

    let client = TangleEvmClient::new(config).await?;
    let payload = DocumentRequest {
        docId: "doc-1".into(),
        contents: "hello world".into(),
    }
    .abi_encode();

    client
        .submit_job(0, hello_tangle_blueprint::CREATE_DOCUMENT_JOB, Bytes::from(payload))
        .await?;
    Ok(())
}
```

Replace the IDs and payload with your own Blueprint/job configuration when
running operators locally.

## 6. Snapshots

Every Anvil-based suite now seeds state from the JSON snapshot stored at
`crates/chain-setup/anvil/snapshots/localtestnet-state.json`. The harness
loads this file automatically and only replays the Foundry broadcast when the
snapshot is missing or fails validation.

Refresh the snapshot whenever the protocol deployment changes:

1. Run `scripts/update-anvil-snapshot.sh` to pull the latest fixtures from
   `tnt-core-fixtures`.

## 7. Generating preregistration payloads

Operators can now capture the TLV payload that the contracts expect during
registration without submitting the on-chain transaction. Use the new CLI
command while pointing at your local Anvil instance:

```bash
cargo tangle blueprint preregister \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./local-operator-keys \
  --settings-file ./settings.env
```

This spins up the blueprint manager in `REGISTRATION_MODE_ON`, launches the
blueprint with `REGISTRATION_CAPTURE_ONLY=1`, and waits for the blueprint to
write `registration_inputs.bin` under your data directory. The CLI prints the
path so you can feed it directly into `cargo tangle blueprint register`.

Blueprint authors should check `BlueprintEnvironment::registration_mode()` early
in their binaries and emit the payload through the helper
`blueprint_sdk::registration::write_registration_inputs`. The runner will block
until the file exists and will automatically pass the bytes to the manager when
registration is requested on-chain.
