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

Provider ownership combines a persisted random manager identity and service IDs.
The owner ID in `blueprint/manager-owner` is synced before rental; invalid IDs fail
closed. Preserve manager state across restart and stop managers before moving it.
Provider deadlines drive startup and periodic `retire_expired` reconciliation;
GPU leases do not enter the legacy deployment tracker. Inventory and creation
share an OS-backed service lock in manager state, independent of cache roots.
Initialized CPU-only adapters do not participate in managed inventory; a failed
configured adapter cannot be treated as proof of absence. Failed retirement stays
visible and retryable; service termination searches only the manager namespace
and exact service lease identity on enabled managed providers.

CPU and confidential-compute requests retain the existing provisioner path.
The unused `RemoteDeploymentExtensions`/`TtlManager` facade is removed; SDK
consumers of those public names must migrate to managed lease operations. SSH
host-check opt-outs and the test-only localhost constructor are also removed.

GPU container uploads pass the assigned device count and requested ports through
the shared SSH container builder. Workload environment reaches an owner-only
remote file over SSH stdin; it is absent from command arguments and debug logs.
OpenSSH must already trust the host through the operator's known-hosts policy.
Unknown or changed host identity fails before any workload credential transfer;
the manager does not bootstrap trust from an unauthenticated scan. Process peers
verify the launch arguments and failure contract, not live GPU visibility or a
production host-key enrollment flow.
