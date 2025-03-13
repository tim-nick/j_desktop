use thiserror::Error;
use libsql::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("LibSQL error")]
    LibsqlError(#[from] libsql::Error),
    
    #[error("Serialization error")]
    SerdeError(#[from] serde_json::Error),

    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Custom error: {0}")]
    Custom(String),
}