//! Engram - Distributed Memory System for AI Agents
//!
//! This is the Rust implementation of the Engram system, providing
//! a distributed memory system with Git-based storage, CLI interface,
//! and extensible architecture for AI agents.

pub mod analytics;
pub mod ask;
pub mod cli;
pub mod collab;
pub mod config;
pub mod engines;
pub mod entities;
pub mod error;
pub mod locus_cli;
pub mod migration;
pub mod nlq;
pub mod quality_gates;
pub mod sandbox;
pub mod session;
pub mod storage;
pub mod validation;
pub mod version;

use std::result::Result as StdResult;

/// Common result type used throughout the application
pub type Result<T> = StdResult<T, error::EngramError>;

pub use config::Config;
/// Re-export commonly used types
pub use entities::*;
pub use error::EngramError;
pub use storage::{MemoryEntity, Storage};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_compilation() {
        // Basic test to ensure library compiles
        assert!(true);
    }
}
