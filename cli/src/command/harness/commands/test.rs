use crate::command::harness::TestArgs;
use crate::command::harness::config::HarnessConfig;
use crate::command::harness::orchestrator::Orchestrator;
use color_eyre::eyre::{Result, eyre};
use std::path::PathBuf;
use std::process::Stdio;

/// Boot the harness, run each blueprint's test_command, report results, shut down.
pub async fn run(args: TestArgs) -> Result<()> {
    let mut config = if let Some(compose) = &args.compose {
        let paths: Vec<PathBuf> = compose
            .split(',')
            .map(|s| {
                let s = s.trim();
                if let Some(rest) = s.strip_prefix("~/") {
                    let home = std::env::var("HOME").unwrap_or_default();
                    PathBuf::from(home).join(rest)
                } else {
                    PathBuf::from(s)
                }
            })
            .collect();
        HarnessConfig::compose(&paths)?
    } else {
        HarnessConfig::load(args.config.as_deref())?
    };

    config.apply_chain_overrides(&args.chain);
    config.filter(args.only.as_deref());

    // Check that at least one blueprint has a test_command
    let testable: Vec<_> = config
        .blueprints
        .iter()
        .filter(|bp| bp.test_command.is_some())
        .collect();

    if testable.is_empty() {
        return Err(eyre!(
            "no blueprints have test_command set in their harness.toml"
        ));
    }

    println!(
        "Running tests for {} blueprint(s): {}",
        testable.len(),
        testable
            .iter()
            .map(|bp| bp.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!();

    // Boot the harness
    let mut orchestrator = Orchestrator::bootstrap(&config).await?;
    orchestrator.spawn_blueprints(&config).await?;

    // Run tests
    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut results: Vec<(String, bool, String)> = Vec::new();

    for bp in &config.blueprints {
        let Some(cmd) = &bp.test_command else {
            continue;
        };

        println!("━━━ Testing: {} ━━━", bp.name);
        println!("  command: {cmd}");
        println!("  cwd: {}", bp.path.display());
        println!();

        let output = tokio::process::Command::new("sh")
            .args(["-c", cmd])
            .current_dir(&bp.path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .await;

        match output {
            Ok(out) if out.status.success() => {
                println!();
                println!("  ✅ {} PASSED", bp.name);
                passed += 1;
                results.push((bp.name.clone(), true, String::new()));
            }
            Ok(out) => {
                let code = out.status.code().unwrap_or(-1);
                println!();
                println!("  ❌ {} FAILED (exit code {})", bp.name, code);
                failed += 1;
                results.push((bp.name.clone(), false, format!("exit code {code}")));
            }
            Err(e) => {
                println!();
                println!("  ❌ {} ERROR: {e}", bp.name);
                failed += 1;
                results.push((bp.name.clone(), false, e.to_string()));
            }
        }
        println!();
    }

    // Summary
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Results: {} passed, {} failed", passed, failed);
    for (name, ok, reason) in &results {
        if *ok {
            println!("    ✅ {name}");
        } else {
            println!("    ❌ {name}: {reason}");
        }
    }
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Shut down
    // Note: we don't call run_until_shutdown() — we exit after tests.
    // The orchestrator's Drop impl will kill child processes.
    drop(orchestrator);

    if failed > 0 {
        Err(eyre!("{failed} test(s) failed"))
    } else {
        Ok(())
    }
}
