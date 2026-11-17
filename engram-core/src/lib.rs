//! engram-core — shared traits, types, and errors.
//!
//! This crate contains the foundational abstractions used across all engram
//! crates. No storage implementations, no CLI, no git operations — just the
//! trait contracts and shared types.

pub mod entity_types;
pub mod error;
pub mod relationship;
pub mod storage_types;

#[cfg(test)]
mod tests;

pub use entity_types::{Entity, EntityDeserializeFn, EntityRegistry, GenericEntity};
pub use error::{ConfigError, EngramError, StorageError};
pub use relationship::{
    EntityRelationType, EntityRelationship, RelationshipConstraints, RelationshipDirection,
    RelationshipFilter, RelationshipStrength,
};
pub use storage_types::{
    ConflictResolution, GitCommit, QueryFilter, QueryResult, RemoteAuth, RemoteSyncDirection,
    RemoteSyncOptions, SortOrder, Storage, StorageStats, SyncResult, SyncStrategy, TimeRange,
};
