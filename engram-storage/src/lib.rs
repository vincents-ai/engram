//! engram-storage — storage implementations for the engram AI memory layer.
//!
//! Provides:
//! - `MemoryEntity` — content-addressable entity for storage
//! - Re-exports of all storage types from engram-core

pub mod memory_entity;
pub mod relationship_storage;
pub mod memory_only_storage;

pub use memory_entity::MemoryEntity;
pub use memory_only_storage::MemoryStorage;
pub use relationship_storage::*;

// Re-export storage types from engram-core for convenience
pub use engram_core::storage_types::{
    Storage, GitCommit, QueryFilter, QueryResult, StorageStats, SortOrder, TimeRange,
};
pub use engram_core::entity_types::{Entity, GenericEntity, EntityRegistry};
pub use engram_core::error::{EngramError, StorageError, Result};
pub use engram_core::relationship::{
    EntityRelationship, RelationshipFilter, EntityRelationType,
    RelationshipDirection, RelationshipStrength, RelationshipConstraints,
};
