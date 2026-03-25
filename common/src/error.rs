use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    HyprCtl(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("No filepath for profile")]
    NoFilepath,

    #[error("Home directory not found")]
    NoHomeDir,

    #[error(transparent)]
    Pattern(#[from] glob::PatternError),

    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
}
