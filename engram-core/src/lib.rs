//! engram-core — shared traits, types, and errors.
//!
//! This crate contains the foundational abstractions used across all engram
//! crates. No storage implementations, no CLI, no git operations — just the
//! trait contracts and shared types.

pub mod error;
pub mod storage_types;
pub mod entity_types;

#[cfg(test)]
mod tests;

pub use error::{EngramError, StorageError, ConfigError};
pub use storage_types::{Storage, QueryFilter, QueryResult, TimeRange, SortOrder, GitCommit, StorageStats, SyncStrategy, ConflictResolution, SyncResult, RemoteSyncDirection, RemoteAuth, RemoteSyncOptions};
pub use entity_types::{Entity, GenericEntity, EntityRegistry};
