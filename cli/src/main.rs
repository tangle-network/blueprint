mod bin_support;

use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    bin_support::init_tracing_subscriber();
    cargo_tangle::install_crypto_provider();
    bin_support::run().await
}
