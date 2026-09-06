# Anvil snapshot fixtures

These fixtures deploy real tnt-core contracts.
Keep their ABI compatible with the `tnt-core-bindings` dependency resolved by this workspace.
A mismatch can decode successful calls into incorrect fields.
When changing the bindings, regenerate affected fixtures in the same change and verify representative contract calls.

Read the broadcast file's top-level `commit` to identify its source revision.
Use `script/sh/update-localtestnet-fixtures.sh` in the owning tnt-core checkout to regenerate both broadcast and state files.
Follow that script's current Forge requirements, including any initcode size limit needed for the deployment.
Retain source revision and tool identity with generated fixtures.

Deployment order determines CREATE addresses.
After regeneration, read addresses from the broadcast and update the matching constants in `crates/testing-utils/anvil/src/tangle.rs` and `cli/src/command/dev/up.rs`.
Keep the state dump compatible with the Anvil runtime selected by `ANVIL_TAG` in [anvil.rs](../src/anvil.rs).
Check the actual reader and writer versions when compatibility is uncertain.
