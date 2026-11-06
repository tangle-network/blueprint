# blueprint-manager

Blueprint runtime manager for orchestrating service execution.

`blueprint-manager` is responsible for loading runtime configuration, resolving protocol/runtime sources, and coordinating execution of Blueprint services.

GitHub sources select the release's cargo-dist artifact for the native target and
use its executable asset path. The manager checks a declared SHA256 archive
checksum, requires GitHub attestation unless explicitly configured otherwise, and
always verifies the registered binary digest, including cached executables.
Release-host, HTTP-client and GitHub-CLI builders are application configuration;
blueprint metadata cannot override them. Linux release archives use `.tar.xz`.

## What it provides

- Manager runtime orchestration
- Protocol/runtime source handling
- Execution entrypoint (`run_blueprint_manager`)
- Integration surface with manager bridge and auth components

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/manager
- CLI docs: https://docs.tangle.tools/developers/cli/tangle
