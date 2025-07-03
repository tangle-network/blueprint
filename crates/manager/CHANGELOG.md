# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0-alpha.16](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.15...blueprint-manager-v0.3.0-alpha.16) - 2025-07-03

### Added

- *(manager)* feature-gate vm sandbox (#1053)
- automate nftables NAT setup for VMs (#1021)
- quality of service (#968)

## [0.3.0-alpha.15](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.14...blueprint-manager-v0.3.0-alpha.15) - 2025-06-18

### Added

- improve testing source fetcher and error handling (#1052)

## [0.3.0-alpha.14](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.13...blueprint-manager-v0.3.0-alpha.14) - 2025-06-16

### Added

- *(tangle-testing-utils)* support manager bridge in test harness (#1024)
- *(manager)* [**breaking**] allow spawning services without sandboxing (#1022)
- *(manager)* [**breaking**] add manager <-> service bridge (#969)
- *(cargo-tangle)* blueprint create without interaction (#996)
- add auth proxy to blueprint manager (#994)

### Other

- *(blueprint-manager-bridge)* add description (#1044)

## [0.3.0-alpha.13](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.12...blueprint-manager-v0.3.0-alpha.13) - 2025-05-14

### Other

- update Cargo.toml dependencies

## [0.3.0-alpha.12](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.11...blueprint-manager-v0.3.0-alpha.12) - 2025-05-13

### Other

- update Cargo.toml dependencies

## [0.3.0-alpha.11](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.10...blueprint-manager-v0.3.0-alpha.11) - 2025-05-09

### Fixed

- *(manager)* correct platform check (#970)

### Other

- remove gadget references (#967)

## [0.3.0-alpha.10](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.9...blueprint-manager-v0.3.0-alpha.10) - 2025-05-07

### Added

- *(manager)* [**breaking**] support verifying binaries with gh attestations (#938)

### Other

- *(ci)* fixes from attestation pr (#964)

## [0.3.0-alpha.9](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.8...blueprint-manager-v0.3.0-alpha.9) - 2025-05-06

### Other

- updated the following local packages: blueprint-networking, blueprint-runner, blueprint-clients

## [0.3.0-alpha.8](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.7...blueprint-manager-v0.3.0-alpha.8) - 2025-05-01

### Fixed

- *(manager)* handle github sources (#922)
- *(manager)* set correct host for local endpoints (#919)

### Other

- *(runner)* document crate (#920)
- remove unused dependencies (#915)

## [0.3.0-alpha.7](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.6...blueprint-manager-v0.3.0-alpha.7) - 2025-04-21

### Added

- *(manager)* functional container sources (#883)

## [0.3.0-alpha.6](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.5...blueprint-manager-v0.3.0-alpha.6) - 2025-04-18

### Added

- allow for blueprint manage to take a keystore. (#888)
- Blueprint Runner MVP 0 (#881)

## [0.3.0-alpha.5](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.4...blueprint-manager-v0.3.0-alpha.5) - 2025-04-16

### Other

- updated the following local packages: blueprint-clients

## [0.3.0-alpha.4](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.3...blueprint-manager-v0.3.0-alpha.4) - 2025-04-15

### Other

- updated the following local packages: blueprint-keystore, blueprint-runner, blueprint-clients

## [0.3.0-alpha.3](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.2...blueprint-manager-v0.3.0-alpha.3) - 2025-04-14

### Added

- *(manager)* support multiple sources (#866)

## [0.3.0-alpha.2](https://github.com/tangle-network/blueprint/compare/blueprint-manager-v0.3.0-alpha.1...blueprint-manager-v0.3.0-alpha.2) - 2025-04-11

### Added

- *(tangle-extra)* [**breaking**] support multiple blueprint source types (#864)
