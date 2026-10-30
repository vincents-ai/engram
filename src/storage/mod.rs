//! Storage layer — re-exported from engram-core and engram-storage.
//!
//! The canonical Storage trait and types live in engram-core.
//! Storage implementations (MemoryEntity, MemoryStorage, RelationshipStorage) live in engram-storage.
//! GitRefsStorage stays in the main crate (deeply coupled to CLI config).

pub mod git_refs_storage;

// Re-export storage types from engram-core
pub use engram_core::storage_types::{
    ConflictResolution, GitCommit, QueryFilter, QueryResult, RemoteAuth, RemoteSyncDirection,
    RemoteSyncOptions, SortOrder, Storage, StorageStats, SyncResult, SyncStrategy, TimeRange,
};

// Re-export everything from engram-storage (MemoryEntity, MemoryStorage, RelationshipStorage, etc.)
pub use engram_storage::*;

// Re-export everything from git_refs_storage (GitRefsStorage, helpers)
pub use git_refs_storage::*;
