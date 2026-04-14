//! `cargo-tangle dev` — zero-config local Tangle devnet with Blueprint Manager.
//!
//! Replaces the shell-scripted boot sequence (anvil → keystore → manager → permitted caller
//! → write .tangle.toml) with a single command. Runs detached so it doesn't block the terminal;
//! `dev down` tears everything down.

pub mod down;
pub mod status;
pub mod up;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum DevCommands {
    /// Boot a local devnet: Anvil with seeded contracts, Blueprint Manager, and a
    /// pre-registered operator. Writes `.tangle.toml` so every other cargo-tangle
    /// command works with zero arguments in this directory.
    Up(up::UpArgs),
    /// Stop the local devnet and remove its scratch directory.
    Down(down::DownArgs),
    /// Show the status of the local devnet.
    Status,
}
