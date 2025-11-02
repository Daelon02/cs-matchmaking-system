#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Message: {0}")]
    StringError(String),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

pub type AppResult<T> = Result<T, AppError>;
