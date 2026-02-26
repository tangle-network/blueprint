# Blueprint Crates

This directory contains the Rust crates that make up the Blueprint SDK workspace.

## Core runtime

- [`blueprint-sdk`](./sdk): umbrella crate for most Blueprint applications
- [`blueprint-core`](./core): core job and runtime primitives
- [`blueprint-router`](./router): job ID to handler dispatch layer
- [`blueprint-runner`](./runner): runtime loop and orchestration

## Trigger and ingress crates

- [`blueprint-producers-extra`](./producers-extra): protocol-agnostic producers (for example cron)
- [`blueprint-webhooks`](./webhooks): authenticated HTTP webhook ingress
- [`blueprint-x402`](./x402): paid HTTP ingress via x402 settlement

## Protocol and integration crates

- [`blueprint-tangle-extra`](./tangle-extra): Tangle protocol integrations
- [`blueprint-evm-extra`](./evm-extra): EVM integration helpers
- [`blueprint-eigenlayer-extra`](./eigenlayer-extra): EigenLayer integration helpers
- [`blueprint-clients`](./clients): client metapackage and protocol client crates

## Security, ops, and support

- [`blueprint-auth`](./auth): auth proxy and request auth primitives
- [`blueprint-qos`](./qos): heartbeat/metrics/logging QoS surfaces
- [`blueprint-manager`](./manager): manager runtime orchestration
- [`blueprint-manager-bridge`](./manager/bridge): manager/service bridge types
- [`blueprint-keystore`](./keystore): signing and key management
- [`blueprint-pricing-engine`](./pricing-engine): RFQ and pricing server components

## Utility families

- [`blueprint-crypto`](./crypto): crypto metapackage and algorithm crates
- [`blueprint-metrics`](./metrics): metrics metapackage and providers
- [`blueprint-stores`](./stores): storage provider crates
- [`blueprint-testing-utils`](./testing-utils): testing metapackage and helpers
- [`blueprint-chain-setup`](./chain-setup): local chain setup helpers
- [`blueprint-build-utils`](./build-utils): build-time helpers
- [`blueprint-std`](./std): shared std exports/utilities
- [`blueprint-macros`](./macros): proc-macro support crates

Each crate has a local `README.md` with crate-specific scope and links.
