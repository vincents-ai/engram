//! Storage trait and supporting types.
//!
//! The `Storage` trait is the core abstraction for all engram storage
//! backends. Implementations include git-refs (primary), memory-only
//! (testing), and future backends.

use crate::error::EngramError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Filter for querying entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilter {
    pub entity_type: Option<String>,
    pub agent: Option<String>,
    pub text_search: Option<String>,
    pub field_filters: HashMap<String, serde_json::Value>,
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

/// Time range filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

/// Sort order for query results.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SortOrder {
    Asc,
    #[default]
    Desc,
}

/// Result of a query operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub entities: Vec<crate::entity_types::GenericEntity>,
    pub total_count: usize,
    pub has_more: bool,
}

/// Git commit information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommit {
    pub id: String,
    pub author: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub parents: Vec<String>,
}

/// Statistics about stored entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_entities: usize,
    pub entities_by_type: HashMap<String, usize>,
    pub repo_size_bytes: u64,
    pub last_commit: Option<String>,
}

/// Sync strategy for remote operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStrategy {
    Pull,
    Push,
    PullPush,
}

/// Conflict resolution strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    Ours,
    Theirs,
    Merge,
    Fail,
}

/// Result of a sync operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub pulled: usize,
    pub pushed: usize,
    pub conflicts: Vec<String>,
}

/// Remote sync direction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemoteSyncDirection {
    Pull,
    Push,
    Both,
}

/// Authentication for remote operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteAuth {
    pub method: String,
    pub credentials: serde_json::Value,
}

/// Options for remote sync operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSyncOptions {
    pub remote_url: String,
    pub direction: RemoteSyncDirection,
    pub auth: Option<RemoteAuth>,
    pub conflict_resolution: ConflictResolution,
    pub dry_run: bool,
}

/// Core storage trait — all engram storage backends implement this.
///
/// Provides CRUD, querying, branching, and sync operations.
/// Not async — engram's git-refs storage is synchronous (gix/git2 are sync).
pub trait Storage: Send {
    /// Store a memory entity.
    fn store(&mut self, entity: &crate::entity_types::GenericEntity) -> Result<(), EngramError>;

    /// Retrieve an entity by ID and type.
    fn get(&self, id: &str, entity_type: &str) -> Result<Option<crate::entity_types::GenericEntity>, EngramError>;

    /// Advanced query with filtering, sorting, and pagination.
    fn query(&self, filter: &QueryFilter) -> Result<QueryResult, EngramError>;

    /// Query entities by agent.
    fn query_by_agent(
        &self,
        agent: &str,
        entity_type: Option<&str>,
    ) -> Result<Vec<crate::entity_types::GenericEntity>, EngramError>;

    /// Query entities by time range.
    fn query_by_time_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<crate::entity_types::GenericEntity>, EngramError>;

    /// Query entities by type with optional filters.
    fn query_by_type(
        &self,
        entity_type: &str,
        filters: Option<&HashMap<String, serde_json::Value>>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<QueryResult, EngramError>;

    /// Text search across entities.
    fn text_search(
        &self,
        query: &str,
        entity_types: Option<&[String]>,
        limit: Option<usize>,
    ) -> Result<Vec<crate::entity_types::GenericEntity>, EngramError>;

    /// Count entities matching criteria.
    fn count(&self, filter: &QueryFilter) -> Result<usize, EngramError>;

    /// Delete an entity.
    fn delete(&mut self, id: &str, entity_type: &str) -> Result<(), EngramError>;

    /// List all entity IDs of a type.
    fn list_ids(&self, entity_type: &str) -> Result<Vec<String>, EngramError>;

    /// Get all entities of a specific type.
    fn get_all(&self, entity_type: &str) -> Result<Vec<crate::entity_types::GenericEntity>, EngramError>;

    /// Sync with remote repository.
    fn sync(&mut self) -> Result<(), EngramError>;

    /// Get current branch.
    fn current_branch(&self) -> Result<String, EngramError>;

    /// Create a new branch.
    fn create_branch(&mut self, branch_name: &str) -> Result<(), EngramError>;

    /// Switch to a branch.
    fn switch_branch(&mut self, branch_name: &str) -> Result<(), EngramError>;

    /// Merge branches.
    fn merge_branches(&mut self, source: &str, target: &str) -> Result<(), EngramError>;

    /// Get commit history.
    fn history(&self, limit: Option<usize>) -> Result<Vec<GitCommit>, EngramError>;

    /// Bulk store operations.
    fn bulk_store(&mut self, entities: &[crate::entity_types::GenericEntity]) -> Result<(), EngramError>;

    /// Get statistics about stored entities.
    fn get_stats(&self) -> Result<StorageStats, EngramError>;

    /// Cast to concrete type for accessing specific implementations.
    fn as_any(&self) -> &dyn std::any::Any;
}
