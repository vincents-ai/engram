//! Git refs-based storage implementation
//!
//! This implementation stores entities as Git objects referenced by Git refs,
//! eliminating the need for a separate .engram directory structure.
//! Entities are stored as Git blobs and referenced by refs in the format:
//! refs/engram/{entity_type}/{entity_id}

use super::{
    relationship_storage::{
        EntityPath, RelationshipIndex, RelationshipStats, RelationshipStorage, TraversalAlgorithm,
    },
    GitCommit, MemoryEntity, QueryFilter, QueryResult, SortOrder, Storage, StorageStats,
};
use crate::entities::{EntityRegistry, EntityRelationship, GenericEntity, RelationshipFilter};
use crate::error::{EngramError, StorageError};
use git2::Repository;
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Git refs-based storage for entities
///
/// Stores entities as Git blobs with refs pointing to them in the format:
/// refs/engram/{entity_type}/{entity_id}
///
/// This eliminates the need for .engram directory structure and provides
/// better integration with Git tooling and distributed workflows.
pub struct GitRefsStorage {
    repository: Arc<Mutex<Repository>>,
    workspace_path: PathBuf,
    #[allow(dead_code)]
    entity_registry: EntityRegistry,
    current_agent: String,
    relationship_index: Arc<Mutex<RelationshipIndex>>,
}

impl std::fmt::Debug for GitRefsStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitRefsStorage")
            .field("workspace_path", &self.workspace_path)
            .field("current_agent", &self.current_agent)
            .finish()
    }
}

impl GitRefsStorage {
    /// Create new Git refs storage instance
    pub fn new(workspace_path: &str, agent: &str) -> Result<Self, EngramError> {
        let workspace_path = PathBuf::from(workspace_path);

        let repository = if !workspace_path.join(".git").exists() {
            Repository::init(&workspace_path).map_err(|e| EngramError::Git(e.to_string()))?
        } else {
            Repository::open(&workspace_path).map_err(|e| EngramError::Git(e.to_string()))?
        };

        let mut registry = EntityRegistry::new();
        registry.register::<crate::entities::Task>();
        registry.register::<crate::entities::Context>();
        registry.register::<crate::entities::Reasoning>();
        registry.register::<crate::entities::Knowledge>();
        registry.register::<crate::entities::Session>();
        registry.register::<crate::entities::Compliance>();
        registry.register::<crate::entities::EntityRelationship>();

        let mut storage = GitRefsStorage {
            repository: Arc::new(Mutex::new(repository)),
            workspace_path,
            entity_registry: registry,
            current_agent: agent.to_string(),
            relationship_index: Arc::new(Mutex::new(RelationshipIndex::new())),
        };

        storage.rebuild_relationship_index()?;

        Ok(storage)
    }

    /// Get ref name for an entity
    fn get_entity_ref(&self, entity_type: &str, entity_id: &str) -> String {
        format!("refs/engram/{}/{}", entity_type, entity_id)
    }

    /// Store entity as Git blob and create ref
    fn store_entity_as_ref(&self, entity: &GenericEntity) -> Result<(), EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let data_map = match &entity.data {
            Value::Object(map) => map.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            _ => {
                let mut map = HashMap::new();
                map.insert("raw_data".to_string(), entity.data.clone());
                map
            }
        };

        let memory_entity = MemoryEntity::new(
            entity.id.clone(),
            entity.entity_type.clone(),
            entity.agent.clone(),
            entity.timestamp,
            data_map,
        );

        let json_content = serde_json::to_string_pretty(&memory_entity)?;

        let blob_oid = repo
            .blob(json_content.as_bytes())
            .map_err(|e| EngramError::Git(format!("Failed to create blob: {}", e)))?;

        let ref_name = self.get_entity_ref(&entity.entity_type, &entity.id);
        repo.reference(
            &ref_name,
            blob_oid,
            true,
            &format!("Update {} {}", entity.entity_type, entity.id),
        )
        .map_err(|e| EngramError::Git(format!("Failed to create ref: {}", e)))?;

