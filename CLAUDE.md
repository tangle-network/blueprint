# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Tangle Network Blueprint SDK - a Rust workspace for building blockchain services ("blueprints") on Tangle Network, EigenLayer, and EVM networks. The project is in alpha stage with 40+ crates organized in a modular architecture.

## Essential Commands

### Build & Test
```bash
# Build entire workspace
cargo build

# Run all tests
cargo test

# Run tests with nextest (preferred in CI)
cargo nextest run --profile ci

# Format code
cargo fmt

# Format check in CI parity mode (required before push)
cargo +nightly-2026-02-24 fmt -- --check

# Lint code
cargo clippy -- -D warnings
cargo clippy --tests --examples -- -D warnings
```

### CLI Tool
```bash
# Install the CLI
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint --force

# Create new blueprint
cargo tangle blueprint create --name <blueprint_name>

# Deploy to devnet (auto-starts local testnet)
cargo tangle blueprint deploy tangle --network devnet

# Deploy to testnet/mainnet using a definition manifest
cargo tangle blueprint deploy tangle --network testnet --definition ./path/to/definition.json

# Generate keys
cargo tangle blueprint generate-keys -k <KEY_TYPE> -p <PATH> -s <SURI/SEED>
```

## Architecture

### Core Abstraction: Job System
The blueprint SDK centers around a job system that provides:
- Multi-network execution contexts (Tangle, EigenLayer, EVM)
- Dynamic routing and scheduling via `blueprint-router`
- Protocol-specific execution via `blueprint-runner`
- Network management via `blueprint-manager`

### Workspace Structure
```
cli/                    # cargo-tangle CLI tool
crates/
├── blueprint-sdk/      # Main SDK re-export crate
├── blueprint-core/     # Job system primitives
├── blueprint-runner/   # Job execution engine
├── blueprint-router/   # Dynamic job scheduling
├── blueprint-manager/  # Network connection layer
├── blueprint-clients/  # Network-specific clients (Tangle, EigenLayer, EVM)
├── blueprint-crypto/   # Multi-scheme cryptography (BLS, BN254, Ed25519, K256, Sr25519)
├── blueprint-networking/ # P2P with MPC protocol extensions
├── blueprint-testing-utils/ # Test utilities for different networks
└── blueprint-stores/   # Local database implementations
examples/               # Example blueprints (incredible-squaring)
```

### Key Architectural Patterns

1. **Meta-crates Pattern**: Major functionality is exposed through meta-crates that re-export component crates
2. **Multi-Network Abstraction**: Common interfaces with network-specific implementations in separate crates
3. **Hardware Wallet Support**: Built-in support for Ledger and remote signing (AWS KMS, GCP)
4. **Test Isolation**: Certain crates require serial test execution due to resource conflicts

## Development Constraints

### Rust Configuration
- Version: 1.88 (2024 edition)
- Workspace resolver: v3
- Rustfmt in CI is pinned to: `nightly-2026-02-24`

### Linting Rules
- Clippy pedantic enabled with specific allowances
- Deny: rust_2018_idioms, trivial_casts, unused_import_braces
- Custom doc-valid-idents including "EigenLayer"

### Serial Test Crates
These require `--test-threads=1` or nextest serial execution:
- `blueprint-tangle-testing-utils`
- `blueprint-client-evm`
- `blueprint-tangle-extra`
- `blueprint-networking`
- `blueprint-qos`
- `cargo-tangle`

### External Dependencies
- **Substrate**: sp-core, sp-runtime (v34-39.x)
- **Alloy**: EVM interaction (v1.8.x)
- **libp2p**: P2P networking (v0.55)
- **Eigensdk**: EigenLayer integration (v0.5)
- **Foundry**: Smart contract compilation

### Workspace Dependency Hygiene

**Never add a `[dev-dependencies]` entry using `workspace = true` on any workspace member that participates (directly or transitively) in a publish cycle with the current crate.**

The umbrella case (`blueprint-sdk`) is the obvious one, but the same trap fires on *any* internal cycle. Cycles we hit on `0.2.0-alpha.2` after the umbrella fix landed:

- `blueprint-client-tangle` → DEV `blueprint-anvil-testing-utils` → `blueprint-runner` → `blueprint-client-tangle` (optional)
- `blueprint-client-evm` → DEV `blueprint-anvil-testing-utils` → … → `blueprint-client-evm`
- `blueprint-client-eigenlayer` → DEV `blueprint-eigenlayer-testing-utils` → `blueprint-runner` → `blueprint-client-eigenlayer`
- `blueprint-tangle-extra` → DEV `blueprint-anvil-testing-utils` → `blueprint-client-tangle` → `blueprint-tangle-extra`
- `blueprint-qos` → DEV `blueprint-anvil-testing-utils` → `blueprint-runner` → `blueprint-qos`
- `blueprint-eigenlayer-extra` → DEV `blueprint-eigenlayer-testing-utils` → `blueprint-runner` (eigenlayer feature) → `blueprint-eigenlayer-extra`
- `blueprint-pricing-engine`, `blueprint-manager`, `cargo-tangle` — same pattern via testing-utils

