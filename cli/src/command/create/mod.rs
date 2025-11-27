pub use crate::command::create::error::Error;
pub use crate::command::create::source::Source;
pub use crate::command::create::types::BlueprintType;
use crate::foundry::FoundryToolchain;
use clap::Args;
use std::collections::HashMap;
use types::{BlueprintVariant, EigenlayerVariant};

pub mod error;
pub mod source;
pub mod types;

const BASE_REQUIRED_TEMPLATE_KEYS: [&str; 6] = [
    "gh-username",
    "gh-repo",
    "gh-organization",
    "project-description",
    "project-homepage",
    "container",
];

const CONTAINER_TEMPLATE_KEYS: [&str; 2] = ["base-image", "container-registry"];

#[derive(Debug, Clone, Default, Args)]
pub struct TemplateVariables {
    /// GitHub username associated with the blueprint repository
    #[arg(long = "gh-username", value_name = "USERNAME")]
    pub gh_username: Option<String>,

    /// GitHub repository name for this blueprint
    #[arg(long = "gh-repo", value_name = "REPO")]
    pub gh_repo: Option<String>,

    /// GitHub organization or user that owns the repository
    #[arg(long = "gh-organization", value_name = "ORG")]
    pub gh_organization: Option<String>,

    /// Short description of the project
    #[arg(long = "project-description", value_name = "TEXT")]
    pub project_description: Option<String>,

    /// Homepage or documentation URL
    #[arg(long = "project-homepage", value_name = "URL")]
    pub project_homepage: Option<String>,

    /// Enable Nix flakes support
    #[arg(long = "flakes", value_name = "BOOL")]
    pub flakes: Option<bool>,

    /// Generate container assets
    #[arg(long = "container", value_name = "BOOL")]
    pub container: Option<bool>,

    /// Base image to use when generating containers
    #[arg(long = "base-image", value_name = "IMAGE")]
    pub base_image: Option<String>,

    /// Container registry for pushing images
    #[arg(long = "container-registry", value_name = "REGISTRY")]
    pub container_registry: Option<String>,

    /// Enable CI workflows
    #[arg(long = "ci", value_name = "BOOL")]
    pub ci: Option<bool>,

    /// Enable Rust-specific CI workflows
    #[arg(long = "rust-ci", value_name = "BOOL")]
    pub rust_ci: Option<bool>,

    /// Enable release CI workflows
    #[arg(long = "release-ci", value_name = "BOOL")]
    pub release_ci: Option<bool>,
}

impl TemplateVariables {
    pub fn merge_into(self, define: &mut Vec<String>) {
        Self::push("gh-username", self.gh_username, define);
        Self::push("gh-repo", self.gh_repo, define);
        Self::push("gh-organization", self.gh_organization, define);
        Self::push("project-description", self.project_description, define);
        Self::push("project-homepage", self.project_homepage, define);
        Self::push("flakes", self.flakes, define);
        Self::push("container", self.container, define);
        Self::push("base-image", self.base_image, define);
        Self::push("container-registry", self.container_registry, define);
        Self::push("ci", self.ci, define);
        Self::push("rust-ci", self.rust_ci, define);
        Self::push("release-ci", self.release_ci, define);
    }

    fn push<T: ToString>(key: &str, value: Option<T>, define: &mut Vec<String>) {
        if let Some(value) = value {
            define.push(format!("{key}={}", value.to_string()));
        }
    }
}

fn ensure_default_bool(define: &mut Vec<String>, key: &str, default: bool) {
    let key_eq = format!("{key}=");
    if define.iter().any(|entry| entry.starts_with(&key_eq)) {
        return;
    }

    define.push(format!("{key}={}", default));
}

fn missing_required_template_variables(define: &[String]) -> Vec<&'static str> {
    let provided = build_define_map(define);
    let mut missing = Vec::new();

    for key in BASE_REQUIRED_TEMPLATE_KEYS {
        if !provided.contains_key(key) {
            missing.push(key);
        }
    }

    if container_fields_required(&provided) {
        for key in CONTAINER_TEMPLATE_KEYS {
            if !provided.contains_key(key) {
                missing.push(key);
            }
        }
    }

    missing
}

fn build_define_map(define: &[String]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for entry in define {
        if let Some((key, value)) = entry.split_once('=') {
            map.insert(key.to_string(), value.to_string());
        }
    }
    map
}

fn container_fields_required(provided: &HashMap<String, String>) -> bool {
    provided
        .get("container")
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "true" | "1" | "yes"
            )
        })
        .unwrap_or(true)
}

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
/// * `template_variables` - Typed template variable overrides supplied via CLI flags
/// * `template_values_file` - Optional path to a file containing template values
/// * `skip_prompts` - Whether to skip all interactive prompts, using defaults for unspecified values
pub fn new_blueprint(
    name: &str,
    source: Option<Source>,
    blueprint_type: Option<BlueprintType>,
    mut define: Vec<String>,
    template_variables: TemplateVariables,
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

    template_variables.merge_into(&mut define);
    ensure_default_bool(&mut define, "flakes", true);
    ensure_default_bool(&mut define, "ci", true);
    ensure_default_bool(&mut define, "rust-ci", true);
    ensure_default_bool(&mut define, "release-ci", true);

    if skip_prompts {
        println!(
            "Skipping prompts; all template variables must be provided via CLI flags when using --skip-prompts."
        );
        let missing = missing_required_template_variables(&define);
        if !missing.is_empty() {
            let missing_list = missing.join(", ");
            return Err(Error::MissingTemplateVariables(missing_list));
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
        no_workspace: false,
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
