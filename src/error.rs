//! Error types — re-exported from engram-core.
//!
//! The canonical error types live in `engram_core::error`. This module
//! re-exports them. Git-specific `From` impls are in engram-core behind
//! the `git` feature flag.

pub use engram_core::error::{ConfigError, EngramError, Result, StorageError};
