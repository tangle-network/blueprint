use crate::command::harness::config::HarnessConfig;
use color_eyre::eyre::Result;

/// List all discoverable harness.toml files.
pub async fn run() -> Result<()> {
    let harnesses = HarnessConfig::discover();

    if harnesses.is_empty() {
        println!("No harness.toml files found.");
        println!();
        println!("Create one in your blueprint directory:");
        println!();
        println!("  # harness.toml");
        println!("  [[blueprint]]");
        println!("  name = \"my-blueprint\"");
        println!("  path = \".\"");
        println!("  binary = \"target/release/my-operator\"");
        println!();
        println!("Then run: cargo tangle harness up");
        return Ok(());
    }

    println!("Discovered {} harness(es):", harnesses.len());
    println!();

    for (name, path) in &harnesses {
        // Try to load and count blueprints
        let detail = match HarnessConfig::load(Some(path)) {
            Ok(config) => {
                let bp_names: Vec<_> = config.blueprints.iter().map(|b| b.name.as_str()).collect();
                let test_count = config
                    .blueprints
                    .iter()
                    .filter(|b| b.test_command.is_some())
                    .count();
                format!(
                    "{} blueprint(s) [{}], {} with tests",
                    config.blueprints.len(),
                    bp_names.join(", "),
                    test_count
                )
            }
            Err(e) => format!("(error loading: {e})"),
        };

        println!("  {name}");
        println!("    path: {}", path.display());
        println!("    {detail}");
        println!();
    }

    println!("Usage:");
    println!("  cargo tangle harness up                              # uses ./harness.toml");
    println!("  cargo tangle harness up -c /path/to/harness.toml     # specific config");
    println!("  cargo tangle harness up --compose dir1,dir2,dir3     # compose multiple");
    println!("  cargo tangle harness test                            # run all test_commands");
    println!("  cargo tangle harness test --only llm,voice           # test specific blueprints");

    Ok(())
}
