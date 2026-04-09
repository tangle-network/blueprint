use crate::command::harness::LogsArgs;
use color_eyre::eyre::Result;

pub async fn run(_args: LogsArgs) -> Result<()> {
    println!("Logs stream to stdout in the `up` process.");
    Ok(())
}
