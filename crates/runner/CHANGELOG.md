# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0-alpha.1](https://github.com/tangle-network/blueprint/releases/tag/blueprint-runner-v0.1.0-alpha.1) - 2025-04-08

### Added

- increase eigen client methods ([#697](https://github.com/tangle-network/blueprint/pull/697))
- add targets for some logs ([#799](https://github.com/tangle-network/blueprint/pull/799))
- *(cargo-tangle)* tangle command workflow  ([#703](https://github.com/tangle-network/blueprint/pull/703))
- EVM Consumers ([#30](https://github.com/tangle-network/blueprint/pull/30))
- add tracing to router calls
- re-integrate the blueprint configs ([#28](https://github.com/tangle-network/blueprint/pull/28))
- add blueprint! macro ([#21](https://github.com/tangle-network/blueprint/pull/21))
- reflection ([#16](https://github.com/tangle-network/blueprint/pull/16))
- add Tangle result consumer
- support jobs that always run
- EVM impl, contract and block events. ([#3](https://github.com/tangle-network/blueprint/pull/3))

### Fixed

- *(blueprint-runner)* update job call handling to use async tasks ([#813](https://github.com/tangle-network/blueprint/pull/813))
- *(runner)* stop blocking on job calls ([#809](https://github.com/tangle-network/blueprint/pull/809))
- visibility of networking fields in `BlueprintSettings` ([#800](https://github.com/tangle-network/blueprint/pull/800))
- many broken doc links ([#779](https://github.com/tangle-network/blueprint/pull/779))
- hide background service warnings if none are registered ([#753](https://github.com/tangle-network/blueprint/pull/753))
- finish migration of new job system ([#699](https://github.com/tangle-network/blueprint/pull/699))

### Other

- release ([#844](https://github.com/tangle-network/blueprint/pull/844))
- release ([#843](https://github.com/tangle-network/blueprint/pull/843))
- set crates to pre-release versions ([#842](https://github.com/tangle-network/blueprint/pull/842))
- pin workspace to 1.85 ([#821](https://github.com/tangle-network/blueprint/pull/821))
- clippy ([#812](https://github.com/tangle-network/blueprint/pull/812))
- remove Sync bound for producers and consumers ([#791](https://github.com/tangle-network/blueprint/pull/791))
- Implement the crate naming conventions ([#759](https://github.com/tangle-network/blueprint/pull/759))
- remove utils crates ([#714](https://github.com/tangle-network/blueprint/pull/714))
- *(clippy)* use workspace lints globally ([#710](https://github.com/tangle-network/blueprint/pull/710))
- update alloy & eigensdk ([#35](https://github.com/tangle-network/blueprint/pull/35))
- rustdoc and READMEs for crates ([#27](https://github.com/tangle-network/blueprint/pull/27))
- rename `GadgetConfiguration` to `BlueprintEnvironment` ([#34](https://github.com/tangle-network/blueprint/pull/34))

## [0.1.0-alpha.1](https://github.com/tangle-network/blueprint/releases/tag/blueprint-runner-v0.1.0-alpha.1) - 2025-04-08

### Other

- Initial release
