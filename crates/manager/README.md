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

## Remote GPU leases

`RemoteProviderManager` uses `CloudProvisioner::allocate` for GPU requests that
do not require confidential compute. The request supplies an SSH-ready image;
no image is inferred from GPU metadata. The manager's configured spending cap
and enabled provider regions constrain SDK quote selection. Workload environment
is uploaded over lease SSH access, never placed in the provider launch request.

Provider ownership combines the canonical manager data-directory namespace and
blueprint/service ids. Keep that directory path stable across restarts. Native
provider deadlines drive startup and periodic `retire_expired` reconciliation;
GPU leases do not enter the legacy deployment tracker. Allocation inventory and
creation share the SDK's OS-backed service lease lock across manager processes.
Initialized CPU-only adapters do not participate in managed inventory; a failed
configured adapter cannot be treated as proof of absence. Failed retirement stays
visible and retryable; service termination searches only the manager namespace
and exact service lease identity on enabled managed providers.

CPU and confidential-compute requests retain the existing provisioner path.
The unused `RemoteDeploymentExtensions`/`TtlManager` facade is removed; SDK
consumers of those public names must migrate to the managed lease operations.


GPU container uploads pass the assigned device count and requested ports through
the shared SSH container builder. Workload environment reaches an owner-only
remote file over SSH stdin; it is absent from command arguments and debug logs.
OpenSSH must already trust the host through the operator's known-hosts policy.
Unknown or changed host identity fails before any workload credential transfer;
the manager does not bootstrap trust from an unauthenticated scan. Process peers
verify the launch arguments and failure contract, not live GPU visibility or a
production host-key enrollment flow.