In every case, a production crate's `[dev-dependencies]` entry pulled in a testing utility that reaches back into the production crate via the runner/clients chain. The fix is exactly the same as the umbrella case: convert the dev-dep to path-only.

The workspace declares each crate as `name = { version = "...", path = "./crates/..." }` — the `version + path` combo means cargo will write the version constraint into a published crate's manifest and then resolve it against crates.io at publish time. Because the testing-utils → runner → client/qos/extra chain creates back-edges, a workspace-style dev-dep on a testing crate from a production crate forms a publish-time circular dependency:

```
cargo publish blueprint-core 0.2.0-alpha.X
  → resolves dev-dep `blueprint-sdk = "^0.2.0-alpha.X"` against crates.io
  → fails because blueprint-sdk 0.2.0-alpha.X isn't published yet
  → and it can't be, because blueprint-sdk depends on blueprint-core
```

This deadlock is what kept the entire `0.2.0-alpha.2` release stuck on crates.io with only `0.2.0-alpha.1` published until the fix landed.

**The rule:** if a crate's tests, doctests, or trybuild fixtures need access to another workspace crate that's part of the publish set, use a **path-only** dev-dep that bypasses the workspace dep table entirely:

```toml
# correct: stripped from the published manifest, still resolves locally for cargo test
[dev-dependencies]
blueprint-anvil-testing-utils = { path = "../testing-utils/anvil" }
blueprint-sdk = { path = "../sdk", features = ["std"] }
```

```toml
# wrong: cargo publish carries the version constraint into the published manifest
# and then deadlocks on the workspace cycle
[dev-dependencies]
blueprint-anvil-testing-utils = { workspace = true }
blueprint-sdk = { workspace = true, features = ["std"] }
```

Path-only dev-deps still respect features. If a test doesn't actually need the dep, just delete it — many of the historical entries were vestigial.

**Detection:** if `gh run view <publish-crates run> --log` shows `failed to select a version for the requirement \`blueprint-* = "^X.Y.Z-alpha.N"\` ... candidate versions found which didn't match: X.Y.Z-alpha.{N-1}`, this is the symptom. The fix is always: convert the dev-dep to path-only.

**Audit command:** before bumping versions and running `publish-crates`, sanity-check the publish surface:

```sh
cargo metadata --no-deps --format-version 1 \
  | python3 -c 'import json,sys; m=json.load(sys.stdin); ws={p["name"] for p in m["packages"]}; \
[print(p["name"], "→", d["name"]) for p in m["packages"] if p["name"] in ws \
 for d in p["dependencies"] if d.get("kind")=="dev" and d["name"] in ws \
 and not d["name"].startswith("workspace-hack") and d.get("req","*")!="*"]'
```

Any line printed is a workspace dev-dep that retained its version constraint in the published manifest. Confirm it does not form a cycle (target's transitive regular deps must not lead back to the source crate); if it does, convert to path-only.

## Harness Process (Required)

Use this process for non-trivial changes:

1. Define behavior contract first: current behavior, intended behavior, invariants, fail-open/fail-closed choice.
2. Reproduce with the smallest harness/test before implementing.
3. Implement the change and add negative-path assertions.
4. Verify with crate-scoped commands first, then broader workspace checks as needed.
5. Document operator/customer/developer impact plus migration/rollback notes in the PR.

Reference:
- `docs/engineering/HARNESS_ENGINEERING_PLAYBOOK.md`
- `docs/engineering/HARNESS_ENGINEERING_SPEC.md`

## PR Quality Gate

PRs targeting `main` are validated by `.github/workflows/pr-quality-gate.yml`. The PR body **must** contain these markdown sections (exactly as `## Section Name`):

1. `## Summary` — bullet-point description of changes
2. `## Change Class` — must include `- Selected class: Class X` and `- Why this class: ...`
3. `## Behavior Contract` — describe what changed behaviorally (new errors, changed defaults, etc.)
4. `## Risk And Scope` — blast radius, mitigation, risk assessment
5. `## Verification` — at least one fenced code block with test/build commands
6. `## Harness Evidence` — which tests cover the changes and their status
7. `## Checklist` — markdown checklist of quality items

### Change Class Rules (from `.github/pr-quality-gate.toml`)
- **Class D** (required) when changing files under:
  - `crates/manager/src/protocol/`, `crates/manager/src/rt/container/`, `crates/manager/src/sources/`
  - `crates/clients/tangle/src/`, `crates/tee/src/`, `cli/src/command/deploy/`
- **Class C** is auto-promoted for multi-crate changes or CLI+crate changes
- **Docs-only** PRs (only `*.md`, `docs/**`, `.github/**`) skip validation

### Pre-push Hook
The local `.git/hooks/pre-push` runs: (1) `cargo fmt -- --check`, (2) clippy on changed crates, (3) tests on changed crates, (4) optional security audit. All checks must pass before push succeeds.

### PR Body Source
The quality gate reads the PR body from `GITHUB_EVENT_PATH` (the event payload), **not** from the live API. This means:
- Editing the PR body after push does **not** update already-running checks
- To re-evaluate after a body edit, push a new commit (even empty) to trigger a fresh `synchronize` event
- `gh run rerun` replays the old event payload and will see the old body
