use std::path::PathBuf;

use crate::deploy::tangle::{deploy_to_tangle, Opts};
use cargo_tangle::create::BlueprintType;
#[cfg(feature = "eigenlayer")]
use cargo_tangle::deploy::eigenlayer::{deploy_to_eigenlayer, EigenlayerDeployOpts, NetworkTarget};
use cargo_tangle::{create, deploy, keys};
use clap::{Parser, Subcommand};
use gadget_crypto::KeyTypeId;

/// Tangle CLI tool
#[derive(Parser, Debug)]
#[clap(
    bin_name = "cargo-tangle",
    version,
    propagate_version = true,
    arg_required_else_help = true
)]
struct Cli {
    #[command(flatten)]
    manifest: clap_cargo::Manifest,
    #[command(flatten)]
    features: clap_cargo::Features,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Blueprint subcommand
    #[command(visible_alias = "bp")]
    Blueprint {
        #[command(subcommand)]
        subcommand: GadgetCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum GadgetCommands {
    /// Create a new blueprint
    #[command(visible_alias = "c")]
    Create {
        /// The name of the blueprint
        #[arg(short, long, value_name = "NAME", env = "NAME")]
        name: String,

        #[command(flatten)]
        source: Option<create::Source>,

        #[command(flatten)]
        blueprint_type: Option<BlueprintType>,
    },

    /// Deploy a blueprint to the Tangle Network or Eigenlayer.
    #[command(visible_alias = "d")]
    Deploy {
        #[command(subcommand)]
        target: DeployTarget,
    },

    /// Generate a key
    Keygen {
        /// The type of key to generate
        #[arg(short, long, value_enum)]
        key_type: KeyTypeId,

        /// The path to save the key to, if not provided, the key will be printed
        /// to the console instead
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// The seed to use for the generation of the key in hex format without 0x prefix,
        /// if not provided, a random seed will be generated
        #[arg(short, long, value_name = "SEED_HEX", env = "SEED")]
        seed: Option<String>,

        /// If true, the secret key will be printed along with the public key
        #[arg(long)]
        show_secret: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum DeployTarget {
    /// Deploy to Tangle Network
    Tangle {
        /// HTTP RPC URL to use
        #[arg(
            long,
            value_name = "URL",
            default_value = "https://rpc.tangle.tools",
            env
        )]
        http_rpc_url: String,
        /// Tangle RPC URL to use
        #[arg(
            long,
            value_name = "URL",
            default_value = "wss://rpc.tangle.tools",
            env
        )]
        ws_rpc_url: String,
        /// The package to deploy (if the workspace has multiple packages).
        #[arg(short, long, value_name = "PACKAGE", env = "CARGO_PACKAGE")]
        package: Option<String>,
    },
    /// Deploy to Eigenlayer
    #[cfg(feature = "eigenlayer")]
    Eigenlayer {
        /// Deploy to local network with given RPC URL
        #[arg(long, value_name = "URL", group = "network")]
        local: Option<String>,
        /// Deploy to testnet with given RPC URL
        #[arg(long, value_name = "URL", group = "network")]
        testnet: Option<String>,
        /// Deploy to mainnet with given RPC URL
        #[arg(long, value_name = "URL", group = "network")]
        mainnet: Option<String>,
        /// Path to the configuration file
        #[arg(long, value_name = "PATH")]
        config: PathBuf,
    },
}

#[tokio::main]
#[allow(clippy::needless_return)]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing_subscriber();
    let args: Vec<String> = if std::env::args()
        .nth(1)
        .map(|x| x.eq("tangle"))
        .unwrap_or(false)
    {
        // since this runs as a cargo subcommand, we need to skip the first argument
        // to get the actual arguments for the subcommand
        std::env::args().skip(1).collect()
    } else {
        std::env::args().collect()
    };

    // Parse the CLI arguments
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Blueprint { subcommand } => match subcommand {
            GadgetCommands::Create {
                name,
                source,
                blueprint_type,
            } => {
                create::new_blueprint(name, source, blueprint_type)?;
            }
            GadgetCommands::Deploy { target } => match target {
                DeployTarget::Tangle {
                    http_rpc_url,
                    ws_rpc_url,
                    package,
                } => {
                    let manifest_path = cli
                        .manifest
                        .manifest_path
                        .unwrap_or_else(|| PathBuf::from("Cargo.toml"));
                    let _ = deploy_to_tangle(Opts {
                        http_rpc_url,
                        ws_rpc_url,
                        manifest_path,
                        pkg_name: package,
                        signer: None,
                        signer_evm: None,
                    })
                    .await?;
                }
                #[cfg(feature = "eigenlayer")]
                DeployTarget::Eigenlayer {
                    local,
                    testnet,
                    mainnet,
                    config,
                } => {
                    let (network, rpc_url) = match (local, testnet, mainnet) {
                        (Some(url), None, None) => (NetworkTarget::Local, url),
                        (None, Some(url), None) => (NetworkTarget::Testnet, url),
                        (None, None, Some(url)) => (NetworkTarget::Mainnet, url),
                        _ => return Err(color_eyre::eyre::eyre!("Must specify exactly one network target (--local, --testnet, or --mainnet)")),
                    };

                    deploy_to_eigenlayer(EigenlayerDeployOpts {
                        network,
                        rpc_url,
                        config_path: config,
                    })
                    .await?;
                }
            },
            GadgetCommands::Keygen {
                key_type,
                path,
                seed,
                show_secret,
            } => {
                let seed = seed.map(hex::decode).transpose()?;
                let (public, secret) =
                    keys::generate_key(key_type, path.as_ref(), seed.as_deref(), show_secret)?;

                eprintln!("Generated {} key:", key_type.name());
                eprintln!("Public key: {}", public);
                if show_secret || path.is_none() {
                    eprintln!("Private key: {}", secret.expect("Should exist"));
                }
            }
        },
    }
    Ok(())
}

fn init_tracing_subscriber() {
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::prelude::*;

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_span_events(FmtSpan::CLOSE)
        .pretty();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(fmt_layer)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
