//! engram-storage — storage implementations for the engram AI memory layer.
//!
//! Provides:
//! - `MemoryEntity` — content-addressable entity for storage
//! - Re-exports of all storage types from engram-core

pub mod memory_entity;
pub mod memory_only_storage;
pub mod relationship_storage;

pub use memory_entity::MemoryEntity;
pub use memory_only_storage::MemoryStorage;
pub use relationship_storage::*;

// Re-export storage types from engram-core for convenience
pub use engram_core::entity_types::{Entity, EntityRegistry, GenericEntity};
pub use engram_core::error::{EngramError, Result, StorageError};
pub use engram_core::relationship::{
    EntityRelationType, EntityRelationship, RelationshipConstraints, RelationshipDirection,
    RelationshipFilter, RelationshipStrength,
};
pub use engram_core::storage_types::{
    GitCommit, QueryFilter, QueryResult, SortOrder, Storage, StorageStats, TimeRange,
};
