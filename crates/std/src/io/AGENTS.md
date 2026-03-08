# io

## Purpose
Provides `no_std`-compatible replacements for `std::io` traits and types. When the `std` feature is disabled, this module supplies `Read`, `Write`, `Cursor`, `Error`, and `ErrorKind` implementations that work in bare-metal / WASM environments without an allocator-backed standard library.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Defines `Read`, `Write`, and `Cursor` traits/types with implementations for `&[u8]`, `&mut [u8]`, and `Vec<u8>`; re-exports `error` module
- `error.rs` - Conditionally provides `std::io::{Error, ErrorKind, Result}` (std) or a custom no-std `Error`/`ErrorKind`/`Result` with the same interface

## Key APIs
- `Read` trait: `read`, `read_exact`, `by_ref`
- `Write` trait: `write`, `flush`, `write_all`, `by_ref`
- `Cursor<T>`: positional read/write wrapper for in-memory buffers (`new`, `into_inner`, `get_ref`, `get_mut`, `position`, `set_position`)
- `Error` / `ErrorKind` / `Result`: mirror std::io error types for no-std

## Relationships
- Only compiled when `not(feature = "std")` -- when `std` is enabled, `blueprint_std` re-exports `std::io` directly
- Used by any crate in the workspace that needs serialization I/O without depending on the standard library
