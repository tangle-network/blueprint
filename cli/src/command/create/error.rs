#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to generate blueprint: {0}")]
    GenerationFailed(anyhow::Error),
    #[error("Failed to initialize submodules, see .gitmodules to add them manually")]
    SubmoduleInit,
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Missing required template variables when --skip-prompts is used: {0}")]
    MissingTemplateVariables(String),
    #[error("Failed to write embedded template: {0}")]
    EmbeddedTemplateFailed(String),
}
