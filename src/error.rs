use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
