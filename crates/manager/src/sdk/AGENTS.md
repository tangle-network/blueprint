# sdk

## Purpose
Shared utilities and setup helpers for the blueprint manager. Provides logging configuration, the `SendFuture` trait bound, and filesystem/network utility functions used across the manager codebase.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports `entry` and `utils` submodules.
- `entry.rs` - `SendFuture` trait alias (combining `Send + Future + 'a`). `setup_blueprint_manager_logger()` configures `tracing_subscriber` with verbosity level (0=ERROR through 4+=TRACE), optional pretty formatting, and env filter.
- `utils.rs` - `hash_bytes_to_hex()` (SHA-256), `valid_file_exists()` (async file existence check), `get_formatted_os_string()` (maps OS to target triple fragment), `make_executable()` (sets Unix +x bits or adds .exe on Windows), `slice_32_to_sha_hex_string()` (byte array to hex). `PortLock` enum that binds a `TcpListener` to reserve a port until `unlock()` is called, used for Kubernetes service port allocation.

## Key APIs (no snippets)
- `SendFuture<'a, T>` - trait alias for `Send + Future<Output = T> + 'a`, used by executor entry points
- `setup_blueprint_manager_logger(verbose, pretty, filter)` - initializes tracing with configurable verbosity
- `make_executable(path) -> Result<PathBuf>` - ensures a binary file is executable
- `PortLock` - lock/unlock pattern for reserving TCP ports

## Relationships
- `SendFuture` used as bound on `shutdown_cmd` in `executor/mod.rs`
- `make_executable` called by `sources/github.rs`, `sources/remote.rs`, and `sources/testing.rs` after fetching binaries
- `get_formatted_os_string` used by `blueprint/native.rs` for platform matching
- `PortLock` used by `config/ctx.rs` for Kubernetes service port reservation
