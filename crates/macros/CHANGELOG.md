# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/tangle-network/blueprint/releases/tag/blueprint-macros-v0.1.0) - 2025-04-08

### Added

- *(cargo-tangle)* tangle command workflow  ([#703](https://github.com/tangle-network/blueprint/pull/703))
- impl `IntoTangleFieldTypes` for `TangleResult` in `Result` and `Option` ([#729](https://github.com/tangle-network/blueprint/pull/729))
- debug macros, sdk crate ([#23](https://github.com/tangle-network/blueprint/pull/23))
- new networking ([#664](https://github.com/tangle-network/blueprint/pull/664))
- Add multinode test executor
- *(cargo-tangle)* eigenlayer deployment ([#645](https://github.com/tangle-network/blueprint/pull/645))
- gadget workspace migration

### Fixed

- misc fixes ([#709](https://github.com/tangle-network/blueprint/pull/709))
- remove `#[call_id]` ([#713](https://github.com/tangle-network/blueprint/pull/713))
- finish migration of new job system ([#699](https://github.com/tangle-network/blueprint/pull/699))
- update some tests
- blueprint examples ([#643](https://github.com/tangle-network/blueprint/pull/643))
- update blueprint examples ([#628](https://github.com/tangle-network/blueprint/pull/628))

### Other

- *(ci)* use nightly rustfmt ([#835](https://github.com/tangle-network/blueprint/pull/835))
- pin workspace to 1.85 ([#821](https://github.com/tangle-network/blueprint/pull/821))
- clippy ([#812](https://github.com/tangle-network/blueprint/pull/812))
- Implement the crate naming conventions ([#759](https://github.com/tangle-network/blueprint/pull/759))
- improve readme ([#755](https://github.com/tangle-network/blueprint/pull/755))
- remove `async-trait` ([#717](https://github.com/tangle-network/blueprint/pull/717))
- remove proc macro core ([#716](https://github.com/tangle-network/blueprint/pull/716))
- *(clippy)* use workspace lints globally ([#710](https://github.com/tangle-network/blueprint/pull/710))
- remove old event listeners and runners
- rustdoc and READMEs for crates ([#27](https://github.com/tangle-network/blueprint/pull/27))
- bump alloy & eigensdk ([#696](https://github.com/tangle-network/blueprint/pull/696))
- Generalize networking key type ([#685](https://github.com/tangle-network/blueprint/pull/685))
- cleanup crates for release
- Update to tangle main services ([#674](https://github.com/tangle-network/blueprint/pull/674))
- *(networking)* stop using `Box<dyn Error>` ([#657](https://github.com/tangle-network/blueprint/pull/657))
- remove `StdGadgetConfiguration` ([#656](https://github.com/tangle-network/blueprint/pull/656))
- cleanup crate features & update `tangle-subxt` ([#642](https://github.com/tangle-network/blueprint/pull/642))
- add descriptions to crates ([#616](https://github.com/tangle-network/blueprint/pull/616))
