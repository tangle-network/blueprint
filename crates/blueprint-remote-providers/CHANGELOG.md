# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.3.0...blueprint-remote-providers-v0.4.0) - 2026-11-08

### Breaking

- `ProviderQuote.measurement` is now `Option<MeasurementReceipt>` (public provenance only) instead of `Option<blueprint_core::resources::ThroughputMeasurement>`.
- Request fingerprints sort JSON object keys, so lease authorization stores written by a 0.3.0 build with serde_json `preserve_order` enabled no longer match on recover.

### Added

- *(remote-providers,cli)* versioned managed GPU JSON process boundary (#1664)

## [0.3.0](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0...blueprint-remote-providers-v0.3.0) - 2026-11-07

### Fixed

- *(remote-providers)* preserve orphan template lease timestamps (#1662)
- *(remote-providers)* [**breaking**] recover owned leases without new stock or rental (#1660)
- *(remote-providers)* use native mode type for Unix ownership checks (#1661)
- *(remote-providers)* ask gcloud for each GCP token instead of reusing one for an hour (#1658)
- *(manager)* scope GPU leases and protect SSH workload delivery (#1651)

### Other

- *(remote-providers)* retire self-generated resource-limit assertions (#1657)

## [0.2.0](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.26...blueprint-remote-providers-v0.2.0) - 2026-11-07

### Added

- *(cargo-tangle)* ship cloud commands in released cargo-tangle and add a cloud-only flavor (#1623)

### Fixed

- *(remote-providers)* refuse unobserved CoreWeave lifecycle success (#1650)
- *(remote-providers)* enforce selected GPU quote price ceilings (#1649)

### Other

- Fill new ResourceSpec fields from defaults in remote-provider tests (#1655)
- Allocate GPU leases with per-lease access and retire them from provider deadlines (#1648)

## [0.2.0-alpha.26](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.25...blueprint-remote-providers-v0.2.0-alpha.26) - 2026-11-07

### Added

- *(remote-providers)* add OVH Public Cloud as a cloud provider (#1640)

### Fixed

- *(remote-providers)* reach GCP with the operator's Google login (#1642)

### Other

- Restore fmt and clippy on the GPU provider HTTP paths (#1647)
- Ship native GPU sellers through one managed lease boundary (#1643)
- Share GPU resource requests and preserve scheduling metadata (#1630)

## [0.2.0-alpha.25](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.24...blueprint-remote-providers-v0.2.0-alpha.25) - 2026-11-06

### Added

- *(core)* publish the canonical GPU catalog (#1622)

### Other

- *(remote)* restore supported clippy checks (#1629)
- Use one live VM price source and reject unknown estimates (#1626)

## [0.2.0-alpha.24](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.23...blueprint-remote-providers-v0.2.0-alpha.24) - 2026-11-03

### Other

- updated the following local packages: blueprint-pricing-engine

## [0.2.0-alpha.23](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.22...blueprint-remote-providers-v0.2.0-alpha.23) - 2026-11-02

### Other

- updated the following local packages: blueprint-pricing-engine

## [0.2.0-alpha.22](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.21...blueprint-remote-providers-v0.2.0-alpha.22) - 2026-10-27

### Fixed

- *(deps)* make main build again on Alloy 2, rand 0.10 and dynosaur 0.3 (#1605)

## [0.2.0-alpha.21](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.20...blueprint-remote-providers-v0.2.0-alpha.21) - 2026-10-13

### Other

- updated the following local packages: blueprint-pricing-engine

## [0.2.0-alpha.20](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.19...blueprint-remote-providers-v0.2.0-alpha.20) - 2026-10-07

### Other

- updated the following local packages: blueprint-pricing-engine

## [0.2.0-alpha.19](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.18...blueprint-remote-providers-v0.2.0-alpha.19) - 2026-10-02

### Other

- updated the following local packages: blueprint-pricing-engine

## [0.2.0-alpha.18](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.17...blueprint-remote-providers-v0.2.0-alpha.18) - 2026-10-02

### Other

- updated the following local packages: blueprint-pricing-engine

## [0.2.0-alpha.17](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.16...blueprint-remote-providers-v0.2.0-alpha.17) - 2026-10-01

### Other

- updated the following local packages: blueprint-keystore, blueprint-pricing-engine

## [0.2.0-alpha.16](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.15...blueprint-remote-providers-v0.2.0-alpha.16) - 2026-10-01

### Other

- updated the following local packages: blueprint-pricing-engine, blueprint-keystore

## [0.2.0-alpha.15](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.14...blueprint-remote-providers-v0.2.0-alpha.15) - 2026-09-29

### Other

- updated the following local packages: blueprint-pricing-engine

## [0.2.0-alpha.13](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.12...blueprint-remote-providers-v0.2.0-alpha.13) - 2026-08-02

### Other

- remove EigenLayer support and fix two pre-existing build breakages (#1479)

## [0.2.0-alpha.12](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.11...blueprint-remote-providers-v0.2.0-alpha.12) - 2026-07-24

### Other

- updated the following local packages: blueprint-pricing-engine

## [0.2.0-alpha.11](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.10...blueprint-remote-providers-v0.2.0-alpha.11) - 2026-07-08

### Added

- adopt tnt-core 0.19 bindings (draft — blocked on publishing tnt-core-bindings 0.19.0) (#1467)

## [0.2.0-alpha.10](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.9...blueprint-remote-providers-v0.2.0-alpha.10) - 2026-07-03

### Other

- updated the following local packages: blueprint-pricing-engine

## [0.2.0-alpha.9](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.8...blueprint-remote-providers-v0.2.0-alpha.9) - 2026-06-13

### Added

- *(remote-providers)* verify TEE attestations fail closed (#1453)

## [0.2.0-alpha.8](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.7...blueprint-remote-providers-v0.2.0-alpha.8) - 2026-05-21

### Other

- update Cargo.toml dependencies

## [0.2.0-alpha.7](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.6...blueprint-remote-providers-v0.2.0-alpha.7) - 2026-05-20

### Other

- updated the following local packages: blueprint-core, blueprint-std, blueprint-keystore, blueprint-pricing-engine

## [0.2.0-alpha.6](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.5...blueprint-remote-providers-v0.2.0-alpha.6) - 2026-05-17

### Other

- updated the following local packages: blueprint-keystore, blueprint-pricing-engine

## [0.2.0-alpha.4](https://github.com/tangle-network/blueprint/compare/blueprint-remote-providers-v0.2.0-alpha.3...blueprint-remote-providers-v0.2.0-alpha.4) - 2026-04-22

### Fixed

- *(tangle)* cap log queries and add gas fallbacks (#1382)
