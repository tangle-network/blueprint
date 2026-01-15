pub use crate::command::create::error::Error;
pub use crate::command::create::source::Source;
pub use crate::command::create::types::BlueprintType;
use crate::foundry::FoundryToolchain;
use templates::TangleEvmTemplate;
use types::{BlueprintVariant, EigenlayerVariant};

pub mod error;
pub mod source;
mod templates;
pub mod types;

/// Generate a new blueprint from a template
///
/// # Errors
///
/// See [`cargo_generate::generate()`]
///
/// # Parameters
///
/// * `name` - The name of the blueprint
/// * `source` - Optional source information (repo, branch, path)
/// * `blueprint_type` - Optional blueprint type (Tangle or Eigenlayer)
/// * `define` - Template variable definitions (key=value pairs)
/// * `template_values_file` - Optional path to a file containing template values
/// * `skip_prompts` - Whether to skip all interactive prompts, using defaults for unspecified values
pub fn new_blueprint(
    name: &str,
    source: Option<Source>,
    blueprint_type: Option<BlueprintType>,
    mut define: Vec<String>,
    template_values_file: &Option<String>,
    skip_prompts: bool,
) -> Result<(), Error> {
    println!("Generating blueprint with name: {}", name);

    let source = source.unwrap_or_default();
    let blueprint_variant = blueprint_type.map(|t| t.get_type()).unwrap_or_default();
    let template_path_opt: Option<cargo_generate::TemplatePath> = source.into();

    // Track if we're using embedded templates (need to clean up temp dir later)
    let mut using_embedded_template = false;

    let template_path = template_path_opt.unwrap_or_else(|| {
        match blueprint_variant {
            // Use embedded template for Tangle EVM blueprints
            Some(BlueprintVariant::Tangle) | None => {
                println!("Using embedded Tangle EVM template");
                match TangleEvmTemplate::write_to_temp_dir() {
                    Ok(temp_dir) => {
                        using_embedded_template = true;
                        cargo_generate::TemplatePath {
                            path: Some(temp_dir.to_string_lossy().into()),
                            ..Default::default()
                        }
                    }
                    Err(e) => {
                        println!(
                            "Warning: Failed to write embedded template: {e}. Falling back to GitHub."
                        );
                        cargo_generate::TemplatePath {
                            git: Some(
                                "https://github.com/tangle-network/blueprint-template".into(),
                            ),
                            branch: Some(String::from("main")),
                            ..Default::default()
                        }
                    }
                }
            }
            // EigenLayer templates still use GitHub
            Some(BlueprintVariant::Eigenlayer(EigenlayerVariant::BLS)) => {
                cargo_generate::TemplatePath {
                    git: Some("https://github.com/tangle-network/eigenlayer-bls-template".into()),
                    branch: Some(String::from("main")),
                    ..Default::default()
                }
            }
            Some(BlueprintVariant::Eigenlayer(EigenlayerVariant::ECDSA)) => {
                cargo_generate::TemplatePath {
                    git: Some("https://github.com/tangle-network/eigenlayer-ecdsa-template".into()),
                    branch: Some(String::from("main")),
                    ..Default::default()
                }
            }
        }
    });

    if skip_prompts {
        println!("Skipping prompts and using default values for unspecified template variables");

        // Create a map of existing variable definitions
        let mut defined_vars = std::collections::HashMap::new();
        for def in &define {
            if let Some((key, value)) = def.split_once('=') {
                defined_vars.insert(key.to_string(), value.to_string());
            }
        }

        // Define default values for template variables
        // These cover both embedded templates and legacy GitHub templates
        let defaults = [
            ("project-description", "A Tangle EVM blueprint"),
            ("author", "Tangle Network"),
            // Legacy GitHub template variables (for backwards compatibility)
            ("gh-username", ""),
            ("gh-repo", ""),
            ("gh-organization", ""),
            ("project-homepage", ""),
            ("flakes", "false"),
            ("container", "true"),
            ("base-image", "rustlang/rust:nightly"),
            ("container-registry", "docker.io"),
            ("ci", "true"),
            ("rust-ci", "true"),
            ("release-ci", "true"),
        ];

        // Add default values for any variables that aren't already defined
        for (key, value) in defaults {
            if !defined_vars.contains_key(key) {
                define.push(format!("{key}={value}"));
                println!("  Using default value for {key}: {value}");
            }
        }
    } else {
        println!("Running in interactive mode - will prompt for template variables as needed");
    }

    if !define.is_empty() {
        println!("Using template variables: {:?}", define);
    }
    // Enable silent mode when skip_prompts is true or a values file is provided
    let template_values_file = template_values_file.clone();
    let silent = skip_prompts || template_values_file.is_some();
    if template_values_file.is_some() {
        println!(
            "Using template values file: {}",
            template_values_file.as_ref().unwrap()
        );
    }

    let generation_result = cargo_generate::generate(cargo_generate::GenerateArgs {
        template_path,
        list_favorites: false,
        name: Some(name.to_string()),
        force: false,
        verbose: false,
        template_values_file,
        silent,
        config: None,
        vcs: Some(cargo_generate::Vcs::Git),
        lib: false,
        ssh_identity: None,
        gitconfig: None,
        define,
        init: false,
        destination: None,
        force_git_init: false,
        allow_commands: false,
        overwrite: false,
        skip_submodules: false,
        other_args: Option::default(),
        continue_on_error: false,
        quiet: false,
        no_workspace: false,
    });

    // Clean up embedded template temp directory if we used it
    if using_embedded_template {
        if let Err(e) = TangleEvmTemplate::cleanup_temp_dir() {
            println!("Warning: Failed to clean up temp template directory: {e}");
        }
    }

    let path = generation_result.map_err(Error::GenerationFailed)?;

    println!("Blueprint generated at: {}", path.display());

    let foundry = FoundryToolchain::new();
    if !foundry.forge.is_installed() {
        blueprint_core::warn!("Forge not installed, skipping dependencies");
        blueprint_core::warn!("NOTE: See <https://getfoundry.sh>");
        blueprint_core::warn!(
            "NOTE: After installing Forge, you can run `forge soldeer update -d` to install dependencies"
        );
        return Ok(());
    }

    std::env::set_current_dir(path)?;
    if let Err(e) = foundry.forge.install_dependencies() {
        blueprint_core::error!("{e}");
    }

    Ok(())
}
