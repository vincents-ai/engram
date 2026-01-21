//! Error types for the Engram system

use thiserror::Error;

/// Main error type for Engram operations
#[derive(Error, Debug)]
pub enum EngramError {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Entity validation error: {0}")]
    Validation(String),

    #[error("Git operation failed: {0}")]
    Git(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

impl From<git2::Error> for EngramError {
    fn from(error: git2::Error) -> Self {
        EngramError::Git(error.to_string())
    }
}

impl From<anyhow::Error> for EngramError {
    fn from(error: anyhow::Error) -> Self {
        EngramError::InvalidOperation(error.to_string())
    }
}

/// Storage-specific errors
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("Invalid repository state: {0}")]
    InvalidState(String),

    #[error("Git operation failed: {0}")]
    GitOperation(String),

    #[error("Entity not found: {0}")]
    EntityNotFound(String),
}

/// Configuration-specific errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, EngramError>;
