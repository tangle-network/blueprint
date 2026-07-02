# Tangle CLI

Create, run, and operate blueprints on the Tangle EVM and EigenLayer.

## Table of Contents

- [Tangle CLI](#tangle-cli)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Installation](#installation)
    - [Feature flags](#feature-flags)
  - [Creating a New Blueprint](#creating-a-new-blueprint)
  - [Running a Blueprint on Tangle](#running-a-blueprint-on-tangle)
  - [Registering an Operator](#registering-an-operator)
  - [Service Lifecycle Commands](#service-lifecycle-commands)
  - [Cloud Deployment](#cloud-deployment)
    - [Configure Cloud Provider](#configure-cloud-provider)
    - [Cost Estimation](#cost-estimation)
    - [Deploy Blueprint to Cloud](#deploy-blueprint-to-cloud)
    - [Monitor Cloud Deployments](#monitor-cloud-deployments)
  - [Key Management](#key-management)
  - [EigenLayer Helpers](#eigenlayer-helpers)
  - [Generating Keys from the Command Line](#generating-keys-from-the-command-line)
    - [Flags](#flags)

## Overview

The CLI bundles every workflow needed for the EVM-only SDK:

- `cargo tangle blueprint create` scaffolds a new blueprint from the templates.
- `cargo tangle blueprint run --protocol tangle` boots a manager connected to the Tangle v2 contracts.
- `cargo tangle blueprint register-tangle` registers an operator against `ITangle`, `MultiAssetDelegation`, and `OperatorStatusRegistry`.
- `cargo tangle key *` manages local and remote k256 keys via `blueprint-keystore`.

All Substrate helpers have been removed; the CLI now targets EVM-first flows only.

## Installation

> Linux, macOS, and Windows (WSL2) are supported.

```bash
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint --force
```

### Feature flags

The CLI supports optional features that can be enabled at build time:

**`remote-providers`** - Enables cloud deployment functionality

Adds support for deploying blueprints to AWS, GCP, Azure, DigitalOcean, and Vultr. This enables the `cargo tangle cloud` subcommand and the `--remote` flag for blueprint deployment.

```bash
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint \
  --features remote-providers --force
```

Without this feature, cloud commands are not available and using `--remote` will show:
```
❌ Remote deployment requires the 'remote-providers' feature.
   Build with: cargo build --features remote-providers
```

**`vm-debug`** - Enables VM sandbox debugging (Linux only)

```bash
cargo build --features vm-debug
```

## Creating a New Blueprint

```bash
cargo tangle blueprint create --name my_blueprint
```

The scaffold asks for a source template, optional variables, and whether to skip prompts. The generated workspace already depends on `blueprint-sdk` with the `tangle` feature.

For Tangle blueprints, the scaffold also includes a
`metadata/blueprint-metadata.json` file. Publish that JSON to IPFS or HTTPS,
then set the resulting URI as `metadata_uri` in your deploy definition. Add
either `metadata_hash` or `metadata_file` as well so the deploy manifest pins
the expected payload digest onchain. The URI is what lands onchain; the full
JSON stays offchain and is what
`tangle-cloud` ingests to render tier-2 hosted blueprint surfaces.

For production tier-2 hosting, publish to `ipfs://` and include an owner-signed
metadata attestation. If the shared host cannot verify provenance, it falls
back to the protocol-controlled generic blueprint UI.

## Running a Blueprint on Tangle

The runner expects RPC URLs, a keystore, and the EVM contract coordinates. You can provide them via CLI flags or a `settings.env` file that the command loads before boot.

```bash
BLUEPRINT_ID=0 \
SERVICE_ID=0 \
TANGLE_CONTRACT=0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9 \
STAKING_CONTRACT=0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0 \
STATUS_REGISTRY_CONTRACT=0x5f3f1dBD7B74C6B46e8c44f98792A1dAf8d69154 \
cargo tangle blueprint run \
  --protocol tangle \
  --http-rpc-url http://127.0.0.1:8545 \
  --ws-rpc-url ws://127.0.0.1:8546 \
  --keystore-path ./keystore \
  --settings-file ./settings.env
```

| Variable | Description |
| --- | --- |
| `BLUEPRINT_ID` | Blueprint registered on `ITangle`. |
| `SERVICE_ID` | Optional fixed service (leave unset to process all). |
| `TANGLE_CONTRACT` | `ITangle` contract address. |
| `STAKING_CONTRACT` | `MultiAssetDelegation` staking contract. |
| `STATUS_REGISTRY_CONTRACT` | `OperatorStatusRegistry` heartbeat contract. |

The CLI automatically ensures an ECDSA key exists under `--keystore-path` and derives the operator address from it.

## Registering an Operator

`register-tangle` performs the on-chain registration + announcement flow in one command:

```bash
cargo tangle blueprint register-tangle \
  --http-rpc-url https://rpc.tangle.tools \
  --ws-rpc-url wss://rpc.tangle.tools \
  --keystore-path ./keystore \
  --tangle-contract 0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9 \
  --staking-contract 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0 \
  --status-registry-contract 0x5f3f1dBD7B74C6B46e8c44f98792A1dAf8d69154 \
  --blueprint-id 0 \
  --registration-inputs ./registration.tlv
```

You can optionally set `--rpc-endpoint` to push metadata to a remote operator directory service during registration.

## Service Lifecycle Commands

The `cargo tangle blueprint service` namespace mirrors everything exposed in `Tangle.sol`:

- `service request` submits a pending service with per-operator exposures, rich config payloads, and optional asset security requirements.
- `service approve` / `service reject` lets operators respond to a request; approvals can include explicit asset commitments when the request included requirements.
- `service join` / `service leave` let operators participate in dynamic membership services; leaving succeeds when the service's exit queue allows the legacy helper.
- `service list` and `service requests` surface active services vs. pending requests, with a `--json` toggle for scripting.

### Requesting a Service

```bash
cargo tangle blueprint service request \
  --blueprint-id 1 \
  --operator 0x... --operator 0x... \
  --operator-exposure-bps 7000 --operator-exposure-bps 3000 \
  --permitted-caller 0xdeadbeef... \
  --config-file ./service-config.bin \
  --ttl 86400 \
  --payment-token 0x0000000000000000000000000000000000000000 \
  --payment-amount 1000000000000000000 \
  --security-requirement native:_ :5000:10000
```

- Provide one `--operator-exposure-bps` per operator (basis points, 10_000 = 100%). Leave unset to fall back to the default BPS of 100% per operator.
- Security requirements use the format `KIND:TOKEN:MIN:MAX` where `KIND` is `native` or `erc20`. Use `_`/`0` for the native token placeholder.
- TTLs are expressed in seconds to match the `Tangle.sol` schema.

### Approving or Rejecting

```bash
# Simple staking approval
cargo tangle blueprint service approve \
  --request-id 42 \
  --staking-percent 50

# Approval that matches request-level security requirements
cargo tangle blueprint service approve \
  --request-id 42 \
  --security-commitment native:_ :6000
```

- `--security-commitment` mirrors the `KIND:TOKEN:EXPOSURE` structure expected by `approveServiceWithCommitments`.
- Use `service reject --request-id <ID>` to decline participation.

### Listing Services/Requests

```bash
cargo tangle blueprint service list --json
cargo tangle blueprint service requests --json
```

Both commands read via `TangleClient::{list_services,list_service_requests}` and print either friendly tables or JSON for automation.

## Blueprint Upgrade Flow

Shipping a new build of a blueprint touches three actors. The CLI gives each
of them a first-class subcommand surface so nobody has to hand-craft calldata.

```
            ┌─────────────────────┐
            │  blueprint owner    │  publish-version → set-active-version
            └─────────┬───────────┘
                      │
                      ▼
            ┌─────────────────────┐
            │  auditor / scanner  │  attest submit → (optional) attest revoke
            └─────────┬───────────┘
                      │
                      ▼
            ┌─────────────────────┐
            │  service operator   │  service set-policy → service ack-version
            └─────────────────────┘
```

### 1. Publish a new version (blueprint owner)

```bash
# Hash the artifact locally, pin to IPFS, and submit publishBinaryVersion.
cargo tangle blueprint publish-version \
  --blueprint-id 7 \
  --binary ./target/release/my-blueprint \
  --pin-to-ipfs \
  --attestation-bundle ./dist/sigstore-bundle.json \
  --json
```

- `--binary <path>` is hashed (sha256) locally; the digest becomes
  `sha256Hash` on-chain.
- Provide either `--binary-uri ipfs://<cid>` (you've already pinned) or
  `--pin-to-ipfs` (the CLI pins via `IPFS_API_URL`+`IPFS_API_TOKEN` if set,
  or `PINATA_JWT` as a fallback).
- `--attestation-bundle <path>` is hashed and stored as `attestationHash`.
  Pass `--attestation-hash <hex>` to override directly (e.g. when the
  bundle lives off-disk).

The command prints the new `version_id` and tx hash. In JSON mode the
output is a single line you can pipe to `jq`:

```json
{ "event": "binary_version_published",
  "blueprint_id": 7, "version_id": 3,
  "sha256_hash": "0x...", "binary_uri": "ipfs://...",
  "attestation_hash": "0x...", "tx_hash": "0x...", "block_number": 12345 }
```

Promote (or roll back) the active pointer once you're ready:

```bash
cargo tangle blueprint set-active-version --blueprint-id 7 --version-id 3
cargo tangle blueprint deprecate-version  --blueprint-id 7 --version-id 1
cargo tangle blueprint list-versions      --blueprint-id 7        # table view
cargo tangle blueprint show-version       --blueprint-id 7 --version-id 3
```

### 2. Attest as an auditor

```bash
# Hash the audit report locally, pin it, and emit attestBinaryVersion.
cargo tangle attest submit \
  --blueprint-id 7 \
  --version-id 3 \
  --report ./audits/2026-05-tangle-bp-7-v3.pdf \
  --pin-report-to-ipfs \
  --kind AUDIT \
  --severity low \
  --expires-in 6m \
  --json
```

- `--report` accepts either a local file (which is hashed; optionally pinned
  via `--pin-report-to-ipfs`) or a remote URL (`https://...`, `ipfs://...`).
  When passing a URL, pre-compute the hash and feed it via `--report-hash <hex>`.
- `--kind` is one of `AUDIT`, `FUZZ`, `FORMAL`, `BUG_BOUNTY`, `SELF`.
- `--severity` maps to the on-chain uint8 ladder: `none`/`info`/`low`/`med`/`high`/`critical`.
- `--expires-in` accepts human durations (`6m`, `30d`, `1y`, `90d12h`); omit
  for non-expiring.

Other attest commands:

```bash
cargo tangle attest list   --blueprint-id 7 --version-id 3 --json
cargo tangle attest revoke --blueprint-id 7 --version-id 3 \
  --attestation-id 2 \
  --reason ipfs://bafy.../revocation-note.json
```

### 3. Adopt the upgrade as an operator

Set the policy once, then ack each new version under `APPROVE`:

```bash
# Pick a policy: AUTO (follow blueprint owner), APPROVE (opt-in), MANUAL (pin to genesis).
cargo tangle blueprint service set-policy --service-id 42 --policy APPROVE

# Opt in to a specific version.
cargo tangle blueprint service ack-version --service-id 42 --version-id 3

# Inspect what's actually running and how far ahead the upstream is.
cargo tangle blueprint service effective-version --service-id 42 --blueprint-id 7 --json
cargo tangle blueprint service upgrade-status     --service-id 42 --blueprint-id 7 --json
```

`upgrade-status` prints the policy, the operator's acked version, the
blueprint's active version, the effective version the manager will dispatch
against, and the latest published version — exactly what an operator-facing
dashboard needs.

### CI gating: trust score

```bash
# Compute the weighted trust score the dapp would render for version 3.
cargo tangle blueprint trust-score \
  --blueprint-id 7 --version-id 3 \
  --auditors-contract 0xAud1tors... \
  --min-score 80           # exits non-zero if score < 80; great for CI
```

The score walks every non-revoked, non-expired attestation, looks each
attester up in `BlueprintAuditors`, and produces a normalized `0..=100`
number plus per-attestation diagnostics. Without `--auditors-contract` the
score collapses to zero (all attesters treated as anonymous) — the flag
should always be set in production.

### IPFS configuration

Two environment knobs:

```bash
# Generic (Kubo / w3up / any service that accepts multipart/form-data + Bearer):
IPFS_API_URL=https://api.web3.storage/upload   IPFS_API_TOKEN=eyJ...

# Pinata fallback (only used if IPFS_API_URL is unset):
PINATA_JWT=eyJhbGciOi...
```

Pin endpoints must return JSON containing one of `cid`, `Hash`, or
`IpfsHash`; the resulting `ipfs://<cid>` URI is what lands on-chain.

## One-command shipping (`cargo tangle blueprint ship`)

For the common case — "I just landed a feature, build the binary, pin it,
publish the new version, and promote it" — there is `cargo tangle blueprint ship`.
It composes all the above steps into a single interactive flow:

```text
$ cargo tangle blueprint ship
🚀 Shipping blueprint:  blueprintId=7  (/home/me/my-blueprint)
  RPC     https://sepolia.base.org
  Wallet  0xAbC...123 ✓ blueprint owner

? Build a release binary now?              › Yes
  > Building: cargo build --release -p my-blueprint
  sha256       0x9af3…                       (6.21 MB)
? Pin binary to IPFS?                       › Yes
  binaryUri    ipfs://bafyb…
? Publish v? on-chain (blueprint=7)?        › Yes
  ✓ Published v3 (block 184221)
? Promote to active version?                › Yes
  ✓ Promoted v3 (tx 0x…)

── Shipped v3 ─────────────────────────────
  sha256     0x9af3…
  binaryUri  ipfs://bafyb…
  promoted   true
  block      184221
  tx_hash    0x…
```

### Auto-detection

The wizard tries (in order) `--blueprint-id`, `BLUEPRINT_ID=` in
`./settings.env`, then `blueprintId` in `./metadata/blueprint-metadata.json`.
It picks the first non-zero value.

### CI mode

Skip all prompts with `--yes`:

```bash
cargo tangle blueprint ship --yes --pin-ipfs --promote \
  --blueprint-id 7 \
  --binary ./target/release/my-blueprint \
  --attestation-bundle ./dist/sigstore-bundle.json \
  --policy-services 42,43
```

In `--yes` mode the wizard prints one JSON event per phase, with the final
`ship_complete` event carrying the new `version_id`, the publish tx, and
whether `setActiveBinaryVersion` ran. The
[`tangle-network/blueprint/.github/actions/ship-release`](../.github/actions/ship-release)
composite action consumes that JSON to populate its outputs.

### Common flag matrix

| Flag                  | Effect |
|-----------------------|--------|
| `--yes`               | Accept every prompt; switch output to JSON |
| `--dry-run`           | Hash + (optionally) pin + report; submit nothing |
| `--no-build --binary <path>` | Skip `cargo build`; ship a pre-built artifact |
| `--binary-uri ipfs://…`      | Skip IPFS pin; assume URI is already addressable |
| `--no-promote`        | Publish but don't `setActiveBinaryVersion` |
| `--policy-services 42,43`    | Bulk-flip listed services into AUTO policy |
| `--attestation-bundle <path>` | Hash the bundle and store its sha256 on-chain |
| `--attestation-hash 0x…`      | Use a pre-computed `attestationHash` |

## Manual-with-assist (operator local-authz)

For operators on `MANUAL` policy who *want* the manager to swap into specific
versions but don't want to (a) move out of MANUAL on-chain or (b) spend gas
on `ackBinaryVersion`, the manager exposes a local authorization layer.
Pre-authorized swaps still run the full sha256+attestation gate — the only
thing the operator is sidestepping is the audit-trail tx.

```bash
# List what versions the manager sees on-chain (no chain calls — talks to the
# local manager's /upgrades/{service_id}/available).
cargo tangle blueprint service upgrades --service-id 42

# Pre-authorize a single one-shot swap to v3 the next time it becomes effective.
cargo tangle blueprint service upgrade-local --service-id 42 --version-id 3

# Stage a fleet-wide rollout: every version in this list is acceptable.
cargo tangle blueprint service upgrade-whitelist --service-id 42 --versions 2,4,5

# Explicitly skip v3 (canary regression). Reason lands in the manager's audit log.
cargo tangle blueprint service upgrade-skip --service-id 42 --version-id 3 \
  --reason "Failed latency canary; waiting for v4"

# Show what local-authz state the manager is holding for this service.
cargo tangle blueprint service upgrade-authz --service-id 42 --json
```

Manager URL resolution (highest wins): `--manager-url` → `BLUEPRINT_MANAGER_URL`
env → `http://127.0.0.1:9000`. The pre-authorization persists in
`<manager-data-dir>/upgrade-authz/<serviceId>.json`, so the operator can stage
a pin/whitelist during a maintenance window and walk away — a restart of
the manager rebuilds the state cleanly.

## Cloud Deployment

> **Note:** Cloud deployment requires the `remote-providers` feature flag. See [Feature flags](#feature-flags) for installation instructions.

The Tangle CLI supports deploying blueprints to cloud providers for scalable, distributed execution:

### Configure Cloud Provider

```bash
# Configure AWS
cargo tangle cloud configure aws --region us-east-1 --set-default

# Configure GCP
cargo tangle cloud configure gcp --region us-central1

# Configure other providers
cargo tangle cloud configure digitalocean --region nyc1
cargo tangle cloud configure vultr --region ewr
cargo tangle cloud configure azure --region eastus
```

### Cost Estimation

```bash
# Compare costs across all providers
cargo tangle cloud estimate --compare --cpu 4 --memory 16

# Estimate for specific provider with spot pricing
cargo tangle cloud estimate --provider aws --spot --duration 30d

# GPU-enabled instances
cargo tangle cloud estimate --provider gcp --gpu 1 --cpu 8 --memory 32
```

### Deploy Blueprint to Cloud

```bash
# Deploy with remote deployment flag
cargo tangle blueprint deploy tangle --remote --package my_blueprint

# Deploy with specific policy
cargo tangle cloud policy --gpu-providers gcp,aws --cost-providers vultr,do
cargo tangle blueprint deploy tangle --remote --package my_blueprint
```

### Monitor Cloud Deployments

```bash
# Check status of all deployments
cargo tangle cloud status

# Check specific deployment
cargo tangle cloud status --deployment-id dep-abc123

# Terminate deployment
cargo tangle cloud terminate --deployment-id dep-abc123
```

### Preflight and TEE Readiness

```bash
# Validate credentials/provider readiness and print manager bootstrap env
cargo tangle cloud preflight --bootstrap-env

# Validate fail-closed TEE readiness (including cryptographic verifier requirements)
cargo tangle cloud preflight --tee-required --bootstrap-env

# Write bootstrap env directly to file
cargo tangle cloud preflight --tee-required --write-env-file .env.remote
```

### Spawning a Service Runtime

Kick off the blueprint manager using a specific runtime without re-running the full deploy flow:

```bash
cargo tangle blueprint service spawn \
  --http-rpc-url https://rpc.tangle.tools \
  --ws-rpc-url wss://rpc.tangle.tools \
  --keystore-path ./keystore \
  --tangle-contract 0xCf7E... \
  --staking-contract 0xe7f1... \
  --status-registry-contract 0xdC64... \
  --blueprint-id 1 \
  --service-id 2 \
  --spawn-method vm \
  --data-dir ./data \
  --allow-unchecked-attestations
```

The command reuses the same manager wiring as `cargo tangle blueprint run`, so any RPC endpoint + runtime combo works (VM/native/container). For devnet, omit the overrides and the defaults point at the local harness.

## Key Management

All keys are EVM-native. Use the `cargo tangle key` subcommands to handle k256 material:

```bash
# Generate a new operator key
cargo tangle key generate --key-type ecdsa --output ./keystore

# Import an existing hex secret into the keystore
cargo tangle key import --key-type ecdsa \
  --secret 0x0123... \
  --keystore-path ./keystore \
  --protocol tangle

# List local keys
cargo tangle key list --keystore-path ./keystore
```

The keystore supports filesystem, in-memory, and remote HSM backends through `blueprint-keystore`.

## EigenLayer Helpers

EigenLayer support remains available under `cargo tangle blueprint eigenlayer <subcommand>`. Use it to register AVSs, list allocations, or run the EigenLayer manager by supplying the addresses exported from the EigenLayer contracts.

## Operator Status

Inspect the latest heartbeat/online status from `OperatorStatusRegistry`:

```bash
cargo tangle operator status \
  --http-rpc-url https://rpc.tangle.tools \
  --ws-rpc-url wss://rpc.tangle.tools \
  --keystore-path ./keystore \
  --tangle-contract 0xCf7E... \
  --staking-contract 0xe7f1... \
  --status-registry-contract 0xdC64... \
  --blueprint-id 1 \
  --service-id 2 \
  --operator 0xdeadbeef... \
  --json
```

Omit `--operator` to query the locally configured operator. Add `--json` for machine-friendly output (timestamp, raw status code, online boolean).

## Need More?

- End-to-end demos and advanced options live on the [CLI reference](https://docs.tangle.tools/developers/cli/reference).
- For pricing/QoS flows, combine the CLI with `crates/pricing-engine` and the new `OPERATOR_*` env vars described in that README.

```bash
cargo tangle blueprint run \
  --protocol eigenlayer \
  --config ./config.toml
```

The manager will:
- Read all active AVS registrations from `~/.tangle/eigenlayer_registrations.json`
- Spawn a separate blueprint instance for each AVS
- Monitor rewards and slashing events
- Auto-restart failed blueprints

#### 5. Deregister from an AVS

```bash
cargo tangle blueprint eigenlayer deregister \
  --service-manager 0x... \
  --keystore-uri ./keystore
```

### EigenLayer Command Reference

#### `eigenlayer register`

Register with a new EigenLayer AVS.

```bash
cargo tangle blueprint eigenlayer register \
  --config <CONFIG_FILE> \
  --keystore-uri <KEYSTORE_PATH> \
  [--runtime <RUNTIME>] \
  [--verify]
```

**Arguments**:
- `--config`: Path to JSON configuration file
- `--keystore-uri`: Keystore path (default: `./keystore`)
- `--runtime`: Runtime target (`native`, `hypervisor`, `container`) - overrides config file
- `--verify`: Perform on-chain verification (optional)

**Aliases**: `reg`

#### `eigenlayer deregister`

Deregister from an EigenLayer AVS.

```bash
cargo tangle blueprint eigenlayer deregister \
  --service-manager <ADDRESS> \
  --keystore-uri <KEYSTORE_PATH>
```

**Arguments**:
- `--service-manager`: Service manager contract address
- `--keystore-uri`: Keystore path (default: `./keystore`)

**Aliases**: `dereg`

#### `eigenlayer list`

List all registered AVS services.

```bash
cargo tangle blueprint eigenlayer list \
  [--active-only] \
  [--format <FORMAT>]
```

**Arguments**:
- `--active-only`: Show only active registrations
- `--format`: Output format: `table` (default) or `json`

**Aliases**: `ls`

#### `eigenlayer sync`

Synchronize local registrations with on-chain state.

```bash
cargo tangle blueprint eigenlayer sync \
  --http-rpc-url <URL> \
  --keystore-uri <KEYSTORE_PATH> \
  [--settings-file <FILE>]
```

**Arguments**:
- `--http-rpc-url`: HTTP RPC endpoint (default: `http://127.0.0.1:8545`)
- `--keystore-uri`: Keystore path (default: `./keystore`)
- `--settings-file`: Protocol settings file (optional)

### Runtime Targets

Each AVS can specify its execution runtime:

- **`native`** - Bare process (no sandbox)
  - Fastest startup and lowest overhead
  - For testing only - no isolation
  - Direct process execution

- **`hypervisor`** - cloud-hypervisor VM (default)
  - Production-ready VM isolation
  - Strong security boundaries
  - Resource limits enforced
  - Recommended for production

- **`container`** - Docker/Kata containers (Coming Soon)
  - Not yet implemented
  - For now, use `native` for testing or `hypervisor` for production

Set via config file or override via CLI `--runtime` flag.

## Generating Keys from the Command Line

The following command will generate a keypair for a given key type:

```shell
cargo tangle blueprint generate-keys -k <KEY_TYPE> -p <PATH> -s <SURI/SEED> --show-secret
```

where it is optional to include the path, seed, or the show-secret flags.

### Flags

- `-k` or `--key-type`: Required flag. The key type to generate (sr25519, ecdsa, bls_bn254, ed25519, bls381).
- `-p` or `--path`: The path to write the generated keypair to. If not provided, the keypair will be written solely to stdout.
- `-s` or `--seed`: The suri/seed to generate the keypair from. If not provided, a random keypair will be generated.
- `--show-secret`: Denotes that the Private Key should also be printed to stdout. If not provided, only the public key will be printed.

For all of our features for created and using keys, see the [key management](https://docs.tangle.tools/developers/cli/keys) section of our CLI docs.
