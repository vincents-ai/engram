//! Storage layer for Engram system
//!
//! Provides Git-based persistence with content-addressable storage
//! and multi-agent synchronization capabilities.

pub mod git_refs_storage;
pub mod git_storage;
pub mod memory_entity;
pub mod memory_only_storage;
pub mod relationship_storage;

pub use git_refs_storage::*;
pub use git_storage::*;
pub use memory_entity::*;
pub use memory_only_storage::*;
pub use relationship_storage::*;

use crate::entities::GenericEntity;
use crate::error::EngramError;
use serde_json::Value;
use std::collections::HashMap;

/// Query filter for entity searches
#[derive(Debug, Clone)]
pub struct QueryFilter {
    pub entity_type: Option<String>,
    pub agent: Option<String>,
    pub text_search: Option<String>,
    pub field_filters: HashMap<String, Value>,
    pub time_range: Option<TimeRange>,
    pub sort_by: Option<String>,
    pub sort_order: SortOrder,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Default for QueryFilter {
    fn default() -> Self {
        Self {
            entity_type: None,
            agent: None,
            text_search: None,
            field_filters: HashMap::new(),
            time_range: None,
            sort_by: None,
            sort_order: SortOrder::Desc,
            limit: Some(50),
            offset: Some(0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimeRange {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Query result with pagination info
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub entities: Vec<GenericEntity>,
    pub total_count: usize,
    pub has_more: bool,
}

/// Storage trait for different storage backends
pub trait Storage: Send {
    /// Store a memory entity
    fn store(&mut self, entity: &GenericEntity) -> Result<(), EngramError>;

    /// Retrieve an entity by ID and type
    fn get(&self, id: &str, entity_type: &str) -> Result<Option<GenericEntity>, EngramError>;

    /// Advanced query with filtering, sorting, and pagination
    fn query(&self, filter: &QueryFilter) -> Result<QueryResult, EngramError>;

    /// Query entities by agent
    fn query_by_agent(
        &self,
        agent: &str,
        entity_type: Option<&str>,
    ) -> Result<Vec<GenericEntity>, EngramError>;

    /// Query entities by time range
    fn query_by_time_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<GenericEntity>, EngramError>;

    /// Query entities by type with optional filters
    fn query_by_type(
        &self,
        entity_type: &str,
        filters: Option<&HashMap<String, Value>>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<QueryResult, EngramError>;

    /// Text search across entities
    fn text_search(
        &self,
        query: &str,
        entity_types: Option<&[String]>,
        limit: Option<usize>,
    ) -> Result<Vec<GenericEntity>, EngramError>;

    /// Count entities matching criteria
    fn count(&self, filter: &QueryFilter) -> Result<usize, EngramError>;

    /// Delete an entity
    fn delete(&mut self, id: &str, entity_type: &str) -> Result<(), EngramError>;

    /// List all entity IDs of a type
    fn list_ids(&self, entity_type: &str) -> Result<Vec<String>, EngramError>;

    /// Get all entities of a specific type
    fn get_all(&self, entity_type: &str) -> Result<Vec<GenericEntity>, EngramError>;

    /// Sync with remote repository
    fn sync(&mut self) -> Result<(), EngramError>;

    /// Get current branch
    fn current_branch(&self) -> Result<String, EngramError>;

    /// Create a new branch
    fn create_branch(&mut self, branch_name: &str) -> Result<(), EngramError>;

    /// Switch to a branch
    fn switch_branch(&mut self, branch_name: &str) -> Result<(), EngramError>;

    /// Merge branches
    fn merge_branches(&mut self, source: &str, target: &str) -> Result<(), EngramError>;

    /// Get commit history
    fn history(&self, limit: Option<usize>) -> Result<Vec<GitCommit>, EngramError>;

    /// Bulk operations
    fn bulk_store(&mut self, entities: &[GenericEntity]) -> Result<(), EngramError>;

    /// Get statistics about stored entities
    fn get_stats(&self) -> Result<StorageStats, EngramError>;

    /// Cast to concrete type for accessing specific implementations
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Git commit information
#[derive(Debug, Clone)]
pub struct GitCommit {
    pub id: String,
    pub author: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub parents: Vec<String>,
}

/// Storage statistics
#[derive(Debug, Clone, Default)]
pub struct StorageStats {
    pub total_entities: usize,
    pub entities_by_type: HashMap<String, usize>,
    pub entities_by_agent: HashMap<String, usize>,
    pub total_storage_size: u64,
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
}

/// Sync strategy for multi-agent collaboration
#[derive(Debug, Clone)]
pub enum SyncStrategy {
    LatestWins,
    PriorityWins { priority_agent: String },
    IntelligentMerge,
    ManualResolution,
}

/// Conflict resolution result
#[derive(Debug, Clone)]
pub struct ConflictResolution {
    pub entity_id: String,
    pub entity_type: String,
    pub strategy_used: SyncStrategy,
    pub winner: String,
    pub conflicts_detected: Vec<String>,
}

/// Sync result information
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub entities_synced: usize,
    pub conflicts_resolved: Vec<ConflictResolution>,
    pub errors: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub synced_agents: Vec<String>,
    pub merged_entities: usize,
    pub duration_ms: u64,
}

/// Remote sync direction
#[derive(Debug, Clone)]
pub enum RemoteSyncDirection {
    Pull,
    Push,
    BiDirectional,
}

/// Remote authentication configuration
#[derive(Debug, Clone)]
pub struct RemoteAuth {
    pub auth_type: String, // "http", "ssh", "none"
    pub username: Option<String>,
    pub password: Option<String>,
    pub key_path: Option<String>,
}

/// Remote sync options
#[derive(Debug, Clone)]
pub struct RemoteSyncOptions {
    pub remote: String,
    pub direction: RemoteSyncDirection,
    pub branch: Option<String>,
    pub agent_ids: Vec<String>,
    pub dry_run: bool,
    pub auth: RemoteAuth,
}
