# Tangle CLI

Create, run, and operate blueprints on Tangle EVM.

## Overview

The CLI bundles every workflow needed for the EVM-only SDK:

- `cargo tangle blueprint create` scaffolds a new blueprint from the templates.
- `cargo tangle blueprint run --protocol tangle` boots a manager connected to the Tangle v2 contracts.
- `cargo tangle blueprint register-tangle` registers an operator against `ITangle`, `MultiAssetDelegation`, and `OperatorStatusRegistry`.
- `cargo tangle key *` manages local and remote k256 keys via `blueprint-keystore`.

## Installation

> Linux, macOS, and Windows (WSL2) are supported.

```bash
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint --force
```

### Feature flags

Source installs enable `full` by default, including blueprint development,
registration, operator and key commands. Release archives and the shell
installer also enable `remote-providers`, which adds `cargo tangle cloud`;
add it to a source install with `--features remote-providers`. `vm-debug`
adds the Linux VM sandbox.

The release also includes `cargo-tangle-cloud-<target>.tar.xz` and its
`.sha256` file for native Linux x86_64 and aarch64. These archives contain
`cargo-tangle` built from the same source with `--no-default-features --features
cloud-only`. They expose only the cloud commands, with the same providers as the
standard archives, and have no Anvil, testcontainers or cargo-generate in the
production dependency graph. Chain and key libraries still link, because the
provider and pricing crates depend on them. Standard archives carry the full CLI
with cloud commands. Install the cloud-only flavor from source:

```bash
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint \
  --no-default-features --features cloud-only --force
```

Verify the downloaded `.sha256` file before extracting the archive.

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
TANGLE_CONTRACT=0x0165878A594ca255338adfa4d48449f69242Eb8F \
STAKING_CONTRACT=0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9 \
STATUS_REGISTRY_CONTRACT=0x2bdCC0de6bE1f7D2ee689a0342D76F52E8EFABa3 \
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
  --tangle-contract 0x0165878A594ca255338adfa4d48449f69242Eb8F \
  --staking-contract 0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9 \
  --status-registry-contract 0x2bdCC0de6bE1f7D2ee689a0342D76F52E8EFABa3 \
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

## Cloud Commands

