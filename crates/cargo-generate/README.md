# Blueprint cargo-generate compatibility package

This package derives from `cargo-generate` 0.23.12, licensed under MIT or Apache-2.0.
It retains the library API used by `cargo-tangle` and widens the `regex` requirement from `~1.12` to `^1.13`.
The original requirement conflicts with the `regex ^1.13` requirement of libp2p 0.57 when both libraries appear in one CLI dependency graph.
The upstream source is https://github.com/cargo-generate/cargo-generate/tree/v0.23.12.

Version 0.23.12 matches the upstream cargo-generate implementation and its template version check.
It pins kstring 2.0.2 because 2.0.5 requires Rust 1.96; cargo-tangle supports Rust 1.93.
