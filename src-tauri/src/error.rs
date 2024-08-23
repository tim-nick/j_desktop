use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("SQLite error")]
    SqliteError(#[from] rusqlite::Error),
    
    #[error("Serialization error")]
    SerdeError(#[from] serde_json::Error),

    #[error("IO error")]
    IoError(#[from] std::io::Error),
}