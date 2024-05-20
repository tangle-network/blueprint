use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf, str::FromStr};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

// #[derive(Debug, StructOpt)]
// #[structopt(
// name = "Gadget",
// about = "An MPC executor that connects to the Tangle network to perform work"
// )]
#[derive(Serialize, Deserialize, Debug, Tsify)]
#[tsify(from_wasm_abi)]
pub struct Opt {
    // /// The path to the configuration file. If not provided, the default configuration will be used.
    // /// Note that if the configuration file is provided, the command line arguments will be ignored.
    // #[structopt(global = true, parse(from_os_str), short = "c", long = "config")]
    pub config: Option<PathBuf>,
    // /// The verbosity level, can be used multiple times
    // #[structopt(long, short = "v", global = true, parse(from_occurrences))]
    pub verbose: i32,
    // /// Whether to use pretty logging
    // #[structopt(global = true, long)]
    pub pretty: bool,
    // /// The options for the shell
    // #[structopt(flatten)]
    pub options: TomlConfig,
}

// #[structopt(rename_all = "snake_case")]
#[derive(Default, Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "snake_case")]
#[tsify(from_wasm_abi)]
pub enum SupportedChains {
    #[default]
    LocalTestnet,
    LocalMainnet,
    Testnet,
    Mainnet,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(from_wasm_abi)]
pub struct TomlConfig {
    // /// The IP address to bind to for the libp2p node.
    // #[structopt(long = "bind-ip", short = "i", default_value = defaults::BIND_IP)]
    // #[serde(default = "defaults::bind_ip")]
    pub bind_ip: String,
    // /// The port to bind to for the libp2p node.
    // #[structopt(long = "port", short = "p", default_value = defaults::BIND_PORT)]
    // #[serde(default = "defaults::bind_port")]
    pub bind_port: u16,
    // /// The RPC URL of the Tangle Node.
    // #[structopt(long = "url", parse(try_from_str = url::Url::parse), default_value = defaults::RPC_URL)]
    // #[serde(default = "defaults::rpc_url")]
    pub url: String,
    // /// The List of bootnodes to connect to
    // #[structopt(long = "bootnodes", parse(try_from_str = <Multiaddr as std::str::FromStr>::from_str))]
    // #[serde(default)]
    pub bootnodes: Vec<String>,
    // /// The node key in hex format. If not provided, a random node key will be generated.
    // #[structopt(long = "node-key", env, parse(try_from_str = parse_node_key))]
    // #[serde(skip_serializing)]
    pub node_key: Option<String>,
    // /// The base path to store the shell data, and read data from the keystore.
    // #[structopt(
    // parse(from_os_str),
    // long,
    // short = "d",
    // required_unless = "config",
    // default_value_if("config", None, ".")
    // )]
    pub base_path: PathBuf,
    // /// Keystore Password, if not provided, the password will be read from the environment variable.
    // #[structopt(long = "keystore-password", env)]
    pub keystore_password: Option<String>,
    // /// The chain to connect to, must be one of the supported chains.
    // #[structopt(
    // long,
    // default_value,
    // possible_values = &[
    // "local_testnet",
    // "local_mainnet",
    // "testnet",
    // "mainnet"
    // ]
    // )]
    // #[serde(default)]
    pub chain: SupportedChains,
}

impl FromStr for SupportedChains {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "local_testnet" => Ok(SupportedChains::LocalTestnet),
            "local_mainnet" => Ok(SupportedChains::LocalMainnet),
            "testnet" => Ok(SupportedChains::Testnet),
            "mainnet" => Ok(SupportedChains::Mainnet),
            _ => Err(format!("Invalid chain: {}", s)),
        }
    }
}

impl Display for SupportedChains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedChains::LocalTestnet => write!(f, "local_testnet"),
            SupportedChains::LocalMainnet => write!(f, "local_mainnet"),
            SupportedChains::Testnet => write!(f, "testnet"),
            SupportedChains::Mainnet => write!(f, "mainnet"),
        }
    }
}
