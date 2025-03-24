# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/tangle-network/blueprint/releases/tag/blueprint-tangle-extra-v0.1.0) - 2025-03-24

### Added

- impl IntoTangleFieldTypes for Void ([#780](https://github.com/tangle-network/blueprint/pull/780))
- add support for `request_params` and `registration_params` ([#769](https://github.com/tangle-network/blueprint/pull/769))
- *(testing-utils)* support per-node contexts ([#739](https://github.com/tangle-network/blueprint/pull/739))
- impl `IntoTangleFieldTypes` for `TangleResult` in `Result` and `Option` ([#729](https://github.com/tangle-network/blueprint/pull/729))
- *(tangle-extra)* add `List` and `Optional` extractors ([#726](https://github.com/tangle-network/blueprint/pull/726))
- core extractors ([#36](https://github.com/tangle-network/blueprint/pull/36))
- EVM Consumers ([#30](https://github.com/tangle-network/blueprint/pull/30))
- re-integrate the blueprint configs ([#28](https://github.com/tangle-network/blueprint/pull/28))
- debug macros, sdk crate ([#23](https://github.com/tangle-network/blueprint/pull/23))
- add blueprint! macro ([#21](https://github.com/tangle-network/blueprint/pull/21))
- reflection ([#16](https://github.com/tangle-network/blueprint/pull/16))
- add Tangle job layer
- add Tangle result consumer
- allow empty call returns
- EVM impl, contract and block events. ([#3](https://github.com/tangle-network/blueprint/pull/3))

### Fixed

- many broken doc links ([#779](https://github.com/tangle-network/blueprint/pull/779))
- module job_definition is private error ([#775](https://github.com/tangle-network/blueprint/pull/775))
- *(testing-utils)* allow setting harness context after creation ([#733](https://github.com/tangle-network/blueprint/pull/733))
- misc fixes ([#709](https://github.com/tangle-network/blueprint/pull/709))
- finish migration of new job system ([#699](https://github.com/tangle-network/blueprint/pull/699))

### Other

- Implement the crate naming conventions ([#759](https://github.com/tangle-network/blueprint/pull/759))
- move `gadget-blueprint-serde` into `blueprint-tangle-extra` ([#757](https://github.com/tangle-network/blueprint/pull/757))
- remove proc macro core ([#716](https://github.com/tangle-network/blueprint/pull/716))
- remove utils crates ([#714](https://github.com/tangle-network/blueprint/pull/714))
- *(clippy)* use workspace lints globally ([#710](https://github.com/tangle-network/blueprint/pull/710))
- rustdoc and READMEs for crates ([#27](https://github.com/tangle-network/blueprint/pull/27))
- split Tangle incredible-squaring into workspace
