//! Error types for the engram system.
//!
//! These are the canonical error types used across all engram crates.
//! The main `engram` crate re-exports these and adds `From` impls for
//! git2/gix error types.

use thiserror::Error;

/// Main error type for engram operations.
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

    #[error("Not implemented: {0}")]
    Unimplemented(String),
}

/// Storage-specific errors.
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

    #[error("Write conflict: {0}")]
    WriteConflict(String),

    #[error("Lock error: {0}")]
    LockError(String),

    #[error("Corruption detected: {0}")]
    Corruption(String),

    #[error("Remote sync failed: {0}")]
    SyncFailed(String),

    #[error("Branch error: {0}")]
    BranchError(String),

    #[error("Merge conflict: {0}")]
    MergeConflict(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),
}

/// Configuration errors.
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Missing configuration: {0}")]
    Missing(String),
}

/// Result type alias for convenience.
pub type Result<T> = std::result::Result<T, EngramError>;

// Git-specific From impls (behind feature flag)
#[cfg(feature = "git")]
impl From<git2::Error> for EngramError {
    fn from(error: git2::Error) -> Self {
        EngramError::Git(error.to_string())
    }
}

#[cfg(feature = "git")]
impl From<gix::hash::decode::Error> for EngramError {
    fn from(error: gix::hash::decode::Error) -> Self {
        EngramError::Git(error.to_string())
    }
}
