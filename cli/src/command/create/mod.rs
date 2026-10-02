pub use crate::command::create::error::Error;
pub use crate::command::create::source::Source;
pub use crate::command::create::types::BlueprintType;
use crate::foundry::FoundryToolchain;
use clap::Args;
use std::collections::HashMap;
use types::BlueprintVariant;

pub mod error;
pub mod source;
pub mod types;

/// Required template keys for Tangle templates
const TANGLE_REQUIRED_KEYS: [&str; 2] = ["project-description", "project-authors"];

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

    /// Authors of the project (comma-separated list)
    #[arg(long = "project-authors", value_name = "AUTHORS")]
    pub project_authors: Option<String>,

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
        Self::push("project-authors", self.project_authors, define);
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

    for key in &TANGLE_REQUIRED_KEYS {
        if !provided.contains_key(*key) {
            missing.push(*key);
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

/// Entries that are allowed to pre-exist in an `--init` target directory.
///
/// `.git` is the whole point of `--init` (the operator already ran `git init`),
/// and `.gitignore` is commonly written alongside it.
const INIT_ALLOWED_ENTRIES: [&str; 2] = [".git", ".gitignore"];

/// Check that `dir` is safe to generate a blueprint into with `--init`.
///
/// `--init` sets `GenerateArgs::init`, which makes `ProjectDir::try_from`
/// return `destination()` (the cwd) instead of `destination()/sanitize(name)`.
/// That path skips the "Target directory already exists" bail in
/// `crates/cargo-generate/src/template_variables/project_dir.rs`, and the copy
/// stage uses `safe_copy_skip_existing`, which only *warns* on a name collision.
/// Generating into a populated directory would therefore scatter template
/// files around existing ones and exit 0, so we reject it up front.
///
/// # Errors
///
/// Returns [`Error::InitDirNotEmpty`] if `dir` holds anything other than
/// [`INIT_ALLOWED_ENTRIES`].
pub(crate) fn ensure_init_dir_is_empty(dir: &std::path::Path) -> Result<(), Error> {
    let mut unexpected: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        if INIT_ALLOWED_ENTRIES.contains(&file_name.as_str()) {
            continue;
        }
        unexpected.push(file_name);
    }

    if unexpected.is_empty() {
        return Ok(());
    }

    unexpected.sort();
    // Keep the message readable when a directory holds a lot of entries.
    let shown = unexpected.len().min(5);
    let mut listing = unexpected[..shown].join(", ");
    if unexpected.len() > shown {
        listing.push_str(&format!(" (and {} more)", unexpected.len() - shown));
    }

    Err(Error::InitDirNotEmpty(dir.display().to_string(), listing))
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
/// * `blueprint_type` - Optional blueprint type (Tangle)
/// * `define` - Template variable definitions (key=value pairs)
/// * `template_variables` - Typed template variable overrides supplied via CLI flags
/// * `template_values_file` - Optional path to a file containing template values
/// * `skip_prompts` - Whether to skip all interactive prompts, using defaults for unspecified values
/// * `init` - Generate into the current directory instead of a new `<name>/` subdirectory
pub fn new_blueprint(
    name: &str,
    source: Option<Source>,
    blueprint_type: Option<BlueprintType>,
    mut define: Vec<String>,
    template_variables: TemplateVariables,
    template_values_file: &Option<String>,
    skip_prompts: bool,
    init: bool,
) -> Result<(), Error> {
    // With `GenerateArgs::init`, cargo-generate sets
    // `should_initialize_git = !init || force_git_init()`, so an existing
    // `.git` is left alone. We only force the fresh `git init` when there is
    // none, which keeps the "a generated blueprint is a git repo" invariant
    // for the empty-directory case without re-initialising the operator's repo.
    let force_git_init = init && !std::path::Path::new(".git").exists();

    if init {
        // `GenerateArgs::init` makes the destination the cwd, so `name` no longer
        // names a directory and is only used as the crate/package name.
        let cwd = std::env::current_dir()?;
        println!(
            "Generating blueprint in current directory: {}",
            cwd.display()
        );
        ensure_init_dir_is_empty(&cwd)?;
    }

    println!("Generating blueprint with name: {name}");

    let source = source.unwrap_or_default();
    let blueprint_variant = blueprint_type.map(|t| t.get_type()).unwrap_or_default();
    let template_path_opt: Option<cargo_generate::TemplatePath> = source.into();

    let template_path = template_path_opt.unwrap_or_else(|| {
        // TODO: Interactive selection (#352)
        let template_repo: String = match blueprint_variant {
            Some(BlueprintVariant::Tangle) | None => {
                "https://github.com/tangle-network/blueprint-template".into()
            }
        };

        cargo_generate::TemplatePath {
            git: Some(template_repo),
            branch: Some(String::from("main")),
            ..Default::default()
        }
    });

    // Determine if this is a Tangle template (simpler requirements)

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
        bin: false,
        ssh_identity: None,
        gitconfig: None,
        define,
        init,
        destination: None,
        force_git_init,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_dir_accepts_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        assert!(ensure_init_dir_is_empty(dir.path()).is_ok());
    }

    #[test]
    fn init_dir_accepts_git_and_gitignore() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(".git")).unwrap();
        std::fs::write(dir.path().join(".gitignore"), "target\n").unwrap();
        assert!(ensure_init_dir_is_empty(dir.path()).is_ok());
    }

    #[test]
    fn init_dir_rejects_populated_dir() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("Cargo.toml"), "[package]\n").unwrap();
        let err = ensure_init_dir_is_empty(dir.path()).unwrap_err();
        assert!(
            matches!(&err, Error::InitDirNotEmpty(_, listing) if listing == "Cargo.toml"),
            "unexpected error: {err:?}"
        );
    }

    #[test]
    fn init_dir_rejects_dir_next_to_allowed_entries() {
        // The guard must not stop scanning at `.git` just because it is allowed.
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(".git")).unwrap();
        std::fs::write(dir.path().join("README.md"), "hi\n").unwrap();
        assert!(ensure_init_dir_is_empty(dir.path()).is_err());
    }

    #[test]
    fn init_dir_error_lists_sorted_and_truncates() {
        let dir = tempfile::tempdir().unwrap();
        for name in ["zeta", "alpha", "beta", "gamma", "delta", "epsilon"] {
            std::fs::write(dir.path().join(name), "").unwrap();
        }

        let Error::InitDirNotEmpty(_, listing) = ensure_init_dir_is_empty(dir.path()).unwrap_err()
        else {
            panic!("expected InitDirNotEmpty");
        };

        // Sorted, so the message is stable across filesystems.
        assert_eq!(listing, "alpha, beta, delta, epsilon, gamma (and 1 more)");
    }

    #[test]
    fn init_dir_does_not_create_the_directory() {
        // A read-only check: `--init` must not be what makes the dir exist.
        let parent = tempfile::tempdir().unwrap();
        let target = parent.path().join("not-yet");
        let err = ensure_init_dir_is_empty(&target).unwrap_err();
        assert!(matches!(err, Error::Io(_)), "unexpected error: {err:?}");
        assert!(!target.exists());
    }
}