> **Note:** Release archives include cloud commands; source installs need the `remote-providers` feature. See [Feature flags](#feature-flags).

The cloud commands configure providers, quote live prices, and inspect and terminate deployments recorded in the deployment tracker. No CLI command allocates capacity, updates a running deployment or reads its logs, and `cargo tangle blueprint run` cannot enable the manager's remote deployment path.

### Configure Cloud Provider

`configure` saves a region and default provider. Credentials come from the environment, and for AWS also from `~/.aws/credentials`. `configure` names the variables a provider needs; it never stores a credential. A token can also come from the Compute Engine metadata server for GCP, and from a managed identity or `az login` for Azure.

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

# Estimate a configured provider using its live on-demand price
cargo tangle cloud estimate --provider aws --duration 30d

# GPU-enabled instances
cargo tangle cloud estimate --provider gcp --gpu 1 --cpu 8 --memory 32
```

### Deployment Policy

```bash
# Rank providers for the manager's remote deployments
cargo tangle cloud policy --gpu-providers gcp,aws --cost-providers vultr,do
```

### Monitor Cloud Deployments

`status` and `terminate` read the tracker at `TANGLE_DEPLOYMENT_TRACKER_PATH`, by default `~/.tangle/remote_deployments`. Both reach each provider in the region its record names, and a provider that is missing a variable affects only its own deployments. `status` shows `unknown` when the provider cannot answer, and `status <id>` also says why. `terminate` exits non-zero if any deployment is not confirmed terminated, and that deployment stays tracked.

```bash
# Check status of all deployments
cargo tangle cloud status

# Check specific deployment
cargo tangle cloud status dep-abc123

# Terminate one deployment
cargo tangle cloud terminate dep-abc123

# Terminate every tracked deployment
cargo tangle cloud terminate --all
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

- End-to-end demos and advanced options live on the [CLI reference](https://docs.tangle.tools/developers/cli/reference).

## Managed GPU JSON boundary

With `remote-providers` (including `cloud-only`), `cloud catalog/quote/create/recover/status/list/destroy/state-init --json`
uses the schema version exported by `cloud::SCHEMA_VERSION`. This source contract
is incompatible with the earlier central draft; an installed binary and its
consumer must be qualified and pinned together. Legacy `cloud status [service]`,
`cloud list`, configure and tracker termination retain their existing meanings.
JSON status/list cannot be combined with legacy status IDs or watch flags.

`catalog --json` is offline and reads no stdin or inherited credential/config
settings. Its public output includes the canonical hardware catalog, actual
managed credential variants, generated request/response schemas, `schema_sha256`
and the input/output ceiling owned by `cloud::MAX_JSON_BYTES`. Consumers generate
provider/envelope/error types from that pinned output and fail their maintained
build when regenerated output/digest differs. It is not a second hardware or
provider capability table. A descriptor does not promise stock or support for
every request shape.

Other JSON verbs accept exactly one bounded document on stdin, followed by EOF,
and emit one bounded public envelope on stdout. The parser rejects duplicate and
unknown typed fields, excessive nesting, malformed numbers and overflow. Request
schemas describe deserialization, so `ResourceSpec.env` remains input even though
it is omitted from serialized state. Errors use the SDK's closed
`ProviderErrorCode`, fixed public text and a nonzero exit. JSON invocations do not
initialize inherited tracing; provider bodies and parser errors are not reflected.
Put credentials in private stdin/vault custody, never arguments or public files.

Requests use a tagged `ProviderCredentials` array. Quote/create/recover carry the
canonical `spec.ownership` and `spec.lease_duration_secs`; there is no second
ownership or TTL field. Status/destroy carry one `identity`, `provider`, and a
provider ID (optional for destroy). Destroy retires only an identity this
`state_dir` authorized under the same account: listed objects of any other
identity, including another store's lease on the same account, are refused with
`conflict` and left in place. It waits for an in-flight create of the identity
in the same `state_dir`. List carries `namespace` and lists the full configured account roster; it has no
provider filter. All operations after quote require `state_dir`; use the exported
request schemas for the complete field contract. Quote returns the complete SDK
`QuoteSet`, including failures beside ranked offers. Keep its candidate string
unchanged and preserve the exact original spec, caller public key and account
roster before create. New quote pricing source is the actual HTTP(S) service origin that returned the
offer, without endpoint paths/userinfo/query/fragment. Custom broker origins stay
custom; no default vendor URL or fallback rate is substituted. Invalid or opaque
sources fail before public quote output.

Create opens existing state and calls state-attached `create_managed`. Its SDK
reservation preserves original quote, account binding, key and deadline before
POST. For an attempted identity, repeated create recovers the original authority
and ignores a replacement candidate; it never re-prices or rents again. A fresh
invalid candidate is refused. `recover` is no-create and an absent result never
permits re-renting that attempted identity. Keep a caller-owned SSH public key in
the original spec; this public process boundary never exports a generated private
key. Workload environment must be rehydratable under the caller's private custody
for identical recovery; SDK state does not serialize it.

`state-init` is an explicit, resource-free enrollment of an existing empty,
owner-only, absolute Unix directory without symlink components. Normal verbs
only open it and never repair missing/corrupt state. Central orchestration must
own persistent placement, enrollment markers and a single coherent filesystem
context across calls/restarts. Temporary HOME is not authority. Carry SDK state,
caller records and keys together after stopping the previous writer; independent
copies are not coordinated by local file locks. Keep lost state visible for
operator reconciliation rather than initializing a replacement store.

Non-default QoS is deliberately unsupported rather than silently applied.
Raw `TANGLE_GPU_WORKER_TOKEN` env is refused, not removed or replaced by an
invented bootstrap variable. Central still owns its maintained one-time bootstrap.

Measured ranking uses the full original private recipe. New candidates clone the
selected arm with only its measurement copies cleared, while the fingerprint
commits the untouched original spec/env/measurements. The SDK validates those
selectors against the canonical original arms. The public `MeasurementReceipt`
contains only audit hashes, timestamp, counters, concurrency and TP/replica counts;
model/revision/image/raw-result locators are not fields of that response type.
It cannot be replayed as a private execution recipe. This changes the Rust
`ProviderQuote.measurement` API and requires a containing minor release/adoption,
not a patch compatibility claim. The request schema still accepts full original
private measurement rows and the response schema derives the new receipt.

The SDK reads existing version1 authorization records/full-arm candidates without
rewriting them, checking legacy full-arm equality and original key/account/spec
and deadline. Newly normalized measured candidates need the new validator;
downgrading them to old readers is not supported. Candidate encoding is reversible.
Public identifiers, origins and digests remain trusted public metadata, not a
general secret-string detector, anonymization guarantee or measurement-truth proof.

Public resources omit raw native instance metadata, env, keys and authorizations,
retain distinct worker/endpoint/SSH URLs and require reported expiry for every
present instance. `status` is canonical SDK `InstanceStatus`, with a separate
`preempted` boolean; `Running` says capacity is running, not that a workload is
ready. `created_at` is retained under its canonical name and is not a measured
billing/service start. Central must persist the original absolute expiry in its
existing hard-expiry renewal guard, refuse missing/invalid expiry and never
replace it with a later observation. Preserve original spec separately from
mutable renewal policy. No SDK extension API or second TTL engine is introduced.
Provider observations and inventory remain unverified hardware facts.

The maintained process/HTTP qualification requires an actual SDK827-built legacy
producer and its reviewed source/artifact receipt via `D12_LEGACY_PRODUCER` and
`D12_LEGACY_PRODUCER_RECEIPT`. Absence fails the integration parent; it never skips
legacy compatibility. The test helper uses old public APIs to create an actual
fake-provider full-arm quote/private state, then the new binary recovers/retires
it with the same POST count and unchanged record/candidate bytes. This is a source
compatibility fixture, not an installed or paid-provider proof.
