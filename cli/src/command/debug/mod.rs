pub mod spawn;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum DebugCommands {
    /// Launch a local Anvil stack and run the blueprint against it.
    Spawn(spawn::SpawnArgs),
}
