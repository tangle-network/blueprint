use color_eyre::eyre::Result;

pub async fn run() -> Result<()> {
    println!("Harness uses foreground execution — press Ctrl+C on the `up` process to stop.");
    println!("(Background daemon mode coming in a future release.)");
    Ok(())
}