        Ok(())
    }

    /// Load entity from Git ref
    fn load_entity_from_ref(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<Option<GenericEntity>, EngramError> {
        let ref_name = self.get_entity_ref(entity_type, entity_id);

        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let result = match repo.find_reference(&ref_name) {
            Ok(reference) => {
                let oid = reference.target().ok_or_else(|| {
                    EngramError::Storage(StorageError::InvalidState(format!(
                        "Ref {} has no target",
                        ref_name
                    )))
                })?;

                let blob = repo
                    .find_blob(oid)
                    .map_err(|e| EngramError::Git(format!("Failed to find blob {}: {}", oid, e)))?;

                let json_content = std::str::from_utf8(blob.content()).map_err(|e| {
                    EngramError::Storage(StorageError::InvalidState(format!(
                        "Invalid UTF-8 in blob: {}",
                        e
                    )))
                })?;

                let memory_entity: MemoryEntity = serde_json::from_str(json_content)
                    .map_err(|e| EngramError::Deserialization(e.to_string()))?;

                let generic_entity = GenericEntity {
                    id: memory_entity.id,
                    entity_type: memory_entity.entity_type,
                    agent: memory_entity.agent,
                    timestamp: memory_entity.timestamp,
                    data: serde_json::Value::Object(
                        memory_entity
                            .data
                            .into_iter()
                            .map(|(k, v)| (k, v))
                            .collect(),
                    ),
                };

                Ok(Some(generic_entity))
            }
            Err(_) => Ok(None),
        };
        result
    }

    /// Delete entity ref
    fn delete_entity_ref(&self, entity_type: &str, entity_id: &str) -> Result<(), EngramError> {
        let ref_name = self.get_entity_ref(entity_type, entity_id);

        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let result = match repo.find_reference(&ref_name) {
            Ok(mut reference) => {
                reference
                    .delete()
                    .map_err(|e| EngramError::Git(format!("Failed to delete ref: {}", e)))?;
                Ok(())
            }
            Err(_) => Ok(()),
        };
        result
    }

    /// List all entity refs of a given type
    fn list_entity_refs(&self, entity_type: &str) -> Result<Vec<String>, EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let ref_prefix = format!("refs/engram/{}/", entity_type);
        let mut entity_ids = Vec::new();

        // Iterate through all references
        let refs = repo
            .references()
            .map_err(|e| EngramError::Git(format!("Failed to list references: {}", e)))?;

        for reference in refs {
            let reference = reference
                .map_err(|e| EngramError::Git(format!("Failed to read reference: {}", e)))?;

            if let Some(name) = reference.name() {
                if name.starts_with(&ref_prefix) {
                    // Extract entity ID from ref name
                    let entity_id = name.strip_prefix(&ref_prefix).unwrap();
                    entity_ids.push(entity_id.to_string());
                }
            }
        }

        Ok(entity_ids)
    }

    /// Rebuild relationship index from all stored entities
    fn rebuild_relationship_index(&mut self) -> Result<(), EngramError> {
        let mut index = self.relationship_index.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState("Index lock failed".to_string()))
        })?;
        index.clear();

        // Get all entity types and rebuild index
        let entity_types = [
            "task",
            "context",
            "reasoning",
            "knowledge",
            "session",
            "compliance",
            "relationship",
        ];

        for entity_type in &entity_types {
            let entity_ids = self.list_entity_refs(entity_type)?;

            for entity_id in entity_ids {
                if let Some(entity) = self.load_entity_from_ref(entity_type, &entity_id)? {
                    if *entity_type == "relationship" {
                        // Special handling for relationship entities
                        if let Ok(relationship) =
                            serde_json::from_value::<EntityRelationship>(entity.data)
                        {
                            index.add_relationship(&relationship);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

// Storage trait implementation will be added next
impl Storage for GitRefsStorage {
    fn store(&mut self, entity: &GenericEntity) -> Result<(), EngramError> {
        self.store_entity_as_ref(entity)?;

        // Update relationship index if this is a relationship entity
        if entity.entity_type == "relationship" {
            if let Ok(relationship) =
                serde_json::from_value::<EntityRelationship>(entity.data.clone())
            {
                let mut index = self.relationship_index.lock().map_err(|_| {
                    EngramError::Storage(StorageError::InvalidState(
                        "Index lock failed".to_string(),
                    ))
                })?;
                index.add_relationship(&relationship);
            }
        }

        Ok(())
    }

    fn get(&self, id: &str, entity_type: &str) -> Result<Option<GenericEntity>, EngramError> {
        self.load_entity_from_ref(entity_type, id)
    }

    fn delete(&mut self, id: &str, entity_type: &str) -> Result<(), EngramError> {
        // Remove from relationship index if it's a relationship
        if entity_type == "relationship" {
            if let Some(entity) = self.load_entity_from_ref(entity_type, id)? {
                if let Ok(relationship) = serde_json::from_value::<EntityRelationship>(entity.data)
                {
                    let mut index = self.relationship_index.lock().map_err(|_| {
                        EngramError::Storage(StorageError::InvalidState(
                            "Index lock failed".to_string(),
                        ))
                    })?;
                    index.remove_relationship(&relationship);
                }
            }
        }

        self.delete_entity_ref(entity_type, id)
    }

    fn query(&self, filter: &QueryFilter) -> Result<QueryResult, EngramError> {
        let mut results = Vec::new();

        // Determine which entity types to search
        let entity_types = if let Some(entity_type) = &filter.entity_type {
            vec![entity_type.clone()]
        } else {
            vec![
                "task".to_string(),
                "context".to_string(),
                "reasoning".to_string(),
                "knowledge".to_string(),
                "session".to_string(),
                "compliance".to_string(),
            ]
        };

        for entity_type in entity_types {
            let entity_ids = self.list_entity_refs(&entity_type)?;

            for entity_id in entity_ids {
                if let Some(entity) = self.load_entity_from_ref(&entity_type, &entity_id)? {
                    // Apply filters
                    if let Some(agent_filter) = &filter.agent {
                        if entity.agent != *agent_filter {
                            continue;
                        }
                    }

                    // Apply field filters
                    let mut matches = true;
                    for (field, value) in &filter.field_filters {
                        if let Some(entity_value) = entity.data.get(field) {
                            if entity_value != value {
                                matches = false;
                                break;
                            }
                        } else {
                            matches = false;
                            break;
                        }
                    }

                    if matches {
                        results.push(entity);
                    }
                }
            }
        }

        // Apply sorting
        results.sort_by(|a, b| {
            if let Some(sort_field) = &filter.sort_by {
                let a_val = a.data.get(sort_field);
                let b_val = b.data.get(sort_field);
                match filter.sort_order {
                    SortOrder::Asc => match (a_val, b_val) {
                        (Some(a), Some(b)) => a.to_string().cmp(&b.to_string()),
                        (Some(_), None) => std::cmp::Ordering::Greater,
                        (None, Some(_)) => std::cmp::Ordering::Less,
                        (None, None) => std::cmp::Ordering::Equal,
                    },
                    SortOrder::Desc => match (b_val, a_val) {
                        (Some(a), Some(b)) => a.to_string().cmp(&b.to_string()),
                        (Some(_), None) => std::cmp::Ordering::Greater,
                        (None, Some(_)) => std::cmp::Ordering::Less,
                        (None, None) => std::cmp::Ordering::Equal,
                    },
                }
            } else {
                // Default sort by timestamp
                match filter.sort_order {
                    SortOrder::Asc => a.timestamp.cmp(&b.timestamp),
                    SortOrder::Desc => b.timestamp.cmp(&a.timestamp),
                }
            }
        });

        // Apply pagination
        let offset = filter.offset.unwrap_or(0);
        let limit = filter.limit.unwrap_or(50);
        let total = results.len();

        let paginated_results = results.into_iter().skip(offset).take(limit).collect();

        let has_more = offset + limit < total;
        Ok(QueryResult {
            entities: paginated_results,
            total_count: total,
            has_more,
        })
    }

    fn get_stats(&self) -> Result<StorageStats, EngramError> {
        let mut stats = StorageStats::default();

        let entity_types = [
            "task",
            "context",
            "reasoning",
            "knowledge",
            "session",
            "compliance",
            "relationship",
        ];

        for entity_type in &entity_types {
            let entity_ids = self.list_entity_refs(entity_type)?;
            let count = entity_ids.len();

            stats.total_entities += count;
            stats
                .entities_by_type
                .insert(entity_type.to_string(), count);
        }

        Ok(stats)
    }

    fn get_all(&self, entity_type: &str) -> Result<Vec<GenericEntity>, EngramError> {
        let entity_ids = self.list_entity_refs(entity_type)?;
        let mut entities = Vec::new();

        for entity_id in entity_ids {
            if let Some(entity) = self.load_entity_from_ref(entity_type, &entity_id)? {
                entities.push(entity);
            }
        }

        Ok(entities)
    }

    fn query_by_agent(
        &self,
        agent: &str,
        entity_type: Option<&str>,
    ) -> Result<Vec<GenericEntity>, EngramError> {
        let filter = QueryFilter {
            entity_type: entity_type.map(String::from),
            agent: Some(agent.to_string()),
            ..Default::default()
        };
        self.query(&filter).map(|result| result.entities)
    }

    fn query_by_time_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<GenericEntity>, EngramError> {
        let filter = QueryFilter::default();
        let result = self.query(&filter)?;

        let filtered_entities = result
            .entities
            .into_iter()
            .filter(|entity| entity.timestamp >= start && entity.timestamp <= end)
            .collect();

        Ok(filtered_entities)
    }

    fn query_by_type(
        &self,
        entity_type: &str,
        filters: Option<&HashMap<String, Value>>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<QueryResult, EngramError> {
        let mut filter = QueryFilter {
            entity_type: Some(entity_type.to_string()),
            limit,
            offset,
            ..Default::default()
        };

        if let Some(field_filters) = filters {
            filter.field_filters = field_filters.clone();
        }

        self.query(&filter)
    }

    fn text_search(
        &self,
        query: &str,
        entity_types: Option<&[String]>,
        limit: Option<usize>,
    ) -> Result<Vec<GenericEntity>, EngramError> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        let default_types = [
            "task".to_string(),
            "context".to_string(),
            "reasoning".to_string(),
            "knowledge".to_string(),
            "session".to_string(),
            "compliance".to_string(),
        ];
        let search_types = entity_types.unwrap_or(&default_types);

        for entity_type in search_types {
            let entities = self.get_all(entity_type)?;

            for entity in entities {
                let entity_json = serde_json::to_string(&entity.data).unwrap_or_default();
                if entity_json.to_lowercase().contains(&query_lower) {
                    results.push(entity);
                }

                if let Some(limit) = limit {
                    if results.len() >= limit {
                        return Ok(results);
                    }
                }
            }
        }

        Ok(results)
    }

    fn count(&self, filter: &QueryFilter) -> Result<usize, EngramError> {
        let result = self.query(filter)?;
        Ok(result.total_count)
    }

    fn list_ids(&self, entity_type: &str) -> Result<Vec<String>, EngramError> {
        self.list_entity_refs(entity_type)
    }

    fn sync(&mut self) -> Result<(), EngramError> {
        // For Git refs storage, sync could involve pushing/pulling refs
        // This is a simplified implementation
        Ok(())
    }

    fn current_branch(&self) -> Result<String, EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let head = repo
            .head()
            .map_err(|e| EngramError::Git(format!("Failed to get HEAD: {}", e)))?;

        if let Some(name) = head.shorthand() {
            Ok(name.to_string())
        } else {
            Ok("HEAD".to_string())
        }
    }

    fn create_branch(&mut self, branch_name: &str) -> Result<(), EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let head_commit = repo
            .head()
            .map_err(|e| EngramError::Git(format!("Failed to get HEAD: {}", e)))?
            .peel_to_commit()
            .map_err(|e| EngramError::Git(format!("Failed to get HEAD commit: {}", e)))?;

        repo.branch(branch_name, &head_commit, false)
            .map_err(|e| EngramError::Git(format!("Failed to create branch: {}", e)))?;

        Ok(())
    }

    fn switch_branch(&mut self, branch_name: &str) -> Result<(), EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let branch = repo
            .find_branch(branch_name, git2::BranchType::Local)
            .map_err(|e| EngramError::Git(format!("Failed to find branch: {}", e)))?;

        let branch_ref = branch.get();
        repo.set_head(branch_ref.name().unwrap())
            .map_err(|e| EngramError::Git(format!("Failed to switch branch: {}", e)))?;

        Ok(())
    }

    fn merge_branches(&mut self, _source: &str, _target: &str) -> Result<(), EngramError> {
        // Simplified merge implementation
        // In a real implementation, this would handle merge conflicts, etc.
        Err(EngramError::Git(
            "Branch merging not yet implemented for Git refs storage".to_string(),
        ))
    }

    fn history(&self, limit: Option<usize>) -> Result<Vec<GitCommit>, EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let mut revwalk = repo
            .revwalk()
            .map_err(|e| EngramError::Git(format!("Failed to create revwalk: {}", e)))?;

        revwalk
            .push_head()
            .map_err(|e| EngramError::Git(format!("Failed to push HEAD: {}", e)))?;

        let mut commits = Vec::new();
        let max_commits = limit.unwrap_or(100);

        for (i, oid_result) in revwalk.enumerate() {
            if i >= max_commits {
                break;
            }

            let oid = oid_result
                .map_err(|e| EngramError::Git(format!("Failed to get commit OID: {}", e)))?;
            let commit = repo
                .find_commit(oid)
                .map_err(|e| EngramError::Git(format!("Failed to find commit: {}", e)))?;

            let git_commit = GitCommit {
                id: commit.id().to_string(),
                author: commit.author().name().unwrap_or("Unknown").to_string(),
                message: commit.message().unwrap_or("").to_string(),
                timestamp: chrono::DateTime::from_timestamp(commit.time().seconds(), 0)
                    .unwrap_or_else(chrono::Utc::now),
                parents: commit.parent_ids().map(|id| id.to_string()).collect(),
            };

            commits.push(git_commit);
        }

        Ok(commits)
    }

    fn bulk_store(&mut self, entities: &[GenericEntity]) -> Result<(), EngramError> {
        for entity in entities {
            self.store(entity)?;
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// RelationshipStorage trait implementation
impl RelationshipStorage for GitRefsStorage {
    fn store_relationship(&mut self, relationship: &EntityRelationship) -> Result<(), EngramError> {
        let generic_entity = GenericEntity {
            id: relationship.id.clone(),
            entity_type: "relationship".to_string(),
            agent: relationship.agent.clone(),
            timestamp: relationship.timestamp,
            data: serde_json::to_value(relationship)?,
        };

        self.store(&generic_entity)
    }

    fn get_relationship(&self, id: &str) -> Result<Option<EntityRelationship>, EngramError> {
        if let Some(entity) = self.get(id, "relationship")? {
            let relationship = serde_json::from_value(entity.data)
                .map_err(|e| EngramError::Deserialization(e.to_string()))?;
            Ok(Some(relationship))
        } else {
            Ok(None)
        }
    }

    fn delete_relationship(&mut self, id: &str) -> Result<(), EngramError> {
        self.delete(id, "relationship")
    }

    fn query_relationships(
        &self,
        _filter: &RelationshipFilter,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        Ok(Vec::new())
    }

    fn get_entity_relationships(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let index = self.relationship_index.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState("Index lock failed".to_string()))
        })?;

        let rel_ids = index.get_all_relationships(entity_id);
        drop(index);

        let mut relationships = Vec::new();
        for rel_id in rel_ids {
            if let Some(rel) = self.get_relationship(&rel_id)? {
                relationships.push(rel);
            }
        }

        Ok(relationships)
    }

    fn get_outbound_relationships(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let index = self.relationship_index.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState("Index lock failed".to_string()))
        })?;

        let rel_ids = index.get_outbound(entity_id);
        drop(index);

        let mut relationships = Vec::new();
        for rel_id in rel_ids {
            if let Some(rel) = self.get_relationship(&rel_id)? {
                relationships.push(rel);
            }
        }

        Ok(relationships)
    }

    fn get_inbound_relationships(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let index = self.relationship_index.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState("Index lock failed".to_string()))
        })?;

        let rel_ids = index.get_inbound(entity_id);
        drop(index);

        let mut relationships = Vec::new();
        for rel_id in rel_ids {
            if let Some(rel) = self.get_relationship(&rel_id)? {
                relationships.push(rel);
            }
        }

        Ok(relationships)
    }

    fn find_paths(
        &self,
        _source_id: &str,
        _target_id: &str,
        _algorithm: TraversalAlgorithm,
        _max_depth: Option<usize>,
    ) -> Result<Vec<EntityPath>, EngramError> {
        Ok(Vec::new())
    }

    fn get_connected_entities(
        &self,
        _entity_id: &str,
        _algorithm: TraversalAlgorithm,
        _max_depth: Option<usize>,
    ) -> Result<Vec<String>, EngramError> {
        Ok(Vec::new())
    }

    fn get_relationship_index(&self) -> Result<&RelationshipIndex, EngramError> {
        Err(EngramError::Storage(StorageError::InvalidState(
            "Direct relationship index access not supported in Git refs storage".to_string(),
        )))
    }

    fn rebuild_relationship_index(&mut self) -> Result<(), EngramError> {
        self.rebuild_relationship_index()
    }

    fn get_relationship_stats(&self) -> Result<RelationshipStats, EngramError> {
        Ok(RelationshipStats {
            total_relationships: 0,
            relationships_by_type: HashMap::new(),
            bidirectional_count: 0,
            average_connections_per_entity: 0.0,
            most_connected_entity: None,
            relationship_density: 0.0,
        })
    }
}
