use blueprint_runner::error::RunnerError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TestRunnerError {
    #[error(transparent)]
    Client(#[from] blueprint_clients::Error),
    #[error("Runner setup failed: {0}")]
    Setup(String),
    #[error("Runner execution failed: {0}")]
    Execution(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Keystore(#[from] blueprint_keystore::Error),
    #[error(transparent)]
    Parse(#[from] url::ParseError),
    #[error(transparent)]
    Runner(#[from] RunnerError),
}
