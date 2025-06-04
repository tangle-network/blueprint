pub use crate::command::create::error::Error;
pub use crate::command::create::source::Source;
pub use crate::command::create::types::BlueprintType;
use crate::foundry::FoundryToolchain;
use types::{BlueprintVariant, EigenlayerVariant};

pub mod error;
pub mod source;
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

    let template_path = template_path_opt.unwrap_or_else(|| {
        // TODO: Interactive selection (#352)
        let template_repo: String = match blueprint_variant {
            Some(BlueprintVariant::Tangle) | None => {
                "https://github.com/tangle-network/blueprint-template".into()
            }
            Some(BlueprintVariant::Eigenlayer(EigenlayerVariant::BLS)) => {
                "https://github.com/tangle-network/eigenlayer-bls-template".into()
            }
            Some(BlueprintVariant::Eigenlayer(EigenlayerVariant::ECDSA)) => {
                "https://github.com/tangle-network/eigenlayer-ecdsa-template".into()
            }
        };

        cargo_generate::TemplatePath {
            git: Some(template_repo),
            branch: Some(String::from("main")),
            ..Default::default()
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

        // Define default values for common template variables
        let defaults = [
            ("gh-username", ""),
            ("gh-repo", ""),
            ("gh-organization", ""),
            ("project-description", ""),
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
    let (silent, template_values_file) = if let Some(file) = &template_values_file {
        println!("Using template values file: {}", file);
        (true, Some(file.clone()))
    } else {
        (false, None)
    };

    let path = cargo_generate::generate(cargo_generate::GenerateArgs {
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
        bin: true,
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
    })
    .map_err(Error::GenerationFailed)?;

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
