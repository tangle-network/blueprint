# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0-alpha.19](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.18...cargo-tangle-v0.4.0-alpha.19) - 2025-07-07

### Fixed

- typo (#1075)

## [0.4.0-alpha.18](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.17...cargo-tangle-v0.4.0-alpha.18) - 2025-07-03

### Added

- add rustls as default crypto provider (#1070)
- *(manager)* feature-gate vm sandbox (#1053)
- automate nftables NAT setup for VMs (#1021)
- quality of service (#968)

### Fixed

- use blueprint-core logging macros (#1062)
- remove git dependencies (#1056)
- gracefully handle AlreadyOperator error in blueprint registration (#1055)

## [0.4.0-alpha.17](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.16...cargo-tangle-v0.4.0-alpha.17) - 2025-06-18

### Added

- improve testing source fetcher and error handling (#1052)

## [0.4.0-alpha.16](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.15...cargo-tangle-v0.4.0-alpha.16) - 2025-06-16

### Added

- *(tangle-testing-utils)* support manager bridge in test harness (#1024)
- *(manager)* [**breaking**] allow spawning services without sandboxing (#1022)
- *(manager)* [**breaking**] add manager <-> service bridge (#969)
- *(cargo-tangle)* blueprint create without interaction (#996)
- add support for request params (#984)

### Fixed

- *(cargo-tangle)* missing template defaults (#999)

## [0.4.0-alpha.15](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.14...cargo-tangle-v0.4.0-alpha.15) - 2025-05-14

### Other

- update Cargo.toml dependencies

## [0.4.0-alpha.14](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.13...cargo-tangle-v0.4.0-alpha.14) - 2025-05-13

### Other

- update Cargo.toml dependencies

## [0.4.0-alpha.13](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.12...cargo-tangle-v0.4.0-alpha.13) - 2025-05-09

### Other

- remove gadget references (#967)

## [0.4.0-alpha.12](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.11...cargo-tangle-v0.4.0-alpha.12) - 2025-05-07

### Added

- *(manager)* [**breaking**] support verifying binaries with gh attestations (#938)

### Other

- *(ci)* fixes from attestation pr (#964)

## [0.4.0-alpha.11](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.10...cargo-tangle-v0.4.0-alpha.11) - 2025-05-06

### Other

- updated the following local packages: blueprint-runner, blueprint-clients, blueprint-contexts, blueprint-chain-setup, blueprint-manager, blueprint-testing-utils, blueprint-testing-utils

## [0.4.0-alpha.10](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.9...cargo-tangle-v0.4.0-alpha.10) - 2025-05-01

### Added

- *(pricing-engine)* finalized pricing engine implementation (#904)

### Fixed

- *(cargo-tangle)* properly encode optional fields (#921)

### Other

- remove unused dependencies (#915)

## [0.4.0-alpha.9](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.8...cargo-tangle-v0.4.0-alpha.9) - 2025-04-22

### Fixed

- *(cargo-tangle)* stop always choosing eigenlayer bls template (#909)

## [0.4.0-alpha.8](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.7...cargo-tangle-v0.4.0-alpha.8) - 2025-04-21

### Added

- impl pricing engine diff network updates (#890)
- *(manager)* functional container sources (#883)

## [0.4.0-alpha.7](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.6...cargo-tangle-v0.4.0-alpha.7) - 2025-04-18

### Added

- allow for blueprint manage to take a keystore. (#888)
- Blueprint Runner MVP 0 (#881)

## [0.4.0-alpha.6](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.5...cargo-tangle-v0.4.0-alpha.6) - 2025-04-16

### Other

- updated the following local packages: blueprint-clients, blueprint-contexts, blueprint-chain-setup, blueprint-manager, blueprint-testing-utils, blueprint-testing-utils

## [0.4.0-alpha.5](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.4...cargo-tangle-v0.4.0-alpha.5) - 2025-04-15

### Fixed

- *(cargo-tangle)* construct a keystore for blueprint run (#882)

## [0.4.0-alpha.4](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.3...cargo-tangle-v0.4.0-alpha.4) - 2025-04-15

### Other

- updated the following local packages: blueprint-keystore, blueprint-keystore, blueprint-runner, blueprint-clients, blueprint-contexts, blueprint-chain-setup, blueprint-manager, blueprint-testing-utils, blueprint-testing-utils

## [0.4.0-alpha.3](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.2...cargo-tangle-v0.4.0-alpha.3) - 2025-04-14

### Other

- update Cargo.lock dependencies

## [0.4.0-alpha.2](https://github.com/tangle-network/blueprint/compare/cargo-tangle-v0.4.0-alpha.1...cargo-tangle-v0.4.0-alpha.2) - 2025-04-11

### Added

- *(tangle-extra)* [**breaking**] support multiple blueprint source types (#864)

## [0.4.0-alpha.1](https://github.com/tangle-network/blueprint/releases/tag/blueprint-metrics-v0.1.0-alpha.1) - 2025-04-08

### Other

- Initial release
